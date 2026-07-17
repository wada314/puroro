//! Composable field types for generated protobuf messages.
//!
//! Each proto field (except oneof variants) maps to a self-contained runtime
//! type in this module. Field getters/setters/encode/decode only touch:
//!
//! 1. The field's own struct member, and
//! 2. [`MessageCommon`](shared::MessageCommon) (presence bitfield, allocator, unknown-field buffer).
//!
//! Singular fields (non-repeated — both `IMPLICIT` and `EXPLICIT` presence) are
//! parametrised by **wire type** (`ProtoInt32`, `ProtoString`, … — thin wrappers
//! for singular fields; repeated fields use the inner `Value` / `Storage`)
//! and **presence policy** ([`Implicit`](shared::field_presence::Implicit) / [`Explicit`](shared::field_presence::Explicit) / [`Oneof`](shared::field_presence::Oneof)).
//!
//! Flat crate-root re-exports of the catalog live in [`crate`](crate) (`lib.rs`).
//!
//! # Module layout
//!
//! | Submodule | Contents |
//! |---|---|
//! | [`shared`] | `MessageCommon`, `PresenceBits`, `FieldPresence`, `ValueSlot`, … |
//! | [`wire`] | `ProtoType` (thin wrappers), `VarintProtoType` / `LenProtoType` (repeated inners), fixed-width stubs |
//! | [`singular`] | `SingularField`, `NestedMessageField` |
//! | [`repeated`] | `RepeatedVarintField`, `RepeatedLenField` |
//! | [`oneof`] | `OneofSlot` |

pub(crate) mod enum_variant;
pub(crate) mod oneof;
pub(crate) mod repeated;
pub(crate) mod shared;
pub(crate) mod singular;
pub(crate) mod wire;
