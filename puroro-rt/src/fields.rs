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
//! # Module layout
//!
//! | Submodule | Contents |
//! |---|---|
//! | [`shared`] | `MessageCommon`, `PresenceBits`, `FieldPresence`, `ValueSlot`, … |
//! | [`wire`] | `ScalarProtoType` (thin wrappers), `VarintProtoType` / `LenProtoType` (repeated inners), fixed-width stubs |
//! | [`singular`] | `SingularField`, `NestedMessageField` |
//! | [`repeated`] | `RepeatedVarintField`, `RepeatedLenField` |
//! | [`oneof`] | `OneofSlot` |

pub mod enum_variant;
pub mod oneof;
pub mod repeated;
pub mod shared;
pub mod singular;
pub mod wire;

pub use ::protobuf_core::FieldNumber;
pub use enum_variant::EnumVariant;
pub use oneof::{
    OneofDeallocate, OneofEncodable, OneofSlot, OneofSlotMut, OneofSlotRef, OneofVariantRef,
};
pub use repeated::{
    Expanded, Packed, RepeatedBytes, RepeatedExpandedInt32, RepeatedExpandedVarintField,
    RepeatedLen, RepeatedLenField, RepeatedLenFieldMut, RepeatedLenFieldRef, RepeatedPackedInt32,
    RepeatedPackedVarintField, RepeatedString, RepeatedVarint, RepeatedVarintEncoding,
    RepeatedVarintField, RepeatedVarintFieldMut, RepeatedVarintFieldRef,
};
pub use shared::{
    DeallocateIn, DefaultIn, MessageCommon, PresenceBits, ProtoEmpty,
    bindable::{Bindable, BindableMut},
    field_presence::{Explicit, FieldPresence, Implicit, LegacyRequired, RequiredFieldPresence},
    slot_init::{AlwaysInitialized, BitInitMut, BitInitView, SlotInitMut, SlotInitView},
    value_slot::ValueSlot,
};
pub use singular::{
    ExplicitBytes, ExplicitEnum, ExplicitInt32, ExplicitLenField, ExplicitString, ExplicitVarint,
    ExplicitVarintField, ImplicitBytes, ImplicitEnum, ImplicitInt32, ImplicitLenField,
    ImplicitString, ImplicitVarint, ImplicitVarintField, MessagePresence, NestedMessage,
    NestedMessageField, NestedMessageFieldMut, NestedMessageFieldRef, Singular, SingularField,
    SingularFieldMut, SingularFieldRef, SingularLen, SingularLenField, SingularLenFieldMut,
    SingularLenFieldRef, SingularVarint, SingularVarintField, SingularVarintFieldMut,
    SingularVarintFieldRef,
};
pub use wire::{
    LenProtoType, ProtoBool, ProtoBytes, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64,
    ProtoSint32, ProtoSint64, ProtoString, ProtoUInt32, ProtoUInt64, ScalarProtoType,
    VarintProtoType, enum_value_is_known,
};

// Preserve the historical `Oneof` name for the field-presence marker (distinct from `oneof` module).
pub use shared::field_presence::Oneof;
