//! Value slot storage for singular fields — always-initialized (`T`) or
//! presence-tracked ([`MaybeUninit<T>`]), selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T` and [`MaybeUninit<T>`]. The trait's
//! type parameter `T` disambiguates the two blanket impls so they do not overlap.
//!
//! Construction and teardown thread an allocator (via
//! [`DefaultIn`](super::DefaultIn) / [`DeallocateIn`](super::DeallocateIn)).
//! Read / mutation go through short-lived views from [`with`](ValueSlot::with) /
//! [`with_mut`](ValueSlot::with_mut), which return
//! [`ValueSlotRefAccess`] / [`ValueSlotMutAccess`] (RPIT). Those traits are
//! implemented separately for views over storage `T` and [`MaybeUninit<T>`].

use ::core::marker::PhantomData;
use ::core::mem::MaybeUninit;

use ::allocator_api2::alloc::Allocator;

use super::{
    DeallocateIn, DefaultIn,
    slot_init::{SlotInitMut, SlotInitView},
};

/// Storage construction / teardown and view binding for a singular field value slot.
///
/// Prefer [`with`](Self::with) / [`with_mut`](Self::with_mut) for reads and
/// mutation. [`new_in`](Self::new_in) / [`deallocate_in`](Self::deallocate_in)
/// cover construction and message / oneof teardown.
///
/// Bit-packed bools use a separate [`BoolField`](crate::BoolField) rather than
/// this trait, because a bit is not addressable as `&mut T`.
pub trait ValueSlot<T: DefaultIn + DeallocateIn>: Sized {
    /// Creates value storage when the parent message is constructed.
    ///
    /// For raw `T`, returns [`DefaultIn::default_in`] — the slot is always
    /// initialized. For [`MaybeUninit`], returns [`MaybeUninit::uninit`] and
    /// ignores `alloc` — the slot starts absent.
    fn new_in<A: Allocator>(alloc: A) -> Self;

    /// Consumes the slot and frees any live payload through `alloc`.
    ///
    /// Used from message / oneof `Drop` paths. For [`MaybeUninit`] the init view
    /// decides whether a payload exists; always-initialized slots always free.
    fn deallocate_in<A: Allocator>(self, init: &impl SlotInitView, alloc: A);

    /// Pairs this slot with an init view for read access.
    fn with<'s, 'i, I: SlotInitView>(&'s self, init: &'i I) -> impl ValueSlotRefAccess<'s, T>;

    /// Pairs this slot with init state and an allocator for mutation.
    fn with_mut<'a, I: SlotInitMut, A: Allocator + Clone>(
        &'a mut self,
        init: I,
        alloc: A,
    ) -> impl ValueSlotMutAccess<'a, T>;
}

/// Read ops on a value-slot view whose logical payload type is `T`.
///
/// Separate impls for views over storage `T` and [`MaybeUninit<T>`] distinguish
/// always-init vs presence-tracked slots.
pub trait ValueSlotRefAccess<'s, T> {
    /// Borrows `&T` when initialized (`None` when the slot is not initialized).
    fn get(self) -> Option<&'s T>;
}

/// Mutation ops on a value-slot view whose logical payload type is `T`.
///
/// Separate impls for views over storage `T` and [`MaybeUninit<T>`] distinguish
/// always-init vs presence-tracked slots.
pub trait ValueSlotMutAccess<'a, T> {
    /// Lazy-initializes with [`DefaultIn::default_in`] when uninitialized, then
    /// returns `&mut T`.
    ///
    /// Callers that need a growable guard (`StringGuard`, …) chain
    /// `.with_mut(alloc)` on the result.
    fn get_mut(self) -> &'a mut T;

    /// Assigns `value`, releasing any previously stored payload through the
    /// bound allocator and updating init state when the slot was uninitialized.
    ///
    /// Unlike [`get_mut`](Self::get_mut), the slot does **not** need to be
    /// initialized first — for [`MaybeUninit`] this uses [`MaybeUninit::write`]
    /// on first assign.
    fn set(self, value: T);

    /// Drops an initialized payload and clears init state; the always-initialized
    /// variant reinstalls an empty value.
    fn clear(self);
}

/// Short-lived read view of a value slot paired with init state.
///
/// `'s` is the slot borrow (and thus the lifetime of
/// [`ValueSlotRefAccess::get`]); `'i` is only the init-view borrow and need not
/// outlive the returned `&T`.
pub struct ValueSlotRef<'s, 'i, S: ?Sized, T, I: SlotInitView> {
    slot: &'s S,
    init: &'i I,
    _t: PhantomData<T>,
}

impl<'s, 'i, T: DefaultIn + DeallocateIn, I: SlotInitView> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, 'i, T, T, I>
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        Some(self.slot)
    }
}

