pub mod field_clone;
pub mod field_deser;
pub mod field_new;
pub mod field_ser;
pub mod field_take_or_init;
pub mod repeated_slice_view;
use std::borrow::Borrow;
use std::convert::TryFrom;

pub use field_clone::FieldClone;
pub use field_deser::{FieldDeserFromIter, FieldDeserFromSlice};
pub use field_new::FieldNew;
pub use field_ser::FieldSer;
pub use field_take_or_init::FieldTakeOrInit;
use puroro::Message;
pub use repeated_slice_view::RepeatedSliceViewField;

use crate::ser::MessageSerializer;
use crate::tags;
use crate::Result;

pub trait DoDefaultCheck {
    const DO_DEFAULT_CHECK: bool = false;
}
impl DoDefaultCheck for tags::Optional3 {
    const DO_DEFAULT_CHECK: bool = true;
}
impl DoDefaultCheck for tags::Required {}
impl DoDefaultCheck for tags::Optional2 {}
impl DoDefaultCheck for tags::Repeated {}

pub trait MapEntryForNormalImpl {
    type OwnedKeyType;
    type OwnedValueType;
    fn into_tuple(self) -> (Self::OwnedKeyType, Self::OwnedValueType);
    fn ser_kv<T, Q, R>(key: &Q, value: &R, serializer: &mut T) -> Result<()>
    where
        T: MessageSerializer,
        Self::OwnedKeyType: Borrow<Q>,
        Self::OwnedValueType: Borrow<R>;
}

pub trait MapEntryForSliceViewImpl<'slice> {
    type OwnedKeyType;
    type ValueGetterType;
    fn key_eq<Q>(&self, key: &Q) -> bool
    where
        Self::OwnedKeyType: Borrow<Q>,
        Q: Eq + ?Sized;
    fn value(&self) -> Self::ValueGetterType;
}

/// An alternative for `std::default::Default` just only because of `Result<Enum, i32>`.
trait Default {
    fn default() -> Self;
}
macro_rules! impl_default {
    ($ty:ty) => {
        impl Default for $ty {
            fn default() -> Self {
                ::std::default::Default::default()
            }
        }
    };
}
impl_default!(i32);
impl_default!(i64);
impl_default!(u32);
impl_default!(u64);
impl_default!(f32);
impl_default!(f64);
impl_default!(bool);
impl_default!(String);
impl_default!(Vec<u8>);
impl<T: TryFrom<i32, Error = i32>> Default for ::std::result::Result<T, i32> {
    fn default() -> Self {
        T::try_from(0i32)
    }
}
pub trait WrappedFieldType<LabelTag>
where
    LabelTag: tags::FieldLabelTag,
{
    type Item;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item;
    fn as_slice(&self) -> &[Self::Item];
}
impl<T> WrappedFieldType<tags::Required> for T {
    type Item = T;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = item;
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, _: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
    fn as_slice(&self) -> &[Self::Item] {
        std::slice::from_ref(self)
    }
}
impl<T> WrappedFieldType<tags::Optional2> for Option<T> {
    type Item = T;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = Some(item);
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.get_or_insert_with(f)
    }
    fn as_slice(&self) -> &[Self::Item] {
        match self {
            Some(x) => std::slice::from_ref(x),
            None => &[],
        }
    }
}
impl<T> WrappedFieldType<tags::Optional3> for T {
    type Item = T;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = item;
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, _: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
    fn as_slice(&self) -> &[Self::Item] {
        std::slice::from_ref(self)
    }
}
impl<VT> WrappedFieldType<tags::Repeated> for VT
where
    VT: VecType,
{
    type Item = VT::Item;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        for ritem in iter {
            self.push(ritem?);
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.push((f)());
        self.last_mut().unwrap()
    }
    fn as_slice(&self) -> &[Self::Item] {
        <Self as VecType>::as_slice(self)
    }
}

