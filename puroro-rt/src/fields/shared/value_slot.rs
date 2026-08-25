//! Value slot storage for singular fields — always-initialized (`T`),
//! bit-tracked ([`MaybeUninit<T>`]), or pointer-present ([`Option<T>`]),
//! selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T`, [`MaybeUninit<T>`], and [`Option<T>`]
//! when `T: AddressableSlot`. Always-init construction ([`ValueSlotNew`]) needs
//! [`DefaultIn`] for raw `T` only. Slot payloads
//! are physical storage (`i32`, `()`, SSO slots, [`UnmanagedBox`](::unmanaged::UnmanagedBox),
//! …); logical bit-packed bool values live in [`MessageCommon`](super::MessageCommon).
//!
//! Construction uses [`ValueSlotNew::new_in`](ValueSlotNew::new_in). Message Drop / clone of a
//! slot go through [`take_value`](ValueSlot::take_value) /
//! [`get_value`](ValueSlot::get_value) / [`from_optional`](ValueSlot::from_optional);
//! how the live payload is freed or deep-copied is decided by
//! [`ValueLayout`](super::value_layout::ValueLayout). Mut views
//! ([`replace`](ValueSlotMutAccess::replace) / [`take_clear`](ValueSlotMutAccess::take_clear))
//! only extract the previous payload — callers release it.
//! Read / mutation go through short-lived views from [`with`](ValueSlot::with) /
//! [`with_mut`](ValueSlot::with_mut), which return
//! [`ValueSlotRefAccess`] / [`ValueSlotMutAccess`] (RPIT).

use ::core::marker::PhantomData;
use ::core::mem::{self, MaybeUninit};

use ::allocator_api2::alloc::Allocator;

use super::{
    DefaultIn, MessageCommon, MessageCommonBits,
    slot_init::{SlotInitMut, SlotInitView},
};
use crate::fields::wire::sso_buf::SsoBuf;

/// Marker for payloads stored as addressable `T` / [`MaybeUninit<T>`] in the
/// field slot.
pub trait AddressableSlot {}

impl AddressableSlot for u32 {}
impl AddressableSlot for u64 {}
impl AddressableSlot for i32 {}
impl AddressableSlot for i64 {}
impl AddressableSlot for f32 {}
impl AddressableSlot for f64 {}
impl AddressableSlot for bool {}
impl AddressableSlot for () {}

impl<A: Allocator> AddressableSlot for ::unmanaged::UnmanagedString<A> {}
impl<A: Allocator> AddressableSlot for ::unmanaged::UnmanagedVec<u8, A> {}
impl<H> AddressableSlot for SsoBuf<H> {}

/// Storage construction, live-payload extract/rebuild, and view binding for a
/// singular field value slot.
///
/// Prefer [`with`](Self::with) / [`with_mut`](Self::with_mut) for reads and
/// mutation. Message Drop / field clone use [`take_value`](Self::take_value) /
/// [`get_value`](Self::get_value) / [`from_optional`](Self::from_optional);
/// [`ValueLayout`](super::value_layout::ValueLayout) decides how to free or
/// deep-copy the extracted `T`.
pub trait ValueSlot<T, A>: Sized
where
    T: AddressableSlot,
    A: Allocator,
{
    /// Extracts a live payload, if any.
    ///
    /// `initialized` is ignored for always-present / pointer-present slots
    /// (`T`, [`Option<T>`]). Callers obtain it from the presence init marker +
    /// [`MessageCommon`] when the slot is bit-tracked ([`MaybeUninit<T>`]).
    fn take_value(self, initialized: bool) -> Option<T>;

    /// Borrows a live payload, if any. Same `initialized` rules as
    /// [`take_value`](Self::take_value).
    fn get_value(&self, initialized: bool) -> Option<&T>;

    /// Rebuilds slot storage from an optional live payload (clone path).
    fn from_optional(value: Option<T>) -> Self;

    /// Pairs this slot with an init marker and message common for read access.
    fn with<'s, I: SlotInitView, Pb>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T>
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    /// Pairs this slot with an init marker and message common for mutation.
    fn with_mut<'a, I: SlotInitMut, Pb>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits;
}

/// Allocator-aware slot construction. Separate from [`ValueSlot`] so always-init
/// `T` can be *named* without [`DefaultIn`] (oneof nested messages).
pub trait ValueSlotNew<T, A>: ValueSlot<T, A>
where
    T: AddressableSlot,
    A: Allocator,
{
    /// Creates value storage when the parent message is constructed.
    ///
    /// For raw `T`, returns [`DefaultIn::default_in`] — the slot is always
    /// initialized. For [`MaybeUninit`], returns [`MaybeUninit::uninit`] and
    /// ignores `alloc` — the slot starts absent.
    fn new_in(alloc: A) -> Self;
}

