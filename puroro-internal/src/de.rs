use crate::de::from_iter::ScopedIter;
use crate::{EnumTypeGen, FieldTypeGen, MsgTypeGen, Result, StructInternalTypeGen};
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

pub trait DeserEnumFromBytesIter<X, L>: EnumTypeGen<X, L> + StructInternalTypeGen {
    fn deser_from_scoped_bytes_iter<I, E>(
        field: &mut <Self as EnumTypeGen<X, L>>::EnumType<E>,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
        E: Default + TryFrom<i32> + PartialEq;
}
pub trait DeserMsgFromBytesIter<X, L>: MsgTypeGen<X, L> + StructInternalTypeGen {
    fn deser_from_scoped_bytes_iter<I, M>(
        field: &mut <Self as MsgTypeGen<X, L>>::MsgType<M>,
        data: FieldData<&mut ScopedIter<I>>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
