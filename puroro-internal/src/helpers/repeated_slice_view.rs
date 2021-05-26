use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::intrinsics::transmute;
use std::marker::PhantomData;

use crate::deser::LdSlice;
use crate::types::{FieldData, SliceViewField};
use crate::InternalDataForSliceViewStruct;
use crate::{tags, MapEntryForSliceViewImpl};
use crate::{ErrorKind, Result, ResultHelper};
use ::itertools::{Either, Itertools};
use ::puroro::{MapField, RepeatedField};
use puroro::DeserializableFromSlice;

pub trait FieldDataIntoIter<'slice, 'msg> {
    type Item: 'msg;
    type Iter: 'msg + Iterator<Item = Result<Self::Item>>;
    fn into(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter>;
}
impl<'slice, 'msg> FieldDataIntoIter<'slice, 'msg> for tags::Int32
where
    'slice: 'msg,
{
    type Item = i32;
    type Iter = impl 'msg + Iterator<Item = Result<Self::Item>>;
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
impl<'slice, 'msg> FieldDataIntoIter<'slice, 'msg> for tags::String
where
    'slice: 'msg,
{
    type Item = Cow<'msg, str>;
    type Iter = impl 'msg + Iterator<Item = Result<Self::Item>>;
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
impl<'slice, 'msg, T> FieldDataIntoIter<'slice, 'msg> for tags::Message<T>
where
    T: 'msg + DeserializableFromSlice<'slice> + ToOwned<Owned = T>,
    'slice: 'msg,
{
    type Item = Cow<'msg, T>;
    type Iter = impl 'msg + Iterator<Item = Result<Self::Item>>;
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
pub struct RepeatedSliceViewField<'slice, 'msg, TypeTag>
where
    TypeTag: FieldDataIntoIter<'slice, 'msg>,
{
    field: Option<&'msg SliceViewField<'slice>>,
    field_number: usize,
    internal_data: &'msg InternalDataForSliceViewStruct<'slice, 'msg>,
    phantom: PhantomData<TypeTag>,
}

impl<'slice, 'msg, TypeTag> RepeatedSliceViewField<'slice, 'msg, TypeTag>
where
    TypeTag: FieldDataIntoIter<'slice, 'msg>,
{
    pub fn new(
        field: Option<&'msg SliceViewField<'slice>>,
        field_number: usize,
        internal_data: &'msg InternalDataForSliceViewStruct<'slice, 'msg>,
    ) -> Self {
        Self {
            field,
            field_number,
            internal_data,
            phantom: PhantomData,
        }
    }

    fn iter_impl(
        &'msg self,
    ) -> impl 'msg + Iterator<Item = <TypeTag as FieldDataIntoIter<'slice, 'msg>>::Item> {
        self.internal_data
            .field_data_iter(self.field_number, self.field)
            .map_ok(|field| -> Result<_> { <TypeTag as FieldDataIntoIter>::into(field) })
            .map(|rrval| rrval.flatten())
            .flatten_ok()
            .map(|rrval| rrval.flatten())
            .map(|result| result.unwrap())
    }
}

impl<'slice, 'msg, T, TypeTag> RepeatedField<'msg, T>
    for RepeatedSliceViewField<'slice, 'msg, TypeTag>
where
    'slice: 'msg,
    T: 'msg,
    TypeTag: FieldDataIntoIter<'slice, 'msg, Item = T>,
{
    fn for_each<F>(&'msg self, f: F)
    where
        F: FnMut(T),
    {
        self.iter_impl().for_each(f)
    }
    fn boxed_iter(&'msg self) -> Box<dyn 'msg + Iterator<Item = T>> {
        Box::new(self.iter_impl())
    }

    type Iter = impl Iterator<Item = T>;
    fn iter(&'msg self) -> Self::Iter {
        self.iter_impl()
    }
}

#[rustfmt::skip]
impl<'slice, 'a, Q, R, Entry> MapField<'a, Q, R>
    for RepeatedSliceViewField<'slice, 'a, tags::Message<Entry>>
where
    'slice: 'a,
    Q: ?Sized + Hash + Eq,
    R: 'a,
    Entry::OwnedKeyType: Borrow<Q>,
    Entry: 'a
        + MapEntryForSliceViewImpl<'slice, ValueGetterType = R>
        + DeserializableFromSlice<'slice>
        + ToOwned<Owned = Entry>,
{
    fn get(&'a self, key: &Q) -> Option<R>
    where
        Q: Hash + Eq,
    {
        self.iter_impl().find_map(|entry| {
            if entry.key_eq(key) {
                Some(entry.value())
            } else {
                None
            }
        })
    }
}
