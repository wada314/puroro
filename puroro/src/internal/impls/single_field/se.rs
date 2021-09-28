use super::VecOrOptionOrBare;
use crate::internal::se::to_io_write::write_field_number_and_wire_type;
use crate::internal::SerializableMessageToIoWrite;
use crate::types::WireType;
use crate::variant::Variant;
use crate::{tags, Result};
use ::std::convert::TryInto;
use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::ops::Deref;

struct NullWrite(usize);
impl Write for NullWrite {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        if let Some(new_size) = usize::checked_add(self.0, buf.len()) {
            self.0 = new_size;
            Ok(buf.len())
        } else {
            Err(::std::io::Error::new(
                ::std::io::ErrorKind::Unsupported,
                "Too long to serialize",
            ))
        }
    }
    fn flush(&mut self) -> ::std::io::Result<()> {
        Ok(())
    }
}
impl NullWrite {
    fn new() -> Self {
        Self(0usize)
    }
    fn len(&self) -> usize {
        self.0
    }
}

// ser to Write methods

pub struct SerFieldToIoWrite<L, V>(PhantomData<(L, V)>);

impl<L> SerFieldToIoWrite<L, tags::Bytes>
where
    L: tags::FieldLabelTag,
{
    pub fn ser_field<BytesType, FieldType, W>(
        field: &FieldType,
        number: i32,
        out: &mut W,
    ) -> Result<()>
    where
        BytesType: Deref<Target = [u8]>,
        FieldType: VecOrOptionOrBare<BytesType>,
        W: Write,
    {
        for item in field.iter() {
            if !L::DO_DEFAULT_CHECK || !item.deref().is_empty() {
                write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
                let len_i32: i32 = item
                    .len()
                    .try_into()
                    .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
                Variant::from_i32(len_i32)?.encode_bytes(out)?;
                out.write(item.deref())?;
            }
        }
        Ok(())
    }
}

impl<L> SerFieldToIoWrite<L, tags::String>
where
    L: tags::FieldLabelTag,
{
    pub fn ser_field<StringType, FieldType, W>(
        field: &FieldType,
        number: i32,
        out: &mut W,
    ) -> Result<()>
    where
        StringType: Deref<Target = str>,
        FieldType: VecOrOptionOrBare<StringType>,
        W: Write,
    {
        for item in field.iter() {
            if !L::DO_DEFAULT_CHECK || !item.deref().is_empty() {
                write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
                let len_i32: i32 = item
                    .len()
                    .try_into()
                    .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
                Variant::from_i32(len_i32)?.encode_bytes(out)?;
                out.write(item.deref().as_bytes())?;
            }
        }
        Ok(())
    }
}

impl<L, M> SerFieldToIoWrite<L, tags::Message<M>>
where
    L: tags::FieldLabelTag,
    M: SerializableMessageToIoWrite,
{
    pub fn ser_field<Unused, FieldType, W>(
        field: &FieldType,
        number: i32,
        out: &mut W,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<M>,
        W: Write,
    {
        for item in field.iter() {
            let len = {
                let mut null_out = NullWrite::new();
                <M as SerializableMessageToIoWrite>::ser(item, &mut null_out)?;
                null_out.len()
            };
            let len_i32: i32 = len
                .try_into()
                .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
            write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
            Variant::from_i32(len_i32)?.encode_bytes(out)?;
            <M as SerializableMessageToIoWrite>::ser(item, out)?;
        }
        Ok(())
    }
}
