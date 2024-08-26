use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
};

type Subscribers<T> = Arc<Mutex<Vec<Sender<T>>>>;

#[derive(Clone)]
pub struct EventEmitter<T> {
    subscribers: Subscribers<T>,
}

impl<T: Send + Clone + 'static> EventEmitter<T> {
    pub fn new() -> Self {
        EventEmitter {
            subscribers: Arc::new(Mutex::new(Vec::default())),
        }
    }

    pub fn subscribe<F: 'static + Send + Fn(&T)>(&self, handler: F) {
        let (tx, rx) = channel();

        if let Ok(mut subs) = self.subscribers.lock() {
            subs.push(tx);
            dbg!("SUBS LEN!!!!", subs.len());
        }

        thread::spawn(move || {
            for event in rx {
                handler(&event);
            }
        });
    }

    pub fn emit(&self, event: T) {
        if let Ok(subs) = self.subscribers.lock() {
            for handler in subs.iter() {
                let _ = handler.send(event.clone());
            }
        }
    }
}

impl<T: Send + Clone + 'static> Default for EventEmitter<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestEvent {
        message: String,
    }

    #[test]
    fn test_event_emitter_with_captured_variable() {
        let event_emitter = EventEmitter::new();
        let captured_message = "Captured".to_string();

        let (tx, rx) = channel::<String>();

        event_emitter.subscribe(move |event: &TestEvent| {
            // Use the captured variable in the handler.
            let response = format!("{}: {}", captured_message, event.message);
            tx.send(response).unwrap();
        });

        let test_event = TestEvent {
            message: "Hello, world!".to_string(),
        };
        event_emitter.emit(test_event);

        let handler_response = rx.recv().unwrap();
        assert_eq!(handler_response, "Captured: Hello, world!");
    }
}
