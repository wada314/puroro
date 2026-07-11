//! Binding traits for the field `bind` / `bind_mut` idiom.
//!
//! Every field family that participates in generated accessors implements both
//! [`Bindable`] and [`BindableMut`]. The associated
//! [`Bound`](Bindable::Bound) / [`BoundMut`](BindableMut::BoundMut) GATs name the
//! short-lived views returned from binding (for example
//! [`SingularFieldRef`](crate::fields::singular::field::SingularFieldRef) /
//! [`SingularFieldMut`](crate::fields::singular::field::SingularFieldMut)).
//!
//! Generated getters always bind first — `field.bind(&common).optional()` /
//! `field.bind_mut(&mut common).value_mut()` — even when a particular accessor
//! does not consult `common`, so read and write paths share one shape.

/// Read-side binding for a field storage type.
///
/// `bind` produces a short-lived view that carries `ctx` for accessors that
/// need shared message state (presence, allocator, …). Accessors that do not
/// need `ctx` still go through the bound view so generated code stays uniform.
///
/// `Ctx` is the type passed at bind time. Generated messages use
/// [`MessageCommon`](crate::fields::shared::MessageCommon)`<Pb, A>` today, but
/// the trait is not tied to that concrete type.
pub trait Bindable<Ctx: ?Sized> {
    /// Short-lived read view after binding to `ctx`.
    type Bound<'a>
    where
        Self: 'a,
        Ctx: 'a;

    /// Binds this field to `ctx` for read access.
    fn bind<'a>(&'a self, ctx: &'a Ctx) -> Self::Bound<'a>;
}

/// Mutation-side binding for a field storage type.
///
/// Generated decode/merge code uses
/// `BindableMut::bind_mut(field, &mut ctx).merge(…)`.
pub trait BindableMut<Ctx: ?Sized> {
    /// Short-lived mutation view after binding to `ctx`.
    type BoundMut<'f, 'c>
    where
        Self: 'f,
        Ctx: 'c;

    /// Binds this field to `ctx` for mutation.
    fn bind_mut<'f, 'c>(&'f mut self, ctx: &'c mut Ctx) -> Self::BoundMut<'f, 'c>;
}
