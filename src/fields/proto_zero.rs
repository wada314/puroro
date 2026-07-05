//! Protobuf type-zero semantics for scalar field storage types.

use super::varint::ProtoEnumStorage;

/// Protobuf type-zero for a singular scalar storage type.
///
/// [`is_proto_zero`](Self::is_proto_zero) is the primary API — used for implicit
/// presence omit-on-wire checks. [`set_proto_zero`](Self::set_proto_zero) writes
/// the type-zero into an existing storage slot (init or clear).
pub trait ProtoZero {
    /// Returns `true` when `value` equals the protobuf type-zero (`0`, `false`, …).
    fn is_proto_zero(value: &Self) -> bool;

    /// Writes the protobuf type-zero into `value`.
    fn set_proto_zero(value: &mut Self);
}

impl ProtoZero for i32 {
    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }

    fn set_proto_zero(value: &mut Self) {
        *value = 0;
    }
}

impl ProtoZero for i64 {
    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }

    fn set_proto_zero(value: &mut Self) {
        *value = 0;
    }
}

impl ProtoZero for u32 {
    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }

    fn set_proto_zero(value: &mut Self) {
        *value = 0;
    }
}

impl ProtoZero for u64 {
    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }

    fn set_proto_zero(value: &mut Self) {
        *value = 0;
    }
}

impl ProtoZero for bool {
    fn is_proto_zero(value: &Self) -> bool {
        !*value
    }

    fn set_proto_zero(value: &mut Self) {
        *value = false;
    }
}

impl<E: ProtoEnumStorage + PartialEq> ProtoZero for E {
    fn is_proto_zero(value: &Self) -> bool {
        *value == E::proto_zero()
    }

    fn set_proto_zero(value: &mut Self) {
        *value = E::proto_zero();
    }
}