/// Read ops on a value-slot view whose physical payload type is `T`.
pub trait ValueSlotRefAccess<'s, T> {
    /// Borrows `&T` when initialized (`None` when the slot is not initialized).
    fn get(self) -> Option<&'s T>;
}

/// Mutation ops on a value-slot view whose physical payload type is `T`.
pub trait ValueSlotMutAccess<'a, T, A: Allocator> {
    /// Lazy-initializes when uninitialized, then returns `&mut T`.
    fn get_mut(self) -> &'a mut T
    where
        A: Clone,
        T: DefaultIn<A>;

    /// Installs `value` and returns the previous live payload, if any.
    ///
    /// The caller must release the returned value (typically
    /// [`DeallocateIn::deallocate_in`](::unmanaged::DeallocateIn::deallocate_in))
    /// with the **parent** message allocator that owned the previous payload.
    fn replace(self, value: T) -> Option<T>;

    /// Clears init state and returns the previous live payload, if any.
    ///
    /// Always-initialized slots reinstall [`DefaultIn::default_in`] and still
    /// return the old value. The caller must release it.
    fn take_clear(self) -> Option<T>
    where
        A: Clone,
        T: DefaultIn<A>;
}

// ---------------------------------------------------------------------------
// Addressable views
// ---------------------------------------------------------------------------

/// Short-lived read view of an addressable value slot.
pub(crate) struct ValueSlotRef<'s, S: ?Sized, T, I: SlotInitView, Pb, A: Allocator> {
    slot: &'s S,
    init: I,
    common: &'s MessageCommon<Pb, A>,
    _t: PhantomData<T>,
}

/// Short-lived mutation view of an addressable value slot.
pub(crate) struct ValueSlotMut<'a, S: ?Sized, T, I: SlotInitMut, Pb, A: Allocator> {
    slot: &'a mut S,
    init: I,
    common: &'a mut MessageCommon<Pb, A>,
    _t: PhantomData<T>,
}

impl<'s, T, I: SlotInitView, Pb, A: Allocator> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, T, T, I, Pb, A>
where
    T: AddressableSlot,
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        Some(self.slot)
    }
}

impl<'s, T, I: SlotInitView, Pb, A: Allocator> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, MaybeUninit<T>, T, I, Pb, A>
where
    T: AddressableSlot,
    MessageCommon<Pb, A>: MessageCommonBits,
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        if self.init.is_initialized(|b| self.common.is_bit_set(b)) {
            // SAFETY: init bit set implies a live payload.
            Some(unsafe { self.slot.assume_init_ref() })
        } else {
            None
        }
    }
}

impl<'a, T, I: SlotInitMut, Pb, A: Allocator> ValueSlotMutAccess<'a, T, A>
    for ValueSlotMut<'a, T, T, I, Pb, A>
where
    T: AddressableSlot,
{
    #[inline]
    fn get_mut(self) -> &'a mut T
    where
        A: Clone,
        T: DefaultIn<A>,
    {
        self.slot
    }

    #[inline]
    fn replace(self, value: T) -> Option<T> {
        Some(mem::replace(self.slot, value))
    }

    #[inline]
    fn take_clear(self) -> Option<T>
    where
        A: Clone,
        T: DefaultIn<A>,
    {
        Some(mem::replace(
            self.slot,
            T::default_in(self.common.alloc.clone()),
        ))
    }
}

impl<'a, T, I: SlotInitMut, Pb, A: Allocator> ValueSlotMutAccess<'a, T, A>
    for ValueSlotMut<'a, MaybeUninit<T>, T, I, Pb, A>
where
    T: AddressableSlot,
    MessageCommon<Pb, A>: MessageCommonBits,
{
    #[inline]
    fn get_mut(self) -> &'a mut T
    where
        A: Clone,
        T: DefaultIn<A>,
    {
        if !self.init.is_initialized(|b| self.common.is_bit_set(b)) {
            self.slot.write(T::default_in(self.common.alloc.clone()));
            self.init
                .set_initialized(|b, v| self.common.set_bit(b, v), true);
        }
        // SAFETY: just ensured the slot is initialized.
        unsafe { self.slot.assume_init_mut() }
    }

    #[inline]
    fn replace(self, value: T) -> Option<T> {
        if self.init.is_initialized(|b| self.common.is_bit_set(b)) {
            // SAFETY: init bit set implies a live payload.
            Some(mem::replace(unsafe { self.slot.assume_init_mut() }, value))
        } else {
            self.slot.write(value);
            self.init
                .set_initialized(|b, v| self.common.set_bit(b, v), true);
            None
        }
    }

    #[inline]
    fn take_clear(self) -> Option<T>
    where
        A: Clone,
        T: DefaultIn<A>,
    {
        if self.init.is_initialized(|b| self.common.is_bit_set(b)) {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            let old = unsafe { self.slot.assume_init_read() };
            self.init
                .set_initialized(|b, v| self.common.set_bit(b, v), false);
            Some(old)
        } else {
            None
        }
    }
}

