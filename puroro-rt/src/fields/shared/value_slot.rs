//! Value slot storage for singular fields — always-initialized (`T`),
//! bit-tracked ([`MaybeUninit<T>`]), or pointer-present ([`Option<T>`]),
//! selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T`, [`MaybeUninit<T>`], and [`Option<T>`]
//! when `T` implements [`DefaultIn`] / [`unmanaged::DeallocateIn`] for the
//! message allocator `A`. Slot payloads are physical storage (`i32`, `()`,
//! `UnmanagedString`, [`UnmanagedBox`](::unmanaged::UnmanagedBox), …); logical
//! bit-packed bool values live in [`MessageCommon`](super::MessageCommon).
//!
//! Construction and teardown thread an allocator (via
//! [`DefaultIn`](super::DefaultIn) / [`unmanaged::DeallocateIn`]).
//! Read / mutation go through short-lived views from [`with`](ValueSlot::with) /
//! [`with_mut`](ValueSlot::with_mut), which return
//! [`ValueSlotRefAccess`] / [`ValueSlotMutAccess`] (RPIT).

use ::core::marker::PhantomData;
use ::core::mem::{self, MaybeUninit};

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::DeallocateIn;

use super::{
    DefaultIn, MessageCommon, PresenceBits,
    slot_init::{SlotInitMut, SlotInitView},
};

/// Marker for payloads stored as addressable `T` / [`MaybeUninit<T>`] in the
/// field slot.
pub trait AddressableSlot {}

impl AddressableSlot for u32 {}
impl AddressableSlot for u64 {}
impl AddressableSlot for i32 {}
impl AddressableSlot for i64 {}
impl AddressableSlot for () {}

impl<A: Allocator> AddressableSlot for ::unmanaged::UnmanagedString<A> {}
impl<A: Allocator> AddressableSlot for ::unmanaged::UnmanagedVec<u8, A> {}

/// Storage construction / teardown and view binding for a singular field value slot.
///
/// Prefer [`with`](Self::with) / [`with_mut`](Self::with_mut) for reads and
/// mutation. [`new_in`](Self::new_in) / [`deallocate_in`](Self::deallocate_in)
/// cover construction and message / oneof teardown.
pub trait ValueSlot<T, A>: Sized
where
    T: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    A: Allocator + Clone,
{
    /// Creates value storage when the parent message is constructed.
    ///
    /// For raw `T`, returns [`DefaultIn::default_in`] — the slot is always
    /// initialized. For [`MaybeUninit`], returns [`MaybeUninit::uninit`] and
    /// ignores `alloc` — the slot starts absent.
    fn new_in(alloc: A) -> Self;

    /// Consumes the slot and frees any live payload through `alloc`.
    ///
    /// Used from message / oneof `Drop` paths. `initialized` is taken from the
    /// init marker + [`MessageCommon`] by the caller.
    fn deallocate_in(self, initialized: bool, alloc: A);

    /// Pairs this slot with an init marker and message common for read access.
    fn with<'s, I: SlotInitView, Pb: PresenceBits>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T>;

    /// Pairs this slot with an init marker and message common for mutation.
    fn with_mut<'a, I: SlotInitMut, Pb: PresenceBits>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A>;
}

/// Read ops on a value-slot view whose physical payload type is `T`.
pub trait ValueSlotRefAccess<'s, T> {
    /// Borrows `&T` when initialized (`None` when the slot is not initialized).
    fn get(self) -> Option<&'s T>;
}

/// Mutation ops on a value-slot view whose physical payload type is `T`.
pub trait ValueSlotMutAccess<'a, T, A: Allocator + Clone> {
    /// Lazy-initializes when uninitialized, then returns `&mut T`.
    fn get_mut(self) -> &'a mut T;

    /// Assigns `value`, releasing any previously stored payload and updating
    /// init state when the slot was uninitialized.
    fn set(self, value: T);

    /// Drops an initialized payload and clears init state; the always-initialized
    /// variant reinstalls an empty value.
    fn clear(self);
}

// ---------------------------------------------------------------------------
// Addressable views
// ---------------------------------------------------------------------------

/// Short-lived read view of an addressable value slot.
pub struct ValueSlotRef<'s, S: ?Sized, T, I: SlotInitView, Pb: PresenceBits, A: Allocator> {
    slot: &'s S,
    init: I,
    common: &'s MessageCommon<Pb, A>,
    _t: PhantomData<T>,
}

/// Short-lived mutation view of an addressable value slot.
pub struct ValueSlotMut<'a, S: ?Sized, T, I: SlotInitMut, Pb: PresenceBits, A: Allocator> {
    slot: &'a mut S,
    init: I,
    common: &'a mut MessageCommon<Pb, A>,
    _t: PhantomData<T>,
}

impl<'s, T, I: SlotInitView, Pb: PresenceBits, A: Allocator> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, T, T, I, Pb, A>
where
    T: AddressableSlot,
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        Some(self.slot)
    }
}

