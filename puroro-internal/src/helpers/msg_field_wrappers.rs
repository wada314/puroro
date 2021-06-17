use crate::deser::{LdIter, MergeableMessageFromIter};
use crate::types::FieldData;
use crate::Result;
use crate::{bumpalo, hashbrown};
use ::puroro::tags;
use puroro::Message;
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
pub trait WrappedMessageFieldType<'a, MessageType, LabelTag>
where
    MessageType: Message,
    LabelTag: tags::FieldLabelTag,
{
    type Item: Message;
    type MergeableMut: DerefMut;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item;
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
}
impl<'a, M> WrappedMessageFieldType<'a, M, tags::Required> for M
where
    M: 'a + Message + MergeableMessageFromIter,
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
impl<'a, M> WrappedMessageFieldType<'a, M, tags::Optional2> for Option<M::BoxedType>
where
    M: 'a + Message + MergeableMessageFromIter,
    M::BoxedType: 'a + AsMut<M> + AsRef<M>,
{
    type Item = M;
    type MergeableMut = &'a mut Self::Item;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item,
    {
        self.get_or_insert_with(|| (f)().into_boxed()).as_mut()
    }
    fn try_for_each<F>(&'a self, mut f: F) -> Result<()>
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
impl<'a, M> WrappedMessageFieldType<'a, M, tags::Optional3> for Option<M::BoxedType>
where
    M: 'a + Message + MergeableMessageFromIter,
    M::BoxedType: AsMut<M> + AsRef<M>,
{
    type Item = M;
    type MergeableMut = &'a mut Self::Item;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item,
    {
        self.get_or_insert_with(|| (f)().into_boxed()).as_mut()
    }
    fn try_for_each<F>(&'a self, mut f: F) -> Result<()>
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
impl<'a, M, RM> WrappedMessageFieldType<'a, M, tags::Repeated> for RM
where
    M: Message + MergeableMessageFromIter,
    RM: RepeatedMessageType<'a, M>,
{
    type Item = M;
    type MergeableMut = <RM as RepeatedMessageType<'a, M>>::MergeableMut;
    fn get_or_insert_with<F>(&'a mut self, f: F) -> Self::MergeableMut
    where
        F: FnOnce() -> Self::Item,
    {
        <Self as RepeatedMessageType<M>>::insert_mut(self, (f)())
    }
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <Self as RepeatedMessageType<M>>::try_for_each(self, f)?;
        Ok(())
    }
}

pub trait RepeatedMessageType<'a, Msg> {
    type MergeableMut: DerefMut;
    fn insert_mut(&'a mut self, item: Msg) -> Self::MergeableMut;
    fn try_for_each<F>(&'a self, f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>;
}

impl<'a, Msg> RepeatedMessageType<'a, Msg> for Vec<Msg>
where
    Msg: 'a + Message + MergeableMessageFromIter,
{
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
impl<'a, 'bump, Msg> RepeatedMessageType<'a, Msg> for bumpalo::collections::Vec<'bump, Msg>
where
    Msg: 'a + Message + MergeableMessageFromIter,
{
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

impl<'a, K, V, Msg> RepeatedMessageType<'a, Msg> for HashMap<K, V>
where
    K: Eq + Hash,
    Msg: 'a + Message + crate::MapEntry<KeyType = K, ValueType = V> + MergeableMessageFromIter,
    Self: 'a,
{
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
impl<'a, 'bump, K, V, Msg> RepeatedMessageType<'a, Msg>
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
