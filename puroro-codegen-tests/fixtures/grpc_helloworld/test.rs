//! Compile checks for tonic service stubs (opt-in via `--features tonic`).

use crate::grpc_helloworld::helloworld::{HelloReply, HelloRequest};
use puroro::{Message, StringMut};

#[test]
fn hello_messages_round_trip_without_tonic_feature() {
    let mut req = HelloRequest::new();
    req.name_mut().push_str("world");
    let bytes = Message::encode_to_vec(&req);
    let decoded = HelloRequest::<::allocator_api2::alloc::Global>::decode(bytes.as_slice())
        .expect("decode");
    assert_eq!(decoded.name(), "world");

    let mut reply = HelloReply::new();
    reply.message_mut().push_str("hello");
    assert_eq!(reply.message(), "hello");
}

#[cfg(feature = "tonic")]
mod tonic_stubs {
    use super::*;
    use crate::grpc_helloworld::helloworld::greeter_client::GreeterClient;
    use crate::grpc_helloworld::helloworld::greeter_server::{Greeter, GreeterServer};
    use ::allocator_api2::alloc::Global;

    #[test]
    fn stub_types_are_public() {
        // Touch the names so a missing emit fails this fixture under `--features tonic`.
        let _ = std::any::type_name::<GreeterClient<::tonic::transport::Channel>>();
        let _ = std::any::type_name::<GreeterServer<()>>();
        fn _server_trait_bound<T: Greeter>() {}
        let _ = <HelloRequest<Global> as Message>::FULL_NAME;
        let _ = <HelloReply<Global> as Message>::FULL_NAME;
    }
}
