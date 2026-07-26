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
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
    type MutTarget: ?Sized;

    type Mut<'a>: DerefMut<Target = Self::MutTarget>
    where
        Self: 'a;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, key: impl Borrow<K>) -> Option<&V>;

    fn get_mut(&mut self, key: impl Borrow<K>) -> Option<Self::Mut<'_>>;

    /// Inserts a type-default value if `key` is absent, then returns a mut handle.
    fn entry_mut(&mut self, key: impl Borrow<K>) -> Self::Mut<'_>;

    fn remove(&mut self, key: impl Borrow<K>);

    fn clear(&mut self);
}
