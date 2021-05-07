# puroro, a protocol buffer implementation for rust

This library is under development and it is very possible to make breaking changes in near future.

## TODOs
- proto2
    - [ ] Groups, at least correctly ignore it (where's document!?)
    - [ ] default
    - [ ] extensions
- proto2 & 3
    - [x] Maps
        - [ ] Bumpalo's `field_take_or_init.rs` implementation needs improvement because `bumpalo::boxed::Box` does not support moving out the value like `std::boxed::Box` does.
    - [ ] OneOfs
    - [ ] Anys, and other well-known types
    - [ ] Unit tests
    - [ ] Write document!!
    - [ ] Print comments in the generated files
    - [ ] More usable message traits
        - [ ] Mutable interface
        - [ ] Repeated field interface
        - [ ] Map interface
    - [ ] Keep unknown fields
    - [ ] Deserializer from a slice
    - [ ] Required field checker
    - [ ] Other implementations
        - [x] Bumpalo -- Use Bumpalo for `Vec` and `String` allocation
        - [ ] SliceView (name TBD) -- A viewer over a `&[u8]` slice
        - [ ] Append (name TBD) -- A thin wrapper over other impls, just overriding few fields using `with_myfield()` method
    - [ ] Naming of the other implementations. Consider using a type generator class
    - [ ] Support the `allocator_api`. Waiting for the `String` support
    - [ ] RPCs / services
    - [ ] Deserializer vulnerbility: Need to limit the recursion depth

## subcrates

- puroro -- The crate that the library user need to import
- puroro-internal -- The crate that the generated code need to import
- puroro-plugin -- A protoc compiler plugin

## Sample command
Keep in mind that protoc command not work properly with Windows path separator "\\". Use "/" instead.

The below is a sample command for Windows OS.
Please replace the ".exe" below into your OS's one.

```
$ protoc <protofile-path> --plugin=protoc-gen-rust=./target/debug/puroro-plugin.exe --rust_out=<output-dir> --proto_path=<protofile-dir>
```