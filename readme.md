# puroro, a protocol buffer implementation for Rust

A yet another protocol buffer compiler implementation for Rust language.
This project is licensed under Apache 2.0 license.

This is not an officially supported Google product.

See [puroro/src/lib.rs](https://docs.rs/puroro/0.1.0/puroro/) for more documents.

## important notes

This library is under development and it is very possible to make breaking changes.

Currently this library only supports Rust nightly channel.

## subcrates
- puroro -- The crate that the library user need to import
- puroro-plugin -- A protoc compiler plugin
- tests -- Test cases
- tests-pb -- Compiling .pb files used by tests crate
- protobuf -- A git submodule of Google's official protobuf repository
- puroro-protobuf-compiled -- Compiled .rs files in protobuf crate so that puroro-plugin crate can use it
    - update-plugin-protos.bat -- A batch file to generate the compiled .rs files

## Sample command to compile
The crate `puroro-plugin` generates an executable file which can be used as
a `protoc` command's `--plugin=` flag target.
So once you build `puroro-plugin` somehow you can run `protoc` command like this
(The sample below is for Windows OS. For Linux, just replace .exe):

```
$ protoc <protofile-path> --plugin=protoc-gen-rust=./target/debug/puroro-plugin.exe --rust_out=<output-dir> --proto_path=<protofile-dir>
```

(Keep in mind that protoc command not work properly with Windows path separator "\\". Use "/" instead)

Check `tests-pb/build.rs` and `tests-pb/Cargo.toml` for a sample build script.
This library generates root `lib.rs` file and submodule files and directories, so you will
want to make a separated crate containing only generated .rs code (and build.rs and the source .pb files).

## TODOs
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
        - [ ] Mutable interface
        - [ ] (More) Repeated field interface
        - [ ] Map interface
    - [ ] Keep unknown fields
    - [ ] Deserializer from a slice
    - [ ] Serializer performance improvement
        - [ ] Allow heap memory allocation for serializer (maybe keep the v0.1's no-alloc ver as a choice, but anyway no-alloc ver is too slow)
    - [ ] Custom deserializer (?)
    - [ ] Required field checker
    - [ ] Other implementations
        - [ ] Bumpalo -- Use Bumpalo for `Vec` and `String` allocation
        - [ ] SliceView -- A viewer over a `&[u8]` slice, without allocating any extra memories
        - [x] Empty(Unit) -- `()`, which only returns default values
        - [x] Merged -- `(T, U)`
        - [x] Either -- `::either::Either<T, U>`
        - [x] Option -- `Option<T>`
        - [x] SingleField -- Similar with the simple implementation, though has only 1 field and others are same with `()`. Might be useful to make a minimum memory size struct when combined with `(T, U)` message types.
            - [x] oneof field support
            - [x] Builder
            - [ ] `Into<SomeIntType>` support for numerical fields
    - [ ] Support the `allocator_api`. Waiting for the `String` support
    - [ ] RPCs / services
    - [ ] Deserializer vulnerbility: Need to limit the recursion depth
