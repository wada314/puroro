//! Wire-encoding types (`VarintProtoType`, `LenProtoType`, `ScalarProtoType`, …).
//!
//! Singular scalars are thin wrappers (`ProtoInt32(i32)`, …). Repeated fields
//! use the inner `Value` / `Storage` associated types.

pub mod fixed;
pub mod len;
pub mod scalar;
pub mod varint;

pub use fixed::{Fixed32ProtoType, Fixed64ProtoType};
pub use len::{LenProtoType, ProtoBytes, ProtoString};
pub use scalar::ScalarProtoType;
pub use varint::{
    ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64,
    ProtoUInt32, ProtoUInt64, VarintProtoType, WIRE_TYPE as VARINT_WIRE_TYPE, enum_value_is_known,
};
