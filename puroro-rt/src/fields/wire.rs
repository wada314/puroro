//! Wire-encoding types (`NumericalType`, `SingularType`, …).
//!
//! Singular fields use allocator-free type markers (`ProtoInt32`, …) and store
//! [`SingularType::Slot`]. Repeated fields store [`RepeatedElement::Element`]
//! (slice / storage) and project [`RepeatedElement::RefView`] for shared reads.
//! Map fields use [`MapKey`](crate::fields::map::MapKey) for keys and
//! [`RepeatedElement`] for values (both share `RefView`).

pub(crate) mod encode_type;
pub(crate) mod fixed;
pub(crate) mod len;
pub(crate) mod numerical;
pub(crate) mod proto_message;
pub(crate) mod proto_ref_ops;
pub(crate) mod repeated_element;
pub(crate) mod singular_type;
pub(crate) mod varint;
pub(crate) mod wire_payload;

pub use fixed::{
    ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoSFixed32, ProtoSFixed64,
};
pub use len::{ProtoBytes, ProtoString};
pub use proto_message::ProtoMessage;
pub use repeated_element::RepeatedElement;
pub use singular_type::SingularType;
pub use varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32,
    ProtoInt64, ProtoSInt32, ProtoSInt64, ProtoUInt32, ProtoUInt64,
};
