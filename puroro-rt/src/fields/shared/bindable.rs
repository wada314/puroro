//! Binding traits for the field `bind` / `bind_mut` idiom.
//!
//! Every field family that participates in generated accessors implements both
//! [`Bindable`] and [`BindableMut`]. The associated
//! [`Bound`](Bindable::Bound) / [`BoundMut`](BindableMut::BoundMut) name the
//! short-lived views returned from binding (for example
//! [`SingularFieldRef`](crate::fields::singular::field::SingularFieldRef) /
//! [`SingularFieldMut`](crate::fields::singular::field::SingularFieldMut)).
//!
//! `self` and `ctx` are taken **by value**. Lifetimes live in the receiver and
//! context types themselves — typically `Self = &'a Field` / `&'a mut Field`
//! and `Ctx = &'a MessageCommon<…>` / `&'a mut MessageCommon<…>` — so the traits
//! need no GATs. Owned contexts (for example an allocator) are equally valid.
//!
//! Read and write use **separate method names** (`bind` / `bind_mut`). A single
//! `bind` for both would be ambiguous: Rust coerces `&mut Ctx` to `&Ctx`, so
//! method resolution would prefer the shared impl.
//!
//! Generated getters always bind first — `field.bind(&common).optional()` /
//! `field.bind_mut(&mut common).value_mut()` — even when a particular accessor
//! does not consult `common`, so read and write paths share one shape.

/// Read-side binding: pairs a shared field receiver with a context value.
///
/// Implementations are normally on `&Field` rather than on the owned storage
/// type. `Ctx` is whatever the caller passes at bind time — generated messages
/// use `&MessageCommon<…>` today, but the trait is not tied to that type.
pub trait Bindable<Ctx> {
    /// Short-lived read view after binding `self` to `ctx`.
    type Bound;

    /// Binds this receiver to `ctx` for read access.
    fn bind(self, ctx: Ctx) -> Self::Bound;
}

/// Mutation-side binding: pairs a mutable field receiver with a context value.
///
/// Implementations are normally on `&mut Field`. Generated decode/merge code
/// uses `field.bind_mut(&mut common).merge(…)`.
pub trait BindableMut<Ctx> {
    /// Short-lived mutation view after binding `self` to `ctx`.
    type BoundMut;

    /// Binds this receiver to `ctx` for mutation.
    fn bind_mut(self, ctx: Ctx) -> Self::BoundMut;
}
