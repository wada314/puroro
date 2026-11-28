//! Compile checks and an in-process tonic unary round-trip (opt-in via `--features tonic`).

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
    use ::std::net::SocketAddr;
    use ::tonic::transport::server::TcpIncoming;
    use ::tonic::{Request, Response, Status};

    #[test]
    fn stub_types_are_public() {
        // Touch the names so a missing emit fails this fixture under `--features tonic`.
        let _ = std::any::type_name::<GreeterClient<::tonic::transport::Channel>>();
        let _ = std::any::type_name::<GreeterServer<()>>();
        fn _server_trait_bound<T: Greeter>() {}
        let _ = <HelloRequest<Global> as Message>::FULL_NAME;
        let _ = <HelloReply<Global> as Message>::FULL_NAME;
    }

    /// Minimal server: unary `SayHello` only; streaming RPC stays unimplemented.
    struct TestGreeter;

    #[::tonic::async_trait]
    impl Greeter for TestGreeter {
        async fn say_hello(
            &self,
            request: Request<HelloRequest>,
        ) -> Result<Response<HelloReply>, Status> {
            let name = request.into_inner().name().to_owned();
            let mut reply = HelloReply::new();
            reply.message_mut().push_str(&format!("Hello {name}"));
            Ok(Response::new(reply))
        }

        type SayHelloStreamStream =
            ::tonic::codegen::tokio_stream::Empty<Result<HelloReply, Status>>;

        async fn say_hello_stream(
            &self,
            _request: Request<HelloRequest>,
        ) -> Result<Response<Self::SayHelloStreamStream>, Status> {
            Err(Status::unimplemented("unary round-trip only"))
        }
    }

    #[::tokio::test]
    async fn unary_say_hello_round_trip() {
        let incoming = TcpIncoming::bind(SocketAddr::from(([127, 0, 0, 1], 0)))
            .expect("bind loopback");
        let addr = incoming.local_addr().expect("local_addr");
        let (shutdown_tx, shutdown_rx) = ::tokio::sync::oneshot::channel::<()>();

        let serve = ::tokio::spawn(async move {
            ::tonic::transport::Server::builder()
                .add_service(GreeterServer::new(TestGreeter))
                .serve_with_incoming_shutdown(incoming, async {
                    let _ = shutdown_rx.await;
                })
                .await
        });

        let mut client = GreeterClient::connect(format!("http://{addr}"))
            .await
            .expect("connect");
        let mut req = HelloRequest::new();
        req.name_mut().push_str("world");
        let reply = client
            .say_hello(Request::new(req))
            .await
            .expect("SayHello")
            .into_inner();
        assert_eq!(reply.message(), "Hello world");

        let _ = shutdown_tx.send(());
        serve.await.expect("join server").expect("serve");
    }
}
