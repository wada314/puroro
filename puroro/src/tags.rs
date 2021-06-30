use std::marker::PhantomData;

/// A tag type corresponding to the field's type.
/// e.g. Int32, Float, String, Message<M>
pub trait ValueTypeTag {}

/// A tag type corresponding to the field's type which is the wire type is variant.
/// e.g. Int32, UInt64
pub trait VariantValueTypeTag: ValueTypeTag {}

/// A tag type corresponding to the field's type which is the wire type is length delimited.
/// e.g. String, Message<M>
pub trait LengthDelimitedValueTypeTag: ValueTypeTag {}

/// A tag type corresponding to the field's type which is the wire type is 32 bits.
/// e.g. Float, Fixed32
pub trait Bits32ValueTypeTag: ValueTypeTag {}

/// A tag type corresponding to the field's type which is the wire type is 64 bits.
/// e.g. Double, Fixed64
pub trait Bits64ValueTypeTag: ValueTypeTag {}

/// A tag type corresponding to the field's wire type.
/// e.g. Variant, LengthDelimited, Bytes32
pub trait WireTypeTag {}

/// A tag type corresponding to the proto syntax.
/// Proto2 or Proto3.
pub trait ProtoSyntaxTag {}

/// A tuple of (`ProtoSyntaxTag`, `WireTypeTag`, `ValueTypeTag`).
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

impl ValueTypeTag for value::Int32 {}
impl ValueTypeTag for value::Int64 {}
impl ValueTypeTag for value::UInt32 {}
impl ValueTypeTag for value::UInt64 {}
impl ValueTypeTag for value::SInt32 {}
impl ValueTypeTag for value::SInt64 {}
impl ValueTypeTag for value::Bool {}
impl ValueTypeTag for value::Bytes {}
impl ValueTypeTag for value::String {}
impl<T> ValueTypeTag for value::Enum<T> {}
impl<T> ValueTypeTag for value::Message<T> {}
impl ValueTypeTag for value::Float {}
impl ValueTypeTag for value::Double {}
impl ValueTypeTag for value::Fixed32 {}
impl ValueTypeTag for value::Fixed64 {}
impl ValueTypeTag for value::SFixed32 {}
impl ValueTypeTag for value::SFixed64 {}

impl VariantValueTypeTag for value::Int32 {}
impl VariantValueTypeTag for value::Int64 {}
impl VariantValueTypeTag for value::UInt32 {}
impl VariantValueTypeTag for value::UInt64 {}
impl VariantValueTypeTag for value::SInt32 {}
impl VariantValueTypeTag for value::SInt64 {}
impl VariantValueTypeTag for value::Bool {}
impl<T> VariantValueTypeTag for value::Enum<T> {}

impl LengthDelimitedValueTypeTag for value::String {}
impl LengthDelimitedValueTypeTag for value::Bytes {}
impl<T> LengthDelimitedValueTypeTag for value::Message<T> {}

impl Bits32ValueTypeTag for value::Fixed32 {}
impl Bits32ValueTypeTag for value::SFixed32 {}
impl Bits32ValueTypeTag for value::Float {}

impl Bits64ValueTypeTag for value::Fixed64 {}
impl Bits64ValueTypeTag for value::SFixed64 {}
impl Bits64ValueTypeTag for value::Double {}

impl WireTypeTag for wire::Variant {}
impl WireTypeTag for wire::LengthDelimited {}
impl WireTypeTag for wire::Bits32 {}
impl WireTypeTag for wire::Bits64 {}

impl ProtoSyntaxTag for Proto2 {}
impl ProtoSyntaxTag for Proto3 {}

impl<S, T> FieldTypeTag for (S, wire::Variant, T)
where
    S: ProtoSyntaxTag,
    T: VariantValueTypeTag,
{
}
impl<S, T> FieldTypeTag for (S, wire::LengthDelimited, T)
where
    S: ProtoSyntaxTag,
    T: LengthDelimitedValueTypeTag,
{
}
impl<S, T> FieldTypeTag for (S, wire::Bits32, T)
where
    S: ProtoSyntaxTag,
    T: Bits32ValueTypeTag,
{
}
impl<S, T> FieldTypeTag for (S, wire::Bits64, T)
where
    S: ProtoSyntaxTag,
    T: Bits64ValueTypeTag,
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
