use crate::variant::{Variant, VariantType};
use crate::{PuroroError, Result};
use std::io::Write;

trait MessageSerializer: Drop {
    fn serialize_variant<T: VariantType>(
        &mut self,
        field_number: usize,
        value: T::NativeType,
    ) -> Result<()>;
    fn serialize_variants<T: VariantType, I: Iterator<Item = Result<T::NativeType>>>(
        &mut self,
        field_number: usize,
        value: T::NativeType,
    ) -> Result<()>;

    fn serialize_message<T: Serializable>(
        &mut self,
        field_number: usize,
        message: &T,
    ) -> Result<()>;
    fn serialize_messages<'a, T: 'a + Serializable, I: Iterator<Item = Result<&'a T>>>(
        &mut self,
        field_number: usize,
        messages: I,
    ) -> Result<()>;
}

trait Serializable {
    fn serialize<T: MessageSerializer>(&self, serializer: &mut T) -> Result<()>;
}
struct MessageSerializerImpl<W>
where
    W: std::io::Write,
{
    write: W,
}

impl<W> MessageSerializerImpl<W>
where
    W: std::io::Write,
{
    fn new(write: W) -> Self {
        Self { write }
    }
}
impl<W> Drop for MessageSerializerImpl<W>
where
    W: std::io::Write,
{
    fn drop(&mut self) {
        todo!()
    }
}
impl<W> MessageSerializer for MessageSerializerImpl<W>
where
    W: std::io::Write,
{
    fn serialize_variant<T: VariantType>(
        &mut self,
        field_number: usize,
        value: T::NativeType,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_variants<T: VariantType, I: Iterator<Item = Result<T::NativeType>>>(
        &mut self,
        field_number: usize,
        value: T::NativeType,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_message<T: Serializable>(
        &mut self,
        field_number: usize,
        message: &T,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_messages<'a, T: 'a + Serializable, I: Iterator<Item = Result<&'a T>>>(
        &mut self,
        field_number: usize,
        messages: I,
    ) -> Result<()> {
        todo!()
    }
}
