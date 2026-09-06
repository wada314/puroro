//! Wire-encoding types (`Numerical` / `LenScalar`, `SingularType`, …).
//!
//! Singular fields use allocator-free type markers (`ProtoInt32`, …) and store
//! [`ValueLayout::Slot`](crate::fields::shared::value_layout::ValueLayout). Repeated fields store [`RepeatedElement::Element`]
//! (slice / storage) and project [`RepeatedElement::RefView`] for shared reads.
//! Map fields use [`MapKey`](crate::fields::map::MapKey) for keys and
//! [`RepeatedElement`] for values (both share `RefView`).
//!
//! Concrete `Proto*` aliases and codecs live in [`numerical`] (VARINT / I32 /
//! I64 / enums) and [`len`] (string / bytes).

pub(crate) mod encode_type;
pub(crate) mod len;
pub(crate) mod numerical;
pub(crate) mod proto_message;
pub(crate) mod proto_ref_ops;
pub(crate) mod repeated_element;
pub(crate) mod singular_type;
pub(crate) mod sso_buf;
pub(crate) mod sso_bytes;
pub(crate) mod sso_string;
pub(crate) mod wire_or_sso;
pub(crate) mod wire_payload;

pub use len::{ProtoBytes, ProtoString, ProtoStringUnchecked};
pub use numerical::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoDouble, ProtoEnum, ProtoEnumStorage,
    ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoInt32, ProtoInt64, ProtoSFixed32, ProtoSFixed64,
    ProtoSInt32, ProtoSInt64, ProtoUInt32, ProtoUInt64,
};
pub use proto_message::ProtoMessage;
pub use repeated_element::RepeatedElement;
pub use singular_type::{PayloadAccess, SingularType};
pub use sso_bytes::{SsoBytes, SsoBytesMut};
pub use sso_string::{INLINE_CAP, SsoString, SsoStringMut};
pub use wire_or_sso::{LazyBytesSlot, LazyStringSlot, WireOrSsoKind, WireOrSsoSlot};
