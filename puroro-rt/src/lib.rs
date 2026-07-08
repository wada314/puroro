//! Runtime implementation details for **puroro-generated** protobuf code.
//!
//! Library users should depend on [`puroro`] only. This crate exists so the
//! field catalog, wire helpers, and other generator-facing types can evolve on
//! a separate semver track from the stable user API.

pub mod decode;
pub mod defaults;
pub mod encode;
pub mod fields;

pub use defaults::ProtoDefault;
pub use fields::{
    Explicit, ExplicitBytes, ExplicitEnum, ExplicitInt32, ExplicitLenField, ExplicitString,
    ExplicitVarint, ExplicitVarintField,     FieldNumber, FieldPresence, Implicit, ImplicitBytes,
    Bindable, BindableMut, EnumVariant,
    ImplicitEnum, ImplicitInt32, ImplicitLenField, ImplicitString, ImplicitVarint,
    ImplicitVarintField, LegacyRequired, LenProtoType, MessageCommon, MessagePresence,
    NestedMessage, NestedMessageField, NestedMessageFieldMut, Oneof, OneofDeallocate,
    OneofEncodable, OneofSlot, OneofSlotMut, OneofVariantRef, Expanded, Packed, PresenceBits,
    ProtoBool, ProtoBytes, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64, ProtoSint32,
    ProtoSint64, ProtoString, ProtoUInt32, ProtoUInt64, RepeatedBytes, RepeatedExpandedInt32,
    RepeatedExpandedVarintField, RepeatedLen, RepeatedLenField, RepeatedLenFieldMut,
    RepeatedPackedInt32, RepeatedPackedVarintField, RepeatedString, RepeatedVarint,
    RepeatedVarintEncoding, RepeatedVarintField, RepeatedVarintFieldMut, RequiredFieldPresence,
    Singular, SingularLen, SingularLenField, SingularLenFieldMut, SingularVarint,
    SingularVarintField, SingularVarintFieldMut, ValueSlot, VarintProtoType,
};