/// Need a special treat for Message type fields because:
///  * The wrapper type rule is diffirent with the ordinary fields:
///    * `required` ==> `T`
///    * `optional` (proto2) ==> `Option<Box<T>>`
///    * `optional` (proto3) ==> `Option<Box<T>>`
///    * _(unlabeled)_ (proto3) ==> `Option<Box<T>>`
///    * `repeated` ==> `Vec<T>`
///  * Need to handle the map fields.
///
pub trait WrappedMessageFieldType<MessageType, LabelTag>
where
    MessageType: Message,
    LabelTag: tags::FieldLabelTag,
{
    type Item: Message;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item;
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
}
impl<M> WrappedMessageFieldType<M, tags::Required> for M
where
    M: Message,
{
    type Item = M;
    fn get_or_insert_with<F>(&mut self, _: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        (f)(self)
    }
}
impl<M> WrappedMessageFieldType<M, tags::Optional2> for Option<M::BoxedType>
where
    M: Message,
    M::BoxedType: AsMut<M> + AsRef<M>,
{
    type Item = M;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.get_or_insert_with(|| (f)().into_boxed()).as_mut()
    }
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        if let Some(boxed) = self {
            (f)(boxed.as_ref())
        } else {
            Ok(())
        }
    }
}
impl<T> WrappedMessageFieldType<T, tags::Optional3> for Option<T::BoxedType>
where
    T: Message,
    T::BoxedType: AsMut<T> + AsRef<T>,
{
    type Item = T;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.get_or_insert_with(|| (f)().into_boxed()).as_mut()
    }
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        if let Some(boxed) = self {
            (f)(boxed.as_ref())
        } else {
            Ok(())
        }
    }
}
impl<T, VT> WrappedMessageFieldType<T, tags::Repeated> for VT
where
    T: Message,
    T::BoxedType: AsMut<T> + AsRef<T>,
    VT: VecType<Item = T>,
{
    type Item = T;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        <Self as VecType>::push(self, (f)());
        <Self as VecType>::last_mut(self).unwrap()
    }
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        for item in self.as_slice() {
            (f)(item)?;
        }
        Ok(())
    }
}

pub trait VecType {
    type Item;
    fn len(&self) -> usize;
    fn push(&mut self, item: Self::Item);
    fn last_mut(&mut self) -> Option<&mut Self::Item>;
    fn clear(&mut self);
    fn reserve(&mut self, bytes_len: usize);
    fn as_slice(&self) -> &[Self::Item];
}
impl<T> VecType for Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        <Vec<Self::Item>>::len(self)
    }
    fn push(&mut self, item: Self::Item) {
        <Vec<Self::Item>>::push(self, item)
    }
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        <[Self::Item]>::last_mut(self)
    }
    fn clear(&mut self) {
        <Vec<Self::Item>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <Vec<Self::Item>>::reserve(self, bytes_len)
    }
    fn as_slice(&self) -> &[Self::Item] {
        <Self as AsRef<[Self::Item]>>::as_ref(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump, T> VecType for ::bumpalo::collections::Vec<'bump, T> {
    type Item = T;
    fn len(&self) -> usize {
        <::bumpalo::collections::Vec<'bump, Self::Item>>::len(self)
    }
    fn push(&mut self, item: Self::Item) {
        <::bumpalo::collections::Vec<'bump, Self::Item>>::push(self, item)
    }
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        <[Self::Item]>::last_mut(self)
    }
    fn clear(&mut self) {
        <::bumpalo::collections::Vec<'bump, Self::Item>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <::bumpalo::collections::Vec<'bump, Self::Item>>::reserve(self, bytes_len)
    }
    fn as_slice(&self) -> &[Self::Item] {
        <Self as AsRef<[Self::Item]>>::as_ref(self)
    }
}

pub trait StringType {
    fn len(&self) -> usize;
    fn as_bytes(&self) -> &[u8];
    fn push(&mut self, c: char);
    fn clear(&mut self);
    fn reserve(&mut self, bytes_len: usize);
}
impl StringType for String {
    fn len(&self) -> usize {
        <String>::len(self)
    }
    fn as_bytes(&self) -> &[u8] {
        <String>::as_bytes(self)
    }
    fn push(&mut self, c: char) {
        <String>::push(self, c)
    }
    fn clear(&mut self) {
        <String>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <String>::reserve(self, bytes_len)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> StringType for ::bumpalo::collections::String<'bump> {
    fn len(&self) -> usize {
        <::bumpalo::collections::String<'bump>>::len(self)
    }
    fn as_bytes(&self) -> &[u8] {
        <::bumpalo::collections::String<'bump>>::as_bytes(self)
    }
    fn push(&mut self, c: char) {
        <::bumpalo::collections::String<'bump>>::push(self, c)
    }
    fn clear(&mut self) {
        <::bumpalo::collections::String<'bump>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <::bumpalo::collections::String<'bump>>::reserve(self, bytes_len)
    }
}
