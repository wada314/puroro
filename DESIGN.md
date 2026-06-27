# puroro — Design Document

This document describes the design of the `puroro` Protocol Buffers runtime library and specifies what code the future code generator should emit.

## Table of contents

1. [Goals](#1-goals)
2. [Wire format overview](#2-wire-format-overview)
3. [Runtime trait API](#3-runtime-trait-api)
4. [Generated code specification](#4-generated-code-specification)
   - 4.1 [Scalar fields](#41-scalar-fields)
   - 4.2 [String fields](#42-string-fields)
   - 4.3 [Bytes fields](#43-bytes-fields)
   - 4.4 [Repeated fields](#44-repeated-fields)
   - 4.5 [Nested message fields](#45-nested-message-fields)
   - 4.6 [Enum fields](#46-enum-fields)
   - 4.7 [Oneof fields](#47-oneof-fields)
   - 4.8 [Unknown fields](#48-unknown-fields)
5. [Allocator design](#5-allocator-design)
   - 5.1 [Chosen design: single type parameter](#51-chosen-design-single-type-parameter)
   - 5.2 [Alternative: per-field allocator type parameters](#52-alternative-per-field-allocator-type-parameters)
   - 5.3 [Alternative: dynamic dispatch (`Box<dyn Allocator>`)](#53-alternative-dynamic-dispatch-boxdyn-allocator)
   - 5.4 [Comparison summary](#54-comparison-summary)
6. [Design decisions and trade-offs](#6-design-decisions-and-trade-offs)
7. [Future work](#7-future-work)

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

Reference proto file for this section:

```protobuf
syntax = "proto3";
package example;

enum PhoneType {
    PHONE_TYPE_UNSPECIFIED = 0;
    MOBILE = 1;
    HOME   = 2;
    WORK   = 3;
}

message Address {
    string street = 1;
    string city   = 2;
}

message Person {
    string          name               = 1;
    int32           age                = 2;
    bytes           avatar             = 3;
    repeated string emails             = 4;
    Address         address            = 5;
    PhoneType       primary_phone_type = 6;
    oneof contact_method {
        string phone_number = 7;
        string fax_number   = 8;
    }
}
```

The full compilable code is in `src/sample.rs`. The subsections below explain each pattern.

### 4.1 Scalar fields

Proto3 scalar types map to Rust copy types. The accessor returns by value.

```rust
// Proto:  int32 age = 2;
// Field:
age: i32,

// Accessors:
pub fn age(&self) -> i32 { self.age }
pub fn set_age(&mut self, v: i32) { self.age = v; }
```

**Encoding** uses `encode_varint_field(field_number, v as u64, buf)`. The field is **omitted** when it equals the proto3 default (`0` for integers, `false` for bools).

**Decoding** reads `decode_varint(buf)? as i32`. The match arm is:
```rust
(2, WireType::Varint) => { self.age = decode_varint(buf)? as i32; }
```

**Extension to other integer types:**

| Proto type | Decode expression | `as u64` cast for encode |
|---|---|---|
| `int64` | `decode_varint(buf)? as i64` | `v as u64` |
| `uint32` | `decode_varint(buf)? as u32` | `v as u64` |
| `uint64` | `decode_varint(buf)?` | `v` |
| `sint32` | `unzigzag32(decode_varint(buf)?)` | `zigzag32(v)` |
| `sint64` | `unzigzag64(decode_varint(buf)?)` | `zigzag64(v)` |
| `bool` | `decode_varint(buf)? != 0` | `v as u64` |
| `float` | `f32::from_bits(buf.get_u32_le())` (WireType::I32) | — |
| `double` | `f64::from_bits(buf.get_u64_le())` (WireType::I64) | — |
| `fixed32` | `buf.get_u32_le()` (WireType::I32) | — |
| `fixed64` | `buf.get_u64_le()` (WireType::I64) | — |
| `sfixed32` | `buf.get_i32_le()` (WireType::I32) | — |
| `sfixed64` | `buf.get_i64_le()` (WireType::I64) | — |

### 4.2 String fields

Strings are stored as `Box<str, A>` (2 words: pointer + byte length, no capacity), and exposed via `&str` accessors.

```rust
// Field:
name: allocator_api2::boxed::Box<str, A>,

// Accessors:
pub fn name(&self) -> &str { &self.name }
pub fn set_name(&mut self, v: &str) {
    self.name = puroro::decode::str_to_box_in(v, self._alloc.clone());
}
```

The helper `str_to_box_in(s: &str, alloc: A) -> Box<str, A>` allocates a `Vec<u8, A>`, copies the bytes, then transmutes to `Box<str, A>` (one unsafe block, hidden in the runtime library).

An **empty string** is the proto3 default; it is **omitted** from the wire.

### 4.3 Bytes fields

Bytes fields are stored as `Vec<u8, A>` (3 words, but allows incremental appending) and exposed via `&[u8]`.

```rust
// Field:
avatar: allocator_api2::vec::Vec<u8, A>,

// Accessors:
pub fn avatar(&self) -> &[u8] { &self.avatar }
pub fn set_avatar(&mut self, v: &[u8]) {
    self.avatar.clear();
    self.avatar.extend_from_slice(v);
}
```

### 4.4 Repeated fields

Repeated fields are stored as `Vec<Element, A>`. The accessor returns `&[Element]` — **not** a `Vec` — so callers get O(1) random access without triggering a re-allocation.

```rust
// Proto:  repeated string emails = 4;
// Field:
emails: allocator_api2::vec::Vec<allocator_api2::boxed::Box<str, A>, A>,

// Accessors:
pub fn emails(&self) -> &[allocator_api2::boxed::Box<str, A>] { &self.emails }
pub fn push_email(&mut self, v: &str) { self.emails.push(str_to_box_in(v, self._alloc.clone())); }
pub fn clear_emails(&mut self) { self.emails.clear(); }
```

Because `Box<str, A>: Deref<Target = str>`, iterating produces values that coerce to `&str`:

```rust
for email in person.emails() {
    let s: &str = email;  // deref coercion — no copy
    println!("{email}");  // Box<str, A>: Display via Box<T: Display>
}
```

**Packed repeated scalars** (e.g. `repeated int32`) are encoded as a single LEN record. Decoding handles both packed and non-packed forms, as required by the proto3 spec. The helpers `encode_packed_varint_field` / `encoded_len_packed_varint_field` are provided by the runtime.

### 4.5 Nested message fields

Message fields are `Option<Box<M<A>, A>>` (`None` = not set). Boxing avoids infinite-size structs for mutually recursive messages and keeps the parent message compact.

```rust
// Field:
address: Option<allocator_api2::boxed::Box<Address<A>, A>>,

// Accessors:
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

**Decode** uses `buf.take(len)` to limit the sub-buffer to exactly the declared length, matching the proto3 spec for nested message parsing:

```rust
(5, WireType::Len) => {
    let len = decode_varint(buf)? as usize;
    let addr = self.address.get_or_insert_with(|| { /* create default */ });
    let leftover = {
        let mut sub_buf = (&mut *buf).take(len);
        addr.merge_from(&mut sub_buf)?;
        sub_buf.remaining()
    };
    if leftover > 0 { buf.advance(leftover); }
}
```

Merging into an existing sub-message (rather than replacing it) correctly implements protobuf's "concatenation = merge" semantics.

### 4.6 Enum fields

Proto3 enums are **open**: the wire may carry numeric values not listed in the definition. The field is stored as `i32`; a typed accessor wraps it in `Result<MyEnum, i32>`.

```rust
// Field:
primary_phone_type: i32,

// Raw access (always available):
pub fn primary_phone_type_raw(&self) -> i32 { self.primary_phone_type }
pub fn set_primary_phone_type_raw(&mut self, v: i32) { self.primary_phone_type = v; }

// Typed access (returns Err(i32) for unknown values):
pub fn primary_phone_type(&self) -> Result<PhoneType, i32> {
    PhoneType::try_from(self.primary_phone_type)
}
pub fn set_primary_phone_type(&mut self, v: PhoneType) {
    self.primary_phone_type = v as i32;
}
```

The generated enum implements `TryFrom<i32>` and `From<EnumType> for i32`.

### 4.7 Oneof fields

Each `oneof` group is represented as an `Option<SomeEnum<A>>` where the enum is placed in a submodule named after the parent message (lower-snake-case).

```rust
// In module `person`:
pub enum ContactMethod<A: Allocator = Global> {
    PhoneNumber(Box<str, A>),
    FaxNumber(Box<str, A>),
}

// Field in Person:
contact_method: Option<person::ContactMethod<A>>,

// Accessors:
pub fn contact_method(&self) -> Option<&person::ContactMethod<A>>;
pub fn contact_method_mut(&mut self) -> Option<&mut person::ContactMethod<A>>;
pub fn set_contact_method(&mut self, v: Option<person::ContactMethod<A>>);
// Convenience setters for individual variants:
pub fn set_phone_number(&mut self, v: &str);  // clears fax_number
pub fn set_fax_number(&mut self, v: &str);    // clears phone_number
```

**Oneof decode semantics:** setting any variant replaces the whole `Option`, so the last field seen on the wire wins — which is the correct proto3 behaviour.

### 4.8 Unknown fields

Unknown fields are accumulated as raw wire bytes in a `Vec<u8, A>`. On encode, they are re-emitted verbatim at the end of the message, ensuring forward-compatibility round-trips.

```rust
_unknown_fields: allocator_api2::vec::Vec<u8, A>,

pub fn unknown_fields(&self) -> &[u8] { &self._unknown_fields }
```

The runtime helper `skip_field_and_save(field_number, wire_type, buf, &mut self._unknown_fields)` handles accumulation.

**Disabling unknown-field preservation:** A future extension could let users annotate a message type with something like `#[puroro(no_unknown_fields)]`, which would swap `skip_field_and_save` for `skip_field` (a discard-only variant) and remove the `_unknown_fields` member. This is not yet implemented.

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

## 6. Design decisions and trade-offs

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

## 7. Future work

- **Zero-copy decode.** Introduce borrowing view types (`PersonView<'buf>`) that store `&'buf str` and `&'buf [u8]` directly.
- **Unknown-field preservation opt-out.** A `#[puroro(no_unknown_fields)]` attribute or similar mechanism for messages where round-trip byte-for-byte fidelity is not required.
- **Map fields.** Currently not implemented. Map fields are syntactic sugar for `repeated MessageEntry` (where `MessageEntry` has a `key` and a `value` field). Implementation requires an allocator-aware hash map (e.g. `hashbrown::HashMap<K, V, S, A>`).
- **Service / RPC definitions.** Out of scope for the runtime library; will be handled by a separate code-generation layer.
- **Well-known types.** `google.protobuf.Timestamp`, `Duration`, `Any`, etc.
- **Reflection / descriptors.** Runtime introspection of message schema.
