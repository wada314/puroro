//! Binding traits for the field `bind` / `bind_mut` idiom.
//!
//! Storage types that participate in the bound-view mutation pattern implement
//! [`BindableMut`]. The associated [`BoundMut`](BindableMut::BoundMut) GAT names
//! the short-lived view returned from binding (for example
//! [`SingularLenFieldMut`](crate::fields::singular::len::SingularLenFieldMut)).
//!
//! Read-side binding ([`Bindable`]) uses the same shape with immutable
//! `common`; not every field family has a separate read view yet.

use ::allocator_api2::alloc::Allocator;

use super::{MessageCommon, PresenceBits};

/// Read-side binding for a field storage type.
///
/// `bind` produces a short-lived view that carries `common` for accessors that
/// need presence or allocator context. Field families without a dedicated read
/// view may omit this trait until one is introduced.
pub trait Bindable<A: Allocator + Clone> {
    /// Short-lived read view after binding to `common`.
    type Bound<'a, Pb: PresenceBits + 'a>
    where
        Self: 'a,
        A: 'a;

    /// Binds this field to `common` for read access.
    fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &MessageCommon<Pb, A>,
    ) -> Self::Bound<'a, Pb>;
}

/// Mutation-side binding for a field storage type.
///
/// Generated decode/merge code uses `field.bind_mut(&mut common).merge(…)` (or
/// the inherent [`bind`](crate::fields::singular::len::SingularLenField::bind)
/// alias on types that expose it today).
pub trait BindableMut<A: Allocator + Clone> {
    /// Short-lived mutation view after binding to `common`.
    type BoundMut<'f, 'c, Pb: PresenceBits + 'c>
    where
        Self: 'f,
        A: 'c,
        A: 'f;

    /// Binds this field to `common` for mutation.
    fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self::BoundMut<'f, 'c, Pb>;
}
