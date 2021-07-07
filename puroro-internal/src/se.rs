pub mod to_io_write;
use crate::{FieldTypeGen, Result, StructInternalTypeGen};

pub fn ser_to_io_write<Msg, W>(message: &Msg, out: &mut W) -> Result<()> {
    todo!()
}

pub trait SerFieldToIoWrite<LabelAndType>:
    FieldTypeGen<LabelAndType> + StructInternalTypeGen
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<LabelAndType>>::Type,
        out: &mut W,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: ::std::io::Write;
}
