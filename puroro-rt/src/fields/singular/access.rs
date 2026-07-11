//! Accessor payload types (`Ref` / `Mut`) for singular field wrappers.
//!
//! Thin adapter so generated oneof aliases can name getter / `_mut` return types
//! without hard-coding `StringGuard`, `BitRef`, etc. [`SingularField`] forwards
//! to [`ScalarProtoType`]; [`BoolField`] / [`NestedMessageField`] supply their
//! own projections.

use ::allocator_api2::alloc::Allocator;

use crate::fields::shared::field_presence::FieldPresence;
use crate::fields::shared::value_slot::ValueSlot;
use crate::fields::wire::scalar::ScalarProtoType;

use super::bool::BoolField;
use super::field::SingularField;
use super::message::{MessagePresence, NestedMessageField};

/// Getter / mutable-accessor payload types for a singular field wrapper.
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
}

impl<M, P: MessagePresence, const FIELD: u32, A: Allocator> SingularAccess
    for NestedMessageField<M, P, FIELD, A>
{
    type Ref<'a>
        = &'a M
    where
        Self: 'a;
    type Mut<'a, A2: Allocator + 'a>
        = &'a mut M
    where
        Self: 'a;
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
}
