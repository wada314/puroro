use crate::Result;
use crate::{
    types::WireType,
    variant::{RustUsize, VariantType},
};
use std::io::{Result as IoResult, Write};

pub trait MessageSerializer {
    fn serialize_variant<T: VariantType>(
        &mut self,
        field_number: usize,
        value: T::NativeType,
    ) -> Result<()>;

    fn serialize_variants_twice<T, I>(&mut self, field_number: usize, iter: I) -> Result<()>
    where
        T: VariantType,
        I: Clone + Iterator<Item = Result<T::NativeType>>;

    fn serialize_bytes_twice<I>(&mut self, field_number: usize, bytes: I) -> Result<()>
    where
        I: Clone + Iterator<Item = Result<u8>>;

    fn serialize_message_twice<T: Serializable>(
        &mut self,
        field_number: usize,
        message: &T,
    ) -> Result<()>;

    fn direct_write_field<I>(
        &mut self,
        field_number: usize,
        wire_type: WireType,
        length: usize,
        input: I,
    ) -> Result<()>
    where
        I: Iterator<Item = IoResult<u8>>;
}
pub fn default_serializer<'a, W>(write: &'a mut W) -> impl MessageSerializer + 'a
where
    W: std::io::Write,
{
    MessageSerializerImpl::new(write)
}

pub trait Serializable {
    fn serialize<T: MessageSerializer>(&self, serializer: &mut T) -> Result<()>;
}
struct MessageSerializerImpl<'a, W>
where
    W: std::io::Write,
{
    write: &'a mut W,
}

impl<'a, W> MessageSerializerImpl<'a, W>
where
    W: std::io::Write,
{
    fn new(write: &'a mut W) -> Self {
        Self { write }
    }
    fn get_write(&self) -> &W {
        &self.write
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
impl<'a, W> MessageSerializer for MessageSerializerImpl<'a, W>
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

    fn serialize_bytes_twice<I>(&mut self, field_number: usize, bytes: I) -> Result<()>
    where
        I: Clone + Iterator<Item = Result<u8>>,
    {
        self.write_field_number_and_wire_type(field_number, WireType::LengthDelimited)?;
        let length = bytes.clone().count();
        RustUsize::to_variant(length)?.encode_bytes(&mut self.write)?;
        for byte in bytes {
            self.write.write_all(std::slice::from_ref(&byte?))?;
        }
        Ok(())
    }

    fn serialize_message_twice<T: Serializable>(
        &mut self,
        field_number: usize,
        message: &T,
    ) -> Result<()> {
        let mut counter = CounterWrite::new();
        let mut length_counting_serializer = MessageSerializerImpl::new(&mut counter);
        message.serialize(&mut length_counting_serializer)?;

        self.write_field_number_and_wire_type(field_number, WireType::LengthDelimited)?;
        let length = length_counting_serializer.get_write().count();
        RustUsize::to_variant(length)?.encode_bytes(&mut self.write)?;
        message.serialize(self)?;

        Ok(())
    }

    fn direct_write_field<I>(
        &mut self,
        field_number: usize,
        wire_type: WireType,
        length: usize,
        input: I,
    ) -> Result<()>
    where
        I: Iterator<Item = IoResult<u8>>,
    {
        self.write_field_number_and_wire_type(field_number, wire_type)?;
        RustUsize::to_variant(length)?.encode_bytes(&mut self.write)?;
        for rbyte in input {
            self.write.write_all(std::slice::from_ref(&rbyte?))?;
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
