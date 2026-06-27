# puroro — Design Document

This document specifies the **public interface** of the `puroro` Protocol Buffers runtime library and the accessor API that the code generator must emit. It intentionally omits internal implementation details — storage types, encode/decode algorithms, and runtime helper usage are documented in [IMPLEMENTATION.md](IMPLEMENTATION.md).

## Table of contents

1. [Goals](#1-goals)
2. [Wire format overview](#2-wire-format-overview)
3. [Runtime trait API](#3-runtime-trait-api)
4. [Generated code specification](#4-generated-code-specification)
   - 4.1 [Scalar fields](#41-scalar-fields) — implicit vs explicit presence
   - 4.2 [String fields](#42-string-fields)
   - 4.3 [Bytes fields](#43-bytes-fields)
   - 4.4 [Repeated fields](#44-repeated-fields) — packed vs non-packed, dual-form decode
   - 4.5 [Nested message fields](#45-nested-message-fields)
   - 4.6 [Enum fields](#46-enum-fields) — open vs closed
   - 4.7 [Oneof fields](#47-oneof-fields)
   - 4.8 [Required fields (`LEGACY_REQUIRED`)](#48-required-fields-legacy_required)
   - 4.9 [Unknown fields](#49-unknown-fields)
5. [Allocator design](#5-allocator-design)
   - 5.1 [Chosen design: single type parameter](#51-chosen-design-single-type-parameter)
   - 5.2 [Alternative: per-field allocator type parameters](#52-alternative-per-field-allocator-type-parameters)
   - 5.3 [Alternative: dynamic dispatch (`Box<dyn Allocator>`)](#53-alternative-dynamic-dispatch-boxdyn-allocator)
   - 5.4 [Comparison summary](#54-comparison-summary)
6. [Proto syntax versions: proto2, proto3, and editions](#6-proto-syntax-versions-proto2-proto3-and-editions)
   - 6.1 [Wire format is version-agnostic](#61-wire-format-is-version-agnostic)
   - 6.2 [Editions as the unified syntax](#62-editions-as-the-unified-syntax)
   - 6.3 [Editions feature matrix](#63-editions-feature-matrix)
   - 6.4 [Migration from proto2 / proto3](#64-migration-from-proto2--proto3)
   - 6.5 [Extensions (not yet implemented)](#65-extensions-not-yet-implemented)
7. [Design decisions and trade-offs](#7-design-decisions-and-trade-offs)
8. [Future work](#8-future-work)

---

## 1. Goals

- **Protobuf spec compliance.** Support the canonical wire format (varints, I32, I64, LEN records, packed repeated, oneofs, unknown fields) for proto2, proto3, and editions. Group tags are deprecated and need not be generated, but the decoder must preserve them for round-trip fidelity.
- **Allocator support.** Every generated type is generic over `A: Allocator` using the `allocator-api2` crate. Arena allocators (e.g. `bumpalo`) and custom pools are first-class citizens.
- **Performance-oriented interface.** Accessors return borrowed references (`&str`, `&[u8]`, `&[T]`), never freshly allocated containers. The `encode_to_vec` / `encode_to_bytes` convenience methods allocate, but `encode_raw` does not.
- **Rust idioms.** Private fields accessed via generated accessor methods; `Optional<T, impl HasDefault<T>>` for explicit-presence scalar and string fields; `Option<&M<A>>` for optional message fields; `Result<EnumType, i32>` for enum accessors; no `unsafe` in user-visible APIs.
- **Implementation flexibility.** The public interface described here must remain stable even if internal storage representations change. For example, presence tracking could use `Option<T>` fields or a per-message bitmask; the accessor API is the same either way.
- **Nightly toolchain, minimal unstable features.** The `rust-toolchain.toml` pins nightly; no `#![feature(…)]` flags are used in this crate itself.

---

## 2. Wire format overview

See the [official encoding reference](https://protobuf.dev/programming-guides/encoding/) for full details. The essentials:

```
message    := (tag value)*
tag        := varint   where lower 3 bits = wire_type, upper bits = field_number
value      := varint         for wire_type 0 (VARINT)
            | 8 bytes LE     for wire_type 1 (I64)
            | len + payload  for wire_type 2 (LEN)
            | 4 bytes LE     for wire_type 5 (I32)
```

Wire types 3 and 4 (SGROUP / EGROUP) are deprecated. The decoder must skip them; the encoder must never emit them.

| Protobuf type | Wire type |
|---|---|
| `int32`, `int64`, `uint32`, `uint64`, `sint32`, `sint64`, `bool`, `enum` | VARINT |
| `fixed32`, `sfixed32`, `float` | I32 |
| `fixed64`, `sfixed64`, `double` | I64 |
| `string`, `bytes`, embedded messages, packed repeated | LEN |

---

## 3. Runtime trait API

The runtime library (`puroro`) exposes two core traits and one accessor trait. Generated code depends only on these public items.

### `HasDefault` and `Optional`

```rust
/// Implemented by a zero-sized struct that carries a compile-time default.
/// The implementing struct is defined locally inside each accessor method.
pub trait HasDefault<T: Copy> {
    const DEFAULT: T;
}

/// Concrete return type for every explicit-presence field accessor.
/// `T` — Copy scalar or reference (`&'a str`, `&'a [u8]`).
/// `D` — zero-sized default provider; hidden behind `impl HasDefault<T>`.
pub struct Optional<T: Copy, D: HasDefault<T>> { … }

impl<T: Copy, D: HasDefault<T>> Optional<T, D> {
    pub fn new(value: Option<T>, _tag: D) -> Self;
    pub fn get(&self) -> T;            // value or proto-declared default
    pub fn get_opt(&self) -> Option<T>; // None when not set
    pub fn is_set(&self) -> bool;
}
```

`Optional` is a **concrete struct** (not a trait), so the borrow checker can always verify its trivial drop — enabling direct chaining for both scalar and string accessors.

The concrete `D` type is a private zero-sized struct defined locally inside the accessor method body.  The return type in generated code uses `impl HasDefault<T>` in the second type-parameter position to keep `D` opaque:

```rust
pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> { … }
pub fn title<'s>(&'s self) -> Optional<&'s str, impl HasDefault<&'s str>> { … }

// Both chain directly without a let binding:
let n: i32  = task.max_retries().get();
let s: &str = task.title().get();
```

### `MessageEncode`

```rust
pub trait MessageEncode {
    /// Exact number of bytes this message occupies on the wire.
    /// Must be consistent with `encode_raw`.
    fn encoded_len(&self) -> usize;

    /// Writes the message body to `buf` without a framing length prefix.
    fn encode_raw<B: bytes::BufMut>(&self, buf: &mut B);

    // Provided convenience methods:
    fn encode_to_vec(&self) -> Vec<u8>;
    fn encode_to_bytes(&self) -> bytes::Bytes;
}
```

`encode_raw` is generic over `B: BufMut` so the compiler can monomorphise and inline field writes. This makes the trait **non-object-safe** by design; trait objects are not a target use case.

### `MessageDecode`

```rust
pub trait MessageDecode: Sized {
    /// Reads fields from `buf` and merges them into `self`.
    ///
    /// Merge semantics (identical across proto2, proto3, editions):
    /// - Singular scalar: last value seen wins.
    /// - Singular message: recursively merged.
    /// - Repeated: each occurrence appends to the list.
    /// - Unknown fields: accumulated for round-trip preservation.
    fn merge_from<B: bytes::Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    /// Decodes a complete message. Provided; requires `Self: Default`.
    fn decode<B: bytes::Buf>(buf: B) -> Result<Self, DecodeError>
    where
        Self: Default;
}
```

The primary operation is *merge*, not *decode-from-scratch*. `decode` is a convenience wrapper.

---

## 4. Generated code specification

This section is the normative reference for what the code generator emits. All field patterns are illustrated using a single **editions** reference schema, since editions can express every variant (implicit/explicit presence, custom defaults, required-like semantics, open/closed enums, packed/expanded repeated) in one file.

### Reference schema

```protobuf
edition = "2024";
package example;

// Edition 2024 file-level defaults:
//   field_presence         = EXPLICIT  (presence tracked for all singular fields)
//   repeated_field_encoding = PACKED   (numeric repeated fields are packed)
//   enum_type              = OPEN      (unknown enum values are accepted)

// ── Enums ──────────────────────────────────────────────────────────────────

// Open enum (edition default): unknown values stored in the typed field.
enum Status {
    STATUS_UNSPECIFIED = 0;
    PENDING = 1;
    DONE    = 2;
}

// Closed enum: unknown values diverted to unknown fields.
enum Priority {
    option features.enum_type = CLOSED;
    PRIORITY_UNSPECIFIED = 0;
    LOW  = 1;
    HIGH = 2;
}

// ── Messages ───────────────────────────────────────────────────────────────

message Address {
    string street = 1;   // EXPLICIT presence (edition default)
    string city   = 2;
}

message Task {
    // Field 1: EXPLICIT presence (edition 2024 default) — tracks whether set
    string title = 1;

    // Field 2: IMPLICIT presence — omit when zero, like proto3 scalars
    int32 score = 2 [features.field_presence = IMPLICIT];

    // Field 3: EXPLICIT presence with custom default value
    int32 max_retries = 3 [default = 3];

    // Field 4: LEGACY_REQUIRED — must be present; validated via validate()
    string owner_id = 4 [features.field_presence = LEGACY_REQUIRED];

    // Field 5: bytes (EXPLICIT presence)
    bytes payload = 5;

    // Field 6: Packed repeated int32 (PACKED is the edition default)
    repeated int32 tag_ids = 6;

    // Field 7: Non-packed repeated int32 (explicitly EXPANDED)
    repeated int32 scores = 7 [features.repeated_field_encoding = EXPANDED];

    // Field 8: Repeated string (strings cannot be packed; EXPANDED implicitly)
    repeated string labels = 8;

    // Field 9: Open enum, IMPLICIT presence
    Status status = 9 [features.field_presence = IMPLICIT];

    // Field 10: Closed enum (EXPLICIT presence)
    Priority priority = 10;

    // Field 11: Nested message (EXPLICIT presence)
    Address assignee = 11;

    // Fields 12–13: Oneof (presence is intrinsic to oneof)
    oneof notification {
        string email_address = 12;
        string phone_number  = 13;
    }
}
```

The subsections below document each field pattern in terms of its generated accessor API.

---

### 4.1 Scalar fields

Scalar types are copy types in Rust (`i32`, `bool`, `f32`, …). The key variation is **field presence**.

#### Implicit presence (`features.field_presence = IMPLICIT`)

Equivalent to proto3's default singular scalar behaviour.

- Value accessor: `fn score(&self) -> i32`
- Setter: `fn set_score(&mut self, v: i32)`
- Wire rule: field absent from the wire when value equals the type-zero (`0`, `false`, `0.0`).

#### Explicit presence (`features.field_presence = EXPLICIT`, edition 2024 default)

Equivalent to proto2 `optional`. Presence is tracked independently of value.
Three accessors are generated:

| Method | Return type | Description |
|---|---|---|
| `max_retries()` | `Optional<i32, impl HasDefault<i32>>` | Rich view: `get()`, `get_opt()`, `is_set()` |
| `max_retries_raw()` | `i32` | Direct value with default applied; no wrapper |
| `has_max_retries()` | `bool` | Presence check |

- Setter: `fn set_max_retries(&mut self, v: i32)` — makes `has_max_retries()` true
- Clearer: `fn clear_max_retries(&mut self)` — makes `has_max_retries()` false
- Wire rule: field absent when `has_max_retries()` is `false`; present even when the value is zero.

The `[default = 3]` option means `max_retries().get()` and `max_retries_raw()` return `3` when unset.

Because `Optional` is a concrete struct with no custom `Drop`, chaining compiles directly:

```rust
let n: i32 = task.max_retries().get();   // ok
if task.max_retries().is_set() { … }     // ok
match task.max_retries().get_opt() { … } // ok
let n: i32 = task.max_retries_raw();     // ok — bypasses Optional
```

#### Scalar type mapping

| Proto type | Accessor return type (implicit/explicit) | Wire type |
|---|---|---|
| `int32`, `sint32`, `sfixed32` | `i32` | VARINT / VARINT(ZigZag) / I32 |
| `int64`, `sint64`, `sfixed64` | `i64` | VARINT / VARINT(ZigZag) / I64 |
| `uint32`, `fixed32` | `u32` | VARINT / I32 |
| `uint64`, `fixed64` | `u64` | VARINT / I64 |
| `bool` | `bool` | VARINT |
| `float` | `f32` | I32 |
| `double` | `f64` | I64 |

Both presence modes apply uniformly across every entry in this table.

---

### 4.2 String fields

String fields always yield a borrowed `&str`. The internal storage type is an implementation detail.

**Implicit presence:**

- Accessor: `fn name(&self) -> &str` — returns `""` when not set
- Setter: `fn set_name(&mut self, v: &str)`

**Explicit presence (three accessors):**

| Method | Return type | Description |
|---|---|---|
| `title()` | `Optional<&'s str, impl HasDefault<&'s str>>` | Rich view: `get()`, `get_opt()`, `is_set()` |
| `title_raw()` | `&str` | Direct `&str` with default applied |
| `has_title()` | `bool` | Presence check |

Because `Optional` is a concrete struct, chaining compiles directly for string fields too:

```rust
let s: &str = task.title().get();          // ok
if task.title().is_set() { … }             // ok
let s: &str = task.title_raw();            // ok — bypasses Optional
```

Wire rule: absent when `has_title()` is false (EXPLICIT) or `""` (IMPLICIT).

---

### 4.3 Bytes fields

Bytes fields follow the same three-accessor pattern as strings, with `&[u8]` as the value type instead of `&str`.

**Explicit presence:**

- `payload()` → `Optional<&'s [u8], impl HasDefault<&'s [u8]>>` (`.get()`, `.get_opt()`, `.is_set()`)
- `payload_raw()` → `&[u8]` (direct, with default applied)
- `has_payload()` → `bool`

All three chain directly, same as string fields.

---

### 4.4 Repeated fields

Repeated fields expose a slice-like read API and append/clear mutation. The accessor returns a reference to a contiguous sequence, **not** a `Vec`, so callers get O(1) random access without allocation.

**Packed repeated scalar (`tag_ids: repeated int32`, field 6):**

```rust
pub fn tag_ids(&self) -> &[i32];
pub fn push_tag_id(&mut self, v: i32);
pub fn clear_tag_ids(&mut self);
```

**Non-packed repeated scalar (`scores: repeated int32 [EXPANDED]`, field 7):**

```rust
pub fn scores(&self) -> &[i32];
pub fn push_score(&mut self, v: i32);
pub fn clear_scores(&mut self);
```

The accessor API is identical for packed and non-packed; the difference is only in the wire encoding.

**Repeated string (`labels: repeated string`, field 8):**

```rust
// Returns a slice whose elements deref to &str:
pub fn labels(&self) -> &[impl Deref<Target = str>];
pub fn push_label(&mut self, v: &str);
pub fn clear_labels(&mut self);
```

The concrete element type is an implementation detail; callers rely on the `Deref<Target = str>` bound.

#### Packed vs non-packed encoding

| Feature setting | Wire encoding |
|---|---|
| `PACKED` (edition 2024 default for numeric fields) | One LEN record containing all element values |
| `EXPANDED` | One VARINT/I32/I64 record per element |

**Compatibility requirement (spec-mandated):** The decoder must accept *both* packed and non-packed forms for any packable repeated field, regardless of what the schema declares.

String and message fields cannot be packed; they always use one LEN record per element.

---

### 4.5 Nested message fields

Message fields are always optional in the generated struct (`None` = not present). The accessor returns `Option<&M<A>>`.

```rust
pub fn assignee(&self) -> Option<&Address<A>>;

/// Returns a mutable reference, creating a default value if absent.
pub fn assignee_mut(&mut self) -> &mut Address<A>;

pub fn set_assignee(&mut self, v: Address<A>);
pub fn clear_assignee(&mut self);
```

**Merge semantics:** when the same message field appears more than once on the wire, occurrences are *merged* rather than replaced. This implements protobuf's "concatenated bytes = merged message" property.

---

### 4.6 Enum fields

The wire encoding is always VARINT. The variation is whether unknown numeric values are accepted.

#### Open enum (`enum_type = OPEN`, edition 2024 default)

Unknown numeric values are stored in the typed field. A raw accessor always succeeds; a typed accessor returns `Result`.

```rust
// Always available:
pub fn status_raw(&self) -> i32;
pub fn set_status_raw(&mut self, v: i32);

// Typed; Err(raw_value) for unknown values:
pub fn status(&self) -> Result<Status, i32>;
pub fn set_status(&mut self, v: Status);
```

Wire rule: field absent when the raw value is 0 (IMPLICIT presence).

#### Closed enum (`enum_type = CLOSED`)

Unknown numeric values are diverted to unknown fields. The typed field may be absent even when the field was on the wire (unknown value was encountered).

```rust
// Typed; None if unset, Some(Ok(_)) for known values, Some(Err(raw)) for unknown:
pub fn priority(&self) -> Option<Result<Priority, i32>>;
pub fn set_priority(&mut self, v: Priority);
pub fn clear_priority(&mut self);
```

Wire rule: field absent when not set; present even for the zero variant.

#### Generated enum type

Both open and closed enums produce the same enum definition:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Status { Unspecified = 0, Pending = 1, Done = 2 }

impl TryFrom<i32> for Status { type Error = i32; … }
impl From<Status> for i32 { … }
```

---

### 4.7 Oneof fields

Each `oneof` group generates a Rust `enum` in a submodule named after the parent message (lower-snake-case). The parent message holds an `Option` of that enum.

```rust
// In module `task`:
pub enum Notification<A: Allocator = Global> {
    EmailAddress(/* owned string */),
    PhoneNumber(/* owned string */),
}

// Accessors on Task:
pub fn notification(&self) -> Option<&task::Notification<A>>;
pub fn notification_mut(&mut self) -> Option<&mut task::Notification<A>>;
pub fn set_notification(&mut self, v: Option<task::Notification<A>>);

// Per-variant convenience setters; each clears any previously set variant:
pub fn set_email_address(&mut self, v: &str);
pub fn set_phone_number(&mut self, v: &str);
```

Setting any variant replaces the whole `Option`; the last field seen on the wire wins.

---

### 4.8 Required fields (`LEGACY_REQUIRED`)

`features.field_presence = LEGACY_REQUIRED` corresponds to proto2's `required` modifier. It is intended as a **migration path** for existing proto2 schemas and is not recommended for new fields.

On the wire, a `LEGACY_REQUIRED` field is indistinguishable from an `EXPLICIT` field; the constraint is schema-level only.

**Accessor API** — same three-accessor pattern as `EXPLICIT`, with two additional methods:

```rust
// Same as EXPLICIT:
pub fn owner_id<'s>(&'s self) -> Optional<&'s str, impl HasDefault<&'s str>>;
pub fn owner_id_raw(&self) -> &str;
pub fn has_owner_id(&self) -> bool;
pub fn set_owner_id(&mut self, v: &str);
pub fn clear_owner_id(&mut self);

// Additional: required-field validation
pub fn validate(&self) -> Result<(), DecodeError>;
pub fn decode_strict<B: Buf>(buf: B) -> Result<Self, DecodeError>
where
    Self: Default + MessageDecode;
```

When a `LEGACY_REQUIRED` field was absent from the wire, `owner_id().is_set()` is `false` and `validate()` returns `Err(DecodeError::MissingRequiredField { field_number })`.

`MessageDecode::decode` does **not** call `validate()` automatically.

---

### 4.9 Unknown fields

All generated messages expose the raw bytes of any unrecognised fields:

```rust
pub fn unknown_fields(&self) -> &[u8];
```

These bytes form a valid partial protobuf stream and are re-emitted verbatim at the end of the message on encode, ensuring forward-compatibility round-trips.

**Disabling unknown-field preservation:** a future attribute (e.g. `#[puroro(no_unknown_fields)]`) would omit the unknown-fields buffer. Not yet implemented.

---

## 5. Allocator design

### 5.1 Chosen design: single type parameter

Every generated type carries a single allocator type parameter `A` that applies to all heap allocations within that message and its nested messages:

```rust
pub struct Task<A: Allocator = Global> { /* … */ }
```

`A` defaults to `Global`, so `Task` (without a type argument) works identically to a version without allocator support.

**Constructor API:**

```rust
impl<A: Allocator + Clone> Task<A> {
    /// Creates an empty message using the given allocator.
    pub fn new_in(alloc: A) -> Self;
}

impl Task<Global> {
    pub fn new() -> Self;
}

impl<A: Allocator + Clone + Default> Default for Task<A> { … }
```

**Allocator bound on mutation:** setter methods and `push_*` methods require `A: Clone` because they may create new heap values at call time. Read-only methods do not.

**Decode API:**

```rust
impl<A: Allocator + Clone + Default> MessageDecode for Task<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;
}
```

The `Default` bound is required because `MessageDecode::decode` calls `A::default()` to obtain the initial allocator.  Callers using a non-`Default` allocator must call `new_in(alloc)` followed by `merge_from` instead of `decode`.

**Arena allocator example:**

```rust
let bump = bumpalo::Bump::new();
let mut t = Task::new_in(&bump);
t.set_title("Fix bug");
// All allocations (title, labels, nested messages) land in `bump`.
// Dropping `bump` frees everything at once without individual Drop calls.
```

---

### 5.2 Alternative: per-field allocator type parameters

To allow each field to use a different allocator, every dynamically-sized field would require its own type parameter:

```rust
// Illustrative sketch — not the chosen design:
pub struct Task<A1 = Global, A2 = Global, A3 = Global, …>
where A1: Allocator, A2: Allocator, …
{ … }
```

**Problems:**

1. **Combinatorial explosion.** A message with `N` heap-allocated fields needs up to `2N` type parameters; nested messages multiply this.
2. **No natural default allocator for setters.**
3. **Type inference failure** at call sites.
4. **Unwieldy function signatures.**

**Verdict:** Impractical.

---

### 5.3 Alternative: dynamic dispatch (`Box<dyn Allocator>`)

```rust
// Illustrative sketch — not the chosen design:
pub struct Task {
    // all fields allocated through Arc<dyn Allocator + Send + Sync>
}
```

**Problems:**

1. **Runtime overhead** — every allocation goes through a virtual call.
2. **`Arc` cost** — atomic reference counting.
3. **`Send + Sync` requirement** excludes arena allocators like `bumpalo::Bump` (`!Sync`).
4. **Loss of monomorphisation.**

**Verdict:** Viable as a type alias for users who need runtime dispatch, but not as the primary design.

---

### 5.4 Comparison summary

| Property | Single `A` (chosen) | Per-field type params | `dyn Allocator` |
|---|---|---|---|
| Number of type params | 1 | O(fields × depth) | 0 |
| Can mix allocators | No | Yes | Yes (runtime) |
| Zero-cost for `Global` | Yes (ZST) | Yes | No (Arc overhead) |
| Arena allocator support | Yes | Yes | Partial (`!Sync` excluded) |
| Type inference ergonomics | Good | Poor | Good |
| Compile-time monomorphisation | Yes | Yes | No |
| Generated code complexity | Low | Very high | Low |

---

## 6. Proto syntax versions: proto2, proto3, and editions

### 6.1 Wire format is version-agnostic

The wire format is **identical** across proto2, proto3, and editions. The runtime library (`MessageEncode`, `MessageDecode`, all helpers) requires no changes to support different syntax versions. All differences are in what the **code generator emits**.

### 6.2 Editions as the unified syntax

Editions (edition 2023 onward) replace the proto2/proto3 binary choice with per-field feature flags. They can express every behaviour that proto2 and proto3 offered:

| proto2 / proto3 concept | Editions equivalent |
|---|---|
| proto3 implicit-presence scalar | `[features.field_presence = IMPLICIT]` |
| proto2 `optional` | `[features.field_presence = EXPLICIT]` (edition 2024 default) |
| proto2 `[default = 100]` | `[default = 100]` (supported unchanged) |
| proto2 `required` | `[features.field_presence = LEGACY_REQUIRED]` (migration only) |
| proto2 non-packed repeated | `[features.repeated_field_encoding = EXPANDED]` |
| proto3 packed repeated | `[features.repeated_field_encoding = PACKED]` (edition 2024 default) |
| proto3 open enum | `features.enum_type = OPEN` (edition 2024 default) |
| proto2 closed enum | `option features.enum_type = CLOSED;` inside the enum |

Note: **edition 2024 defaults to `EXPLICIT` field presence**, which is the opposite of proto3's default. A proto3-style implicit-presence scalar must be annotated with `[features.field_presence = IMPLICIT]` in an editions file.

### 6.3 Editions feature matrix

| Feature | Values | Effect on generated API |
|---|---|---|
| `field_presence` | `IMPLICIT` / `EXPLICIT` / `LEGACY_REQUIRED` | Presence query methods generated or not; `validate()` for `LEGACY_REQUIRED` |
| `repeated_field_encoding` | `PACKED` / `EXPANDED` | Affects encode format (decode always accepts both) |
| `enum_type` | `OPEN` / `CLOSED` | Accessor returns `Result<E, i32>` vs `Option<Result<E, i32>>` |
| `message_encoding` | `LENGTH_PREFIXED` / `DELIMITED` | `DELIMITED` (groups) is deprecated; not generated |
| `utf8_validation` | `VERIFY` / `NONE` | Whether decode errors on invalid UTF-8 strings |

### 6.4 Migration from proto2 / proto3

Editions are a **superset** of both proto2 and proto3: any valid proto2 or proto3 schema can be mechanically translated into an equivalent editions schema (the official Prototiller tool does this). The wire format is unchanged by such a migration.

The generated Rust accessor API after migration is identical — the same accessor names, presence semantics, and enum handling — because the generated code pattern is determined by the field's feature settings, not by whether the source was proto2, proto3, or editions.

### 6.5 Extensions (not yet implemented)

proto2 `extensions` / `extend` blocks are **out of scope** for the current design. They require a fundamentally different storage mechanism: a message must hold an opaque, field-number-keyed store for extension fields not known to the current compilation unit.

A future design would need:
- An `ExtensionSet<A>` type with typed get/set accessors.
- A registration mechanism for extension descriptors.
- A code-generation strategy for extension accessors separate from the base message.

---

## 7. Design decisions and trade-offs

### No zero-copy decode (for now)

Giving `string` and `bytes` fields a lifetime parameter would enable zero-copy decoding but would propagate that lifetime to every generated type and its callers. The current API is lifetime-free, which significantly simplifies usage. A future extension could introduce a borrowing "view" type alongside the existing owned type without breaking the current API.

### Accessor methods instead of public fields

Public struct fields are simpler but prevent changing internal representations without a breaking API change. Accessor methods decouple the interface from the implementation — for example, presence tracking could move from per-field `Option<T>` to a per-message bitmask without any change to the accessor signatures.

### `Optional<T, impl HasDefault<T>>` as a concrete struct

Explicit-presence field accessors return `Optional<T, D>` — a concrete struct rather than a trait or `Option<T>` directly.  Key benefits of this design:

- **Default value as compile-time constant.** `D::DEFAULT` is a `const` expression, so the "return default when not set" branch has zero runtime overhead.
- **No RPIT drop-check restriction.** Because the concrete struct's `Drop` is trivially visible to the borrow checker, `task.title().get()` chains directly without a `let` binding — even for string fields that return `&str`.
- **Private default provider.** The `D` type is a zero-sized struct defined locally inside the method body, then hidden behind `impl HasDefault<T>` in the return type.  Callers never need to name it.
- **String defaults without unstable features.** `&'static str` cannot currently be a `const` generic parameter, but `const DEFAULT: &'a str` in a blanket `impl<'a> HasDefault<&'a str>` is fully stable on nightly.

### Slice-like return type for repeated fields

Repeated field accessors return a reference to a contiguous sequence rather than a freshly allocated `Vec`. This allows O(1) random access without triggering allocation. The concrete element type is intentionally not part of the stable API.

### `Default` bound on `MessageDecode::decode`

The provided `decode` method requires `Self: Default`. The lower-level `merge_from` has no such requirement, which is important for callers using non-`Default` allocators.

---

## 8. Future work

- **Zero-copy decode.** Borrowing view types (e.g. `TaskView<'buf>`) for string and bytes fields.
- **Unknown-field preservation opt-out.** A per-message attribute to omit the unknown-fields buffer.
- **Map fields.** Syntactic sugar for a repeated message entry; requires an allocator-aware map type.
- **Service / RPC definitions.** Out of scope for the runtime library.
- **Well-known types.** `google.protobuf.Timestamp`, `Duration`, `Any`, etc.
- **Reflection / descriptors.** Runtime introspection of message schema.
- **Extensions.** See §6.5.
