mod deser_iter;
mod field_wrapper;
mod ser_io_write;
mod type_gen;

use crate::{ChooseStructVisibility, StructInternalTypeGen};
use ::puroro::{tags, RepeatedField};
use ::std::borrow::Cow;
pub use field_wrapper::{LabelWrappedLdType, LabelWrappedMessageType, LabelWrappedType};
use std::borrow::Borrow;

pub struct SimpleImpl;
impl tags::ImplTypeTag for SimpleImpl {}

impl<Public, Private> ChooseStructVisibility<Public, Private> for SimpleImpl {
    type Type = Public;
}

// Struct's internal type generator
impl StructInternalTypeGen for SimpleImpl {
    // TODO
    type Type = ();
}

struct RepeatedFieldImplForNonLdTypes<'msg, T>(&'msg Vec<T>);
struct RepeatedFieldImplForLdTypes<'msg, T>(&'msg Vec<T>);
struct CowedIter<B, Iter>(Iter);

impl<'msg, T> IntoIterator for RepeatedFieldImplForNonLdTypes<'msg, T> {
    type Item = T;
    type IntoIter = ::std::iter::Cloned<::std::slice::Iter<'msg, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}
impl<'msg, T> RepeatedField<'msg, T> for RepeatedFieldImplForLdTypes<'msg, T> {}
impl<'msg, B> IntoIterator for RepeatedFieldImplForLdTypes<'msg, <B as ToOwned>::Owned> {
    type Item = Cow<'msg, B>;
    type IntoIter = CowedIter<B, ::std::slice::Iter<'msg, <B as ToOwned>::Owned>>;
    fn into_iter(self) -> Self::IntoIter {
        CowedIter(self.0.iter())
    }
}
impl<'msg, B> RepeatedField<'msg, Cow<'msg, B>>
    for RepeatedFieldImplForLdTypes<'msg, <B as ToOwned>::Owned>
{
}
impl<'msg, B, Iter> Iterator for CowedIter<B, Iter>
where
    Iter: Iterator<Item = &'msg <B as ToOwned>::Owned>,
{
    type Item = Cow<'msg, B>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|owned| Cow::Borrowed(owned.borrow()))
    }
}
