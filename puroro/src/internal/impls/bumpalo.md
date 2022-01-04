
# Bumpalo message structs

**The implementation is highly experimental and the interface will change
in very soon!!**

## Naming

The bumpalo struct has postfix `Bumpalo`.
For example, from a given `message` name `Foo` then 
`struct FooBumpalo` is generated.

## Bumpalo allocator

The generated struct always hold a reference to the instance of `Bump`.
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

