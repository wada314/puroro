use crate::types::FieldData;
use crate::Result;
use from_iter::ScopedIter;

pub mod from_iter;

pub trait DeserFieldsFromBytesIter {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
impl<T> DeserFieldsFromBytesIter for Box<T>
where
    T: DeserFieldsFromBytesIter,
{
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        Box::as_mut(self).deser_field(field_number, data)
    }
}
