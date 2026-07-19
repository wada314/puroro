//! Hash map storage for protobuf map fields.
//!
//! Wire order is unspecified for maps, so entries are kept only in a
//! [`HashMap`] (allocator-owning). Duplicate keys use last-wins semantics.
//!
//! `K` / `V` here are **physical** element types (`i32`, `UnmanagedString`, …),
//! not wire markers — see [`MapField`](super::MapField).

use ::core::hash::Hash;

use ::allocator_api2::alloc::Allocator;
use ::core::mem;

use ::hashbrown::DefaultHashBuilder;
use ::hashbrown::Equivalent;
use ::hashbrown::HashMap;
use ::hashbrown::hash_map::Drain as HashMapDrain;
use ::hashbrown::hash_map::Iter as HashMapIter;
use ::unmanaged::CloneIn;

type EntryMap<K, V, A> = HashMap<K, V, DefaultHashBuilder, A>;

/// Protobuf map payload: a single allocator-aware [`HashMap`].
pub struct MapEntries<K, V, A>
where
    A: Allocator + Clone,
{
    map: EntryMap<K, V, A>,
}

impl<K, V, A> MapEntries<K, V, A>
where
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            map: HashMap::with_hasher_in(DefaultHashBuilder::default(), alloc),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn allocator(&self) -> &A {
        self.map.allocator()
    }

    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map.get(key)
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map.get_mut(key)
    }

    /// Inserts `key` / `value` (last-wins).
    ///
    /// When the key was already present, returns `(incoming_key, previous_value)`
    /// so the caller can release both (the map keeps its existing key).
    ///
    /// [`HashMap::insert`](HashMap::insert) would drop the incoming key on
    /// collision, which is unsafe for `Unmanaged*` payloads — hence the
    /// contains-then-replace path.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)>
    where
        K: Eq + Hash,
    {
        if let Some(slot) = self.map.get_mut(&key) {
            let previous = mem::replace(slot, value);
            Some((key, previous))
        } else {
            self.map.insert(key, value);
            None
        }
    }

    /// Removes the entry. Returns owned `(key, value)` so the caller can release
    /// both (dropping `Unmanaged*` keys would panic).
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Eq + Hash,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map.remove_entry(key)
    }

    #[inline]
    pub fn drain(&mut self) -> HashMapDrain<'_, K, V, A> {
        self.map.drain()
    }

    pub fn clone_in(&self, alloc: A) -> Self
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        let mut out = HashMap::with_capacity_and_hasher_in(
            self.map.len(),
            DefaultHashBuilder::default(),
            alloc.clone(),
        );
        out.extend(
            self.map
                .iter()
                .map(|(k, v)| (k.clone_in(alloc.clone()), v.clone_in(alloc.clone()))),
        );
        Self { map: out }
    }

    #[inline]
    pub fn iter(&self) -> MapEntriesIter<'_, K, V> {
        MapEntriesIter(self.map.iter())
    }
}

/// Iterator over map entries (order unspecified).
pub struct MapEntriesIter<'a, K, V>(HashMapIter<'a, K, V>);

impl<'a, K, V> Iterator for MapEntriesIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::MapEntries;
    use ::allocator_api2::alloc::Global;

    #[test]
    fn insert_get_last_wins() {
        let mut entries = MapEntries::<i32, i32, _>::new_in(Global);
        assert_eq!(entries.insert(1, 10), None);
        assert_eq!(entries.insert(1, 11), Some((1, 10)));
        assert_eq!(entries.get(&1), Some(&11));
        assert_eq!(entries.len(), 1);
    }
}
