# protoc-gen-puroro

`protoc` plugin that emits Rust for the [puroro](https://github.com/wada314/puroro) runtime.

**This is not an official Google product.** For production Rust protobuf code, use [`google-protobuf`](https://crates.io/crates/google-protobuf).

Install the binary and leave it on `PATH`. `cargo install` builds the `protoc-gen-puroro` executable:

```shell
cargo install protoc-gen-puroro
mkdir -p out
protoc --puroro_out=out -I . shelf.proto
```

`protoc` runs `protoc-gen-puroro` from `PATH` when it sees `--puroro_out`. The crate name in that `Cargo.toml`, the `name` value under `[package]`, is the string `generated` unless you pass `crate_name`. The diagram in the [puroro README](https://github.com/wada314/puroro/blob/master/README.md#generate-a-crate) shows protobuf `package demo` / `message Shelf` as `generated::demo::Shelf`.

### Options (`--puroro_opt`)

Comma-separated `key=value` pairs (a bare `lazy` also works):

| Option | Default | Meaning |
|---|---|---|
| `crate_name` | `generated` | `[package] name` in the generated `Cargo.toml` |
| `proto2_utf8` | `none` | proto2 string views: `none` → `&[u8]`, `verify` → `&str` |
| `lazy` | off | Experimental: emit `NameLazy` and enable `puroro-rt/lazy` |
| `tonic` | on when services exist | Emit cfg-gated tonic client/server stubs; `false` suppresses |
| `tonic_client` | on | Emit `{service}_client` when `tonic` is on |
| `tonic_server` | on | Emit `{service}_server` when `tonic` is on |

## gRPC stubs (tonic)

When the `.proto` file declares `service` / `rpc`, the plugin also emits tonic-style
client and server modules next to the message types. Message bytes go through
[`puroro::tonic::PuroroCodec`](https://docs.rs/puroro/latest/puroro/tonic/struct.PuroroCodec.html)
(generated stubs construct it via `Default`, which uses the `Global` allocator).

### Generate

```protobuf
syntax = "proto3";
package helloworld;

message HelloRequest { string name = 1; }
message HelloReply { string message = 1; }

service Greeter {
  rpc SayHello(HelloRequest) returns (HelloReply);
}
```

```shell
mkdir -p greeter
protoc --puroro_out=greeter --puroro_opt=crate_name=greeter -I . greeter.proto
```

Stub emission is on by default when services exist. Pass
`--puroro_opt=tonic=false` to suppress it (or `tonic_client=false` /
`tonic_server=false` to emit only one side).

### Depend on the generated crate

Enable the generated crate's optional `tonic` feature (it also turns on
`puroro/tonic`). Application code typically also depends on `tonic` with
transport:

```toml
[dependencies]
greeter = { path = "../greeter", features = ["tonic"] }
tonic = "0.14"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }
```

Without `features = ["tonic"]`, message types still compile; the
`{service}_client` / `{service}_server` modules are `cfg`-gated out.

### Implement and call

Paths follow the protobuf package. For `package helloworld` and `service Greeter`:

```text
greeter::helloworld::HelloRequest
greeter::helloworld::greeter_server::{Greeter, GreeterServer}
greeter::helloworld::greeter_client::GreeterClient
```

```rust,ignore
use greeter::helloworld::greeter_server::{Greeter, GreeterServer};
use greeter::helloworld::{HelloReply, HelloRequest};
use greeter::puroro::StringMut;
use tonic::{Request, Response, Status};

struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name().to_owned();
        let mut reply = HelloReply::new();
        reply.message_mut().push_str(&format!("Hello {name}"));
        Ok(Response::new(reply))
    }
}

// Server:
tonic::transport::Server::builder()
    .add_service(GreeterServer::new(MyGreeter))
    .serve("127.0.0.1:50051".parse()?)
    .await?;
```

```rust,ignore
use greeter::helloworld::greeter_client::GreeterClient;
use greeter::helloworld::HelloRequest;
use greeter::puroro::StringMut;
use tonic::Request;

let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;
let mut req = HelloRequest::new();
req.name_mut().push_str("world");
let reply = client.say_hello(Request::new(req)).await?.into_inner();
assert_eq!(reply.message(), "Hello world");
```

Stubs stay on eager messages with `Global`. Custom decode allocators are available
on `PuroroCodec` / `PuroroDecoder` for hand-rolled tonic wiring; they are not
threaded through the generated stub APIs yet.

Avoid naming a message so its companion module collides with a stub module
(for example `message GreeterClient` next to `service Greeter` both want
`mod greeter_client`).

## Build the plugin from a checkout

This is for people changing puroro. Other users stop at `cargo install`.

```shell
cargo build -p protoc-gen-puroro
mkdir -p out
protoc --plugin=protoc-gen-puroro=$PWD/target/debug/protoc-gen-puroro --puroro_out=out -I . shelf.proto
```

Application code should not depend on this crate as a library. Depend on the generated crate. Runtime traits are that crate's `puroro` module.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
