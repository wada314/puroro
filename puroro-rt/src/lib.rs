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
    Bindable, BindableMut, BoolField, BoolFieldMut, BoolFieldRef, EnumVariant, Expanded, Explicit,
    ExplicitBoolField, ExplicitBytes, ExplicitEnum, ExplicitInt32, ExplicitLenField, ExplicitString,
    ExplicitVarint, ExplicitVarintField, FieldDeallocate, FieldNumber, FieldPresence, Implicit,
    ImplicitBoolField, ImplicitBytes, ImplicitEnum, ImplicitInt32, ImplicitLenField, ImplicitString,
    ImplicitVarint, ImplicitVarintField, LegacyRequired, LegacyRequiredBoolField, LenProtoType,
    MessageCommon, MessagePresence, NestedMessage, NestedMessageField, NestedMessageFieldMut,
    NestedMessageFieldRef, Oneof, OneofBoolField, OneofDeallocate, OneofEncodable, OneofSlot,
    OneofSlotMut, OneofSlotRef, OneofVariantRef, Packed, PresenceBits, ProtoBool, ProtoBytes,
    ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64, ProtoString,
    ProtoUInt32, ProtoUInt64, RepeatedBytes, RepeatedExpandedInt32, RepeatedExpandedVarintField,
    RepeatedLen, RepeatedLenField, RepeatedLenFieldMut, RepeatedLenFieldRef, RepeatedPackedInt32,
    RepeatedPackedVarintField, RepeatedString, RepeatedVarint, RepeatedVarintEncoding,
    RepeatedVarintField, RepeatedVarintFieldMut, RepeatedVarintFieldRef, RequiredFieldPresence,
    ScalarProtoType, Singular, SingularField, SingularFieldMut, SingularFieldRef, SingularLen,
    SingularLenField, SingularLenFieldMut, SingularLenFieldRef, SingularVarint, SingularVarintField,
    SingularVarintFieldMut, SingularVarintFieldRef, ValueSlot, VarintProtoType,
};
