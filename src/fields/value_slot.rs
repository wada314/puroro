//! Value slot storage for singular fields — always-initialized (`T`) or
//! presence-tracked ([`MaybeUninit<T>`]), selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T` and [`MaybeUninit<T>`]. The trait's
//! type parameter `T` disambiguates the two blanket impls so they do not overlap.
//!
//! Mutation takes a [`SlotPresence`](super::slot_presence::SlotPresence) adapter so
//! init-state branching is centralized here rather than in [`FieldPresence`](super::field_presence::FieldPresence).

use ::core::mem::MaybeUninit;

use super::proto_zero::ProtoZero;
use super::slot_presence::{SlotInitState, SlotPresence};

/// Storage operations for a singular field value slot.
pub trait ValueSlot<T: ProtoZero> {
    /// Constructs an empty slot at message creation (`new_in` / `Default`).
    ///
    /// For raw `T`, writes type-zero via [`ProtoZero::set_proto_zero`]. For
    /// [`MaybeUninit`], leaves the slot uninitialized.
    fn new_empty() -> Self;

    /// Assigns `value`, updating presence when the slot was uninitialized.
    ///
    /// Unlike [`as_mut`](Self::as_mut), the slot does **not** need to be initialized
    /// first — for [`MaybeUninit`] this uses [`MaybeUninit::write`] on first assign.
    fn set(&mut self, presence: &mut impl SlotPresence, value: T);

    /// Drops an initialized payload and clears presence; no-op when uninitialized.
    fn clear(&mut self, presence: &mut impl SlotPresence);

    /// Returns `&mut T`, lazy-initializing with type-zero when the slot is uninitialized.
    ///
    /// Like [`Box::as_mut`](Box::as_mut), but also marks the slot present via `presence`.
    /// When the slot is not yet initialized, writes protobuf type-zero as a placeholder
    /// before returning — use [`set`](Self::set) when you have the final value and want
    /// to avoid that intermediate state.
    fn as_mut<'a>(&'a mut self, presence: &mut impl SlotPresence) -> &'a mut T;

    /// Borrows `&T` when initialized.
    ///
    /// Like [`Option::as_ref`](Option::as_ref) — `None` when the slot is not initialized.
    fn as_ref<'a>(&'a self, init: &impl SlotInitState) -> Option<&'a T>;
}

impl<T: ProtoZero> ValueSlot<T> for T {
    fn new_empty() -> Self {
        let mut slot = MaybeUninit::<T>::uninit();
        unsafe {
            T::set_proto_zero(&mut *slot.as_mut_ptr());
            slot.assume_init()
        }
    }

    fn set(&mut self, _: &mut impl SlotPresence, value: T) {
        *self = value;
    }

    fn clear(&mut self, _: &mut impl SlotPresence) {
        T::set_proto_zero(self);
    }

    fn as_mut(&mut self, _: &mut impl SlotPresence) -> &mut T {
        self
    }

    fn as_ref(&self, _: &impl SlotInitState) -> Option<&T> {
        Some(self)
    }
}

impl<T: ProtoZero> ValueSlot<T> for MaybeUninit<T> {
    fn new_empty() -> Self {
        MaybeUninit::uninit()
    }

    fn set(&mut self, presence: &mut impl SlotPresence, value: T) {
        if presence.is_initialized() {
            unsafe {
                *self.assume_init_mut() = value;
            }
        } else {
            MaybeUninit::write(self, value);
            presence.set_initialized(true);
        }
    }

    fn clear(&mut self, presence: &mut impl SlotPresence) {
        if presence.is_initialized() {
            unsafe {
                self.assume_init_drop();
            }
            presence.set_initialized(false);
        }
    }

    fn as_mut(&mut self, presence: &mut impl SlotPresence) -> &mut T {
        if !presence.is_initialized() {
            unsafe {
                T::set_proto_zero(&mut *self.as_mut_ptr());
            }
            presence.set_initialized(true);
        }
        unsafe { self.assume_init_mut() }
    }

    fn as_ref(&self, init: &impl SlotInitState) -> Option<&T> {
        if init.is_initialized() {
            Some(unsafe { self.assume_init_ref() })
        } else {
            None
        }
    }
}
