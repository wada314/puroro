
# Priorities

- (P0) Fast & Efficient [de]serialization
- (P1) Reasonable get / set interfaces
- (P2) Generated code readability

# What is needed for the [de]serialization performance?

## Lazy load
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

Probably the solution here is to add the lifetime parameter even for the non-lazy message types, and prepare 2 kinds of deser methods which keeps the reference into the slice and which does not (copying it to inner `Vec`):

```rust
struct MyNonLazyMessage<'a>(/* ... */, PhantomData<&'a ()>);
struct MyLazyMessage<'a>(/* ... */, Cow<'a, [u8]>);
impl<'a> MyNonLazyMessage<'a> {
    fn deser_from_slice(slice: &'a [u8]) -> MyNonLazyMessage<'a> {
        // ...
    }
    fn deser_from_slice2(slice: &[u8]) -> MyNonLazyMessage<'static> {
        // same as above
    }
}
impl<'a> MyLazyMessage<'a> {
    fn deser_from_slice(slice: &'a [u8]) -> MyLazyMessage<'a> {
        // keeps the slice ref in the message struct
    }
    fn deser_from_slice2(slice: &[u8]) -> MyLazyMessage<'static> {
        // copies the input slice into `Vec` (`Cow` in this case)
    }
}
```

DO WE REALLY NEED TO PAY THIS COST TO JUST MAKE A LAZY FIELD LOADING?

## Pass around the message: Deser and then ser

The typical usecase of the protobuf I've seen in the actual product is something on a gRPC-based backend server code which is invoking another backend server via gRPC.
The server sometimes just returns the backend-backend server's call result `***Response` proto type, or do a simple re-packing into another `***Response` proto type.
Even in such cases, we need to deserialize the proto, allocate & initialize a message type, and then serialize it again.
If we can keep the original encoded bytes in the immutable message type, then the serialize operation will be just returning the stored bytes. If we use the lazy decode method discussed above, we don't even need to deserialize the message.

## Deserialization source: Read, a slice, slices

### `::std::io::Read` (or `::futures::io::AsyncReadEx`)

pros:

cons:
- Always copies into the owned buffer.
