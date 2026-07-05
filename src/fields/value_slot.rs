//! Value slot storage for singular fields — always-initialized (`T`) or
//! presence-tracked ([`MaybeUninit<T>`]), selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T` and [`MaybeUninit<T>`]. The trait's
//! type parameter `T` disambiguates the two blanket impls so they do not overlap.

use ::core::mem::MaybeUninit;

/// Operations on a singular field's value slot.
pub trait ValueSlot<T> {
    /// Constructs an empty slot at message creation (`new_in` / `Default`).
    ///
    /// For raw `T`, stores `proto_zero`. For [`MaybeUninit`], leaves the slot
    /// uninitialized.
    fn new_in(proto_zero: T) -> Self;

    /// Writes a value after merge or setter.
    ///
    /// `replacing` is `true` when the slot already holds an initialized value
    /// (e.g. explicit presence was already set).
    fn write(&mut self, value: T, replacing: bool);

    /// Clears the payload slot (bitfield updates are handled separately).
    ///
    /// For raw `T`, always stores `proto_zero`. For [`MaybeUninit`], drops the
    /// initialized value when `was_set` is `true`, leaving the slot uninitialized.
    fn clear(&mut self, proto_zero: T, was_set: bool);

    /// `true` when the payload equals the protobuf type-zero (implicit omit rule).
    ///
    /// Only called for implicit presence policies; explicit policies use the
    /// message bitfield instead.
    fn is_payload_empty(&self, proto_zero: T) -> bool;

    /// Borrows the stored value.
    ///
    /// # Safety
    ///
    /// For [`MaybeUninit`] slots, the caller must ensure the slot is initialized
    /// (e.g. explicit presence bit is set).
    unsafe fn read_unchecked(&self) -> &T;

    /// Mutably borrows the stored value.
    ///
    /// # Safety
    ///
    /// Same as [`read_unchecked`](Self::read_unchecked).
    unsafe fn mut_unchecked(&mut self) -> &mut T;
}

impl<T: PartialEq> ValueSlot<T> for T {
    fn new_in(proto_zero: T) -> Self {
        proto_zero
    }

    fn write(&mut self, value: T, _replacing: bool) {
        *self = value;
    }

    fn clear(&mut self, proto_zero: T, _was_set: bool) {
        *self = proto_zero;
    }

    fn is_payload_empty(&self, proto_zero: T) -> bool {
        *self == proto_zero
    }

    unsafe fn read_unchecked(&self) -> &T {
        self
    }

    unsafe fn mut_unchecked(&mut self) -> &mut T {
        self
    }
}

impl<T> ValueSlot<T> for MaybeUninit<T> {
    fn new_in(_proto_zero: T) -> Self {
        MaybeUninit::uninit()
    }

    fn write(&mut self, value: T, replacing: bool) {
        if replacing {
            unsafe { *self.assume_init_mut() = value };
        } else {
            self.write(value);
        }
    }

    fn clear(&mut self, _proto_zero: T, was_set: bool) {
        if was_set {
            unsafe { self.assume_init_drop() };
        }
    }

    fn is_payload_empty(&self, _proto_zero: T) -> bool {
        false
    }

    unsafe fn read_unchecked(&self) -> &T {
        unsafe { self.assume_init_ref() }
    }

    unsafe fn mut_unchecked(&mut self) -> &mut T {
        unsafe { self.assume_init_mut() }
    }
}
