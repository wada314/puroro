# puroro, a protocol buffer implementation for Rust

A yet another protocol buffer compiler implementation for Rust language.
This project is licensed under Apache 2.0 license.

This is not an officially supported Google product.

See [puroro/src/lib.rs](https://docs.rs/puroro/) for more documents.

## important notes

This library is under development and it is very possible to make breaking changes.

Currently this library only supports Rust nightly channel.

# How to compile your .pb files to .rs files

First, let's create a crate for your .pb files (and the generated .rs files).
It is highly recommended to make a separated crate which only contains
the .pb files and the generated .rs files.

```shell
$ cargo new my-examples --lib
$ cd my-examples

# Download and compile puroro-plugin executable file
# The command generates bin/ directory and put the executable file under it
# Check the output and remember the version of puroro-plugin you installed
$ cargo install puroro-plugin --root=./

# Create your .pb files under this directory
$ mkdir protos
```

As an example, let's make a simple proto file `test1.proto`:

```protobuf
syntax = "proto3";
package library;
message Book {
    string title = 1;
    uint32 num_pages = 2;
}
```

Note that the file names does not make any effect in the generated codes
(For the generated code's filenames, the package names specified in the
proto files are used).


Then edit the `Cargo.toml` to add the dependency to `puroro` library crate:

```toml
[dependencies]
# Use the same version with the puroro-plugin you installed at above
puroro = "0.4.0"
```

As a last step, create a file `build.rs` into the crate root directory
with a contents like this (You can just copy-and-paste. Don't forget to
edit the "exe" extension for non-Windows users).

```rust
use std::env;
use std::path::PathBuf;
use std::process::Command;
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=protos/*.proto");
    // ############ You may want to edit here for non-windows environment ############
    let plugin_exe_path = ["bin", "puroro-plugin.exe"].iter().collect::<PathBuf>();
    let output_rust_path = ["src"].iter().collect::<PathBuf>();
    // ######### You may also want to edit here for finding protoc command ###########
    let protoc_exe = env::var("PURORO_PROTOC_PATH").unwrap_or("protoc".to_string());
    let protoc_status = Command::new(&protoc_exe)
        .arg("protos/*.proto")
        .arg(format!(
            "--plugin=protoc-gen-rust={}",
            plugin_exe_path.to_str().unwrap()
        ))
        .arg(format!("--rust_out={}", output_rust_path.to_str().unwrap()))
        .arg(format!("--proto_path={}", "./protos/"))
        .arg("--experimental_allow_proto3_optional")
        .status()
        .unwrap();
    if !protoc_status.success() {
        println!("cargo:warning=Failed to run `protoc` command.");
        panic!("Failed to run `protoc` command.")
    }
}
```

Once you have finished these steps, the directory should be like this:

    + my-examples/
        ├ src/
        │   └ (empty)
        ├ protos/
        │   └ test1.proto
        ├ bin/
        │   └ puroro-plugin.exe
        ├ cargo.toml
        ├ build.rs
        ├ (some other generated files)

Then run `cargo build` command. If it successfully runs, then the generated
`.rs` files will be generated under `src/` directory and you can use it from
your own crate. Congraturations!

# subcrates
- puroro -- The crate that the library user need to import
- puroro-plugin -- A protoc compiler plugin
- tests -- Test cases
- tests-pb -- Compiling .pb files used by tests crate
- protobuf -- A git submodule of Google's official protobuf repository
- puroro-protobuf-compiled -- Compiled .rs files in protobuf crate so that puroro-plugin crate can use it
    - update-plugin-protos.bat -- A batch file to generate the compiled .rs files

# TODOs
- proto2
    - [ ] Groups, at least correctly ignore it
    - [x] Enums (In proto2 we need to refuse the unknown value)
    - [x] default value (something like `optional int32 foo = 1; [default=10]`)
    - [ ] extensions
- proto2 & 3
    - [ ] Maps
    - [x] OneOfs
        - [x] Type definitions
        - [x] serialize / deserialize
    - [ ] Anys, and other well-known types
    - [ ] Enum allow-alias option
    - [ ] More more more unit tests
    - [ ] More more more more documents
    - [ ] Print comments in the generated files
    - [ ] Reflections
        - [ ] Get message metadata (descriptors)
    - [ ] Nightly / stable features
        - [ ] Support stable (not using nightly features)
    - [ ] More useful message traits
        - [x] Mutable interface
        - [ ] (More) Repeated field interface
        - [ ] Map interface
    - [ ] Keep unknown fields
    - [ ] Deserializer from a slice
    - [ ] Serializer performance improvement
        - [x] Allow heap memory allocation for serializer (maybe keep the v0.1's no-alloc ver as a choice, but anyway no-alloc ver is too slow)
    - [ ] Custom deserializer (?)
    - [ ] Required field checker
    - [ ] Other implementations
        - [x] Bumpalo -- Use Bumpalo for `Vec` and `String` allocation
        - [ ] SliceView -- A viewer over a `&[u8]` slice, without allocating any extra memories
        - [x] Empty(Unit) -- `()`, which only returns default values
        - [x] Merged -- `(T, U)`
        - [x] Either -- `::either::Either<T, U>`
        - [x] Option -- `Option<T>`
        - [x] SingleField -- Similar with the simple implementation, though has only 1 field and others are same with `()`. Might be useful to make a minimum memory size struct when combined with `(T, U)` message types.
            - [x] oneof field support
            - [x] Builder
            - [x] `Into<SomeIntType>` support for numerical fields
                - [ ] Allow the fields type to be non-`Clone`, `Debug`, `PartialEq` (currently we are just deriving those traits when defining a struct, but instead we should write a manual implementation which is only available when the field impls those traits).
    - [ ] Support the `allocator_api`. Waiting for the `String` support
    - [ ] RPCs / services
    - [ ] Deserializer vulnerbility: Need to limit the recursion depth
    - [ ] Get multiple fields mutable references at once
