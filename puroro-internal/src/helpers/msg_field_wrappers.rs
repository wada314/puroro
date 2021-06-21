use crate::deser::{LdIter, MergeableMessageFromIter};
use crate::types::FieldData;
use crate::Result;
use crate::{bumpalo, hashbrown};
use ::puroro::tags;
use puroro::{IsMessageImplOfTag, Message};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

/// Need a special treat for Message type fields because:
///  * The wrapper type rule is diffirent with the ordinary fields:
///    * `required` ==> `T`
///    * `optional` (proto2) ==> `Option<Box<T>>`
///    * `optional` (proto3) ==> `Option<Box<T>>`
///    * _(unlabeled)_ (proto3) ==> `Option<Box<T>>`
///    * `repeated` ==> `Vec<T>`
///  * Need to handle the map fields.
///
pub trait WrappedMessageFieldType<'a, MessageTag, LabelTag>
where
    LabelTag: tags::FieldLabelTag,
{
    type Item;
    type MergeableMut: DerefMut;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item;
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
}
impl<'a, MessageTag, M> WrappedMessageFieldType<'a, MessageTag, tags::Required> for M
where
    M: 'a + Message + MergeableMessageFromIter + IsMessageImplOfTag<MessageTag>,
{
    type Item = M;
    type MergeableMut = &'a mut Self::Item;
    fn get_or_insert_with<F>(&'a mut self, _: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
    fn try_for_each<F>(&'a self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        (f)(self)
    }
}
impl<'a, MessageTag, BoxedType> WrappedMessageFieldType<'a, MessageTag, tags::Optional2>
    for Option<BoxedType>
where
    BoxedType: Deref + DerefMut,
    <BoxedType as Deref>::Target: 'a
        + Message<BoxedType = BoxedType>
        + MergeableMessageFromIter
        + IsMessageImplOfTag<MessageTag>,
{
    type Item = <BoxedType as Deref>::Target;
    type MergeableMut = &'a mut Self::Item;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item,
    {
        <Option<BoxedType>>::get_or_insert_with(self, || (f)().into_boxed()).deref_mut()
    }
    fn try_for_each<F>(&'a self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        if let Some(boxed) = self {
            (f)(boxed.deref())
        } else {
            Ok(())
        }
    }
}
impl<'a, MessageTag, BoxedType> WrappedMessageFieldType<'a, MessageTag, tags::Optional3>
    for Option<BoxedType>
where
    BoxedType: Deref + DerefMut,
    <BoxedType as Deref>::Target: 'a
        + Message<BoxedType = BoxedType>
        + MergeableMessageFromIter
        + IsMessageImplOfTag<MessageTag>,
{
    type Item = <BoxedType as Deref>::Target;
    type MergeableMut = &'a mut Self::Item;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item,
    {
        <Option<BoxedType>>::get_or_insert_with(self, || (f)().into_boxed()).deref_mut()
    }
    fn try_for_each<F>(&'a self, mut f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        if let Some(boxed) = self {
            (f)(boxed.deref())
        } else {
            Ok(())
        }
    }
}
impl<'a, MessageTag, RM> WrappedMessageFieldType<'a, MessageTag, tags::Repeated> for RM
where
    <RM as RepeatedMessageType<'a>>::MessageType: Message + MergeableMessageFromIter,
    RM: RepeatedMessageType<'a>,
{
    type Item = <RM as RepeatedMessageType<'a>>::MessageType;
    type MergeableMut = <RM as RepeatedMessageType<'a>>::MergeableMut;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item,
    {
        <Self as RepeatedMessageType>::insert_mut(self, (f)())
    }
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <Self as RepeatedMessageType>::try_for_each(self, f)?;
        Ok(())
    }
}

pub trait RepeatedMessageType<'a> {
    type MessageType;
    type MergeableMut: DerefMut;
    fn insert_mut(&'a mut self, item: Self::MessageType) -> Self::MergeableMut;
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Self::MessageType) -> Result<()>;
}

