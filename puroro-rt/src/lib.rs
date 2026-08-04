//! Runtime implementation details for **puroro-generated** protobuf code.
//!
//! Library users should depend on [`puroro`] only. This crate exists so the
//! field catalog, wire helpers, and other generator-facing types can evolve on
//! a separate semver track from the stable user API.
//!
//! Crate-root re-exports are the generator-facing surface (markers, field
//! wrappers, presence, visitors). Wire-shape payloads (`WirePayload`, …) are
//! not re-exported; see `fields::wire::wire_payload`.

pub mod decode;
pub(crate) mod defaults;
pub mod encode;
pub(crate) mod fields;
pub mod message_encode;
pub mod message_merge;

pub use ::protobuf_core::FieldNumber;
/// Generator-facing re-exports from `unmanaged`.
///
/// Library users of generated messages should prefer [`puroro::String`](::puroro::String)
/// (and other `puroro` traits) in public signatures; these remain for catalog code.
pub use ::unmanaged::{CloneIn, DeallocateIn, String, ToOwnedIn, UnmanagedString};
pub use defaults::ProtoDefault;
pub use fields::map::{MapField, MapFieldMut, MapFieldRef, MapKey};
pub use fields::oneof::{
    OneofDeallocate, OneofEncodable, OneofGroup, OneofSlot, OneofSlotMut, OneofSlotRef,
    OneofVariantRef, OneofView, OneofViewMut,
};
pub use fields::oneof_variant::OneofVariant;
pub use fields::repeated::{
    Expanded, Packed, RepeatedBytesMut, RepeatedContainerMut, RepeatedElementsMut,
    RepeatedEncoding, RepeatedField, RepeatedFieldMut, RepeatedFieldRef, RepeatedStringMut,
};
pub use fields::shared::field_presence::{
    Explicit, FieldPresence, Implicit, LegacyRequired, Message, Oneof, RequiredFieldPresence,
};
pub use fields::shared::value_slot::ValueSlot;
pub use fields::shared::{
    BitPacked, CatalogField, CloneFieldsVisitor, DebugStructVisitor, EncodeRawVisitor,
    EncodedLenVisitor, FieldCloneIn, FieldDeallocVisitor, FieldDeallocate, FieldDebug, FieldEncode,
    FieldEqVisitor, FieldPairVisitor, FieldPairVisitorMut, FieldPartialEq, FieldVisitor,
    FieldVisitorMut, Inline, MessageCommon, MessageCommonAlloc, MessageCommonBits, ValueLayout,
};
pub use fields::singular::{SingularField, SingularFieldMut, SingularFieldRef};
pub use fields::wire::{
    Closed, ClosedEnum, EncodeType, NumericalType, Open, OpenEnum, PackableRepeatedElement,
    PayloadAccess, ProtoBool, ProtoBytes, ProtoDouble, ProtoEnum, ProtoEnumStorage, ProtoFixed32,
    ProtoFixed64, ProtoFloat, ProtoInt32, ProtoInt64, ProtoMessage, ProtoSFixed32, ProtoSFixed64,
    ProtoSInt32, ProtoSInt64, ProtoString, ProtoUInt32, ProtoUInt64, RepeatedElement,
    RepeatedElementMerge, RepeatedElementMut, RepeatedVecMut, SingularType, encode_field,
    encoded_len_field,
};
pub use message_encode::{EncodeCtx, MessageEncode, encode_message, encode_message_to_vec};
pub use message_merge::{MessageMerge, merge_message};
