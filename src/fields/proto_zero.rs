//! Protobuf type-zero semantics for scalar field storage types.

use super::varint::ProtoEnumStorage;

/// Protobuf type-zero for a singular scalar storage type.
///
/// [`proto_zero`](Self::proto_zero) constructs the type-zero value (message ctor,
/// lazy-init placeholder). [`is_proto_zero`](Self::is_proto_zero) drives implicit
/// presence omit-on-wire checks. [`set_proto_zero`](Self::set_proto_zero) writes
/// the type-zero into an existing storage slot (clear).
pub trait ProtoZero: Copy {
    /// Returns the protobuf type-zero (`0`, `false`, …).
    fn proto_zero() -> Self;

    /// Returns `true` when `value` equals the protobuf type-zero.
    fn is_proto_zero(value: &Self) -> bool;

    /// Writes the protobuf type-zero into `value`.
    fn set_proto_zero(value: &mut Self) {
        *value = Self::proto_zero();
    }
}

impl ProtoZero for i32 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for i64 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for u32 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for u64 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for bool {
    fn proto_zero() -> Self {
        false
    }

    fn is_proto_zero(value: &Self) -> bool {
        !*value
    }
}

impl<E: ProtoEnumStorage + PartialEq> ProtoZero for E {
    fn proto_zero() -> Self {
        E::proto_zero()
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == E::proto_zero()
    }
}
