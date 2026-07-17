//! Accessor and bind API for singular field wrappers.
//!
//! [`SingularAccess`] names getter / `_mut` payload types (`Ref` / `Mut`) and
//! pairs a field with [`MessageCommon`] via `bind` / `bind_mut` (`View` /
//! `ViewMut`). [`SingularField`] forwards to [`ProtoType`];
//! [`NestedMessageField`] supplies its own.

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedBox;

use crate::fields::shared::field_presence::FieldPresence;
use crate::fields::shared::value_slot::ValueSlot;
use crate::fields::shared::{MessageCommon, PresenceBits};
use crate::fields::wire::proto_type::ProtoType;

use super::field::{SingularField, SingularFieldMut, SingularFieldRef};
use super::message::{NestedMessageField, NestedMessageFieldMut, NestedMessageFieldRef};

/// Getter / mutable-accessor payloads and MessageCommon binding for a singular
/// field wrapper.
///
/// Implemented by [`SingularField`] and [`NestedMessageField`].
pub trait SingularAccess {
    /// Allocator type retained by this field's storage.
    type Alloc: Allocator + Clone;

    /// Borrowed or by-value view returned by shared getters.
    type Ref<'a>
    where
        Self: 'a;

    /// Mutable handle returned by `_mut` accessors (and oneof `as_mut` payloads).
    type Mut<'a>
    where
        Self: 'a;

    /// Short-lived `(field, common)` read view from [`bind`](Self::bind).
    type View<'a, Pb: PresenceBits>
    where
        Self: 'a,
        Pb: 'a,
        Self::Alloc: 'a;

    /// Short-lived `(field, common)` mutation view from [`bind_mut`](Self::bind_mut).
    type ViewMut<'f, 'c, Pb: PresenceBits>
    where
        Self: 'f,
        Pb: 'c,
        Self::Alloc: 'f,
        Self::Alloc: 'c;

    /// Binds this field to `common` for read access.
    fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, Self::Alloc>,
    ) -> Self::View<'a, Pb>;

    /// Binds this field to `common` for mutation.
    fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, Self::Alloc>,
    ) -> Self::ViewMut<'f, 'c, Pb>;
}

impl<T: ProtoType, P: FieldPresence, const FIELD: u32, D> SingularAccess
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    type Alloc = T::Alloc;
    type Ref<'a>
        = T::Ref<'a>
    where
        Self: 'a;
    type Mut<'a>
        = T::Mut<'a>
    where
        Self: 'a;
    type View<'a, Pb: PresenceBits>
        = SingularFieldRef<'a, T, P, FIELD, D, Pb, T::Alloc>
    where
        Self: 'a,
        Pb: 'a,
        T::Alloc: 'a;
    type ViewMut<'f, 'c, Pb: PresenceBits>
        = SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, T::Alloc>
    where
        Self: 'f,
        Pb: 'c,
        T::Alloc: 'f,
        T::Alloc: 'c;

    #[inline]
    fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> Self::View<'a, Pb> {
        SingularFieldRef::new(self, common)
    }

    #[inline]
    fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, T::Alloc>,
    ) -> Self::ViewMut<'f, 'c, Pb> {
        SingularFieldMut::new(self, common)
    }
}

impl<M, P: FieldPresence, const FIELD: u32, AField: Allocator + Clone> SingularAccess
    for NestedMessageField<M, P, FIELD, AField>
where
    M: ::puroro::Message<Alloc = AField>,
    P::ValueSlot<UnmanagedBox<M, AField>>: ValueSlot<UnmanagedBox<M, AField>>,
{
    type Alloc = AField;
    type Ref<'a>
        = &'a M
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut M
    where
        Self: 'a;
    /// `A` is the message allocator; the field's own allocator type is `AField`.
    /// Call sites always use the same type for both.
    type View<'a, Pb: PresenceBits>
        = NestedMessageFieldRef<'a, M, P, FIELD, AField, AField, Pb>
    where
        Self: 'a,
        Pb: 'a,
        AField: 'a;
    type ViewMut<'f, 'c, Pb: PresenceBits>
        = NestedMessageFieldMut<'f, 'c, M, P, FIELD, AField, AField, Pb>
    where
        Self: 'f,
        Pb: 'c,
        AField: 'f,
        AField: 'c;

    #[inline]
    fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, AField>,
    ) -> Self::View<'a, Pb> {
        NestedMessageFieldRef::new(self, common)
    }

    #[inline]
    fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, AField>,
    ) -> Self::ViewMut<'f, 'c, Pb> {
        NestedMessageFieldMut::new(self, common)
    }
}
