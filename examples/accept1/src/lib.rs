use nostr_rs_plugin::Plugin;
use nostr_rs_proto::nauthz_grpc::{EventRequest, EventReply, Decision};

struct Accept1;

// accept only kind 1 events
impl Plugin for Accept1 {
    fn start(&self) {}

    fn name(&self) -> String {
        return "Accept1".to_owned();
    }

    fn admit_event(&self, request: &EventRequest) -> EventReply {
        let reply;
        let opt_event = &request.event;

        match opt_event {
            Some(event) => {
                if event.kind == 1 {
                    reply = EventReply {
                        decision: Decision::Permit as i32,
                        message: Some(format!("I like kind 1")),
                    }
                } else {
                    reply = EventReply {
                        decision: Decision::Deny as i32,
                        message: Some(format!("I don't like kind {}", event.kind)),
                    }
                }
            },
            None => {
                reply = EventReply {
                    decision: Decision::Deny as i32,
                    message: Some(format!("No event in request!")),
                }
            }
        }

        return reply;
    }

    fn stop(&self) {}
}

#[no_mangle]
pub fn get_plugin() -> *mut dyn Plugin {
    // Return a raw pointer to an instance of our plugin
    Box::into_raw(Box::new(Accept1 {}))
}