impl<'s, T, I: SlotInitView, Pb: PresenceBits, A: Allocator> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, MaybeUninit<T>, T, I, Pb, A>
where
    T: AddressableSlot,
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        if self.init.is_initialized(self.common) {
            // SAFETY: init bit set implies a live payload.
            Some(unsafe { self.slot.assume_init_ref() })
        } else {
            None
        }
    }
}

impl<'a, T, I: SlotInitMut, Pb: PresenceBits, A: Allocator + Clone> ValueSlotMutAccess<'a, T, A>
    for ValueSlotMut<'a, T, T, I, Pb, A>
where
    T: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    #[inline]
    fn get_mut(self) -> &'a mut T {
        self.slot
    }

    #[inline]
    fn set(self, value: T) {
        let old = mem::replace(self.slot, value);
        // SAFETY: `common.alloc` owns `old`'s buffer.
        unsafe { old.deallocate_in(self.common.alloc.clone()) };
    }

    #[inline]
    fn clear(self) {
        let old = mem::replace(self.slot, T::default_in(self.common.alloc.clone()));
        // SAFETY: `common.alloc` owns `old`'s buffer.
        unsafe { old.deallocate_in(self.common.alloc.clone()) };
    }
}

impl<'a, T, I: SlotInitMut, Pb: PresenceBits, A: Allocator + Clone> ValueSlotMutAccess<'a, T, A>
    for ValueSlotMut<'a, MaybeUninit<T>, T, I, Pb, A>
where
    T: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    #[inline]
    fn get_mut(self) -> &'a mut T {
        if !self.init.is_initialized(self.common) {
            self.slot.write(T::default_in(self.common.alloc.clone()));
            self.init.set_initialized(self.common, true);
        }
        // SAFETY: just ensured the slot is initialized.
        unsafe { self.slot.assume_init_mut() }
    }

    #[inline]
    fn set(self, value: T) {
        if self.init.is_initialized(self.common) {
            // SAFETY: init bit set implies a live payload.
            let old = mem::replace(unsafe { self.slot.assume_init_mut() }, value);
            // SAFETY: `common.alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(self.common.alloc.clone()) };
        } else {
            self.slot.write(value);
            self.init.set_initialized(self.common, true);
        }
    }

    #[inline]
    fn clear(self) {
        if self.init.is_initialized(self.common) {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            let old = unsafe { self.slot.assume_init_read() };
            // SAFETY: `common.alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(self.common.alloc.clone()) };
            self.init.set_initialized(self.common, false);
        }
    }
}

impl<T, A> ValueSlot<T, A> for T
where
    T: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    A: Allocator + Clone,
{
    fn new_in(alloc: A) -> Self {
        T::default_in(alloc)
    }

    fn deallocate_in(self, _: bool, alloc: A) {
        // SAFETY: `alloc` owns this value's buffer.
        unsafe { DeallocateIn::deallocate_in(self, alloc) };
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb: PresenceBits>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T> {
        ValueSlotRef {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, Pb: PresenceBits>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A> {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}

impl<T, A> ValueSlot<T, A> for MaybeUninit<T>
where
    T: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    A: Allocator + Clone,
{
    fn new_in(_alloc: A) -> Self {
        MaybeUninit::uninit()
    }

    fn deallocate_in(self, initialized: bool, alloc: A) {
        if initialized {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            let value = unsafe { self.assume_init() };
            // SAFETY: `alloc` owns `value`'s buffer.
            unsafe { DeallocateIn::deallocate_in(value, alloc) };
        }
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb: PresenceBits>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T> {
        ValueSlotRef {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, Pb: PresenceBits>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A> {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}

impl<'s, T, I: SlotInitView, Pb: PresenceBits, A: Allocator> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, Option<T>, T, I, Pb, A>
where
    T: AddressableSlot,
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        self.slot.as_ref()
    }
}

impl<'a, T, I: SlotInitMut, Pb: PresenceBits, A: Allocator + Clone> ValueSlotMutAccess<'a, T, A>
    for ValueSlotMut<'a, Option<T>, T, I, Pb, A>
where
    T: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    #[inline]
    fn get_mut(self) -> &'a mut T {
        self.slot
            .get_or_insert_with(|| T::default_in(self.common.alloc.clone()))
    }

    #[inline]
    fn set(self, value: T) {
        if let Some(old) = self.slot.replace(value) {
            // SAFETY: `common.alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(self.common.alloc.clone()) };
        }
    }

    #[inline]
    fn clear(self) {
        if let Some(old) = self.slot.take() {
            // SAFETY: `common.alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(self.common.alloc.clone()) };
        }
    }
}

impl<T, A> ValueSlot<T, A> for Option<T>
where
    T: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    A: Allocator + Clone,
{
    fn new_in(_alloc: A) -> Self {
        None
    }

    fn deallocate_in(self, _: bool, alloc: A) {
        if let Some(value) = self {
            // SAFETY: `alloc` owns `value`'s buffer.
            unsafe { DeallocateIn::deallocate_in(value, alloc) };
        }
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb: PresenceBits>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T> {
        ValueSlotRef {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, Pb: PresenceBits>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A> {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}
