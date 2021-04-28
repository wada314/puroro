use std::marker::PhantomData;

pub trait FieldTypeTag {}
pub trait VariantTypeTag: FieldTypeTag {}
pub trait FieldLabelTag {}
pub trait FieldTypeAndLabelTag {}

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
// Map is a little special, they cannot have [repeated|optional|required]
// labels. So `Map` is NOT a `FieldTypeTag`.
pub struct Map<K, V>(PhantomData<(K, V)>);

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
impl FieldTypeTag for Float {}
impl FieldTypeTag for Double {}
impl FieldTypeTag for Fixed32 {}
impl FieldTypeTag for Fixed64 {}
impl FieldTypeTag for SFixed32 {}
impl FieldTypeTag for SFixed64 {}

pub struct Repeated;
// Proto3 unlabeled field.
pub struct Optional2;
// Proto2 optional field || Proto3 explicitly optional marked field.
pub struct Optional3;
pub struct Required;
impl FieldLabelTag for Repeated {}
impl FieldLabelTag for Optional2 {}
impl FieldLabelTag for Optional3 {}
impl FieldLabelTag for Required {}

impl VariantTypeTag for Int32 {}
impl VariantTypeTag for Int64 {}
impl VariantTypeTag for UInt32 {}
impl VariantTypeTag for UInt64 {}
impl VariantTypeTag for SInt32 {}
impl VariantTypeTag for SInt64 {}
impl VariantTypeTag for Bool {}
impl<T> VariantTypeTag for Enum<T> {}

impl<T, L> FieldTypeAndLabelTag for (T, L)
where
    T: FieldTypeTag,
    L: FieldLabelTag,
{
}
impl<K, V> FieldTypeAndLabelTag for Map<K, V>
where
    K: FieldTypeTag,
    V: FieldTypeTag,
{
}
