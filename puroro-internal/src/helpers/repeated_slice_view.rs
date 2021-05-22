use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::intrinsics::transmute;
use std::marker::PhantomData;

use crate::deser::LdSlice;
use crate::types::{FieldData, SliceViewField};
use crate::InternalDataForSliceViewStruct;
use crate::{tags, MapEntry};
use crate::{ErrorKind, Result, ResultHelper};
use ::itertools::{Either, Itertools};
use ::puroro::{MapField, RepeatedField};
use puroro::DeserializableFromSlice;

pub trait FieldDataIntoIter<'slice, 'par> {
    type Item;
    type Iter: 'par + Iterator<Item = Result<Self::Item>>;
    fn into(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter>;
}
impl<'slice, 'par> FieldDataIntoIter<'slice, 'par> for tags::Int32
where
    'slice: 'par,
{
    type Item = i32;
    type Iter = impl 'par + Iterator<Item = Result<Self::Item>>;
    fn into(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        Ok(match field_data {
            FieldData::Variant(variant) => {
                Either::Left(std::iter::once(variant.to_native::<Self>()))
            }
            FieldData::LengthDelimited(ld_slice) => Either::Right(
                ld_slice
                    .variants()
                    .map_ok(|variant| variant.to_native::<Self>())
                    .map(|rrval| rrval.flatten()),
            ),
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        .into_iter())
    }
}
impl<'slice, 'par> FieldDataIntoIter<'slice, 'par> for tags::String
where
    'slice: 'par,
{
    type Item = Cow<'par, str>;
    type Iter = impl 'par + Iterator<Item = Result<Self::Item>>;
    fn into(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::LengthDelimited(ld_slice) = field_data {
            Ok(std::iter::once(Ok(Cow::Borrowed(unsafe {
                transmute(ld_slice.as_slice())
            }))))
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}
impl<'slice, 'par, T> FieldDataIntoIter<'slice, 'par> for tags::Message<T>
where
    T: 'par + DeserializableFromSlice<'slice> + ToOwned<Owned = T>,
    'slice: 'par,
{
    type Item = Cow<'par, T>;
    type Iter = impl 'par + Iterator<Item = Result<Self::Item>>;
    fn into(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::LengthDelimited(ld_slice) = field_data {
            Ok(std::iter::once(Ok(Cow::Owned(
                <T as DeserializableFromSlice>::deser_from_slice(ld_slice.as_slice())?,
            ))))
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepeatedSliceViewField<'slice, 'par, TypeTag>
where
    TypeTag: FieldDataIntoIter<'slice, 'par>,
{
    field: Option<&'par SliceViewField<'slice>>,
    field_number: usize,
    internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,
    phantom: PhantomData<TypeTag>,
}

impl<'slice, 'par, TypeTag> RepeatedSliceViewField<'slice, 'par, TypeTag>
where
    TypeTag: FieldDataIntoIter<'slice, 'par>,
{
    pub fn new(
        field: Option<&'par SliceViewField<'slice>>,
        field_number: usize,
        internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,
    ) -> Self {
        Self {
            field,
            field_number,
            internal_data,
            phantom: PhantomData,
        }
    }

    fn iter_impl(
        &'par self,
    ) -> impl 'par + Iterator<Item = <TypeTag as FieldDataIntoIter<'slice, 'par>>::Item> {
        self.internal_data
            .field_data_iter(self.field_number, self.field)
            .map_ok(|field| -> Result<_> { <TypeTag as FieldDataIntoIter>::into(field) })
            .map(|rrval| rrval.flatten())
            .flatten_ok()
            .map(|rrval| rrval.flatten())
            .map(|result| result.unwrap())
    }
}

impl<'slice, 'par, T, TypeTag> RepeatedField<'par, T>
    for RepeatedSliceViewField<'slice, 'par, TypeTag>
where
    'slice: 'par,
    T: 'par,
    TypeTag: FieldDataIntoIter<'slice, 'par, Item = T>,
{
    fn for_each<F>(&'par self, f: F)
    where
        F: FnMut(T),
    {
        self.iter_impl().for_each(f)
    }
    fn boxed_iter(&'par self) -> Box<dyn 'par + Iterator<Item = T>> {
        Box::new(self.iter_impl())
    }

    type Iter = impl Iterator<Item = T>;
    fn iter(&'par self) -> Self::Iter {
        self.iter_impl()
    }
}

impl<'slice, 'a, K, V, Entry> MapField<'a, K, V>
    for RepeatedSliceViewField<'slice, 'a, tags::Message<Entry>>
where
    'slice: 'a,
    K: ?Sized,
    Entry::KeyType: Hash + Eq + Borrow<K>,
    Entry: 'a + MapEntry<ValueType = V> + DeserializableFromSlice<'slice> + ToOwned<Owned = Entry>,
{
    fn get(&'a self, key: &K) -> Option<V>
    where
        K: Hash + Eq,
    {
        self.iter_impl().find_map(|entry| {
            let (ekey, evalue) = entry.into_owned().into_tuple();
            if ekey.borrow().eq(key) {
                Some(evalue)
            } else {
                None
            }
        })
    }
}
