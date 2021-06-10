use crate::deser::{DeserializableMessageFromIter, LdIter};
use crate::types::FieldData;
use crate::Result;
use crate::{bumpalo, hashbrown, tags};
use puroro::Message;
use std::collections::HashMap;
use std::hash::Hash;

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
    VT: RepeatedMessageType<T>,
{
    type Item = T;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        <Self as RepeatedMessageType<T>>::insert_mut(self, (f)())
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

pub trait RepeatedMessageType<Msg> {
    type Item;
    type RefMut<'a>
    where
        Self: 'a;
    fn insert_mut(&mut self, item: Self::Item) -> Self::RefMut<'_>;
}

impl<Msg> RepeatedMessageType<Msg> for Vec<Msg>
where
    Msg: Message,
{
    type Item = Msg;
    type RefMut<'a>
    where
        Self: 'a,
    = &'a mut Msg;
    fn insert_mut(&mut self, item: Self::Item) -> Self::RefMut<'_> {
        self.push(item);
        self.last_mut().unwrap()
    }
}

impl<K, V, Msg> RepeatedMessageType<Msg> for HashMap<K, V>
where
    K: Eq + Hash,
    Msg: Message + crate::MapEntry<KeyType = K, ValueType = V>,
{
    type Item = Msg;
    type RefMut<'a>
    where
        Self: 'a,
    = MapEntryWrapper<'a, Msg, Self>;
    fn insert_mut(&mut self, item: Self::Item) -> Self::RefMut<'_> {
        MapEntryWrapper {
            entry: item,
            map: self,
        }
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
