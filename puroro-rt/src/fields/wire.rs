//! Wire-encoding types (`VarintProtoType`, `ProtoType`, …).
//!
//! Singular fields use allocator-free type markers (`ProtoInt32`, …) and store
//! [`ProtoType::Slot`]. Repeated fields store [`RepeatedElement::Element`].
//! Map fields use [`MapKey`] for keys and [`RepeatedElement`] for values.

pub(crate) mod fixed;
pub(crate) mod len;
pub(crate) mod map_element;
pub(crate) mod proto_message;
pub(crate) mod proto_ref_ops;
pub(crate) mod proto_type;
pub(crate) mod repeated_element;
pub(crate) mod varint;

pub use fixed::{
    Fixed32ProtoType, Fixed64ProtoType, ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat,
    ProtoSFixed32, ProtoSFixed64,
};
pub use len::{ProtoBytes, ProtoString};
pub use map_element::{MapKey, MapValueView};
pub use proto_message::ProtoMessage;
pub use proto_type::{PayloadAccess, ProtoType};
pub use repeated_element::{
    PackableRepeatedElement, RepeatedElement, RepeatedElementMerge, RepeatedElementMut,
    RepeatedSlicePush, RepeatedVecMut,
};
pub use varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32,
    ProtoInt64, ProtoSint32, ProtoSint64, ProtoUInt32, ProtoUInt64, VarintProtoType,
};
