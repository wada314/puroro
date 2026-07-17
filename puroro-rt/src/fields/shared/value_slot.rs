//! Value slot storage for singular fields — always-initialized (`T`),
//! bit-tracked ([`MaybeUninit<T>`]), or pointer-present ([`Option<T>`]),
//! selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T`, [`MaybeUninit<T>`], and [`Option<T>`]
//! when `T` is [`AddressableSlot`]. Slot payloads are physical storage (thin
//! wrappers, ZST [`ProtoBool`](crate::ProtoBool) for bit-packed bool, or
//! [`UnmanagedBox`](::unmanaged::UnmanagedBox) for nested messages); logical
//! bit-packed values live in [`MessageCommon`](super::MessageCommon).
//!
//! Construction and teardown thread an allocator (via
//! [`DefaultIn`](super::DefaultIn) / [`DeallocateIn`](super::DeallocateIn)).
//! Read / mutation go through short-lived views from [`with`](ValueSlot::with) /
//! [`with_mut`](ValueSlot::with_mut), which return
//! [`ValueSlotRefAccess`] / [`ValueSlotMutAccess`] (RPIT).

use ::core::marker::PhantomData;
use ::core::mem::MaybeUninit;

use ::allocator_api2::alloc::Allocator;

use super::{
    DeallocateIn, DefaultIn, MessageCommon, PresenceBits,
    slot_init::{SlotInitMut, SlotInitView},
};
use crate::fields::wire::len::{ProtoBytes, ProtoString};
use crate::fields::wire::varint::{
    ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64,
    ProtoUInt32, ProtoUInt64,
};

/// Marker for payloads stored as addressable `T` / [`MaybeUninit<T>`] in the
/// field slot (varint wrappers, LEN wrappers, enums, and ZST [`ProtoBool`] for
/// bit-packed bool presence/init only).
pub trait AddressableSlot:
    DefaultIn<Alloc = Self::SlotAlloc> + DeallocateIn<Alloc = Self::SlotAlloc>
{
    type SlotAlloc: Allocator + Clone;
}

impl<A: Allocator + Clone> AddressableSlot for ProtoUInt32<A> { type SlotAlloc = A; }
impl<A: Allocator + Clone> AddressableSlot for ProtoUInt64<A> { type SlotAlloc = A; }
impl<A: Allocator + Clone> AddressableSlot for ProtoInt32<A> { type SlotAlloc = A; }
impl<A: Allocator + Clone> AddressableSlot for ProtoInt64<A> { type SlotAlloc = A; }
impl<A: Allocator + Clone> AddressableSlot for ProtoSint32<A> { type SlotAlloc = A; }
impl<A: Allocator + Clone> AddressableSlot for ProtoSint64<A> { type SlotAlloc = A; }
impl<A: Allocator + Clone> AddressableSlot for ProtoString<A> { type SlotAlloc = A; }
impl<A: Allocator + Clone> AddressableSlot for ProtoBytes<A> { type SlotAlloc = A; }
impl<E: ProtoEnumStorage, K, A: Allocator + Clone> AddressableSlot for ProtoEnum<E, K, A> { type SlotAlloc = A; }
impl<A: Allocator + Clone, const VALUE_BIT: usize> AddressableSlot for ProtoBool<A, VALUE_BIT> { type SlotAlloc = A; }

/// Storage construction / teardown and view binding for a singular field value slot.
///
/// Prefer [`with`](Self::with) / [`with_mut`](Self::with_mut) for reads and
/// mutation. [`new_in`](Self::new_in) / [`deallocate_in`](Self::deallocate_in)
/// cover construction and message / oneof teardown.
pub trait ValueSlot<T: AddressableSlot>: Sized {
    /// Creates value storage when the parent message is constructed.
    ///
    /// For raw `T`, returns [`DefaultIn::default_in`] — the slot is always
    /// initialized. For [`MaybeUninit`], returns [`MaybeUninit::uninit`] and
    /// ignores `alloc` — the slot starts absent.
    fn new_in(alloc: <T as DefaultIn>::Alloc) -> Self;

    /// Consumes the slot and frees any live payload through `alloc`.
    ///
    /// Used from message / oneof `Drop` paths. `initialized` is taken from the
    /// init marker + [`MessageCommon`] by the caller.
    fn deallocate_in(self, initialized: bool, alloc: <T as DefaultIn>::Alloc);

    /// Pairs this slot with an init marker and message common for read access.
    fn with<'s, I: SlotInitView, Pb: PresenceBits>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, T::SlotAlloc>,
    ) -> impl ValueSlotRefAccess<'s, T>;

    /// Pairs this slot with an init marker and message common for mutation.
    fn with_mut<'a, I: SlotInitMut, Pb: PresenceBits>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, T::SlotAlloc>,
    ) -> impl ValueSlotMutAccess<'a, T>;
}

/// Read ops on a value-slot view whose physical payload type is `T`.
pub trait ValueSlotRefAccess<'s, T> {
    /// Borrows `&T` when initialized (`None` when the slot is not initialized).
    fn get(self) -> Option<&'s T>;
}

