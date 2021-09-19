use crate::internal::se::to_io_write::write_field_number_and_wire_type;
use crate::types::WireType;
use crate::variant::Variant;
use crate::SerializableMessageToIoWrite;
use crate::{tags, Result};
use ::std::convert::TryInto;
use ::std::io::Write;
use ::std::marker::PhantomData;
use ::std::ops::Deref;

use super::super::simple::VecOrOptionOrBare;

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
        FieldType: VecOrOptionOrBare<BytesType>,
        BytesType: Deref<Target = [u8]>,
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
        FieldType: VecOrOptionOrBare<StringType>,
        StringType: Deref<Target = str>,
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

impl<M, _1, _2, _3> SerFieldToIoWrite<tags::LabelNonRepeated<_1, _2, _3>, tags::Message<M>>
where
    M: SerializableMessageToIoWrite,
{
    pub fn ser_field<MessageType, FieldType, W>(
        field: &FieldType,
        number: i32,
        out: &mut W,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<MessageType>,
        MessageType: Deref<Target = M>,
        W: Write,
    {
        for item in field.iter() {
            let len = {
                let mut null_out = NullWrite::new();
                <M as SerializableMessageToIoWrite>::ser(item.deref(), &mut null_out)?;
                null_out.len()
            };
            let len_i32: i32 = len
                .try_into()
                .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
            write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
            Variant::from_i32(len_i32)?.encode_bytes(out)?;
            <M as SerializableMessageToIoWrite>::ser(item.deref(), out)?;
        }
        Ok(())
    }
}

impl<M> SerFieldToIoWrite<tags::Repeated, tags::Message<M>>
where
    M: SerializableMessageToIoWrite,
{
    pub fn ser_field<MessageType, W>(
        field: &Vec<MessageType>,
        number: i32,
        out: &mut W,
    ) -> Result<()>
    where
        MessageType: Deref<Target = M>,
        W: Write,
    {
        for item in field {
            let len = {
                let mut null_out = NullWrite::new();
                <M as SerializableMessageToIoWrite>::ser(item.deref(), &mut null_out)?;
                null_out.len()
            };
            let len_i32: i32 = len
                .try_into()
                .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
            write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
            Variant::from_i32(len_i32)?.encode_bytes(out)?;
            <M as SerializableMessageToIoWrite>::ser(item.deref(), out)?;
        }
        Ok(())
    }
}
