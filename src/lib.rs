use nostr_rs_proto::nauthz_grpc::{EventReply, EventRequest};

pub trait Plugin {
    fn start(&mut self);
    fn name(&self) -> String;
    fn admit_event(&mut self, request: &EventRequest) -> EventReply;
    fn stop(&mut self);
}
