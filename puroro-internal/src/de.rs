use crate::de::from_iter::ScopedIter;
use crate::{
    EnumTypeGen, FieldTypeGen, MessageInternal, MsgTypeGen, Result, StructInternalTypeGen,
};
use ::puroro::tags;
use ::puroro::types::FieldData;
use ::std::convert::TryFrom;

pub mod from_iter;

pub trait DeserAnyFieldFromBytesIter:
    DeserInternalDataFromBytesIter
    + DeserEnumFromBytesIterProxy<tags::Proto2, tags::Required>
    + DeserEnumFromBytesIterProxy<tags::Proto2, tags::Optional>
    + DeserEnumFromBytesIterProxy<tags::Proto2, tags::Repeated>
    + DeserEnumFromBytesIterProxy<tags::Proto3, tags::Unlabeled>
    + DeserEnumFromBytesIterProxy<tags::Proto3, tags::Optional>
    + DeserEnumFromBytesIterProxy<tags::Proto3, tags::Repeated>
    + DeserMsgFromBytesIterProxy<tags::Proto2, tags::Required>
    + DeserMsgFromBytesIterProxy<tags::Proto2, tags::Optional>
    + DeserMsgFromBytesIterProxy<tags::Proto2, tags::Repeated>
    + DeserMsgFromBytesIterProxy<tags::Proto3, tags::Unlabeled>
    + DeserMsgFromBytesIterProxy<tags::Proto3, tags::Optional>
    + DeserMsgFromBytesIterProxy<tags::Proto3, tags::Repeated>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Int32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::UInt32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::SInt32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Int64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::UInt64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::SInt64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Bool>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Bytes>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::String>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Float>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Double>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::SFixed32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Fixed32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::SFixed64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Required, tags::Fixed64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Int32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::UInt32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::SInt32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Int64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::UInt64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::SInt64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Bool>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Bytes>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::String>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Float>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Double>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::SFixed32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Fixed32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::SFixed64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Optional, tags::Fixed64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Int32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::UInt32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::SInt32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Int64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::UInt64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::SInt64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Bool>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Bytes>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::String>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Float>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Double>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::SFixed32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Fixed32>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::SFixed64>
    + DeserFieldFromBytesIter<tags::Proto2, tags::Repeated, tags::Fixed64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Int32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::UInt32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::SInt32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Int64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::UInt64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::SInt64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Bool>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Bytes>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::String>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Float>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Double>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::SFixed32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Fixed32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::SFixed64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Unlabeled, tags::Fixed64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Int32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::UInt32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::SInt32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Int64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::UInt64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::SInt64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Bool>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Bytes>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::String>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Float>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Double>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::SFixed32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Fixed32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::SFixed64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Optional, tags::Fixed64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Int32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::UInt32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::SInt32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Int64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::UInt64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::SInt64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Bool>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Bytes>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::String>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Float>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Double>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::SFixed32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Fixed32>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::SFixed64>
    + DeserFieldFromBytesIter<tags::Proto3, tags::Repeated, tags::Fixed64>
{
}

pub trait DoDefaultCheck {
    const VALUE: bool = false;
}
impl DoDefaultCheck for (tags::Proto3, tags::Unlabeled) {
    const VALUE: bool = true;
}
impl DoDefaultCheck for (tags::Proto3, tags::MapEntry) {
    const VALUE: bool = true;
}
impl DoDefaultCheck for (tags::Proto3, tags::Repeated) {}
impl<_1, _2> DoDefaultCheck for (tags::Proto3, tags::OptionalOrRequired<_1, _2>) {}
impl DoDefaultCheck for (tags::Proto3, tags::Oneof) {}
impl<L> DoDefaultCheck for (tags::Proto2, L) {}

pub trait MessageFromBytesIter: ::puroro::DeserFromBytesIter {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserFieldFromBytesIter<X, L, V>: FieldTypeGen<X, L, V> + StructInternalTypeGen {
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<X, L, V>>::Type,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserEnumFromBytesIterProxy<X, L>: EnumTypeGen<X, L> + StructInternalTypeGen {
    // We need to proxy this associated type once before calling the method
    // because in *trait method* implementation we cannot add extra
    // constraints to trait bounds:
    type DeserEnum<E>: DeserEnumFromBytesIter<
        X,
        L,
        E,
        <Self as EnumTypeGen<X, L>>::EnumType<E>,
        <Self as StructInternalTypeGen>::Type,
    >
    where
        E: Default + TryFrom<i32> + PartialEq + Clone;
}
pub trait DeserEnumFromBytesIter<X, L, E, EnumFieldType, InternalDataType>:
    EnumTypeGen<X, L> + StructInternalTypeGen
where
    E: PartialEq,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut EnumFieldType,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &InternalDataType,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
pub trait DeserMsgFromBytesIterProxy<X, L>: MsgTypeGen<X, L> + StructInternalTypeGen {
    // We need to proxy this associated type once before calling the method
    // because in *trait method* implementation we cannot add extra
    // constraints to trait bounds:
    type DeserMsg<M>: DeserMsgFromBytesIter<
        X,
        L,
        M,
        <Self as MsgTypeGen<X, L>>::MsgType<M>,
        <Self as StructInternalTypeGen>::Type,
    >
    where
        M: MessageFromBytesIter + MessageInternal<ImplTypeTag = Self> + Clone;
}
pub trait DeserMsgFromBytesIter<X, L, M, MsgFieldType, InternalDataType> {
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut MsgFieldType,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &InternalDataType,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserInternalDataFromBytesIter: StructInternalTypeGen {
    fn deser_from_scoped_bytes_iter<I>(
        internal_data: &mut <Self as StructInternalTypeGen>::Type,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
