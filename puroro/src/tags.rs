use std::convert::TryFrom;

/// A tag trait for types corresponding to the field's type.
/// e.g. Int32, Float, String, Message<M>
/// This type actually consist of two tags for generics specialization:
/// `wire::wire_tag<value::value_tag>`.
pub trait ValueTypeTag {}

/// A tag trait for types corresponding to the proto syntax.
/// Proto2 or Proto3.
pub trait ProtoSyntaxTag {}

/// A tuple of (`ProtoSyntaxTag`, `ValueTypeTag`).
pub trait FieldTypeTag {}

/// A `FieldTypeTag` which has wire type one of Variant, Bits32 or Bits64.
pub trait NumericalFieldTypeTag: FieldTypeTag {
    type NativeType: Clone;
    fn default() -> Self::NativeType;
}

/// A tag trait for types corresponding to the field label.
/// e.g. Optional, Repeated, Required
pub trait FieldLabelTag {}

/// A tuple of (`FieldLabelTag`, `FieldTypeTag`).
/// TODO: Maybe map type should have its own tag type.
pub trait FieldLabelAndTypeTag {}

/// A tag trait for implementations of the proto message.
/// e.g. Simple, Bumpalo, SliceView
pub trait ImplTypeTag {}

pub mod value {
    use std::marker::PhantomData;
    pub struct Int32;
    pub struct UInt32;
    pub struct SInt32;
    pub struct Int64;
    pub struct UInt64;
    pub struct SInt64;
    pub struct Bool;
    pub struct Bytes;
    pub struct String;
    pub struct Enum<T>(PhantomData<T>);
    pub struct Message<T>(PhantomData<T>);
    pub struct Float;
    pub struct Double;
    pub struct SFixed32;
    pub struct SFixed64;
    pub struct Fixed32;
    pub struct Fixed64;
}

pub mod wire {
    use ::std::marker::PhantomData;
    pub type Variant<V> = (PhantomData<V>, (bool, ()), ((), ()));
    pub type LengthDelimited<V> = (PhantomData<V>, ((), bool), ((), ()));
    pub type Bits32<V> = (PhantomData<V>, ((), ()), (bool, ()));
    pub type Bits64<V> = (PhantomData<V>, ((), ()), ((), bool));

    pub type NonLD<V, _1, _2> = (PhantomData<V>, (_1, ()), _2);
    pub type Bits32Or64<V, _1> = (PhantomData<V>, ((), ()), _1);
}

pub type Int32 = wire::Variant<value::Int32>;
pub type SInt32 = wire::Variant<value::SInt32>;
pub type UInt32 = wire::Variant<value::UInt32>;
pub type Int64 = wire::Variant<value::Int64>;
pub type SInt64 = wire::Variant<value::SInt64>;
pub type UInt64 = wire::Variant<value::UInt64>;
pub type Bool = wire::Variant<value::Bool>;
pub type String = wire::LengthDelimited<value::String>;
pub type Bytes = wire::LengthDelimited<value::Bytes>;
pub type Float = wire::Bits32<value::Float>;
pub type Fixed32 = wire::Bits32<value::Fixed32>;
pub type SFixed32 = wire::Bits32<value::SFixed32>;
pub type Double = wire::Bits64<value::Double>;
pub type Fixed64 = wire::Bits64<value::Fixed64>;
pub type SFixed64 = wire::Bits64<value::SFixed64>;
pub type Enum<T> = wire::Variant<value::Enum<T>>;
pub type Message<T> = wire::LengthDelimited<value::Message<T>>;

/// A repeated field, which is available in both proto2 and proto3.
pub type Repeated = ((bool, ()), ((), ()));
/// Proto2 optional field || Proto3 explicitly optional marked field.
pub type Optional = (((), bool), ((), ()));
/// Proto3 unlabeled field.
pub type Unlabeled = (((), ()), (bool, ()));
/// Only available in proto2.
pub type Required = (((), ()), ((), bool));

pub type NonRepeated<_1, _2> = (((), _1), _2);

trait WrappedType<Label> {
    type Type;
}
impl<T, _1, _2> WrappedType<NonRepeated<_1, _2>> for T {
    type Type = Option<T>;
}
impl<T> WrappedType<Repeated> for T {
    type Type = Vec<T>;
}

pub struct Proto2;
pub struct Proto3;

impl ValueTypeTag for Int32 {}
impl ValueTypeTag for Int64 {}
impl ValueTypeTag for UInt32 {}
impl ValueTypeTag for UInt64 {}
impl ValueTypeTag for SInt32 {}
impl ValueTypeTag for SInt64 {}
impl ValueTypeTag for Bool {}
impl ValueTypeTag for Bytes {}
impl ValueTypeTag for String {}
impl<T> ValueTypeTag for Enum<T> {}
impl<T> ValueTypeTag for Message<T> {}
impl ValueTypeTag for Float {}
impl ValueTypeTag for Double {}
impl ValueTypeTag for Fixed32 {}
impl ValueTypeTag for Fixed64 {}
impl ValueTypeTag for SFixed32 {}
impl ValueTypeTag for SFixed64 {}

impl ProtoSyntaxTag for Proto2 {}
impl ProtoSyntaxTag for Proto3 {}

impl<S, T> FieldTypeTag for (S, T)
where
// Not setting these bounds for code simplicity
//S: ProtoSyntaxTag,
//T: ValueTypeTag,
{
}

impl<S> NumericalFieldTypeTag for (S, Int32) {
    type NativeType = i32;
    fn default() -> Self::NativeType {
        0
    }
}
impl<S> NumericalFieldTypeTag for (S, UInt32) {
    type NativeType = u32;
    fn default() -> Self::NativeType {
        0
    }
}
impl<S> NumericalFieldTypeTag for (S, SInt32) {
    type NativeType = i32;
    fn default() -> Self::NativeType {
        0
    }
}
impl<S> NumericalFieldTypeTag for (S, Float) {
    type NativeType = f32;
    fn default() -> Self::NativeType {
        0.0
    }
}
impl<S> NumericalFieldTypeTag for (S, Int64) {
    type NativeType = i64;
    fn default() -> Self::NativeType {
        0
    }
}
impl<S> NumericalFieldTypeTag for (S, UInt64) {
    type NativeType = u64;
    fn default() -> Self::NativeType {
        0
    }
}
impl<S> NumericalFieldTypeTag for (S, SInt64) {
    type NativeType = i64;
    fn default() -> Self::NativeType {
        0
    }
}
impl<S> NumericalFieldTypeTag for (S, Double) {
    type NativeType = f64;
    fn default() -> Self::NativeType {
        0.0
    }
}
impl<S> NumericalFieldTypeTag for (S, Bool) {
    type NativeType = bool;
    fn default() -> Self::NativeType {
        false
    }
}
impl<T: Clone + Default> NumericalFieldTypeTag for (Proto2, Enum<T>) {
    type NativeType = T;
    fn default() -> Self::NativeType {
        Default::default()
    }
}
impl<T: Clone + TryFrom<i32, Error = i32>> NumericalFieldTypeTag for (Proto3, Enum<T>) {
    type NativeType = ::std::result::Result<T, i32>;
    fn default() -> Self::NativeType {
        <T as TryFrom<i32>>::try_from(0)
    }
}

impl FieldLabelTag for Repeated {}
impl FieldLabelTag for Optional {}
impl FieldLabelTag for Unlabeled {}
impl FieldLabelTag for Required {}

impl<L, V> FieldLabelAndTypeTag for (L, V)
where
// Not setting these bounds for code simplicity
//    L: FieldLabelTag,
//    V: FieldTypeTag,
{
}
