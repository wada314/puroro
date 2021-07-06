use crate::de::from_iter::ScopedIter;
use crate::Result;
use ::puroro::types::FieldData;
use ::puroro::{FieldTypeGen, StructInternalTypeGen};

pub mod from_iter;

pub trait DeserFromBytesIterInternal: ::puroro::DeserFromBytesIter {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserFieldFromBytesIter<LabelAndType>:
    FieldTypeGen<LabelAndType> + StructInternalTypeGen
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserFieldFromBytesSlice<LabelAndType>:
    FieldTypeGen<LabelAndType> + StructInternalTypeGen
{
    fn deser_from_bytes_slice(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&[u8]>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>;
}

impl<T, LabelAndType> DeserFieldFromBytesSlice<LabelAndType> for T
where
    T: DeserFieldFromBytesIter<LabelAndType>,
{
    fn deser_from_bytes_slice(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&[u8]>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()> {
        use std::io::Read as _;
        <Self as DeserFieldFromBytesIter<LabelAndType>>::deser_from_scoped_bytes_iter(
            field,
            data.map(|slice| ScopedIter::new(slice.bytes())).as_mut(),
            internal_data,
        )
    }
}
