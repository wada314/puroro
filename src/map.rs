//! User-facing map field views (no catalog markers).

use ::core::borrow::Borrow;
use ::core::ops::DerefMut;

/// Shared view of a `map<K, V>` field.
///
/// `K` is the key view type (`str`, `i32`, …) — not an owned buffer type.
/// [`get`](Self::get) returns [`None`] when the key is absent — not because the
/// value is “unset” in the singular-presence sense.
///
/// Key arguments take [`Borrow`]`<K>` so both `get(1)` and `get(&1)` work for
/// sized keys, and `get("k")` for `K = str`.
pub trait MapRef<K: ?Sized, V: ?Sized> {
    /// Number of entries in the map.
    fn len(&self) -> usize;

    /// `true` when the map has no entries.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Shared view of the value for `key`, or [`None`] if absent.
    fn get(&self, key: impl Borrow<K>) -> Option<&V>;
}

/// Mutable view of a `map<K, V>` field (pairs with [`MapRef`]).
///
/// - [`V`] is the shared value view (`i32`, `str`, `Address<A>`, …).
/// - [`MutTarget`](Self::MutTarget) is the `_mut` target (`i32`, [`crate::String`]`<A>`, …).
///
/// Set values via [`entry_mut`](Self::entry_mut) then assign / fill
/// (`*m.entry_mut("k") = 81`, `m.entry_mut(1).push_str("…")`).
/// Key arguments take [`Borrow`]`<K>` (same as [`MapRef::get`]).
pub trait MapMut<K: ?Sized, V: ?Sized> {
    /// Owned / mutator target behind [`Mut`](Self::Mut) (`i32`, [`crate::String`]`<A>`, …).
    type MutTarget: ?Sized;

    /// Short-lived mutable handle (`DerefMut<Target = MutTarget>`).
    type Mut<'a>: DerefMut<Target = Self::MutTarget>
    where
        Self: 'a;

    /// Number of entries in the map.
    fn len(&self) -> usize;

    /// `true` when the map has no entries.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Shared view of the value for `key`, or [`None`] if absent.
    fn get(&self, key: impl Borrow<K>) -> Option<&V>;

    /// Mutable handle for an existing entry, or [`None`] if `key` is absent.
    fn get_mut(&mut self, key: impl Borrow<K>) -> Option<Self::Mut<'_>>;

    /// Inserts a type-default value if `key` is absent, then returns a mut handle.
    fn entry_mut(&mut self, key: impl Borrow<K>) -> Self::Mut<'_>;

    /// Removes the entry for `key` if present (heap payloads are released).
    fn remove(&mut self, key: impl Borrow<K>);

    /// Removes all entries (heap payloads are released).
    fn clear(&mut self);
}
