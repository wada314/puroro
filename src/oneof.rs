//! User-facing oneof group views.

/// Shared bound view of a oneof group (available even when unset).
///
/// [`Ref`](Self::Ref) is tied to the view value's lifetime (same as generated
/// `notification()` holding a borrow of the message).
pub trait OneofView {
    /// Payload-less discriminant of the active variant.
    type Case;

    /// Projected read view of the active variant.
    type Ref;

    /// Which variant is set (`None` when the group is unset).
    fn case(&self) -> Option<Self::Case>;

    /// Projected read view of the active variant, if any.
    fn as_ref(&self) -> Option<Self::Ref>;
}

/// Mutable bound view of a oneof group (available even when unset).
///
/// Prefer per-variant `_mut` accessors on the message when switching or
/// initializing a variant. [`as_mut`](Self::as_mut) only projects the
/// *currently* active variant (no switch); [`clear`](Self::clear) drops it.
pub trait OneofViewMut {
    /// Payload-less discriminant of the active variant.
    type Case;

    /// Projected read view from a shared reborrow ([`as_view`](Self::as_view)).
    type Ref<'a>
    where
        Self: 'a;

    /// Projected mutable view of the active variant.
    type Mut;

    /// Shared reborrow for `case` / `as_ref` while holding a mut view.
    type Shared<'a>: OneofView<Case = Self::Case, Ref = Self::Ref<'a>>
    where
        Self: 'a;

    /// Reborrow as a shared bound view.
    fn as_view(&self) -> Self::Shared<'_>;

    /// Which variant is set (`None` when the group is unset).
    fn case(&self) -> Option<Self::Case>;

    /// Projected mutable view of the *currently active* variant (no switch).
    ///
    /// Consumes this bound view (same as the runtime `OneofViewMut` API) so the
    /// message borrow can be re-used for the projected mutator. Returns `None`
    /// when the group is unset.
    #[allow(clippy::wrong_self_convention)]
    fn as_mut(self) -> Option<Self::Mut>;

    /// Clears whichever variant is active.
    fn clear(self);
}
