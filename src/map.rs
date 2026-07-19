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

/// Mutable view of a `map<K, V>` field.
pub trait MapMut<K: ?Sized, V> {
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    /// Builds a key from a byte slice (`map<string, …>` / `map<bytes, …>`) and inserts.
    fn insert_in(&mut self, key: impl AsRef<[u8]>, value: V) -> Result<(), DecodeError>;

    fn remove(&mut self, key: &K);

    fn clear(&mut self);
}
