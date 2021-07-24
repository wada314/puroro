use crate::de::from_iter::ScopedIter;
use crate::{
    EnumTypeGen, FieldTypeGen, MessageInternal, MsgTypeGen, Result, StructInternalTypeGen,
};
use ::puroro::tags;
use ::puroro::types::FieldData;
use ::std::convert::TryFrom;

pub mod from_iter;

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
    // i.e. Cannot add `Self: EnumTypeGen<..., EnumType<E> = Something>`
    type DeserEnum<E>: DeserEnumFromBytesIter<X, L, E>
    where
        E: Default + TryFrom<i32> + PartialEq;
}
pub trait DeserEnumFromBytesIter<X, L, E>: EnumTypeGen<X, L> + StructInternalTypeGen
where
    E: PartialEq,
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as EnumTypeGen<X, L>>::EnumType<E>,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
pub trait DeserMsgFromBytesIterProxy<X, L>: MsgTypeGen<X, L> + StructInternalTypeGen {
    // We need to proxy this associated type once before calling the method
    // because in *trait method* implementation we cannot add extra
    // constraints to trait bounds:
    // i.e. Cannot add `Self: EnumTypeGen<..., EnumType<E> = Something>`
    type DeserMessage<M>: DeserMsgFromBytesIter<X, L, M>
    where
        M: MessageFromBytesIter + MessageInternal<ImplTypeTag = Self>;
}
pub trait DeserMsgFromBytesIter<X, L, M>: MsgTypeGen<X, L> + StructInternalTypeGen {
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as MsgTypeGen<X, L>>::MsgType<M>,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
