# Future tasks (internal)

Scratch notes from library-user / generated-code discussions. Not a
commitment to ship any of these; they are reminders for later design work.
The public mapping is documented in [generated-code.md](./generated-code.md).

---

## Enum variant prefix stripping

**Current behavior:** A proto enum value is converted to a Rust `TitleCase`
variant with **no** prefix stripping. For example:

```protobuf
enum Color {
    COLOR_UNSPECIFIED = 0;
    COLOR_RED = 1;
}
```

becomes `Color::ColorUnspecified` / `Color::ColorRed`, not `Unspecified` /
`Red`.

Official protobuf Rust *does* strip a prefix that matches the enum name
(see the [Rust generated code guide](https://protobuf.dev/reference/rust/rust-generated/)),
because the style guide requires prefixing values in `.proto` files to avoid
C++-style unscoped clashes, and that prefix is redundant once the variants
live inside a Rust `enum` / `impl`.

**Idea:** Add a codegen option to strip the enum-name prefix when it is
safe, **or** just change the default to strip. Need to define “safe”
(official generators refuse to strip when the remainder would not be a
legal identifier, e.g. `DEVICE_TIER_1` → `1`).

---

## Enum representation: Rust `enum` vs `i32` + associated constants

**Current behavior:** puroro emits a real Rust `enum`:

```rust
pub enum Color {
    ColorUnspecified,
    ColorRed,
    _None(i32), // proto3 only
}
```

Official protobuf Rust emits a newtype `struct Color(i32)` plus associated
constants (`Color::Unspecified`, …). That representation can hold any `i32`,
including duplicate numbers (`allow_alias`) and unknown values, without a
catch-all variant.

**Idea:** Add a codegen option to emit the `i32` + constants style, **or**
make that the default. If we keep a Rust `enum`, it is the `.proto` author’s
responsibility not to use the same number on two variants: a Rust `enum`
cannot express `allow_alias`. proto2 unknown values already fail `TryFrom`;
proto3 uses `_None(i32)`. Switching to constants would change matching
ergonomics (`Color::Red` vs `Color::_None(n)`) and `Default` / conversion
traits.

Related existing TODO: `enum allow_alias` in the repository README.
