use std::rc::Rc;

use nostr_rs_plugin::Plugin;
use nostr_rs_proto::nauthz_grpc::{EventRequest, EventReply, Decision};
use ahash::AHashMap;
#[cfg(not(test))]
use tracing::debug;
#[cfg(test)]
use std::{println as debug};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::runtime::Builder;


struct Welcome {
    event_tx: mpsc::Sender<EventRequest>,
    shutdown_send: mpsc::UnboundedSender<()>,
}

async fn event_handler(
    mut event_rx: mpsc::Receiver<EventRequest>,
    mut shutdown: UnboundedReceiver<()>,
) -> usize {
    let mut seen_users: AHashMap<Vec<u8>, bool> = AHashMap::new();

    loop {
        tokio::select! {
            _ = shutdown.recv() => {
                break;
            }

            Some(event_request) = event_rx.recv() => {
                let event = event_request.event.unwrap();

                if !seen_users.contains_key(&event.pubkey) {
                    let pubkey = event.pubkey.clone();

                    seen_users.insert(pubkey.clone(), true);

                    debug!("Welcoming {} user", seen_users.len());
                    // send welcome message to user
                }

                debug!("Event received of kind {}, seen users count: {}", event.kind, seen_users.len())
            }
        }
    }
    debug!("Event handler end");

    seen_users.len()
}

// wip
impl Plugin for Welcome {
    fn start(&self) {
    }

    fn name(&self) -> String {
        return "Welcome".to_owned();
    }

    fn admit_event(&self, request: &EventRequest) -> EventReply {
        self.event_tx.try_send(request.clone()).ok();

        return EventReply {
            decision: Decision::Permit as i32,
            message: Some(format!("Welcome")),
        };
    }

    fn stop(&self) {
        self.shutdown_send.send(()).ok();
    }
}

#[no_mangle]
pub fn get_plugin() -> *mut dyn Plugin {
    let (event_tx, event_rx) = mpsc::channel::<EventRequest>(1024);
    let (shutdown_send, shutdown_recv) = mpsc::unbounded_channel::<()>();

    let rt = Rc::new(Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(4)
        .build()
        .unwrap());
    
    rt.spawn(event_handler(
        event_rx,
        shutdown_recv,
    ));

    // Return a raw pointer to an instance of our plugin
    Box::into_raw(Box::new(Welcome {
        event_tx,
        shutdown_send,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr_rs_proto::nauthz_grpc::Event;
    use nostr_rs_proto::nauthz_grpc::event::TagEntry;
    use tokio::time::Duration;

    fn mock_event_request() -> EventRequest {
        EventRequest {
            event: Some(Event {
                id: vec![12],
                pubkey: vec![151],
                created_at: 35432,
                kind: 1,
                content: "content".to_owned(),
                sig: vec![12],
                tags: vec![TagEntry { values: vec!["tag".to_owned()]}]
            }),
            ip_addr: Some("127.0.0.1".to_owned()),
            origin: Some("origin".to_owned()),
            user_agent: None,
            auth_pubkey: Some(vec![1]),
            nip05: None,
        }
    }

    #[tokio::test]
    async fn test_plugin() {        
        let (event_tx, event_rx) = mpsc::channel::<EventRequest>(1024);
        let (shutdown_send, shutdown_recv) = mpsc::unbounded_channel::<()>();
        
        let handle = tokio::task::spawn(event_handler(
            event_rx,
            shutdown_recv,
        ));

        let request: EventRequest = mock_event_request();

        let welcome = Welcome {
            event_tx: event_tx.clone(),
            shutdown_send: shutdown_send.clone(),
        };

        let reply: EventReply = welcome.admit_event(&request);

        assert_eq!(reply.decision, Decision::Permit as i32);

        let reply: EventReply = welcome.admit_event(&request);

        assert_eq!(reply.decision, Decision::Permit as i32);

        tokio::task::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
    
            shutdown_send.clone().send(()).ok().unwrap();
        });

        let size = handle.await.ok().unwrap();
        
        assert_eq!(size, 1);
    }
}