impl<'a, Msg> RepeatedMessageType<'a> for Vec<Msg>
where
    Msg: 'a + Message + MergeableMessageFromIter,
{
    type MessageType = Msg;
    type MergeableMut = &'a mut Msg;
    fn insert_mut(&'a mut self, item: Msg) -> Self::MergeableMut {
        self.push(item);
        self.last_mut().unwrap()
    }
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>,
    {
        <std::slice::Iter<'a, Msg> as Iterator>::try_for_each(&mut <[Msg]>::iter(self), f)?;
        Ok(())
    }
}

#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, Msg> RepeatedMessageType<'a> for bumpalo::collections::Vec<'bump, Msg>
where
    Msg: 'a + Message + MergeableMessageFromIter,
{
    type MessageType = Msg;
    type MergeableMut = &'a mut Msg;
    fn insert_mut(&'a mut self, item: Msg) -> Self::MergeableMut {
        self.push(item);
        self.last_mut().unwrap()
    }
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>,
    {
        <std::slice::Iter<Msg> as Iterator>::try_for_each(&mut <[Msg]>::iter(self), f)?;
        Ok(())
    }
}

/*
impl<'a, K, V, Msg> RepeatedMessageType<'a> for HashMap<K, V>
where
    K: Eq + Hash,
    Msg: 'a + Message + crate::MapEntry<KeyType = K, ValueType = V> + MergeableMessageFromIter,
    Self: 'a,
{
    type MessageType = Msg;
    type MergeableMut = MapEntryWrapper<'a, Msg, Self>;
    fn insert_mut(&'a mut self, item: Msg) -> Self::MergeableMut {
        MapEntryWrapper {
            entry: item,
            map: self,
        }
    }

    fn try_for_each<F>(&'a self, mut f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>,
    {
        self.iter()
            .map(|(k, v)| Msg::from_kv(k, v))
            .try_for_each(move |msg| -> Result<_> { (f)(&msg) })?;
        Ok(())
    }
}

#[cfg(feature = "puroro-bumpalo")]
impl<'a, 'bump, K, V, Msg> RepeatedMessageType<'a>
    for hashbrown::HashMap<
        K,
        V,
        hashbrown::hash_map::DefaultHashBuilder,
        hashbrown::BumpWrapper<'bump>,
    >
where
    K: Eq + Hash,
    Msg: 'a + Message + crate::MapEntry<KeyType = K, ValueType = V> + MergeableMessageFromIter,
    Self: 'a,
{
    type MessageType = Msg;
    type MergeableMut
    where
        Self: 'a,
    = MapEntryWrapper<'a, Msg, Self>;
    fn insert_mut(&'a mut self, item: Msg) -> Self::MergeableMut {
        MapEntryWrapper {
            entry: item,
            map: self,
        }
    }

    fn try_for_each<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>,
    {
        self.iter()
            .map(|(k, v)| Msg::from_kv(k, v))
            .try_for_each(move |msg| -> Result<_> { (f)(&msg) })?;
        Ok(())
    }
}
*/
pub struct MapEntryWrapper<'a, Msg, Map>
where
    Msg: crate::MapEntry<KeyType = Map::Key, ValueType = Map::Value>,
    Map: MapType,
{
    entry: Msg,
    map: &'a mut Map,
}
impl<'a, Msg, Map> Deref for MapEntryWrapper<'a, Msg, Map>
where
    Msg: crate::MapEntry<KeyType = Map::Key, ValueType = Map::Value>,
    Map: MapType,
{
    type Target = Self;
    fn deref(&self) -> &Self::Target {
        self
    }
}
impl<'a, Msg, Map> DerefMut for MapEntryWrapper<'a, Msg, Map>
where
    Msg: crate::MapEntry<KeyType = Map::Key, ValueType = Map::Value>,
    Map: MapType,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
impl<'a, Msg, Map> MergeableMessageFromIter for MapEntryWrapper<'a, Msg, Map>
where
    Msg: crate::MapEntry<KeyType = Map::Key, ValueType = Map::Value> + MergeableMessageFromIter,
    Map: MapType,
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

pub trait MapType {
    type Key;
    type Value;
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}
impl<K, V> MapType for HashMap<K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        <Self>::insert(self, key, value)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump, K, V> MapType
    for hashbrown::HashMap<
        K,
        V,
        hashbrown::hash_map::DefaultHashBuilder,
        hashbrown::BumpWrapper<'bump>,
    >
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        <Self>::insert(self, key, value)
    }
}
