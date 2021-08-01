mod deser_iter;
mod field_wrapper;
mod ser_io_write;
mod type_gen;

use crate::{ChooseStructVisibility, StructInternalTypeGen};
use ::puroro::{tags, RepeatedField};
use ::std::borrow::Borrow;
use ::std::borrow::Cow;
use ::std::marker::PhantomData;
pub use field_wrapper::{LabelWrappedLdType, LabelWrappedMessageType, LabelWrappedType};

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

pub struct RepeatedFieldImplForNonLdTypes<'msg, T>(&'msg Vec<T>);
pub struct RepeatedFieldImplForLdTypes<'msg, B: ?Sized + ToOwned>(&'msg Vec<<B as ToOwned>::Owned>);
pub struct CowedIter<B: ?Sized, Iter>(Iter, PhantomData<B>);

impl<'msg, T: Clone> IntoIterator for RepeatedFieldImplForNonLdTypes<'msg, T> {
    type Item = T;
    type IntoIter = ::std::iter::Cloned<::std::slice::Iter<'msg, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}
impl<'msg, T: Clone> RepeatedField<'msg, T> for RepeatedFieldImplForNonLdTypes<'msg, T> {}

impl<'msg, B: ?Sized + 'msg + ToOwned> IntoIterator for RepeatedFieldImplForLdTypes<'msg, B> {
    type Item = Cow<'msg, B>;
    type IntoIter = CowedIter<B, ::std::slice::Iter<'msg, <B as ToOwned>::Owned>>;
    fn into_iter(self) -> Self::IntoIter {
        CowedIter(self.0.iter(), PhantomData)
    }
}
impl<'msg, B: ?Sized + 'msg + ToOwned> RepeatedField<'msg, Cow<'msg, B>>
    for RepeatedFieldImplForLdTypes<'msg, B>
{
}
impl<'msg, B, Iter> Iterator for CowedIter<B, Iter>
where
    B: ?Sized + 'msg + ToOwned,
    Iter: Iterator<Item = &'msg <B as ToOwned>::Owned>,
{
    type Item = Cow<'msg, B>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|owned| Cow::Borrowed(owned.borrow()))
    }
}
