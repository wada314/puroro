
# Bumpalo message structs

**The implementation is highly experimental and the interface will change
in very soon!!**

The generated struct always own a reference to a instance of 
[`Bump`](crate::bumpalo::Bump). Currently all the parent and children bumpalo
message instances must refer to the same `Bump` instance.

## Naming

The bumpalo struct has postfix `Bumpalo`.
For example, from a given `message` named `Foo`,
`struct FooBumpalo` is generated.

## Initialize

Instead of the `new()` method, it has `new_in(bump: &Bump)` initializer method:

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

The interface become a little complicated, but still you can use it as
same as the normal message struct. For any of the following given inputs:

```protobuf
optional string foo = 1;

// proto2 only
required string foo = 1;

// proto3 only
string foo = 1;
```

The generated interface is:

```rust
# use std::ops::DerefMut;
# struct FooBumpalo<'bump>(std::marker::PhantomData<&'bump ()>);
# impl<'bump> FooBumpalo<'bump> {
pub fn foo_mut(&mut self) -> impl DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>> {
    // ...
#   unsafe { std::ptr::NonNull::dangling().as_mut() }
}

pub fn clear_foo(&mut self) {
    // ...
#   todo!()
}
# }
```

The normal message struct's [`String`] is replaced by
[`::puroro::bumpalo::collections::String<'bump>`](crate::bumpalo::collections::String),
and `&mut` reference is replaced by [`DerefMut`](std::ops::DerefMut).

For `bytes` field, it's using [`::puroro::bumpalo::collections::Vec<'bump, u8>`](crate::bumpalo::collections::Vec) type instead.

### Singular message field mutable getters

It's same as the normal message struct's getter, just the
returned message type is not a normal message struct but
a bumpalo message struct.

### Repeated field mutable getters

The interface is TBD.

