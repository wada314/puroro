use std::borrow::{Borrow, Cow};
use std::convert::TryFrom;
use std::hash::Hash;
use std::intrinsics::transmute;
use std::marker::PhantomData;

use crate::deser::LdSlice;
use crate::internal_data::SliceSource;
use crate::types::{FieldData, SliceViewField};
use crate::InternalDataForSliceViewStruct;
use crate::{tags, MapEntryForSliceViewImpl};
use crate::{ErrorKind, PuroroError, Result, ResultHelper};
use ::itertools::{Either, Itertools};
use ::puroro::{MapField, RepeatedField};

pub trait FieldDataIntoIter<'slice> {
    type Item;
    type Iter: Iterator<Item = Result<Self::Item>>;
    fn into(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter>;
}
impl<'slice> FieldDataIntoIter<'slice> for tags::Int32 {
    type Item = i32;
    type Iter = impl Iterator<Item = Result<Self::Item>>;
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
impl<'slice> FieldDataIntoIter<'slice> for tags::String {
    type Item = Cow<'slice, str>;
    type Iter = impl Iterator<Item = Result<Self::Item>>;
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
impl<'slice, T> FieldDataIntoIter<'slice> for tags::Message<T>
where
    T: 'slice + TryFrom<&'slice [u8], Error = PuroroError> + ToOwned<Owned = T>,
{
    type Item = Cow<'slice, T>;
    type Iter = impl Iterator<Item = Result<Self::Item>>;
    fn into(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::LengthDelimited(ld_slice) = field_data {
            Ok(std::iter::once(Ok(Cow::Owned(<T as TryFrom<
                &'slice [u8],
            >>::try_from(
                ld_slice.as_slice()
            )?))))
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepeatedSliceViewField<'slice, 'msg, S, TypeTag> {
    maybe_field: Option<&'msg SliceViewField<'slice>>,
    field_number: usize,
    internal_data: &'msg InternalDataForSliceViewStruct<'slice, S>,
    phantom: PhantomData<TypeTag>,
}

impl<'slice, 'msg, S, TypeTag> RepeatedSliceViewField<'slice, 'msg, S, TypeTag>
where
    TypeTag: FieldDataIntoIter<'slice>,
    S: SliceSource<'slice>,
{
    pub fn new(
        maybe_field: Option<&'msg SliceViewField<'slice>>,
        field_number: usize,
        internal_data: &'msg InternalDataForSliceViewStruct<'slice, S>,
    ) -> Self {
        Self {
            maybe_field,
            field_number,
            internal_data,
            phantom: PhantomData,
        }
    }

    fn iter_impl(
        &'msg self,
    ) -> impl 'msg + Iterator<Item = <TypeTag as FieldDataIntoIter<'slice>>::Item> {
        self.maybe_field.into_iter().flat_map(move |field| {
            self.internal_data
                .maybe_source_slices
                .clone()
                .into_iter()
                .flat_map(move |source_slices| {
                    field
                        .field_data_iter(self.field_number, source_slices)
                        .map_ok(|field| -> Result<_> {
                            <TypeTag as FieldDataIntoIter>::into(field)
                        })
                        .map(|rrval| rrval.flatten())
                        .flatten_ok()
                        .map(|rrval| rrval.flatten())
                        .map(|result| result.unwrap())
                })
        })
    }
}

impl<'slice, 'msg, S, T, TypeTag> RepeatedField<'msg, T>
    for RepeatedSliceViewField<'slice, 'msg, S, TypeTag>
where
    TypeTag: FieldDataIntoIter<'slice, Item = T>,
    S: SliceSource<'slice>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(T),
    {
        self.iter_impl().for_each(f)
    }
    fn boxed_iter(&self) -> Box<dyn '_ + Iterator<Item = T>> {
        Box::new(self.iter_impl())
    }

    type Iter<'this> = impl Iterator<Item = T>;
    fn iter(&self) -> Self::Iter<'_> {
        self.iter_impl()
    }
}

#[rustfmt::skip]
impl<'slice, 'msg, S, Q, R, Entry> MapField<'msg, Q, R>
    for RepeatedSliceViewField<'slice, 'msg, S, tags::Message<Entry>>
where
    'slice: 'msg,
    Q: ?Sized + Hash + Eq,
    R: 'msg,
    S: SliceSource<'slice>,
    Entry::OwnedKeyType: Borrow<Q>,
    Entry: 'slice
        + MapEntryForSliceViewImpl<'slice, ValueGetterType = R>
        + TryFrom<&'slice[u8], Error=PuroroError>
        + ToOwned<Owned = Entry>,
{
    fn get(&'msg self, key: &Q) -> Option<R>
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
