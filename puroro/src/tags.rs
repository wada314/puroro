use std::marker::PhantomData;

/// A tag type corresponding to the field's type.
/// e.g. Int32, Float, String, Message<M>
/// This type actually consist of two tags for generics specialization:
/// `(wire::wire_tag, value::value_tag)`.
pub trait ValueTypeTag {}

/// A tag type corresponding to the proto syntax.
/// Proto2 or Proto3.
pub trait ProtoSyntaxTag {}

/// A tuple of (`ProtoSyntaxTag`, `ValueTypeTag`).
pub trait FieldTypeTag {}

/// A tag type corresponding to the field label.
/// e.g. Optional, Repeated, Required
pub trait FieldLabelTag {}

/// A tuple of (`FieldLabelTag`, `FieldTypeTag`).
/// TODO: Maybe map type should have its own tag type.
pub trait FieldLabelAndTypeTag {}

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
    pub struct Variant;
    pub struct LengthDelimited;
    pub struct Bits32;
    pub struct Bits64;
}

pub type Int32 = (wire::Variant, value::Int32);
pub type SInt32 = (wire::Variant, value::SInt32);
pub type UInt32 = (wire::Variant, value::UInt32);
pub type Int64 = (wire::Variant, value::Int64);
pub type SInt64 = (wire::Variant, value::SInt64);
pub type UInt64 = (wire::Variant, value::UInt64);
pub type Bool = (wire::Variant, value::Bool);
pub type Enum<T> = (wire::Variant, value::Enum<T>);
pub type String = (wire::LengthDelimited, value::String);
pub type Bytes = (wire::LengthDelimited, value::Bytes);
pub type Message<T> = (wire::LengthDelimited, value::Message<T>);
pub type Float = (wire::Bits32, value::Float);
pub type Fixed32 = (wire::Bits32, value::Fixed32);
pub type SFixed32 = (wire::Bits32, value::SFixed32);
pub type Double = (wire::Bits64, value::Double);
pub type Fixed64 = (wire::Bits64, value::Fixed64);
pub type SFixed64 = (wire::Bits64, value::SFixed64);

/// A repeated field, which is available in both proto2 and proto3.
pub struct Repeated;
/// Proto2 optional field || Proto3 explicitly optional marked field.
pub struct Optional;

/// Proto3 unlabeled field.
pub struct Unlabeled;
/// Only available in proto2.
pub struct Required;

// Proto struct implementation types.
pub struct SimpleStruct;
pub struct Bumpalo<'bump>(PhantomData<&'bump ()>);
pub struct SliceView<'slice, S>(PhantomData<(&'slice (), S)>);

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
    S: ProtoSyntaxTag,
    T: ValueTypeTag,
{
}

impl FieldLabelTag for Repeated {}
impl FieldLabelTag for Optional {}
impl FieldLabelTag for Unlabeled {}
impl FieldLabelTag for Required {}

impl<L, V> FieldLabelAndTypeTag for (L, V)
where
    L: FieldLabelTag,
    V: FieldTypeTag,
{
}

impl ImplTypeTag for SimpleStruct {}
impl<'bump> ImplTypeTag for Bumpalo<'bump> {}
impl<'slice, S> ImplTypeTag for SliceView<'slice, S> {}
