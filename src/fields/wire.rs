//! Wire-encoding type markers (`VarintProtoType`, `LenProtoType`, …).

pub mod fixed;
pub mod len;
pub mod varint;

pub use fixed::{Fixed32ProtoType, Fixed64ProtoType};
pub use len::{LenProtoType, ProtoBytes, ProtoString};
pub use varint::{
    enum_value_is_known, ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64,
    ProtoSint32, ProtoSint64, ProtoUInt32, ProtoUInt64, VarintProtoType, WIRE_TYPE as VARINT_WIRE_TYPE,
};
