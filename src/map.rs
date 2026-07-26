//! User-facing map field views (no catalog markers).

use ::core::ops::DerefMut;

/// Shared view of a `map<K, V>` field.
///
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

/// Mutable view of a map with a sized key (integral / `bool`).
///
/// - [`V`](Self::get) is the shared view (`i32`, `str`, `Address<A>`, …).
/// - [`MutTarget`](Self::MutTarget) is the `_mut` target (`i32`, [`String`](crate::String), …),
///   matching singular field `*_mut()` / `DerefMut` handles.
///
/// Use [`entry_mut`](Self::entry_mut) to ensure a key exists (type-default value).
/// Sized view values also implement [`MapEntryInsert`] for `insert(key, value)`.
pub trait MapEntryMut<K, V: ?Sized> {
    /// Target of [`get_mut`](Self::get_mut) / [`entry_mut`](Self::entry_mut).
    type MutTarget;

    /// Mutable handle (`&mut i32`, string guard, …).
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
    fn entry_mut(&mut self, key: K) -> Self::Mut<'_>;

    fn remove(&mut self, key: &K);

    fn clear(&mut self);
}

/// Owned-value insert for maps whose shared view type is sized (scalars, enums,
/// messages).
pub trait MapEntryInsert<K, V>: MapEntryMut<K, V, MutTarget = V> {
    fn insert(&mut self, key: K, value: V);
}

/// Mutable view of a `map<string, V>` field.
///
/// Keys are `&str`. Value mutators follow the same `V` /
/// [`MutTarget`](Self::MutTarget) split as [`MapEntryMut`].
pub trait MapMut<V: ?Sized> {
    type MutTarget;

    type Mut<'a>: DerefMut<Target = Self::MutTarget>
    where
        Self: 'a;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: &str) -> Option<&V>;

    fn get_mut(&mut self, key: &str) -> Option<Self::Mut<'_>>;

    /// Ensures an entry for `key` exists and returns a mut handle.
    fn entry_mut(&mut self, key: &str) -> Self::Mut<'_>;

    fn remove(&mut self, key: &str);

    fn clear(&mut self);
}

/// `insert_str` for string-key maps with a sized view value (scalars, enums,
/// messages).
pub trait MapStrInsert<V>: MapMut<V, MutTarget = V> {
    fn insert_str(&mut self, key: &str, value: V);
}
