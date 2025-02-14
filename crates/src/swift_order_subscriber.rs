use std::time::{SystemTime, UNIX_EPOCH};

use anchor_lang::AnchorDeserialize;
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    constants::MarketExt,
    types::{Context, MarketId, SdkError, SdkResult, SwiftOrderParamsMessage},
    DriftClient,
};

pub const SWIFT_DEVNET_WS_URL: &str = "wss://master.swift.drift.trade";
pub const SWIFT_MAINNET_WS_URL: &str = "wss://swift.drift.trade";

const LOG_TARGET: &str = "swift";

#[derive(Deserialize)]
struct SwiftOrderMessage<'a> {
    #[serde(borrow)]
    uuid: &'a str,
    ts: u64,
    #[serde(borrow)]
    order_message: &'a str,
}

#[derive(Deserialize)]
struct SwiftMessage {
    #[serde(default)] // TODO: should be `SwiftOrderMessage` not a string
    order: Option<String>,
}

/// Emits `SwiftOrderParamsMessage` from the Ws server
pub type SwiftOrderStream = ReceiverStream<SwiftOrderParamsMessage>;

/// Subscribe to the Swift WebSocket server, authenticate, and listen to new orders
///
/// `client` Drift client instance
/// `markets` markets to subscribe for new swift orders
///
/// Returns a stream of new Swift order messages
pub async fn subscribe_swift_orders(
    client: &DriftClient,
    markets: &[MarketId],
) -> SdkResult<SwiftOrderStream> {
    let swift_base_url = if client.context == Context::MainNet {
        SWIFT_MAINNET_WS_URL
    } else {
        SWIFT_DEVNET_WS_URL
    };
    let maker_pubkey = client.wallet().authority().to_string();
    let (ws_stream, _) = connect_async(format!("{swift_base_url}/ws?pubkey={maker_pubkey}"))
        .await
        .map_err(|err| {
            log::error!(target: LOG_TARGET, "couldn't connect to server: {err:?}");
            SdkError::WsClient(err)
        })?;

    let (mut outgoing, mut incoming) = ws_stream.split();

    // handle authentication and subscription
    while let Some(msg) = incoming.next().await {
        let msg = msg.map_err(|err| {
            log::error!(target: LOG_TARGET, "failed reading swift msg: {err:?}");
            SdkError::WsClient(err)
        })?;

        if let Message::Text(text) = msg {
            log::debug!(target: LOG_TARGET, "msg: {text}");
            let message: Value = serde_json::from_str(&text).expect("Failed to parse message");

            if let Some(err) = message.get("error") {
                log::error!(target: LOG_TARGET, "swift server error: {err:?}");
                return Err(SdkError::WebsocketError);
            }

            // authenticate with Ws server
            if message["channel"] == "auth" && message.get("nonce").is_some() {
                let nonce = message["nonce"].as_str().expect("got nonce");
                let signature = client
                    .wallet()
                    .sign_message(nonce.as_bytes())
                    .expect("infallible");
                let signature_b64 = base64.encode(signature.as_ref());

                let auth_message = json!({
                    "pubkey": maker_pubkey,
                    "signature": signature_b64,
                });
                outgoing
                    .send(Message::Text(auth_message.to_string()))
                    .await?;
                continue;
            }

            // subscribe to markets
            if message["channel"] == "auth" && message["message"] == "Authenticated" {
                let subscribe_msgs: Vec<Result<Message, _>> = markets
                    .iter()
                    .map(|m| {
                        assert!(m.is_perp(), "only perp markets");
                        let market = client
                            .program_data()
                            .perp_market_config_by_index(m.index())
                            .expect("market exists");
                        let subscribe_msg = json!({
                          "action": "subscribe",
                          "market_type": "perp",
                          "market_name": market.symbol(),
                        });
                        Ok(Message::Text(subscribe_msg.to_string()))
                    })
                    .collect();

                outgoing
                    .send_all(&mut futures_util::stream::iter(subscribe_msgs))
                    .await?;
                break;
            }
        }
    }

    let (tx, rx) = tokio::sync::mpsc::channel(256);

    // handle swift orders
    tokio::spawn(async move {
        while let Some(msg) = incoming.next().await {
            match msg {
                Ok(Message::Text(ref text)) => match serde_json::from_str::<SwiftMessage>(text) {
                    Ok(msg) => {
                        if let Some(ref order_json) = msg.order {
                            let order_json =
                                serde_json::from_str::<SwiftOrderMessage>(order_json).unwrap();
                            log::debug!(target: LOG_TARGET, "uuid: {}, latency: {}ms", order_json.uuid, unix_now_ms().saturating_sub(order_json.ts));
                            let order_message_buf =
                                hex::decode(order_json.order_message).expect("valid hex");
                            let swift_order_params: SwiftOrderParamsMessage =
                                AnchorDeserialize::deserialize(&mut &order_message_buf[8..])
                                    .expect("valid swift borsh");

                            if let Err(err) = tx.try_send(swift_order_params) {
                                log::error!(target: LOG_TARGET, "order chan failed: {err:?}");
                                break;
                            }
                        }
                    }
                    Err(err) => {
                        log::error!(target: LOG_TARGET, "{text}. invalid json: {err:?}");
                        break;
                    }
                },
                Ok(Message::Close(_)) => {
                    log::error!(target: LOG_TARGET, "server closed connection");
                    break;
                }
                Ok(_) => continue,
                Err(err) => {
                    log::error!(target: LOG_TARGET, "failed reading swift msg: {err:?}");
                    break;
                }
            }
        }
    });

    Ok(ReceiverStream::new(rx))
}

fn unix_now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[test]
fn test_swift_order_deser() {
    let msg = r#"{
        "channel":"swift_orders_perp_1",
        "order":"{
            \"market_index\":1,\"market_type\":\"perp\",\"order_message\":\"b9c165ffdf70594d0001010080841e00000000000000000000000000010000000000000000013201a4e99abc16000000011ab2f982160000000300900f84150000000072753959424c52740000\",
            \"order_signature\":\"FIgxWlW+C0abvtE8esSko7At1YGM8h66T0u5lJpwXirW63CuvEllVWZ68NNVFsaqcj4jqgQInXUnLPjIf/PQDA==\",
            \"signing_authority\":\"4rmhwytmKH1XsgGAUyUUH7U64HS5FtT6gM8HGKAfwcFE\",
            \"taker_authority\":\"4rmhwytmKH1XsgGAUyUUH7U64HS5FtT6gM8HGKAfwcFE\",
            \"ts\":1739518796400,
            \"uuid\":\"ru9YBLRt\"
        }"
    }"#;
    let s: SwiftMessage = serde_json::from_str(&msg).unwrap();
    serde_json::from_str::<SwiftOrderMessage>(s.order.unwrap().as_str()).unwrap();
}
