# Design Conisderations

## Priorities

- (P0) Fast & Efficient [de]serialization
- (P1) Reasonable get / set interfaces
- (P2) Generated code readability

## What is needed for the [de]serialization performance?

### Lazy load
The message type field supports `[lazy = true]` and `[unverfied_lazy = true]` options.

Quote from the code comment:
```rust
  // Should this field be parsed lazily?  Lazy applies only to message-type
  // fields.  It means that when the outer message is initially parsed, the
  // inner message's contents will not be parsed but instead stored in encoded
  // form.  The inner message will actually be parsed when it is first accessed.
  // ...
  // This option does not affect the public interface of any generated code;
  // all method signatures remain the same.  Furthermore, thread-safety of the
  // interface is not affected by this option; const methods remain safe to
  // call from multiple threads concurrently, while non-const methods continue
  // to require exclusive access.
```

Though the comment says "*This option does not affect the public interface of any generated code*", when deserializing we may want to keep the reference to the input slice to not copy the input slice into newly allocated buffer, and that requires interface code change in Rust:

```rust
struct MyNonLazyMessage(/* ... */);
struct MyLazyMessage<'a>(/* ... */, &'a [u8]);
impl MyNonLazyMessage {
    fn deser_from_slice(slice: &[u8]) -> Self {
        // ...
    }
}
impl<'a> MyLazyMessage<'a> {
    fn deser_from_slice(slice: &'a [u8]) -> Self {
        // ...
    }
}
```
