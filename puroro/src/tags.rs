use std::marker::PhantomData;

pub trait ValueTypeTag {}
pub trait VariantTypeTag: ValueTypeTag {}
pub trait LengthDelimitedTypeTag: ValueTypeTag {}
pub trait Bits32TypeTag: ValueTypeTag {}
pub trait Bits64TypeTag: ValueTypeTag {}
pub trait WireTypeTag {}
pub trait WireAndValueTypeTag {}
pub trait FieldLabelTag {}
pub trait FieldTypeAndLabelTag {}

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
pub struct Optional2;

/// Proto3 unlabeled field.
pub struct Optional3;
/// Only available in proto2.
pub struct Required;

// Proto struct implementation types.
pub struct SimpleStruct;
pub struct Bumpalo<'bump>(PhantomData<&'bump ()>);
pub struct SliceView<'slice>(PhantomData<&'slice ()>);

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

impl VariantTypeTag for value::Int32 {}
impl VariantTypeTag for value::Int64 {}
impl VariantTypeTag for value::UInt32 {}
impl VariantTypeTag for value::UInt64 {}
impl VariantTypeTag for value::SInt32 {}
impl VariantTypeTag for value::SInt64 {}
impl VariantTypeTag for value::Bool {}
impl<T> VariantTypeTag for value::Enum<T> {}

impl LengthDelimitedTypeTag for value::String {}
impl LengthDelimitedTypeTag for value::Bytes {}
impl<T> LengthDelimitedTypeTag for value::Message<T> {}

impl Bits32TypeTag for value::Fixed32 {}
impl Bits32TypeTag for value::SFixed32 {}
impl Bits32TypeTag for value::Float {}

impl Bits64TypeTag for value::Fixed64 {}
impl Bits64TypeTag for value::SFixed64 {}
impl Bits64TypeTag for value::Double {}

impl WireTypeTag for wire::Variant {}
impl WireTypeTag for wire::LengthDelimited {}
impl WireTypeTag for wire::Bits32 {}
impl WireTypeTag for wire::Bits64 {}

impl<T> WireAndValueTypeTag for (wire::Variant, T) where T: VariantTypeTag {}
impl<T> WireAndValueTypeTag for (wire::LengthDelimited, T) where T: LengthDelimitedTypeTag {}
impl<T> WireAndValueTypeTag for (wire::Bits32, T) where T: Bits32TypeTag {}
impl<T> WireAndValueTypeTag for (wire::Bits64, T) where T: Bits64TypeTag {}

impl FieldLabelTag for Repeated {}
impl FieldLabelTag for Optional2 {}
impl FieldLabelTag for Optional3 {}
impl FieldLabelTag for Required {}

impl ImplTypeTag for SimpleStruct {}
impl<'bump> ImplTypeTag for Bumpalo<'bump> {}
impl<'slice> ImplTypeTag for SliceView<'slice> {}
