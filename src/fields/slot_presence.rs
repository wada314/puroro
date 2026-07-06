//! Initialization state for singular value slots.
//!
//! [`SlotPresence`] tracks whether a [`MaybeUninit`](core::mem::MaybeUninit) payload is
//! initialized (mirroring an explicit-presence bit). [`ValueSlot`](super::value_slot::ValueSlot)
//! mutation methods take a presence adapter so init-state branching lives in one place.

use ::allocator_api2::alloc::Allocator;

use super::common::MessageCommon;
use super::presence::PresenceBits;

/// Read-only view of whether a value slot is initialized.
pub trait SlotInitState {
    /// `true` when the slot holds a valid value.
    fn is_initialized(&self) -> bool;
}

/// Read/write initialization state for value-slot mutation.
pub trait SlotPresence: SlotInitState {
    /// Marks the slot initialized or uninitialized.
    fn set_initialized(&mut self, initialized: bool);
}

/// Presence adapter for always-initialized slots ([`Implicit`](super::field_presence::Implicit),
/// [`Oneof`](super::field_presence::Oneof)).
#[derive(Clone, Copy, Debug, Default)]
pub struct AlwaysInitialized;

impl SlotInitState for AlwaysInitialized {
    #[inline]
    fn is_initialized(&self) -> bool {
        true
    }
}

impl SlotPresence for AlwaysInitialized {
    #[inline]
    fn set_initialized(&mut self, _initialized: bool) {}
}

/// Read-only view of an explicit-presence bit in [`MessageCommon`].
pub struct BitInitView<'a, const BIT: usize, P: PresenceBits, A: Allocator> {
    common: &'a MessageCommon<P, A>,
}

impl<'a, const BIT: usize, P: PresenceBits, A: Allocator> BitInitView<'a, BIT, P, A> {
    #[inline]
    pub fn new(common: &'a MessageCommon<P, A>) -> Self {
        Self { common }
    }
}

impl<const BIT: usize, P: PresenceBits, A: Allocator> SlotInitState for BitInitView<'_, BIT, P, A> {
    #[inline]
    fn is_initialized(&self) -> bool {
        self.common.is_present(BIT)
    }
}

/// Mutable adapter for an explicit-presence bit in [`MessageCommon`].
pub struct BitPresence<'a, const BIT: usize, P: PresenceBits, A: Allocator> {
    common: &'a mut MessageCommon<P, A>,
}

impl<'a, const BIT: usize, P: PresenceBits, A: Allocator> BitPresence<'a, BIT, P, A> {
    #[inline]
    pub fn new(common: &'a mut MessageCommon<P, A>) -> Self {
        Self { common }
    }
}

impl<const BIT: usize, P: PresenceBits, A: Allocator> SlotInitState for BitPresence<'_, BIT, P, A> {
    #[inline]
    fn is_initialized(&self) -> bool {
        self.common.is_present(BIT)
    }
}

impl<const BIT: usize, P: PresenceBits, A: Allocator> SlotPresence for BitPresence<'_, BIT, P, A> {
    #[inline]
    fn set_initialized(&mut self, initialized: bool) {
        self.common.set_presence(BIT, initialized);
    }
}
