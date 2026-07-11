//! Wire-encoding types (`VarintProtoType`, `LenProtoType`, `ScalarProtoType`, …).
//!
//! Singular scalars are thin wrappers (`ProtoInt32(i32)`, …). Repeated fields
//! use the inner `Value` / `Storage` associated types.

pub(crate) mod fixed;
pub(crate) mod len;
pub(crate) mod scalar;
pub(crate) mod varint;

pub use len::{LenProtoType, ProtoBytes, ProtoString};
pub use scalar::ScalarProtoType;
pub use varint::{
    ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64,
    ProtoUInt32, ProtoUInt64, VarintProtoType,
};
