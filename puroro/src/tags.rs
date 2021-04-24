use std::marker::PhantomData;

pub trait FieldTypeTag {}
pub trait SingularFieldTypeTag: FieldTypeTag {}
pub trait VariantTypeTag: FieldTypeTag {}

pub struct Int32();
pub struct UInt32();
pub struct SInt32();
pub struct Int64();
pub struct UInt64();
pub struct SInt64();
pub struct Bool();
pub struct Bytes();
pub struct String();
pub struct Enum<T>(PhantomData<T>);
pub struct Message<T>(PhantomData<T>);
pub struct Float();
pub struct Double();
pub struct SFixed32();
pub struct SFixed64();
pub struct Fixed32();
pub struct Fixed64();
pub struct Repeated<T: SingularFieldTypeTag>(PhantomData<T>);

impl FieldTypeTag for Int32 {}
impl FieldTypeTag for Int64 {}
impl FieldTypeTag for UInt32 {}
impl FieldTypeTag for UInt64 {}
impl FieldTypeTag for SInt32 {}
impl FieldTypeTag for SInt64 {}
impl FieldTypeTag for Bool {}
impl FieldTypeTag for Bytes {}
impl FieldTypeTag for String {}
impl<T> FieldTypeTag for Enum<T> {}
impl<T> FieldTypeTag for Message<T> {}
impl<T: SingularFieldTypeTag> FieldTypeTag for Repeated<T> {}

impl SingularFieldTypeTag for Int32 {}
impl SingularFieldTypeTag for Int64 {}
impl SingularFieldTypeTag for UInt32 {}
impl SingularFieldTypeTag for UInt64 {}
impl SingularFieldTypeTag for SInt32 {}
impl SingularFieldTypeTag for SInt64 {}
impl SingularFieldTypeTag for Bool {}
impl SingularFieldTypeTag for String {}
impl<T> SingularFieldTypeTag for Enum<T> {}
impl<T> SingularFieldTypeTag for Message<T> {}

impl VariantTypeTag for Int32 {}
impl VariantTypeTag for Int64 {}
impl VariantTypeTag for UInt32 {}
impl VariantTypeTag for UInt64 {}
impl VariantTypeTag for SInt32 {}
impl VariantTypeTag for SInt64 {}
impl VariantTypeTag for Bool {}

pub trait WireTypeTag {}
pub struct Variant();
impl WireTypeTag for Variant {}
pub struct LengthDelimited();
impl WireTypeTag for LengthDelimited {}
pub struct Bits32();
impl WireTypeTag for Bits32 {}
pub struct Bits64();
impl WireTypeTag for Bits64 {}

pub trait FieldLabelTag {}
pub struct FieldLabelOptional();
impl FieldLabelTag for FieldLabelOptional {}
pub struct FieldLabelRepeated();
impl FieldLabelTag for FieldLabelRepeated {}
pub struct FieldLabelRequired();
impl FieldLabelTag for FieldLabelRequired {}
