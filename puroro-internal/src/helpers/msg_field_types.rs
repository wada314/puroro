use crate::deser::{DeserializableMessageFromIter, LdIter};
use crate::types::FieldData;
use crate::Result;
use crate::{bumpalo, hashbrown, tags};
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
pub trait WrappedMessageFieldType<MessageType, LabelTag>
where
    MessageType: Message,
    LabelTag: tags::FieldLabelTag,
{
    type Item: Message;
    type Deserable<'a>: DeserializableMessageFromIter
    where
        Self: 'a,
        Self::Item: 'a;
    type DeserableMut<'a>: DerefMut<Target = Self::Deserable<'a>>
    where
        Self: 'a,
        Self::Item: 'a;
    fn get_or_insert_with<'a, F>(&'a mut self, f: F) -> Self::DeserableMut<'_>
    where
        F: FnOnce() -> Self::Item,
        Self::Item: 'a;
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>;
}
impl<M> WrappedMessageFieldType<M, tags::Required> for M
where
    M: Message + DeserializableMessageFromIter,
{
    type Item = M;
    type Deserable<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = M;
    type DeserableMut<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = &'a mut Self::Item;
    fn get_or_insert_with<'a, F>(&'a mut self, _: F) -> Self::DeserableMut<'_>
    where
        F: FnOnce() -> Self::Item,
        Self::Item: 'a,
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
    M: Message + DeserializableMessageFromIter,
    M::BoxedType: AsMut<M> + AsRef<M>,
{
    type Item = M;
    type Deserable<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = M;
    type DeserableMut<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = &'a mut Self::Item;
    fn get_or_insert_with<'a, F>(&'a mut self, f: F) -> Self::DeserableMut<'_>
    where
        F: FnOnce() -> Self::Item,
        Self::Item: 'a,
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
impl<M> WrappedMessageFieldType<M, tags::Optional3> for Option<M::BoxedType>
where
    M: Message + DeserializableMessageFromIter,
    M::BoxedType: AsMut<M> + AsRef<M>,
{
    type Item = M;
    type Deserable<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = M;
    type DeserableMut<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = &'a mut Self::Item;
    fn get_or_insert_with<'a, F>(&'a mut self, f: F) -> Self::DeserableMut<'_>
    where
        F: FnOnce() -> Self::Item,
        Self::Item: 'a,
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
impl<M, RM> WrappedMessageFieldType<M, tags::Repeated> for RM
where
    M: Message + DeserializableMessageFromIter,
    RM: RepeatedMessageType<M>,
{
    type Item = M;
    type Deserable<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = <RM as RepeatedMessageType<M>>::Deserable<'a>;
    type DeserableMut<'a>
    where
        Self: 'a,
        Self::Item: 'a,
    = <RM as RepeatedMessageType<M>>::DeserableMut<'a>;
    fn get_or_insert_with<'a, F>(&'a mut self, f: F) -> Self::DeserableMut<'_>
    where
        F: FnOnce() -> Self::Item,
        Self::Item: 'a,
    {
        <Self as RepeatedMessageType<M>>::insert_mut(self, (f)())
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Self::Item) -> Result<()>,
    {
        <Self as RepeatedMessageType<M>>::try_for_each(self, f)?;
        Ok(())
    }
}

pub trait RepeatedMessageType<Msg> {
    type Deserable<'a>: DeserializableMessageFromIter
    where
        Self: 'a;
    type DeserableMut<'a>: DerefMut<Target = Self::Deserable<'a>>
    where
        Self: 'a;
    fn insert_mut(&mut self, item: Msg) -> Self::DeserableMut<'_>;
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>;
}

impl<Msg> RepeatedMessageType<Msg> for Vec<Msg>
where
    Msg: Message + DeserializableMessageFromIter,
{
    type Deserable<'a>
    where
        Self: 'a,
    = Msg;
    type DeserableMut<'a>
    where
        Self: 'a,
    = &'a mut Self::Deserable<'a>;
    fn insert_mut(&mut self, item: Msg) -> Self::DeserableMut<'_> {
        self.push(item);
        self.last_mut().unwrap()
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>,
    {
        <std::slice::Iter<Msg> as Iterator>::try_for_each(&mut <[Msg]>::iter(self), f)?;
        Ok(())
    }
}

#[cfg(feature = "puroro-bumpalo")]
impl<'bump, Msg> RepeatedMessageType<Msg> for bumpalo::collections::Vec<'bump, Msg>
where
    Msg: Message + DeserializableMessageFromIter,
{
    type Deserable<'a>
    where
        Self: 'a,
    = Msg;
    type DeserableMut<'a>
    where
        Self: 'a,
    = &'a mut Self::Deserable<'a>;
    fn insert_mut(&mut self, item: Msg) -> Self::DeserableMut<'_> {
        self.push(item);
        self.last_mut().unwrap()
    }
    fn try_for_each<F>(&self, f: F) -> Result<()>
    where
        F: FnMut(&Msg) -> Result<()>,
    {
        <std::slice::Iter<Msg> as Iterator>::try_for_each(&mut <[Msg]>::iter(self), f)?;
        Ok(())
    }
}

impl<K, V, Msg> RepeatedMessageType<Msg> for HashMap<K, V>
where
    K: Eq + Hash,
    Msg: Message + crate::MapEntry<KeyType = K, ValueType = V> + DeserializableMessageFromIter,
{
    type Deserable<'a>
    where
        Self: 'a,
    = MapEntryWrapper<'a, Msg, Self>;
    type DeserableMut<'a>
    where
        Self: 'a,
    = MapEntryWrapper<'a, Msg, Self>;
    fn insert_mut(&mut self, item: Msg) -> Self::DeserableMut<'_> {
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

#[cfg(feature = "puroro-bumpalo")]
impl<'bump, K, V, Msg> RepeatedMessageType<Msg>
    for hashbrown::HashMap<
        K,
        V,
        hashbrown::hash_map::DefaultHashBuilder,
        hashbrown::BumpWrapper<'bump>,
    >
where
    K: Eq + Hash,
    Msg: Message + crate::MapEntry<KeyType = K, ValueType = V> + DeserializableMessageFromIter,
{
    type Deserable<'a>
    where
        Self: 'a,
    = MapEntryWrapper<'a, Msg, Self>;
    type DeserableMut<'a>
    where
        Self: 'a,
    = MapEntryWrapper<'a, Msg, Self>;
    fn insert_mut(&mut self, item: Msg) -> Self::DeserableMut<'_> {
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
impl<'a, Msg, Map> DeserializableMessageFromIter for MapEntryWrapper<'a, Msg, Map>
where
    Msg:
        crate::MapEntry<KeyType = Map::Key, ValueType = Map::Value> + DeserializableMessageFromIter,
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