impl<'s, 'i, T: DefaultIn + DeallocateIn, I: SlotInitView> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, 'i, MaybeUninit<T>, T, I>
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        if self.init.is_initialized() {
            // SAFETY: init bit set implies a live payload.
            Some(unsafe { self.slot.assume_init_ref() })
        } else {
            None
        }
    }
}

/// Short-lived mutation view of a value slot paired with init state and an
/// allocator. Every method consumes the view.
pub struct ValueSlotMut<'a, S: ?Sized, T, I: SlotInitMut, A: Allocator> {
    slot: &'a mut S,
    init: I,
    alloc: A,
    _t: PhantomData<T>,
}

impl<'a, T: DefaultIn + DeallocateIn, I: SlotInitMut, A: Allocator + Clone> ValueSlotMutAccess<'a, T>
    for ValueSlotMut<'a, T, T, I, A>
{
    #[inline]
    fn get_mut(self) -> &'a mut T {
        self.slot
    }

    #[inline]
    fn set(self, value: T) {
        let old = ::core::mem::replace(self.slot, value);
        // SAFETY: `alloc` is the message allocator that owns `old`'s buffer.
        unsafe { old.deallocate_in(self.alloc) };
    }

    #[inline]
    fn clear(self) {
        let old = ::core::mem::replace(self.slot, T::default_in(self.alloc.clone()));
        // SAFETY: `alloc` owns `old`'s buffer.
        unsafe { old.deallocate_in(self.alloc) };
    }
}

impl<'a, T: DefaultIn + DeallocateIn, I: SlotInitMut, A: Allocator + Clone> ValueSlotMutAccess<'a, T>
    for ValueSlotMut<'a, MaybeUninit<T>, T, I, A>
{
    #[inline]
    fn get_mut(mut self) -> &'a mut T {
        if !self.init.is_initialized() {
            self.slot.write(T::default_in(self.alloc));
            self.init.set_initialized(true);
        }
        // SAFETY: just ensured the slot is initialized.
        unsafe { self.slot.assume_init_mut() }
    }

    #[inline]
    fn set(mut self, value: T) {
        if self.init.is_initialized() {
            // SAFETY: init bit set implies a live payload.
            let old = ::core::mem::replace(unsafe { self.slot.assume_init_mut() }, value);
            // SAFETY: `alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(self.alloc) };
        } else {
            self.slot.write(value);
            self.init.set_initialized(true);
        }
    }

    #[inline]
    fn clear(mut self) {
        if self.init.is_initialized() {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            let old = unsafe { self.slot.assume_init_read() };
            // SAFETY: `alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(self.alloc) };
            self.init.set_initialized(false);
        }
    }
}

impl<T: DefaultIn + DeallocateIn> ValueSlot<T> for T {
    fn new_in<A: Allocator>(alloc: A) -> Self {
        T::default_in(alloc)
    }

    fn deallocate_in<A: Allocator>(self, _: &impl SlotInitView, alloc: A) {
        // SAFETY: `alloc` owns this value's buffer.
        unsafe { DeallocateIn::deallocate_in(self, alloc) };
    }

    #[inline]
    fn with<'s, 'i, I: SlotInitView>(&'s self, init: &'i I) -> impl ValueSlotRefAccess<'s, T> {
        ValueSlotRef {
            slot: self,
            init,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, A: Allocator + Clone>(
        &'a mut self,
        init: I,
        alloc: A,
    ) -> impl ValueSlotMutAccess<'a, T> {
        ValueSlotMut {
            slot: self,
            init,
            alloc,
            _t: PhantomData,
        }
    }
}

impl<T: DefaultIn + DeallocateIn> ValueSlot<T> for MaybeUninit<T> {
    fn new_in<A: Allocator>(_alloc: A) -> Self {
        MaybeUninit::uninit()
    }

    fn deallocate_in<A: Allocator>(self, init: &impl SlotInitView, alloc: A) {
        if init.is_initialized() {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            let value = unsafe { self.assume_init() };
            // SAFETY: `alloc` owns `value`'s buffer.
            unsafe { DeallocateIn::deallocate_in(value, alloc) };
        }
    }

    #[inline]
    fn with<'s, 'i, I: SlotInitView>(&'s self, init: &'i I) -> impl ValueSlotRefAccess<'s, T> {
        ValueSlotRef {
            slot: self,
            init,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, A: Allocator + Clone>(
        &'a mut self,
        init: I,
        alloc: A,
    ) -> impl ValueSlotMutAccess<'a, T> {
        ValueSlotMut {
            slot: self,
            init,
            alloc,
            _t: PhantomData,
        }
    }
}