impl<T, A> ValueSlot<T, A> for T
where
    T: AddressableSlot,
    A: Allocator,
{
    #[inline]
    fn take_value(self, _: bool) -> Option<T> {
        Some(self)
    }

    #[inline]
    fn get_value(&self, _: bool) -> Option<&T> {
        Some(self)
    }

    #[inline]
    fn from_optional(value: Option<T>) -> Self {
        value.expect("always-initialized ValueSlot cannot be rebuilt from None")
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlotRef {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, Pb>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}

impl<T, A> ValueSlotNew<T, A> for T
where
    T: AddressableSlot + DefaultIn<A>,
    A: Allocator,
{
    fn new_in(alloc: A) -> Self {
        T::default_in(alloc)
    }
}

impl<T, A> ValueSlot<T, A> for MaybeUninit<T>
where
    T: AddressableSlot,
    A: Allocator,
{
    #[inline]
    fn take_value(self, initialized: bool) -> Option<T> {
        if initialized {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            Some(unsafe { self.assume_init() })
        } else {
            None
        }
    }

    #[inline]
    fn get_value(&self, initialized: bool) -> Option<&T> {
        if initialized {
            // SAFETY: init bit set implies a live payload.
            Some(unsafe { self.assume_init_ref() })
        } else {
            None
        }
    }

    #[inline]
    fn from_optional(value: Option<T>) -> Self {
        match value {
            Some(v) => MaybeUninit::new(v),
            None => MaybeUninit::uninit(),
        }
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlotRef {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, Pb>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}

impl<T, A> ValueSlotNew<T, A> for MaybeUninit<T>
where
    T: AddressableSlot,
    A: Allocator,
{
    fn new_in(_alloc: A) -> Self {
        MaybeUninit::uninit()
    }
}

impl<'s, T, I: SlotInitView, Pb, A: Allocator> ValueSlotRefAccess<'s, T>
    for ValueSlotRef<'s, Option<T>, T, I, Pb, A>
where
    T: AddressableSlot,
{
    #[inline]
    fn get(self) -> Option<&'s T> {
        self.slot.as_ref()
    }
}

impl<'a, T, I: SlotInitMut, Pb, A: Allocator> ValueSlotMutAccess<'a, T, A>
    for ValueSlotMut<'a, Option<T>, T, I, Pb, A>
where
    T: AddressableSlot,
{
    #[inline]
    fn get_mut(self) -> &'a mut T
    where
        A: Clone,
        T: DefaultIn<A>,
    {
        self.slot
            .get_or_insert_with(|| T::default_in(self.common.alloc.clone()))
    }

    #[inline]
    fn replace(self, value: T) -> Option<T> {
        self.slot.replace(value)
    }

    #[inline]
    fn take_clear(self) -> Option<T>
    where
        A: Clone,
        T: DefaultIn<A>,
    {
        self.slot.take()
    }
}

impl<T, A> ValueSlot<T, A> for Option<T>
where
    T: AddressableSlot,
    A: Allocator,
{
    #[inline]
    fn take_value(self, _: bool) -> Option<T> {
        self
    }

    #[inline]
    fn get_value(&self, _: bool) -> Option<&T> {
        self.as_ref()
    }

    #[inline]
    fn from_optional(value: Option<T>) -> Self {
        value
    }

    #[inline]
    fn with<'s, I: SlotInitView, Pb>(
        &'s self,
        init: I,
        common: &'s MessageCommon<Pb, A>,
    ) -> impl ValueSlotRefAccess<'s, T>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlotRef {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }

    #[inline]
    fn with_mut<'a, I: SlotInitMut, Pb>(
        &'a mut self,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl ValueSlotMutAccess<'a, T, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlotMut {
            slot: self,
            init,
            common,
            _t: PhantomData,
        }
    }
}

impl<T, A> ValueSlotNew<T, A> for Option<T>
where
    T: AddressableSlot,
    A: Allocator,
{
    fn new_in(_alloc: A) -> Self {
        None
    }
}
