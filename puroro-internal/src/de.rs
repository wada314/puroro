use crate::de::from_iter::ScopedIter;
use crate::Result;
use ::puroro::types::FieldData;

pub mod from_iter;
pub trait MessageFromBytesIter: ::puroro::DeserFromBytesIter {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserFieldFromBytesIter<X, L, V> {
    fn deser_from_scoped_bytes_iter<I, FieldType, InternalDataType>(
        field: &mut FieldType,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &InternalDataType,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
