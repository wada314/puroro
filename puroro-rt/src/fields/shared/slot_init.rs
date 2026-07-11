//! Value-slot initialization state — whether a [`MaybeUninit`](core::mem::MaybeUninit)
//! payload currently holds a valid `T`.
//!
//! This is **storage init state** (for [`ValueSlot`](super::value_slot::ValueSlot) borrows
//! and writes), not wire-level field presence. Wire policy lives on
//! [`FieldPresence`](super::field_presence::FieldPresence).
//!
//! Init handles are **borrow-free markers** ([`AlwaysInitialized`] / [`BitInit`]).
//! They read and update state through a [`MessageCommon`](super::MessageCommon)
//! passed into each method, so [`ValueSlot::with`](super::value_slot::ValueSlot::with) /
//! [`with_mut`](super::value_slot::ValueSlot::with_mut) can take both `init` and
//! `common` without overlapping borrows.

use ::allocator_api2::alloc::Allocator;

use super::{MessageCommon, PresenceBits};

/// Read-only view of whether a value slot is initialized.
pub trait SlotInitView {
    /// `true` when the slot holds a valid value.
    fn is_initialized<Pb: PresenceBits, A: Allocator>(
        &self,
        common: &MessageCommon<Pb, A>,
    ) -> bool;
}

/// Mutable handle to a slot's initialization state.
pub trait SlotInitMut: SlotInitView {
    /// Marks the slot initialized or uninitialized.
    fn set_initialized<Pb: PresenceBits, A: Allocator>(
        &self,
        common: &mut MessageCommon<Pb, A>,
        initialized: bool,
    );
}

/// Init adapter for always-initialized slots ([`Implicit`](super::field_presence::Implicit),
/// [`Oneof`](super::field_presence::Oneof)).
#[derive(Clone, Copy, Debug, Default)]
pub struct AlwaysInitialized;

impl SlotInitView for AlwaysInitialized {
    #[inline]
    fn is_initialized<Pb: PresenceBits, A: Allocator>(
        &self,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        true
    }
}

impl SlotInitMut for AlwaysInitialized {
    #[inline]
    fn set_initialized<Pb: PresenceBits, A: Allocator>(
        &self,
        _common: &mut MessageCommon<Pb, A>,
        _initialized: bool,
    ) {
    }
}

/// Init adapter for an explicit / legacy-required presence bit at `BIT`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BitInit<const BIT: usize>;

impl<const BIT: usize> SlotInitView for BitInit<BIT> {
    #[inline]
    fn is_initialized<Pb: PresenceBits, A: Allocator>(
        &self,
        common: &MessageCommon<Pb, A>,
    ) -> bool {
        common.is_bit_set(BIT)
    }
}

impl<const BIT: usize> SlotInitMut for BitInit<BIT> {
    #[inline]
    fn set_initialized<Pb: PresenceBits, A: Allocator>(
        &self,
        common: &mut MessageCommon<Pb, A>,
        initialized: bool,
    ) {
        common.set_bit(BIT, initialized);
    }
}
