pub mod to_io_write;
use ::puroro::SerToIoWrite;

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
pub trait SerEnumToIoWriteProxy<X, L>: EnumTypeGen<X, L> + StructInternalTypeGen {
    type SerEnum<E>: SerEnumToIoWrite<X, L, E>
    where
        E: PartialEq;
}
pub trait SerEnumToIoWrite<X, L, E>: EnumTypeGen<X, L> + StructInternalTypeGen
where
    E: PartialEq,
{
    fn ser_to_io_write<W>(
        field: &<Self as EnumTypeGen<X, L>>::EnumType<E>,
        field_number: i32,
        out: &mut W,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        W: ::std::io::Write;
}
pub trait SerMsgToIoWriteProxy<X, L>: MsgTypeGen<X, L> + StructInternalTypeGen {
    type SerMsg<M>: SerMsgToIoWrite<X, L, M>
    where
        M: SerToIoWrite;
}
pub trait SerMsgToIoWrite<X, L, M>: MsgTypeGen<X, L> + StructInternalTypeGen {
    fn ser_to_io_write<W>(
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
