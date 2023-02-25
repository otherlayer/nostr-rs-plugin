use dlopen2::wrapper::WrapperApi;
use nostr_rs_proto::nauthz_grpc::{EventReply, EventRequest};

pub trait Plugin {
    fn admit_event(&self, request: &EventRequest) -> EventReply;
}

#[derive(WrapperApi)]
struct PluginApi {
    get_plugin: extern fn() -> *mut dyn Plugin,
}
