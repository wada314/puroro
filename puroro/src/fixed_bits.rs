use crate::tags;

pub trait Bits32TypeTag: tags::NumericalFieldTypeTag {
    fn from_array(array: [u8; 4]) -> Self::NativeType;
    fn into_array(val: Self::NativeType) -> [u8; 4];
}
impl<X> Bits32TypeTag for (X, tags::Float) {
    fn from_array(array: [u8; 4]) -> Self::NativeType {
        f32::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 4] {
        f32::to_le_bytes(val)
    }
}
impl<X> Bits32TypeTag for (X, tags::Fixed32) {
    fn from_array(array: [u8; 4]) -> Self::NativeType {
        u32::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 4] {
        u32::to_le_bytes(val)
    }
}
impl<X> Bits32TypeTag for (X, tags::SFixed32) {
    fn from_array(array: [u8; 4]) -> Self::NativeType {
        i32::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 4] {
        i32::to_le_bytes(val)
    }
}

pub trait Bits64TypeTag: tags::NumericalFieldTypeTag {
    fn from_array(array: [u8; 8]) -> Self::NativeType;
    fn into_array(val: Self::NativeType) -> [u8; 8];
}
impl<X> Bits64TypeTag for (X, tags::Double) {
    fn from_array(array: [u8; 8]) -> Self::NativeType {
        f64::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 8] {
        f64::to_le_bytes(val)
    }
}
impl<X> Bits64TypeTag for (X, tags::Fixed64) {
    fn from_array(array: [u8; 8]) -> Self::NativeType {
        u64::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 8] {
        u64::to_le_bytes(val)
    }
}
impl<X> Bits64TypeTag for (X, tags::SFixed64) {
    fn from_array(array: [u8; 8]) -> Self::NativeType {
        i64::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 8] {
        i64::to_le_bytes(val)
    }
}
