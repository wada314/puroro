//! Composable field types for generated protobuf messages.
//!
//! Each proto field (except oneof variants) maps to a self-contained runtime
//! type in this module. Field getters/setters/encode/decode only touch:
//!
//! 1. The field's own struct member, and
//! 2. [`MessageCommon`](shared::MessageCommon) (presence bitfield, allocator, unknown-field buffer).
//!
//! Singular fields (non-repeated — both `IMPLICIT` and `EXPLICIT` presence) are
//! parametrised by **wire type** (`ProtoInt32`, `ProtoString`, …) and
//! **presence policy** ([`Implicit`](shared::field_presence::Implicit) /
//! [`Explicit`](shared::field_presence::Explicit) /
//! [`Oneof`](shared::field_presence::Oneof)). Repeated fields use the same
//! markers via [`RepeatedElement`](wire::RepeatedElement) (`Element` storage).
//!
//! Flat crate-root re-exports of the catalog live in [`crate`](crate) (`lib.rs`).
//!
//! # Module layout
//!
//! | Submodule | Contents |
//! |---|---|
//! | [`shared`] | `MessageCommon`, `MessageCommonBits`, `MessageCommonAlloc`, `FieldPresence`, `ValueSlot`, … |
//! | [`wire`] | `SingularType`, `RepeatedElement`, `NumericalType`, fixed / varint markers |
//! | [`singular`] | `SingularField` |
//! | [`repeated`] | `RepeatedField` |
//! | [`map`] | `MapField` (hash map entries) |
//! | [`oneof`] | `OneofSlot` |
//! | [`oneof_variant`] | `OneofVariant` (field-number dispatch) |

pub(crate) mod map;
pub(crate) mod oneof;
pub(crate) mod oneof_variant;
pub(crate) mod repeated;
pub(crate) mod shared;
pub(crate) mod singular;
pub(crate) mod wire;
