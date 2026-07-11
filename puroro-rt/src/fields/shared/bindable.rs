//! Binding traits for the field `bind` / `bind_mut` idiom.
//!
//! Storage types that participate in the bound-view mutation pattern implement
//! [`BindableMut`]. The associated [`BoundMut`](BindableMut::BoundMut) GAT names
//! the short-lived view returned from binding (for example
//! [`SingularLenFieldMut`](crate::fields::singular::len::SingularLenFieldMut)).
//!
//! Read-side binding ([`Bindable`]) uses the same shape with immutable
//! context; not every field family has a separate read view yet.

/// Read-side binding for a field storage type.
///
/// `bind` produces a short-lived view that carries `ctx` for accessors that
/// need shared message state (presence, allocator, …). Field families without a
/// dedicated read view may omit this trait until one is introduced.
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
    fn bind_mut<'f, 'c>(
        &'f mut self,
        ctx: &'c mut Ctx,
    ) -> Self::BoundMut<'f, 'c>;
}
