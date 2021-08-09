use ::puroro::fixed_bits::{Bits32TypeTag, Bits64TypeTag};
use ::puroro::types::FieldData;
use ::puroro::variant::VariantTypeTag;
use ::puroro::{tags, RepeatedField, Result};
use ::std::borrow::Borrow;
use ::std::borrow::Cow;
use ::std::marker::PhantomData;
use puroro::ErrorKind;

use crate::de::from_iter::{ScopedIter, Variants};

pub struct VecWrapper<'msg, T>(&'msg Vec<T>);

impl<'msg, T> VecWrapper<'msg, T> {
    pub fn new(vec: &'msg Vec<T>) -> Self {
        Self(vec)
    }
}

impl<'msg, T: Clone> IntoIterator for VecWrapper<'msg, T> {
    type Item = T;
    type IntoIter = std::iter::Cloned<std::slice::Iter<'msg, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}
impl<'msg, T: Clone> RepeatedField<'msg, T> for VecWrapper<'msg, T> {}

pub struct VecCowWrapper<'msg, B: ?Sized + ToOwned>(&'msg Vec<B::Owned>);

impl<'msg, B: ?Sized + ToOwned> VecCowWrapper<'msg, B> {
    pub fn new(vec: &'msg Vec<B::Owned>) -> Self {
        Self(vec)
    }
}
impl<'msg, B: 'msg + ?Sized + ToOwned> IntoIterator for VecCowWrapper<'msg, B> {
    type Item = Cow<'msg, B>;
    type IntoIter = CowIter<'msg, B, std::slice::Iter<'msg, B::Owned>>;

    fn into_iter(self) -> Self::IntoIter {
        CowIter(self.0.iter(), PhantomData)
    }
}
pub struct CowIter<'msg, B, Iter>(Iter, PhantomData<B>)
where
    B: 'msg + ?Sized + ToOwned,
    Iter: Iterator<Item = &'msg B::Owned>;
impl<'msg, B, Iter> Iterator for CowIter<'msg, B, Iter>
where
    B: 'msg + ?Sized + ToOwned,
    Iter: Iterator<Item = &'msg B::Owned>,
{
    type Item = Cow<'msg, B>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| Cow::Borrowed(x.borrow()))
    }
}
impl<'msg, B> RepeatedField<'msg, Cow<'msg, B>> for VecCowWrapper<'msg, B> where
    B: 'msg + ?Sized + ToOwned
{
}

// deser from iter methods

pub trait VecOrOptionOrBare<T> {
    fn push(&mut self, val: T);
}
impl<T> VecOrOptionOrBare<T> for Option<T> {
    fn push(&mut self, val: T) {
        *self = Some(val);
    }
}
impl<T> VecOrOptionOrBare<T> for Vec<T> {
    fn push(&mut self, val: T) {
        self.push(val);
    }
}
impl<T> VecOrOptionOrBare<T> for T {
    fn push(&mut self, val: T) {
        *self = val;
    }
}

pub struct DeserFieldFromBytesIter<L, V>(PhantomData<(L, V)>);

impl<L, V> DeserFieldFromBytesIter<L, tags::wire::Variant<V>>
where
    L: tags::FieldLabelTag,
    V: VariantTypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<V as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        match input {
            FieldData::Variant(v) => {
                let native_value = v.to_native::<V>()?;
                if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                    field.push(native_value);
                }
            }
            FieldData::LengthDelimited(iter) => {
                let variants = Variants::new(iter);
                let values = variants.map(|rv| rv.and_then(|v| v.to_native::<V>()));
                for rvalue in values {
                    let native_value = rvalue?;
                    if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                        field.push(native_value);
                    }
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}

impl<L, V> DeserFieldFromBytesIter<L, tags::wire::Bits32<V>>
where
    L: tags::FieldLabelTag,
    V: Bits32TypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<V as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits32(bytes) = input {
            let native_value = V::from_array(bytes);
            if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                field.push(native_value);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L, V> DeserFieldFromBytesIter<L, tags::wire::Bits64<V>>
where
    L: tags::FieldLabelTag,
    V: Bits64TypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<V as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits64(bytes) = input {
            let native_value = V::from_array(bytes);
            if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                field.push(native_value);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}
