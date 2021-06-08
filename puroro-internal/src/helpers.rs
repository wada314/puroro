pub mod field_clone;
pub mod field_merge_from_iter;
pub mod field_merge_from_slice;
pub mod field_new;
pub mod field_ser;
pub mod field_take_or_init;
pub mod repeated_slice_view;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::DerefMut;

pub use field_clone::FieldClone;
pub use field_merge_from_iter::FieldMergeFromIter;
pub use field_merge_from_slice::FieldMergeFromSlice;
pub use field_new::FieldNew;
pub use field_ser::FieldSer;
pub use field_take_or_init::FieldTakeOrInit;
use puroro::Message;
pub use repeated_slice_view::RepeatedSliceViewField;

use crate::bumpalo;
use crate::deser::DeserializableMessageFromIter;
use crate::deser::LdIter;
use crate::ser::MessageSerializer;
use crate::tags;
use crate::types::FieldData;
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
pub trait Default {
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
    fn len(&self) -> usize;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item;
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
    fn as_slice(&self) -> &[Self::Item];
}
impl<T> WrappedFieldType<tags::Required> for T {
    type Item = T;
    fn len(&self) -> usize {
        1
    }
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
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        (f)(self)
    }
}
impl<T> WrappedFieldType<tags::Optional2> for Option<T> {
    type Item = T;
    fn len(&self) -> usize {
        match self {
            Some(_) => 1,
            None => 0,
        }
    }
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
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        if let Some(item) = self {
            (f)(item)?;
        }
        Ok(())
    }
}
impl<T> WrappedFieldType<tags::Optional3> for T {
    type Item = T;
    fn len(&self) -> usize {
        1
    }
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
    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        (f)(self)
    }
}
impl<VT> WrappedFieldType<tags::Repeated> for VT
where
    VT: VecType,
{
    type Item = VT::Item;
    fn len(&self) -> usize {
        <Self as VecType>::len(self)
    }
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
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <Self as VecType>::try_for_each(self, f)
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

struct MapEntryWrapper<'a, T, M>
where
    T: crate::MapEntry<KeyType = M::Key, ValueType = M::Value>,
    M: MapType,
{
    entry: T,
    map: &'a mut M,
}
impl<'a, T, M> DeserializableMessageFromIter for MapEntryWrapper<'a, T, M>
where
    T: crate::MapEntry<KeyType = M::Key, ValueType = M::Value> + DeserializableMessageFromIter,
    M: MapType,
{
    fn met_field<'b, I>(
        &mut self,
        field: FieldData<&'b mut LdIter<I>>,
        field_number: usize,
    ) -> Result<bool>
    where
        I: Iterator<Item = std::io::Result<u8>>,
    {
        self.entry.met_field(field, field_number)
    }
}
impl<'a, T, M> Drop for MapEntryWrapper<'a, T, M>
where
    T: crate::MapEntry<KeyType = M::Key, ValueType = M::Value>,
    M: MapType,
{
    fn drop(&mut self) {
        let (key, value) = self.entry.take_kv();
        self.map.insert(key, value);
    }
}

pub trait VecType {
    type Item;
    fn len(&self) -> usize;
    fn push(&mut self, item: Self::Item);
    fn last_mut(&mut self) -> Option<&mut Self::Item>;
    fn as_slice(&self) -> &[Self::Item];
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
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
    fn as_slice(&self) -> &[Self::Item] {
        <Self as AsRef<[Self::Item]>>::as_ref(self)
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <std::slice::Iter<'_, Self::Item> as Iterator>::try_for_each(
            &mut <[Self::Item]>::iter(self),
            f,
        )
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump, T> VecType for crate::bumpalo::collections::Vec<'bump, T> {
    type Item = T;
    fn len(&self) -> usize {
        <bumpalo::collections::Vec<'bump, Self::Item>>::len(self)
    }
    fn push(&mut self, item: Self::Item) {
        <bumpalo::collections::Vec<'bump, Self::Item>>::push(self, item)
    }
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        <[Self::Item]>::last_mut(self)
    }
    fn as_slice(&self) -> &[Self::Item] {
        <Self as AsRef<[Self::Item]>>::as_ref(self)
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <std::slice::Iter<'_, Self::Item> as Iterator>::try_for_each(
            &mut <[Self::Item]>::iter(self),
            f,
        )
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
impl<'bump> StringType for crate::bumpalo::collections::String<'bump> {
    fn len(&self) -> usize {
        <bumpalo::collections::String<'bump>>::len(self)
    }
    fn as_bytes(&self) -> &[u8] {
        <bumpalo::collections::String<'bump>>::as_bytes(self)
    }
    fn push(&mut self, c: char) {
        <bumpalo::collections::String<'bump>>::push(self, c)
    }
    fn clear(&mut self) {
        <bumpalo::collections::String<'bump>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <bumpalo::collections::String<'bump>>::reserve(self, bytes_len)
    }
}

pub trait BytesType {
    fn len(&self) -> usize;
    fn push(&mut self, byte: u8);
    fn clear(&mut self);
    fn reserve(&mut self, bytes_len: usize);
    fn as_slice(&self) -> &[u8];
}
impl BytesType for Vec<u8> {
    fn len(&self) -> usize {
        <Vec<u8>>::len(self)
    }
    fn push(&mut self, byte: u8) {
        <Vec<u8>>::push(self, byte)
    }
    fn clear(&mut self) {
        <Vec<u8>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <Vec<u8>>::reserve(self, bytes_len)
    }
    fn as_slice(&self) -> &[u8] {
        <Vec<u8>>::as_slice(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> BytesType for crate::bumpalo::collections::Vec<'bump, u8> {
    fn len(&self) -> usize {
        <bumpalo::collections::Vec<'bump, u8>>::len(self)
    }
    fn push(&mut self, byte: u8) {
        <bumpalo::collections::Vec<'bump, u8>>::push(self, byte)
    }
    fn clear(&mut self) {
        <bumpalo::collections::Vec<'bump, u8>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <bumpalo::collections::Vec<'bump, u8>>::reserve(self, bytes_len)
    }
    fn as_slice(&self) -> &[u8] {
        <bumpalo::collections::Vec<'bump, u8>>::as_slice(self)
    }
}

pub trait RepeatedMessageType<'a> {
    type Item;
    type RefMut: 'a;
    fn insert_mut(&'a mut self, item: Self::Item) -> Self::RefMut;
}

impl<'a, T> RepeatedMessageType<'a> for Vec<T>
where
    T: 'a + Message,
{
    type Item = T;
    type RefMut = &'a mut T;
    fn insert_mut(&'a mut self, item: Self::Item) -> Self::RefMut {
        self.push(item);
        self.last_mut().unwrap()
    }
}

impl<'a, K, V> RepeatedMessageType<'a> for HashMap<K, V>
where
    K: 'a,
    V: 'a,
    Entry<'a, K, V>: 'a,
{
    type Item = (K, V);
    type RefMut = Entry<'a, K, V>;
    fn insert_mut(&'a mut self, item: Self::Item) -> Self::RefMut {
        todo!()
    }
}
pub struct Entry<'a, K, V> {
    key: K,
    value: V,
    map_ref: &'a mut HashMap<K, V>,
}
impl<'a, K, V> Drop for Entry<'a, K, V> {
    
}

pub trait MapType {
    type Key;
    type Value;
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}
