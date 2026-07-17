//! Shared infrastructure for generated message fields.
//!
//! [`MessageCommon`] and [`PresenceBits`] are per-message state. [`DefaultIn`],
//! [`DeallocateIn`], [`ProtoEmpty`], [`ValueSlot`](value_slot::ValueSlot),
//! [`SlotInitView`](slot_init::SlotInitView) / [`SlotInitMut`](slot_init::SlotInitMut),
//! and [`FieldPresence`](field_presence::FieldPresence) govern singular scalar
//! storage and init state.

pub(crate) mod field_deallocate;
pub(crate) mod field_presence;
pub(crate) mod slot_init;
pub(crate) mod value_layout;
pub(crate) mod value_slot;

pub use field_deallocate::FieldDeallocate;
pub use value_layout::{BitPacked, Inline, ValueLayout};

use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::unmanaged::UnmanagedVec;

use crate::decode::{UnknownFieldsIter, iter_unknown_fields};

// ---------------------------------------------------------------------------
// Message bitfield (`PresenceBits`)
// ---------------------------------------------------------------------------

/// Read/write interface to a message's packed bitfield.
///
/// Codegen assigns stable `bit` indices for:
/// - EXPLICIT / LEGACY_REQUIRED singular **presence**
/// - singular / oneof **bool value** bits packed into the same array
///
/// The historical name `PresenceBits` remains; the bits themselves are not
/// presence-only. Generated newtypes typically wrap `bitvec::BitArray` and return
/// bitvec's `BitRef<'_, Mut, …>` from [`bit_mut`](Self::bit_mut).
pub trait PresenceBits {
    /// Returns whether bit `bit` is set.
    fn is_set(&self, bit: usize) -> bool;

    /// Sets or clears bit `bit`.
    fn set(&mut self, bit: usize, value: bool);

    /// Clears bit `bit`.
    #[inline]
    fn clear(&mut self, bit: usize) {
        self.set(bit, false);
    }

    /// Returns a mutable handle to bit `bit`.
    ///
    /// Generated presence newtypes wrap `bitvec::BitArray<[u8; N], Lsb0>`, so
    /// the handle is a concrete [`BitRef`](::bitvec::ptr::BitRef).
    ///
    /// # Panics
    ///
    /// Generated impls panic if `bit` is out of range for the message bitfield.
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0>;
}

// ---------------------------------------------------------------------------
// Per-message common state (`MessageCommon`)
// ---------------------------------------------------------------------------

/// Infrastructure fields shared by every field in a generated message.
///
/// Field types take `&Self` or `&mut Self` rather than a back-pointer to the
/// parent message struct.
///
/// The allocator `alloc` is the single canonical copy for the whole message:
/// unmanaged field payloads retain only its type and receive cloned instances
/// for operations that (de)allocate. `unknown_fields` is wrapped in
/// [`ManuallyDrop`], so it never frees itself implicitly; the owning message
/// releases it via [`deallocate`](Self::deallocate) in its `Drop`.
pub struct MessageCommon<P, A: Allocator> {
    pub presence: P,
    pub unknown_fields: ManuallyDrop<UnmanagedVec<u8, A>>,
    pub alloc: A,
}

impl<P, A: Allocator + Clone> MessageCommon<P, A> {
    /// Creates common state with the given presence bitfield and allocator.
    pub fn new_in(presence: P, alloc: A) -> Self {
        let unknown_fields = ManuallyDrop::new(UnmanagedVec::new(alloc.clone()));
        Self {
            presence,
            unknown_fields,
            alloc,
        }
    }

    /// Iterates preserved unknown fields as structured views.
    ///
    /// Storage remains a contiguous wire blob; this only parses it for the
    /// public accessor shape.
    #[inline]
    pub fn iter_unknown_fields(&self) -> UnknownFieldsIter<'_> {
        iter_unknown_fields(&self.unknown_fields)
    }
}

impl<P, A: Allocator + Clone> MessageCommon<P, A> {
    /// Releases the unknown-field buffer. Must be called exactly once from the
    /// owning message's `Drop`; afterwards `self` must not be used.
    pub fn deallocate(&mut self) {
        // SAFETY: called once from the message `Drop`; `unknown_fields` is not
        // touched again, and an owned clone of `self.alloc` is interchangeable
        // with the clones that grew the buffer (`Allocator + Clone` contract).
        let uf = unsafe { ManuallyDrop::take(&mut self.unknown_fields) };
        unsafe { uf.deallocate(self.alloc.clone()) };
    }
}

