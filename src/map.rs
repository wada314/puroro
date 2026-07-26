//! User-facing map field views (no catalog markers).

use crate::DecodeError;

/// Shared view of a `map<K, V>` field.
pub trait MapRef<K: ?Sized, V> {
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: &K) -> Option<&V>;
}

/// Mutable view of a `map<string, V>` field (`insert_in` from a byte slice).
pub trait MapMut<K: ?Sized, V> {
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    /// Builds a key from a byte slice (`map<string, …>`) and inserts.
    fn insert_in(&mut self, key: impl AsRef<[u8]>, value: V) -> Result<(), DecodeError>;

    fn remove(&mut self, key: &K);

    fn clear(&mut self);
}

/// Mutable view of a map with a sized key (integral / `bool`).
///
/// Use [`insert`](Self::insert) with an owned key. String-key maps use
/// [`MapMut::insert_in`] instead.
pub trait MapEntryMut<K, V> {
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn insert(&mut self, key: K, value: V);

    fn remove(&mut self, key: &K);

    fn clear(&mut self);
}
