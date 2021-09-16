# puroro, a protocol buffer implementation for rust

This library is under development and it is very possible to make breaking changes in very near future.

## TODOs
- proto2
    - [ ] Groups, at least correctly ignore it (where's document!?)
    - [x] Enums (In proto2 we need to refuse the unknown value)
    - [ ] default value (which is set like `optional int32 foo = 1; [default=10]`)
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
    - [ ] Custom deserializer (?)
    - [ ] Required field checker
    - [ ] Other implementations
        - [ ] Bumpalo -- Use Bumpalo for `Vec` and `String` allocation
        - [ ] SliceView -- A viewer over a `&[u8]` slice
        - [x] Empty(Unit) -- `()`, which only returns default values
        - [x] Merged -- `(T, U)`
        - [x] Either -- `::itertools::Either<T, U>`
        - [x] Option -- `Some<T>`
        - [x] SingleField -- Similar with the simple implementation, though has only 1 field and others are same with `()`. Might be useful to make a minimum memory size struct when combined with `(T, U)` message types.
            - [ ] oneof field support
            - [x] Builder
    - [ ] Support the `allocator_api`. Waiting for the `String` support
    - [ ] RPCs / services
    - [ ] Deserializer vulnerbility: Need to limit the recursion depth

## subcrates

- puroro -- The crate that the library user need to import
- puroro-plugin -- A protoc compiler plugin

## Sample command
Keep in mind that protoc command not work properly with Windows path separator "\\". Use "/" instead.

The below is a sample command for Windows OS.
Please replace the ".exe" below into your OS's one.

```
$ protoc <protofile-path> --plugin=protoc-gen-rust=./target/debug/puroro-plugin.exe --rust_out=<output-dir> --proto_path=<protofile-dir>
```

`protobuf-pb/build.rs` is a sample build script.
