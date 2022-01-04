# Generated trait
For every message in input .pb files, puroro generates a Trait named
`<MyMessageName>Trait`.
The trait only implements immutable interfaces, not mutable ones.

Please also check the [Generated struct](crate::internal::impls::simple) page for
related informations like generated `enum`.

## Fields

### Singular non-message fields

For any of the following inputs,

```protobuf
optional int32 foo = 1;
repeated int32 foo = 1;

// proto3 only
int32 foo = 1;
```

The following trait methods are generated:

```rust
# trait MyMessageTrait {
// Returns the field value if the field is present.
// Please note that for proto3 bare singluar field, 0-value fields
// (or "" for string, b"" for bytes) are also considered as not present.
// If the field is not present, then returns the default value,
// which is normally 0 but in proto2 you can override it with
// a field option like `[default = 10]`.
fn foo(&self) -> i32;

// Returns `Some` if the field is present, and `None` if not.
// Unlike the `foo()` method, the default value setting does not
// make any effect to this method.
fn foo_opt(&self) -> Option<i32>;

// A shorthand of `self.foo_opt().is_some()`.
fn has_opt(&self) -> bool;
# }
```

The field return type are corresponding rust primitive type for
numerical types, `&str` for `string`, `&[u8]` for `bytes`, and the
generated `enum` (by value) for `enum` field.

### Singluar message field

Assuming we have another message type `message Bar`,
for any of the following inputs:

```protobuf
optional Bar foo = 1;
repeated Bar foo = 1;

// proto3 only
Bar foo = 1;
```

The following trait methods and typedefs are generated:

```rust
# trait BarTrait {}
# trait MyMessageTrait {
#
type FooMessageType<'this>: BarTrait;

// Returns `Some` if the field is present, and `None` if not.
fn foo(&self) -> Option<Self::FooMessageType>;

// Exactly the same as `foo()` method.
fn foo_opt(&self) -> Option<Self::FooMessageType>;

// A shorthand of `self.foo_opt().is_some()`.
fn has_opt(&self) -> bool;
# }
```

The typedef `FooMessageType<'this>` is `&'this Bar` in the
normal message implementation. So essentially, the interfaces are
exactly the same as the normal message interface.

### Repeated numerical field

For the following input:

```protobuf
repeated int32 foo = 1;
```

The generated code is:

```rust
# trait MyMessageTrait {
pub fn foo(&self) -> !;
#}
```

## oneofs

From a proto like this:
```protobuf
syntax = "proto3";
message MyMessage {
    oneof my_oneofs {
        int32 item1 = 1;
        float item2 = 2;
    }
}
```

This trait and enum are generated:

```rust
pub trait MyMessageTrait {
    fn my_oneofs(&self) -> Option<my_message::MyOneofs>;

    fn item1(&self) -> i32;
    fn item1_opt(&self) -> Option<i32>;
    fn has_item1(&self) -> bool;
    fn item2(&self) -> f32;
    fn item2_opt(&self) -> Option<f32>;
    fn has_item2(&self) -> bool;
}
pub mod my_message {
    pub enum MyOneofs {
        Item1(i32),
        Item2(f32),
    }
}
```

# trait impls

The generated trait is implemented for the generated message structs and
the following types:

```rust
# trait MyMessageTrait {}
impl MyMessageTrait for () { /* ... */ }
impl<'a, T: MyMessageTrait> MyMessageTrait for &'a T { /* ... */ }
impl<'a, T: MyMessageTrait> MyMessageTrait for &'a mut T { /* ... */ }
impl<T: MyMessageTrait> MyMessageTrait for Box<T> { /* ... */ }
impl<T: MyMessageTrait> MyMessageTrait for Option<T> { /* ... */ }
impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for (T, U) { /* ... */ }
impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for puroro::Either<T, U> { /* ... */ }
```

### `&'a T`, `&'a mut T`, `Box<T>`
Behaves as same as `T`.

### `()`
Always returns default values in every methods.

### `Option<T>`
If the value is `Some`, then behaves as same as `T`.
If the value is `None`, then behaves as same as `()`.

### `puroro::Either<T, U>`
Behaves as either `T` or `U`.

### `(T, U)`
Behaves as a merged message of `T` and `U`.
- Non-repeated, non-message field: Prioritize `U`'s value.
- Non-repeated, message field: Merges `T`'s and `U`'s values.
- Repeated field: Concatenates `T` and `U`'s repaeted values.

