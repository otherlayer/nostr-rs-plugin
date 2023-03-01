use nostr_rs_proto::nauthz_grpc::{EventReply, EventRequest};

pub trait Plugin {
    fn start(&self);
    fn name(&self) -> String;
    fn admit_event(&self, request: &EventRequest) -> EventReply;
    fn stop(&self);
}