impl<P: PresenceBits, A: Allocator> MessageCommon<P, A> {
    /// Returns whether bit `bit` is set.
    #[inline]
    pub fn is_bit_set(&self, bit: usize) -> bool {
        self.presence.is_set(bit)
    }

    /// Sets or clears bit `bit`.
    #[inline]
    pub fn set_bit(&mut self, bit: usize, value: bool) {
        self.presence.set(bit, value);
    }

    /// Returns a mutable handle to bit `bit` (`DerefMut<Target = bool>`).
    #[inline]
    pub fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        self.presence.bit_mut(bit)
    }
}

// ---------------------------------------------------------------------------
// Allocator-aware construction / release (`DefaultIn` / `DeallocateIn`)
// ---------------------------------------------------------------------------

/// Generalizes [`Default`] for payloads whose empty form may need an allocator
/// (`UnmanagedString`, `UnmanagedVec`). Allocator-less scalars ignore `alloc`.
///
/// The allocator is a **trait parameter** chosen by the caller (not an
/// associated type), matching the wg-allocators direction: scalars like `i32`
/// can implement `DefaultIn<A>` for every `A`, while `UnmanagedString<A>` only
/// implements `DefaultIn<A>` for its own `A`.
pub trait DefaultIn<A: Allocator + Clone> {
    /// Builds the empty / type-zero value, using `alloc` when heap-backed.
    fn default_in(alloc: A) -> Self;
}

/// Allocator-aware release of a stored value.
///
/// Pairs with [`DefaultIn`] so [`ValueSlot`](value_slot::ValueSlot) can replace
/// or clear a payload without leaking. Allocator-less scalars are a no-op.
pub trait DeallocateIn<A: Allocator + Clone> {
    /// Drops the value and frees its backing allocation through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns this value's buffer.
    unsafe fn deallocate_in(self, alloc: A);
}

/// Empty / type-zero predicate for IMPLICIT omit-on-encode.
///
/// Copy scalars compare to `0` / `false`; LEN payloads use `is_empty`.
pub trait ProtoEmpty {
    /// `true` when the value equals the protobuf empty / type-zero.
    fn is_proto_empty(&self) -> bool;
}

macro_rules! impl_scalar_default_in {
    ($ty:ty, $zero:expr) => {
        impl<A: Allocator + Clone> DefaultIn<A> for $ty {
            #[inline]
            fn default_in(_alloc: A) -> Self {
                $zero
            }
        }

        impl<A: Allocator + Clone> DeallocateIn<A> for $ty {
            #[inline]
            unsafe fn deallocate_in(self, _alloc: A) {}
        }
    };
}

impl_scalar_default_in!(u32, 0);
impl_scalar_default_in!(u64, 0);
impl_scalar_default_in!(i32, 0);
impl_scalar_default_in!(i64, 0);
impl_scalar_default_in!((), ());

impl ProtoEmpty for u32 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0
    }
}
impl ProtoEmpty for u64 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0
    }
}
impl ProtoEmpty for i32 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0
    }
}
impl ProtoEmpty for i64 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0
    }
}
impl ProtoEmpty for () {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        true
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for ::unmanaged::UnmanagedString<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        ::unmanaged::UnmanagedString::new(alloc)
    }
}

impl<A: Allocator + Clone> DeallocateIn<A> for ::unmanaged::UnmanagedString<A> {
    #[inline]
    unsafe fn deallocate_in(self, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { self.deallocate(alloc) };
    }
}

impl<A: Allocator> ProtoEmpty for ::unmanaged::UnmanagedString<A> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for ::unmanaged::UnmanagedVec<u8, A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        ::unmanaged::UnmanagedVec::new(alloc)
    }
}

impl<A: Allocator + Clone> DeallocateIn<A> for ::unmanaged::UnmanagedVec<u8, A> {
    #[inline]
    unsafe fn deallocate_in(self, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { self.deallocate(alloc) };
    }
}

impl<A: Allocator> ProtoEmpty for ::unmanaged::UnmanagedVec<u8, A> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.is_empty()
    }
}
