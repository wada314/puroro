//! Runtime implementation details for **puroro-generated** protobuf code.
//!
//! Library users should depend on [`puroro`] only. This crate exists so the
//! field catalog, wire helpers, and other generator-facing types can evolve on
//! a separate semver track from the stable user API.

pub mod decode;
pub(crate) mod defaults;
pub mod encode;
pub(crate) mod fields;

pub use ::protobuf_core::FieldNumber;
pub use defaults::ProtoDefault;
pub use fields::enum_variant::EnumVariant;
pub use fields::oneof::{
    OneofDeallocate, OneofEncodable, OneofGroup, OneofSlot, OneofSlotMut, OneofSlotRef,
    OneofVariantRef, OneofView, OneofViewMut,
};
pub use fields::repeated::{
    Expanded, Packed, RepeatedBytes, RepeatedExpandedInt32, RepeatedExpandedVarintField,
    RepeatedLen, RepeatedLenField, RepeatedLenFieldMut, RepeatedLenFieldRef, RepeatedPackedInt32,
    RepeatedPackedVarintField, RepeatedString, RepeatedVarint, RepeatedVarintEncoding,
    RepeatedVarintField, RepeatedVarintFieldMut, RepeatedVarintFieldRef,
};
pub use fields::shared::{FieldDeallocate, MessageCommon, PresenceBits};
pub use fields::shared::field_presence::{
    Explicit, FieldPresence, Implicit, LegacyRequired, Oneof, RequiredFieldPresence,
};
pub use fields::shared::value_slot::ValueSlot;
pub use fields::singular::{
    BoolField, BoolFieldMut, BoolFieldRef, ExplicitBoolField, ExplicitBytes, ExplicitEnum,
    ExplicitInt32, ExplicitLenField, ExplicitString, ExplicitVarint, ExplicitVarintField,
    ImplicitBoolField, ImplicitBytes, ImplicitEnum, ImplicitInt32, ImplicitLenField, ImplicitString,
    ImplicitVarint, ImplicitVarintField, LegacyRequiredBoolField, MessagePresence, NestedMessage,
    NestedMessageField, NestedMessageFieldMut, NestedMessageFieldRef, OneofBoolField, Singular,
    SingularAccess, SingularField, SingularFieldMut, SingularFieldRef, SingularLen,
    SingularLenField, SingularLenFieldMut, SingularLenFieldRef, SingularVarint, SingularVarintField,
    SingularVarintFieldMut, SingularVarintFieldRef,
};
pub use fields::wire::{
    LenProtoType, ProtoBool, ProtoBytes, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64,
    ProtoSint32, ProtoSint64, ProtoString, ProtoUInt32, ProtoUInt64, ScalarProtoType,
    VarintProtoType,
};
