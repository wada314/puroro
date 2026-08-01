//! Value-slot initialization state — whether a [`MaybeUninit`](core::mem::MaybeUninit)
//! payload currently holds a valid `T`.
//!
//! This is **storage init state** (for [`ValueSlot`](super::value_slot::ValueSlot) borrows
//! and writes), not wire-level field presence. Wire policy lives on
//! [`FieldPresence`](super::field_presence::FieldPresence).
//!
//! Init handles are **borrow-free markers** ([`AlwaysInitialized`] / [`BitInit`]).
//! Bit-tracked adapters learn the bit state through caller-supplied probes so
//! [`AlwaysInitialized`] does not require [`PresenceBits`](super::PresenceBits).

/// Read-only view of whether a value slot is initialized.
pub trait SlotInitView {
    /// `true` when the slot holds a valid value.
    ///
    /// `probe` is invoked with a bit index only when this adapter tracks init
    /// in the message bitfield; [`AlwaysInitialized`] never calls it.
    fn is_initialized(&self, probe: impl FnOnce(usize) -> bool) -> bool;
}

/// Mutable handle to a slot's initialization state.
pub trait SlotInitMut: SlotInitView {
    /// Marks the slot initialized or uninitialized.
    ///
    /// `set` is invoked with `(bit, initialized)` only when this adapter tracks
    /// init in the message bitfield; [`AlwaysInitialized`] never calls it.
    fn set_initialized(&self, set: impl FnOnce(usize, bool), initialized: bool);
}

/// Init adapter for always-initialized slots ([`Implicit`](super::field_presence::Implicit),
/// [`Oneof`](super::field_presence::Oneof)).
#[derive(Clone, Copy, Debug, Default)]
pub struct AlwaysInitialized;

impl SlotInitView for AlwaysInitialized {
    #[inline]
    fn is_initialized(&self, _probe: impl FnOnce(usize) -> bool) -> bool {
        true
    }
}

impl SlotInitMut for AlwaysInitialized {
    #[inline]
    fn set_initialized(&self, _set: impl FnOnce(usize, bool), _initialized: bool) {}
}

/// Init adapter for an explicit / legacy-required presence bit at `BIT`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BitInit<const BIT: usize>;

impl<const BIT: usize> SlotInitView for BitInit<BIT> {
    #[inline]
    fn is_initialized(&self, probe: impl FnOnce(usize) -> bool) -> bool {
        probe(BIT)
    }
}

impl<const BIT: usize> SlotInitMut for BitInit<BIT> {
    #[inline]
    fn set_initialized(&self, set: impl FnOnce(usize, bool), initialized: bool) {
        set(BIT, initialized);
    }
}