/// Mutation ops on a value-slot view whose physical payload type is `T`.
pub trait ValueSlotMutAccess<'a, T> {
    /// Lazy-initializes when uninitialized, then returns `&mut T`.
    ///
    /// Callers that need a logical accessor chain through
    /// [`ProtoType::with_mut`](crate::fields::wire::proto_type::ProtoType::with_mut).
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

impl<'s, T: AddressableSlot, I: SlotInitView, Pb: PresenceBits, A: Allocator>
    ValueSlotRefAccess<'s, T> for ValueSlotRef<'s, T, T, I, Pb, A>
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        Some(self.slot)
    }
}

impl<'s, T: AddressableSlot, I: SlotInitView, Pb: PresenceBits, A: Allocator>
    ValueSlotRefAccess<'s, T> for ValueSlotRef<'s, MaybeUninit<T>, T, I, Pb, A>
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

/// Short-lived mutation view of an addressable value slot.
pub struct ValueSlotMut<'a, S: ?Sized, T, I: SlotInitMut, Pb: PresenceBits, A: Allocator> {
    slot: &'a mut S,
    init: I,
    common: &'a mut MessageCommon<Pb, A>,
    _t: PhantomData<T>,
}

impl<'a, T: AddressableSlot<SlotAlloc = A>, I: SlotInitMut, Pb: PresenceBits, A: Allocator + Clone>
    ValueSlotMutAccess<'a, T> for ValueSlotMut<'a, T, T, I, Pb, A>
{
    #[inline]
    fn get_mut(self) -> &'a mut T {
        self.slot
    }

    #[inline]
    fn set(self, value: T) {
        let old = ::core::mem::replace(self.slot, value);
        // SAFETY: `common.alloc` owns `old`'s buffer.
        unsafe { old.deallocate_in(self.common.alloc.clone()) };
    }

    #[inline]
    fn clear(self) {
        let old = ::core::mem::replace(self.slot, T::default_in(self.common.alloc.clone()));
        // SAFETY: `common.alloc` owns `old`'s buffer.
        unsafe { old.deallocate_in(self.common.alloc.clone()) };
    }
}

impl<'a, T: AddressableSlot<SlotAlloc = A>, I: SlotInitMut, Pb: PresenceBits, A: Allocator + Clone>
    ValueSlotMutAccess<'a, T> for ValueSlotMut<'a, MaybeUninit<T>, T, I, Pb, A>
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
            let old = ::core::mem::replace(unsafe { self.slot.assume_init_mut() }, value);
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

impl<T: AddressableSlot> ValueSlot<T> for T {
    fn new_in(alloc: <T as DefaultIn>::Alloc) -> Self {
        T::default_in(alloc)
    }

    fn deallocate_in(self, _: bool, alloc: <T as DefaultIn>::Alloc) {
        // SAFETY: `alloc` owns this value's buffer.
        unsafe { DeallocateIn::deallocate_in(self, alloc) };
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb: PresenceBits>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, T::SlotAlloc>,
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
        common: &'a mut MessageCommon<Pb, T::SlotAlloc>,
    ) -> impl ValueSlotMutAccess<'a, T> {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}

impl<T: AddressableSlot> ValueSlot<T> for MaybeUninit<T> {
    fn new_in(_alloc: <T as DefaultIn>::Alloc) -> Self {
        MaybeUninit::uninit()
    }

    fn deallocate_in(self, initialized: bool, alloc: <T as DefaultIn>::Alloc) {
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
        common: &'s MessageCommon<Pb, T::SlotAlloc>,
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
        common: &'a mut MessageCommon<Pb, T::SlotAlloc>,
    ) -> impl ValueSlotMutAccess<'a, T> {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}

impl<'s, T: AddressableSlot, I: SlotInitView, Pb: PresenceBits, A: Allocator>
    ValueSlotRefAccess<'s, T> for ValueSlotRef<'s, Option<T>, T, I, Pb, A>
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        self.slot.as_ref()
    }
}

impl<'a, T: AddressableSlot<SlotAlloc = A>, I: SlotInitMut, Pb: PresenceBits, A: Allocator + Clone>
    ValueSlotMutAccess<'a, T> for ValueSlotMut<'a, Option<T>, T, I, Pb, A>
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

impl<T: AddressableSlot> ValueSlot<T> for Option<T> {
    fn new_in(_alloc: <T as DefaultIn>::Alloc) -> Self {
        None
    }

    fn deallocate_in(self, _: bool, alloc: <T as DefaultIn>::Alloc) {
        if let Some(value) = self {
            // SAFETY: `alloc` owns `value`'s buffer.
            unsafe { DeallocateIn::deallocate_in(value, alloc) };
        }
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb: PresenceBits>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, T::SlotAlloc>,
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
        common: &'a mut MessageCommon<Pb, T::SlotAlloc>,
    ) -> impl ValueSlotMutAccess<'a, T> {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}
