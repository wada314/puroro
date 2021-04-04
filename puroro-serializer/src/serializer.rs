use crate::{
    types::WireType,
    variant::{Variant, VariantType},
};
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
        iter: I,
    ) -> Result<()>;

    fn serialize_message<T: Serializable>(
        &mut self,
        field_number: usize,
        message: &T,
    ) -> Result<()>;
    fn serialize_messages<'a, T: 'a + Serializable, I: Iterator<Item = Result<&'a T>>>(
        &mut self,
        field_number: usize,
        iter: I,
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
    fn write_field_number_and_wire_type(
        &mut self,
        field_number: usize,
        wire_type: WireType,
    ) -> Result<()> {
        let combined: usize = (field_number << 3) | (wire_type as usize);
        let variant = crate::variant::RustUsize::to_variant(combined)?;
        variant.encode_bytes(&mut self.write)?;
        Ok(())
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
        self.write_field_number_and_wire_type(field_number, WireType::Variant)?;
        let variant = T::to_variant(value)?;
        variant.encode_bytes(&mut self.write)?;
        Ok(())
    }

    fn serialize_variants<T: VariantType, I: Iterator<Item = Result<T::NativeType>>>(
        &mut self,
        field_number: usize,
        iter: I,
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
        iter: I,
    ) -> Result<()> {
        todo!()
    }
}

struct CounterWrite(usize);
impl CounterWrite {
    fn new() -> Self {
        Self(0)
    }
    fn count(&self) -> usize {
        self.0
    }
}
impl Write for CounterWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0 += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
