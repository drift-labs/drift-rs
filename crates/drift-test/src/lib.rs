use drift_abi::{
    accounts::User,
    types::{Order, PerpPosition, SpotPosition},
};
use solana_program::{pubkey::Pubkey, slot_history::Slot};

mod drift_abi;

extern "C" {
    pub fn authority(user: &User, slot: Slot) -> Pubkey;
}

#[test]
fn test_ffi_call() {
    let user = User {
        authority: Pubkey::new_unique(),
        delegate: Pubkey::new_unique(),
        ..Default::default()
    };

    unsafe {
        let x = authority(
            &User {
                ..Default::default()
            },
            3_u64,
        );
        dbg!(x.to_string());
    }
}
