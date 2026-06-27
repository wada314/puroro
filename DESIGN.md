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
   - 4.8 [Required fields (proto2 only)](#48-required-fields-proto2-only)
   - 4.9 [Unknown fields](#49-unknown-fields)
5. [Allocator design](#5-allocator-design)
   - 5.1 [Chosen design: single type parameter](#51-chosen-design-single-type-parameter)
   - 5.2 [Alternative: per-field allocator type parameters](#52-alternative-per-field-allocator-type-parameters)
   - 5.3 [Alternative: dynamic dispatch (`Box<dyn Allocator>`)](#53-alternative-dynamic-dispatch-boxdyn-allocator)
   - 5.4 [Comparison summary](#54-comparison-summary)
6. [Proto syntax versions: proto2, proto3, and editions](#6-proto-syntax-versions-proto2-proto3-and-editions)
   - 6.1 [Wire format is version-agnostic](#61-wire-format-is-version-agnostic)
   - 6.2 [Field presence and optional scalars](#62-field-presence-and-optional-scalars)
   - 6.3 [Custom default values (proto2)](#63-custom-default-values-proto2)
   - 6.4 [Required fields (proto2)](#64-required-fields-proto2)
   - 6.5 [Closed vs open enums](#65-closed-vs-open-enums)
   - 6.6 [Packed repeated fields](#66-packed-repeated-fields)
   - 6.7 [Editions feature matrix](#67-editions-feature-matrix)
   - 6.8 [Extensions (not yet implemented)](#68-extensions-not-yet-implemented)
   - 6.9 [Comparison summary](#69-comparison-summary)
7. [Design decisions and trade-offs](#7-design-decisions-and-trade-offs)
8. [Future work](#8-future-work)

---

## 1. Goals

- **Protobuf spec compliance.** Support the canonical wire format (varints, I32, I64, LEN records, packed repeated, oneofs, unknown fields) for proto2, proto3, and editions. Group tags are deprecated and need not be generated, but the decoder must preserve them for round-trip fidelity.
- **Allocator support.** Every generated type is generic over `A: Allocator` using the `allocator-api2` crate. Arena allocators (e.g. `bumpalo`) and custom pools are first-class citizens.
- **Performance-oriented interface.** Accessors return borrowed references (`&str`, `&[u8]`, `&[T]`), never freshly allocated containers. The `encode_to_vec` / `encode_to_bytes` convenience methods allocate, but `encode_raw` does not.
- **Rust idioms.** Private fields accessed via generated accessor methods; `Option<&T>` for optional message fields; `Result<EnumType, i32>` for enum accessors; no `unsafe` in user-visible APIs.
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

The runtime library (`puroro`) exposes two core traits. Generated code depends only on these public items.

### `MessageEncode`

```rust
pub trait MessageEncode {
    /// Exact number of bytes this message occupies on the wire.
    /// Must be consistent with `encode_raw`.
    fn encoded_len(&self) -> usize;

    /// Writes the message body to `buf` without a framing length prefix.
    /// Panics or produces garbage if `buf` has insufficient capacity.
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

This section is the normative reference for what the code generator emits. Each subsection covers all syntax versions (proto2, proto3, editions). The two reference schemas used throughout are:

**proto3 reference** (`src/sample.rs` → `example`):

```protobuf
syntax = "proto3";
package example;

enum PhoneType { PHONE_TYPE_UNSPECIFIED = 0; MOBILE = 1; HOME = 2; WORK = 3; }

message Address { string street = 1; string city = 2; }

message Person {
    string          name               = 1;   // implicit presence
    int32           age                = 2;   // implicit presence
    bytes           avatar             = 3;
    repeated string emails             = 4;
    Address         address            = 5;
    PhoneType       primary_phone_type = 6;   // open enum
    oneof contact_method { string phone_number = 7; string fax_number = 8; }
}
```

**proto2 reference** (`src/sample.rs` → `example_proto2`):

```protobuf
syntax = "proto2";
package example;

enum Difficulty { EASY = 0; NORMAL = 1; HARD = 2; }  // closed enum

message PlayerConfig {
    required string     player_id  = 1;
    optional int32      score      = 2 [default = 100];
    optional bool       active     = 3;
    repeated int32      levels     = 4 [packed = false];
    optional Difficulty difficulty = 5;               // closed enum
}
```

---

### 4.1 Scalar fields

The Rust accessor type for a scalar field is the natural Rust equivalent of the proto type. The key variation across syntax versions is **field presence**.

#### Implicit presence (proto3 default / editions `IMPLICIT`)

- Value accessor: `fn age(&self) -> i32`
- Setter: `fn set_age(&mut self, v: i32)`
- Wire rule: field is absent from the wire when the value equals the type's zero (`0`, `false`, `0.0`).

#### Explicit presence (proto2 `optional` / proto3 `optional` keyword / editions `EXPLICIT`)

- Presence query: `fn has_score(&self) -> bool`
- Value accessor: `fn score(&self) -> i32` — returns the proto-declared default (or the type-zero if none) when `has_score()` is `false`
- Setter: `fn set_score(&mut self, v: i32)` — also sets `has_score()` to `true`
- Clearer: `fn clear_score(&mut self)` — sets `has_score()` to `false`
- Wire rule: field absent when `has_score()` is `false`; present even when the value is zero.

#### Scalar type mapping

| Proto type | Accessor return type | Wire type |
|---|---|---|
| `int32`, `sint32`, `sfixed32` | `i32` | VARINT / VARINT(ZigZag) / I32 |
| `int64`, `sint64`, `sfixed64` | `i64` | VARINT / VARINT(ZigZag) / I64 |
| `uint32`, `fixed32` | `u32` | VARINT / I32 |
| `uint64`, `fixed64` | `u64` | VARINT / I64 |
| `bool` | `bool` | VARINT |
| `float` | `f32` | I32 |
| `double` | `f64` | I64 |

All seven presence patterns (implicit vs explicit) apply uniformly across every entry in this table.

---

### 4.2 String fields

- Value accessor: `fn name(&self) -> &str`
- Setter: `fn set_name(&mut self, v: &str)` — copies the string data

The internal storage representation (e.g. `Box<str, A>`) is an implementation detail and may change. The accessor always returns a borrowed `&str`.

**Wire rules:**

| Presence mode | Absent from wire when |
|---|---|
| Implicit (proto3 default) | value is the empty string `""` |
| Explicit (`optional` / `optional` keyword) | `has_name()` is false |

For explicit-presence strings the same `has_X()` / `set_X()` / `clear_X()` pattern from §4.1 applies.

---

### 4.3 Bytes fields

- Value accessor: `fn avatar(&self) -> &[u8]`
- Setter: `fn set_avatar(&mut self, v: &[u8])` — copies the byte data

For explicit-presence bytes, `has_avatar()` / `clear_avatar()` are also generated. Wire omission follows the same rule as strings.

---

### 4.4 Repeated fields

Repeated fields expose a slice-like read API and append/clear mutation. The accessor returns a reference to a contiguous sequence, **not** a `Vec`, so callers get O(1) random access without allocation.

**Repeated string example (`repeated string emails = 4`):**

```rust
// Read — returns a slice whose elements deref to &str:
pub fn emails(&self) -> &[impl Deref<Target = str>];

// Write:
pub fn push_email(&mut self, v: &str);
pub fn clear_emails(&mut self);
```

Note: the concrete element type is an implementation detail; callers rely on the `Deref<Target = str>` bound and should not name the type directly.

**Repeated scalar example (`repeated int32 levels = 4`):**

```rust
pub fn levels(&self) -> &[i32];
pub fn push_level(&mut self, v: i32);
pub fn clear_levels(&mut self);
```

#### Packed vs non-packed encoding

The default packing behaviour depends on syntax version and field type:

| Syntax | Packable numeric `repeated` field | Default encoding |
|---|---|---|
| proto3 | Yes (all numeric types) | **Packed** (one LEN record for all elements) |
| proto2 | No (unless `[packed = true]`) | **Non-packed** (one record per element) |
| editions | Per-field `features.repeated_field_encoding` | `PACKED` or `EXPANDED` |

**Compatibility requirement (spec-mandated):** The decoder must accept *both* packed and non-packed forms for any packable repeated field, regardless of what the schema declares. This ensures forward- and backward-compatibility when the `packed` option changes between schema versions.

Repeated **string** and **message** fields cannot be packed; they always use one LEN record per element.

---

### 4.5 Nested message fields

Message fields are always optional in the generated struct (`None` = not present). The accessor returns `Option<&M<A>>`.

```rust
pub fn address(&self) -> Option<&Address<A>>;

/// Returns a mutable reference to the field, creating a default value if absent.
pub fn address_mut(&mut self) -> &mut Address<A>;

pub fn set_address(&mut self, v: Address<A>);
pub fn clear_address(&mut self);
```

**Merge semantics:** when the same message field appears more than once on the wire, occurrences are *merged* (not replaced). This is identical across all syntax versions and implements protobuf's "concatenated bytes = merged message" property.

---

### 4.6 Enum fields

The wire encoding is always VARINT. The variation is whether unknown numeric values are accepted.

#### Open enum (proto3 / editions `OPEN`)

Unknown numeric values are accepted and preserved. A raw accessor always succeeds; a typed accessor returns `Result`.

```rust
// Always available:
pub fn primary_phone_type_raw(&self) -> i32;
pub fn set_primary_phone_type_raw(&mut self, v: i32);

// Typed; returns Err(raw_value) for unknown values:
pub fn primary_phone_type(&self) -> Result<PhoneType, i32>;
pub fn set_primary_phone_type(&mut self, v: PhoneType);
```

Wire rule: field absent when the raw value is 0 (implicit presence).

#### Closed enum (proto2 / editions `CLOSED`)

Unknown numeric values are rejected and stored as unknown fields instead. The typed field may be absent even if the field was present on the wire (because the value was unknown).

```rust
// Typed; returns None if unset, Some(Ok(_)) for known values:
pub fn difficulty(&self) -> Option<Result<Difficulty, i32>>;
pub fn set_difficulty(&mut self, v: Difficulty);
pub fn clear_difficulty(&mut self);
```

Wire rule: field absent when not set (`None`); present even when the value is the zero variant.

#### Generated enum type

Both open and closed enums generate the same enum type:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PhoneType {
    Unspecified = 0,
    Mobile = 1,
    // …
}

impl TryFrom<i32> for PhoneType { type Error = i32; … }
impl From<PhoneType> for i32 { … }
```

---

### 4.7 Oneof fields

Each `oneof` group generates a Rust `enum` placed in a submodule named after the parent message (lower-snake-case). The parent message holds an `Option` of that enum.

```rust
// In module `person`:
pub enum ContactMethod<A: Allocator = Global> {
    PhoneNumber(/* owned string */),
    FaxNumber(/* owned string */),
}

// Accessors on Person:
pub fn contact_method(&self) -> Option<&person::ContactMethod<A>>;
pub fn contact_method_mut(&mut self) -> Option<&mut person::ContactMethod<A>>;
pub fn set_contact_method(&mut self, v: Option<person::ContactMethod<A>>);

// Per-variant convenience setters; each clears any previously set variant:
pub fn set_phone_number(&mut self, v: &str);
pub fn set_fax_number(&mut self, v: &str);
```

Setting any variant replaces the whole `Option`; the last field seen on the wire wins, which is correct for all syntax versions.

---

### 4.8 Required fields (proto2 only)

`required` fields exist only in proto2. On the wire they are indistinguishable from `optional` fields; the constraint is schema-level only.

The generated message provides a `validate()` method that checks all required fields are present, and a `decode_strict()` that combines decode and validation:

```rust
/// Returns Err if any required field was absent from the decoded wire data.
pub fn validate(&self) -> Result<(), DecodeError>;

/// Decodes and validates in one step.
pub fn decode_strict<B: Buf>(buf: B) -> Result<Self, DecodeError>
where
    Self: Default + MessageDecode;
```

The accessor for a required field returns `Option<&str>` (or `Option<T>`) — `None` means the field was not seen on the wire. `validate()` converts those `None` values into `Err(DecodeError::MissingRequiredField { field_number })`.

`MessageDecode::decode` does **not** call `validate()` automatically, keeping the trait generic across proto2 and proto3.

---

### 4.9 Unknown fields

All generated messages expose the raw bytes of any fields whose field numbers are not known to the current schema version:

```rust
pub fn unknown_fields(&self) -> &[u8];
```

These bytes are a valid (partial) protobuf wire stream and are re-emitted verbatim at the end of the message on encode, ensuring forward-compatibility round-trips. This behaviour is identical across all syntax versions.

**Disabling unknown-field preservation:** a future attribute (e.g. `#[puroro(no_unknown_fields)]`) would omit the unknown-fields buffer from a specific message type. Not yet implemented.

---

## 5. Allocator design

### 5.1 Chosen design: single type parameter

Every generated type carries a single allocator type parameter `A` that applies to all heap allocations within that message and its nested messages:

```rust
pub struct Person<A: Allocator = Global> { /* … */ }
```

`A` defaults to `Global`, so `Person` (without a type argument) behaves identically to a version without allocator support.

**Constructor API:**

```rust
impl<A: Allocator + Clone> Person<A> {
    /// Creates an empty message using the given allocator.
    pub fn new_in(alloc: A) -> Self;
}

impl Person<Global> {
    /// Creates an empty message using the global allocator.
    pub fn new() -> Self;
}

impl<A: Allocator + Clone + Default> Default for Person<A> { … }
```

**Allocator bound on mutation:** setter methods and `push_*` methods require `A: Clone` because they may create new heap values at call time. Read-only methods do not.

**Decode API:**

```rust
impl<A: Allocator + Clone + Default> MessageDecode for Person<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;
}
```

The `Default` bound is required because `MessageDecode::decode` calls `A::default()` to obtain the initial allocator.  Callers using a non-`Default` allocator must call `new_in(alloc)` followed by `merge_from` instead of calling `decode` directly.

**Arena allocator example:**

```rust
let bump = bumpalo::Bump::new();
let mut p = Person::new_in(&bump);
p.set_name("Alice");
// All allocations — name, emails, nested messages — land in `bump`.
// Dropping `bump` frees everything at once without individual Drop calls.
```

---

### 5.2 Alternative: per-field allocator type parameters

To allow each field to use a different allocator, every dynamically-sized field would require its own type parameter:

```rust
// Illustrative sketch — not the chosen design:
pub struct Person<A1 = Global, A2 = Global, A3 = Global, …>
where A1: Allocator, A2: Allocator, …
{
    name:    /* string stored with A1 */,
    emails:  /* each element with A2, container with A3 */,
    address: Option</* Address<???, …> stored with A4 */>,
    // address itself has its own type parameters …
}
```

**Problems that make this design impractical:**

1. **Combinatorial explosion.** A message with `N` heap-allocated fields needs up to `2N` type parameters; nested messages multiply this by their own count.
2. **No natural default allocator for setters.** With a single `A`, a setter can always use the stored allocator. With per-field parameters there is no obvious choice.
3. **Type inference failure.** Call sites require turbofish annotations for every parameter.
4. **Unwieldy function signatures.** Any function accepting a `Person<…>` must list all allocator parameters.

**Verdict:** Impractical.

---

### 5.3 Alternative: dynamic dispatch (`Box<dyn Allocator>`)

A shared reference-counted pointer is used in place of the type parameter:

```rust
// Illustrative sketch — not the chosen design:
pub struct Person {
    // all fields allocated through a shared Arc<dyn Allocator + Send + Sync>
}
```

**Problems:**

1. **Runtime overhead.** Every allocation goes through a virtual call.
2. **`Arc` cost.** Reference counting requires atomic operations.
3. **`Send + Sync` requirement.** Many useful arena allocators (e.g. `bumpalo::Bump`) are `!Sync`.
4. **Loss of monomorphisation.** The compiler cannot inline or specialise allocations.

**Verdict:** Viable as a concrete type alias for users who need runtime allocator dispatch, but not as the primary design.

---

### 5.4 Comparison summary

| Property | Single `A` (chosen) | Per-field type params | `dyn Allocator` |
|---|---|---|---|
| Number of type params | 1 | O(fields × depth) | 0 |
| Can mix allocators | No | Yes | Yes (runtime) |
| Zero-cost when using `Global` | Yes (ZST) | Yes | No (Arc overhead) |
| Arena allocator support | Yes | Yes | Partial (`!Sync` excluded) |
| Type inference ergonomics | Good | Poor | Good |
| Compile-time monomorphisation | Yes | Yes | No |
| Generated code complexity | Low | Very high | Low |

The single-`A` design is the clear practical choice.

---

## 6. Proto syntax versions: proto2, proto3, and editions

### 6.1 Wire format is version-agnostic

The wire format is **identical** across proto2, proto3, and editions. The runtime library (`MessageEncode`, `MessageDecode`, all helpers) requires no changes to support different syntax versions. All differences are in what the **code generator emits**.

### 6.2 Field presence and optional scalars

See **§4.1** for the complete accessor API.

| Syntax | Singular scalar | Presence mode | Omit from wire when |
|---|---|---|---|
| proto3 (default) | `fn age() -> i32` | Implicit | value == 0 |
| proto3 `optional` keyword | `fn age() -> i32` + `fn has_age() -> bool` | Explicit | `has_age()` false |
| proto2 `optional` | same as proto3 `optional` | Explicit | `has_age()` false |
| editions `IMPLICIT` | same as proto3 default | Implicit | value == 0 |
| editions `EXPLICIT` | same as proto3 `optional` | Explicit | `has_age()` false |

### 6.3 Custom default values (proto2)

proto2 allows `optional int32 score = 2 [default = 100];`.

The `score()` accessor returns `100` (the declared default) when `has_score()` is false. The default is a *read-time* substitution, not a stored value; `new_in()` initialises the field to the "not set" state, not to 100.

### 6.4 Required fields (proto2)

See **§4.8** for the complete accessor API (`validate()` + `decode_strict()`).

### 6.5 Closed vs open enums

See **§4.6** for the complete accessor API.

### 6.6 Packed repeated fields

See **§4.4** for the complete accessor API and the dual-form decode requirement.

### 6.7 Editions feature matrix

| Feature | Values | Effect on generated API |
|---|---|---|
| `field_presence` | `IMPLICIT` / `EXPLICIT` | Presence query methods generated or not |
| `repeated_field_encoding` | `PACKED` / `EXPANDED` | Affects encode format (decode always accepts both) |
| `enum_type` | `OPEN` / `CLOSED` | Determines open vs closed enum accessor pattern |
| `message_encoding` | `LENGTH_PREFIXED` / `DELIMITED` | `DELIMITED` (groups) is deprecated; not generated |
| `utf8_validation` | `VERIFY` / `NONE` | Whether decode errors on invalid UTF-8 strings |

Edition 2023 file-level defaults: `IMPLICIT`, `PACKED`, `OPEN`.

### 6.8 Extensions (not yet implemented)

proto2 `extensions` / `extend` blocks are **out of scope** for the current design. They require a fundamentally different storage mechanism: a message must hold an opaque, field-number-keyed store for extension fields unknown to the current compilation unit. This is architecturally more complex than the existing unknown-fields buffer, which is opaque and not keyed.

A future design would need to specify:
- An `ExtensionSet<A>` type with typed get/set accessors.
- A registration mechanism for extension descriptors (compile-time and/or runtime).
- A code-generation strategy for extension accessor files separate from the base message.

### 6.9 Comparison summary

| | Runtime change? | Generated API change |
|---|---|---|
| Explicit-presence scalar | None | `has_X()` / `clear_X()` methods added |
| Custom default value | None | `X()` returns `unwrap_or(declared_default)` |
| `required` field | `DecodeError::MissingRequiredField` added | `validate()` + `decode_strict()` added |
| Closed enum | `save_unknown_varint_field` helper added | Accessor returns `Option<Result<E, i32>>` |
| Non-packed repeated | None | Encode uses one-per-element loop |
| Editions features | None | Generator reads features, emits matching pattern |
| Extensions | **Significant** — new storage and accessor mechanism | Separate accessor generation |

---

## 7. Design decisions and trade-offs

### No zero-copy decode (for now)

Giving `string` and `bytes` fields a lifetime parameter would enable zero-copy decoding but would propagate that lifetime to every generated type and every callers. The current API is lifetime-free, which significantly simplifies usage. A future extension could introduce a borrowing "view" type alongside the existing owned type without breaking the current API.

### Accessor methods instead of public fields

Public struct fields are simpler but make it impossible to change internal representations without a breaking API change. Accessor methods add generated boilerplate but decouple the interface from the implementation — for example, string storage could change from one owned type to another, or scalar presence tracking could move from per-field `Option` to a per-message bitmask, without any change to the accessor API.

### Slice-like return type for repeated fields

Repeated field accessors return a reference to a contiguous sequence rather than a freshly allocated `Vec`. This allows O(1) random access without triggering allocation and matches the principle that `encode_raw` is zero-allocation. The concrete element type is intentionally not part of the stable API; callers use it through its `Deref` bound.

### `Default` bound on `MessageDecode::decode`

The provided `decode` method requires `Self: Default` to initialise the message before merging. The lower-level `merge_from` has no such requirement, which is important for callers using non-`Default` allocators (they call `new_in(alloc)` + `merge_from`).

---

## 8. Future work

- **Zero-copy decode.** Introduce borrowing view types (e.g. `PersonView<'buf>`) for string and bytes fields.
- **Unknown-field preservation opt-out.** A per-message attribute to omit the unknown-fields buffer where round-trip fidelity is not required.
- **Map fields.** Syntactic sugar for a `repeated` message with `key` and `value` fields; requires an allocator-aware map type (e.g. `hashbrown::HashMap<K, V, S, A>`).
- **Service / RPC definitions.** Out of scope for the runtime library.
- **Well-known types.** `google.protobuf.Timestamp`, `Duration`, `Any`, etc.
- **Reflection / descriptors.** Runtime introspection of message schema.
- **Extensions.** See §6.8.
