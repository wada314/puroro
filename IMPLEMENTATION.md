# puroro — Implementation Notes

This document describes the **internal implementation** of generated code. It covers storage types, encode/decode algorithms, and runtime helper usage. The stable **public interface** is specified in [DESIGN.md](DESIGN.md).

> **Note:** The public API described in DESIGN.md must remain stable even if the internal representations documented here change. For example, presence tracking for optional scalars currently uses `Option<T>` fields, but could be replaced by a per-message bitfield without any change to the accessor API.

## Table of contents

0. [Project context](#0-project-context)
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
8. [`TaskLazy` storage sketch](#8-tasklazy-storage-sketch)

---

## 0. Project context

| Crate | Responsibility |
|---|---|
| **`protobuf-core`** | Submodule providing wire-format primitives (varint, tags, field I/O). Used by tooling and potentially by `puroro` internals; **generated code does not import it directly**. |
| **`puroro`** | Message runtime imported by generated code (`MessageEncode`, `MessageDecode`, `Optional`, encode/decode helpers). |
| **`protoc` plugin** | Emits the Rust types and `impl` blocks described here. Primary invocation path; see [DESIGN.md §0](DESIGN.md#0-project-architecture). |

All examples below use the **`Task` / `Address` reference schema** from [DESIGN.md §4](DESIGN.md#reference-schema). That pair is the canonical design example — not a throwaway fixture.

### Runtime stubs (implementation in progress)

The design in DESIGN.md is ahead of the current `puroro` runtime in a few areas:

| Area | Current behaviour | Target behaviour |
|---|---|---|
| UTF-8 validation | `decode_string_in` always validates and returns `DecodeError::InvalidUtf8`. | Generated code calls `decode_string_in` (VERIFY) or `decode_string_unchecked_in` (NONE) based on the field's `utf8_validation` feature. |
| Recursion limit | Not enforced during nested-message merge. | Generated `merge_from` passes a decremented depth counter; depth 0 → `DecodeError::RecursionLimitExceeded`. |
| Group wire types | `skip_field_and_save` / `skip_field` return `DecodeError::InvalidTag` for `SGroup` / `EGroup`. | Unchanged — groups are not preserved; error or ignore is acceptable per DESIGN.md §0. |

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

No `Optional` is involved — `fn score(&self) -> i32` returns the value directly.

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

**Accessor implementation** — the `HasDefault` implementor is defined locally inside the
method body.  Using `impl HasDefault<i32>` as the second type parameter of `Optional`
keeps the concrete type private while allowing the compiler to infer it.

```rust
// Generated for: optional int32 max_retries = 3 [default = 3];
pub fn max_retries(&self) -> ::puroro::Optional<i32, impl ::puroro::HasDefault<i32>> {
    struct Default3;
    impl ::puroro::HasDefault<i32> for Default3 { const DEFAULT: i32 = 3; }
    ::puroro::Optional::new(self.max_retries, Default3)
    // Note: Optional exposes only get() and is_set() — no get_opt() or From<Option>
}

pub fn max_retries_raw(&self) -> i32 { self.max_retries().get() }
pub fn has_max_retries(&self) -> bool { self.max_retries().is_set() }
pub fn set_max_retries(&mut self, v: i32) { self.max_retries = Some(v); }
pub fn clear_max_retries(&mut self) { self.max_retries = None; }
```

On generated **traits**, `max_retries_raw` and `has_max_retries` are **default methods** with the same bodies shown above.  The native `impl` block on `Task<A>` implements the `Optional` accessor directly from the internal `Option<i32>` field and may expose `_raw` / `has_` either by delegating to `max_retries()` or by reading the field (as shown here).

**Fallible trait impl** wraps the infallible `Optional` accessor:

```rust
// In impl TaskMessageFallible for Task<A> { type Error = Infallible; … }
fn max_retries(&self) -> Result<Optional<i32, impl HasDefault<i32>>, Infallible> {
    Ok(TaskMessage::max_retries(self))
}
// max_retries_raw / has_max_retries inherit the fallible trait's default methods
```

Because `Optional<i32, Default3>` is a concrete struct with no custom `Drop`,
`task.max_retries().get()` chains directly without a `let` binding.
No `get_opt()` is provided: `Optional` is intentionally not convertible to `Option<T>`
to preserve the default-value semantics.

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

`decode_string_in(buf, alloc)` reads a LEN-prefixed payload, validates UTF-8 when `utf8_validation = VERIFY`, returns `Box<str, A>`.

When the schema sets `utf8_validation = NONE` on a string field, generated decode arms call a runtime helper that skips UTF-8 validation (unchecked conversion after copy). The `VERIFY` path is what `decode_string_in` implements today; the `NONE` helper is **not yet exposed** in the runtime.

**Setter:**
```rust
pub fn set_title(&mut self, v: &str) {
    self.title = Some(str_to_box_in(v, self._alloc.clone()));
}
```

**Accessor implementation** — for string fields the `HasDefault<&'a str>` implementor uses a
lifetime-generic blanket impl so the same private struct works for any borrow lifetime.

```rust
// Generated for: string title = 1 [default = "N/A"];  (EXPLICIT presence)
pub fn title<'s>(&'s self)
    -> ::puroro::Optional<&'s str, impl ::puroro::HasDefault<&'s str>>
{
    struct DefaultNA;
    // &'static str coerces to &'a str for any 'a, so the blanket impl works:
    impl<'a> ::puroro::HasDefault<&'a str> for DefaultNA {
        const DEFAULT: &'a str = "N/A";
    }
    ::puroro::Optional::new(self.title.as_deref(), DefaultNA)
}

pub fn title_raw<'s>(&'s self) -> &'s str { self.title().get() }
pub fn has_title(&self) -> bool { self.title().is_set() }
```

Because `Optional<&'s str, DefaultNA>` is a concrete struct (not an opaque `impl Trait`
return), the borrow checker sees its trivial drop, and `task.title().get()` chains
directly — no `let` binding required.

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
        addr.merge_from(&mut sub_buf)?;  // TODO: pass recursion depth; enforce RecursionLimitExceeded
        sub_buf.remaining()
    };
    if leftover > 0 { buf.advance(leftover); }
}
```

Merging into an existing sub-message (not replacing it) implements "concatenation = merge".

> **Recursion limit (stub).** Nested-message decode should decrement a depth counter and return `DecodeError::RecursionLimitExceeded` at depth zero. The error variant exists in `puroro`; generated code and/or a `merge_from_with_depth` runtime helper will enforce the limit — not yet implemented.

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

**Deprecated groups.** `SGroup` / `EGroup` wire types are not saved to `_unknown_fields`. The runtime returns `DecodeError::InvalidTag` (or may skip/panic). Round-trip preservation of group payloads is not a goal — see [DESIGN.md §0](DESIGN.md#0-project-architecture).

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

### Recursion limit enforcement

Generated `merge_from` for messages containing nested message fields (e.g. `Task.assignee → Address`) should thread a remaining-depth parameter. A planned runtime signature:

```rust
// Illustrative — not yet in puroro:
fn merge_from_with_depth<B: Buf>(&mut self, buf: &mut B, depth: u32) -> Result<(), DecodeError>;
```

When `depth == 0` before recursing into a sub-message, return `DecodeError::RecursionLimitExceeded`. The default `merge_from` wrapper starts at a fixed limit (e.g. 100). **Status: stub** — error type only; no enforcement yet.

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

---

## 8. `TaskLazy` storage sketch

Planned internal layout for the reference `TaskLazy<A>` message (see [DESIGN.md §8 — lazy parse timing](DESIGN.md#tasklaya--lazy-parse-timing)).

### Message-level storage

```rust
pub struct TaskLazy<A: Allocator = Global> {
    /// Shared wire stream (`Arc` internally).  Subslice via `.slice()` for nested messages.
    _wire: Bytes,

    /// Per-field decode / scan caches (interior mutability).
    title_cache: RefCell<FieldCache<Box<str, A>>>,
    max_retries_cache: RefCell<FieldCache<i32>>,
    assignee_cache: RefCell<FieldCache<AddressLazy<A>>>,
    tag_ids_cache: RefCell<RepeatedCache<i32, A>>,
    // …

    _alloc: A,   // used for decoded/cached values, not for _wire
}

impl<A: Allocator + Clone> AddressLazy<A> {
    /// Constructs from a `Bytes` subslice (typically `parent._wire.slice(start..end)`).
    fn from_wire(wire: Bytes, alloc: A) -> Self { … }
}
```

### `merge_from`: append only

```rust
fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
    let chunk = buf.copy_to_bytes(buf.remaining());
    self._wire = cat_bytes(self._wire.clone(), chunk);
    self.invalidate_all_field_caches();
    Ok(())
}
```

No tags are read.  Concatenating on merge may allocate a new backing buffer; nested field access does **not** copy payload bytes (see nested message below).

### Field cache (singular)

```rust
enum FieldCache<T> {
    Uninitialized,
    Absent,                    // wire-scanned; field not found
    WireFound { wire: Bytes }, // payload subslice located, not semantically decoded
    Parsed(T),
    Failed(DecodeError),
}
```

`WireFound { wire: Bytes }` holds a `Bytes` subslice into `_wire` (or the full `_wire` range for that payload) — no separate copy.

### Nested message: zero-copy child

```rust
// In TaskLazy::assignee() — after wire scan locates field 11 LEN payload:
let child_wire = self._wire.slice(start..start + len);
let child = AddressLazy::from_wire(child_wire, self._alloc.clone());
*cache = FieldCache::Parsed(child);
// child._wire shares Arc allocation with parent._wire
```

### Getter flow (explicit-presence string)

```rust
fn title(&self) -> Result<Optional<…>, DecodeError> {
    let mut cache = self.title_cache.borrow_mut();
    match &*cache {
        FieldCache::Parsed(s) => return Ok(Optional::new(Some(s.as_ref()), TitleDefault)),
        FieldCache::Absent => return Ok(Optional::new(None, TitleDefault)),
        FieldCache::Failed(e) => return Err(e.clone()),
        _ => {}
    }
    let payload = wire_find_last_len_payload(&self._wire, 1)?; // scan _wire
    match payload {
        None => { *cache = FieldCache::Absent; Ok(Optional::new(None, TitleDefault)) }
        Some(range) => {
            let s = decode_string_semantics(&self._wire[range], self._alloc.clone())?;
            *cache = FieldCache::Parsed(s);
            Ok(Optional::new(Some(/* borrow from cache */), TitleDefault))
        }
    }
}

fn has_title(&self) -> Result<bool, DecodeError> {
    // Use cache if known; else wire-scan _wire for field 1 (skip payload, no UTF-8)
    …
}
```

### Repeated field: cursor in `_wire`

```rust
struct RepeatedCache<T, A> {
    state: FieldCache<Vec<T, A>>,
    scan_cursor: usize,   // next wire scan starts here in _wire
}

// tag_ids().next():
//   Parsed(vec) + index → return cached elements
//   else scan _wire from scan_cursor for field 6, decode one occurrence,
//   advance scan_cursor; optionally append to partial Vec
```
