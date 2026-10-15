# Generated Code Guide

This page describes the Rust interface that puroro generates from a `.proto`
file: the modules, types, and methods a library user calls. Internal helpers
(`_fields`, `_pinternal`, oneof unions, and similar) are omitted unless they
are part of the public API.

The mapping follows the official
[Protocol Buffers style guide](https://protobuf.dev/programming-guides/style/)
and the scalar type table in the
[proto3 language guide](https://protobuf.dev/programming-guides/proto3/#scalar).
For comparison with other implementations, see the official
[C++ generated code guide](https://protobuf.dev/reference/cpp/cpp-generated/)
and
[Rust generated code guide](https://protobuf.dev/reference/rust/rust-generated/).
puroro’s API is not the same as protobuf-official Rust; differences are noted
where they matter.

You should read the
[proto2](https://protobuf.dev/programming-guides/proto2/) or
[proto3](https://protobuf.dev/programming-guides/proto3/) language guide
before this document. Protobuf Editions are not supported.

This library currently supports proto2 and proto3. Some proto features are not
generated yet; see [Unsupported features](#unsupported-features).

# Output files and modules

Unlike C++ or Java, puroro does **not** emit one `.rs` file per `.proto` file.
The `.proto` file name is ignored. Layout is determined only by:

- the `package` declaration
- nested messages, enums, and oneofs

Several `.proto` files that share a package are merged into the same Rust
module.

When the generated crate root is `lib.rs` (the default), a typical tree looks
like this:

```text
src/lib.rs           // crate root: `pub mod` for each top-level package
src/library.rs       // `package library;`
src/catalog.rs       // `package catalog.v1;` → `pub mod v1;`
src/catalog/v1.rs
```

The crate root also re-exports the `puroro` crate as `puroro` so generated
code can name library types without depending on how the crate was imported.

## Packages

The official style guide requires package names to be a dot-delimited sequence
of `lower_snake_case` identifiers, without Java-style prefixes such as
`com.company.x.y`.

Each dot-separated component becomes a nested Rust module in `lower_snake_case`:

```protobuf
package library.v1;
```

```rust,ignore
// crate_root::library::v1
pub mod library {
    pub mod v1 { /* messages and enums in this package */ }
}
```

Unlike [official protobuf Rust](https://protobuf.dev/reference/rust/rust-generated/),
puroro **does** use `package` to build the module tree.

# Identifier mapping

The style guide defines how names should be written in `.proto` files, then
each language maps those names into its own convention. puroro’s mapping is:

| `.proto` construct | Style guide | Generated Rust |
| --- | --- | --- |
| Package component | `lower_snake_case` | `lower_snake_case` module |
| Message | `TitleCase` | `TitleCase` struct (`Book`, `BookView`) |
| Field | `lower_snake_case` | `lower_snake_case` methods (`title()`, `title_mut()`) |
| Oneof | `lower_snake_case` | getter `group_one()`, case enum `GroupOneCase` |
| Enum type | `TitleCase` | `TitleCase` enum |
| Enum value | `UPPER_SNAKE_CASE` | `TitleCase` variant (`VALUE_SEVEN` → `ValueSeven`) |
| Field number constant | (derived) | `TITLE_FIELD_NUMBER` |

Words are split on `_` and on camel-case boundaries, then recased. If a name
already matches the target case, it is kept as-is.

Unlike official protobuf Rust, enum variants are **not** prefix-stripped.
`FOO_BAR_UNSPECIFIED` becomes `FooBarUnspecified`, not `Unspecified`. The
style guide still recommends prefixing enum values with the enum name in the
`.proto` file, because protobuf enum values are not scoped by the enum type.

Rust keywords are escaped:

- Most keywords (`type`, `match`, `enum`, …) become raw identifiers
  (`r#type()`). Suffixed names such as `type_mut` / `has_type` / `clear_type`
  do not need escaping.
- `crate`, `self`, `Self`, and `super` cannot be raw identifiers, so they
  get a leading underscore (`message Self` → `struct _Self`).

The style guide also warns that field / oneof names using prefixes `has_`,
`get_`, `set_`, `clear_` or the suffix `_value` can collide with generated
accessors. Avoid those names.

# Messages

Given:

```protobuf
syntax = "proto3";
package library;
message Book {}
```

the generator emits two public types in module `library`:

- `Book` — owned, mutable message
- `BookView` — immutable view of the same message

The relationship is the same idea as `Vec<T>` / `[T]`: `Book` implements
`Deref<Target = BookView>`, so getters defined on `BookView` can be called on
a `Book` value. Mutating methods live only on `Book`.

`BookView` is defined in a hidden `_view` module and re-exported with
`#[doc(inline)]`, so users write `library::BookView`.

## Owned type `Book`

```rust,ignore
pub struct Book(/* private */);

impl Book {
    // per-field mutators — see Fields
    pub fn title_mut(&mut self) -> &mut puroro::String { /* ... */ }
    pub fn clear_title(&mut self) { /* ... */ }

    pub const TITLE_FIELD_NUMBER: i32 = 1;
}

impl puroro::Message for Book {
    type ViewType = BookView;
    fn from_bytes_iter<I>(iter: I) -> puroro::Result<Self>
    where
        I: Iterator<Item = std::io::Result<u8>>;
    fn merge_from_bytes_iter<I>(&mut self, iter: I) -> puroro::Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>;
}
```

`Book` also implements `Default`, `Clone` (deep copy), `PartialEq`, `Debug`,
`Deref<Target = BookView>`, and `Borrow<BookView>`.

There is no generated `new()`. Use `Book::default()` or
`Book::from_bytes_iter(...)`.

## View type `BookView`

```rust,ignore
pub struct BookView { /* private */ }

impl BookView {
    // per-field getters — see Fields
    pub fn title(&self) -> &str { /* ... */ }
    pub fn title_opt(&self) -> Option<&str> { /* ... */ }
    pub fn has_title(&self) -> bool { /* ... */ }
}

impl puroro::MessageView for BookView {
    type MessageType = Book;
    fn to_bytes<W: std::io::Write>(&self, out: &mut W) -> puroro::Result<()>;
}
```

`BookView` also implements `PartialEq`, `Debug`, and
`ToOwned<Owned = Book>`. Because `Book: Deref<Target = BookView>`,
`book.to_bytes(...)` works on the owned type as well.

Take `&BookView` in APIs that only read a message. That keeps the caller from
depending on the owned layout.

## Nested types

A message that contains nested messages, nested enums, or oneofs also gets a
module named after the parent message in `lower_snake_case`:

```protobuf
syntax = "proto3";
package nested;

message Msg {
    int32 item_outer = 1;

    message Submsg {
        int32 item_inner = 1;
    }

    enum Enum {
        MY_VALUE = 0;
    }
}
```

Generated layout:

```text
nested.rs          // struct Msg, struct MsgView, `pub mod msg;`
nested/msg.rs      // struct Submsg, struct SubmsgView, enum Enum
```

User paths:

| `.proto` | Rust |
| --- | --- |
| `nested.Msg` | `nested::Msg` / `nested::MsgView` |
| `nested.Msg.Submsg` | `nested::msg::Submsg` / `nested::msg::SubmsgView` |
| `nested.Msg.Enum` | `nested::msg::Enum` |

This matches the nested-module pattern in the official Rust generated code
guide (`Foo` plus module `foo` for nested `Bar`).

If a message has no nested types and no oneofs, no extra module is generated.

# Scalar type mapping

Scalar fields use the same Rust types as the official proto3 scalar table,
except that `string` and `bytes` use puroro’s own buffer types instead of
`String` / `Vec<u8>`:

| Proto type | Getter (`foo()`) | Mutator (`foo_mut()`) | Owned element type |
| --- | --- | --- | --- |
| `double` | `f64` | `&mut f64` | `f64` |
| `float` | `f32` | `&mut f32` | `f32` |
| `int32`, `sint32`, `sfixed32` | `i32` | `&mut i32` | `i32` |
| `int64`, `sint64`, `sfixed64` | `i64` | `&mut i64` | `i64` |
| `uint32`, `fixed32` | `u32` | `&mut u32` | `u32` |
| `uint64`, `fixed64` | `u64` | `&mut u64` | `u64` |
| `bool` | `bool` | `&mut bool` | `bool` |
| `string` | `&str` | `&mut puroro::String` | `puroro::String` |
| `bytes` | `&[u8]` | `&mut puroro::Bytes` | `puroro::Bytes` |
| enum `E` | `E` | `&mut E` | `E` |
| message `M` | `Option<&MView>` | `&mut M` | `M` |

`puroro::String` derefs to `str` and implements `From<&str>` and
`From<std::string::String>`, so assignment looks like
`*book.title_mut() = "Yo".into();`.
`puroro::Bytes` derefs to `[u8]`.

# Fields

For every field `foo` in the `.proto` file, puroro generates snake_case
accessors. Following the official C++ / Rust guides, the field-name portion
keeps the `.proto` spelling (which should already be `lower_snake_case`).

Message fields always have presence in proto3. For non-message fields,
presence depends on syntax and label; see below.

In addition to accessors, each field produces a public constant on the owned
message:

```rust,ignore
impl Book {
    pub const TITLE_FIELD_NUMBER: i32 = 1;
    pub const NUM_PAGES_FIELD_NUMBER: i32 = 2;
}
```

The name is the field name converted to `UPPER_SNAKE_CASE`, plus
`_FIELD_NUMBER`.

Comments attached to a message, enum, enum value, or field in the `.proto`
file are copied into `#[doc]` on the corresponding generated item.

## Presence: proto2 `optional` / `required`, proto3 `optional`

These fields have **explicit presence** (a dedicated “is set” bit):

- proto2 `optional` and `required`
- proto3 `optional` (the `optional` label, also known as proto3 explicit
  presence)

`required` currently generates the same accessors as `optional`. Missing
required fields are **not** rejected at parse time.

For a non-repeated, non-message field `foo`:

| Method | On | Returns |
| --- | --- | --- |
| `foo()` | `FooView` | Current value, or the field default if unset |
| `foo_opt()` | `FooView` | `Some(value)` if set, otherwise `None` |
| `has_foo()` | `FooView` | `true` iff the field is set |
| `foo_mut()` | `Foo` | `&mut T`, creating / marking the field as set |
| `clear_foo()` | `Foo` | Unsets the field. After this, `has_foo()` is `false` and `foo()` returns the default |

Calling `foo_mut()` sets presence even if you leave the default value (for
example `0` or `""`).

proto2 `[default = ...]` is honored: `foo()` returns that value when the
field is unset. proto3 does not allow custom defaults; the language default
is used (`0`, `false`, `""`, first enum value, and so on).

For a message field the getter is already `Option<&BarView>`, so `foo()` and
`foo_opt()` return the same `Option`. `foo_mut()` inserts `Bar::default()`
if the field was empty.

## Presence: proto3 implicit (unlabeled) fields

A proto3 field without `optional` / `repeated` / `oneof` has **implicit
presence**. (Message fields still behave as explicit presence; see the proto3
guide section
[Message Type Fields Always Have Field Presence](https://protobuf.dev/programming-guides/proto3/#message-type-fields-always-have-field-presence).)

puroro still generates `has_foo()` and `foo_opt()` for these fields, unlike
official protobuf Rust, which omits them for implicit scalars.

For a non-message implicit field, “set” means “value is not the type
default”:

- `has_foo()` is `false` for `0`, `false`, `""`, empty bytes, or the
  zero/default enum value
- Calling `foo_mut()` and leaving the default does **not** make `has_foo()`
  true
- Assigning a non-default value makes `has_foo()` true
- `clear_foo()` restores the default and `has_foo()` becomes `false`

Implicit message fields use the same `Option<&BarView>` / `bar_mut()` API as
optional message fields.

## Repeated fields

For `repeated T foo = N;`:

| Method | On | Returns |
| --- | --- | --- |
| `foo()` | `FooView` | `impl RepeatedFieldView<'_, Item = V>` |
| `foo_mut()` | `Foo` | `&mut Vec<Owned>` |
| `clear_foo()` | `Foo` | Clears the vector |

`RepeatedFieldView` supports `len()`, `is_empty()`, indexing, and
`IntoIterator`. There is no `has_foo()` / `foo_opt()` for repeated fields.

View item type `V` vs mutator element type:

| Proto `T` | `foo()` item | `foo_mut()` |
| --- | --- | --- |
| numeric / bool / enum | owned `T` (`i32`, `MyEnum`, …) | `&mut Vec<T>` |
| `string` | `str` | `&mut Vec<puroro::String>` |
| `bytes` | `[u8]` | `&mut Vec<puroro::Bytes>` |
| message `M` | `MView` | `&mut Vec<M>` |

Example:

```rust,ignore
msg.i32_repeated_mut().extend([30, 40]);
assert_eq!(
    vec![30, 40],
    msg.i32_repeated().into_iter().cloned().collect::<Vec<_>>()
);
```

# Enumerations

Given:

```protobuf
syntax = "proto3";
enum Color {
    COLOR_UNSPECIFIED = 0;
    COLOR_RED = 1;
    COLOR_BLUE = 2;
}
```

puroro generates a Rust `enum` (not a newtype `struct Color(i32)` as in
official protobuf Rust):

```rust,ignore
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Color {
    ColorUnspecified,
    ColorRed,
    ColorBlue,
    _None(i32), // proto3 only
}

impl Default for Color { /* first listed value */ }
impl From<Color> for i32 { /* variant → number */ }
impl From<i32> for Color { /* number → variant, unknown → _None */ }
```

Notes:

- Variant names are the `.proto` value converted to `TitleCase`, with no
  prefix stripping.
- `Default` is the **first listed** value, not necessarily the `0` value.
  The style guide says the first value should be the zero value with an
  `_UNSPECIFIED` / `_UNKNOWN` suffix; proto2 does not require that, and
  puroro follows declaration order.
- proto3 unknown numbers become `Color::_None(n)` and convert back to `n`
  via `i32::from`.
- proto2 has no `_None` variant. Unknown numbers fail
  `TryFrom<i32>` with `PuroroError::UnknownEnumVariant`.

# Oneofs

Given:

```protobuf
message Msg {
    oneof group_one {
        int32 g1_int32 = 1;
        string g1_string = 2;
    }
}
```

Each oneof field gets the same per-field accessors as an `optional` field of
that type (`g1_int32()`, `has_g1_int32()`, `g1_int32_mut()`,
`clear_g1_int32()`, …). Setting one field clears the others in the oneof.

In addition, the generator emits:

- `fn group_one(&self) -> Option<GroupOneCase<...>>` on `MsgView`
- `fn clear_group_one(&mut self)` on `Msg`
- a public case enum in the parent message’s nested module

```rust,ignore
pub enum GroupOneCase<TG1Int32 = (), TG1String = ()> {
    G1Int32(TG1Int32),
    G1String(TG1String),
}
```

The getter fills the type parameters with borrowed field types, so a match
looks like:

```rust,ignore
match msg.group_one() {
    Some(msg::GroupOneCase::G1Int32(n)) => { /* n: i32 */ }
    Some(msg::GroupOneCase::G1String(s)) => { /* s: &str */ }
    None => { /* nothing set */ }
}
```

A message-typed oneof arm yields `&MView`. The case enum lives next to other
nested items, for example `oneofs2::msg::GroupOneCase` for message `Msg` in
package `oneofs2`.

If a nested message would use the same `TitleCase` name as a oneof case enum
(for example a nested `ConflictCase` plus `oneof conflict`), the case enum
is still generated in the `_case` submodule and re-exported. Prefer avoiding
that clash, as the style guide recommends.

# Unknown fields

Unknown field numbers encountered while parsing are stored and written back
by `to_bytes()`. They participate in `PartialEq` and `Debug`. There is no
public accessor for the unknown-field set.

# Comments

Leading and trailing comments on messages, fields, enums, and enum values in
the `.proto` file are emitted as Rust `#[doc]` attributes, so they show up in
`cargo doc` / docs.rs for the generated crate.

# Unsupported features

The following are not generated (or not fully implemented) at the moment:

- `map<K, V>` fields
- services / RPC
- extensions
- groups
- Protobuf Editions
- `allow_alias` on enums
- special handling of well-known types (`Any`, `Timestamp`, …)
- enforcing proto2 `required` on parse

See the repository README for the broader TODO list.
