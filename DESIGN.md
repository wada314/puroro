# puroro — Design Document

This document specifies the **public interface** of the `puroro` Protocol Buffers runtime library and the accessor API that the code generator must emit. It intentionally omits internal implementation details — storage types, encode/decode algorithms, and the composable field catalog — which live in the sibling **`puroro-rt`** crate and are documented in [IMPLEMENTATION.md](IMPLEMENTATION.md).

## Table of contents

0. [Project architecture](#0-project-architecture)
1. [Goals](#1-goals)
2. [Wire format overview](#2-wire-format-overview)
3. [Runtime trait API](#3-runtime-trait-api)
4. [Generated code specification](#4-generated-code-specification)
   - 4.0 [Generated per-message traits](#40-generated-per-message-traits)
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
   - 8.1 [Specialized message implementations](#specialized-message-implementations) (`TaskLazy<A>`, `TaskView<'buf>`) — includes [lazy parse timing](#tasklaya--lazy-parse-timing)
   - 8.2 [Other future work](#other-future-work)

---

## 0. Project architecture

The puroro project comprises several crates and tools with distinct roles:

| Component | Role |
|---|---|
| **`protobuf-core`** (git submodule) | Wire-format **primitives** — `Varint`, `Tag`, `WireType`, field I/O traits, varint read/write. Used by `puroro` and `puroro-rt`; **generated code does not import it directly**. |
| **`puroro`** | **Stable user-facing runtime API** — `Message`, `Optional`, `HasDefault`, `DecodeError` / `EncodeError`, `WireType`, `UnknownField` / `UnknownPayload`. Library users depend on this crate; generated message code imports it for traits, accessor return types, and error handling. |
| **`puroro-rt`** | **Generated-code runtime** — composable field catalog (`fields::*`, `MessageCommon`), wire encode/decode helpers (`encode` / `decode` modules), `ProtoDefault`, and allocator-aware string/bytes utilities. Emitted generated code imports this crate (transitively for end users). Semver is looser than `puroro`; do not depend on it directly from application code. See [IMPLEMENTATION.md §4](IMPLEMENTATION.md#4-shared-infrastructure). |
| **Code generator** (`protoc` plugin) | Reads `.proto` input (via `protoc`) and emits Rust source implementing the API defined in this document. **Primary execution path:** register as a `protoc` plugin (`--puroro_out=…`). Other invocation styles (standalone CLI, `build.rs` wrapper, etc.) are permitted but not required. Emits fully-qualified paths into both `::puroro::…` (traits, `Optional`, errors) and `::puroro_rt::…` (field catalog, wire helpers). |

**Reference schema.** The `Task` and `Address` messages in [§4 Reference schema](#reference-schema) are the **canonical examples** for describing and reviewing generated code. All field-pattern subsections (§4.1–4.9) and [IMPLEMENTATION.md](IMPLEMENTATION.md) use this same schema unless noted otherwise.

**Non-goals.**

- **No compatibility** with other Rust protobuf libraries (`prost`, `protobuf`, `quick-protobuf`, etc.). API shapes, type names, and generated module layout are puroro-specific.
- **Extensions** (§6.5) are out of scope for the current design.

**Runtime completeness.** The public API in this document is normative and intended to remain stable; the **`puroro-rt` implementation is still evolving**. Items designed but not yet fully wired:

| Feature | Design intent | Implementation status |
|---|---|---|
| `utf8_validation` (`VERIFY` / `NONE`) | Generated decode paths honour the per-field Editions setting. | Error type and `VERIFY` path exist; `NONE` bypass and per-field dispatch in generated code are **pending**. |
| Recursion limit | Nested-message `merge_from` enforces a depth limit; excess depth → `DecodeError::RecursionLimitExceeded`. | Error variant exists; depth tracking in generated code / runtime helper is **not yet implemented** (stub). |
| Deprecated groups (`SGroup` / `EGroup`) | Never generated; not preserved on decode. | Decoder may return `DecodeError::InvalidTag`, skip, or panic — round-trip fidelity for groups is **not** a goal. |

---

## 1. Goals

- **Protobuf spec compliance.** Support the canonical wire format (varints, I32, I64, LEN records, packed repeated, oneofs, unknown fields) for proto2, proto3, and editions. Deprecated group wire types (`SGroup` / `EGroup`) are never generated; the decoder does not preserve them (see [§0](#0-project-architecture)).
- **Allocator support.** Every generated type is generic over `A: Allocator` using the `allocator-api2` crate. Arena allocators (e.g. `bumpalo`) and custom pools are first-class citizens.
- **Performance-oriented interface.** Accessors return borrowed references (`&str`, `&[u8]`, `&[T]`), never freshly allocated containers. The `encode_to_vec` / `encode_to_bytes` convenience methods allocate, but `encode_raw` does not.
- **Rust idioms.** Private fields accessed via generated accessor methods; `Optional<T, impl HasDefault<T>>` for explicit-presence scalar, string, and enum fields; `Option<&M<A>>` for optional message fields; no `unsafe` in user-visible APIs.
- **Implementation flexibility.** The public interface described here must remain stable even if internal storage representations change. Eager messages use a per-message presence bitfield (see [IMPLEMENTATION.md §10](IMPLEMENTATION.md#10-presence-bit-indices)); the accessor API is unchanged if storage layout evolves.
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

Wire types 3 and 4 (SGROUP / EGROUP) are deprecated. The encoder must never emit them. The decoder does not preserve group payloads for round-trip; encountering a group tag may be rejected with an error or otherwise ignored (see [§0](#0-project-architecture)).

| Protobuf type | Wire type |
|---|---|
| `int32`, `int64`, `uint32`, `uint64`, `sint32`, `sint64`, `bool`, `enum` | VARINT |
| `fixed32`, `sfixed32`, `float` | I32 |
| `fixed64`, `sfixed64`, `double` | I64 |
| `string`, `bytes`, embedded messages, packed repeated | LEN |

---

## 3. Runtime trait API

The **`puroro`** crate exposes the shared [`Message`](#message) trait and the accessor helpers below. Generated code imports these items from `puroro`; the field catalog and wire helpers come from `puroro-rt` (see [§0](#0-project-architecture)). Library users who handle decode errors or write generic code over `Message` depend on **`puroro` only**.

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

    /// Value or proto-declared default; always returns something meaningful.
    pub fn get(&self) -> T;

    /// True when explicitly set; false when using the proto default.
    pub fn is_set(&self) -> bool;

    // No get_opt() / From<Optional> for Option<T>:
    // Converting to Option<T> would conflate "not set" with "no value",
    // undermining the default-value semantics of Optional.
}
```

`Optional` is a **concrete struct** (not a trait), so the borrow checker can always verify its trivial drop — enabling direct chaining for both scalar and string accessors.

**No `Option<T>` conversion is provided.**  Proto explicit-presence fields always carry a meaningful value (either the explicit value or the declared default).  If presence matters, call `is_set()` first:

```rust
if task.max_retries().is_set() {
    // get() returns the explicit value
} else {
    // get() still works — returns the proto default (3)
}
```

The concrete `D` type is a private zero-sized struct defined locally inside the accessor method body.  The return type uses `impl HasDefault<T>` to keep `D` opaque:

```rust
pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> { … }
pub fn title<'s>(&'s self) -> Optional<&'s str, impl HasDefault<&'s str>> { … }

// Both chain directly without a let binding:
let n: i32  = task.max_retries().get();
let s: &str = task.title().get();
if task.max_retries().is_set() { … }
```

**Lazy implementations (`TaskLazy`).** On the eager path, `Optional::new` receives `Some(value)` or `None` derived from the internal presence bitfield and value slot.  On the lazy path, getters **wire-scan** the stored buffer and semantically decode on demand; the `Optional` getter returns `Err` before constructing `Optional` if decode fails (e.g. `InvalidUtf8`).  `has_*()` may wire-scan for presence without semantic decode.  See [§8 — `TaskLazy` lazy parse timing](#tasklaya--lazy-parse-timing).

### `Message`

Implemented by **every** generated message (C++ `MessageLite`-like surface: codec + shared infrastructure). Field accessors stay as inherent methods so proto field names do not collide with these helpers.

```rust
pub trait Message: Sized {
    type Alloc: Allocator + Clone;
    fn new_in(alloc: Self::Alloc) -> Self;

    // Codec (required)
    fn encoded_len(&self) -> usize;
    fn encode_raw<B: bytes::BufMut>(&self, buf: &mut B);
    fn merge_from<B: bytes::Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    // Codec (provided)
    fn encode_to_vec(&self) -> Vec<u8>;
    fn encode_to_bytes(&self) -> bytes::Bytes;
    fn decode<B: bytes::Buf>(buf: B) -> Result<Self, DecodeError>
    where
        Self: Default;

    // Infrastructure (required)
    fn unknown_fields(&self) -> impl Iterator<Item = UnknownField<'_>> + '_;
    fn validate(&self) -> Result<(), DecodeError>; // Ok(()) when no LEGACY_REQUIRED
}
```

`encode_raw` / `merge_from` are generic over `BufMut` / `Buf` so the compiler can monomorphise; the trait is **non-object-safe** by design.

**Merge semantics** (identical across proto2, proto3, editions): singular scalar — last wins; singular message — recursive merge; repeated — append; unknown fields — accumulated for round-trip.

**Non-deterministic field order.** The encoder may emit known fields in any order. Two encodes of the same logical message are not guaranteed to produce identical bytes; compare with `PartialEq`, not with `encode_raw` output equality. See [IMPLEMENTATION.md §12](IMPLEMENTATION.md#12-message-level-wire-io).

**Name conflicts.** If a proto field is named `validate`, `unknown_fields`, `encoded_len`, etc., the inherent field getter wins; use `Message::validate(&msg)` (UFCS).

**Allocator.** `Alloc` is the message's single allocator type. Nested message fields construct children with `Message::new_in` under the bound `M: Message<Alloc = A>` (same `A` as the parent). An inherent `new()` for `Global` may still be emitted on the concrete type. `decode` requires `Self: Default` (typically `A: Default`); otherwise use `new_in(alloc)` then `merge_from`.

---

## 4. Generated code specification

This section is the normative reference for what the code generator emits. All field patterns are illustrated using a single **editions** reference schema, since editions can express every variant (implicit/explicit presence, custom defaults, required-like semantics, open/closed enums, packed/expanded repeated) in one file.

For each message type the code generator produces **three kinds of output**:

1. **Two traits** — a stable API contract that multiple implementations satisfy (§4.0).
2. **The primary struct** — a full-featured owned implementation (§4.1–4.9), internally a product of **`puroro_rt::fields` catalog types** + shared `MessageCommon` (see [IMPLEMENTATION.md §2](IMPLEMENTATION.md#2-architecture-overview)).
3. **(Future) Specialized structs** — alternative implementations for specific performance scenarios (§8).

Generated Rust is not hand-edited; the plugin still emits **section banners, proto field labels, and `merge_from` dispatch comments** so build output is navigable when debugging. Convention: [IMPLEMENTATION.md §9 — Generated code comments](IMPLEMENTATION.md#generated-code-comments). Reference output: [`sample-generated/`](sample-generated/).

---

### 4.0 Generated per-message traits

For each message the generator emits two traits. User code that is generic over a message type depends on these traits, not on any concrete struct.

#### `FooMessage` — infallible, for eager implementations

All accessors succeed unconditionally.  The primary struct `Task<A>` implements this trait.

```rust
// Generated for: message Task { … }
pub trait TaskMessage {
    // IMPLICIT scalar — always returns a value
    fn score(&self) -> i32;

    // EXPLICIT scalar — Optional is the required accessor; _raw / has_ are trait defaults
    fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>>;
    fn max_retries_raw(&self) -> i32 { self.max_retries().get() }
    fn has_max_retries(&self) -> bool { self.max_retries().is_set() }

    // EXPLICIT string — same pattern
    fn title<'s>(&'s self) -> Optional<&'s str, impl HasDefault<&'s str>>;
    fn title_raw<'s>(&'s self) -> &'s str { self.title().get() }
    fn has_title(&self) -> bool { self.title().is_set() }
    fn set_title(&mut self, v: &str);
    fn clear_title(&mut self);

    // Repeated scalar — slice reference (O(1) random access)
    fn tag_ids(&self) -> &[i32];
    fn push_tag_id(&mut self, v: i32);
    fn clear_tag_ids(&mut self);

    // Repeated string — iterator of str references (implementation-agnostic)
    fn labels(&self) -> impl Iterator<Item = &str> + '_;
    fn push_label(&mut self, v: &str);
    fn clear_labels(&mut self);

    // Nested message
    fn assignee(&self) -> Option<&impl AddressMessage>;
    fn assignee_mut(&mut self) -> &mut impl AddressMessage;
    fn has_assignee(&self) -> bool;

    // Open enum (IMPLICIT) — Optional; `is_set` when wire value is non-zero
    fn status(&self) -> Optional<Status, impl HasDefault<Status>>;

    // Closed enum (EXPLICIT) — Optional; `has_` is a trait default
    fn priority(&self) -> Optional<Priority, impl HasDefault<Priority>>;
    fn has_priority(&self) -> bool {
        self.priority().is_set()
    }

    // Oneof
    fn notification(&self) -> Option<&task::Notification<impl Allocator>>;
    fn set_notification(&mut self, v: Option<task::Notification<impl Allocator>>);
}
```

#### `FooMessageFallible` — Result-returning, for lazy implementations

All accessors return `Result`; even presence checks may fail (e.g., if the field has not yet been parsed from the wire).  The primary struct `Task<A>` also implements this trait with `Error = Infallible`.

Explicit-presence fields mirror [`TaskMessage`](#foomessage--infallible-for-eager-implementations): the **`Optional` accessor is the required method**; `_raw` and `has_` accessors are **provided as default implementations** that delegate to it.

```rust
pub trait TaskMessageFallible {
    type Error;

    fn score(&self) -> Result<i32, Self::Error>;

    // EXPLICIT scalar — Optional is the required accessor
    fn max_retries(&self) -> Result<Optional<i32, impl HasDefault<i32>>, Self::Error>;
    fn max_retries_raw(&self) -> Result<i32, Self::Error> {
        Ok(self.max_retries()?.get())
    }
    fn has_max_retries(&self) -> Result<bool, Self::Error> {
        Ok(self.max_retries()?.is_set())
    }

    // EXPLICIT string — same pattern
    fn title<'s>(&'s self) -> Result<Optional<&'s str, impl HasDefault<&'s str>>, Self::Error>;
    fn title_raw<'s>(&'s self) -> Result<&'s str, Self::Error> {
        Ok(self.title()?.get())
    }
    fn has_title(&self) -> Result<bool, Self::Error> {
        Ok(self.title()?.is_set())
    }

    // EXPLICIT bytes — same pattern (field 5: payload)
    fn payload<'s>(&'s self) -> Result<Optional<&'s [u8], impl HasDefault<&'s [u8]>>, Self::Error>;
    fn payload_raw<'s>(&'s self) -> Result<&'s [u8], Self::Error> {
        Ok(self.payload()?.get())
    }
    fn has_payload(&self) -> Result<bool, Self::Error> {
        Ok(self.payload()?.is_set())
    }

    // LEGACY_REQUIRED string — same Optional pattern as EXPLICIT (field 4: owner_id)
    fn owner_id<'s>(&'s self) -> Result<Optional<&'s str, impl HasDefault<&'s str>>, Self::Error>;
    fn owner_id_raw<'s>(&'s self) -> Result<&'s str, Self::Error> {
        Ok(self.owner_id()?.get())
    }
    fn has_owner_id(&self) -> Result<bool, Self::Error> {
        Ok(self.owner_id()?.is_set())
    }

    // Repeated: lazy iterator; early termination is possible
    fn tag_ids(&self) -> impl Iterator<Item = Result<i32, Self::Error>> + '_;
    fn labels(&self) -> impl Iterator<Item = Result<&str, Self::Error>> + '_;

    // Nested message — the sub-message is also fallible
    fn assignee(&self) -> Result<Option<impl AddressMessageFallible<Error = Self::Error>>, Self::Error>;

    fn status(&self) -> Result<Optional<Status, impl HasDefault<Status>>, Self::Error>;
    fn priority(&self) -> Result<Optional<Priority, impl HasDefault<Priority>>, Self::Error>;
    fn has_priority(&self) -> Result<bool, Self::Error> {
        Ok(self.priority()?.is_set())
    }
}
```

> **Trait default methods for explicit presence.**  On both `TaskMessage` and `TaskMessageFallible`, generated traits emit `_raw` and `has_` as **default method bodies** that call the `Optional` accessor.  Concrete `impl` blocks only need to implement the `Optional`-returning method (plus any field-specific logic).  Native methods on the struct (§4.0 note below) may still expose all three names for ergonomics, but trait implementors inherit the defaults for free.

#### Primary struct implements both

For **`Task<A>`** (eager):

- **`TaskMessage`** — implement required accessors; `_raw` / `has_` for explicit-presence fields inherit trait defaults.
- **`TaskMessageFallible`** — `type Error = Infallible`; wrap each infallible accessor in `Ok(…)`. Explicit-presence fallible getters return `Ok(TaskMessage::field(self))` for the `Optional` accessor.

Using `Infallible` signals at compile time that generic code over `TaskMessageFallible` never encounters a real error when given `Task<A>`.

On **`TaskLazy<A>`**, the same trait signatures apply, but getters wire-scan `_wire` and decode on demand. Explicit-presence getters return `Err` before constructing `Optional` on semantic failure. `has_*()` wire-scans for presence only. See [§8 — `TaskLazy`](#tasklaya--lazy-parse-timing).

> **Native methods vs trait methods.** Each concrete struct also has a native `impl` block. Traits are the interoperability contract; native methods may expose richer APIs (e.g. `&[i32]` instead of an iterator on the trait, mutation helpers not on the trait). Same pattern as `Vec<T>` implementing `Iterator` while also providing `push` and `sort`.

---

### Reference schema

> **Canonical example.** `Task` and `Address` below are the standard messages used throughout this document and in [IMPLEMENTATION.md](IMPLEMENTATION.md) to illustrate every generated-code pattern. When reviewing or extending the design, treat these two types as the single source of truth for accessor shapes, storage layout, and encode/decode behaviour.

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

    // Fields 12–15: Oneof (presence is intrinsic to oneof). Deliberately
    // heterogeneous to show LEN, VARINT, and message variants.
    oneof notification {
        string  email_address = 12;
        string  phone_number  = 13;
        int32   webhook_id    = 14 [default = -1];  // VARINT + custom default
        Address postal        = 15;  // message variant
    }
}
```

The subsections below document each field pattern in terms of its generated accessor API.

---

### 4.1 Scalar fields

Scalar types are copy types in Rust (`i32`, `bool`, `f32`, …). In the runtime catalog these are **singular** fields: **non-repeated**, covering both presence-tracked (“optional” / `EXPLICIT`) and non-presence-tracked (`IMPLICIT`) cases. Cardinality (singular vs repeated) is separate from presence (`FieldPresence`).

The key variation for scalars is **field presence**.

#### Implicit presence (`features.field_presence = IMPLICIT`)

Equivalent to proto3's default singular scalar behaviour.

- Value accessor: `fn score(&self) -> i32`
- Setter: `fn set_score(&mut self, v: i32)`
- Wire rule: field absent from the wire when value equals the type-zero (`0`, `false`, `0.0`).

#### Explicit presence (`features.field_presence = EXPLICIT`, edition 2024 default)

Equivalent to proto2 `optional`. Presence is tracked independently of value.
The **`Optional` accessor is the primary API**; `_raw` and `has_` are convenience accessors provided as **default methods on generated traits** (see [§4.0](#40-generated-per-message-traits)):

| Method | Return type | Description |
|---|---|---|
| `max_retries()` | `Optional<i32, impl HasDefault<i32>>` | **Required** trait method — `get()` and `is_set()` |
| `max_retries_raw()` | `i32` | Trait **default** — `self.max_retries().get()` |
| `has_max_retries()` | `bool` | Trait **default** — `self.max_retries().is_set()` |

- Setter: `fn set_max_retries(&mut self, v: i32)` — makes `has_max_retries()` true
- Clearer: `fn clear_max_retries(&mut self)` — makes `has_max_retries()` false
- Wire rule: field absent when `has_max_retries()` is `false`; present even when the value is zero.

The `[default = 3]` option means `max_retries().get()` and `max_retries_raw()` return `3` when unset.

Because `Optional` is a concrete struct with no custom `Drop`, chaining compiles directly:

```rust
let n: i32 = task.max_retries().get();  // value or default
if task.max_retries().is_set() { … }    // presence check
let n: i32 = task.max_retries_raw();   // bypasses Optional
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

**Explicit presence:**

The **`Optional` accessor is the primary API**; `_raw` and `has_` are trait default methods (see [§4.0](#40-generated-per-message-traits)):

| Method | Return type | Description |
|---|---|---|
| `title()` | `Optional<&'s str, impl HasDefault<&'s str>>` | **Required** trait method — `get()` and `is_set()` |
| `title_raw()` | `&str` | Trait **default** — `self.title().get()` |
| `has_title()` | `bool` | Trait **default** — `self.title().is_set()` |
| `set_title(&mut self, v: &str)` | `()` | Copies string data; marks field as set |
| `clear_title(&mut self)` | `()` | Marks field as unset |

Because `Optional` is a concrete struct, chaining compiles directly for string fields too:

```rust
let s: &str = task.title().get();   // value or default
if task.title().is_set() { … }      // presence check
let s: &str = task.title_raw();     // bypasses Optional
```

Wire rule: absent when `has_title()` is false (EXPLICIT) or `""` (IMPLICIT).

**UTF-8 validation** (Editions `utf8_validation` feature): when `VERIFY` (default), invalid UTF-8 in a string payload causes `DecodeError::InvalidUtf8` during decode. When `NONE`, bytes are copied without validation. Generated decode arms select the appropriate runtime helper per field; see [§0](#0-project-architecture) for implementation status.

---

### 4.3 Bytes fields

Bytes fields follow the same pattern as strings with `&[u8]` as the value type.

**Implicit presence:**

- `fn payload(&self) -> &[u8]` — returns `&[]` when not set
- `fn set_payload(&mut self, v: &[u8])` — copies data

**Explicit presence:**

Same trait pattern as strings — `payload()` is the required `Optional` accessor; `_raw` / `has_` are trait defaults:

| Method | Return type | Description |
|---|---|---|
| `payload()` | `Optional<&'s [u8], impl HasDefault<&'s [u8]>>` | **Required** trait method — `get()` and `is_set()` |
| `payload_raw()` | `&[u8]` | Trait **default** — `self.payload().get()` |
| `has_payload()` | `bool` | Trait **default** — `self.payload().is_set()` |
| `set_payload(&mut self, v: &[u8])` | `()` | Copies data; marks field as set |
| `clear_payload(&mut self)` | `()` | Marks field as unset |

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

The wire encoding is always VARINT. Open vs closed is reflected in the **generated enum newtype** (see [Generated enum type](#generated-enum-type) below); field accessors use the same [`Optional`](#hasdefault-and-optional) pattern as other singular fields — no `Result` wrapper and no `_raw` accessors.

**Spec reference:** [Enum Behavior](https://protobuf.dev/programming-guides/enum/) (and Editions [`features.enum_type`](https://protobuf.dev/editions/features/#enum_type)). For an unrecognized wire integer:

| Kind | Storage | Accessor `is_set` | Accessor value |
|---|---|---|---|
| Open | field | set | that integer |
| Closed | unknown field set | unset | enum default |

#### Open enum (`enum_type = OPEN`, edition 2024 default)

Unknown wire values are stored in the field. The generated type accepts any `i32` (`From<i32>`); converting a known named value back to `i32` is fallible (`TryFrom<Status> for i32`).

```rust
pub fn status(&self) -> Optional<Status, impl HasDefault<Status>> {
    self.status.bind(&self._common).optional()
}
pub fn status_mut(&mut self) -> &mut Status;
```

IMPLICIT presence: `is_set()` is `true` when the wire value is non-zero; `get()` returns `Status::UNSPECIFIED` when unset.

#### Closed enum (`enum_type = CLOSED`)

Unknown wire values are diverted to unknown fields on decode. The generated type accepts only known values (`TryFrom<i32>`); conversion to `i32` is infallible (`From<Priority> for i32`).

```rust
pub fn priority(&self) -> Optional<Priority, impl HasDefault<Priority>> {
    self.priority.bind(&self._common).optional()
}
pub fn priority_mut(&mut self) -> &mut Priority;
pub fn clear_priority(&mut self);
```

EXPLICIT presence: `is_set()` tracks the presence bit; `get()` returns `Priority::UNSPECIFIED` when unset.

#### Generated enum type

Both open and closed enums produce a **newtype-over-`i32`** — not a Rust `enum`. Openness is a **type-level** distinction:

- Markers [`Open`](puroro-rt/src/fields/wire/varint.rs) / [`Closed`](puroro-rt/src/fields/wire/varint.rs)
- Traits [`OpenEnum`](puroro-rt/src/fields/wire/varint.rs) (`From<i32>`) / [`ClosedEnum`](puroro-rt/src/fields/wire/varint.rs) (`TryFrom<i32>`)
- Field wrapper [`ProtoEnum<E, K>`](puroro-rt/src/fields/wire/varint.rs)

[`SingularField`](puroro-rt/src/fields/singular/field.rs) (alias `SingularField`) stores `ProtoEnum<E, K>` and reuses the same `optional()` / `value_mut()` paths as other singular scalars (getters still project to `E` / `&mut E`). Merge is a single decode-then-write path: closed enums signal unknowns via [`DecodeError::UnknownClosedEnum`](src/error.rs) from `decode_wire`, and `merge` catches that to append the raw varint to unknown fields (`TryFrom` is the single source of truth).

```rust
// Open — any wire value is valid storage
status: SingularField<ProtoEnum<Status, Open>, Implicit, FIELD>,

// Closed — only known wire values are stored
priority: SingularField<ProtoEnum<Priority, Closed>, Explicit<BIT>, FIELD>,

#[repr(transparent)]
pub struct Status(i32);

impl ProtoEnumStorage for Status {
    fn proto_zero() -> Self { … }
    fn to_wire(self) -> i32 { … }
}
impl OpenEnum for Status {}

#[repr(transparent)]
pub struct Priority(i32);

impl ProtoEnumStorage for Priority { … }
impl ClosedEnum for Priority {}
```

Alias names with the same integer all map to the same `Self(v)`; equality is by wire value.

---

### 4.7 Oneof fields

Each `oneof` group generates types in a submodule named after the parent message (lower-snake-case), because the owned storage holds allocator-less `unmanaged` values (unsafe to drop implicitly) that must not leak into the public API.

Shape, storage, and projected views share **one** generic enum; Storage / Ref / Mut are type aliases. Group bound views live in `puroro-rt` (`OneofView` / `OneofViewMut`), parametrised by `OneofGroup` (implemented on the crate-internal storage alias):

```rust
// In module `task` / `notification`:

// Canonical shape — variant payloads are type parameters.
pub enum Notification<Ea, Pn, Wh, Po, Ur> {
    EmailAddress(Ea), PhoneNumber(Pn), WebhookId(Wh), Postal(Po), Urgent(Ur),
}

// (1) Owned storage — crate-internal alias; per-variant private field aliases
//     are the single source of truth. Implements OneofGroup, OneofDeallocate,
//     encode glue. Never public.
type EmailAddressField = SingularField<ProtoString, Oneof, FIELD_EMAIL>;
// … PhoneNumberField, WebhookIdField, PostalField<A>, UrgentField
pub(crate) type NotificationStorage<A> = Notification<
    EmailAddressField, PhoneNumberField, WebhookIdField, PostalField<A>, UrgentField,
>;

// (2) Payload-less case discriminant (unset is `None`, so no `NotSet` member).
pub enum NotificationCase { EmailAddress, PhoneNumber, WebhookId, Postal, Urgent }

// (3) Safe projected aliases — payloads from ProtoType on each variant's type
//     marker (no StringGuard / BitRef hard-coding). Pattern-match with
//     `Notification::…`.
pub type NotificationRef<'a, A> = Notification<
    <ProtoString<A> as ProtoType>::Ref<'a>,
    /* … */,
>;
pub type NotificationMut<'a, A> = Notification<
    <ProtoString<A> as ProtoType>::Mut<'a>,
    /* … */,
>;

// Accessors on Task — RPIT so `NotificationStorage` stays out of the signature:
pub fn notification_case(&self) -> Option<NotificationCase>; // = notification().case()
pub fn notification(&self) -> OneofView<'_, impl OneofGroup<
    Case = NotificationCase,
    Ref<'_> = NotificationRef<'_, A>,
    Mut<'_> = NotificationMut<'_, A>,
>>;
// Mut view omits `Ref` from the RPIT bounds so `as_view` reborrows stay short.
pub fn notification_mut(&mut self) -> OneofViewMut<'_, impl OneofGroup<
    Case = NotificationCase,
    Mut<'_> = NotificationMut<'_, A>,
>>;

// On OneofView:
//   case() -> Option<G::Case>
//   as_ref() -> Option<G::Ref<'_>>
// On OneofViewMut:
//   as_view(&self) -> OneofView<'_, G>
//   as_mut(self) -> Option<G::Mut<'_>>  // active variant; no switch
//   clear(self)                         // free active variant

// Per-variant `_mut` accessors switch the oneof to that variant (freeing any
// previously-active one) and return a handle appropriate to the kind:
pub fn email_address_mut(&mut self) -> impl DerefMut<Target = ::unmanaged::String<A>> + '_;
pub fn phone_number_mut(&mut self) -> impl DerefMut<Target = ::unmanaged::String<A>> + '_;
pub fn webhook_id_mut(&mut self) -> impl DerefMut<Target = i32> + '_; // VARINT variant
pub fn postal_mut(&mut self) -> &mut Address<A>;       // message variant

// Clears whichever variant is active (freeing it):
pub fn clear_notification(&mut self); // = notification_mut().clear()
```

Note: enums cannot carry unused lifetime/allocator parameters via `PhantomData` (unlike a struct). Integer-only oneofs therefore omit `'a` / `A` from the shape and from `Ref`/`Mut` aliases when no variant payload needs them.

**Variants own field wrappers, not raw storage.** A oneof member of a given kind reuses the exact field wrapper an ordinary singular field of that kind uses (`SingularField` / aliases — including [`ProtoBool`](puroro-rt/src/fields/wire/varint.rs) for `bool` and [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs) for messages), so the storage / `value` / `value_mut` / `deallocate` machinery is shared rather than reimplemented. The wrapper's *presence* is inert for a oneof — presence is tracked by the enclosing `OneofSlot` — so `FieldPresence::Oneof` is used (omit rules never consulted; bool still reads/writes its value bit).

To keep generated code thin, each wrapper is driven with the **field's own** construction, merge, and access primitives — no bespoke helpers on the oneof enum. Empty construction lives on [`EnumVariant::new_value`](puroro-rt/src/fields/enum_variant.rs); mut paths use a single bind:

| kind | `EnumVariant::new_value` | merge one occurrence | mut accessor |
|---|---|---|---|
| LEN | `SingularField::new_in(alloc)` | `slot.bind_mut(common).variant_mut::<V>().bind_mut(common).merge(…)` | `…variant_mut::<V>().bind_mut(common).value_mut()` |
| VARINT | `SingularField::new_in(alloc)` | same | same |
| bool | `SingularField::<ProtoBool<BIT>>::new_in(alloc)` | same | same → `impl DerefMut<Target = bool>` |
| message | `SingularField::with_message_in(alloc)` (`ProtoMessage`) | same | same → `&mut M` |

Every variant merges through the **same** `slot.bind_mut(common).variant_mut::<V>().bind_mut(common).merge(wire, buf)` shape. The message variant merges *into* the present child rather than replacing it.

The VARINT variant owns no heap: its `DeallocateIn` is a no-op.

**The message variant uses `Oneof` storage — always-present box, not `Option`.** An ordinary message field (`SingularField<ProtoMessage<…>, NonOneof, …>`) stores `Option<UnmanagedBox<M, A>>` via `FieldPresence::NonOneof`. A *oneof* message variant uses `SingularField<ProtoMessage<…>, Oneof, …>`, whose storage is always-present `UnmanagedBox<M, A>` under `ManuallyDrop`, so accessors are plain `value(common)` / `value_mut(common)`; `ManuallyDrop` enables uniform [`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs) from `&mut self`.

**Why these types, and why Storage is not public under the `Notification` name alone.** The storage alias's payloads are `unmanaged`-backed field wrappers, which panic on implicit drop and need the message allocator to free. Exposing that alias publicly would let a caller own one and hit that footgun, and would leak `unmanaged` into the API. So `NotificationStorage` is `pub(crate)`. The public surface is the shape `Notification`, `NotificationCase`, projected aliases `NotificationRef` / `NotificationMut`, and rt bound views `OneofView` / `OneofViewMut` (via RPIT `impl OneofGroup<…>` so Storage never appears in signatures). Shared getters live on `OneofView`; while holding a mut view, call `as_view()` (not a trait and not `Deref` — a by-value reborrowed view cannot be returned from `Deref::deref`).

Group accessors follow the same bound-view idiom as other fields: `slot.bind(&common)` / `slot.bind_mut(&mut common)` yield [`OneofSlotRef`](puroro-rt/src/fields/oneof.rs) / [`OneofSlotMut`](puroro-rt/src/fields/oneof.rs). [`OneofView`](puroro-rt/src/fields/oneof.rs) / [`OneofViewMut`](puroro-rt/src/fields/oneof.rs) wrap that pair, keyed by [`OneofGroup`](puroro-rt/src/fields/oneof.rs) on the storage alias. `OneofSlotMut::variant_mut::<V>()` selects (or installs) variant `V` via [`EnumVariant::new_value`](puroro-rt/src/fields/enum_variant.rs) and returns `&mut` the variant's field wrapper (`EnumVariant::Value`). That consumes the slot view so `common` can be re-borrowed; callers then `field.bind_mut(common)` for `.value_mut()` / `.merge(…)`. `set(value)` replaces the whole group, and `clear()` frees the active variant. Each of these releases the previously-active variant via `OneofDeallocate::deallocate` before overwriting the slot. The old `set_*` per-variant setters are removed.

Parent `_mut` accessors and decode arms are then —
`slot.bind_mut(common).variant_mut::<EmailAddress>().bind_mut(common).value_mut()`,
`slot.bind_mut(common).variant_mut::<WebhookId>().bind_mut(common).merge(wire, buf)`, etc.

The group's **encode** glue lives on the storage enum. Variant field-number constants sit at **module scope** so they remain usable as `match` patterns.

#### Default values on oneof members

Protobuf distinguishes **custom defaults** (`[default = X]` in proto2 / editions) from **type defaults** (0, `""`, `false`, first enum value, …). Oneof semantics differ from ordinary singular fields: presence is the oneof *case*, not a per-field has-bit, and getter vs mutator treat defaults differently.

The normative reference below is the official C++ generated API ([C++ Generated Code Guide — oneof fields](https://protobuf.dev/reference/cpp/cpp-generated/)), cross-checked against `protoc`'s C++ / C# field generators and `descriptor.cc` in [protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf). (proto3 forbids custom defaults entirely, so it is omitted here; the interesting cases are **proto2** and **editions**.)

**What the schema allows (proto2 / editions)**

| | proto2 / editions |
|---|---|
| `[default = X]` on a scalar / enum oneof member | allowed (descriptor does not reject oneof members; see e.g. `unittest_lite.proto`) |
| `[default = X]` on a message oneof member | not allowed (`Messages can't have default values`) |
| Default *variant* when the oneof group is unset | no — `case()` / `WhichOneof()` is `NOT_SET` |

There is no way to declare “when the oneof is unset, behave as if variant `foo` were selected.”

**Type-default exception on the wire.** For ordinary implicit-presence scalars, setting a field to its type default means “unset” and the value is omitted on the wire. For a oneof member the rule is inverted: *if that variant is selected and holds the type default* (e.g. `int32` 0), the oneof **case is set** and the value **is serialized**.

**Official accessor split (do not conflate getter and mutator)**

| API | Sets oneof case? | Value when that variant was not active |
|---|---|---|
| Const getter (`foo()`) | **No** | Returns the field's **proto default** (`[default = X]` if declared, else the type default). `has_foo()` stays false. |
| Mutator (`mutable_foo()` / `set_foo`) | **Yes** (clears any other variant first) | `mutable_*` installs a **fresh empty / zero / empty-message** payload — documented as empty, *not* the custom default. `set_*` writes the caller-supplied value. |

So a custom default on a oneof member is **not unused**, but it is used only on the **read path when the case is not that member**:

| Uses `[default = X]` | Does **not** use `[default = X]` |
|---|---|
| Const getter / hazzer-false path (`return $kDefault$` / `$kDefaultStr$`) | `mutable_*` init (`InitDefault()` → empty string sentinel; new empty submessage) |
| After `clear_foo()` / `clear_oneof()`, subsequent getters | Wire encode of an unset oneof (nothing is emitted) |
| Reflection `Get*` when the field is unset | Automatically selecting a default variant |
| Descriptor metadata / some language codecs (e.g. C# `has ? stored : default`) | |

Concrete C++ codegen shapes (proto2 / editions):

- Numeric oneof getter: `if (has) return field; return $kDefault$;` — `$kDefault$` is `DefaultValue(...)`, i.e. the custom default when present.
- String oneof getter: `if (!has) return $kDefaultStr$;` — same.
- String oneof `mutable_*`: `clear_oneof(); set_has; field.InitDefault();` then return a mutable buffer. `InitDefault()` points at the empty-string sentinel, not the custom default. The guide states explicitly: *“If the oneof case was not `kFoo` prior to the call, then the returned string will be empty (not the default value).”*

**puroro vs that contract.** The hand-written `Task` sample matches both halves:

- Per-variant **`_mut`** (`email_address_mut`, …) goes through `slot.bind_mut(common).variant_mut::<V>().bind_mut(common)`, which force-switches the case and builds a fresh wrapper via [`EnumVariant::new_value`](puroro-rt/src/fields/enum_variant.rs) (**type** default). That matches official `mutable_*`.
- Per-variant **getters** use `slot.bind(&common).variant_of::<V>().optional()`. The field wrapper's `D` type parameter is the proto default marker (`ProtoDefault`, or a message-local ZST such as `WebhookIdDefault` for `[default = -1]`). When the case is unset or another variant, `optional` returns `Optional::new(None)` so `is_set()` is false and `get()` yields `D::DEFAULT` — without selecting the variant. That matches official const getters.
- When the variant **is** active, getters return the **stored** value (including type zero / empty string), same as official.

**Codegen rule.** `[default = X]` on a oneof scalar becomes the field wrapper's `D` (`Singular*Field<…, D>`), which flows into the read accessor (`Optional<…, D>`). It must **not** change `_mut` installation: mutators keep installing type-default storage; custom defaults must not be written into the slot merely because the caller asked for a mutable handle.

---

### 4.8 Required fields (`LEGACY_REQUIRED`)

`features.field_presence = LEGACY_REQUIRED` corresponds to proto2's `required` modifier. It is intended as a **migration path** for existing proto2 schemas and is not recommended for new fields.

On the wire, a `LEGACY_REQUIRED` field is indistinguishable from an `EXPLICIT` field; the constraint is schema-level only.

**Accessor API** — same `Optional` pattern as `EXPLICIT`, plus [`Message::validate`](#message). On generated traits, `_raw` and `has_` are default methods (see [§4.0](#40-generated-per-message-traits)). Additionally:

- **`Message::validate() -> Result<(), DecodeError>`** — returns `MissingRequiredField` when a `LEGACY_REQUIRED` field is unset. Messages with no such fields still implement `Message` and return `Ok(())`.

`Message::decode` does **not** call `validate()` automatically.

When a `LEGACY_REQUIRED` field was absent from the wire, `owner_id().is_set()` is `false` and `Message::validate()` returns `Err(MissingRequiredField { … })`.

---

### 4.9 Unknown fields

All generated messages expose unrecognised fields (and closed-enum unknowns diverted into the unknown set) via [`Message::unknown_fields`](#message):

```rust
fn unknown_fields(&self) -> impl Iterator<Item = UnknownField<'_>> + '_;
```

[`UnknownField`](src/unknown.rs) carries the field number and an [`UnknownPayload`](src/unknown.rs) (`Varint` / `Fixed64` / `Bytes` / `Fixed32`). `wire_type()` is derived from the payload. The same field number may appear more than once.

This shape is the **common public contract**: it does not require contiguous wire storage, so future policies (discard, alternate layouts, custom handlers) can keep the same accessor. A common-trait `as_bytes()` is intentionally not provided.

**Default implementation (today):** `MessageCommon` still stores a contiguous partial protobuf stream (`UnmanagedVec<u8>`), filled by `skip_field_and_save` / closed-enum diversion. Encode re-emits that blob verbatim at the end of the message. The public iterator parses the blob via `puroro_rt::decode::iter_unknown_fields`.

**Future policies (not yet implemented):**

| Policy | Behaviour |
|---|---|
| **Preserve** (default) | Keep unknowns for round-trip; expose via the iterator |
| **Discard** | Opt-in (e.g. `#[puroro(no_unknown_fields)]`); omit the buffer; iterator is empty. Spec prefers preserve; discard is a deliberate size/privacy trade-off |
| **Custom** | Later hook / type-parameterised storage owned by the message |

---

## 5. Allocator design

### 5.1 Chosen design: single type parameter

Every generated type carries a single allocator type parameter `A` that applies to all heap allocations within that message and its nested messages:

```rust
pub struct Task<A: Allocator + Clone = Global> { /* … */ }
```

`A` defaults to `Global`, so `Task` (without a type argument) works identically to a version without allocator support. The `Clone` bound lets each message clone its allocator into nested children and free every field from a single `Drop` (see below).

**Single canonical allocator (no per-field copies).** The allocator is stored *once*, in `MessageCommon.alloc`. Heap-backed fields do **not** embed an allocator *instance*: they use the private [`unmanaged`](unmanaged/) types — `UnmanagedBox<T, A>`, `UnmanagedVec<T, A>`, `UnmanagedString<A>` — which keep only `ptr`/`len`/`cap` (plus `PhantomData<A>`) inline and receive an owned allocator (an `alloc.clone()`) on each operation that (de)allocates. The allocator **type** `A` still appears on those buffers and on protobuf type markers (`ProtoString<A>`, `ProtoInt32<A>`, …) so `DefaultIn` / `DeallocateIn` can use an associated `Alloc = A`. For a non-ZST allocator (e.g. `&bumpalo::Bump`) this removes the redundant *value* copy that a naive `Box<T, A>` / `Vec<T, A>` layout would embed in every field, so `size_of::<Task<A>>` grows by exactly one `A` regardless of field count.

**Owned allocator, never a borrow.** Operations pass the allocator **by value** rather than `&A`: the caller clones the canonical `MessageCommon.alloc` for each field operation. This keeps the allocator type consistently `A` for both a buffer's growth and its eventual free — mixing `&A` at allocation with `A` at deallocation is fragile and not obviously idempotent. Correctness relies on the `Allocator + Clone` contract that clones are interchangeable. (The one exception is building an *empty* `unmanaged` container, which never allocates, so it may borrow.)

**Manual release via `Drop`.** Because `unmanaged` values cannot free themselves (they panic if dropped implicitly), each field wraps its payload in `ManuallyDrop` and exposes `deallocate(&mut self, alloc: A)`. Every generated message implements `Drop`, walking its fields and calling `deallocate` with an `alloc.clone()` of the single `MessageCommon.alloc`; nested messages are freed recursively by their own `Drop`. The `unsafe` boundary is confined to the `puroro-rt` runtime and the generated `Drop`.

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

**Derived traits.** Generated messages implement `Default` (for `A: Allocator + Clone + Default`) plus a custom `Drop` (for `A: Allocator + Clone`). Additional `Global`-only convenience (`Task::new()`) applies when `A = Global`. `Clone` / `PartialEq` / `Debug` cannot be `#[derive]`d because the allocator-less fields need an owned allocator to copy or format; a `clone_in(&self, alloc)`-style API is future work. Wire bytes are not deterministic across encodes; compare semantically via getters. Full matrix: [IMPLEMENTATION.md §13](IMPLEMENTATION.md#13-derived-traits).

**Mutation API (`_mut`).** Mutation is unified under `_mut` accessors that return a guard implementing `impl DerefMut<Target = …>` (RPIT): `title_mut()` yields `impl DerefMut<Target = ::unmanaged::String<A>>`, `payload_mut()`/`tag_ids_mut()` yield `impl DerefMut<Target = Vec<_, A>>`, and scalar/enum `_mut` accessors also return `impl DerefMut<Target = T>` (today that is `&mut T`). The guard **owns** a clone of the message allocator when the payload is heap-backed. Acquiring an explicit-presence `_mut` sets the presence bit. The old `set_*` / `push_*` setters are removed; the one exception is repeated `string`/`bytes`, which keep a typed `push_*` helper because their element storage is allocator-less and impractical to construct through a bare `DerefMut`.

**Bound-view accessors (`bind` / `bind_mut`).** Every field family — [`SingularField`](puroro-rt/src/fields/singular/field.rs), repeated wrappers, and [`OneofSlot`](puroro-rt/src/fields/oneof.rs) — uses the same inherent call shape (`field.bind(&self._common)` / `field.bind_mut(&mut self._common)`). Each returns a short-lived view (`SingularFieldRef` / `SingularFieldMut`, `Repeated*FieldRef` / `Repeated*FieldMut`, `OneofSlotRef` / `OneofSlotMut`) that carries `(field, common)` together. The actual operation (`optional` / `value` / `as_slice` / `get` / `value_mut` / `merge` / `clear` / …) is a consuming method on that view. This keeps the field struct a pure storage holder and collapses each generated accessor to a single call — e.g. `self.owner_id.bind(&self._common).optional()` or `self.priority.bind_mut(&mut self._common).clear()`. Binding happens even when a particular accessor does not consult `common` (e.g. `IMPLICIT` `value()`, repeated `as_slice()`), so read and write share one shape. The presence bit index for EXPLICIT / LEGACY_REQUIRED fields is a **const generic on the field type** (`Explicit<BIT>`), not a runtime argument to `bind` / `bind_mut`. Encode / `deallocate` / `validate_required` stay as plain field methods that take `&common` directly (they are not generated getters). Getter / `_mut` payload types (`Ref` / `Mut`) live on [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs), not on a separate singular-access trait.

Varint and LEN singular fields share one runtime type, [`SingularField`](puroro-rt/src/fields/singular/field.rs), parametrised by [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) type markers such as `ProtoInt32<A>` / `ProtoString<A>` / `ProtoBool<A, VALUE_BIT>` (see [IMPLEMENTATION.md §7](IMPLEMENTATION.md#7-field-wrappers) / [§14](IMPLEMENTATION.md#14-singular-fields)). The field stores `T::Slot` (`Self` for all markers — ZST for bool, with the logical value in `_common.presence`). **Interim:** `VALUE_BIT` remains on `ProtoBool` for singular/oneof codegen stability; a future cleanup should move it to the field / layout side. **`repeated bool` must not reuse this bit-packed marker** — store plain `bool` elements with no MessageCommon bit index (see [IMPLEMENTATION.md § Bit-packed bool](IMPLEMENTATION.md#bit-packed-bool-protobool)). Nested messages use the same wrapper with `T = ProtoMessage<M, A>` and `P = NonOneof` / `Oneof`.

**Decode API:**

```rust
impl<A: Allocator + Clone + Default> Message for Task<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;
}
```

The `Default` bound is required because `Message::decode` calls `A::default()` to obtain the initial allocator.  Callers using a non-`Default` allocator must call `new_in(alloc)` followed by `merge_from` instead of `decode`.

**Arena allocator example:**

```rust
let bump = bumpalo::Bump::new();
let mut t = Task::new_in(&bump);
t.title_mut().push_str("Fix bug");
// All allocations (title, labels, nested messages) land in `bump`.
// Here `A = &Bump`, which is `Copy`, so each op just re-uses the borrow; for an
// owned arena handle the message would clone it per op instead.
// When `t` drops, its `Drop` calls `deallocate((&bump).clone())` on each field.
// For an arena that is effectively a no-op; the memory is reclaimed in bulk when
// `bump` itself is dropped. (For `Global`, `deallocate` returns the blocks.)
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

The wire format is **identical** across proto2, proto3, and editions. The runtime library (`Message`, all helpers) requires no changes to support different syntax versions. All differences are in what the **code generator emits**.

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
| `enum_type` | `OPEN` / `CLOSED` | Generated newtype implements `OpenEnum` vs `ClosedEnum`; field is `ProtoEnum<E, Open>` vs `ProtoEnum<E, Closed>` (unknown closed values → unknown fields) |
| `message_encoding` | `LENGTH_PREFIXED` / `DELIMITED` | `DELIMITED` (groups) is deprecated; not generated |
| `utf8_validation` | `VERIFY` / `NONE` | `VERIFY`: `decode_string_in` returns `DecodeError::InvalidUtf8` on bad UTF-8 (default). `NONE`: copy bytes without validation (generated code uses an unchecked conversion). Runtime support for per-field dispatch is **pending** — see [§0](#0-project-architecture). |

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

### No builder / immutable-object pattern

The Java protobuf implementation separates a mutable `Builder` type from an immutable `Message` type, with `build()` as the transition point.  This pattern exists because Java has no ownership semantics: any holder of a reference could mutate a shared object, so immutability must be enforced at the type level.

Rust's ownership system already provides the same guarantees without a dedicated builder type:

- A caller that binds with `let mut` can mutate the message; one that binds with `let` cannot.
- Sharing across threads uses `Arc<Task>` (immutable) or `Arc<Mutex<Task>>` (shared mutation).
- `LEGACY_REQUIRED` field validation is covered by `Message::validate()` without needing a `build()` step.

Generating both `Task<A>` and `TaskBuilder<A>` would double the generated code, complicate the allocator design, and add a `build()` conversion step — all for a guarantee Rust already provides for free through its borrow checker.

### Accessor methods instead of public fields

Public struct fields are simpler but prevent changing internal representations without a breaking API change. Accessor methods decouple the interface from the implementation — for example, presence tracking uses a per-message bitfield internally while exposing `has_X()` / `Optional` accessors unchanged.

### `Optional<T, impl HasDefault<T>>` as a concrete struct

Explicit-presence field accessors return `Optional<T, D>` — a concrete struct rather than a trait or `Option<T>` directly.  Key benefits of this design:

- **Default value as compile-time constant.** `D::DEFAULT` is a `const` expression, so the "return default when not set" branch has zero runtime overhead.
- **No RPIT drop-check restriction.** Because the concrete struct's `Drop` is trivially visible to the borrow checker, `task.title().get()` chains directly without a `let` binding — even for string fields that return `&str`.
- **No `Option<T>` conversion.** `Optional` provides only `get()` and `is_set()` — no `get_opt()` and no `From`/`Into` for `Option<T>`.  Proto explicit-presence fields always carry a meaningful value (explicit or default); converting to `Option<T>` would conflate "not set" with "no value" and make the declared default impossible to enforce.
- **Private default provider.** The `D` type is a zero-sized struct defined locally inside the method body, then hidden behind `impl HasDefault<T>` in the return type.  Callers never need to name it.
- **String defaults without unstable features.** `&'static str` cannot currently be a `const` generic parameter, but `const DEFAULT: &'a str` in a blanket `impl<'a> HasDefault<&'a str>` is fully stable on nightly.

### Slice-like return type for repeated fields

Repeated field accessors return a reference to a contiguous sequence rather than a freshly allocated `Vec`. This allows O(1) random access without triggering allocation. The concrete element type is intentionally not part of the stable API.

### Native methods alongside trait methods

Each concrete struct provides its own `impl` block with native methods in addition to implementing the generated traits.  The traits define the minimum interoperability contract; native methods expose whatever that struct can do most efficiently.  Code that knows the concrete type uses native methods; generic code uses the trait.

For explicit-presence fields, **both traits treat the `Optional` accessor as the single required method**; `_raw` and `has_` are default trait methods that wrap it.  Native methods on the struct may implement all three names directly (reading the internal presence bit and value) without calling through the trait defaults.

This is the standard Rust pattern: `Vec<T>` implements `Iterator` but also has `push`, `sort`, and hundreds of other methods that the `Iterator` trait does not mandate.

### `Default` bound on `Message::decode`

The provided `decode` method requires `Self: Default`. The lower-level `merge_from` has no such requirement, which is important for callers using non-`Default` allocators.

### No compatibility with other Rust protobuf libraries

puroro is a greenfield design. Generated types, trait names, and module layout are not interchangeable with `prost`, `protobuf`, or any other existing crate. Wire-format interoperability with other implementations is achieved only through the shared protobuf encoding spec, not through API compatibility.

### Deprecated group wire types

Groups (`SGroup` / `EGroup`) are not generated and are not stored in unknown fields. If a decoder encounters a group tag on the wire, it may return `DecodeError::InvalidTag`, skip the field, or panic — preserving group payloads for round-trip is explicitly out of scope.

### `protoc` plugin as the primary codegen path

The code generator is designed first as a `protoc` plugin. That is the expected way users invoke generation (`protoc --puroro_out=…`). Wrapper scripts, `build.rs` integration, or a standalone binary may exist alongside the plugin, but the plugin interface is the reference integration point.

### `protobuf-core`, `puroro`, and `puroro-rt`

`protobuf-core` holds reusable wire-format primitives (varint, tags, field readers/writers). **`puroro`** holds the stable message-level API (`Message`, `Optional`, errors) that library users and generated `impl` blocks share. **`puroro-rt`** holds the composable field catalog, wire helpers, and other generator-facing runtime pieces. Generated crates list both `puroro` and `puroro-rt` as dependencies; application code should depend on the generated crate and `puroro` only — not on `puroro-rt` or `protobuf-core` directly.

### `Bytes` for lazy wire storage vs `A: Allocator` for decoded values

Eager `Task<A>` stores decoded field data in allocator-less `unmanaged` containers (`UnmanagedString`, `UnmanagedVec<i32>`, …) that share the single `MessageCommon.alloc`.  `TaskLazy<A>` stores the opaque wire stream in **`bytes::Bytes`** so nested messages can share the parent's allocation through **`Bytes::slice`** without copying LEN payloads.  Allocator customisation applies to **owned decoded data**; wire blobs prioritise cheap sharing and sub-slicing over per-field allocator control.

---

## 8. Future work

### Specialized message implementations

In addition to the primary `Task<A>` struct, the following specialized implementations are planned.  Each implements `TaskMessageFallible` (and possibly `TaskMessage`) and is interchangeable with `Task<A>` in generic code that depends only on the trait.

#### `TaskLazy<A>` — lazy parse timing

`TaskLazy<A>` implements the same accessor API as `Task<A>` via `TaskMessageFallible` (`Error = DecodeError`).  All decoding — wire scanning **and** semantic interpretation — is deferred until a getter runs.

##### Wire buffer: `bytes::Bytes` (shared, sliceable)

The internal wire buffer is **`bytes::Bytes`**, not `Vec<u8, A>`.  `Bytes` is a reference-counted handle to a shared byte allocation (internally `Arc` — the same pattern as `Rc`, but `Send + Sync`).  It is already a dependency of `puroro-rt` and is the standard choice in the Rust protobuf/network ecosystem (`prost`, `tonic`, `hyper`).

| Property | Benefit for `TaskLazy` |
|---|---|
| Cheap `Clone` | Parent and child messages share one allocation — refcount increment only |
| `.slice(range)` / `.slice_ref(sub)` | **Zero-copy** sub-range for nested messages — O(1), no payload memcpy |
| Implements `bytes::Buf` | Wire scanners and existing encode/decode helpers work directly |

**Why not `Vec<u8, A>` for `_wire`?**  Custom allocator support for the *wire blob* is less important than cheap sub-slicing.  Decoded values (`UnmanagedString`, `UnmanagedVec<i32>`, …) still allocate through `A: Allocator`.  Only the opaque wire storage uses `Bytes` (global/shared allocator).

**Alternatives** if `Send` is not required: [`slice-rc`](https://docs.rs/slice-rc) (`Src<T>` — literal `Rc` with `.slice()`).  [`arc-slice`](https://docs.rs/arc-slice) (`ArcSlice` — similar to `Bytes` with custom metadata).  **`bytes::Bytes` is the default choice** unless a future requirement forces otherwise.

##### `merge_from`: store the input buffer only

`merge_from` performs **no parsing of any kind** — no tag reading, no field routing, no payload skipping, no UTF-8 validation. It only **appends the remaining input bytes** to `_wire: Bytes` and **invalidates all field caches**.

| Concern | Behaviour |
|---|---|
| Internal storage | `_wire: Bytes` — complete wire stream accumulated across all `merge_from` calls |
| Append cost | Concatenating on merge may reallocate — **O(n) per merge, not per field access** |
| Proto merge semantics | Bytes are appended; last-wins / repeated-append applied when getters scan `_wire` |
| Unknown fields | Stay inside `_wire`; not extracted at merge time |
| Nested messages | Stay inside `_wire` until a getter slices them into a child |

##### Internal storage (`TaskLazy<A>`)

| Component | Type / role |
|---|---|
| `_wire` | `bytes::Bytes` — shared wire blob |
| Per-field cache | `RefCell<FieldCache<T>>` or similar — see state machine below |
| `_alloc` | `A` — for decoded values only, not for `_wire` |
| Nested child | `AddressLazy { _wire: Bytes }` — subslice of parent via `.slice()`, zero-copy |

**Interior mutability** is required because trait accessors take `&self` but populate caches on first decode.

##### Field cache state machine

Each field maintains a **cache slot** (independent of `_wire`).  Slots start **uninitialized** after every `merge_from` that appends data:

```
Uninitialized   — not yet scanned in _wire (or cache invalidated)
Absent          — wire-scanned; field not on wire
WireFound       — optional; payload located as Bytes subslice, not semantically decoded
Parsed          — semantically decoded and cached
Failed          — decode error; subsequent getters return Err without re-parsing
```

Typical transitions on getter access:

1. **`Uninitialized` → scan `_wire`** (wire-level: read tags, match field number, skip or record payload bounds).
2. **`has_*()`** stops after wire scan — sets `Absent` or `WireFound` / equivalent; **no semantic decode**.
3. **`Optional` getter** continues to **semantic decode** → `Parsed` (or `Failed` on error).
4. Subsequent calls hit `Parsed` / `Absent` / `Failed` directly.

**Caching is mandatory** once a field has been successfully decoded (`Parsed`).

##### Parse cursor (in `_wire`)

Each field that may be read incrementally (especially **repeated** fields) stores a **`cursor: usize`** — a byte offset into `_wire` marking how far a **field-specific wire scan** has progressed:

| Use case | Cursor role |
|---|---|
| **Singular fields** | First access scans `_wire` for the **last** occurrence of the field number; cursor not retained after `Parsed` |
| **Repeated iterator** | Cursor resumes the wire scan for the next occurrence of the field number; each `next()` decodes one element and advances cursor |
| **Packed repeated** | After locating a LEN record, a sub-cursor may track progress inside the packed payload |
| **Negative cache** | Once a full scan finds no occurrences, slot becomes `Absent` — no rescan until next `merge_from` invalidates |

The cursor lives in the field's cache slot (or a companion scan state), always relative to **`_wire`**.

##### Getter timing and `Result<Optional<…>>` semantics

Explicit-presence fields expose `Result<Optional<T, impl HasDefault<T>>, DecodeError>` on the fallible trait.  **`Optional` is only constructed after a successful semantic decode** — never before.

| Accessor | Wire scan | Semantic decode | Error timing |
|---|---|---|---|
| `has_*()` | **Yes** — scan `_wire` for field number; skip payloads | **No** | Wire errors (truncated tag, bad varint) → `Err`. Invalid UTF-8 in an unread payload → **not** detected |
| `*_()` → `Result<Optional<…>>` | **Yes** — locate payload (or use cache) | **Yes** — full field decode | Semantic failure (e.g. `InvalidUtf8`) → **`Err` before `Optional` is built** |
| `*_raw()` trait default | Via `Optional` getter | Via `Optional` getter | Same |

Because `has_*()` performs wire scanning but not semantic validation, **`has_title() == true` does not guarantee `title()?` succeeds** — malformed UTF-8 surfaces only when the `Optional` getter runs.

For **implicit-presence** scalars (`score`), the getter wire-scans for the last occurrence (or returns zero if absent), then semantically decodes.

##### Nested messages — zero-copy via `Bytes::slice`

On first access to `assignee()`, the parent wire-scans `_wire` for field 11, then sets the child's `_wire` to `parent._wire.slice(start..start + len)` — **O(1) refcount, no payload copy**. The child shares the parent's allocation until all `Bytes` handles are dropped. The child stores the subslice only; semantic decode happens when the child's getters run.

Repeated nested messages each get their own `Bytes` subslice from the same parent allocation.

> **Contrast with `TaskView<'buf>`.**  `TaskView` uses borrowed `&'buf [u8]` subslices (lifetimes).  `TaskLazy` uses owned `Bytes` subslices (reference counting) so nested types need no lifetime parameter and can live in struct fields indefinitely.

##### Singular vs repeated decode granularity

| Field kind | On getter access |
|---|---|
| Singular scalar / string / bytes / enum | Wire-scan `_wire` for last occurrence → **full semantic decode** of that payload |
| Repeated | Wire-scan from **cursor** for next occurrence → decode **one element**; `tag_ids_all()` scans/decodes all remaining occurrences and may set `Parsed(Vec<…>)` |
| Nested message | Wire-locate LEN payload → **`Bytes::slice` into child `_wire`** (zero-copy) → lazy subtree |

There is no meaningful partial **semantic** decode within a singular string or scalar — once located on the wire, the full payload is decoded.

##### Trait implementation

- Implements `TaskMessageFallible` with `Error = DecodeError`.
- Explicit-presence fields implement the `Optional` getter; `_raw` / `has_` use trait default methods (see [§4.0](#40-generated-per-message-traits)).
- Does **not** implement `TaskMessage` (infallible) — all access goes through the fallible trait.

**Native methods beyond the trait:**

- **`into_eager(self) -> Result<Task<A>, DecodeError>`** — wire-scan and decode all fields; build eager `Task<A>`.
- **`tag_ids_all(&self) -> Result<&[i32], DecodeError>`** — decode all remaining repeated elements in one pass and cache `Parsed(Vec<…>)`.

#### `TaskView<'buf>` — zero-copy, buffer-referencing

- Holds `&'buf [u8]` to the original input; no heap allocation for field data.
- String / bytes fields return subslices into the input buffer.
- Read-only; tied to buffer lifetime.
- Fully lazy (scan on each access) or semi-eager (field-offset index built once).
- Implements `TaskMessageFallible` with `Error = DecodeError`.
- Repeated fields support early termination via fallible iterators.

**Native methods beyond the trait:** `title() -> Result<&'buf str, …>` (buffer lifetime, not `&self`), `to_owned(alloc) -> Task<A>`, `tag_ids_raw() -> Result<&'buf [u8], …>`.

#### Relationship between implementations

```
TaskMessage (infallible trait)
    ↑ impl
    Task<A>  ←── primary, all-in-one

TaskMessageFallible (Result-returning trait)
    ↑ impl
    Task<A>        (Error = Infallible)
    TaskLazy<A>    (Error = DecodeError)
    TaskView<'buf> (Error = DecodeError)
```

Generic code that only reads fields can be written once against `TaskMessageFallible` and used with `Task<A>`, `TaskLazy<A>`, or `TaskView<'buf>`.

---

### Other future work

- **UTF-8 validation (`NONE` path).** Expose an unchecked decode helper; wire per-field dispatch in generated code. (`VERIFY` path exists today.)
- **Recursion limit enforcement.** Thread depth through nested `merge_from`; return `DecodeError::RecursionLimitExceeded`. (Error variant exists; enforcement is a stub.)
- **Unknown-field preservation opt-out.** A per-message attribute to discard unknowns (`Discard` policy in §4.9); public accessor remains an empty iterator.
- **Map fields.** Syntactic sugar for a repeated message entry; requires an allocator-aware map type.
- **Service / RPC definitions.** Out of scope for the runtime library.
- **Well-known types.** `google.protobuf.Timestamp`, `Duration`, `Any`, etc.
- **Reflection / descriptors.** Runtime introspection of message schema.
- **Extensions.** See §6.5.
