//! General-purpose `bind` / `bind_mut` pairing traits.
//!
//! [`Bindable`] / [`BindableMut`] are **not** tied to a particular puroro
//! context (not “fields + [`MessageCommon`](super::MessageCommon) only”).
//! They only mean: take a receiver and a context **by value**, return a
//! short-lived view that carries both. Lifetimes live on `Self` and `Ctx`
//! (no GATs on the traits).
//!
//! Typical catalog use today is `Self = &'a Field` / `&'a mut Field` with
//! `Ctx = &'a MessageCommon<…>` / `&'a mut MessageCommon<…>`, yielding views
//! such as [`SingularFieldRef`](crate::fields::singular::field::SingularFieldRef)
//! / [`SingularFieldMut`](crate::fields::singular::field::SingularFieldMut).
//! That is one application. Other receivers and contexts are valid — including
//! owned contexts (for example an allocator) or future bindings that are not
//! message fields.
//!
//! Read and write use **separate method names** (`bind` / `bind_mut`). A single
//! `bind` for both would be ambiguous: Rust coerces `&mut Ctx` to `&Ctx`, so
//! method resolution would prefer the shared impl.
//!
//! Generated message getters usually bind first —
//! `field.bind(&common).optional()` / `field.bind_mut(&mut common).value_mut()` —
//! even when a particular accessor does not consult `common`, so those paths
//! share one shape. That convention is about generated code, not a restriction
//! on these traits.

/// Read-side binding: pairs a receiver with a context value.
///
/// `Ctx` is whatever the caller passes at bind time. Catalog fields often use
/// `&MessageCommon<…>`; the trait itself does not require that type.
pub trait Bindable<Ctx> {
    /// Short-lived read view after binding `self` to `ctx`.
    type Bound;

    /// Binds this receiver to `ctx` for read access.
    fn bind(self, ctx: Ctx) -> Self::Bound;
}

/// Mutation-side binding: pairs a receiver with a context value.
///
/// Same generality as [`Bindable`]: not limited to fields or `MessageCommon`.
/// Catalog decode/merge often uses `field.bind_mut(&mut common).merge(…)`.
pub trait BindableMut<Ctx> {
    /// Short-lived mutation view after binding `self` to `ctx`.
    type BoundMut;

    /// Binds this receiver to `ctx` for mutation.
    fn bind_mut(self, ctx: Ctx) -> Self::BoundMut;
}
