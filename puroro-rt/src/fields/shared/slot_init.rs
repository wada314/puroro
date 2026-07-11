//! Value-slot initialization state — whether a [`MaybeUninit`](core::mem::MaybeUninit)
//! payload currently holds a valid `T`.
//!
//! This is **storage init state** (for [`ValueSlot`](super::value_slot::ValueSlot) borrows
//! and writes), not wire-level field presence. Wire policy lives on
//! [`FieldPresence`](super::field_presence::FieldPresence).
//!
//! - [`SlotInitView`] — read-only (`is_initialized`), used by [`ValueSlot::as_ref`].
//! - [`SlotInitMut`] — read + update init state, used by [`ValueSlot::set`] /
//!   [`clear`](ValueSlot::clear) / [`as_mut`](ValueSlot::as_mut).

use ::allocator_api2::alloc::Allocator;

use super::{MessageCommon, PresenceBits};

/// Read-only view of whether a value slot is initialized.
pub trait SlotInitView {
    /// `true` when the slot holds a valid value.
    fn is_initialized(&self) -> bool;
}

/// Mutable handle to a slot's initialization state.
pub trait SlotInitMut: SlotInitView {
    /// Marks the slot initialized or uninitialized.
    fn set_initialized(&mut self, initialized: bool);
}

/// Init adapter for always-initialized slots ([`Implicit`](super::field_presence::Implicit),
/// [`Oneof`](super::field_presence::Oneof)).
#[derive(Clone, Copy, Debug, Default)]
pub struct AlwaysInitialized;

impl SlotInitView for AlwaysInitialized {
    #[inline]
    fn is_initialized(&self) -> bool {
        true
    }
}

impl SlotInitMut for AlwaysInitialized {
    #[inline]
    fn set_initialized(&mut self, _initialized: bool) {}
}

/// Read-only view of an explicit field's init bit in [`MessageCommon`].
pub struct BitInitView<'a, const BIT: usize, P: PresenceBits, A: Allocator> {
    common: &'a MessageCommon<P, A>,
}

impl<'a, const BIT: usize, P: PresenceBits, A: Allocator> BitInitView<'a, BIT, P, A> {
    #[inline]
    pub fn new(common: &'a MessageCommon<P, A>) -> Self {
        Self { common }
    }
}

impl<const BIT: usize, P: PresenceBits, A: Allocator> SlotInitView for BitInitView<'_, BIT, P, A> {
    #[inline]
    fn is_initialized(&self) -> bool {
        self.common.is_bit_set(BIT)
    }
}

/// Mutable handle to an explicit field's init bit in [`MessageCommon`].
pub struct BitInitMut<'a, const BIT: usize, P: PresenceBits, A: Allocator> {
    common: &'a mut MessageCommon<P, A>,
}

impl<'a, const BIT: usize, P: PresenceBits, A: Allocator> BitInitMut<'a, BIT, P, A> {
    #[inline]
    pub fn new(common: &'a mut MessageCommon<P, A>) -> Self {
        Self { common }
    }
}

impl<const BIT: usize, P: PresenceBits, A: Allocator> SlotInitView for BitInitMut<'_, BIT, P, A> {
    #[inline]
    fn is_initialized(&self) -> bool {
        self.common.is_bit_set(BIT)
    }
}

impl<const BIT: usize, P: PresenceBits, A: Allocator> SlotInitMut for BitInitMut<'_, BIT, P, A> {
    #[inline]
    fn set_initialized(&mut self, initialized: bool) {
        self.common.set_bit(BIT, initialized);
    }
}
