use std::borrow::Cow;
use std::convert::TryFrom;
use std::marker::PhantomData;

use crate::deser::LdSlice;
use crate::internal_data::SliceSource;
use crate::types::{FieldData, SliceViewField};
use crate::InternalDataForSliceViewStruct;
use crate::{variant, GetSliceViewStructImplFor};
use crate::{ErrorKind, PuroroError, Result, ResultHelper};
use ::itertools::{Either, Itertools};
use ::puroro::tags;
use ::puroro::RepeatedField;
use puroro::IsMessageImplOfTag;

trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

pub trait ItemTypeForTag<'slice, 'msg> {
    type Type;
}
impl<'slice, 'msg, V> ItemTypeForTag<'slice, 'msg> for (tags::wire::Variant, V)
where
    V: tags::VariantTypeTag,
{
    type Type = <V as tags::VariantTypeTag>::NativeType;
}
impl<'slice, 'msg> ItemTypeForTag<'slice, 'msg> for tags::Bytes {
    type Type = Cow<'msg, [u8]>;
}
impl<'slice, 'msg> ItemTypeForTag<'slice, 'msg> for tags::String {
    type Type = Cow<'msg, str>;
}
impl<'slice, 'msg, MessageTag> ItemTypeForTag<'slice, 'msg> for tags::Message<MessageTag>
where
    MessageTag: GetSliceViewStructImplFor<'slice>,
    <MessageTag as GetSliceViewStructImplFor<'slice>>::Type: 'msg + ToOwned,
    'slice: 'msg,
{
    type Type = Cow<'msg, <MessageTag as GetSliceViewStructImplFor<'slice>>::Type>;
}
impl<'slice, 'msg, V> ItemTypeForTag<'slice, 'msg> for (tags::wire::Bits32, V)
where
    V: tags::Bits32TypeTag,
{
    type Type = <V as tags::Bits32TypeTag>::NativeType;
}
impl<'slice, 'msg, V> ItemTypeForTag<'slice, 'msg> for (tags::wire::Bits64, V)
where
    V: tags::Bits64TypeTag,
{
    type Type = <V as tags::Bits64TypeTag>::NativeType;
}

