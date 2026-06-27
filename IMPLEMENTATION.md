# puroro — Implementation Notes

This document describes the **internal implementation** of generated code. It covers storage types, encode/decode algorithms, and runtime helper usage. The stable **public interface** is specified in [DESIGN.md](DESIGN.md).

> **Note:** The public API described in DESIGN.md must remain stable even if the internal representations documented here change. For example, presence tracking for optional scalars currently uses `Option<T>` fields, but could be replaced by a per-message bitfield without any change to the accessor API.

All examples use the **editions reference schema** from DESIGN.md §4.

## Table of contents

1. [Storage types per field kind](#1-storage-types-per-field-kind)
2. [Struct layout](#2-struct-layout)
3. [Constructors and the stored allocator](#3-constructors-and-the-stored-allocator)
4. [Encode implementation](#4-encode-implementation)
5. [Decode implementation](#5-decode-implementation)
6. [Per-field encode/decode patterns](#6-per-field-encodedecode-patterns)
   - 6.1 [Scalar: implicit presence](#61-scalar-implicit-presence)
   - 6.2 [Scalar: explicit presence](#62-scalar-explicit-presence)
   - 6.3 [String fields](#63-string-fields)
   - 6.4 [Bytes fields](#64-bytes-fields)
   - 6.5 [Repeated scalar: packed](#65-repeated-scalar-packed)
   - 6.6 [Repeated scalar: non-packed (with packed fallback)](#66-repeated-scalar-non-packed-with-packed-fallback)
   - 6.7 [Repeated string fields](#67-repeated-string-fields)
   - 6.8 [Nested message fields](#68-nested-message-fields)
   - 6.9 [Open enum fields](#69-open-enum-fields)
   - 6.10 [Closed enum fields](#610-closed-enum-fields)
   - 6.11 [Oneof fields](#611-oneof-fields)
   - 6.12 [Unknown fields](#612-unknown-fields)
   - 6.13 [LEGACY_REQUIRED fields](#613-legacy_required-fields)
7. [Optimization opportunities](#7-optimization-opportunities)

---

## 1. Storage types per field kind

| Field kind | Internal storage type |
|---|---|
| Implicit-presence scalar (`IMPLICIT`) | `T` (e.g. `i32`) — the Rust primitive directly |
| Explicit-presence scalar (`EXPLICIT`) | `Option<T>` |
| Implicit-presence string | `allocator_api2::boxed::Box<str, A>` |
| Explicit-presence string | `Option<allocator_api2::boxed::Box<str, A>>` |
| `bytes` field (implicit presence) | `allocator_api2::vec::Vec<u8, A>` |
| `bytes` field (explicit presence) | `Option<allocator_api2::vec::Vec<u8, A>>` |
| Repeated scalar / string / message | `allocator_api2::vec::Vec<ElementType, A>` |
| Message field (always optional in struct) | `Option<allocator_api2::boxed::Box<MessageType<A>, A>>` |
| Open enum (`OPEN`) | `i32` (IMPLICIT) or `Option<i32>` (EXPLICIT) |
| Closed enum (`CLOSED`) | `Option<i32>` |
| Oneof group | `Option<OurEnum<A>>` |
| Unknown fields | `allocator_api2::vec::Vec<u8, A>` |
| `LEGACY_REQUIRED` field | Same as `EXPLICIT` (`Option<T>`) |

`Box<str, A>` (2 words: ptr + byte length) is preferred over `Vec<u8, A>` (3 words) for owned strings since strings are generally written once and then read. `Vec<u8, A>` is used for `bytes` fields where incremental appending is more natural.

> **Optimization note:** explicit-presence scalars currently use `Option<T>`. A future optimisation could use `T` storage with a per-message presence bitfield, reducing struct size at the cost of slightly more complex accessor code. The public API (`has_X()`, `set_X()`, `clear_X()`) would be identical. This is the approach used by Google protobuf's generated C++ code.

---

## 2. Struct layout

```rust
pub struct Task<A: Allocator = Global> {
    // ── User fields ─────────────────────────────────────────────────────────
    title:       Option<Box<str, A>>,           // field 1: EXPLICIT string
    score:       i32,                           // field 2: IMPLICIT i32
    max_retries: Option<i32>,                   // field 3: EXPLICIT i32, default 3
    owner_id:    Option<Box<str, A>>,           // field 4: LEGACY_REQUIRED string
    payload:     Option<Vec<u8, A>>,            // field 5: EXPLICIT bytes
    tag_ids:     Vec<i32, A>,                   // field 6: repeated packed int32
    scores:      Vec<i32, A>,                   // field 7: repeated expanded int32
    labels:      Vec<Box<str, A>, A>,           // field 8: repeated string
    status:      i32,                           // field 9: IMPLICIT open enum
    priority:    Option<i32>,                   // field 10: EXPLICIT closed enum
    assignee:    Option<Box<Address<A>, A>>,    // field 11: EXPLICIT message
    notification: Option<task::Notification<A>>, // fields 12–13: oneof

    // ── Infrastructure ──────────────────────────────────────────────────────
    _unknown_fields: Vec<u8, A>,
    // Retained allocator for setter / push methods.
    // ZST for Global; one pointer for &'arena Bump.
    _alloc: A,
}
```

The `_alloc` field is necessary because setters and push methods need to create new heap values without the caller providing an allocator. They clone `_alloc` at call time.

---

## 3. Constructors and the stored allocator

```rust
impl<A: Allocator + Clone> Task<A> {
    pub fn new_in(alloc: A) -> Self {
        Task {
            title:        None,
            score:        0,
            max_retries:  None,
            owner_id:     None,
            payload:      None,
            tag_ids:      Vec::new_in(alloc.clone()),
            scores:       Vec::new_in(alloc.clone()),
            labels:       Vec::new_in(alloc.clone()),
            status:       0,
            priority:     None,
            assignee:     None,
            notification: None,
            _unknown_fields: Vec::new_in(alloc.clone()),
            _alloc: alloc,
        }
    }
}

impl<A: Allocator + Clone + Default> Default for Task<A> {
    fn default() -> Self { Self::new_in(A::default()) }
}
```

The helper `str_to_box_in(s: &str, alloc: A) -> Box<str, A>` (runtime library) allocates a `Vec<u8, A>`, copies the bytes, converts to `Box<[u8], A>`, then reinterprets as `Box<str, A>` using one unsafe block inside the runtime.

---

## 4. Encode implementation

```rust
impl<A: Allocator + Clone> MessageEncode for Task<A> {
    fn encoded_len(&self) -> usize {
        let mut len = 0;
        // Field 1: EXPLICIT string — omit if None
        if let Some(s) = &self.title {
            len += encoded_len_len_field(1, s.len());
        }
        // Field 2: IMPLICIT i32 — omit if zero
        if self.score != 0 {
            len += encoded_len_varint_field(2, self.score as u64);
        }
        // Field 3: EXPLICIT i32 — omit if None (do NOT apply "omit if 0" rule)
        if let Some(v) = self.max_retries {
            len += encoded_len_varint_field(3, v as u64);
        }
        // … other fields …
        len += self._unknown_fields.len();
        len
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        if let Some(s) = &self.title {
            encode_len_field(1, s.as_bytes(), buf);
        }
        if self.score != 0 {
            encode_varint_field(2, self.score as u64, buf);
        }
        if let Some(v) = self.max_retries {
            encode_varint_field(3, v as u64, buf);
        }
        // … other fields …
        buf.put_slice(&self._unknown_fields);
    }
}
```

---

## 5. Decode implementation

```rust
impl<A: Allocator + Clone + Default> MessageDecode for Task<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match (field_number, wire_type) {
                (1, WireType::Len) => {
                    self.title = Some(decode_string_in(buf, self._alloc.clone())?);
                }
                (2, WireType::Varint) => {
                    self.score = decode_varint(buf)? as i32;
                }
                (3, WireType::Varint) => {
                    self.max_retries = Some(decode_varint(buf)? as i32);
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

### 6.1 Scalar: implicit presence

**Storage:** `T` (e.g. `i32`). **Omit if zero.**

```rust
// Encode:
if self.score != 0 {
    encode_varint_field(2, self.score as u64, buf);
}
// Decode:
(2, WireType::Varint) => { self.score = decode_varint(buf)? as i32; }
```

**All scalar types:**

| Proto type | Decode expression | Encode cast |
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

### 6.2 Scalar: explicit presence

**Storage:** `Option<T>`. **Omit if `None`; include even if the inner value is zero.**

```rust
// Encode:
if let Some(v) = self.max_retries {
    encode_varint_field(3, v as u64, buf);
}
// Decode: always wrap in Some
(3, WireType::Varint) => { self.max_retries = Some(decode_varint(buf)? as i32); }
```

### 6.3 String fields

**Storage:** `Option<Box<str, A>>` (EXPLICIT, edition 2024 default) or `Box<str, A>` (IMPLICIT).

**Encode (EXPLICIT — omit if None):**
```rust
if let Some(s) = &self.title {
    encode_len_field(1, s.as_bytes(), buf);
}
```

**Encode (IMPLICIT — omit if empty):**
```rust
if !self.name.is_empty() {
    encode_len_field(1, self.name.as_bytes(), buf);
}
```

**Decode:**
```rust
(1, WireType::Len) => {
    self.title = Some(decode_string_in(buf, self._alloc.clone())?);
    // IMPLICIT variant: self.name = decode_string_in(buf, self._alloc.clone())?;
}
```

`decode_string_in(buf, alloc)` reads a LEN-prefixed payload, validates UTF-8, returns `Box<str, A>`.

**Setter:**
```rust
pub fn set_title(&mut self, v: &str) {
    self.title = Some(str_to_box_in(v, self._alloc.clone()));
}
```

### 6.4 Bytes fields

**Storage:** `Option<Vec<u8, A>>` (EXPLICIT) or `Vec<u8, A>` (IMPLICIT).

**Encode/decode** follow the same EXPLICIT/IMPLICIT pattern as strings, using `encode_len_field` / `decode_bytes_in`.

**Setter:**
```rust
pub fn set_payload(&mut self, v: &[u8]) {
    let vec = self.payload.get_or_insert_with(|| Vec::new_in(self._alloc.clone()));
    vec.clear();
    vec.extend_from_slice(v);
}
```

### 6.5 Repeated scalar: packed

**Storage:** `Vec<i32, A>`.

**Encode:**
```rust
encode_packed_varint_field(6, &self.tag_ids, |v| *v as u64, buf);
```

**Decode (always accepts both packed and non-packed):**
```rust
(6, WireType::Varint) => {
    self.tag_ids.push(decode_varint(buf)? as i32);
}
(6, WireType::Len) => {
    let payload_len = decode_varint(buf)? as usize;
    let leftover = {
        let mut sub = (&mut *buf).take(payload_len);
        while sub.has_remaining() {
            self.tag_ids.push(decode_varint(&mut sub)? as i32);
        }
        sub.remaining()
    };
    if leftover > 0 { buf.advance(leftover); }
}
```

### 6.6 Repeated scalar: non-packed (with packed fallback)

**Encode** (one record per element):
```rust
for &v in &self.scores {
    encode_varint_field(7, v as u64, buf);
}
```

**Decode** is identical to the packed case above — both `VARINT` and `LEN` arms are always present regardless of the schema's `EXPANDED` declaration.

### 6.7 Repeated string fields

**Storage:** `Vec<Box<str, A>, A>`.

**Encode:**
```rust
for label in &self.labels {
    encode_len_field(8, label.as_bytes(), buf);
}
```

**Decode:**
```rust
(8, WireType::Len) => {
    let s = decode_string_in(buf, self._alloc.clone())?;
    self.labels.push(s);
}
```

### 6.8 Nested message fields

**Storage:** `Option<Box<Address<A>, A>>`.

**Encode:**
```rust
if let Some(addr) = &self.assignee {
    let msg_len = addr.encoded_len();
    encode_tag(11, WireType::Len, buf);
    encode_varint(msg_len as u64, buf);
    addr.encode_raw(buf);
}
```

**Decode** uses `Buf::take` to bound the sub-message to exactly the declared byte length:
```rust
(11, WireType::Len) => {
    let len = decode_varint(buf)? as usize;
    if buf.remaining() < len { return Err(DecodeError::TruncatedMessage); }
    let addr = self.assignee.get_or_insert_with(|| {
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

Merging into an existing sub-message (not replacing it) implements "concatenation = merge".

### 6.9 Open enum fields

**Storage:** `i32` (IMPLICIT) or `Option<i32>` (EXPLICIT).

```rust
// IMPLICIT — omit if zero:
if self.status != 0 {
    encode_varint_field(9, self.status as u64, buf);
}
(9, WireType::Varint) => { self.status = decode_varint(buf)? as i32; }
```

### 6.10 Closed enum fields

**Storage:** `Option<i32>`. Unknown values are diverted to `_unknown_fields`.

```rust
// Encode (EXPLICIT — include even if value is the zero variant):
if let Some(v) = self.priority {
    encode_varint_field(10, v as u64, buf);
}

// Decode:
(10, WireType::Varint) => {
    let raw = decode_varint(buf)? as i32;
    if Priority::try_from(raw).is_ok() {
        self.priority = Some(raw);
    } else {
        // Unknown value → preserve as unknown field for round-trip
        save_unknown_varint_field(10, raw as u64, &mut self._unknown_fields);
    }
}
```

`save_unknown_varint_field` is provided by the runtime library.

### 6.11 Oneof fields

**Storage:** `Option<task::Notification<A>>`.

```rust
// Encode:
if let Some(n) = &self.notification {
    match n {
        task::Notification::EmailAddress(s) => encode_len_field(12, s.as_bytes(), buf),
        task::Notification::PhoneNumber(s)  => encode_len_field(13, s.as_bytes(), buf),
    }
}

// Decode: each variant overwrites the previous one
(12, WireType::Len) => {
    let s = decode_string_in(buf, self._alloc.clone())?;
    self.notification = Some(task::Notification::EmailAddress(s));
}
(13, WireType::Len) => {
    let s = decode_string_in(buf, self._alloc.clone())?;
    self.notification = Some(task::Notification::PhoneNumber(s));
}
```

### 6.12 Unknown fields

**Storage:** `Vec<u8, A>` — a valid partial protobuf wire stream.

```rust
// Accumulate during decode:
_ => { skip_field_and_save(field_number, wire_type, buf, &mut self._unknown_fields)?; }

// Emit during encode:
buf.put_slice(&self._unknown_fields);
```

### 6.13 LEGACY_REQUIRED fields

**Storage:** `Option<T>` (identical to `EXPLICIT`).

The only difference from `EXPLICIT` is the generated `validate()` method:

```rust
pub fn validate(&self) -> Result<(), DecodeError> {
    if self.owner_id.is_none() {
        return Err(DecodeError::MissingRequiredField { field_number: 4 });
    }
    Ok(())
}

pub fn decode_strict<B: Buf>(buf: B) -> Result<Self, DecodeError>
where
    Self: Default + MessageDecode,
{
    let msg = <Self as MessageDecode>::decode(buf)?;
    msg.validate()?;
    Ok(msg)
}
```

The encode and decode arms are identical to an `EXPLICIT` field.

---

## 7. Optimization opportunities

### Presence bitfield

Currently each `EXPLICIT`-presence scalar uses `Option<T>`, which stores a discriminant alongside the value (e.g. `Option<i32>` is typically 8 bytes). For a message with many optional scalars, a per-message `u64` (or `[u64; N]`) bitfield could replace the discriminants:

```rust
// Hypothetical optimised layout:
pub struct Task<A: Allocator = Global> {
    _presence: u64,  // bit i set ↔ field i is present
    score:     i32,  // plain T, presence checked via bit
    // …
}
```

The public accessor API (`has_X()`, `set_X()`, `clear_X()`, `X()`) is identical in both representations. This is the approach used by Google protobuf's generated C++ and Java code.

### Zero-copy string fields

For use cases where messages are decoded and immediately consumed without modification, zero-copy decoding (`&'buf str` instead of `Box<str, A>`) would eliminate all string allocations. This requires a lifetime parameter on the message type. See DESIGN.md §8 for the future work item.

### Arena allocation

Arena allocators (`&bumpalo::Bump`, etc.) already work with the current design via `new_in(alloc)`. All heap allocations for a message and its nested messages land in the same arena; dropping the arena frees everything at once without `Drop` overhead.
