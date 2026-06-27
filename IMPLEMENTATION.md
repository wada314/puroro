# puroro — Implementation Notes

This document describes the **internal implementation** of the reference generated code in `src/sample.rs`. It covers storage types, encode/decode algorithms, and runtime helper usage. The stable **public interface** is specified in [DESIGN.md](DESIGN.md).

> **Note:** The public API described in DESIGN.md must remain stable even if the internal representations documented here change.  For example, presence tracking for optional scalars currently uses `Option<T>` fields, but could be replaced by a per-message bitfield without any change to the accessor API.

## Table of contents

1. [Storage types per field kind](#1-storage-types-per-field-kind)
2. [Struct layout](#2-struct-layout)
3. [Constructors and the stored allocator](#3-constructors-and-the-stored-allocator)
4. [Encode implementation](#4-encode-implementation)
5. [Decode implementation](#5-decode-implementation)
6. [Per-field encode/decode patterns](#6-per-field-encodedecode-patterns)
   - 6.1 [Scalar fields (implicit presence)](#61-scalar-fields-implicit-presence)
   - 6.2 [Scalar fields (explicit presence)](#62-scalar-fields-explicit-presence)
   - 6.3 [String fields](#63-string-fields)
   - 6.4 [Bytes fields](#64-bytes-fields)
   - 6.5 [Repeated scalar fields (packed)](#65-repeated-scalar-fields-packed)
   - 6.6 [Repeated scalar fields (non-packed, with packed fallback)](#66-repeated-scalar-fields-non-packed-with-packed-fallback)
   - 6.7 [Repeated string / message fields](#67-repeated-string--message-fields)
   - 6.8 [Nested message fields](#68-nested-message-fields)
   - 6.9 [Open enum fields](#69-open-enum-fields)
   - 6.10 [Closed enum fields](#610-closed-enum-fields)
   - 6.11 [Oneof fields](#611-oneof-fields)
   - 6.12 [Unknown fields](#612-unknown-fields)
   - 6.13 [Required fields (proto2)](#613-required-fields-proto2)
7. [Optimization opportunities](#7-optimization-opportunities)

---

## 1. Storage types per field kind

| Field kind | Internal storage type |
|---|---|
| Implicit-presence scalar (e.g. `int32 age`) | `i32` (the Rust primitive directly) |
| Explicit-presence scalar (e.g. `optional int32 score`) | `Option<i32>` |
| Implicit-presence string | `allocator_api2::boxed::Box<str, A>` |
| Explicit-presence string | `Option<allocator_api2::boxed::Box<str, A>>` |
| `bytes` field (implicit presence) | `allocator_api2::vec::Vec<u8, A>` |
| `bytes` field (explicit presence) | `Option<allocator_api2::vec::Vec<u8, A>>` |
| Repeated scalar / string / bytes / message | `allocator_api2::vec::Vec<ElementType, A>` |
| Message field (always optional) | `Option<allocator_api2::boxed::Box<MessageType<A>, A>>` |
| Open enum field | `i32` |
| Closed enum field | `Option<i32>` |
| Oneof group | `Option<Oneof<A>>` |
| Unknown fields | `allocator_api2::vec::Vec<u8, A>` |

`Box<str, A>` (2 words: ptr + byte length, no capacity word) is preferred over `Vec<u8, A>` (3 words) for owned strings since strings are generally set once and then read.  `Vec<u8, A>` is used for `bytes` fields where incremental appending is more natural.

> **Optimization note:** explicit-presence scalars currently use `Option<T>` which takes extra space (e.g. `Option<i32>` is typically 8 bytes on a 64-bit platform due to the discriminant).  A future optimisation could use `T` storage with a per-message bitfield to track presence, reducing struct size at the cost of more complex accessor code.  The public accessor API (`has_X()`, `set_X()`, `clear_X()`) would remain unchanged.

---

## 2. Struct layout

Every generated message struct carries the following private fields:

```rust
pub struct Person<A: Allocator = Global> {
    // ── User fields (one per .proto field) ────────────────────────────────
    name:                Box<str, A>,
    age:                 i32,
    avatar:              Vec<u8, A>,
    emails:              Vec<Box<str, A>, A>,
    address:             Option<Box<Address<A>, A>>,
    primary_phone_type:  i32,                         // open enum
    contact_method:      Option<person::ContactMethod<A>>,

    // ── Infrastructure ─────────────────────────────────────────────────────
    _unknown_fields: Vec<u8, A>,
    // Retained copy of the allocator for use by setter / push methods.
    // For ZST allocators (e.g. Global) this adds zero size.
    _alloc: A,
}
```

The `_alloc` field is necessary because setters and push methods need to create new heap values without the caller passing an allocator argument.  They clone `_alloc` at call time.  For reference-typed allocators such as `&'arena Bump`, cloning is a pointer copy.

---

## 3. Constructors and the stored allocator

```rust
impl<A: Allocator + Clone> Person<A> {
    pub fn new_in(alloc: A) -> Self {
        Person {
            name:               str_to_box_in("", alloc.clone()),
            age:                0,
            avatar:             Vec::new_in(alloc.clone()),
            emails:             Vec::new_in(alloc.clone()),
            address:            None,
            primary_phone_type: 0,
            contact_method:     None,
            _unknown_fields:    Vec::new_in(alloc.clone()),
            _alloc:             alloc,
        }
    }
}

impl<A: Allocator + Clone + Default> Default for Person<A> {
    fn default() -> Self { Self::new_in(A::default()) }
}
```

The helper `str_to_box_in(s: &str, alloc: A) -> Box<str, A>` (provided by the runtime) allocates a `Vec<u8, A>`, copies the bytes, converts to `Box<[u8], A>`, then reinterprets as `Box<str, A>` (one unsafe block inside the runtime, not visible to generated code).

---

## 4. Encode implementation

`MessageEncode` is implemented for `Person<A: Allocator + Clone>`.

The general structure of `encode_raw` and `encoded_len`:

```rust
impl<A: Allocator + Clone> MessageEncode for Person<A> {
    fn encoded_len(&self) -> usize {
        let mut len = 0;
        // Field 1: string name (implicit presence — omit if empty)
        if !self.name.is_empty() {
            len += encoded_len_len_field(1, self.name.len());
        }
        // Field 2: int32 age (implicit presence — omit if zero)
        if self.age != 0 {
            len += encoded_len_varint_field(2, self.age as u64);
        }
        // … other fields …
        // Unknown fields pass through unchanged.
        len += self._unknown_fields.len();
        len
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        if !self.name.is_empty() {
            encode_len_field(1, self.name.as_bytes(), buf);
        }
        if self.age != 0 {
            encode_varint_field(2, self.age as u64, buf);
        }
        // … other fields …
        buf.put_slice(&self._unknown_fields);
    }
}
```

---

## 5. Decode implementation

`MessageDecode` is implemented for `Person<A: Allocator + Clone + Default>`.

```rust
impl<A: Allocator + Clone + Default> MessageDecode for Person<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match (field_number, wire_type) {
                (1, WireType::Len) => {
                    self.name = decode_string_in(buf, self._alloc.clone())?;
                }
                (2, WireType::Varint) => {
                    self.age = decode_varint(buf)? as i32;
                }
                // … other fields …
                _ => {
                    skip_field_and_save(field_number, wire_type, buf, &mut self._unknown_fields)?;
                }
            }
        }
        Ok(())
    }
}
```

---

## 6. Per-field encode/decode patterns

### 6.1 Scalar fields (implicit presence)

**Storage:** plain `T` (e.g. `i32`).

**Encode:** omit when value equals the type-zero.
```rust
// int32 age, field 2
if self.age != 0 {
    encode_varint_field(2, self.age as u64, buf);
}
```

**Decode:**
```rust
(2, WireType::Varint) => { self.age = decode_varint(buf)? as i32; }
```

**Extension to all scalar types:**

| Proto type | Decode expression | Encode: cast to `u64` |
|---|---|---|
| `int32` | `decode_varint(buf)? as i32` | `v as u64` |
| `int64` | `decode_varint(buf)? as i64` | `v as u64` |
| `uint32` | `decode_varint(buf)? as u32` | `v as u64` |
| `uint64` | `decode_varint(buf)?` | `v` |
| `sint32` | `unzigzag32(decode_varint(buf)?)` | `zigzag32(v)` |
| `sint64` | `unzigzag64(decode_varint(buf)?)` | `zigzag64(v)` |
| `bool` | `decode_varint(buf)? != 0` | `v as u64` |
| `float` | `f32::from_bits(buf.get_u32_le())` | `encode_i32_field(n, v.to_bits(), buf)` |
| `double` | `f64::from_bits(buf.get_u64_le())` | `encode_i64_field(n, v.to_bits(), buf)` |
| `fixed32` | `buf.get_u32_le()` | `encode_i32_field(n, v, buf)` |
| `fixed64` | `buf.get_u64_le()` | `encode_i64_field(n, v, buf)` |
| `sfixed32` | `buf.get_i32_le()` | `encode_i32_field(n, v as u32, buf)` |
| `sfixed64` | `buf.get_i64_le()` | `encode_i64_field(n, v as u64, buf)` |

### 6.2 Scalar fields (explicit presence)

**Storage:** `Option<T>`.

**Encode:** include when `Some`, even if the value is zero.
```rust
// optional int32 score = 2 [default = 100]
if let Some(v) = self.score {
    encode_varint_field(2, v as u64, buf);
}
```

**Decode:** always wrap in `Some`.
```rust
(2, WireType::Varint) => { self.score = Some(decode_varint(buf)? as i32); }
```

### 6.3 String fields

**Storage:** `Box<str, A>` (implicit presence) or `Option<Box<str, A>>` (explicit presence).

**Encode:**
```rust
// Implicit — omit if empty
if !self.name.is_empty() {
    encode_len_field(1, self.name.as_bytes(), buf);
}
// Explicit — omit if None
if let Some(ref s) = self.name {
    encode_len_field(1, s.as_bytes(), buf);
}
```

**Decode:**
```rust
(1, WireType::Len) => {
    self.name = decode_string_in(buf, self._alloc.clone())?;
    // For explicit presence: self.name = Some(decode_string_in(…)?);
}
```

`decode_string_in(buf, alloc)` reads a LEN-prefixed payload, validates UTF-8, and returns `Box<str, A>`.

**Setter implementation:**
```rust
pub fn set_name(&mut self, v: &str) {
    self.name = str_to_box_in(v, self._alloc.clone());
}
```

### 6.4 Bytes fields

**Storage:** `Vec<u8, A>` (implicit) or `Option<Vec<u8, A>>` (explicit).

**Encode/decode** follow the same pattern as strings, using `encode_len_field` / `decode_bytes_in`.

**Setter:**
```rust
pub fn set_avatar(&mut self, v: &[u8]) {
    self.avatar.clear();
    self.avatar.extend_from_slice(v);
}
```

### 6.5 Repeated scalar fields (packed)

**Storage:** `Vec<i32, A>`.

**Encode:**
```rust
// encode_packed_varint_field handles tag + length + all elements
encode_packed_varint_field(4, &self.levels, |v| *v as u64, buf);
```

**Decode (accepts both packed and non-packed):**
```rust
(4, WireType::Varint) => {
    self.levels.push(decode_varint(buf)? as i32);
}
(4, WireType::Len) => {
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

### 6.6 Repeated scalar fields (non-packed, with packed fallback)

**Encode** (one record per element):
```rust
for &v in &self.levels {
    encode_varint_field(4, v as u64, buf);
}
```

**Decode** is identical to the packed case above — both VARINT and LEN arms are always present.

### 6.7 Repeated string / message fields

**Storage:** `Vec<Box<str, A>, A>` for strings; `Vec<Box<M<A>, A>, A>` for messages.

**Encode:**
```rust
for email in &self.emails {
    encode_len_field(4, email.as_bytes(), buf);
}
```

**Decode:**
```rust
(4, WireType::Len) => {
    let s = decode_string_in(buf, self._alloc.clone())?;
    self.emails.push(s);
}
```

### 6.8 Nested message fields

**Storage:** `Option<Box<M<A>, A>>`.

**Encode:**
```rust
if let Some(addr) = &self.address {
    let msg_len = addr.encoded_len();
    encode_tag(5, WireType::Len, buf);
    encode_varint(msg_len as u64, buf);
    addr.encode_raw(buf);
}
```

**Decode** uses `Buf::take` to bound the sub-message to exactly the declared byte length:
```rust
(5, WireType::Len) => {
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len { return Err(DecodeError::TruncatedMessage); }
    let addr = self.address.get_or_insert_with(|| {
        Box::new_in(Address::new_in(self._alloc.clone()), self._alloc.clone())
    });
    let leftover = {
        let mut sub_buf = (&mut *buf).take(len);
        addr.merge_from(&mut sub_buf)?;
        sub_buf.remaining()
    };
    if leftover > 0 { buf.advance(leftover); }
}
```

Merging into an existing sub-message (not replacing it) implements protobuf's "concatenation = merge" semantics.

### 6.9 Open enum fields

**Storage:** `i32`.

**Encode:**
```rust
if self.primary_phone_type != 0 {
    encode_varint_field(6, self.primary_phone_type as u64, buf);
}
```

**Decode:**
```rust
(6, WireType::Varint) => {
    self.primary_phone_type = decode_varint(buf)? as i32;
}
```

### 6.10 Closed enum fields

**Storage:** `Option<i32>`.

**Encode:**
```rust
if let Some(v) = self.difficulty {
    encode_varint_field(5, v as u64, buf);
}
```

**Decode:** unknown values are diverted to `_unknown_fields` via `save_unknown_varint_field`:
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

### 6.11 Oneof fields

**Storage:** `Option<person::ContactMethod<A>>` where the enum holds the string/scalar/message value.

**Encode:**
```rust
if let Some(cm) = &self.contact_method {
    match cm {
        person::ContactMethod::PhoneNumber(s) => encode_len_field(7, s.as_bytes(), buf),
        person::ContactMethod::FaxNumber(s)   => encode_len_field(8, s.as_bytes(), buf),
    }
}
```

**Decode:** each variant's field number sets a new `Some(_)`, replacing any prior variant:
```rust
(7, WireType::Len) => {
    let s = decode_string_in(buf, self._alloc.clone())?;
    self.contact_method = Some(person::ContactMethod::PhoneNumber(s));
}
(8, WireType::Len) => {
    let s = decode_string_in(buf, self._alloc.clone())?;
    self.contact_method = Some(person::ContactMethod::FaxNumber(s));
}
```

### 6.12 Unknown fields

**Storage:** `Vec<u8, A>` containing a valid (partial) protobuf wire stream.

**Accumulation during decode:**
```rust
_ => {
    skip_field_and_save(field_number, wire_type, buf, &mut self._unknown_fields)?;
}
```

`skip_field_and_save` re-serialises the tag and copies the payload verbatim into `_unknown_fields`.

For closed-enum unknown values (§6.10), `save_unknown_varint_field` writes only the tag and the already-read value, since the value has been decoded before the check.

**Emit during encode:**
```rust
buf.put_slice(&self._unknown_fields);
```

### 6.13 Required fields (proto2)

**Storage:** `Option<T>` (same as an `optional` field) to track whether the field was seen on the wire.

```rust
// impl block for validate():
pub fn validate(&self) -> Result<(), DecodeError> {
    if self.player_id.is_none() {
        return Err(DecodeError::MissingRequiredField { field_number: 1 });
    }
    Ok(())
}
```

The decode arm is identical to an `optional` field.

---

## 7. Optimization opportunities

### Presence bitfield

Currently, each `optional` scalar field uses `Option<T>` which requires storing a discriminant alongside the value. For a message with many optional fields, this wastes space. An alternative: store all scalar presence flags in a single `u64` (or a `[u64; N]` bitfield) and use plain `T` fields for the values. The public accessor API (`has_X()`, `set_X()`, `clear_X()`) is identical; only the generated implementation changes.

Google protobuf's generated C++ code already does this. The Rust implementation could adopt the same approach.

### `Box<str, A>` vs borrowed string

For use cases where messages are decoded and immediately read without modification, zero-copy decoding (returning `&'buf str` from string fields) would avoid all string allocations. This requires a lifetime parameter on the message type. See §8 of DESIGN.md for the future work item.

### Arena allocation

Arena allocators (e.g. `bumpalo`) already work with the current design via `new_in(&bump)`. All heap allocations for a message land in the same arena; dropping the arena frees them all at once without `Drop` overhead. This is the primary motivator for the `A: Allocator` generic parameter.
