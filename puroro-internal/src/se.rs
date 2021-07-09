pub mod to_io_write;
use crate::{FieldTypeGen, Result, StructInternalTypeGen};

pub trait SerFieldToIoWrite<LabelAndType>:
    FieldTypeGen<LabelAndType> + StructInternalTypeGen
{
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<LabelAndType>>::Type,
        field_number: i32,
        out: &mut W,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: ::std::io::Write;
}

pub trait SerInternalDataToIoWrite: StructInternalTypeGen {
    fn ser_to_io_write<W>(
        out: &mut W,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: ::std::io::Write;
}
