pub mod to_io_write;
use ::puroro::tags;
use ::puroro::SerToIoWrite;

use crate::{EnumTypeGen, FieldTypeGen, MsgTypeGen, Result, StructInternalTypeGen};
pub trait SerAnyFieldToIoWrite:
    SerInternalDataToIoWrite
    + SerEnumToIoWriteProxy<tags::Proto2, tags::Required>
    + SerEnumToIoWriteProxy<tags::Proto2, tags::Optional>
    + SerEnumToIoWriteProxy<tags::Proto2, tags::Repeated>
    + SerEnumToIoWriteProxy<tags::Proto3, tags::Unlabeled>
    + SerEnumToIoWriteProxy<tags::Proto3, tags::Optional>
    + SerEnumToIoWriteProxy<tags::Proto3, tags::Repeated>
    + SerMsgToIoWriteProxy<tags::Proto2, tags::Required>
    + SerMsgToIoWriteProxy<tags::Proto2, tags::Optional>
    + SerMsgToIoWriteProxy<tags::Proto2, tags::Repeated>
    + SerMsgToIoWriteProxy<tags::Proto3, tags::Unlabeled>
    + SerMsgToIoWriteProxy<tags::Proto3, tags::Optional>
    + SerMsgToIoWriteProxy<tags::Proto3, tags::Repeated>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Int32>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::UInt32>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::SInt32>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Int64>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::UInt64>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::SInt64>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Bool>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Bytes>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::String>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Float>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Double>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::SFixed32>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Fixed32>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::SFixed64>
    + SerFieldToIoWrite<tags::Proto2, tags::Required, tags::Fixed64>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Int32>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::UInt32>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::SInt32>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Int64>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::UInt64>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::SInt64>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Bool>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Bytes>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::String>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Float>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Double>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::SFixed32>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Fixed32>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::SFixed64>
    + SerFieldToIoWrite<tags::Proto2, tags::Optional, tags::Fixed64>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Int32>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::UInt32>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::SInt32>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Int64>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::UInt64>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::SInt64>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Bool>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Bytes>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::String>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Float>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Double>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::SFixed32>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Fixed32>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::SFixed64>
    + SerFieldToIoWrite<tags::Proto2, tags::Repeated, tags::Fixed64>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Int32>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::UInt32>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::SInt32>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Int64>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::UInt64>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::SInt64>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Bool>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Bytes>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::String>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Float>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Double>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::SFixed32>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Fixed32>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::SFixed64>
    + SerFieldToIoWrite<tags::Proto3, tags::Unlabeled, tags::Fixed64>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Int32>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::UInt32>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::SInt32>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Int64>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::UInt64>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::SInt64>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Bool>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Bytes>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::String>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Float>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Double>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::SFixed32>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Fixed32>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::SFixed64>
    + SerFieldToIoWrite<tags::Proto3, tags::Optional, tags::Fixed64>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Int32>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::UInt32>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::SInt32>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Int64>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::UInt64>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::SInt64>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Bool>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Bytes>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::String>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Float>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Double>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::SFixed32>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Fixed32>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::SFixed64>
    + SerFieldToIoWrite<tags::Proto3, tags::Repeated, tags::Fixed64>
{
}

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
pub trait SerEnumToIoWriteProxy<X, L>: EnumTypeGen<X, L> + StructInternalTypeGen
where
    X: tags::EnumTypeForSyntax,
{
    type SerEnum<E>: SerEnumToIoWrite<
        X,
        L,
        E,
        <Self as EnumTypeGen<X, L>>::EnumType<E>,
        <Self as StructInternalTypeGen>::Type,
    >
    where
        E: PartialEq,
        i32: From<E>,
        <X as tags::EnumTypeForSyntax>::NativeType<E>: Clone;
}
pub trait SerEnumToIoWrite<X, L, E, EnumFieldType, InternalDataType>
where
    X: tags::EnumTypeForSyntax,
    <X as tags::EnumTypeForSyntax>::NativeType<E>: Clone,
    E: PartialEq,
{
    fn ser_to_io_write<W>(
        field: &EnumFieldType,
        field_number: i32,
        out: &mut W,
        internal_data: &InternalDataType,
    ) -> Result<()>
    where
        W: ::std::io::Write;
}
pub trait SerMsgToIoWriteProxy<X, L>: MsgTypeGen<X, L> + StructInternalTypeGen {
    type SerMsg<M>: SerMsgToIoWrite<
        X,
        L,
        M,
        <Self as MsgTypeGen<X, L>>::MsgType<M>,
        <Self as StructInternalTypeGen>::Type,
    >
    where
        M: SerToIoWrite;
}
pub trait SerMsgToIoWrite<X, L, M, MsgFieldType, InternalDataType> {
    fn ser_to_io_write<W>(
        field: &MsgFieldType,
        field_number: i32,
        out: &mut W,
        internal_data: &InternalDataType,
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
