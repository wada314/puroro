//! Value slot storage for singular fields — always-initialized (`T`) or
//! presence-tracked ([`MaybeUninit<T>`]), selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T` and [`MaybeUninit<T>`]. The trait's
//! type parameter `T` disambiguates the two blanket impls so they do not overlap.
//!
//! Mutation takes a [`SlotInitMut`](super::slot_init::SlotInitMut) handle so init-state
//! branching is centralized here rather than in [`FieldPresence`](super::field_presence::FieldPresence).

use ::core::mem::MaybeUninit;

use super::{slot_init::{SlotInitMut, SlotInitView}, ProtoZero};

/// Storage operations for a singular field value slot.
pub trait ValueSlot<T: ProtoZero> {
    /// Creates value storage when the parent message is constructed ([`Default`]).
    ///
    /// For raw `T`, returns [`ProtoZero::proto_zero`] — the slot is always initialized.
    /// For [`MaybeUninit`], returns [`MaybeUninit::uninit`] — the slot starts absent.
    fn new() -> Self;

    /// Assigns `value`, updating init state when the slot was uninitialized.
    ///
    /// Unlike [`as_mut`](Self::as_mut), the slot does **not** need to be initialized
    /// first — for [`MaybeUninit`] this uses [`MaybeUninit::write`] on first assign.
    fn set(&mut self, init: &mut impl SlotInitMut, value: T);

    /// Drops an initialized payload and clears init state; no-op when uninitialized.
    fn clear(&mut self, init: &mut impl SlotInitMut);

    /// Returns `&mut T`, lazy-initializing with type-zero when the slot is uninitialized.
    ///
    /// Like [`Box::as_mut`](Box::as_mut), but also marks the slot initialized via `init`.
    /// When the slot is not yet initialized, writes protobuf type-zero as a placeholder
    /// before returning — use [`set`](Self::set) when you have the final value and want
    /// to avoid that intermediate state.
    fn as_mut<'a>(&'a mut self, init: &mut impl SlotInitMut) -> &'a mut T;

    /// Borrows `&T` when initialized.
    ///
    /// Like [`Option::as_ref`](Option::as_ref) — `None` when the slot is not initialized.
    fn as_ref<'a>(&'a self, init: &impl SlotInitView) -> Option<&'a T>;
}

impl<T: ProtoZero> ValueSlot<T> for T {
    fn new() -> Self {
        T::proto_zero()
    }

    fn set(&mut self, _: &mut impl SlotInitMut, value: T) {
        *self = value;
    }

    fn clear(&mut self, _: &mut impl SlotInitMut) {
        T::set_proto_zero(self);
    }

    fn as_mut(&mut self, _: &mut impl SlotInitMut) -> &mut T {
        self
    }

    fn as_ref(&self, _: &impl SlotInitView) -> Option<&T> {
        Some(self)
    }
}

impl<T: ProtoZero> ValueSlot<T> for MaybeUninit<T> {
    fn new() -> Self {
        MaybeUninit::uninit()
    }

    fn set(&mut self, init: &mut impl SlotInitMut, value: T) {
        if init.is_initialized() {
            unsafe {
                *self.assume_init_mut() = value;
            }
        } else {
            MaybeUninit::write(self, value);
            init.set_initialized(true);
        }
    }

    fn clear(&mut self, init: &mut impl SlotInitMut) {
        if init.is_initialized() {
            unsafe {
                self.assume_init_drop();
            }
            init.set_initialized(false);
        }
    }

    fn as_mut(&mut self, init: &mut impl SlotInitMut) -> &mut T {
        if !init.is_initialized() {
            MaybeUninit::write(self, T::proto_zero());
            init.set_initialized(true);
        }
        unsafe { self.assume_init_mut() }
    }

    fn as_ref(&self, init: &impl SlotInitView) -> Option<&T> {
        if init.is_initialized() {
            Some(unsafe { self.assume_init_ref() })
        } else {
            None
        }
    }
}
