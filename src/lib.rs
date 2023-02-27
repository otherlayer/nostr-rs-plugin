use nostr_rs_proto::nauthz_grpc::{EventReply, EventRequest};

pub trait Plugin {
    fn name(&self) -> String;
    fn admit_event(&self, request: &EventRequest) -> EventReply;
}
