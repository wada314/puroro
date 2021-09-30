use crate::bool::{False, True};
use ::std::marker::PhantomData;

/// A tag trait for types corresponding to the field's type.
/// e.g. Int32, Float, String, Message<M>
/// This type actually consist of two tags for generics specialization:
/// `wire_tag<value::value_tag>`.
pub trait FieldTypeTag {}

/// A `FieldTypeTag` which has wire type one of Variant (except enum), Bits32 or Bits64.
pub trait NumericalTypeTag {
    type NativeType: 'static + Default + PartialEq + Clone;
}

/// A tag trait for types corresponding to the field label.
/// e.g. Optional, Repeated, Required
pub trait FieldLabelTag {
    const DO_DEFAULT_CHECK: bool;
}

mod value {
    use ::std::marker::PhantomData;
    pub struct Int32;
    pub struct UInt32;
    pub struct SInt32;
    pub struct Int64;
    pub struct UInt64;
    pub struct SInt64;
    pub struct Bool;
    pub struct Bytes;
    pub struct String;
    pub struct Enum2<E>(PhantomData<E>);
    pub struct Enum3<E>(PhantomData<E>);
    pub struct Message<M>(PhantomData<M>);
    pub struct Float;
    pub struct Double;
    pub struct SFixed32;
    pub struct SFixed64;
    pub struct Fixed32;
    pub struct Fixed64;
}

pub type Variant<V> = (PhantomData<V>, (True, False), (False, False));
pub type LengthDelimited<V> = (PhantomData<V>, (False, True), (False, False));
pub type Bits32<V> = (PhantomData<V>, (False, False), (True, False));
pub type Bits64<V> = (PhantomData<V>, (False, False), (False, True));

pub type NonLD<V, _1, _2> = (PhantomData<V>, (_1, False), _2);

pub type Int32 = Variant<value::Int32>;
pub type SInt32 = Variant<value::SInt32>;
pub type UInt32 = Variant<value::UInt32>;
pub type Int64 = Variant<value::Int64>;
pub type SInt64 = Variant<value::SInt64>;
pub type UInt64 = Variant<value::UInt64>;
pub type Bool = Variant<value::Bool>;
pub type String = LengthDelimited<value::String>;
pub type Bytes = LengthDelimited<value::Bytes>;
pub type Float = Bits32<value::Float>;
pub type Fixed32 = Bits32<value::Fixed32>;
pub type SFixed32 = Bits32<value::SFixed32>;
pub type Double = Bits64<value::Double>;
pub type Fixed64 = Bits64<value::Fixed64>;
pub type SFixed64 = Bits64<value::SFixed64>;
pub type Enum2<E> = Variant<value::Enum2<E>>;
pub type Enum3<E> = Variant<value::Enum3<E>>;
pub type Message<M> = LengthDelimited<value::Message<M>>;

/// A repeated field, which is available in both proto2 and proto3.
pub type Repeated = (True, False, False, False, False);
/// Proto2 optional field || Proto3 explicitly optional marked field.
pub type Optional = (False, (True, False), False, False, False);
/// Only available in proto2.
pub type Required = (False, (False, True), False, False, False);
/// Proto3 unlabeled field.
pub type Unlabeled = (False, False, True, False, False);
/// An item of oneof.
pub type OneofField = (False, False, False, True, False);
/// Reserved for future use...
pub type MapEntry = (False, False, False, False, True);

pub type LabelOptReqUnl<_1, _2> = (False, _1, _2, False, False);
pub type LabelNonRepeated<_1, _2, _3> = (False, _1, _2, _3, False);

impl FieldTypeTag for Int32 {}
impl FieldTypeTag for Int64 {}
impl FieldTypeTag for UInt32 {}
impl FieldTypeTag for UInt64 {}
impl FieldTypeTag for SInt32 {}
impl FieldTypeTag for SInt64 {}
impl FieldTypeTag for Bool {}
impl FieldTypeTag for Bytes {}
impl FieldTypeTag for String {}
impl<E> FieldTypeTag for Enum2<E> {}
impl<E> FieldTypeTag for Enum3<E> {}
impl<M> FieldTypeTag for Message<M> {}
impl FieldTypeTag for Float {}
impl FieldTypeTag for Double {}
impl FieldTypeTag for Fixed32 {}
impl FieldTypeTag for Fixed64 {}
impl FieldTypeTag for SFixed32 {}
impl FieldTypeTag for SFixed64 {}

impl NumericalTypeTag for Int32 {
    type NativeType = i32;
}
impl NumericalTypeTag for UInt32 {
    type NativeType = u32;
}
impl NumericalTypeTag for SInt32 {
    type NativeType = i32;
}
impl NumericalTypeTag for Fixed32 {
    type NativeType = u32;
}
impl NumericalTypeTag for SFixed32 {
    type NativeType = i32;
}
impl NumericalTypeTag for Float {
    type NativeType = f32;
}
impl NumericalTypeTag for Int64 {
    type NativeType = i64;
}
impl NumericalTypeTag for UInt64 {
    type NativeType = u64;
}
impl NumericalTypeTag for SInt64 {
    type NativeType = i64;
}
impl NumericalTypeTag for Fixed64 {
    type NativeType = u64;
}
impl NumericalTypeTag for SFixed64 {
    type NativeType = i64;
}
impl NumericalTypeTag for Double {
    type NativeType = f64;
}
impl NumericalTypeTag for Bool {
    type NativeType = bool;
}
impl<E: crate::Enum2> NumericalTypeTag for Enum2<E> {
    type NativeType = E;
}
impl<E: crate::Enum3> NumericalTypeTag for Enum3<E> {
    type NativeType = E;
}

impl FieldLabelTag for Repeated {
    const DO_DEFAULT_CHECK: bool = false;
}
impl FieldLabelTag for Optional {
    const DO_DEFAULT_CHECK: bool = false;
}
impl FieldLabelTag for Unlabeled {
    const DO_DEFAULT_CHECK: bool = true;
}
impl FieldLabelTag for Required {
    const DO_DEFAULT_CHECK: bool = false;
}
impl FieldLabelTag for OneofField {
    const DO_DEFAULT_CHECK: bool = false;
}
