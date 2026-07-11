//! Accessor and bind API for singular field wrappers.
//!
//! [`SingularAccess`] names getter / `_mut` payload types (`Ref` / `Mut`) and
//! pairs a field with [`MessageCommon`] via `bind` / `bind_mut` (`View` /
//! `ViewMut`). [`SingularField`] forwards payloads to [`ScalarProtoType`];
//! [`BoolField`] / [`NestedMessageField`] supply their own.

use ::allocator_api2::alloc::Allocator;

use crate::fields::shared::field_presence::FieldPresence;
use crate::fields::shared::value_slot::ValueSlot;
use crate::fields::shared::{MessageCommon, PresenceBits};
use crate::fields::wire::scalar::ScalarProtoType;

use super::bool::{BoolField, BoolFieldMut, BoolFieldRef};
use super::field::{SingularField, SingularFieldMut, SingularFieldRef};
use super::message::{MessagePresence, NestedMessageField, NestedMessageFieldMut, NestedMessageFieldRef};

/// Getter / mutable-accessor payloads and MessageCommon binding for a singular
/// field wrapper.
///
/// Implemented by [`SingularField`], [`BoolField`], and [`NestedMessageField`].
pub trait SingularAccess {
    /// Borrowed or by-value view returned by shared getters.
    type Ref<'a>
    where
        Self: 'a;

    /// Mutable handle returned by `_mut` accessors (and oneof `as_mut` payloads).
    type Mut<'a, A: Allocator + 'a>
    where
        Self: 'a;

    /// Short-lived `(field, common)` read view from [`bind`](Self::bind).
    type View<'a, Pb: PresenceBits, A: Allocator>
    where
        Self: 'a,
        Pb: 'a,
        A: 'a;

    /// Short-lived `(field, common)` mutation view from [`bind_mut`](Self::bind_mut).
    type ViewMut<'f, 'c, Pb: PresenceBits, A: Allocator>
    where
        Self: 'f,
        Pb: 'c,
        A: 'c;

    /// Binds this field to `common` for read access.
    fn bind<'a, Pb: PresenceBits, A: Allocator + Clone>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self::View<'a, Pb, A>;

    /// Binds this field to `common` for mutation.
    fn bind_mut<'f, 'c, Pb: PresenceBits, A: Allocator + Clone>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self::ViewMut<'f, 'c, Pb, A>;
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> SingularAccess
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T>,
{
    type Ref<'a>
        = T::Ref<'a>
    where
        Self: 'a;
    type Mut<'a, A: Allocator + 'a>
        = T::Mut<'a, A>
    where
        Self: 'a;
    type View<'a, Pb: PresenceBits, A: Allocator>
        = SingularFieldRef<'a, T, P, FIELD, D, Pb, A>
    where
        Self: 'a,
        Pb: 'a,
        A: 'a;
    type ViewMut<'f, 'c, Pb: PresenceBits, A: Allocator>
        = SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>
    where
        Self: 'f,
        Pb: 'c,
        A: 'c;

    #[inline]
    fn bind<'a, Pb: PresenceBits, A: Allocator + Clone>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self::View<'a, Pb, A> {
        SingularFieldRef::new(self, common)
    }

    #[inline]
    fn bind_mut<'f, 'c, Pb: PresenceBits, A: Allocator + Clone>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self::ViewMut<'f, 'c, Pb, A> {
        SingularFieldMut::new(self, common)
    }
}

impl<M, P: MessagePresence, const FIELD: u32, AField: Allocator> SingularAccess
    for NestedMessageField<M, P, FIELD, AField>
{
    type Ref<'a>
        = &'a M
    where
        Self: 'a;
    type Mut<'a, A: Allocator + 'a>
        = &'a mut M
    where
        Self: 'a;
    /// `A` is the message allocator; the field's own allocator type is `AField`.
    /// Call sites always use the same type for both.
    type View<'a, Pb: PresenceBits, A: Allocator>
        = NestedMessageFieldRef<'a, M, P, FIELD, AField, A, Pb>
    where
        Self: 'a,
        Pb: 'a,
        A: 'a;
    type ViewMut<'f, 'c, Pb: PresenceBits, A: Allocator>
        = NestedMessageFieldMut<'f, 'c, M, P, FIELD, AField, A, Pb>
    where
        Self: 'f,
        Pb: 'c,
        A: 'c;

    #[inline]
    fn bind<'a, Pb: PresenceBits, A: Allocator + Clone>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self::View<'a, Pb, A> {
        NestedMessageFieldRef::new(self, common)
    }

    #[inline]
    fn bind_mut<'f, 'c, Pb: PresenceBits, A: Allocator + Clone>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self::ViewMut<'f, 'c, Pb, A> {
        NestedMessageFieldMut::new(self, common)
    }
}

impl<P: FieldPresence, const VALUE_BIT: usize, const FIELD: u32, D> SingularAccess
    for BoolField<P, VALUE_BIT, FIELD, D>
{
    type Ref<'a>
        = bool
    where
        Self: 'a;
    /// Named bit handle for enum payloads; public `_mut` still returns `impl DerefMut`.
    type Mut<'a, A: Allocator + 'a>
        = ::bitvec::ptr::BitRef<'a, ::bitvec::ptr::Mut, u8, ::bitvec::order::Lsb0>
    where
        Self: 'a;
    type View<'a, Pb: PresenceBits, A: Allocator>
        = BoolFieldRef<'a, P, VALUE_BIT, FIELD, D, Pb, A>
    where
        Self: 'a,
        Pb: 'a,
        A: 'a;
    type ViewMut<'f, 'c, Pb: PresenceBits, A: Allocator>
        = BoolFieldMut<'f, 'c, P, VALUE_BIT, FIELD, D, Pb, A>
    where
        Self: 'f,
        Pb: 'c,
        A: 'c;

    #[inline]
    fn bind<'a, Pb: PresenceBits, A: Allocator + Clone>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self::View<'a, Pb, A> {
        BoolFieldRef::new(self, common)
    }

    #[inline]
    fn bind_mut<'f, 'c, Pb: PresenceBits, A: Allocator + Clone>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self::ViewMut<'f, 'c, Pb, A> {
        BoolFieldMut::new(self, common)
    }
}
