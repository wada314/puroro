//! Composable field types for generated protobuf messages.
//!
//! Each proto field (except oneof variants) maps to a self-contained runtime
//! type in this module. Field getters/setters/encode/decode only touch:
//!
//! 1. The field's own struct member, and
//! 2. [`MessageCommon`] (presence bitfield, allocator, unknown-field buffer).
//!
//! Singular fields are parametrised by **wire type** (`ProtoInt32`, `ProtoString`, …)
//! and **presence policy** ([`Implicit`] / [`Explicit`]).
//!
//! # Module layout
//!
//! | Submodule / type | Proto pattern |
//! |---|---|
//! | [`varint`] | `VarintProtoType` markers |
//! | [`field_presence`] | `FieldPresence` (`Implicit` / `Explicit`) |
//! | [`scalar`] | `SingularVarintField<T, P, FIELD>` |
//! | [`len`] / [`len_field`] | `SingularLenField<T, P, FIELD, A>` |
//! | [`message`] | `NestedMessageField` |
//! | [`repeated_varint`] / [`repeated_len`] | `RepeatedVarintField`, `RepeatedLenField` |
//! | [`oneof::OneofSlot`] | `oneof` group |
//! | [`common::MessageCommon`] | shared infrastructure |

pub mod common;
pub mod field_presence;
pub mod fixed32;
pub mod fixed64;
pub mod len;
pub mod len_field;
pub mod message;
pub mod oneof;
pub mod presence;
pub mod repeated_encoding;
pub mod repeated_len;
pub mod repeated_varint;
pub mod scalar;
pub mod varint;

pub use common::MessageCommon;
pub use field_presence::{
    Explicit, ExplicitFieldPresence, FieldPresence, Implicit, LegacyRequired, RequiredFieldPresence,
};
pub use len::{LenProtoType, ProtoBytes, ProtoString};
pub use len_field::{
    ExplicitBytes, ExplicitLenField, ExplicitString, ImplicitBytes, ImplicitLenField,
    ImplicitString, SingularLen, SingularLenField, SingularLenFieldMut,
};
pub use message::{
    MessagePresence, NestedMessage, NestedMessageField, NestedMessageFieldMut, Present,
};
pub use oneof::{OneofDeallocate, OneofSlot, OneofSlotMut};
pub use presence::PresenceBits;
pub use repeated_encoding::{Expanded, Packed, RepeatedVarintEncoding};
pub use repeated_len::{
    RepeatedBytes, RepeatedLen, RepeatedLenField, RepeatedLenFieldMut, RepeatedString,
};
pub use repeated_varint::{
    RepeatedExpandedInt32, RepeatedExpandedVarintField, RepeatedPackedInt32,
    RepeatedPackedVarintField, RepeatedVarint, RepeatedVarintField, RepeatedVarintFieldMut,
};
pub use scalar::{
    ExplicitEnum, ExplicitInt32, ExplicitVarint, ExplicitVarintField, ImplicitEnum, ImplicitInt32,
    ImplicitVarint, ImplicitVarintField, SingularVarint, SingularVarintField,
    SingularVarintFieldMut,
};
pub use varint::{
    ProtoBool, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64, ProtoUInt32, ProtoUInt64,
    VarintProtoType,
};
