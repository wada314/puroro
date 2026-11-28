//! Compile checks and in-process tonic unary / server-streaming round-trips
//! (opt-in via `--features tonic`).

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
    use ::std::pin::Pin;
    use ::tonic::codegen::tokio_stream::{Stream, StreamExt, iter};
    use ::tonic::transport::Channel;
    use ::tonic::transport::server::TcpIncoming;
    use ::tonic::{Request, Response, Status};

    #[test]
    fn stub_types_are_public() {
        // Touch the names so a missing emit fails this fixture under `--features tonic`.
        let _ = std::any::type_name::<GreeterClient<Channel>>();
        let _ = std::any::type_name::<GreeterServer<()>>();
        fn _server_trait_bound<T: Greeter>() {}
        let _ = <HelloRequest<Global> as Message>::FULL_NAME;
        let _ = <HelloReply<Global> as Message>::FULL_NAME;
    }

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
            Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send>>;

        async fn say_hello_stream(
            &self,
            request: Request<HelloRequest>,
        ) -> Result<Response<Self::SayHelloStreamStream>, Status> {
            let name = request.into_inner().name().to_owned();
            let replies = (1..=3).map(move |i| {
                let mut reply = HelloReply::new();
                reply
                    .message_mut()
                    .push_str(&format!("Hello {name} #{i}"));
                Ok(reply)
            });
            Ok(Response::new(Box::pin(iter(replies))))
        }
    }

    struct LiveServer {
        client: GreeterClient<Channel>,
        shutdown_tx: ::tokio::sync::oneshot::Sender<()>,
        serve: ::tokio::task::JoinHandle<Result<(), ::tonic::transport::Error>>,
    }

    impl LiveServer {
        async fn start() -> Self {
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

            let client = GreeterClient::connect(format!("http://{addr}"))
                .await
                .expect("connect");
            Self {
                client,
                shutdown_tx,
                serve,
            }
        }

        async fn shutdown(self) {
            let _ = self.shutdown_tx.send(());
            self.serve.await.expect("join server").expect("serve");
        }
    }

    #[::tokio::test]
    async fn unary_say_hello_round_trip() {
        let mut live = LiveServer::start().await;
        let mut req = HelloRequest::new();
        req.name_mut().push_str("world");
        let reply = live
            .client
            .say_hello(Request::new(req))
            .await
            .expect("SayHello")
            .into_inner();
        assert_eq!(reply.message(), "Hello world");
        live.shutdown().await;
    }

    #[::tokio::test]
    async fn server_stream_say_hello_round_trip() {
        let mut live = LiveServer::start().await;
        let mut req = HelloRequest::new();
        req.name_mut().push_str("world");
        let mut stream = live
            .client
            .say_hello_stream(Request::new(req))
            .await
            .expect("SayHelloStream")
            .into_inner();

        let mut messages = Vec::new();
        while let Some(item) = stream.next().await {
            let reply = item.expect("stream item");
            messages.push(reply.message().to_owned());
        }
        assert_eq!(
            messages,
            ["Hello world #1", "Hello world #2", "Hello world #3"]
        );
        live.shutdown().await;
    }
}
