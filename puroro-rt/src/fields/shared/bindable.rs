//! Binding traits for the field `bind` / `bind_mut` idiom.
//!
//! Storage types that participate in the bound-view mutation pattern implement
//! [`BindableMut`]. The associated [`BoundMut`](BindableMut::BoundMut) GAT names
//! the short-lived view returned from binding (for example
//! [`SingularLenFieldMut`](crate::fields::singular::len::SingularLenFieldMut)).
//!
//! Read-side binding ([`Bindable`]) uses the same shape with immutable
//! `common`; not every field family has a separate read view yet.

/// Read-side binding for a field storage type.
///
/// `bind` produces a short-lived view that carries `common` for accessors that
/// need presence or allocator context. Field families without a dedicated read
/// view may omit this trait until one is introduced.
///
/// `Common` is the message context type (typically
/// [`MessageCommon`](crate::fields::shared::MessageCommon)`<Pb, A>` for some
/// presence bitfield `Pb` and allocator `A`).
pub trait Bindable<Common: ?Sized> {
    /// Short-lived read view after binding to `common`.
    type Bound<'a>
    where
        Self: 'a,
        Common: 'a;

    /// Binds this field to `common` for read access.
    fn bind<'a>(&'a self, common: &Common) -> Self::Bound<'a>;
}

/// Mutation-side binding for a field storage type.
///
/// Generated decode/merge code uses `field.bind_mut(&mut common).merge(…)` (or
/// the inherent [`bind`](crate::fields::singular::len::SingularLenField::bind)
/// alias on types that expose it today).
pub trait BindableMut<Common: ?Sized> {
    /// Short-lived mutation view after binding to `common`.
    type BoundMut<'f, 'c>
    where
        Self: 'f,
        Common: 'c;

    /// Binds this field to `common` for mutation.
    fn bind_mut<'f, 'c>(
        &'f mut self,
        common: &'c mut Common,
    ) -> Self::BoundMut<'f, 'c>;
}
