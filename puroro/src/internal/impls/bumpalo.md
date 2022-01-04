
# Bumpalo message structs

**The implementation is highly experimental and the interface will change
in very soon!!**

## Naming

The bumpalo struct has postfix `Bumpalo`.
For example, from a given `message` name `Foo` then 
`struct FooBumpalo` is generated.

## Initialize

The generated struct always hold a reference to a instance of 
[`Bump`](crate::bumpalo::Bump).
So instead of the `new()` method, it has `new_in(bump: &Bump)` initializer method:

```rust
pub struct FooBumpalo<'bump> {
#   phantom: std::marker::PhantomData<&'bump ()>,
    // ...
}

impl<'bump> FooBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
#       todo!()
        // ...
    }
}
```

And the trait `Default` is not implemented for the bumpalo message struct.

## Fields

### Immutable getters

Immutable getters are the same as the normal message struct impl.
Just the message getters' return types are not the normal message struct but
the bumpalo message struct instead.

### Singular numeric field mutable getters

Still the same as the normal message struct.

### Singular string / bytes field mutable getters

The interface become a little complicated, but you can use as same as
the normal message struct. For the given input:

```protobuf
optional string foo = 1;

// proto2 only
required string foo = 1;

// proto3 only
string foo = 1;
```

### Repeated field mutable getters

The interface is TBD.

