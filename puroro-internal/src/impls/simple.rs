use ::itertools::Itertools;
use ::puroro::types::FieldData;
use ::puroro::variant::{Variant, VariantTypeTag};
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

pub trait MaybeOptional<T> {
    fn set(&mut self, val: T);
}
impl<T> MaybeOptional<T> for Option<T> {
    fn set(&mut self, val: T) {
        *self = Some(val);
    }
}
impl<T> MaybeOptional<T> for T {
    fn set(&mut self, val: T) {
        *self = val;
    }
}
pub trait VecOrOption<T> {
    fn push(&mut self, val: T);
}
impl<T> VecOrOption<T> for Option<T> {
    fn push(&mut self, val: T) {
        *self = Some(val);
    }
}
impl<T> VecOrOption<T> for Vec<T> {
    fn push(&mut self, val: T) {
        self.push(val);
    }
}

pub struct DeserFieldFromBytesIter<L, V>(PhantomData<(L, V)>);

impl<V, _1, _2>
    DeserFieldFromBytesIter<tags::RepeatedOrOptionalOrRequired<_1, _2>, tags::wire::Variant<V>>
where
    V: VariantTypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOption<<V as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        match input {
            FieldData::Variant(v) => {
                field.push(v.to_native::<V>()?);
            }
            FieldData::LengthDelimited(iter) => {
                let variants = Variants::new(iter);
                let values = variants.map(|rv| rv.and_then(|v| v.to_native::<V>()));
                for value in values {
                    field.push(value?);
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}

impl<V> DeserFieldFromBytesIter<tags::Unlabeled, tags::wire::Variant<V>>
where
    V: VariantTypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut <V as tags::NumericalTypeTag>::NativeType,
        input: FieldData<ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        match input {
            FieldData::Variant(v) => {
                let native_value = v.to_native::<V>()?;
                if native_value != Default::default() {
                    *field = native_value;
                }
            }
            FieldData::LengthDelimited(iter) => {
                let variants = Variants::new(iter);
                let native_values = variants.map(|rv| rv.and_then(|v| v.to_native::<V>()));
                for rnative_value in native_values {
                    let native_value = rnative_value?;
                    if native_value != Default::default() {
                        *field = native_value;
                    }
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}
