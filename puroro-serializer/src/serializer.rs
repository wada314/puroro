use crate::Result;
use crate::{
    types::WireType,
    variant::{RustUsize, VariantType},
};
use std::io::Write;

trait MessageSerializer {
    #[must_use]
    fn serialize_variant<T: VariantType>(
        &mut self,
        field_number: usize,
        value: T::NativeType,
    ) -> Result<()>;

    #[must_use]
    fn serialize_variants_twice<T, I>(&mut self, field_number: usize, iter: I) -> Result<()>
    where
        T: VariantType,
        I: Clone + Iterator<Item = Result<T::NativeType>>;

    #[must_use]
    fn serialize_message_twice<T: Serializable>(
        &mut self,
        field_number: usize,
        message: &T,
    ) -> Result<()>;

    #[must_use]
    fn serialize_messages_twice<'a, T, I>(&mut self, field_number: usize, iter: I) -> Result<()>
    where
        T: 'a + Serializable,
        I: Iterator<Item = Result<&'a T>>;
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
    fn get_write(&self) -> &W {
        &self.write
    }
    #[must_use]
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

    fn serialize_variants_twice<T, I>(&mut self, field_number: usize, iter: I) -> Result<()>
    where
        T: VariantType,
        I: Clone + Iterator<Item = Result<T::NativeType>>,
    {
        let iter2 = iter.clone();

        // Count the bytes length using a fake `Write`
        let mut length_counter = CounterWrite::new();
        for rvalue in iter {
            let variant = T::to_variant(rvalue?)?;
            variant.encode_bytes(&mut length_counter)?;
        }

        self.write_field_number_and_wire_type(field_number, WireType::LengthDelimited)?;
        RustUsize::to_variant(length_counter.count())?.encode_bytes(&mut self.write)?;
        // Real write.
        for rvalue in iter2 {
            let variant = T::to_variant(rvalue?)?;
            variant.encode_bytes(&mut self.write)?;
        }

        Ok(())
    }

    fn serialize_message_twice<T: Serializable>(
        &mut self,
        field_number: usize,
        message: &T,
    ) -> Result<()> {
        let mut length_counting_serializer = MessageSerializerImpl::new(CounterWrite::new());
        message.serialize(&mut length_counting_serializer)?;

        self.write_field_number_and_wire_type(field_number, WireType::LengthDelimited)?;
        let length = length_counting_serializer.get_write().count();
        RustUsize::to_variant(length)?.encode_bytes(&mut self.write)?;
        message.serialize(self)?;

        Ok(())
    }

    fn serialize_messages_twice<'a, T, I>(&mut self, field_number: usize, iter: I) -> Result<()>
    where
        T: 'a + Serializable,
        I: Iterator<Item = Result<&'a T>>,
    {
        for rmessage in iter {
            let mut length_counting_serializer = MessageSerializerImpl::new(CounterWrite::new());
            let message = rmessage?;
            message.serialize(&mut length_counting_serializer)?;
            self.write_field_number_and_wire_type(field_number, WireType::LengthDelimited)?;
            let length = length_counting_serializer.get_write().count();
            RustUsize::to_variant(length)?.encode_bytes(&mut self.write)?;
            message.serialize(self)?;
        }

        Ok(())
    }
}

// A fake implementation which is only for measuring the length of the serialized bytes.

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
