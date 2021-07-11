pub mod to_io_write;
use crate::{EnumTypeGen, FieldTypeGen, MsgTypeGen, Result, StructInternalTypeGen};

pub trait SerFieldToIoWrite<X, L, V>: FieldTypeGen<X, L, V> + StructInternalTypeGen {
    fn ser_to_io_write<W>(
        field: &<Self as FieldTypeGen<X, L, V>>::Type,
        field_number: i32,
        out: &mut W,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: ::std::io::Write;
}
pub trait SerEnumToIoWrite<X, L>: EnumTypeGen<X, L> + StructInternalTypeGen {
    fn ser_to_io_write<W, E>(
        field: &<Self as EnumTypeGen<X, L>>::EnumType<E>,
        field_number: i32,
        out: &mut W,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: ::std::io::Write;
}
pub trait SerMsgToIoWrite<X, L>: MsgTypeGen<X, L> + StructInternalTypeGen {
    fn ser_to_io_write<W, M>(
        field: &<Self as MsgTypeGen<X, L>>::MsgType<M>,
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