pub trait IterFromFieldData<'slice, 'a, WireAndTypeTag>: Sized {
    type Iter: Iterator<Item = Result<Self>>;
    fn from(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter>;
}
impl<'slice: 'a, 'a, ValueTypeTag>
    IterFromFieldData<'slice, 'a, (tags::wire::Variant, ValueTypeTag)>
    for <ValueTypeTag as variant::VariantTypeTag>::NativeType
where
    ValueTypeTag: variant::VariantTypeTag,
{
    type Iter = impl Iterator<Item = Result<Self>>;
    fn from(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        Ok(match field_data {
            FieldData::Variant(variant) => {
                Either::Left(std::iter::once(variant.to_native::<ValueTypeTag>()))
            }
            FieldData::LengthDelimited(ld_slice) => Either::Right(
                ld_slice
                    .variants()
                    .map_ok(|variant| variant.to_native::<ValueTypeTag>())
                    .map(|rrval| rrval.flatten()),
            ),
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        .into_iter())
    }
}
impl<'slice: 'a, 'a> IterFromFieldData<'slice, 'a, tags::String> for Cow<'a, str> {
    type Iter = impl Iterator<Item = Result<Self>>;
    fn from(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::LengthDelimited(ld_slice) = field_data {
            Ok(std::iter::once(Ok(String::from_utf8_lossy(
                ld_slice.as_slice(),
            ))))
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}
impl<'slice: 'a, 'a> IterFromFieldData<'slice, 'a, tags::Bytes> for Cow<'a, [u8]> {
    type Iter = impl Iterator<Item = Result<Self>>;
    fn from(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::LengthDelimited(ld_slice) = field_data {
            Ok(std::iter::once(Ok(Cow::Borrowed(ld_slice.as_slice()))))
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}
impl<'slice: 'a, 'a, Message, MessageTag> IterFromFieldData<'slice, 'a, tags::Message<MessageTag>>
    for Cow<'a, Message>
where
    Message: 'a
        + ToOwned<Owned = Message>
        + TryFrom<&'slice [u8], Error = PuroroError>
        + IsMessageImplOfTag<MessageTag>,
{
    type Iter = impl Iterator<Item = Result<Self>>;
    fn from(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::LengthDelimited(ld_slice) = field_data {
            Ok(std::iter::once(Ok(Cow::Owned(<Message as TryFrom<
                &'slice [u8],
            >>::try_from(
                ld_slice.as_slice()
            )?))))
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}
impl<'slice: 'a, 'a, ValueTypeTag> IterFromFieldData<'slice, 'a, (tags::wire::Bits32, ValueTypeTag)>
    for <ValueTypeTag as super::Bits32TypeTag>::NativeType
where
    ValueTypeTag: super::Bits32TypeTag,
{
    type Iter = impl Iterator<Item = Result<Self>>;
    fn from(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::Bits32(bytes) = field_data {
            Ok(std::iter::once(Ok(
                <ValueTypeTag as super::Bits32TypeTag>::from_bytes(bytes),
            )))
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}
impl<'slice: 'a, 'a, ValueTypeTag> IterFromFieldData<'slice, 'a, (tags::wire::Bits64, ValueTypeTag)>
    for <ValueTypeTag as super::Bits64TypeTag>::NativeType
where
    ValueTypeTag: super::Bits64TypeTag,
{
    type Iter = impl Iterator<Item = Result<Self>>;
    fn from(field_data: FieldData<LdSlice<'slice>>) -> Result<Self::Iter> {
        if let FieldData::Bits64(bytes) = field_data {
            Ok(std::iter::once(Ok(
                <ValueTypeTag as super::Bits64TypeTag>::from_bytes(bytes),
            )))
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

    fn iter_impl<Item>(&self) -> impl 'msg + Iterator<Item = Item> + Captures<'slice>
    where
        Item: 'msg + IterFromFieldData<'slice, 'msg, TypeTag>,
        TypeTag: 'msg,
        'slice: 'msg,
    {
        let field_number = self.field_number;
        let internal_data = self.internal_data;
        self.maybe_field.clone().into_iter().flat_map(move |field| {
            internal_data
                .maybe_source_slices
                .clone()
                .into_iter()
                .flat_map(move |source_slices| {
                    field
                        .field_data_iter(field_number, source_slices)
                        .map_ok(<Item as IterFromFieldData<'slice, 'msg, TypeTag>>::from)
                        .map(|rrval| rrval.flatten())
                        .flatten_ok()
                        .map(|rrval| rrval.flatten())
                        .map(|result| result.unwrap())
                })
        })
    }
}

impl<'slice, 'msg, S, TypeTag> RepeatedField<<TypeTag as ItemTypeForTag<'slice, 'msg>>::Type>
    for RepeatedSliceViewField<'slice, 'msg, S, TypeTag>
where
    TypeTag: 'msg + ItemTypeForTag<'slice, 'msg>,
    S: SliceSource<'slice>,
    <TypeTag as ItemTypeForTag<'slice, 'msg>>::Type:
        'msg + IterFromFieldData<'slice, 'msg, TypeTag>,
{
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(<TypeTag as ItemTypeForTag<'slice, 'msg>>::Type),
    {
        self.iter_impl().for_each(f)
    }
    fn boxed_iter(
        &self,
    ) -> Box<dyn '_ + Iterator<Item = <TypeTag as ItemTypeForTag<'slice, 'msg>>::Type>> {
        Box::new(self.iter_impl())
    }

    type Iter<'this> = impl Iterator<Item = <TypeTag as ItemTypeForTag<'slice, 'msg>>::Type>;
    fn iter(&self) -> Self::Iter<'_> {
        self.iter_impl()
    }
}
/*
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
*/
