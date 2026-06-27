# puroro — Design Document

This document describes the design of the `puroro` Protocol Buffers runtime library and specifies what code the future code generator should emit.

## Table of contents

1. [Goals](#1-goals)
2. [Wire format overview](#2-wire-format-overview)
3. [Runtime trait API](#3-runtime-trait-api)
4. [Generated code specification](#4-generated-code-specification)
   - 4.1 [Scalar fields](#41-scalar-fields) — implicit vs explicit presence, all numeric types
   - 4.2 [String fields](#42-string-fields)
   - 4.3 [Bytes fields](#43-bytes-fields)
   - 4.4 [Repeated fields](#44-repeated-fields) — packed vs non-packed, dual-form decode
   - 4.5 [Nested message fields](#45-nested-message-fields)
   - 4.6 [Enum fields](#46-enum-fields) — open (proto3) vs closed (proto2)
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

- **Protobuf spec compliance.** Implement the canonical protobuf3 wire format (varints, I32, I64, LEN records, packed repeated, oneofs, unknown fields). Group tags are deprecated and need not be generated, but the decoder preserves them as unknown bytes.
- **Allocator support.** Every generated type is generic over `A: Allocator` using the `allocator-api2` crate. Arena allocators (e.g. `bumpalo`) and custom pools are first-class citizens.
- **Performance-oriented interface.** Accessors return `&T` or `&[T]`, never a freshly allocated `Vec<T>`. Encoding is zero-allocation for `encode_raw`; a separate `encode_to_vec` helper allocates lazily.
- **Rust idioms.** Private fields + accessor methods, `Option<T>` for optional message fields, `Result<EnumType, i32>` for enum accessors, no `unsafe` in user-visible APIs.
- **Nightly toolchain, minimal unstable features.** The `rust-toolchain.toml` pins nightly; however, no `#![feature(…)]` flags are used in this crate itself. `allocator-api2` mirrors the allocator API without requiring the unstable flag.

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

Wire types 3 and 4 (SGROUP / EGROUP) are deprecated. The decoder must skip them but the encoder must never emit them.

| Protobuf type | Wire type | Rust storage type |
|---|---|---|
| `int32`, `int64` | VARINT | `i32`, `i64` |
| `uint32`, `uint64` | VARINT | `u32`, `u64` |
| `sint32`, `sint64` | VARINT (ZigZag) | `i32`, `i64` |
| `bool` | VARINT | `bool` |
| `enum` | VARINT | `i32` (open enum) |
| `fixed32`, `sfixed32`, `float` | I32 | `u32`, `i32`, `f32` |
| `fixed64`, `sfixed64`, `double` | I64 | `u64`, `i64`, `f64` |
| `string` | LEN | `Box<str, A>` |
| `bytes` | LEN | `Vec<u8, A>` |
| message | LEN | `Option<Box<M<A>, A>>` |
| repeated (packable) | LEN (packed) | `Vec<T, A>` |
| repeated message / string | LEN (one per element) | `Vec<…, A>` |

---

## 3. Runtime trait API

The runtime library (`puroro`) exposes two core traits and helpers. Generated code depends only on these public items.

### `MessageEncode`

```rust
pub trait MessageEncode {
    /// Exact byte length on the wire.  Must agree with `encode_raw`.
    fn encoded_len(&self) -> usize;

    /// Writes the message body to `buf` without a length prefix.
    fn encode_raw<B: bytes::BufMut>(&self, buf: &mut B);

    // Provided:
    fn encode_to_vec(&self) -> Vec<u8>;
    fn encode_to_bytes(&self) -> bytes::Bytes;
}
```

`encode_raw` is generic over `B: BufMut` so the compiler can monomorphise and inline field writes. This makes the trait **non-object-safe** by design; trait objects are not a target use case.

### `MessageDecode`

```rust
pub trait MessageDecode: Sized {
    /// Reads wire fields from `buf` and merges them into `self`.
    fn merge_from<B: bytes::Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    // Provided (requires Self: Default):
    fn decode<B: bytes::Buf>(buf: B) -> Result<Self, DecodeError>
    where
        Self: Default;
}
```

The primary operation is *merge*, not *decode-from-scratch*. This matches protobuf3 semantics:
- Singular scalar: last value wins.
- Singular message: recursively merged.
- Repeated: each occurrence appends.

`decode` is a provided method available only when `Self: Default`.

---

## 4. Generated code specification

This section is the normative reference for what the code generator emits.  Each subsection covers the pattern for **all syntax versions** (proto3, proto2, and editions).  The two reference schemas used throughout are:

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
    repeated string emails             = 4;   // packed not applicable (string)
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
    repeated int32      levels     = 4 [packed = false];  // non-packed
    optional Difficulty difficulty = 5;                   // closed enum
}
```

The subsections below document all field patterns, noting where proto2 or editions introduce a different variant.

---

### 4.1 Scalar fields

Scalar types are copy types in Rust (`i32`, `bool`, `f32`, …).  The key variation across syntax versions is **field presence**: whether the field tracks "has been explicitly set" or just holds a value.

#### Implicit presence (proto3 default / editions `IMPLICIT`)

The field is stored as a plain `T`.  A value equal to the language default (0 / `false` / 0.0) is treated as "not set" and **omitted from the wire**.

```rust
// Proto:  int32 age = 2;
age: i32,

pub fn age(&self) -> i32 { self.age }
pub fn set_age(&mut self, v: i32) { self.age = v; }
```

Encode:
```rust
if self.age != 0 {
    encode_varint_field(2, self.age as u64, buf);
}
```

Decode:
```rust
(2, WireType::Varint) => { self.age = decode_varint(buf)? as i32; }
```

#### Explicit presence (proto2 `optional` / proto3 `optional` keyword / editions `EXPLICIT`)

The field is stored as `Option<T>`.  Presence is tracked independently of the value: `Some(0)` means "explicitly set to zero" and is **included** on the wire; `None` means "not set" and is **omitted**.  A custom proto2 default value (e.g. `[default = 100]`) affects only the *accessor*, not the stored `None`.

```rust
// Proto2:  optional int32 score = 2 [default = 100];
score: Option<i32>,

// Accessor returns the proto-declared default when unset:
pub fn score(&self) -> i32 { self.score.unwrap_or(100) }
pub fn has_score(&self) -> bool { self.score.is_some() }
pub fn set_score(&mut self, v: i32) { self.score = Some(v); }
pub fn clear_score(&mut self) { self.score = None; }
```

Encode (note: includes value even if it is 0):
```rust
if let Some(v) = self.score {
    encode_varint_field(2, v as u64, buf);
}
```

Decode (stores `Some(v)` regardless of value):
```rust
(2, WireType::Varint) => { self.score = Some(decode_varint(buf)? as i32); }
```

#### Extension to all scalar types

The table below shows the decode expression and encode cast for every scalar type.  The implicit/explicit presence pattern from above applies to all of them uniformly; only the wire encoding differs.

| Proto type | Wire type | Decode expression | Encode: `v` as `u64` |
|---|---|---|---|
| `int32` | VARINT | `decode_varint(buf)? as i32` | `v as u64` |
| `int64` | VARINT | `decode_varint(buf)? as i64` | `v as u64` |
| `uint32` | VARINT | `decode_varint(buf)? as u32` | `v as u64` |
| `uint64` | VARINT | `decode_varint(buf)?` | `v` |
| `sint32` | VARINT | `unzigzag32(decode_varint(buf)?)` | `zigzag32(v)` |
| `sint64` | VARINT | `unzigzag64(decode_varint(buf)?)` | `zigzag64(v)` |
| `bool` | VARINT | `decode_varint(buf)? != 0` | `v as u64` |
| `float` | I32 | `f32::from_bits(buf.get_u32_le())` | *(use `encode_i32_field`)* |
| `double` | I64 | `f64::from_bits(buf.get_u64_le())` | *(use `encode_i64_field`)* |
| `fixed32` | I32 | `buf.get_u32_le()` | *(use `encode_i32_field`)* |
| `fixed64` | I64 | `buf.get_u64_le()` | *(use `encode_i64_field`)* |
| `sfixed32` | I32 | `buf.get_i32_le()` | *(use `encode_i32_field`)* |
| `sfixed64` | I64 | `buf.get_i64_le()` | *(use `encode_i64_field`)* |

For I32/I64 types: omit on wire when `None` (explicit presence) or when value is the zero-bit pattern (implicit presence, e.g. `0.0f32`, `0u32`).

---

### 4.2 String fields

Strings are stored as `Box<str, A>` (pointer + byte length, no capacity word) and exposed via `&str` accessors.  The storage and accessor pattern is identical across all syntax versions; only the **wire omission rule** differs.

```rust
// Field (all syntax versions):
name: allocator_api2::boxed::Box<str, A>,

pub fn name(&self) -> &str { &self.name }
pub fn set_name(&mut self, v: &str) {
    self.name = puroro::decode::str_to_box_in(v, self._alloc.clone());
}
```

The helper `str_to_box_in(s: &str, alloc: A) -> Box<str, A>` is provided by the runtime library.

**Wire omission:**

| Presence mode | Omit from wire when… |
|---|---|
| Implicit (proto3 default) | value is the empty string `""` |
| Explicit (proto2 `optional` / proto3 `optional` keyword) | field stores `None` (use `Option<Box<str, A>>`) |

For explicit-presence strings, the field type and accessors follow the same `Option<T>` pattern as scalar fields in §4.1, with `Box<str, A>` in place of the scalar type and `""` as the conceptual zero-value.

**Decode:** Uses `decode_string_in(buf, alloc)` which validates UTF-8 and returns `Box<str, A>`.

---

### 4.3 Bytes fields

`bytes` fields are stored as `Vec<u8, A>` and exposed via `&[u8]`.  The same implicit/explicit presence distinction from §4.1–4.2 applies.

```rust
// Implicit presence (proto3 default):
avatar: allocator_api2::vec::Vec<u8, A>,

pub fn avatar(&self) -> &[u8] { &self.avatar }
pub fn set_avatar(&mut self, v: &[u8]) {
    self.avatar.clear();
    self.avatar.extend_from_slice(v);
}
// Omit from wire when empty.

// Explicit presence (proto2 optional / proto3 optional keyword):
avatar: Option<allocator_api2::vec::Vec<u8, A>>,
pub fn avatar(&self) -> Option<&[u8]> { self.avatar.as_deref() }
pub fn has_avatar(&self) -> bool { self.avatar.is_some() }
// Omit from wire when None; include even if the Vec is empty.
```

**Decode:** Uses `decode_bytes_in(buf, alloc)` which returns `Vec<u8, A>`.

---

### 4.4 Repeated fields

Repeated fields are always stored as `Vec<Element, A>` regardless of syntax version.  The accessor returns `&[Element]` — **not** `Vec` — for O(1) access without allocation.

```rust
// Repeated string (proto3 example):
emails: allocator_api2::vec::Vec<allocator_api2::boxed::Box<str, A>, A>,

pub fn emails(&self) -> &[allocator_api2::boxed::Box<str, A>] { &self.emails }
pub fn push_email(&mut self, v: &str) { self.emails.push(str_to_box_in(v, self._alloc.clone())); }
pub fn clear_emails(&mut self) { self.emails.clear(); }
```

Because `Box<str, A>: Deref<Target = str>`, iteration with deref coercion works:
```rust
for email in person.emails() {
    let s: &str = email;  // deref coercion — no copy
}
```

#### Packed vs non-packed encoding

The default packing behaviour depends on syntax version and field type:

| Syntax | Packable numeric `repeated` field | Default encoding |
|---|---|---|
| proto3 | Yes | **Packed** (single LEN record) |
| proto2 | No (unless `[packed = true]`) | **Non-packed** (one record per element) |
| editions | Controlled by `features.repeated_field_encoding` | |

**Packed encode** (proto3 default for numeric types):
```rust
encode_packed_varint_field(3, &self.levels, |v| *v as u64, buf);
```

**Non-packed encode** (proto2 default, or editions `EXPANDED`):
```rust
for &v in &self.levels {
    encode_varint_field(3, v as u64, buf);  // one tag per element
}
```

#### Dual-form decode (required by the spec)

**Regardless of what the schema declares**, the decoder must accept *both* packed and non-packed forms for all numeric repeated fields.  This ensures forward- and backward-compatibility when the `packed` option changes.

```rust
// Field 4: repeated int32 levels — accepts both forms
(4, WireType::Varint) => {
    // Non-packed: one element per record
    self.levels.push(decode_varint(buf)? as i32);
}
(4, WireType::Len) => {
    // Packed: all elements in one LEN payload
    let payload_len = decode_varint(buf)? as usize;
    let leftover = {
        let mut sub = (&mut *buf).take(payload_len);
        while sub.has_remaining() {
            self.levels.push(decode_varint(&mut sub)? as i32);
        }
        sub.remaining()
    };
    if leftover > 0 { buf.advance(leftover); }
}
```

Repeated **string** and **message** fields cannot be packed; they always use one LEN record per element.

---

### 4.5 Nested message fields

Message fields are always represented as `Option<Box<M<A>, A>>` regardless of syntax version (`None` = not set).  Boxing prevents infinite-size structs for recursive types and keeps the parent compact.

```rust
// Field (proto3 and proto2):
address: Option<allocator_api2::boxed::Box<Address<A>, A>>,

pub fn address(&self) -> Option<&Address<A>> { self.address.as_deref() }
pub fn address_mut(&mut self) -> &mut Address<A> {
    self.address.get_or_insert_with(|| {
        Box::new_in(Address::new_in(self._alloc.clone()), self._alloc.clone())
    })
}
pub fn set_address(&mut self, v: Address<A>) {
    self.address = Some(Box::new_in(v, self._alloc.clone()));
}
pub fn clear_address(&mut self) { self.address = None; }
```

**Decode** bounds the sub-message read to exactly the declared byte length using `Buf::take`:

```rust
(5, WireType::Len) => {
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len { return Err(DecodeError::TruncatedMessage); }
    let addr = self.address.get_or_insert_with(|| { /* create default */ });
    let leftover = {
        let mut sub_buf = (&mut *buf).take(len);
        addr.merge_from(&mut sub_buf)?;
        sub_buf.remaining()
    };
    if leftover > 0 { buf.advance(leftover); }
}
```

Merging into an *existing* sub-message (rather than replacing it) implements protobuf's "concatenated messages = merged message" semantics, which applies identically in proto2, proto3, and editions.

---

### 4.6 Enum fields

The wire encoding of enums is always VARINT.  The difference between syntax versions is whether unknown numeric values are *accepted* (open) or *rejected into unknown fields* (closed).

#### Open enum (proto3 / editions `OPEN`)

The field is stored as plain `i32`; unknown values are stored transparently.

```rust
// Field:
primary_phone_type: i32,

pub fn primary_phone_type_raw(&self) -> i32 { self.primary_phone_type }
pub fn set_primary_phone_type_raw(&mut self, v: i32) { self.primary_phone_type = v; }

// Typed accessor wraps the raw value:
pub fn primary_phone_type(&self) -> Result<PhoneType, i32> {
    PhoneType::try_from(self.primary_phone_type)
}
pub fn set_primary_phone_type(&mut self, v: PhoneType) {
    self.primary_phone_type = v as i32;
}
```

Decode:
```rust
(6, WireType::Varint) => {
    self.primary_phone_type = decode_varint(buf)? as i32;  // accept any value
}
```

Encode (implicit presence — omit when 0):
```rust
if self.primary_phone_type != 0 {
    encode_varint_field(6, self.primary_phone_type as u64, buf);
}
```

#### Closed enum (proto2 / editions `CLOSED`)

The field is stored as `Option<i32>` (`None` = not set, or a known value is present).  Unknown numeric values are diverted to `_unknown_fields` so they survive a round-trip.

```rust
// Field:
difficulty: Option<i32>,

pub fn difficulty_raw(&self) -> Option<i32> { self.difficulty }

// Typed accessor returns None when the field is unset:
pub fn difficulty(&self) -> Option<Result<Difficulty, i32>> {
    self.difficulty.map(Difficulty::try_from)
}
pub fn set_difficulty(&mut self, v: Difficulty) { self.difficulty = Some(v as i32); }
pub fn clear_difficulty(&mut self) { self.difficulty = None; }
```

Decode (unknown values → unknown fields via runtime helper):
```rust
(5, WireType::Varint) => {
    let raw = decode_varint(buf)? as i32;
    if Difficulty::try_from(raw).is_ok() {
        self.difficulty = Some(raw);
    } else {
        save_unknown_varint_field(5, raw as u64, &mut self._unknown_fields);
    }
}
```

Encode (explicit presence — omit when `None`; always include when `Some`):
```rust
if let Some(v) = self.difficulty {
    encode_varint_field(5, v as u64, buf);
}
```

The generated enum type (`TryFrom<i32>` + `From<EnumType> for i32`) is identical between open and closed; only the decode arm differs.

---

### 4.7 Oneof fields

Each `oneof` group is represented as `Option<SomeEnum<A>>` in the parent struct, with the enum placed in a submodule named after the parent message (lower-snake-case).  This pattern is identical across all syntax versions.

```rust
// In module `person`:
pub enum ContactMethod<A: Allocator = Global> {
    PhoneNumber(Box<str, A>),   // field 7
    FaxNumber(Box<str, A>),     // field 8
}

// Field in Person:
contact_method: Option<person::ContactMethod<A>>,

// Accessors:
pub fn contact_method(&self) -> Option<&person::ContactMethod<A>>;
pub fn contact_method_mut(&mut self) -> Option<&mut person::ContactMethod<A>>;
pub fn set_contact_method(&mut self, v: Option<person::ContactMethod<A>>);
// Per-variant convenience setters (each clears the other variants):
pub fn set_phone_number(&mut self, v: &str);
pub fn set_fax_number(&mut self, v: &str);
```

Setting any variant replaces the whole `Option`, so the last field seen on the wire wins — correct for all syntax versions.

---

### 4.8 Required fields (proto2 only)

`required` fields exist only in proto2.  On the wire, a required field is indistinguishable from an `optional` field; the constraint is purely at the schema level.

**Storage:** Same as `optional` — `Option<T>` — so the decoder can detect whether the field was present.

**Validation:** A generated `validate()` inherent method checks that all `required` fields are `Some(_)`.  Callers that need strict enforcement use `decode_strict`, which calls `decode` followed by `validate`.

```rust
// Required string field example:
player_id: Option<allocator_api2::boxed::Box<str, A>>,

pub fn player_id(&self) -> Option<&str> { self.player_id.as_deref() }
pub fn set_player_id(&mut self, v: &str) { self.player_id = Some(str_to_box_in(v, …)); }

pub fn validate(&self) -> Result<(), DecodeError> {
    if self.player_id.is_none() {
        return Err(DecodeError::MissingRequiredField { field_number: 1 });
    }
    // … check other required fields …
    Ok(())
}

pub fn decode_strict<B: Buf>(buf: B) -> Result<Self, DecodeError>
where
    Self: Default + MessageDecode,
{
    let msg = Self::decode(buf)?;
    msg.validate()?;
    Ok(msg)
}
```

`MessageDecode::decode` (the provided trait method) does **not** call `validate()` automatically, keeping the trait generic across proto2 and proto3.

---

### 4.9 Unknown fields

Unknown fields are accumulated as raw wire bytes in a `Vec<u8, A>` and re-emitted verbatim on encode, ensuring forward-compatibility round-trips.  This behaviour is identical across all syntax versions.

```rust
_unknown_fields: allocator_api2::vec::Vec<u8, A>,

pub fn unknown_fields(&self) -> &[u8] { &self._unknown_fields }
```

The runtime helper `skip_field_and_save(field_number, wire_type, buf, &mut self._unknown_fields)` handles accumulation for truly unknown fields.  For **closed-enum unknown values** (§4.6), the dedicated `save_unknown_varint_field(field_number, value, &mut self._unknown_fields)` helper is used instead, since the value has already been read from `buf`.

**Disabling unknown-field preservation:** A future annotation (e.g. `#[puroro(no_unknown_fields)]`) would swap both helpers for their discard-only variants and remove the `_unknown_fields` member.  Not yet implemented.

---

## 5. Allocator design

### 5.1 Chosen design: single type parameter

Every generated message type carries a single allocator type parameter `A`:

```rust
pub struct Person<A: Allocator = Global> {
    name:    Box<str,                  A>,
    emails:  Vec<Box<str, A>,          A>,
    address: Option<Box<Address<A>,    A>>,
    _alloc:  A,   // retained for setter/push methods that create new allocations
}
```

`A` defaults to `Global`, so `Person` (no generic argument) works exactly like it would without allocator support.

The stored `_alloc` field is needed because setters and push methods must allocate new values (e.g. when calling `set_name` on an already-constructed message). This requires a clone of the allocator. For zero-cost allocators like `Global` (a ZST), the field adds zero size. For reference-like allocators (e.g. `&Bump`), it adds one pointer.

**Constructor:**

```rust
impl<A: Allocator + Clone> Person<A> {
    pub fn new_in(alloc: A) -> Self { … }
}
impl Person {
    pub fn new() -> Self { Self::new_in(Global) }
}
impl<A: Allocator + Clone + Default> Default for Person<A> {
    fn default() -> Self { Self::new_in(A::default()) }
}
```

**Decode:**

```rust
impl<A: Allocator + Clone + Default> MessageDecode for Person<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        // Uses self._alloc.clone() when creating new field values.
    }
}
```

**Using an arena allocator (e.g. bumpalo):**

```rust
let bump = bumpalo::Bump::new();
let mut p = Person::new_in(&bump);
p.set_name("Alice");
// All allocations (name, emails, nested messages) land in `bump`.
// Dropping `bump` frees everything at once without individual `Drop` calls.
```

---

### 5.2 Alternative: per-field allocator type parameters

To allow each field to use a different allocator, every dynamically-sized field would need its own type parameter:

```rust
// For a message with 3 heap-allocated fields:
pub struct Person<A1 = Global, A2 = Global, A3 = Global, A4 = Global, A5 = Global>
where
    A1: Allocator, A2: Allocator, A3: Allocator, A4: Allocator, A5: Allocator,
{
    name:    Box<str, A1>,
    emails:  Vec<Box<str, A2>, A3>,  // element allocator ≠ container allocator
    address: Option<Box<Address<???>, A4>>,  // Address itself needs its own params!
    _alloc:  ???,                     // which allocator to clone for new fields?
}
```

**Problems that make this design impractical:**

1. **Combinatorial explosion.** A message with `N` heap-allocated fields requires up to `2N` type parameters (one for each element type, one for each container). Nested messages multiply: if `Address` has `M` such fields, `Person` would need `2N + 2M + …` parameters.

2. **No natural "default allocator" for new values.** When calling `push_email("foo")`, the code must know which allocator to use. With a single `_alloc: A`, the answer is `self._alloc.clone()`. With per-field allocators there is no obvious answer without adding even more parameters.

3. **Type inference failure.** Generic call sites become impossible to write without turbofish annotations for every type parameter.

4. **Interoperability.** Functions that accept a `Person<…>` would need to list all allocator type parameters, making signatures unwieldy.

**Verdict:** The combinatorial complexity grows with message depth and renders this design impractical.

---

### 5.3 Alternative: dynamic dispatch (`Box<dyn Allocator>`)

One allocator reference is shared across all fields through a concrete reference-counted pointer:

```rust
pub struct Person {
    name:   Box<str>,        // uses Global (std::alloc)
    emails: Vec<String>,
    // _alloc is implicit (Global or a thread-local)
    _alloc_ref: std::sync::Arc<dyn Allocator + Send + Sync>,
}
```

Or, each field explicitly carries a `dyn` pointer:

```rust
pub struct Person {
    name:   Box<str, Arc<dyn Allocator + Send + Sync>>,
    emails: Vec<Box<str, Arc<dyn Allocator + Send + Sync>>, Arc<dyn Allocator + Send + Sync>>,
    …
}
```

**Problems:**

1. **Runtime overhead.** Every allocation / deallocation goes through a virtual dispatch. For short-lived messages or high-throughput parsing, this is measurable.

2. **`Arc` cost.** Reference counting adds atomic operations.

3. **`Send + Sync` requirement.** Not all custom allocators are thread-safe (e.g. bumpalo's `Bump` is `!Sync`). Requiring `Send + Sync` excludes them.

4. **`dyn Allocator` is not object-safe in the standard allocator API.** The `Allocator` trait has methods that return `NonNull<[u8]>`, which is object-safe, but composing it with `Send + Sync` is possible. However, it requires wrapping and type-erasing in ways the compiler does not do automatically.

**Verdict:** Viable as a concrete type alias (`type SharedAlloc = Arc<dyn Allocator + Send + Sync>`) for users who truly need runtime allocator dispatch, but not as the primary design. It gives up monomorphisation, which is one of the main performance benefits of the generic approach.

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

The single-`A` design is the clear practical choice. The inability to mix allocators across fields of the same message instance is an acceptable trade-off: in the overwhelming majority of use cases, a message's lifetime is tied to a single allocation domain (global heap, one arena, one pool).

---

## 6. Proto syntax versions: proto2, proto3, and editions

### 6.1 Wire format is version-agnostic

The wire format — varints, I32/I64 fixed-width values, LEN-prefixed payloads — is **identical** across proto2, proto3, and editions.  The runtime library (`MessageEncode`, `MessageDecode`, all encode/decode helpers) therefore requires **no changes** to support proto2 or editions schemas.  All differences are in what the **code generator emits**, not in the runtime that the generated code calls.

### 6.2 Field presence and optional scalars

See **§4.1** for the complete generated code patterns (implicit vs explicit presence, custom defaults, all numeric types).

Summary:

| Syntax | `optional int32 age` | Storage | Omit from wire when |
|---|---|---|---|
| proto3 (default) | Implicit presence | `age: i32` | value == 0 |
| proto3 `optional` keyword | Explicit presence | `age: Option<i32>` | `None` |
| proto2 `optional` | Explicit presence | `age: Option<i32>` | `None` |
| editions `IMPLICIT` | Implicit presence | `age: i32` | value == 0 |
| editions `EXPLICIT` | Explicit presence | `age: Option<i32>` | `None` |

### 6.3 Custom default values (proto2)

See **§4.1** (explicit presence pattern).

The accessor returns the proto-declared default when the field is `None`; the `new_in()` constructor always initialises the field to `None` (unset), not to the custom default.  The default is a *read-time* substitution only.

### 6.4 Required fields (proto2)

See **§4.8** for the complete generated code pattern (`validate()` + `decode_strict()`).

### 6.5 Closed vs open enums

See **§4.6** for the complete generated code patterns and the `save_unknown_varint_field` helper usage.

### 6.6 Packed repeated fields

See **§4.4** for the complete patterns (packed encode, non-packed encode, dual-form decode).

### 6.7 Editions feature matrix

Editions (Edition 2023 onwards) replaces the proto2/proto3 binary choice with per-field feature flags.  The features that affect the generated code are:

| Feature | Values | Effect |
|---|---|---|
| `field_presence` | `IMPLICIT` / `EXPLICIT` | `T` vs `Option<T>` for scalar fields |
| `repeated_field_encoding` | `PACKED` / `EXPANDED` | Packed vs non-packed encoding default |
| `enum_type` | `OPEN` / `CLOSED` | Open vs closed enum handling in decoder |
| `message_encoding` | `LENGTH_PREFIXED` / `DELIMITED` | LEN-prefixed (normal) vs group-delimited (deprecated `DELIMITED`) |
| `utf8_validation` | `VERIFY` / `NONE` | Whether to validate UTF-8 in string fields during decode |

From the runtime and generated-code perspective, all of these translate directly to the patterns described in §4 and §6.2–6.6.  The editions file-level defaults are:

| Feature | Edition 2023 default |
|---|---|
| `field_presence` | `IMPLICIT` (proto3-like) |
| `repeated_field_encoding` | `PACKED` (proto3-like) |
| `enum_type` | `OPEN` (proto3-like) |

### 6.8 Extensions (not yet implemented)

proto2 `extensions` / `extend` blocks are **out of scope** for the current design.  They require a fundamentally different storage mechanism: a message must hold an opaque map of (field number → encoded bytes) for extension fields that are not known to this compilation unit.  This is architecturally more complex than unknown fields (which just accumulate raw bytes) because extension values must be retrievable by field number.

A future design would need to specify:
- An `ExtensionSet<A>` storage type backed by a sorted `Vec<(u32, Vec<u8, A>), A>` or a hash map.
- A way to register extension descriptors at runtime or compile time.
- Accessor generation for extension fields in a separate file from the base message.

### 6.9 Comparison summary

| | Runtime change? | Generated code change |
|---|---|---|
| `optional` scalar → `Option<T>` | None | Field type, accessor, encode/decode arms |
| Custom default value | None | `accessor()` returns `unwrap_or(default)` |
| `required` field | `DecodeError::MissingRequiredField` added | `validate()` + `decode_strict()` methods |
| Closed enum | `save_unknown_varint_field` added | Different decode arm + `Option<i32>` field |
| Non-packed repeated | None (helpers already exist) | Encode loop + dual VARINT/LEN decode arms |
| Editions features | None | Generator reads feature flags, emits matching pattern |
| Extensions | **Significant** — new storage type needed | New accessor generation strategy |

---

## 7. Design decisions and trade-offs

### No zero-copy decode (for now)

Giving `string` / `bytes` fields a lifetime parameter (`&'buf str`, `&'buf [u8]`) would enable zero-copy decoding but would propagate `'buf` to every generated type and its callers. The current design uses owned `Box<str, A>` and `Vec<u8, A>`, which require copying from the input buffer. This is the same trade-off made by prost and the official Google Rust protobuf library.

A future extension could introduce a `PersonView<'buf>` "borrowing view" type alongside the owned `Person<A>`, or adopt a `Cow`-like storage type, without breaking the existing API.

### Accessor methods instead of public fields

Public fields (prost's approach) are simpler to write but make it impossible to change the internal representation later (e.g. from `Box<str, A>` to `&'buf str`) without a breaking API change. Accessor methods add a small amount of boilerplate but give the code generator freedom to change storage internals independently.

### `&[Box<str, A>]` for repeated strings

Returning `&[Box<str, A>]` from `emails()` preserves O(1) random access without allocating. The slightly awkward ergonomics (`email` in a for-loop has type `&Box<str, A>`, not `&str`) is mitigated by:

- Deref coercion: `let s: &str = email;` compiles without explicit dereference.
- `Box<str, A>: Display`, so `println!("{email}")` works directly.

A future `RepeatedStr<'_, A>` wrapper could implement `Index<usize, Output = str>` for fully transparent ergonomics, but this is not needed for correctness.

### `Default` bound on `MessageDecode`

`MessageDecode::decode` (the top-level convenience) requires `Self: Default` so it can create the initial message value. The `merge_from` operation has no such requirement, which means that callers who control memory initialisation (e.g. arena-allocated pre-zeroed memory) can use `merge_from` without the `Default` constraint.

---

## 8. Future work

- **Zero-copy decode.** Introduce borrowing view types (`PersonView<'buf>`) that store `&'buf str` and `&'buf [u8]` directly.
- **Unknown-field preservation opt-out.** A `#[puroro(no_unknown_fields)]` attribute or similar mechanism for messages where round-trip byte-for-byte fidelity is not required.
- **Map fields.** Currently not implemented. Map fields are syntactic sugar for `repeated MessageEntry` (where `MessageEntry` has a `key` and a `value` field). Implementation requires an allocator-aware hash map (e.g. `hashbrown::HashMap<K, V, S, A>`).
- **Service / RPC definitions.** Out of scope for the runtime library; will be handled by a separate code-generation layer.
- **Well-known types.** `google.protobuf.Timestamp`, `Duration`, `Any`, etc.
- **Reflection / descriptors.** Runtime introspection of message schema.
