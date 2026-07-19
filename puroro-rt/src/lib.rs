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
pub use fields::map::{MapEntries, MapEntriesIter, MapField, MapFieldMut, MapFieldRef};
pub use fields::oneof::{
    OneofDeallocate, OneofEncodable, OneofGroup, OneofSlot, OneofSlotMut, OneofSlotRef,
    OneofVariantRef, OneofView, OneofViewMut,
};
pub use fields::repeated::{
    Expanded, Packed, RepeatedContainerMut, RepeatedElementsMut, RepeatedEncoding, RepeatedField,
    RepeatedFieldMut, RepeatedFieldRef,
};
pub use fields::shared::field_presence::{
    Explicit, FieldPresence, Implicit, LegacyRequired, NonOneof, Oneof, RequiredFieldPresence,
};
pub use fields::shared::value_slot::ValueSlot;
pub use fields::shared::{
    BitPacked, CatalogField, CloneFieldsVisitor, DebugStructVisitor, EncodeRawVisitor,
    EncodedLenVisitor, FieldCloneIn, FieldDeallocVisitor, FieldDeallocate, FieldDebug, FieldEncode,
    FieldEqVisitor, FieldPairVisitor, FieldPairVisitorMut, FieldPartialEq, FieldVisitor,
    FieldVisitorMut, Inline, MessageCommon, PresenceBits, ValueLayout,
};
pub use fields::singular::{SingularField, SingularFieldMut, SingularFieldRef};
pub use fields::wire::{
    Closed, ClosedEnum, Fixed32ProtoType, Fixed64ProtoType, MapKey, Open, OpenEnum,
    PackableRepeatedElement, PayloadAccess, ProtoBool, ProtoBytes, ProtoDouble, ProtoEnum,
    ProtoEnumStorage, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoInt32, ProtoInt64, ProtoMessage,
    ProtoSFixed32, ProtoSFixed64, ProtoSint32, ProtoSint64, ProtoString, ProtoType, ProtoUInt32,
    ProtoUInt64, RepeatedElement, RepeatedElementMerge, RepeatedElementMut, RepeatedSlicePush,
    RepeatedVecMut, VarintProtoType,
};
