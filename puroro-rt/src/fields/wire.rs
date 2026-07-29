//! Wire-encoding types (`NumericalType`, `SingularType`, …).
//!
//! Singular fields use allocator-free type markers (`ProtoInt32`, …) and store
//! [`SingularType::Slot`]. Repeated fields store [`RepeatedElement::Element`].
//! Map fields use [`MapKey`] for keys and [`RepeatedElement`] for values.

pub(crate) mod encode_type;
pub(crate) mod fixed;
pub(crate) mod len;
pub(crate) mod map_element;
pub(crate) mod numerical;
pub(crate) mod proto_message;
pub(crate) mod proto_ref_ops;
pub(crate) mod repeated_element;
pub(crate) mod singular_type;
pub(crate) mod varint;
pub(crate) mod wire_payload;

pub use encode_type::{EncodeType, encode_field, encoded_len_field};
pub use fixed::{
    ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoSFixed32, ProtoSFixed64,
};
pub use len::{ProtoBytes, ProtoString};
pub use map_element::{MapKey, MapValueView};
pub use numerical::NumericalType;
pub use proto_message::ProtoMessage;
pub use repeated_element::{
    PackableRepeatedElement, RepeatedElement, RepeatedElementMerge, RepeatedElementMut,
    RepeatedVecMut,
};
pub use singular_type::{PayloadAccess, SingularType};
pub use varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32,
    ProtoInt64, ProtoSInt32, ProtoSInt64, ProtoUInt32, ProtoUInt64,
};
pub use wire_payload::{
    CopyWirePayload, Fixed32Payload, Fixed64Payload, LenPayload, LenPayloadRef, MessageLenRef,
    VarintPayload, WirePayload,
};
