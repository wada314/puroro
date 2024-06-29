A library for running protoc command with your own plugin code as a closure.
When you develop your own protoc compiler plugin, normally you need to compile your code as an executable binary and run `protoc` command by yourself with passing the path to the binary.
This crate provides a convenient replacement for that process: You just need to pass your code as a closure to this library, then it will run the `protoc` command as like as your closure is an executable binary.

# Example

```rust
use protoc_plugin_by_closure::Protoc;
use std::time::Duration;

Protoc::new()
    .proto_file("my_protobuf_file.proto")
    .proto_file("my_protobuf_file2.proto")
    .proto_path("path/to/my/input_proto_dir/")
    .out_dir("path/to/my/output_dir/")
    .run(Duration::from_sec(3), |request_bytes| {
        // Your plugin logic here, which takes the CodeGeneratorRequest bytes
        // and returns the Result of CodeGeneratorResponse bytes.
    })
    .unwrap();

// The generated file names depend on your plugin logic and the contents of
// the input proto files, but typically they will be like this:
assert!(std::path::Path("path/to/my/output_dir/my_protobuf_file.rs").exists());
assert!(std::path::Path("path/to/my/output_dir/my_protobuf_file2.rs").exists());
```

# How it works

This crate is using the `ipc-channel` crate for the communication between the internal plugin binary and your closure code.
When running the `protoc` command, this crate generates an one-time key for the IPC channel, and pass it to the internal plugin binary as a `protoc` argument.
The internal plugin binary then creates the IPC channel with the key, and sends the whole input `CodeGeneratorRequest` bytes back to the caller process.
The generated `CodeGeneratorResponse` bytes are sent back to the internal plugin binary in the same way, and the binary outputs that bytes to the stdout.

# Features

- `on-memory`: Enabled by default.
Provides [`ProtocOnMemory`] struct which makes you to run the `protoc` command without touching the actual filesystem. Because this feature is using the `tempfile` crate, you can disable it if you don't need it.

# Requirements

Nightly channel required.

This crate requires an unstable (as of 2024/06) [cargo feature `bindeps`](https://rust-lang.github.io/rfcs/3028-cargo-binary-dependencies.html).
This crate contains [.cargo/config.toml](.cargo/config.toml) file which enables this feature so you normally don't need to worry about it, but if you include this crate as a part of
your [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), then you need to create the `.cargo/config.toml` file with the following contents *under your cargo workspace directory* manually (see [the official document](https://doc.rust-lang.org/cargo/reference/config.html) for the reasons):

```toml
[unstable]
bindeps = true
```
