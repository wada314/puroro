//! Value slot storage for singular fields — always-initialized (`T`) or
//! presence-tracked ([`MaybeUninit<T>`]), selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T` and [`MaybeUninit<T>`]. The trait's
//! type parameter `T` disambiguates the two blanket impls so they do not overlap.
//!
//! Presence-aware mutation ([`FieldPresence::write_slot`](super::field_presence::FieldPresence::write_slot),
//! [`clear_slot`](super::field_presence::FieldPresence::clear_slot),
//! [`prepare_mut_slot`](super::field_presence::FieldPresence::prepare_mut_slot)) lives on
//! [`FieldPresence`](super::field_presence::FieldPresence).

use ::core::mem::MaybeUninit;

use super::proto_zero::ProtoZero;

/// Storage operations for a singular field value slot.
pub trait ValueSlot<T: ProtoZero> {
    /// Constructs an empty slot at message creation (`new_in` / `Default`).
    ///
    /// For raw `T`, writes type-zero via [`ProtoZero::set_proto_zero`]. For
    /// [`MaybeUninit`], leaves the slot uninitialized.
    fn new_empty() -> Self;

    /// Writes `value` into the slot **without** dropping a previous value.
    ///
    /// For [`MaybeUninit`], uses [`MaybeUninit::write`].
    ///
    /// For raw `T`, assigns in place (same as [`write_dropping_previous`](Self::write_dropping_previous)).
    ///
    /// # Safety
    ///
    /// For [`MaybeUninit`], the slot must be **uninitialized** before this call.
    unsafe fn write_without_drop(&mut self, value: T);

    /// Overwrites the slot with `value`, **dropping** the previous value if `T: Drop`.
    ///
    /// For [`MaybeUninit`], assigns through [`MaybeUninit::assume_init_mut`].
    ///
    /// # Safety
    ///
    /// For [`MaybeUninit`], the slot must be **initialized** before this call.
    unsafe fn write_dropping_previous(&mut self, value: T);

    /// Borrows the stored value.
    ///
    /// # Safety
    ///
    /// For [`MaybeUninit`] slots, the caller must ensure the slot is initialized
    /// (e.g. explicit presence bit is set).
    unsafe fn read_unchecked(&self) -> &T;
}

impl<T: ProtoZero> ValueSlot<T> for T {
    fn new_empty() -> Self {
        let mut slot = MaybeUninit::<T>::uninit();
        unsafe {
            T::set_proto_zero(&mut *slot.as_mut_ptr());
            slot.assume_init()
        }
    }

    unsafe fn write_without_drop(&mut self, value: T) {
        *self = value;
    }

    unsafe fn write_dropping_previous(&mut self, value: T) {
        *self = value;
    }

    unsafe fn read_unchecked(&self) -> &T {
        self
    }
}

impl<T: ProtoZero> ValueSlot<T> for MaybeUninit<T> {
    fn new_empty() -> Self {
        MaybeUninit::uninit()
    }

    unsafe fn write_without_drop(&mut self, value: T) {
        self.write(value);
    }

    unsafe fn write_dropping_previous(&mut self, value: T) {
        unsafe {
            *self.assume_init_mut() = value;
        }
    }

    unsafe fn read_unchecked(&self) -> &T {
        unsafe { self.assume_init_ref() }
    }
}
