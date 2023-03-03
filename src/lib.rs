use nostr_rs_proto::nauthz_grpc::{EventReply, EventRequest, Decision};

pub trait Plugin {
    fn start(&self) {}
    fn name(&self) -> String;
    fn admit_event(&self, _: &EventRequest) -> EventReply {
        return EventReply { decision: Decision::Permit as i32, message: None }
    }
    fn stop(&self) {}
}
