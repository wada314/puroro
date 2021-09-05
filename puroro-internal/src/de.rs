use crate::de::from_iter::ScopedIter;
use crate::Result;
use ::puroro::types::FieldData;

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
