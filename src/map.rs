//! User-facing map field views (no catalog markers).

use ::core::borrow::Borrow;
use ::core::ops::DerefMut;

/// Shared view of a `map<K, V>` field.
///
/// `K` is the key view type (`str`, `i32`, …) — not an owned buffer type.
/// [`get`](Self::get) returns [`None`] when the key is absent — not because the
/// value is “unset” in the singular-presence sense.
pub trait MapRef<K: ?Sized, V: ?Sized> {
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: &K) -> Option<&V>;
}

/// Mutable view of a `map<K, V>` field (string or sized keys).
///
/// - [`V`] is the shared value view (`i32`, `str`, `Address<A>`, …).
/// - [`MutTarget`](Self::MutTarget) is the `_mut` target (`i32`, [`crate::String`]`<A>`, …).
///
/// [`entry_mut`](Self::entry_mut) takes `impl Borrow<K>` so both `entry_mut(1)`
/// (sized / `Copy` keys) and `entry_mut("k")` (`K = str`) work.
pub trait MapEntryMut<K: ?Sized, V: ?Sized> {
    type MutTarget;

    type Mut<'a>: DerefMut<Target = Self::MutTarget>
    where
        Self: 'a;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<Self::Mut<'_>>;

    /// Inserts a type-default value if `key` is absent, then returns a mut handle.
    fn entry_mut(&mut self, key: impl Borrow<K>) -> Self::Mut<'_>;

    fn remove(&mut self, key: &K);

    fn clear(&mut self);
}

/// Owned-key insert for sized keys (`i32`, `bool`, …).
pub trait MapEntryInsert<K, V>: MapEntryMut<K, V, MutTarget = V> {
    fn insert(&mut self, key: K, value: V);
}

/// `&str` insert for `map<string, …>` with a sized view value.
pub trait MapStrInsert<V>: MapEntryMut<str, V, MutTarget = V> {
    fn insert_str(&mut self, key: &str, value: V);
}
