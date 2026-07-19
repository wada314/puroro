//! Dual storage for protobuf map fields via [`cached_pair::Pair`].
//!
//! Left = wire-order [`AllocVec`], right = [`HashMap`]. Accessors use Pair's
//! implicit conversion (`left` / `right` / `*_mut`). Avoiding long-lived
//! duplicate heap instances when both sides are cached is follow-up work.

use ::core::convert::Infallible;
use ::core::hash::Hash;

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::cached_pair::{Converter, Pair};
use ::hashbrown::DefaultHashBuilder;
use ::hashbrown::Equivalent;
use ::hashbrown::HashMap;
use ::hashbrown::hash_map::Iter as HashMapIter;
use ::unmanaged::{CloneIn, DeallocateIn};

type EntryList<K, V, A> = AllocVec<(K, V), A>;
type EntryMap<K, V, A> = HashMap<K, V, DefaultHashBuilder, A>;

/// Rebuilds list ↔ map through [`CloneIn`], taking `A` from the source.
#[derive(Clone, Copy, Default, Debug)]
pub struct MapEntriesConverter;

impl<K, V, A> Converter<EntryList<K, V, A>, EntryMap<K, V, A>> for MapEntriesConverter
where
    K: CloneIn<A> + Eq + Hash,
    V: CloneIn<A>,
    A: Allocator + Clone,
{
    type ToLeftError = Infallible;
    type ToRightError = Infallible;

    fn convert_to_right(&self, left: &EntryList<K, V, A>) -> Result<EntryMap<K, V, A>, Infallible> {
        let alloc = left.allocator().clone();
        let mut map = HashMap::with_capacity_and_hasher_in(
            left.len(),
            DefaultHashBuilder::default(),
            alloc.clone(),
        );
        // Later duplicates win — same as repeated `insert` over the wire list.
        map.extend(
            left.iter()
                .map(|(k, v)| (k.clone_in(alloc.clone()), v.clone_in(alloc.clone()))),
        );
        Ok(map)
    }

    fn convert_to_left(&self, right: &EntryMap<K, V, A>) -> Result<EntryList<K, V, A>, Infallible> {
        let alloc = right.allocator().clone();
        let mut list = AllocVec::with_capacity_in(right.len(), alloc.clone());
        list.extend(
            right
                .iter()
                .map(|(k, v)| (k.clone_in(alloc.clone()), v.clone_in(alloc.clone()))),
        );
        Ok(list)
    }
}

/// Protobuf map payload: [`Pair`] of entry list and hash map.
pub struct MapEntries<K, V, A>
where
    A: Allocator + Clone,
{
    pair: Pair<EntryList<K, V, A>, EntryMap<K, V, A>, MapEntriesConverter>,
}

impl<K, V, A> MapEntries<K, V, A>
where
    A: Allocator + Clone,
{
    /// Empty entry list (decode-friendly default).
    pub fn new_in(alloc: A) -> Self
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        Self {
            pair: Pair::from_left(AllocVec::new_in(alloc)),
        }
    }

    #[inline]
    pub fn len(&self) -> usize
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        // Prefer the map length once either side can satisfy a map view.
        if self.pair.right_opt().is_some() {
            self.pair.right().len()
        } else {
            self.pair.left().len()
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.len() == 0
    }

    /// Map lookup, converting list → map on first use when needed.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.pair.right().get(key)
    }

    /// Appends a wire entry on the list side (implicitly drops a cached map).
    pub fn push(&mut self, key: K, value: V)
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.pair.left_mut().push((key, value));
    }

    /// Inserts with map semantics (last-wins; implicitly drops a cached list).
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        self.pair.right_mut().insert(key, value)
    }

    /// Removes `key` (map side; implicitly drops a cached list).
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.pair.right_mut().remove(key)
    }

    /// Releases every entry payload on the map view, then clears it.
    ///
    /// Uses the map side (converting if needed). Discarded-side `Drop` of
    /// heap payloads is a known follow-up (same theme as avoiding Both
    /// duplicates).
    pub fn clear(&mut self)
    where
        K: CloneIn<A> + Eq + Hash + DeallocateIn<A>,
        V: CloneIn<A> + DeallocateIn<A>,
    {
        let map = self.pair.right_mut();
        let alloc = map.allocator().clone();
        for (k, v) in map.drain() {
            // SAFETY: map allocator owns payloads.
            unsafe {
                k.deallocate_in(alloc.clone());
                v.deallocate_in(alloc.clone());
            }
        }
    }

    pub fn clone_in(&self, alloc: A) -> Self
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        let mut out = HashMap::with_capacity_and_hasher_in(
            self.pair.right().len(),
            DefaultHashBuilder::default(),
            alloc.clone(),
        );
        out.extend(
            self.pair
                .right()
                .iter()
                .map(|(k, v)| (k.clone_in(alloc.clone()), v.clone_in(alloc.clone()))),
        );
        Self {
            pair: Pair::from_right(out),
        }
    }

    pub fn iter(&self) -> MapEntriesIter<'_, K, V>
    where
        K: CloneIn<A> + Eq + Hash,
        V: CloneIn<A>,
    {
        MapEntriesIter::Map(self.pair.right().iter())
    }
}

/// Iterator over map entries (always the map view).
pub enum MapEntriesIter<'a, K, V> {
    Map(HashMapIter<'a, K, V>),
}

impl<'a, K, V> Iterator for MapEntriesIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Map(it) => it.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MapEntries;
    use ::allocator_api2::alloc::Global;

    #[test]
    fn push_then_get_converts_to_map() {
        let mut entries = MapEntries::<i32, i32, _>::new_in(Global);
        entries.push(1, 10);
        entries.push(1, 11);
        // `get` uses `right()`, so the list is converted (and may stay cached).
        assert_eq!(entries.get(&1), Some(&11));
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn insert_uses_map_directly() {
        let mut entries = MapEntries::<i32, i32, _>::new_in(Global);
        assert_eq!(entries.insert(1, 10), None);
        assert_eq!(entries.insert(1, 11), Some(10));
        assert_eq!(entries.get(&1), Some(&11));
    }

    #[test]
    fn push_after_get_returns_to_list() {
        let mut entries = MapEntries::<i32, i32, _>::new_in(Global);
        entries.insert(1, 10);
        entries.push(2, 20);
        // After `push`, list is authoritative; `get` converts again.
        assert_eq!(entries.get(&1), Some(&10));
        assert_eq!(entries.get(&2), Some(&20));
    }
}
