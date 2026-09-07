//! Runtime implementation details for **puroro-generated** protobuf code.
//!
//! Library users should depend on [`puroro`] only. This crate exists so the
//! field catalog, wire helpers, and other generator-facing types can evolve on
//! a separate semver track from the stable user API.
//!
//! Crate-root re-exports are:
//! - symbols the generator names as `::puroro_rt::…`, and
//! - bound-view types (`*FieldRef` / `*FieldMut`, …) that appear in public
//!   method signatures of those wrappers (nameable even when call sites only
//!   chain methods).
//!
//! Wire-shape payloads (`WirePayload`, …) are not re-exported; see
//! `fields::wire::wire_payload`.

pub mod decode;
pub(crate) mod defaults;
pub mod encode;
pub(crate) mod fields;
pub mod message_encode;
pub mod message_merge;
pub(crate) mod unknown_fields;

pub use ::protobuf_core::{Field, FieldNumber, Varint};
/// Generator-facing re-exports from `unmanaged`.
pub use ::unmanaged::{CloneIn, DeallocateIn, DefaultIn};
pub use defaults::ProtoDefault;
pub use encode::field_number_const;
pub use fields::map::{LazyMapField, MapField, MapFieldMut, MapFieldRef};
pub use fields::oneof::{
    OneofDeallocate, OneofEncodable, OneofGroup, OneofSlot, OneofSlotMut, OneofSlotRef,
    OneofVariantRef, OneofView, OneofViewMut,
};
pub use fields::oneof_variant::OneofVariant;
pub use fields::repeated::{
    Expanded, LazyRepeatedField, Packed, RepeatedField, RepeatedFieldMut, RepeatedFieldRef,
};
pub use fields::shared::field_presence::{Explicit, Implicit, LegacyRequired, Message, Oneof};
pub use fields::shared::{
    AddressableSlot, BitPacked, Boxed, CloneBound, CloneFieldsVisitor, DeallocateBound,
    DebugStructVisitor, DiscardUnknowns, EncodeRawVisitor, EncodedLenVisitor, FieldCloneIn,
    FieldDeallocVisitor, FieldDeallocate, FieldEncode, FieldEqVisitor, FieldPairVisitor,
    FieldPairVisitorMut, FieldVisitor, FieldVisitorMut, Inline, InlineOrHeap, InteriorBitArray,
    LazyMessageCommon, MessageBinding, MessageBindingMut, MessageCommon, MessageCommonAlloc,
    MessageCommonBits, MessageCommonSharedBits, SSO_HEAP, SSO_INLINE, UnknownFields, UnknownStore,
    ValueLayout, WireOrSso,
};
pub use fields::singular::{
    SingularField, SingularFieldAccess, SingularFieldMut, SingularFieldRef,
};
pub use fields::wire::{
    Closed, ClosedEnum, INLINE_CAP, LazyBytesSlot, LazyStringSlot, Open, OpenEnum, PayloadAccess,
    ProtoBool, ProtoBytes, ProtoDouble, ProtoEnum, ProtoEnumStorage, ProtoFixed32, ProtoFixed64,
    ProtoFloat, ProtoInt32, ProtoInt64, ProtoMessage, ProtoSFixed32, ProtoSFixed64, ProtoSInt32,
    ProtoSInt64, ProtoString, ProtoStringUnchecked, ProtoUInt32, ProtoUInt64, SingularType,
    SsoBytes, SsoBytesMut, SsoString, SsoStringMut, WireOrSsoKind, WireOrSsoSlot,
};
pub use message_encode::{EncodeCtx, MessageEncode, encode_message, encode_message_to_vec};
pub use message_merge::{MessageMerge, merge_message};
