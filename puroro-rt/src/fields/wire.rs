//! Wire-encoding types (`VarintProtoType`, `LenProtoType`, `ProtoType`, …).
//!
//! Singular fields use thin type markers (`ProtoInt32(i32)`, …). Repeated fields
//! use the inner `Value` / `Storage` associated types.

pub(crate) mod fixed;
pub(crate) mod len;
pub(crate) mod proto_message;
pub(crate) mod proto_type;
pub(crate) mod varint;

pub use len::{LenProtoType, ProtoBytes, ProtoString};
pub use proto_message::ProtoMessage;
pub use proto_type::ProtoType;
pub use varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32,
    ProtoInt64, ProtoSint32, ProtoSint64, ProtoUInt32, ProtoUInt64, VarintProtoType,
};
