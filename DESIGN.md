# puroro — Design Document

This document specifies the **public interface** of the `puroro` Protocol Buffers runtime library and the accessor API that the code generator must emit. It intentionally omits internal implementation details — storage types, encode/decode algorithms, and the composable field catalog — which live in the sibling **`puroro-rt`** crate and are documented in [IMPLEMENTATION.md](IMPLEMENTATION.md).

## Table of contents

0. [Project architecture](#0-project-architecture)
1. [Goals](#1-goals)
2. [Wire format overview](#2-wire-format-overview)
3. [Runtime trait API](#3-runtime-trait-api)
4. [Generated code specification](#4-generated-code-specification)
   - [Module layout and naming](#module-layout-and-naming)
   - [Path qualification](#path-qualification)
   - [Public signatures must not surface `puroro-rt`](#public-signatures-must-not-surface-puroro-rt)
   - 4.0 [Inherent accessors (current)](#40-inherent-accessors-current)
   - 4.1 [Scalar fields](#41-scalar-fields) — implicit vs explicit presence
   - 4.2 [String fields](#42-string-fields)
   - 4.3 [Bytes fields](#43-bytes-fields)
   - 4.4 [Repeated fields](#44-repeated-fields) — packed vs non-packed, dual-form decode
   - 4.5 [Nested message fields](#45-nested-message-fields)
   - 4.6 [Enum fields](#46-enum-fields) — open vs closed
   - 4.7 [Oneof fields](#47-oneof-fields)
   - 4.8 [Required fields (`LEGACY_REQUIRED`)](#48-required-fields-legacy_required)
   - 4.9 [Unknown fields](#49-unknown-fields)
   - 4.10 [Map fields](#410-map-fields)
5. [Allocator design](#5-allocator-design)
   - 5.1 [Chosen design: single type parameter](#51-chosen-design-single-type-parameter)
     - [Why `A: Clone` is a struct bound](#why-a-clone-is-a-struct-bound)
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
   - 8.0 [Per-message traits (not yet generated)](#per-message-traits-not-yet-generated)
   - 8.1 [Specialized message implementations](#specialized-message-implementations) (`TaskLazy<A>`, `TaskView<'buf>`) — includes [lazy parse timing](#tasklaya--lazy-parse-timing) and [eager / lazy semantic gaps](#eager-and-lazy-semantic-gaps)
   - 8.2 [Other future work](#other-future-work)

---

## 0. Project architecture

The puroro project comprises several crates and tools with distinct roles:

| Component | Role |
|---|---|
| **`protobuf-core`** (git submodule) | Wire-format **primitives** — `Varint`, `Tag`, `WireType`, field I/O traits, varint read/write. Used by `puroro` and `puroro-rt`; **generated code does not import it directly**. |
| **`unmanaged`** (git submodule) | Allocator-aware containers that panic on implicit `Drop` (`UnmanagedBox` / `UnmanagedVec` / `UnmanagedString`) plus `DeallocateIn` / `DefaultIn` / `CloneIn`. Generated `Drop` and catalog teardown call `deallocate` explicitly. |
| **`puroro`** | **Stable user-facing runtime API** — `Message`, `Optional`, `HasDefault`, `DecodeError`, `WireType`, `UnknownField` / `UnknownPayload`. Library users depend on this crate; generated message code imports it for traits, accessor return types, and error handling. |
| **`puroro-rt`** | **Generated-code runtime** — composable field catalog (`fields::*`, `MessageCommon`), wire encode/decode helpers (`encode` / `decode` modules), `ProtoDefault`, and allocator-aware string/bytes utilities. Generated crates depend on it **internally**; library users of generated messages must not need to name or import it (see [§4](#public-signatures-must-not-surface-puroro-rt)). Semver is looser than `puroro`. See [IMPLEMENTATION.md §4](IMPLEMENTATION.md#4-shared-infrastructure). |
| **Code generator** (`protoc-gen-puroro`) | `protoc` plugin that reads `.proto` input and emits Rust implementing the API in this document. **Primary execution path:** `--puroro_out=…`. Other invocation styles (standalone CLI, `build.rs` wrapper, etc.) are permitted but not required. External crates use leading-`::` paths (`::puroro::…`, `::puroro_rt::…`); names inside the generated module forest use `self::_root::…` ([Path qualification](#path-qualification)). **Implementation status** (pipeline vs field emission): [IMPLEMENTATION.md §3](IMPLEMENTATION.md#3-implementation-status). |

**Reference schema.** The `Task` and `Address` messages in [§4 Reference schema](#reference-schema) are the **canonical examples** for describing and reviewing generated code. All field-pattern subsections (§4.1–4.10) and [IMPLEMENTATION.md](IMPLEMENTATION.md) use this same schema unless noted otherwise.

**Non-goals.**

- **No compatibility** with other Rust protobuf libraries (`prost`, `protobuf`, `quick-protobuf`, etc.). API shapes, type names, and generated module layout are puroro-specific.
- **Extensions** (§6.5) are out of scope for the current design.

**Runtime completeness.** The public API in this document is normative and intended to remain stable; the **`puroro-rt` implementation is still evolving**. Items designed but not yet fully wired:

| Feature | Design intent | Implementation status |
|---|---|---|
| `utf8_validation` (`VERIFY` / `NONE`) | Generated decode paths honour the per-field Editions setting. `NONE` uses bytes views (`&[u8]`), not `&str`. `VERIFY` failure is `DecodeError::InvalidUtf8` and is not diverted into unknown fields. | **Done** — `ProtoString` (`VERIFY`, `&str`) vs `ProtoStringUnchecked` (`NONE`, `&[u8]`). proto2 default is `NONE`; overlay with `proto2_utf8=verify`. Eager merge fails immediately; lazy checks on the getter ([eager / lazy gaps](#eager-and-lazy-semantic-gaps)). |
| Recursion limit | Nested-message `merge_from` enforces a depth limit; excess depth → `DecodeError::RecursionLimitExceeded`. | **Done** — `puroro_rt::MessageMerge::merge_from_with_depth` + `RECURSION_LIMIT` (100); user `Message::merge_from` wraps `merge_message` at depth 0. Nested catalog paths pass `depth + 1`. |
| Deprecated groups (`SGroup` / `EGroup`) | Never generated; not preserved on decode. | Decoder may return `DecodeError::InvalidTag`, skip, or panic — round-trip fidelity for groups is **not** a goal. |

---

## 1. Goals

- **Protobuf spec compliance.** Support the canonical wire format (varints, I32, I64, LEN records, packed repeated, oneofs, unknown fields) for proto2, proto3, and editions. The compliant unknown-field behavior is preserve-and-round-trip. Opt-in `DISCARD` and `REJECT` leave that rule ([§4.9](#49-unknown-fields)). Deprecated group wire types (`SGroup` / `EGroup`) are never generated; the decoder does not preserve them (see [§0](#0-project-architecture)).
- **Allocator support.** Every generated message is generic over `A: Allocator + Clone` using the `allocator-api2` crate ([why `Clone` is on the struct](#why-a-clone-is-a-struct-bound)). Arena allocators (e.g. `bumpalo`) and custom pools are first-class citizens.
- **Performance-oriented interface.** Accessors return borrowed references (`&str`, `&[u8]`, `&[T]`), never freshly allocated containers. The `encode_to_vec` / `encode_to_bytes` convenience methods allocate; `Message::encode` writes into a caller-owned buffer. Wire-body sizing / writing (`encoded_len` / `encode_raw` with `EncodeCtx`) lives on `puroro_rt::MessageEncode`, not on `Message`.
- **Rust idioms.** Private fields accessed via generated accessor methods; `Optional<T, impl HasDefault<T>>` for explicit-presence scalar, string, and enum fields; singular nested messages return `Option<&M>` / `&mut M` (inlined children are a full `M` with their own `MessageCommon`); no `unsafe` in user-visible APIs.
- **`puroro` for users, `puroro-rt` for generators.** Library users of generated messages depend on the generated crate and **`puroro` only**. Generated code may use `puroro-rt` internally, but **must not surface `puroro-rt` types in public signatures** (see [§4](#public-signatures-must-not-surface-puroro-rt)).
- **Implementation flexibility.** The public interface described here must remain stable even if internal storage representations change. Eager messages use per-message common bits (see [IMPLEMENTATION.md §10](IMPLEMENTATION.md#10-common-bit-indices)); the accessor API is unchanged if storage layout evolves.
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
/// `D` is the field wrapper's default marker (`ProtoDefault` or a custom ZST).
pub trait HasDefault<T: Copy> {
    const DEFAULT: T;
}

/// Concrete return type for every explicit-presence field accessor.
/// `T` — Copy scalar or reference (`&'a str`, `&'a [u8]`).
/// `D` — zero-sized default provider; hidden behind `impl HasDefault<T>`.
pub struct Optional<T: Copy, D: HasDefault<T>> { … }

impl<T: Copy, D: HasDefault<T>> Optional<T, D> {
    pub fn new(value: Option<T>) -> Self;

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

```rust
pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> { … }
pub fn title<'s>(&'s self) -> Optional<&'s str, impl HasDefault<&'s str>> { … }

// Both chain directly without a let binding:
let n: i32  = task.max_retries().get();
let s: &str = task.title().get();
if task.max_retries().is_set() { … }
```

**Lazy implementations (`TaskLazy`).** On the eager path, `Optional::new` receives `Some(value)` or `None` derived from the field's presence bit and value slot. On the lazy alias ([§8](#tasklaya--lazy-parse-timing)), numericals are already materialised when the scan finishes. A string or bytes getter decodes its span on first read and returns `Err` before constructing `Optional` if that decode fails (for example `InvalidUtf8`). `has_*` reports wire presence and does not UTF-8-check the payload.

The concrete `D` type is the field wrapper's default marker (`ProtoDefault`, or a message-local ZST such as `MaxRetriesDefault` for `[default = 3]`). Callers never name it: accessors return `Optional<…, impl HasDefault<T>>`.

### `Message`

Implemented by **every** generated message (C++ `MessageLite`-like surface: codec + shared infrastructure). Field accessors stay as inherent methods so proto field names do not collide with these helpers.

The user-facing trait is slimmer than the generated catalog traits. Wire-body sizing / writing lives on [`puroro_rt::MessageEncode`](puroro-rt/src/message_encode.rs) (`encoded_len` / `encode_raw` with [`EncodeCtx`](puroro-rt/src/message_encode.rs)). Depth-aware merge lives on [`puroro_rt::MessageMerge`](puroro-rt/src/message_merge.rs) (`merge_from_with_depth` over [`DecodeBuf`](src/scoped_buf.rs) / [`ScopedBuf`](src/scoped_buf.rs)). Generated `Message` impls are thin wrappers: `encode` → `encode_message`, `encode_to_vec` → `encode_message_to_vec`, `merge_from` → `merge_message` (depth 0).

```rust
pub trait Message: Sized {
    type Alloc: Allocator; // Clone is required on new_in / merge_from, not on the associated type
    fn new_in(alloc: Self::Alloc) -> Self
    where
        Self::Alloc: Clone;

    // Codec (required on generated impls; encode_to_bytes / decode are provided)
    fn encode<B: bytes::BufMut>(&self, buf: &mut B);
    fn encode_to_vec(&self) -> Vec<u8>;
    fn merge_from<B: bytes::Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>
    where
        Self::Alloc: Clone;
    fn encode_to_bytes(&self) -> bytes::Bytes; // provided: Bytes::from(encode_to_vec())
    fn decode<B: bytes::Buf>(buf: B) -> Result<Self, DecodeError>
    where
        Self: Default,
        Self::Alloc: Clone;

    // Infrastructure (required)
    fn unknown_fields(&self) -> impl Iterator<Item = UnknownField<'_>> + '_;
    fn validate(&self) -> Result<(), DecodeError>; // Ok(()) when no LEGACY_REQUIRED
}
```

`encode` / `merge_from` are generic over `BufMut` / `Buf` so the compiler can monomorphise; the trait is **non-object-safe** by design.

**Merge semantics** (identical across proto2, proto3, editions): singular scalar — last wins; singular message — recursive merge; repeated — append; unknown fields — accumulated for round-trip.

**Non-deterministic field order.** The encoder may emit known fields in any order. Two encodes of the same logical message are not guaranteed to produce identical bytes; compare with `PartialEq`, not with encode output equality. See [IMPLEMENTATION.md §12](IMPLEMENTATION.md#12-message-level-wire-io).

**Name conflicts.** If a proto field is named `validate`, `unknown_fields`, `encode`, `encode_to_vec`, etc., the inherent field getter wins; use `Message::validate(&msg)` (UFCS). Lazy aliases also have inherent helpers such as `encoded_len() -> Result<usize, DecodeError>` and `has_*()`; those collide the same way and are not `Message` trait items.

**Allocator.** `Alloc` is the message's single allocator type. Nested message fields construct children with `Message::new_in` under the bound `M: Message<Alloc = A>` (same `A` as the parent). An inherent `new()` for `Global` may still be emitted on the concrete type. `decode` requires `Self: Default` (typically `A: Default`); otherwise use `new_in(alloc)` then `merge_from`.

---

## 4. Generated code specification

This section is the normative reference for what the code generator emits. All field patterns are illustrated using a single **editions** reference schema, since editions can express every variant (implicit/explicit presence, custom defaults, required-like semantics, open/closed enums, packed/expanded repeated) in one file.

This section is the **normative** eager-path shape. The lazy alias uses the same field names with `Result` getters and no mutators ([§8](#tasklaya--lazy-parse-timing)). Hand-written [`sample-generated/`](sample-generated/) is the reference implementation; the live `protoc-gen-puroro` plugin emits both aliases (status: [IMPLEMENTATION.md §3](IMPLEMENTATION.md#3-implementation-status)). Deliberate sample-vs-production differences — flat modules, short-name `use`s, no `@generated` header — are documented there and in [IMPLEMENTATION.md §9](IMPLEMENTATION.md#9-struct-layout). For each message type the generator must emit:

1. **The primary struct** — a full-featured owned implementation (§4.0–4.10), internally a product of **`puroro_rt::fields` catalog types** + shared `MessageCommon` (see [IMPLEMENTATION.md §2](IMPLEMENTATION.md#2-architecture-overview)). Field accessors are **inherent methods** on that struct.
2. **The lazy alias** — `TaskLazy<A> = TaskImpl<A, Lazy<A>>`, same field names, `Result<_, DecodeError>` getters, no `_mut` ([§8](#tasklaya--lazy-parse-timing)). [`sample-generated/`](sample-generated/) is the reference shape; the plugin emits the same pair of aliases.
3. **(Future) Per-message traits** — `foo::Message` / `foo::MessageFallible` (in that message's companion module) ([§8](#8-future-work)). The plugin does not emit those traits. The handwritten sample implements `task::MessageFallible` on eager `Task` only. A borrowing view is not part of this sample; the measured `Address` spike is [`archive/address-view-spike`](archive/address-view-spike/).

Generated Rust is not hand-edited. Navigational comments in generated output are **not required**; semantics and usage are documented on the library API (`puroro` / `puroro-rt` rustdoc) and in this document / [IMPLEMENTATION.md](IMPLEMENTATION.md). Production modules still carry an `@generated` file header ([IMPLEMENTATION.md §9](IMPLEMENTATION.md#generated-headers)). Reference shape: [`sample-generated/`](sample-generated/).

### Module layout and naming

**Package is the primary module tree.** A protobuf `package` (dot-separated) maps to nested Rust modules. Generated types live under that tree. Each message also gets a snake_case **companion module** (same parent) for message-local items; the struct itself is **not** nested inside that companion.

**Default mapping (idiomatic Rust paths).**

| Proto entity | Default Rust placement |
|---|---|
| `package example.v1` | `example::v1` |
| Top-level `message Task` | Struct `Task` in the package module (`example::v1::Task`). Companion `example::v1::task` holds `FIELD_*` / `BIT_*`, nested types, and oneofs. Omit the companion when it would be empty. |
| Nested `message Inner` inside `Task` | Struct `Inner` in Task's companion (`example::v1::task::Inner`). Companion `example::v1::task::inner` for Inner's own locals. |
| Nested `message Foo` inside `message Foo` | Outer `…::Foo` and inner `…::foo::Foo` — same-named nesting does not collide. |
| File-level `enum Status` | Type `Status` under the package module (exact file splitting is an implementation detail) |
| Nested `enum Kind` inside `Task` | Type `Kind` in Task's companion (`example::v1::task::Kind`) |
| `oneof notification` inside `Task` | Submodule `example::v1::task::notification` (see [§4.7](#47-oneof-fields)) |

Do **not** `pub use` a nested type into the same module as its parent struct. That is what would make `message Foo { message Foo {} }` (and `message Foo { enum Foo {} }`) collide.

**Companion item names omit the message.** The companion module is already the namespace, so generated items there do not repeat the message's Pascal name. For `message Task` the paths are `task::Inner`, `task::Kind`, `task::notification::Notification`, `task::MaxRetriesDefault` — not `TaskInner` / `TaskNotification` / `TaskMaxRetriesDefault`. The same rule covers message-local traits: `task::Message` and `task::MessageFallible`, not `TaskMessage` / `TaskMessageFallible`. The ingest-kind trait follows it too (`task::Layout`, `address::Layout`, `point::Layout`). The plugin emits that trait in the companion. Do not `pub use` these into the parent next to the struct. `Message` would sit beside `puroro::Message`, and `Layout` / `MessageFallible` would collide across messages. Callers qualify them (`task::MessageFallible`). `Layout` and `MessageFallible` are generic identifiers; keep the companion module in the path rather than importing the bare name.

Message-typed *fields* (e.g. `Address assignee`) reference sibling generated types by path; they do **not** by themselves create a nested module under the parent message.

**Shared module paths.** Distinct proto identities may map to the same Rust module path (e.g. `package example.task` and the companion module for `message Task` in `package example` both want `example::task`). The generator **merges** contributors into one module rather than rejecting the path up front. The struct `Task` lives in `example` and is not part of that merge. Item-level name clashes inside a merged module are left to `rustc` (and may later gain sharper generate-time diagnostics). The generator does **not** silently mangle names or make `_`-prefixed modules the normal public API — except the reserved private `_root` alias used for path qualification ([below](#path-qualification)).

**Generate-time renames.** Deliberate path choices (and disambiguation when a merge is undesirable) use **generator / plugin options supplied at generate time** — not options embedded in the `.proto` file. A rename rule identifies a proto entity by a stable logical name (typically its protobuf FQN / path) and substitutes a Rust module or type-path segment. Initial scope is **module and type path elements**; field accessor renames are out of scope until needed.

[`sample-generated/`](sample-generated/) currently keeps a flat module layout (no `example::` package prefix) for readability; production plugin output follows this section.

### Path qualification

Generated code must not rely on ambient `use` imports for the items it references. A `.proto` schema can introduce almost any identifier, so short names in generated output risk colliding with user code in the same scope.

**Two namespaces, two spellings.**

| Referent | Spelling in generated code |
|---|---|
| External crates (`puroro`, `puroro-rt`, `core`, `alloc`, `bytes`, `allocator_api2`, `bitvec`, …) | Leading-`::` absolute path, e.g. `::puroro::Message`, `::puroro_rt::SingularField` |
| Types / modules inside the **same generated forest** | `self::_root::…` from the forest root, e.g. `self::_root::example::v1::Address` |
| Nearby relatives in the same parent (sibling structs, child `mod` names, …) | Ordinary relative paths are fine |

**Why not `::` or `crate::` for generated names.** The forest may be the crate root (`lib.rs` of a generated crate) **or** an embedded submodule (`mod generated { include!(…) }` / a single `.rs` dropped into an app crate). Leading `::` / `crate::` always mean the **host** crate root, so they break under embedding. `self::_root` is stable in both layouts.

**How `_root` is wired.** Layout injects a private `mod _root` into every forest module:

```rust
// Forest root
mod _root {
    pub(super) use super::*;
}

// Every nested module
mod _root {
    pub(super) use super::super::_root::*;
}
```

From inside the nested `_root` submodule, `super::super::_root` always reaches the parent's alias (the extra `super` accounts for `_root` itself). Each module therefore spells the forest root as `self::_root`, at any depth.

**Reserved `_`-prefixed names** introduced by the generator (`_root`, `_common`, …) are exempt from proto-derived naming and must not be treated as public API.

**Crate split.** Items from [§3](#3-runtime-trait-api) (`Message`, `Optional`, `HasDefault`, `DecodeError`, …) are emitted as `::puroro::…`. Field catalog types, `MessageCommon`, wire helpers, and `ProtoDefault` are emitted as `::puroro_rt::…`. A generated crate's `Cargo.toml` lists both dependencies; end-user application code should not add `puroro-rt` directly.

### Public signatures must not surface `puroro-rt`

**Normative.** Application code that uses a generated message crate must compile and type-check with dependencies on **that generated crate and `puroro` only**. It must not need to `use` / name any `puroro_rt::…` item.

Generated crates **may** (and do) depend on `puroro-rt` for field storage, wire helpers, and other generator-facing machinery. That dependency is an implementation detail of the generated crate.

**What “surface” means.** A `puroro-rt` type is surfaced if a **public** item in the generated crate requires the library user to name it — typically:

- a `pub fn` / `pub` associated function **return type** or **parameter type** that mentions `puroro_rt::…` (including via a public type alias);
- a `pub type` / `pub struct` / `pub enum` / `pub trait` bound whose definition exposes `puroro_rt::…` in its public API;
- a trait object / RPIT bound that names a `puroro-rt` trait or associated type the user must mention to use the API.

**Allowed.**

- `puroro-rt` paths in **private** / `pub(crate)` fields, inherent helpers, and `impl` bodies.
- Public accessors that return **`impl Trait` from `puroro`** (e.g. `impl ::puroro::OneofView<…>`, `impl ::puroro::StringMut<A>`, `impl ::puroro::BytesMut<A>`, `impl ::puroro::RepeatedStringMut<A>`, `impl DerefMut<Target = ::puroro::String<A>>` for heap-layout strings) even when the hidden concrete type lives in `puroro-rt`.
- Public return types that are **Rust / `puroro` / generated-message** types only (e.g. `&str`, `i32`, `&Address<A>`, `Optional<…>`, `Notification<&str, …>` with concrete payloads).

**Not allowed (examples).**

- Public aliases such as `pub type FooRef<'a, A> = … <ProtoString as ::puroro_rt::EncodeType>::View<'a, A> …` (forces users/rustdoc to see `puroro-rt`).
- Public method signatures that name `puroro_rt::OneofView`, `SingularField`, `SingularType`, `StringGuard`, `BitRef`, field visitors, etc., when a `puroro` trait or RPIT / concrete user type would do.
- Requiring `use puroro_rt::…` for method resolution on values returned from public accessors (put the trait in `puroro` instead).

**Oneof illustration (current).** `notification()` names a concrete shared `Ref` shape (`Notification<&str, …, &Address<A>, bool>`). `notification_mut()` returns `impl ::puroro::OneofViewMut<Case = …>` without naming `Mut` (typed mutation goes through per-variant `_mut`). Catalog types stay on the `pub(crate)` storage alias / `OneofGroup` impl.

If a new public API would need a `puroro-rt` name, **add or reexport a stable stand-in in `puroro` first**, then generate against that.

---

### 4.0 Inherent accessors (current)

The normative generated API is the concrete message struct's inherent `impl` block. Sample messages ([`Task`](sample-generated/src/task_type.rs) / [`School`](sample-generated/src/school_type.rs) / [`Student`](sample-generated/src/student_type.rs) / [`Point`](sample-generated/src/point_type.rs) / [`Address`](sample-generated/src/address_type.rs)) are owned-only (`Drop` / `Message`); an inlined child is a full `M`, not a view over a shared parent common. Each message is also a lazy alias (`TaskLazy`, …) with fallible getters and no mutators ([§8](#tasklaya--lazy-parse-timing)). Per-message traits (`foo::Message` / `foo::MessageFallible`, in the companion) stay [§8](#8-future-work). Callers of today's generated plugin output use `Name` or `NameLazy` (or [`Message`](#message) for codec / infrastructure).

**Read / write shape (eager `Task<A>`):**

| Field kind | Getter | Mutator | Clear |
|---|---|---|---|
| IMPLICIT scalar (`score`, `done`) | bare value (`i32`, `bool`, …) | `*_mut()` → `impl DerefMut<Target = T>` | `clear_*()` |
| EXPLICIT / LEGACY_REQUIRED scalar, enum | `Optional<…, impl HasDefault<…>>` | `*_mut()` → `impl DerefMut<Target = …>` | `clear_*()` |
| Singular string (VERIFY, default SSO) | `Optional<&str, …>` or `&str` | `*_mut()` → `impl StringMut<A>` | `clear_*()` |
| Singular string (`utf8_validation=NONE`) | same views as bytes (`&[u8]`) | `*_mut()` → `impl BytesMut<A>` | `clear_*()` |
| Singular bytes (default SSO) | `Optional<&[u8], …>` or `&[u8]` | `*_mut()` → `impl BytesMut<A>` | `clear_*()` |
| IMPLICIT open enum (`status`) | `Optional` (`is_set` when non-zero) | `*_mut()` → `impl DerefMut<Target = E>` | `clear_*()` |
| Nested message (`assignee`, `origin`) | `Option<&Child>` (`Child` follows `(puroro.child_parse)` / parent layout: sample `assignee` → `AddressLazy<A>`, `origin` → `Point<A>`) | `*_mut()` → `&mut Child` (creates if absent) | `clear_*()` (+ `set_*` move-replace) |
| Repeated packable | `&[T]` | `*_mut()` → `impl DerefMut<Target = Vec<T, A>>` (`allocator_api2`) | `clear_*()` |
| Repeated message (`watchers`) | `&[Address<A>]` | `impl DerefMut<Target = Vec<Address<A>, A>>` (`push(addr)`) | `clear_*()` |
| Repeated string (VERIFY) / bytes | `&[impl Deref<Target = str>]` / `&[impl Deref<Target = [u8]>]` | `*_mut()` → `impl RepeatedStringMut<A>` / `impl RepeatedBytesMut<A>` (`push` then fill) | `clear_*()` |
| Repeated string (`utf8_validation=NONE`) | `&[impl Deref<Target = [u8]>]` | `*_mut()` → `impl RepeatedBytesMut<A>` | `clear_*()` |
| Map (scalar / string value) | `impl MapRef<K, V>` (`get` → `Option<&V>`) | `impl MapMut<K, V, MutTarget = …>` | `clear_*()` (or `*_mut().clear()`) |
| Map (message value) | [`MapRef`](src/map.rs) (`get` → `Option<&M>`) | [`MapMut`](src/map.rs) (`entry_mut` → `&mut M`) | `clear_*()` |
| Oneof group | `impl OneofView` (`case()` / `as_ref()`); `postal()` → `Option<&Address<A>>` | `impl OneofViewMut` + per-variant `*_mut()` (`postal_mut()` → `&mut Address<A>`) | `clear_notification()` |

Presence for EXPLICIT fields on the **eager** path is checked with `field().is_set()` / `field().get()` — there are **no** generated eager `has_*`, `*_raw`, or `push_*` helpers. The lazy alias does generate `has_*()` for wire presence without semantic decode ([§8](#tasklaya--lazy-parse-timing)). Nested messages use `set_*` as move-replace of the slot (`set_assignee` accepts eager `Address` and converts via `into_lazy` when the child is pinned lazy). Mutation is otherwise via `_mut` (+ `clear_*`); see also [§5.1 Mutation API](#51-chosen-design-single-type-parameter).

```rust
// EXPLICIT string — `push_str` needs `use puroro::StringMut`
let s: &str = task.title().get();
if task.title().is_set() { … }
task.title_mut().push_str("Fix bug");
task.clear_title();

// IMPLICIT scalar
*task.score_mut() = 42;

// Repeated packable
task.tag_ids_mut().push(7);

// Repeated string — allocator-less elements
task.labels_mut().push().push_str("urgent");
```

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
    fixed32 postal_code = 3;
    double latitude = 4;
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

    // Fields 12–15 / 18: Oneof (presence is intrinsic to oneof). Deliberately
    // heterogeneous to show LEN, VARINT, bool, and message variants.
    oneof notification {
        string  email_address = 12;
        string  phone_number  = 13;
        int32   webhook_id    = 14 [default = -1];  // VARINT + custom default
        Address postal        = 15;  // message variant
        bool    urgent        = 18;  // bool variant
    }

    // Field 16: IMPLICIT bool
    bool done = 16 [features.field_presence = IMPLICIT];

    // Field 17: EXPLICIT bool (edition default presence)
    bool flag = 17;

    // Field 19: Repeated nested message (cannot be packed; EXPANDED)
    repeated Address watchers = 19;

    // Field 20: Packed repeated bool (PACKED is the edition default for numeric)
    repeated bool votes = 20;

    // Field 21: Map (wire = repeated MapEntry message; last-wins on duplicate keys)
    map<string, int32> attributes = 21;
}
```

The subsections below document each field pattern in terms of its generated accessor API.

---

### 4.1 Scalar fields

Scalar types are copy types in Rust (`i32`, `bool`, `f32`, …). In the runtime catalog these are **singular** fields: **non-repeated**, covering both presence-tracked (“optional” / `EXPLICIT`) and non-presence-tracked (`IMPLICIT`) cases. Cardinality (singular vs repeated) is separate from presence (`FieldPresence`).

The key variation for scalars is **field presence**.

#### Implicit presence (`features.field_presence = IMPLICIT`)

Equivalent to proto3's default singular scalar behaviour.

```rust
pub fn score(&self) -> i32;
pub fn score_mut(&mut self) -> impl DerefMut<Target = i32> + '_;
pub fn clear_score(&mut self);
```

Wire rule: field absent from the wire when value equals the type-zero (`0`, `false`, `0.0`).

Singular `bool` uses the same accessor shape; storage is bit-packed into the message common bits (see [IMPLEMENTATION.md §10](IMPLEMENTATION.md#10-common-bit-indices)).

#### Explicit presence (`features.field_presence = EXPLICIT`, edition 2024 default)

Equivalent to proto2 `optional`. Presence is tracked independently of value. The **`Optional` accessor is the primary read API**; mutation uses `_mut` / `clear_*` (no generated eager `has_*`, `*_raw`, or `set_*`; lazy `has_*` is [§8](#tasklaya--lazy-parse-timing)):

| Method | Return type | Description |
|---|---|---|
| `max_retries()` | `Optional<i32, impl HasDefault<i32>>` | `get()` and `is_set()` |
| `max_retries_mut()` | `impl DerefMut<Target = i32>` | Sets the presence bit on acquire |
| `clear_max_retries()` | `()` | Clears the presence bit |

Wire rule: field absent when `max_retries().is_set()` is `false`; present even when the value is zero.

The `[default = 3]` option means `max_retries().get()` returns `3` when unset (field wrapper `D = MaxRetriesDefault`).

Because `Optional` is a concrete struct with no custom `Drop`, chaining compiles directly:

```rust
let n: i32 = task.max_retries().get();  // value or default
if task.max_retries().is_set() { … }    // presence check
*task.max_retries_mut() = 5;
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

String getters yield a borrowed `&str` (via `Optional::get` when presence-tracked) when `utf8_validation=VERIFY`. The default singular layout is SSO ([`InlineOrHeap`](IMPLEMENTATION.md#172-string--bytes-inline-optimisation)); `_mut` returns [`StringMut`](src/string_mut.rs) (`set` / `set_string` / `clear` / `push_str` / `push` / `truncate`). Callers need `use puroro::StringMut` for those methods. `(puroro.string_layout) = STRING_LAYOUT_HEAP` selects heap `UnmanagedString` and `impl DerefMut<Target = ::puroro::String<A>>` instead.

Short SSO values live in the message. Bytes past the string length in that slot are uninitialized and are not part of the value: getters, equality, and encode observe the logical `&str` only. Do not read the message's raw memory for a zeroed tail.

**Implicit presence:**

```rust
pub fn name(&self) -> &str; // "" when unset
pub fn name_mut(&mut self) -> impl ::puroro::StringMut<A> + '_;
pub fn clear_name(&mut self);
```

**Explicit presence:**

| Method | Return type | Description |
|---|---|---|
| `title()` | `Optional<&'s str, impl HasDefault<&'s str>>` | `get()` and `is_set()` |
| `title_mut()` | `impl ::puroro::StringMut<A>` | Sets presence; copy via `set` / `push_str` / etc. |
| `clear_title()` | `()` | Marks field as unset |

```rust
let s: &str = task.title().get();   // value or default
if task.title().is_set() { … }
task.title_mut().push_str("Fix bug");
```

Wire rule: absent when `title().is_set()` is false (EXPLICIT) or `""` (IMPLICIT).

**UTF-8 validation** (Editions `utf8_validation` feature): when `VERIFY` (proto3 / edition 2023–2024 default), invalid UTF-8 in a string payload is `DecodeError::InvalidUtf8`. Generated fields use `ProtoString` and the `&str` / `StringMut` accessors above. That error is a failed decode of a known field. It is not written into the unknown-field set. Closed-enum unknowns stay on the divert path: the wire value is a legal varint the schema does not name, and the spec keeps the parse successful. A `VERIFY` string is a field the schema does name, with a payload Editions require the parser to reject. Eager `decode` / `merge_from` return `InvalidUtf8`. Lazy `decode` accepts a structurally complete wire; the getter and `into_eager` return `InvalidUtf8` ([eager / lazy gaps](#eager-and-lazy-semantic-gaps)).

When `NONE` (proto2 syntax default; also settable per-field in editions), wire bytes are copied without validation. Generated fields use `ProtoStringUnchecked` and the **bytes** accessor surface (`&[u8]`, `BytesMut`, `RepeatedBytesMut`) so invalid UTF-8 is never presented as `&str`. The marker stays distinct from `ProtoBytes` because a protobuf `string` may be a map key and `bytes` may not. Schemas that must accept non-UTF-8 strings use `NONE`. There is no lossy-`&str` mode.

To keep proto2 strings as `&str` without changing the `.proto` files, pass the plugin option `proto2_utf8=verify` (`--puroro_opt=proto2_utf8=verify`). That overlays only the proto2 syntax default; editions `features.utf8_validation = NONE` stays bytes.

---

### 4.3 Bytes fields

Bytes fields follow the same pattern as strings with `&[u8]` as the value type. Default singular layout is SSO; `_mut` returns [`BytesMut`](src/bytes_mut.rs) (`set` / `set_vec` / `clear` / `extend_from_slice` / `push` / `truncate`). Callers need `use puroro::BytesMut`. `(puroro.bytes_layout) = BYTES_LAYOUT_HEAP` selects heap `UnmanagedVec` and `impl DerefMut<Target = Vec<u8, A>>` (`allocator_api2`) instead. Unused SSO capacity is uninitialized and is not visible through `&[u8]`, same as strings.

**Implicit presence:**

```rust
pub fn payload(&self) -> &[u8]; // &[] when unset
pub fn payload_mut(&mut self) -> impl ::puroro::BytesMut<A> + '_;
pub fn clear_payload(&mut self);
```

**Explicit presence:**

| Method | Return type | Description |
|---|---|---|
| `payload()` | `Optional<&'s [u8], impl HasDefault<&'s [u8]>>` | `get()` and `is_set()` |
| `payload_mut()` | `impl ::puroro::BytesMut<A>` | Sets presence; copy via `set` / `extend_from_slice` / etc. |
| `clear_payload()` | `()` | Marks field as unset |

---

### 4.4 Repeated fields

Repeated fields expose a slice-like read API and `_mut` / `clear_*` mutation. The getter returns a reference to a contiguous sequence, **not** an owned `Vec`, so callers get O(1) random access without allocation. Mutation guards target `allocator_api2::vec::Vec<T, A>` (shown as `Vec<T, A>` below) except for string/bytes elements.

**Packed repeated scalar (`tag_ids: repeated int32`, field 6):**

```rust
pub fn tag_ids(&self) -> &[i32];
pub fn tag_ids_mut(&mut self) -> impl DerefMut<Target = Vec<i32, A>> + '_;
pub fn clear_tag_ids(&mut self);
```

**Non-packed repeated scalar (`scores: repeated int32 [EXPANDED]`, field 7):**

```rust
pub fn scores(&self) -> &[i32];
pub fn scores_mut(&mut self) -> impl DerefMut<Target = Vec<i32, A>> + '_;
pub fn clear_scores(&mut self);
```

The accessor API is identical for packed and non-packed; the difference is only in the wire encoding.

**Repeated string (`labels: repeated string`, field 8):**

```rust
pub fn labels(&self) -> &[impl Deref<Target = str>];
pub fn labels_mut(&mut self) -> impl RepeatedStringMut<A> + '_;
pub fn clear_labels(&mut self);

// Append: push an empty element, then fill it
task.labels_mut().push().push_str("urgent");
```

Library users import [`RepeatedStringMut`](src/repeated.rs) / [`String`](src/lib.rs) from `puroro` (not `puroro-rt`). Internal storage remains allocator-less; callers write through `push()` rather than constructing elements by hand. There is no generated `push_label` helper.

**Repeated message (`watchers: repeated Address`, field 19):**

```rust
pub fn watchers(&self) -> &[Address<A>];
pub fn watchers_mut(&mut self) -> impl DerefMut<Target = Vec<Address<A>, A>> + '_;
pub fn clear_watchers(&mut self);

// Append: push an empty element, then fill it (or copy_from an owned Address)
task.watchers_mut().push().street_mut().set("2 Side Rd");
assert_eq!(task.watchers().get(0).unwrap().street().get(), "2 Side Rd");
```

Each element is a stored `Address` with its **own** `MessageCommon`. Getters expose that slice (`&[Address<A>]` / growable `Vec`). Each wire occurrence **appends** a newly decoded message. Unlike singular nested messages, repeated elements are not merge-into at a list index. Messages cannot be packed; encode always uses one LEN record per element.

**Repeated bool (`votes: repeated bool`, field 20):**

```rust
pub fn votes(&self) -> &[bool];
pub fn votes_mut(&mut self) -> impl DerefMut<Target = Vec<bool, A>> + '_;
pub fn clear_votes(&mut self);
```

Elements are plain `bool` in a growable vec — **not** bit-packed into `MessageCommon` (that layout is singular / oneof only). Packable: encode follows `PACKED` / `EXPANDED`; decode accepts both.

#### Packed vs non-packed encoding

| Feature setting | Wire encoding |
|---|---|
| `PACKED` (edition 2024 default for numeric fields) | One LEN record containing all element values |
| `EXPANDED` | One VARINT/I32/I64 record per element |

**Compatibility requirement (spec-mandated):** The decoder must accept *both* packed and non-packed forms for any packable repeated field, regardless of what the schema declares.

String and message fields cannot be packed; they always use one LEN record per element.

---

### 4.5 Nested message fields

Message fields are optional in the generated struct (`None` = not present). Singular getters return `Option<&M>` / `&mut M`. An inlined child is a full `M` with its own `MessageCommon` (bits, allocator, unknowns). Boxed children store `UnmanagedBox<M>`. Sample `set_assignee` / `set_origin` move-replace the slot.

```rust
// Sample pins assignee lazy on both aliases (`(puroro.child_parse) = LAZY`).
pub fn assignee(&self) -> Option<&AddressLazy<A>>;

/// Creates a default value if absent.
pub fn assignee_mut(&mut self) -> &mut AddressLazy<A>;

pub fn set_assignee(&mut self, src: Address<A>); // eager → into_lazy, then move-replace
pub fn set_assignee_lazy(&mut self, src: AddressLazy<A>);

pub fn clear_assignee(&mut self);
```

**Merge semantics:** when the same message field appears more than once on the wire, occurrences are *merged* rather than replaced. This implements protobuf's "concatenated bytes = merged message" property. Oneof message variants (`postal`) use the same `&Address` / `&mut Address`; see [§4.7](#47-oneof-fields).

#### Child parse timing

Every proto message emits both an eager type (`Task`) and a lazy type (`TaskLazy`). Which one a caller holds is a Rust type choice. `(puroro.child_parse)` does not make that choice. It chooses the **nested** message's layout. The signatures above are the default: an eager parent stores an eager child, and a lazy parent stores a lazy child.

Resolution, for a singular message, a repeated message, a map value, or a oneof message variant:

1. The field option `(puroro.child_parse)`, when it is `EAGER` or `LAZY`.
2. Otherwise the enclosing message's `(puroro.message_child_parse)`, when that is set.
3. Otherwise the child follows the parent layout.

Unset and `UNSPECIFIED` are step 3. A message option is the default for fields **declared on that message**. It does not force every embedding of that type, in other parents, onto one layout. A child message uses its own option for its own fields.

`(puroro.message_layout)` (inline versus boxed) is a separate choice. A pinned-lazy child may still be inlined or boxed.

A pin that disagrees with the parent layout is a concrete type on both aliases, not a `Layout` associated type. The sample's `assignee` is that case: field-level `LAZY`, so eager `Task` and `TaskLazy` both store `AddressLazy`, and `set_assignee` accepts eager `Address` via `into_lazy`. `origin`, `watchers`, and `postal` are step 3. The generator treats step 3 as the default and emits a pin only when the option says so.

Both extensions are field number `51404` in [`proto/puroro/options.proto`](proto/puroro/options.proto). Protobuf extension names are unique in a package, so the field option is `(puroro.child_parse)` and the message option is `(puroro.message_child_parse)`. The plugin reads that number and emits a pin when the option is `EAGER` or `LAZY`. The sample still comments the `assignee` pin as a stand-in for that option; it is not sample-only.

---

### 4.6 Enum fields

The wire encoding is always VARINT. Open vs closed is reflected in the **generated enum newtype** (see [Generated enum type](#generated-enum-type) below); field accessors use the same [`Optional`](#hasdefault-and-optional) pattern as other singular fields — no `Result` wrapper.

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
pub fn status_mut(&mut self) -> impl DerefMut<Target = Status> + '_;
pub fn clear_status(&mut self);
```

IMPLICIT presence: `is_set()` is `true` when the wire value is non-zero; `get()` returns `Status::UNSPECIFIED` when unset.

#### Closed enum (`enum_type = CLOSED`)

Unknown wire values are diverted to unknown fields on decode. The generated type accepts only known values (`TryFrom<i32>`); conversion to `i32` is infallible (`From<Priority> for i32`).

```rust
pub fn priority(&self) -> Optional<Priority, impl HasDefault<Priority>> {
    self.priority.bind(&self._common).optional()
}
pub fn priority_mut(&mut self) -> impl DerefMut<Target = Priority> + '_;
pub fn clear_priority(&mut self);
```

EXPLICIT presence: `is_set()` tracks the presence bit; `get()` returns `Priority::UNSPECIFIED` when unset.

#### Generated enum type

The default is a **newtype-over-`i32`** — the same shape as official protobuf Rust. Openness is a **type-level** distinction:

- Markers [`Open`](puroro-rt/src/fields/wire/numerical.rs) / [`Closed`](puroro-rt/src/fields/wire/numerical.rs)
- Traits [`OpenEnum`](puroro-rt/src/fields/wire/numerical.rs) (`From<i32>`) / [`ClosedEnum`](puroro-rt/src/fields/wire/numerical.rs) (`TryFrom<i32>`)
- Field wrapper [`ProtoEnum<E, K>`](puroro-rt/src/fields/wire/numerical.rs)

[`SingularField`](puroro-rt/src/fields/singular/field.rs) (alias `SingularField`) stores `ProtoEnum<E, K>` and reuses the same `optional()` / `value_mut()` paths as other singular scalars (getters still project to `E` / `&mut E`). Merge is a single decode-then-write path: closed enums signal unknowns via [`DecodeError::UnknownClosedEnum`](src/error.rs) from `decode_wire`, and `merge` catches that to append the raw varint to unknown fields (`TryFrom` is the single source of truth).

```rust
// Open — any wire value is valid storage
status: SingularField<ProtoEnum<Status, Open>, Implicit, FIELD, A>,

// Closed — only known wire values are stored
priority: SingularField<ProtoEnum<Priority, Closed>, Explicit<BIT>, FIELD, A>,

#[repr(transparent)]
pub struct Status(i32);

impl ProtoEnumStorage for Status {
    fn to_wire(self) -> i32 { … }
}
impl OpenEnum for Status {}

#[repr(transparent)]
pub struct Priority(i32);

impl ProtoEnumStorage for Priority { … }
impl ClosedEnum for Priority {}
```

Alias names with the same integer all map to the same `Self(v)`; equality is by wire value.

To opt into a native Rust `enum` (exhaustive `match` on known names), set `(puroro.enum_repr) = ENUM_REPR_RUST_ENUM` on the enum. Open rust-enums add `_Unknown(i32)` so unknown wire values stay representable; closed rust-enums stay a closed set of unit variants. That form **cannot** host two names that share a number: if the proto actually aliases a value, codegen fails with a diagnostic. `allow_alias = true` with no duplicate numbers is fine. Unset / `ENUM_REPR_UNSPECIFIED` / `ENUM_REPR_NEWTYPE` keep the default newtype.

---

### 4.7 Oneof fields

Each `oneof` group generates types in a submodule named after the **oneof** (lower-snake-case) under the parent message's companion module (e.g. `task::notification`), because the owned storage holds allocator-less `unmanaged` values (unsafe to drop implicitly) that must not leak into the public API. Module placement and collision handling follow [Module layout and naming](#module-layout-and-naming).

Shape and storage share **one** generic enum; Storage is a `pub(crate)` alias. Ref / Mut projections are written inline on `OneofGroup` (no public Ref/Mut aliases). Group bound views are returned as `puroro` traits (`OneofView` / `OneofViewMut`); the concrete runtime structs live in `puroro-rt` and stay hidden behind RPIT:

```rust
// In module `task` / `notification`:

// Canonical shape — variant payloads are type parameters.
pub enum Notification<Ea, Pn, Wh, Po, Ur> {
    EmailAddress(Ea), PhoneNumber(Pn), WebhookId(Wh), Postal(Po), Urgent(Ur),
}

// (1) Owned storage — crate-internal alias; per-variant private field aliases
//     are the single source of truth. Implements OneofGroup, OneofDeallocate,
//     encode glue, OneofGroup::after_deallocate (group-wide bit wipe).
type EmailAddressField<A> = SingularField<
    ProtoString, Oneof, { FIELD_EMAIL_ADDRESS }, A, InlineOrHeap<{ BIT_EMAIL_ADDRESS_SSO }>,
>;
// … PhoneNumberField, WebhookIdField (+ custom D), UrgentField (+ BitPacked)
type PostalField<A> = SingularField<
    ProtoMessage<Address<A>>, Oneof, { FIELD_POSTAL }, A,
>;
pub(crate) type NotificationStorage<A> = Notification<
    EmailAddressField<A>, PhoneNumberField<A>, WebhookIdField<A>, PostalField<A>, UrgentField<A>,
>;

// (2) Payload-less case discriminant (unset is `None`, so no `NotSet` member).
pub enum NotificationCase { EmailAddress, PhoneNumber, WebhookId, Postal, Urgent }

// Accessors on Task — `puroro` traits so library users never name `puroro-rt`.
// Ref is the concrete shape (no alias); Mut is opaque RPIT (per-variant `_mut`
// for typed mutation). Pattern-match shared views with `Notification::…`.
pub fn notification(&self) -> impl ::puroro::OneofView<
    Case = NotificationCase,
    Ref = Notification<&str, &str, i32, &Address<A>, bool>,
> + '_;
pub fn notification_mut(&mut self) -> impl ::puroro::OneofViewMut<Case = NotificationCase> + '_;

// On ::puroro::OneofView:
//   case() -> Option<Case>
//   as_ref() -> Option<Ref>
// On ::puroro::OneofViewMut:
//   as_view(&self) -> Shared<'_>  // implements OneofView
//   as_mut(self) -> Option<Mut>   // active variant; no switch
//   clear(self)                   // free active variant

// Per-variant `_mut` accessors switch the oneof to that variant (freeing any
// previously-active one) and return a handle appropriate to the kind:
pub fn email_address_mut(&mut self) -> impl ::puroro::StringMut<A> + '_;
pub fn phone_number_mut(&mut self) -> impl ::puroro::StringMut<A> + '_;
pub fn webhook_id_mut(&mut self) -> impl DerefMut<Target = i32> + '_; // VARINT variant
pub fn postal_mut(&mut self) -> &mut Address<A>;     // inlined ProtoMessage
pub fn urgent_mut(&mut self) -> impl DerefMut<Target = bool> + '_; // bool variant

// Clears whichever variant is active (freeing it):
pub fn clear_notification(&mut self); // = notification_mut().clear()
```

Note: enums cannot carry unused lifetime/allocator parameters via `PhantomData` (unlike a struct). Integer-only oneofs therefore omit `'a` / `A` from the shape and from `Ref`/`Mut` aliases when no variant payload needs them.

**Variants own field wrappers, not raw storage.** A oneof member of a given kind reuses the exact field wrapper an ordinary singular field of that kind uses (`SingularField` / aliases — including [`ProtoBool`](puroro-rt/src/fields/wire/numerical.rs) for `bool` and [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs) for messages), so the storage / `value` / `value_mut` / `deallocate` machinery is shared rather than reimplemented. The wrapper's *presence* is inert for a oneof — presence is tracked by the enclosing `OneofSlot` — so `FieldPresence::Oneof` is used (omit rules never consulted; bool still reads/writes its value bit).

To keep generated code thin, each wrapper is driven with the **field's own** construction, merge, and access primitives — no bespoke helpers on the oneof enum. Empty construction uses [`DefaultIn`](puroro-rt/src/fields/shared.rs) on the variant's field wrapper (`SingularField::default_in` / `new_in`); mut paths use a single bind:

| kind | install empty variant | merge one occurrence | mut accessor |
|---|---|---|---|
| LEN | `SingularField::default_in(alloc)` | `slot.bind_mut(common).variant_mut::<FIELD_…>().bind_mut(common).merge(…)` | `…variant_mut::<FIELD_…>().bind_mut(common).value_mut()` |
| VARINT | same | same | same |
| bool | same (`ProtoBool` + `BitPacked`) | same | same → `impl DerefMut<Target = bool>` |
| message | same (`ProtoMessage` inlined `M`, or `Boxed`) | same | same → `&mut M` |

Every variant merges through the **same** `slot.bind_mut(common).variant_mut::<FIELD_…>().bind_mut(common).merge(wire, buf)` shape. The message variant merges *into* the present child rather than replacing it.

The VARINT variant owns no heap: its `DeallocateIn` is a no-op.

**The sample message variant uses `Oneof` storage — always-present inlined `Address`, not `Option`.** Ordinary boxed `assignee` is `SingularField<ProtoMessage<…>, Message, …, Boxed>` (`Option<UnmanagedBox<M, A>>`). Oneof `postal` is `SingularField<ProtoMessage<Address>, Oneof, …>` (`Slot = Address`). Accessors are `value(common)` / `value_mut(common)` (`&Address` / `&mut Address`). Switching the case deallocates the child (its own `Drop` releases bits / unknowns), then [`OneofGroup::after_deallocate`](puroro-rt/src/fields/oneof.rs) clears bits the group owns on the parent. `ManuallyDrop` enables uniform [`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs) from `&mut self`.

**Why these types, and why Storage is not public under the `Notification` name alone.** The storage alias's payloads are `unmanaged`-backed field wrappers, which panic on implicit drop and need the message allocator to free. Exposing that alias publicly would let a caller own one and hit that footgun, and would leak `unmanaged` / `puroro-rt` into the API. So `NotificationStorage` is `pub(crate)`. The public surface is the shape `Notification`, `NotificationCase`, and [`puroro::OneofView`](src/oneof.rs) / [`puroro::OneofViewMut`](src/oneof.rs) (RPIT; runtime structs in `puroro-rt` implement those traits). There are no public Ref/Mut aliases — shared `notification()` names the concrete Ref shape; `notification_mut()` keeps Mut opaque. Shared getters live on `OneofView`; while holding a mut view, call `as_view()` (not `Deref` — a by-value reborrowed view cannot be returned from `Deref::deref`).

Group accessors follow the same bound-view idiom as other fields: `slot.bind(&common)` / `slot.bind_mut(&mut common)` yield [`OneofSlotRef`](puroro-rt/src/fields/oneof.rs) / [`OneofSlotMut`](puroro-rt/src/fields/oneof.rs). Runtime [`OneofView`](puroro-rt/src/fields/oneof.rs) / [`OneofViewMut`](puroro-rt/src/fields/oneof.rs) wrap that pair, keyed by [`OneofGroup`](puroro-rt/src/fields/oneof.rs) on the storage alias. `OneofSlotMut::variant_mut::<FIELD>()` selects (or installs) the variant for protobuf field number `FIELD` via [`DefaultIn`](puroro-rt/src/fields/shared.rs) on [`OneofVariant::Value`](puroro-rt/src/fields/oneof_variant.rs) and returns `&mut` the field wrapper. That consumes the slot view so `common` can be re-borrowed; callers then `field.bind_mut(common)` for `.value_mut()` / `.merge(…)`. `set(value)` replaces the whole group, and `clear()` frees the active variant. Each of these releases the previously-active variant via `OneofDeallocate::deallocate` and then [`OneofGroup::after_deallocate`](puroro-rt/src/fields/oneof.rs) (clears **every** bit the group owns — not a per-case match; do not zero neighbouring non-oneof bits such as `done` / `flag`). Message `Drop` uses [`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs) on the slot only. The old `set_*` per-variant setters are removed.

Parent `_mut` accessors and decode arms are then —
`slot.bind_mut(common).variant_mut::<FIELD_EMAIL_ADDRESS>().bind_mut(common).value_mut()`,
`slot.bind_mut(common).variant_mut::<FIELD_WEBHOOK_ID>().bind_mut(common).merge(wire, buf)`, etc.

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

- Per-variant **`_mut`** (`email_address_mut`, …) goes through `slot.bind_mut(common).variant_mut::<FIELD_…>().bind_mut(common)`, which force-switches the case and builds a fresh wrapper via [`DefaultIn`](puroro-rt/src/fields/shared.rs) (**type** default). That matches official `mutable_*`.
- Per-variant **getters** use `slot.bind(&common).variant_of::<FIELD_…>().optional()`. The field wrapper's `D` type parameter is the proto default marker (`ProtoDefault`, or a message-local ZST such as `WebhookIdDefault` for `[default = -1]`). When the case is unset or another variant, `optional` returns `Optional::new(None)` so `is_set()` is false and `get()` yields `D::DEFAULT` — without selecting the variant. That matches official const getters.
- When the variant **is** active, getters return the **stored** value (including type zero / empty string), same as official.

**Codegen rule.** `[default = X]` on a oneof scalar becomes the field wrapper's `D` (`Singular*Field<…, D>`), which flows into the read accessor (`Optional<…, D>`). It must **not** change `_mut` installation: mutators keep installing type-default storage; custom defaults must not be written into the slot merely because the caller asked for a mutable handle.

---

### 4.8 Required fields (`LEGACY_REQUIRED`)

`features.field_presence = LEGACY_REQUIRED` corresponds to proto2's `required` modifier. It is intended as a **migration path** for existing proto2 schemas and is not recommended for new fields.

On the wire, a `LEGACY_REQUIRED` field is indistinguishable from an `EXPLICIT` field; the constraint is schema-level only.

**Accessor API** — same `Optional` + `_mut` / `clear_*` pattern as `EXPLICIT`, plus [`Message::validate`](#message):

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

**Default implementation (today):** [`UnknownFields`](puroro-rt/src/unknown_fields.rs) in `MessageCommon` (about one word when empty). The first unknown allocates a heap blob of this message’s unrecognized wire bytes. `skip_field_and_save` / closed-enum diversion append that blob. Encode re-emits it via `Deref` to `[u8]`. The public iterator parses it via `puroro_rt::decode::iter_unknown_fields`. Inlined children keep their own `MessageCommon` and therefore their own unknown store.

`Message::unknown_fields()` stays the self-blob iterator. An inlined child is a full `M`, so unknowns recorded while merging that child live on the child’s own store and re-encode **inside the child’s LEN**. `clear_*` / oneof switch / Drop of an inlined slot drops that child (and its unknowns).

**Policies:** [`UnknownStore`](puroro-rt/src/unknown_fields.rs) is the third type parameter on [`MessageCommon`](puroro-rt/src/fields/shared.rs) (`U = UnknownFields<A>` by default). [`DiscardUnknowns`](puroro-rt/src/unknown_fields.rs) is a ZST: decode still consumes unrecognized payloads; the iterator is empty; encode emits nothing. [`RejectUnknowns`](puroro-rt/src/unknown_fields.rs) is a ZST: a well-formed unrecognized tag fails decode with [`DecodeError::UnrecognizedField`](src/error.rs); the iterator is empty; encode emits nothing. A truncated payload is that wire error, not `UnrecognizedField`. Public messages stay `Foo<A>` — the store is **baked** into that type (`MessageCommon<Bits, A>`, `MessageCommon<Bits, A, DiscardUnknowns>`, or `MessageCommon<Bits, A, RejectUnknowns>`), so impls are `impl<A> Foo<A>` with no `U:` bound. Sample [`Point`](sample-generated/src/point_type.rs) preserves; sample [`Marker`](sample-generated/src/marker_type.rs) bakes discard.

Closed-enum unknowns are not unrecognized tags. They still take the divert path (`save_unknown_varint_field`). On a ZST store that path drops the value and decode succeeds; the field reads as unset and the value is not re-encoded.

Codegen: [`(puroro.unknown_fields)`](proto/puroro/options.proto) on `google.protobuf.MessageOptions` (`UNSPECIFIED` / `PRESERVE` / `DISCARD` / `REJECT`, field 51403). Unspecified and Preserve emit the default store; Discard bakes `DiscardUnknowns`; Reject bakes `RejectUnknowns`. A parent policy does not change a child’s store. The lazy alias calls the same reject check while scanning; preserve and discard still leave unknown tags in the ingested wire until `into_eager`.

| Policy | Behaviour |
|---|---|
| **Preserve** (default) | Spec behavior. Keep unknowns for round-trip; expose via the iterator |
| **Discard** | Accept the message and drop the bytes. Outside the current preservation rule |
| **Reject** | Fail decode. Outside the protobuf spec. Closed-enum unknowns stay on the divert path |
| **Custom** | Later hook / additional `UnknownStore` impl |

**Outside the spec.** The language guides require a parser to accept a field number it does not know and to write those bytes back on the next serialization ([proto3: Updating A Message Type](https://protobuf.dev/programming-guides/proto3/#updating), and the same section in the [proto2 guide](https://protobuf.dev/programming-guides/proto2/#updating)). Early proto3 dropped unknown fields; preservation has been the rule again since proto3.5, and Editions did not add a reject feature. The wire format carries a wire type so an old parser can skip a new field. `PRESERVE` is that behavior.

`DISCARD` still accepts the message. It only omits the bytes. `REJECT` does not accept the message: a well-formed unrecognized tag is `DecodeError::UnrecognizedField`. No protobuf syntax, Edition feature, or conformance case defines that failure. It is a puroro option for a closed schema, and a message that uses it is not a conforming parser of the wire format. Closed-enum unknowns stay on the spec's "do not fail the parse" path. With no store, that value is dropped, so a `REJECT` message does not preserve them either.

---

### 4.10 Map fields

Protobuf `map<K, V>` is syntactic sugar for a repeated message entry:

```protobuf
message MapFieldEntry {
  key_type key = 1;
  value_type value = 2;
}
repeated MapFieldEntry map_field = N;  // always LEN on the wire
```

**Key types** are integral types, `bool`, or `string` (not floating-point, `bytes`, enum, or message). **Values** may be any non-map type. Duplicate keys use **last-wins** semantics; iteration / encode order is unspecified.

Public map views live in [`src/map.rs`](src/map.rs) (re-exported from `puroro`). `K` / `V` are **view** types (`str`, `i32`, …), not owned buffers — `utf8_validation=VERIFY` string maps use `K = str`; `NONE` string maps use `K = [u8]`.

Same layering as singular fields (`EncodeType::View` / `ValueLayout::Mut` → generic `SingularField`): wire markers carry the view GATs, and `MapFieldRef` / `MapFieldMut` implement the user traits with **blankets** (no K×V macro matrix).

| Layer | Role |
|---|---|
| [`MapKey`](puroro-rt/src/fields/wire/map_element.rs) | `key_from_view` (sized copy vs string allocate) |
| [`RepeatedElement::RefView`](puroro-rt/src/fields/wire/repeated_element.rs) | Shared key/value view + `as_ref_view` (identity vs string/bytes `Deref`) |
| [`RepeatedElementMut::MutTarget`](puroro-rt/src/fields/wire/repeated_element.rs) | `_mut` target (`i32`, `String<A>`, …) |
| [`MapRef`](src/map.rs) / [`MapMut`](src/map.rs) | User API: `get` / `get_mut` / `entry_mut` / `remove` take `impl Borrow<K>` |

There is no separate `insert` API: set values with `entry_mut` then assign / fill. Codegen pins `MutTarget` on the mutator return type so those methods resolve through `impl Trait`. Message-valued maps use the same `MapRef` / `MapMut` with `V = M` (`get` → `Option<&M>`). Stored values remain owned `M` (own `MessageCommon`).

Generated accessors (`tag_ids` / `watchers` idiom):

```rust
/// Shared view (`get` / `len` / `is_empty`).
pub fn attributes(&self) -> impl ::puroro::MapRef<str, i32> + '_;

pub fn attributes_mut(
    &mut self,
) -> impl ::puroro::MapMut<str, i32, MutTarget = i32> + '_;

pub fn clear_attributes(&mut self); // calls MapMut::clear
```

Catalog types (`MapFieldRef` / `ProtoString` / …) stay inside the generated crate / `puroro-rt`.

Typical mutation:

```rust
*task.attributes_mut().entry_mut("region") = 81;
*task.attributes_mut().get_mut("region").unwrap() = 99;
assert_eq!(task.attributes().get("region"), Some(&99));

*flags_mut().entry_mut(1) = true;

// string / bytes value — ensure then fill
labels_mut().entry_mut(1).push_str("hello");
```

- `Option` from `get` means **key absence**, not singular field presence.
- Missing `key` / `value` inside an entry decode as protobuf type defaults.
- Map entry unknowns are skipped (not preserved).

Catalog type: `MapField<K, V, FIELD, A>` with `K: MapKey`, `V: RepeatedElement` — see [IMPLEMENTATION.md §15.1](IMPLEMENTATION.md#151-map-fields).

---

## 5. Allocator design

### 5.1 Chosen design: single type parameter

Every generated type carries a single allocator type parameter `A` that applies to all heap allocations within that message and its nested messages:

```rust
pub struct Task<A: Allocator + Clone = Global> { /* … */ }
```

`A` defaults to `Global`, so `Task` (without a type argument) works identically to a version without allocator support. The struct bound is `Allocator + Clone` on every generated message, eager and lazy. Why `Clone` sits on the struct: [below](#why-a-clone-is-a-struct-bound).

#### Why `A: Clone` is a struct bound

Every generated message struct requires `A: Allocator + Clone`. The bound does not vary by method, and it does not vary by whether the message is eager, lazy, or contains a lazy child. Callers write one bound and do not have to know which fields are lazy.

`Clone` is required because the allocator is stored by value and handed to children by value:

- **Decoded data.** `MessageCommon` stores one `A`. Field operations and `Drop` clone it into unmanaged buffers, map tables, and nested children (see [Owned allocator, never a borrow](#owned-allocator-never-a-borrow) below). The type used to grow a buffer is the same `A` used to free it, and clones are interchangeable.
- **Lazy wire handles.** A lazy child keeps a sliced view of the parent's wire and does not copy the LEN payload. [`SharedWire`](puroro-rt/src/decode/shared_wire.rs) stores `A` beside the `bytes::Bytes` blob and clones `A` in `clone_handle` / `slice`. The child type stays `AddressLazy<A>` with the parent's `A`.

Giving the child `&A` would avoid `Clone`, and `&A: Allocator` whenever `A: Allocator`. The child would then be bound by the parent's lifetime, and each nesting level would add another reference (`Address<&A>`, then `Address<&&A>`). `AsRef` can peel those references back to `&A` at a call site, but the child's type parameter would still be a reference, not `A`. Keeping the child's allocator type equal to the parent's `A` is the point of the single type parameter.

A narrower bound would leak laziness into the public API. `A: Clone` only on methods that walk a lazy field, or only on messages that contain one, changes the signature according to an implementation detail. An eager `Task` whose `assignee` is `AddressLazy<A>` needs the bound for `PartialEq`, `Debug`, and encode: those impls require the child to be `PartialEq` and `Debug`, and the child's impls scan and clone. A getter that only returns `&AddressLazy<A>` does not clone `A` itself; the struct bound still applies so that signature matches every other method.

`Layout` traits stay `A: Allocator`, so naming a layout does not demand `Clone`. An impl whose associated types name another message struct (`PointImpl`, `AddressImpl`) uses `A: Allocator + Clone`, as does `MessageFallible::Alloc`.

Typical allocators (`Global`, arena handles such as `bumpalo`) implement `Clone`.

**Single canonical allocator (almost no per-field copies).** The allocator is stored in `MessageCommon.alloc`. Heap-backed singular / repeated / oneof fields do **not** embed an allocator *instance*: they use the private [`unmanaged`](unmanaged/) types — `UnmanagedBox<T, A>`, `UnmanagedVec<T, A>`, `UnmanagedString<A>` — which keep only `ptr`/`len`/`cap` (plus `PhantomData<A>`) inline and receive an owned allocator (an `alloc.clone()`) on each operation that (de)allocates. The allocator **type** `A` still appears on unmanaged buffers and on **field wrappers** (`SingularField<…, A>`, `RepeatedField<…, A>`, `MapField<…, A>`); markers themselves are allocator-free.

**Exception — map fields.** Each `MapField` stores a `hashbrown::HashMap<…, A>` that **owns** its own `A` (hashbrown’s allocator-aware API embeds `A` in the table). Maps are uncommon, so the extra `A` per map field is accepted; `size_of::<Task<A>>` grows by roughly one `A` for `MessageCommon` plus one `A` per map field.

<a id="owned-allocator-never-a-borrow"></a>

**Owned allocator, never a borrow.** Operations pass the allocator **by value** rather than `&A`: the caller clones the canonical `MessageCommon.alloc` for each field operation. This keeps the allocator type consistently `A` for both a buffer's growth and its eventual free — mixing `&A` at allocation with `A` at deallocation is fragile and not obviously idempotent. Correctness relies on the `Allocator + Clone` contract that clones are interchangeable. (The one exception is building an *empty* `unmanaged` container, which never allocates, so it may borrow.)

**Manual release via `Drop`.** Because `unmanaged` values cannot free themselves (they panic if dropped implicitly), each field wraps its payload in `ManuallyDrop` and implements [`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs) as `deallocate(&mut self, common: &MessageCommon<…>)`. Every generated message implements `Drop`, walking its direct children and calling `deallocate(&self._common)`. Nested messages (boxed or inlined) are freed by their own `Drop` / `DeallocateIn`. The `unsafe` boundary is confined to the `puroro-rt` runtime and the generated `Drop`.

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

**Derived traits.** Generated messages implement `Default` (for `A: Allocator + Clone + Default`) plus a custom `Drop` (for `A: Allocator + Clone`). `Task::new()` is a `Global`-only convenience so `Task::new()` infers (`new()` delegates to `Default`; custom allocators use `new_in` / `Task::<A>::default()`). `Clone` / `PartialEq` / `Debug` are hand-written (not `#[derive]`): `Clone` delegates to field-wise [`FieldCloneIn`](puroro-rt/src/fields/shared/field_inspect.rs) (which uses [`CloneIn`](unmanaged/) for heap payloads); eager `PartialEq` / `Debug` compare / format via semantic getters. The lazy alias compares eager values when both sides build one, and ingested body bytes when both builds fail ([eager / lazy gaps](#eager-and-lazy-semantic-gaps)). Wire bytes are not deterministic across encodes. Full matrix: [IMPLEMENTATION.md §13](IMPLEMENTATION.md#13-derived-traits).

**Mutation API (`_mut`).** Mutation is unified under `_mut` accessors (RPIT). Default singular `string` / `bytes` yield `impl ::puroro::StringMut<A>` / `impl ::puroro::BytesMut<A>` (SSO; heap layout via `(puroro.string_layout)` / `(puroro.bytes_layout) = HEAP` is `impl DerefMut<Target = ::puroro::String<A>>` / `impl DerefMut<Target = Vec<u8, A>>`). Packable repeated `*_mut()` yield `impl DerefMut<Target = Vec<_, A>>` (`allocator_api2`); scalar/enum `_mut` accessors return `impl DerefMut<Target = T>`. The guard **owns** a clone of the message allocator when the payload is heap-backed. Acquiring an explicit-presence `_mut` sets the presence bit. There are no generated `set_*` / `push_*` helpers **for scalars**: append packable elements with `tag_ids_mut().push(…)`; for repeated `string` / `bytes`, use `labels_mut()` / `*_mut()` → [`RepeatedStringMut`](src/repeated.rs) / [`RepeatedBytesMut`](src/repeated.rs) (`push()` then fill the empty element); for repeated messages, `watchers_mut()` → `impl DerefMut<Target = Vec<Address<A>, A>>` (`push(addr)`). Nested singular messages use `set_assignee` / `set_origin` as move-replace. Library users of generated messages depend on `puroro` (+ the generated crate), not on `puroro-rt`.

**Bound-view accessors (`bind` / `bind_mut`).** Every field family — [`SingularField`](puroro-rt/src/fields/singular/field.rs), [`RepeatedField`](puroro-rt/src/fields/repeated/field.rs), and [`OneofSlot`](puroro-rt/src/fields/oneof.rs) — uses the same inherent call shape (`field.bind(&self._common)` / `field.bind_mut(&mut self._common)`). Each returns a short-lived view (`SingularFieldRef` / `SingularFieldMut`, `RepeatedFieldRef` / `RepeatedFieldMut`, `OneofSlotRef` / `OneofSlotMut`) that carries `(field, common)` together. The actual operation (`optional` / `value` / `as_slice` / `get` / `value_mut` / `merge` / `clear` / …) is a consuming method on that view. This keeps the field struct a pure storage holder and collapses each generated accessor to a single call — e.g. `self.owner_id.bind(&self._common).optional()` or `self.priority.bind_mut(&mut self._common).clear()`. Binding happens even when a particular accessor does not consult `common` (e.g. `IMPLICIT` `value()`, repeated `as_slice()`), so read and write share one shape. The presence bit index for EXPLICIT / LEGACY_REQUIRED fields is a **const generic on the field type** (`Explicit<BIT>`), not a runtime argument to `bind` / `bind_mut`. Encode / `deallocate` / `validate_required` stay as plain field methods that take `&common` directly (they are not generated getters). Getter / `_mut` payload types (`Ref` / `Mut`) live on [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs) (`EncodeType::View` / `ValueLayout::Mut`), not on a separate singular-access trait.

Varint and LEN singular fields share one runtime type, [`SingularField`](puroro-rt/src/fields/singular/field.rs), parametrised by allocator-free [`SingularType`](puroro-rt/src/fields/wire/singular_type.rs) markers such as `ProtoInt32` / `ProtoString` / `ProtoBool`, field allocator `A`, and value layout `L` (`Inline` default, [`BitPacked<VALUE_BIT>`](puroro-rt/src/fields/shared/value_layout.rs) for singular/oneof `bool`, or [`InlineOrHeap<SSO_BIT>`](puroro-rt/src/fields/shared/value_layout.rs) for default singular `string` / `bytes` — see [IMPLEMENTATION.md §7](IMPLEMENTATION.md#7-field-wrappers) / [§14](IMPLEMENTATION.md#14-singular-fields) / [§17.2](IMPLEMENTATION.md#172-string--bytes-inline-optimisation)). The field stores `L::Slot` (bare scalars / unmanaged payloads / SSO). **`repeated bool` must not use `BitPacked`** — store plain `bool` elements with no MessageCommon bit index (see [IMPLEMENTATION.md § Bit-packed bool](IMPLEMENTATION.md#bit-packed-bool-protobool)). Nested messages use the same wrapper with `T = ProtoMessage<M>` (sample `origin` / `assignee` / `postal`; repeated / map store `Element = M`). Getters are `Option<&M>` / `&mut M` / `&[M]` / `Vec<M, A>`.

**Decode API:**

```rust
impl<A: Allocator + Clone> Message for Task<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;
    // generated: forwards to puroro_rt::merge_message (ScopedBuf, depth 0)
}

impl<A: Allocator + Clone> MessageMerge for Task<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>;
}
```

`Message` itself does **not** require `A: Default`. The provided `Message::decode` requires `Self: Default` (typically `A: Default` via the inherent `Default` impl). Callers using a non-`Default` allocator must call `new_in(alloc)` followed by `merge_from` instead of `decode`.

**Arena allocator example:**

```rust
let bump = bumpalo::Bump::new();
let mut t = Task::new_in(&bump);
t.title_mut().push_str("Fix bug");
// All allocations (title, labels, nested messages) land in `bump`.
// Here `A = &Bump`, which is `Copy`, so each op just re-uses the borrow; for an
// owned arena handle the message would clone it per op instead.
// When `t` drops, its `Drop` calls `deallocate(&t._common)` on each field.
// For an arena that is effectively a no-op; the memory is reclaimed in bulk when
// `bump` itself is dropped. (For `Global`, deallocate returns the blocks.)
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
| `utf8_validation` | `VERIFY` / `NONE` | `VERIFY`: `ProtoString`; getters `&str` / `StringMut`; invalid UTF-8 → `DecodeError::InvalidUtf8` (not an unknown-field divert). `NONE`: `ProtoStringUnchecked`; getters `&[u8]` / `BytesMut` (proto2 syntax default). Overlay proto2 with plugin option `proto2_utf8=verify`. |

### 6.4 Migration from proto2 / proto3

Editions are a **superset** of both proto2 and proto3: any valid proto2 or proto3 schema can be mechanically translated into an equivalent editions schema (the official Prototiller tool does this). The wire format is unchanged by such a migration.

The generated Rust accessor API after migration is identical for presence, enums, and packing — those follow the field's feature settings, not whether the source was proto2, proto3, or editions. **Exception:** proto2's default `utf8_validation=NONE` uses bytes views for `string`; proto3 / editions default to `VERIFY` (`&str`). After a Prototiller migration the editions file typically gets explicit `VERIFY`, matching proto3. To keep proto2 `.proto` files on `&str` without editing them, pass `proto2_utf8=verify`.

### 6.5 Extensions (not yet implemented)

proto2 `extensions` / `extend` blocks are **out of scope** for the current design. They require a fundamentally different storage mechanism: a message must hold an opaque, field-number-keyed store for extension fields not known to the current compilation unit.

A future design would need:
- An `ExtensionSet<A>` type with typed get/set accessors.
- A registration mechanism for extension descriptors.
- A code-generation strategy for extension accessors separate from the base message.

---

## 7. Design decisions and trade-offs

### No zero-copy decode (for now)

Giving `string` and `bytes` fields a lifetime parameter would enable zero-copy decoding but would propagate that lifetime to every generated type and its callers. The current API is lifetime-free, which significantly simplifies usage. An `Address`-only spike of that idea is kept in [`archive/address-view-spike`](archive/address-view-spike/), outside [`sample-generated`](sample-generated/) and outside the generator.

### No builder / immutable-object pattern

The Java protobuf implementation separates a mutable `Builder` type from an immutable `Message` type, with `build()` as the transition point.  This pattern exists because Java has no ownership semantics: any holder of a reference could mutate a shared object, so immutability must be enforced at the type level.

Rust's ownership system already provides the same guarantees without a dedicated builder type:

- A caller that binds with `let mut` can mutate the message; one that binds with `let` cannot.
- Sharing across threads uses `Arc<Task>` (immutable) or `Arc<Mutex<Task>>` (shared mutation).
- `LEGACY_REQUIRED` field validation is covered by `Message::validate()` without needing a `build()` step.

Generating both `Task<A>` and `TaskBuilder<A>` would double the generated code, complicate the allocator design, and add a `build()` conversion step — all for a guarantee Rust already provides for free through its borrow checker.

### Accessor methods instead of public fields

Public struct fields are simpler but prevent changing internal representations without a breaking API change. Accessor methods decouple the interface from the implementation — for example, presence tracking uses a per-message bitfield internally while exposing `Optional` / `is_set()` unchanged.

### `Optional<T, impl HasDefault<T>>` as a concrete struct

Explicit-presence field accessors return `Optional<T, D>` — a concrete struct rather than a trait or `Option<T>` directly.  Key benefits of this design:

- **Default value as compile-time constant.** `D::DEFAULT` is a `const` expression, so the "return default when not set" branch has zero runtime overhead.
- **No RPIT drop-check restriction.** Because the concrete struct's `Drop` is trivially visible to the borrow checker, `task.title().get()` chains directly without a `let` binding — even for string fields that return `&str`.
- **No `Option<T>` conversion.** `Optional` provides only `get()` and `is_set()` — no `get_opt()` and no `From`/`Into` for `Option<T>`.  Proto explicit-presence fields always carry a meaningful value (explicit or default); converting to `Option<T>` would conflate "not set" with "no value" and make the declared default impossible to enforce.
- **Private default provider.** The `D` type is a zero-sized marker on the field wrapper (`ProtoDefault`, or a message-local ZST for `[default = …]`), hidden behind `impl HasDefault<T>` in the accessor return type. Callers never need to name it.
- **String defaults without unstable features.** `&'static str` cannot currently be a `const` generic parameter, but `const DEFAULT: &'a str` in a blanket `impl<'a> HasDefault<&'a str>` is fully stable on nightly.

### Slice-like return type for repeated fields

Repeated field accessors return a reference to a contiguous sequence rather than a freshly allocated `Vec`. This allows O(1) random access without triggering allocation. The concrete element type is intentionally not part of the stable API.

### Inherent accessors as the current public contract

The generator emits two aliases and their inherent accessors ([§4.0](#40-inherent-accessors-current)): eager `Name` and lazy `NameLazy`. Per-message traits (`foo::Message` / `foo::MessageFallible`, in the companion) are reserved for [§8](#8-future-work) so that eager, lazy, and view layouts can share one generic read API later. Plugin-generated generic code still uses [`Message`](#message) for codec / infrastructure, or is monomorphised over the concrete alias for field access.

### `Default` bound on `Message::decode`

The provided `decode` method requires `Self: Default`. The lower-level `merge_from` has no such requirement, which is important for callers using non-`Default` allocators.

### No compatibility with other Rust protobuf libraries

puroro is a greenfield design. Generated types, trait names, and module layout are not interchangeable with `prost`, `protobuf`, or any other existing crate. Wire-format interoperability with other implementations is achieved only through the shared protobuf encoding spec, not through API compatibility.

### Deprecated group wire types

Groups (`SGroup` / `EGroup`) are not generated and are not stored in unknown fields. If a decoder encounters a group tag on the wire, it may return `DecodeError::InvalidTag`, skip the field, or panic — preserving group payloads for round-trip is explicitly out of scope.

### Module layout: package-first, rename when needed

Protobuf `package` is the primary Rust module hierarchy. Message structs live in the package module (or a parent message's companion). Snake_case submodules are companions for `FIELD_*` / nested types / oneofs / message-local traits (`Message`, `MessageFallible`), and items in a companion omit the message name. Companions are merged when distinct proto identities map to the same path; item-level clashes are left to `rustc`. Users who want a different path use generate-time rename options (not `.proto` options). Cross-references inside the forest use `self::_root::…`. Details: [§4 Module layout and naming](#module-layout-and-naming), [Path qualification](#path-qualification).

### `protoc` plugin as the primary codegen path

The code generator is the `protoc-gen-puroro` crate, invoked as a `protoc` plugin (`protoc --puroro_out=…`). Wrapper scripts, `build.rs` integration, or a standalone binary may exist alongside it, but the plugin interface is the reference integration point. Pipeline and emission progress: [IMPLEMENTATION.md §3](IMPLEMENTATION.md#3-implementation-status).

Each successful run writes two files into the `--puroro_out` directory: `lib.rs` (the forest as that crate's root) and `Cargo.toml` (`[lib] path = "lib.rs"`, with `puroro`, `puroro-rt`, `allocator-api2`, `bitvec`, and `bytes`). Each dependency's version requirement is the version recorded for that crate in the `Cargo.lock` that compiled the plugin. `--puroro_opt=crate_name=<name>` sets the crate name, the `name` value under `[package]`. If omitted, that name is the string `generated`. `protoc` overwrites both files. An application crate depends on that crate and on `puroro`. A protobuf `package library` / `message Book` is `<crate>::library::Book` from outside that crate and `library::Book` inside it. Embedding the `.rs` file in another crate remains valid; that embedder ignores the generated manifest.

### `protobuf-core`, `puroro`, and `puroro-rt`

`protobuf-core` holds reusable wire-format primitives (varint, tags, field readers/writers). **`puroro`** holds the stable message-level API (`Message`, `Optional`, errors) that library users and generated `impl` blocks share. **`puroro-rt`** holds the composable field catalog, wire helpers, and other generator-facing runtime pieces. Generated crates list both `puroro` and `puroro-rt` as dependencies; application code should depend on the generated crate and `puroro` only — not on `puroro-rt` or `protobuf-core` directly. Generators must also keep **`puroro-rt` out of public generated signatures** ([§4](#public-signatures-must-not-surface-puroro-rt)).

### `Bytes` for lazy wire storage vs `A: Allocator` for decoded values

Eager `Task<A>` stores decoded field data in allocator-less `unmanaged` containers (`UnmanagedString`, `UnmanagedVec<i32>`, …) that share the single `MessageCommon.alloc`. `TaskLazy<A>` stores the opaque wire stream in [`SharedWire`](puroro-rt/src/decode/shared_wire.rs) (`bytes::Bytes` plus `A`) so a nested lazy child can share the parent's allocation through a span without copying the LEN payload. Allocator customisation applies to **owned decoded data**; the wire blob prioritises cheap sharing and sub-slicing over placing those bytes in `A`.

---

## 8. Future work

### Per-message traits (not yet generated)

Planned interoperability traits. The lazy alias shipped without them:

- **`foo::Message`** — infallible getters matching the inherent shapes in [§4.0](#40-inherent-accessors-current) (for eager implementations). Defined in the companion (`task::Message` for `message Task`). Not re-exported beside the struct.
- **`foo::MessageFallible`** — `Result`-returning getters for lazy / view layouts (`Error = DecodeError` or `Infallible` on eager). Same placement (`task::MessageFallible`).

These traits are **not** yet emitted by `protoc-gen-puroro`. The hand-written [`sample-generated`](sample-generated/) crate implements [`task::MessageFallible`](sample-generated/src/task/fallible.rs) on eager `Task` (`Error = Infallible`). `TaskLazy` has the same inherent `Result` getters, including oneof and repeated string, and does not implement the trait: a follow-parent field is `Point` on `Task` and `PointLazy` on `TaskLazy`, so one trait cannot name both. Other sample messages do not implement the traits. When the generator adds them, explicit-presence fields should keep `Optional` as the primary read API; convenience `has_*` wrappers (if any) would be trait defaults, not a second required surface on the concrete struct. The lazy alias itself is specified under [lazy parse timing](#tasklaya--lazy-parse-timing).

### Specialized message implementations

Every message has an eager alias and a lazy alias. A borrowing `TaskView<'buf>` is not part of that pair. The measured `Address` spike lives in [`archive/address-view-spike`](archive/address-view-spike/). A shared `task::MessageFallible` impl for the eager and lazy aliases is not emitted yet ([Per-message traits](#per-message-traits-not-yet-generated)).

#### `TaskLazy<A>` — lazy parse timing

Shipped, on the sample and in the plugin. The two aliases are one struct:

```rust
pub type Task<A = Global> = TaskImpl<A, Eager>;
pub type TaskLazy<A = Global> = TaskImpl<A, Lazy<A>>;
```

Eager and lazy are not a runtime enum inside `Task`. Slots that must differ (string / bytes, repeated LEN, maps, follow-parent children) are associated types on the companion `Layout` trait. Numericals, enums, and bools use the same catalog slots on both aliases.

`TaskLazy` is read-oriented. It has no `_mut` or `clear_*`. To mutate, call `into_eager(self) -> Result<Task<A>, DecodeError>` and use the eager API. The implementation lock, including what v1 still does not do, is [`.cursor/plans/lazy-decoder.md`](.cursor/plans/lazy-decoder.md).

**When reads are valid.** `push` accepts one chunk of a single message and applies every complete record. `finish` fails if bytes remain. `merge_from` and `decode` ingest one complete message (`push`, then `finish`). A second `merge_from` is a protobuf merge, not more chunks of the first message. Getters fail until `finish`. They do not return a provisional value.

**What a complete record does.**

| Kind | During the scan | On the getter |
|---|---|---|
| Singular numerical / enum / bool | Catalog `merge_scanned` from `ScannedValue` (last wins) | Already materialised |
| Repeated numerical, packed or expanded | Expanded: `merge_scanned` from `ScannedValue`; packed LEN: `len_span` + `merge_packed_buf` | `&[T]` |
| Singular / oneof string / bytes | Store a span (`WireOrSso`) | Semantic decode. `InvalidUtf8` is `Err` before `Optional` is built. A later get does not re-validate. `has_*` is presence only |
| Nested message | Merge each LEN into the child | `Option<&Child>` |
| Repeated message | One child per occurrence | `&[Child]` |
| Map, repeated string, repeated bytes | Store spans | First get materialises the map or the element list |

Child layout follows [Child parse timing](#child-parse-timing). A follow-parent field is a `Layout` associated type. A pin is a concrete child type on both aliases.

The wire blob is [`SharedWire`](puroro-rt/src/decode/shared_wire.rs): `bytes::Bytes` plus the message's `A`. Each `push` chunk is appended to that island after a successful scan. The scanner does not copy a complete LEN body; it stores a span and callers slice `_wire`. An incomplete record stays in leftover. When a later chunk completes it, leftover is already on the island from the earlier append, so the full record is one contiguous region. A lazy child keeps a handle and a span. It does not copy the LEN body. `Bytes` is the wire store because sub-slicing matters more than placing that blob in `A`. Decoded values still allocate through `A`. Typical generated `TaskLazy` decode is about 1.1× eager after the span scanner and catalog `merge_scanned` (Criterion `generated`, pinned CPU; 1.08–1.14 across runs; gaps under ~10% are noise). Skipping the packed length-prefix rematch does not move that bench — typical Task packed fields are tiny. Map ingest stays a span list, so neither rematch nor packed apply moves `map/decode_lazy`.

**Encode and equality.** Encode of a finished lazy message writes the ingested wire as-is. Field order need not match an eager encode. After more than one `merge_from`, the encoded length is the concatenated ingest. `PartialEq` compares the eager messages when both sides build one. When both sides fail that build (unfinished input, `InvalidUtf8`, and the same class of error), it compares ingested body bytes, so a lazy value stays equal to itself and to a clone that holds the same body. One success and one failure is unequal. `Debug` prints the eager message, or a non-exhaustive stub when that build fails. `unknown_fields()` on the lazy alias is empty. Unknown tags stay in the wire and appear on the eager message after `into_eager`.

#### Eager and lazy semantic gaps

A lazy alias is not a drop-in for the eager alias. The rows below are the behavioural gaps. They are future work. A later change may expose some of them as a generate-time or runtime option; the names in the last column are sketches, not a committed API. The UTF-8 rule stays fixed while that work is open: `VERIFY` fails the decode or the materialise, and the bad LEN is not diverted into unknown fields and is not dropped from a repeated field while its siblings are kept.

| Gap | Eager | Lazy today | Future item |
|---|---|---|---|
| `VERIFY` timing | `decode` / `merge_from` returns `InvalidUtf8` | `decode` succeeds. The getter and `into_eager` return `InvalidUtf8`. `has_*` is presence only and can be true for a payload the getter rejects. Encode writes the bad bytes. | Option sketch `lazy_utf8=on_get` (default) or `on_ingest` (fail `merge_from`, matching eager). On-get keeps the check off the unread path. |
| Repeated / map `VERIFY` | One bad element fails the merge | One bad element fails the whole getter. Earlier valid elements are not returned. | Same knob as the row above. Partial lists stay out of scope. |
| Closed-enum unknown | Divert into the unknown store. The accessor is unset and returns the default. The parse succeeds. | Numericals materialise during the scan, so the accessor matches. The divert also appends the lazy unknown store, and the original tag remains in `_wire`. `unknown_fields()` shows neither. Encode emits the tag once, from `_wire`. | Keep this path for enums only. A future unknown iterator must read `_wire` or the store, not both. |
| `unknown_fields()` | Iterates the store | Always empty. Tags stay in the wire until `into_eager`. | Option sketch: parse `_wire` for `PRESERVE`, stay empty for `DISCARD`. |
| `DISCARD` | Encode omits unknown tags and closed-enum unknowns | Encode writes `_wire`, so those bytes are still emitted. `into_eager` applies the baked discard policy. | Lazy encode should honour the baked store, or wire passthrough stays an explicit mode. |
| `REJECT` | Well-formed unknown tag fails the merge | Plugin scan calls `on_unrecognized_tag` and fails the same way. Handwritten `TaskLazy` uses `Ok(())` for the unknown arm; that sample is preserve-only. | Keep the plugin check. |
| Encode shape | Canonical field order. Merged scalars are emitted once. | Original order. A second `merge_from` concatenates. `PartialEq` can hold while the bytes differ. | Option sketch `lazy_encode=wire` (default) or `canonical`. |
| `validate` | `LEGACY_REQUIRED` presence | Plugin: the same presence check after `ensure_scanned`. It does not UTF-8-check. Sample `TaskLazy::validate` returns `Ok(())`. | Fix the sample. Optional later: `validate` also forces `VERIFY` getters. |
| Recursion limit | Nested merge counts depth and returns `RecursionLimitExceeded` | Ingest stores a child span and does not walk it, so a too-deep message can `decode`. `into_eager` enforces the limit. | Count depth when a lazy child is parsed, or reject at ingest. |
| Getters | Infallible | `Result<_, DecodeError>` | [`MessageFallible`](#per-message-traits-not-yet-generated), already planned. |
| Presence query | No `has_*`. `is_set` on a stored `VERIFY` string means the bytes were valid UTF-8. | `has_*` is wire presence. | Option sketch: `has_*` means present and successfully decoded. |
| Mutation | `_mut` / `clear_*` | None. Mutate through `into_eager`. | Lazy mutation. Encode can no longer be pure `_wire` once a slot diverges from the blob. |
| Follow-parent child | Eager `M` | `MLazy`. A `(puroro.child_parse)` pin is one concrete type on both aliases. | A shared fallible trait has to name both layouts or split. |
| Incomplete input | `decode` fails. There is no live half-parsed value. | `push` without `finish` is a value. Getters return `UnfinishedMessage`. `finish` with leftover is `TruncatedMessage`. | Keep the split. `decode` `Ok` means the wire ended cleanly, not that every `VERIFY` string is valid. |

**Rejected.** An earlier sketch gave each field its own scan of the parent (`Uninitialized` / `WireFound` / `Parsed`, plus a per-field cursor and `tag_ids_all`). Last-wins singular getters would each walk the whole stream, so v1 does not do that. There is no public "is this field loaded?" API, no lazy `_mut`, and no getter before `finish`.

`TaskLazy` does not implement `task::MessageFallible`. Follow-parent fields change Rust type with the parent (`Point` versus `PointLazy`). It also does not implement an infallible per-message `task::Message`. Codec goes through [`Message`](#message), whose lazy `merge_from` is the ingest above and whose lazy `unknown_fields` is empty.

#### `TaskView<'buf>` — zero-copy, buffer-referencing

Archived, not generator output, and not in [`sample-generated`](sample-generated/). The handwritten spike covers `Address` only (two strings, `fixed32`, `double`): one scan, last-wins offsets, UTF-8 on each get. Criterion numbers are in that crate. See [`archive/address-view-spike`](archive/address-view-spike/).

- Holds `&'buf [u8]` to the original input; no heap allocation for field data.
- String / bytes fields return subslices into the input buffer.
- Read-only; tied to buffer lifetime.
- Fully lazy (scan on each access) or semi-eager (field-offset index built once).
- Implements `task::MessageFallible` with `Error = DecodeError`.
- Repeated fields support early termination via fallible iterators.

**Native methods beyond the trait:** `title() -> Result<&'buf str, …>` (buffer lifetime, not `&self`), `to_owned(alloc) -> Task<A>`, `tag_ids_raw() -> Result<&'buf [u8], …>`.

#### Relationship between implementations

```
task::Message (infallible trait, not emitted)
    ↑ impl
    Task<A>        (planned)

task::MessageFallible (Result-returning trait, not emitted)
    ↑ impl
    Task<A>        (handwritten sample only, Error = Infallible)
    TaskLazy<A>    (not implemented; inherent Result getters)
    TaskView<'buf> (archived Address spike; not emitted)
```

Generic code that only reads fields is meant to use `task::MessageFallible` with `Task<A>` or `TaskLazy<A>`. Today only eager `Task` implements that trait. `TaskLazy` is a separate alias with its own inherent `Result` getters.

---

### Other future work

- **Unknown-field preservation opt-out.** `(puroro.unknown_fields) = DISCARD` bakes [`DiscardUnknowns`](puroro-rt/src/unknown_fields.rs) on the eager alias. File-level / package default is not wired. The lazy alias's `unknown_fields()` iterator is empty either way; unknown tags stay in the ingested wire until `into_eager`. The rest of the eager / lazy split, including this one, is listed under [eager / lazy semantic gaps](#eager-and-lazy-semantic-gaps).
- **Unknown-field reject-on-decode.** Shipped as `(puroro.unknown_fields) = REJECT` ([§4.9](#49-unknown-fields)). This failure is outside the protobuf spec. File-level / package default is not wired, same as Discard.
- **Service / RPC definitions.** Out of scope for the runtime library.
- **Well-known types.** `google.protobuf.Timestamp`, `Duration`, `Any`, etc.
- **Reflection / descriptors.** Runtime introspection of message schema.
- **Extensions.** See §6.5.
- **Repeated / map string and bytes SSO.** Singular string / bytes SSO and singular message inline-or-box are shipped. Repeated and map payloads stay on the heap — see [IMPLEMENTATION.md §17.2](IMPLEMENTATION.md#172-string--bytes-inline-optimisation).
