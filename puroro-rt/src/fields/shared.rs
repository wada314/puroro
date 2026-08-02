//! Shared infrastructure for generated message fields.
//!
//! [`MessageCommon`] holds per-message state. Catalog bounds use
//! [`MessageCommonBits`] / [`MessageCommonAlloc`] on that context.
//! [`DefaultIn`], [`ProtoEmpty`], [`ValueSlot`](value_slot::ValueSlot),
//! [`SlotInitView`](slot_init::SlotInitView) / [`SlotInitMut`](slot_init::SlotInitMut),
//! and [`FieldPresence`](field_presence::FieldPresence) govern singular scalar
//! storage and init state. Slot teardown uses [`unmanaged::DeallocateIn`].

pub(crate) mod field_deallocate;
pub(crate) mod field_inspect;
pub(crate) mod field_presence;
pub(crate) mod slot_init;
pub(crate) mod value_layout;
pub(crate) mod value_slot;

pub use field_deallocate::FieldDeallocate;
pub use field_inspect::{
    CatalogField, CloneFieldsVisitor, DebugStructVisitor, EncodeRawVisitor, EncodedLenVisitor,
    FieldCloneIn, FieldDeallocVisitor, FieldDebug, FieldEncode, FieldEqVisitor, FieldPairVisitor,
    FieldPairVisitorMut, FieldPartialEq, FieldVisitor, FieldVisitorMut,
};
pub use value_layout::{BitPacked, Inline, ValueLayout};

use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    array::BitArray,
    order::Lsb0,
    ptr::{BitRef, Mut},
    slice::BitSlice,
};
use ::unmanaged::UnmanagedVec;

use crate::decode::{UnknownFieldsIter, iter_unknown_fields};

// ---------------------------------------------------------------------------
// Private bit-storage helper (not a catalog bound)
// ---------------------------------------------------------------------------

/// Read/write interface to the packed bit array stored in [`MessageCommon`].
///
/// Catalog code bounds [`MessageCommonBits`] on the common context instead.
pub(crate) trait PresenceBits {
    fn is_set(&self, bit: usize) -> bool;
    fn set(&mut self, bit: usize, value: bool);
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0>;
}

impl<const N: usize> PresenceBits for BitArray<[u8; N], Lsb0> {
    #[inline]
    fn is_set(&self, bit: usize) -> bool {
        self[bit]
    }

    #[inline]
    fn set(&mut self, bit: usize, value: bool) {
        BitSlice::set(self, bit, value);
    }

    #[inline]
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        self.get_mut(bit)
            .expect("bool / presence bit index in range")
    }
}

// ---------------------------------------------------------------------------
// MessageCommon capabilities (catalog bounds)
// ---------------------------------------------------------------------------

/// Bitfield access through a [`MessageCommon`] (presence + packed bool values).
///
/// Codegen assigns stable `bit` indices for EXPLICIT / LEGACY_REQUIRED presence
/// and singular / oneof bool value bits in the same array.
pub trait MessageCommonBits {
    /// Returns whether bit `bit` is set.
    fn is_bit_set(&self, bit: usize) -> bool;

    /// Sets or clears bit `bit`.
    fn set_bit(&mut self, bit: usize, value: bool);

    /// Returns a mutable handle to bit `bit` (`DerefMut<Target = bool>`).
    ///
    /// # Panics
    ///
    /// Panics if `bit` is out of range for the message bitfield.
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0>;
}

/// Allocator access through a [`MessageCommon`].
pub trait MessageCommonAlloc {
    /// Message allocator type.
    type Alloc: Allocator + Clone;

    /// Borrow the canonical message allocator.
    fn alloc(&self) -> &Self::Alloc;

    /// Clone the canonical message allocator.
    #[inline]
    fn clone_alloc(&self) -> Self::Alloc {
        self.alloc().clone()
    }
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
///
/// Catalog bounds use [`MessageCommonBits`] / [`MessageCommonAlloc`] on `&Self`
/// rather than constraining the storage type parameter `P` directly.
pub struct MessageCommon<P, A: Allocator> {
    /// Presence / packed-bool bit storage (`BitArray` sized by codegen).
    pub presence: P,
    /// Contiguous unknown-field wire blob (`ManuallyDrop` — freed by the message).
    pub unknown_fields: ManuallyDrop<UnmanagedVec<u8, A>>,
    /// Canonical allocator for the whole message (cloned into field ops).
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

    /// Deep-copies presence bits and unknown-field bytes into `alloc`.
    #[inline]
    pub fn clone_in(&self, alloc: A) -> Self
    where
        P: Clone,
    {
        Self {
            presence: self.presence.clone(),
            unknown_fields: ManuallyDrop::new(self.unknown_fields.clone_in(alloc.clone())),
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

    /// Byte-equality of the preserved unknown-field blob (for message `PartialEq`).
    #[inline]
    pub fn unknown_fields_eq(&self, other: &Self) -> bool {
        self.unknown_fields.as_ref() == other.unknown_fields.as_ref()
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

impl<P: PresenceBits, A: Allocator> MessageCommonBits for MessageCommon<P, A> {
    #[inline]
    fn is_bit_set(&self, bit: usize) -> bool {
        self.presence.is_set(bit)
    }

    #[inline]
    fn set_bit(&mut self, bit: usize, value: bool) {
        self.presence.set(bit, value);
    }

    #[inline]
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        self.presence.bit_mut(bit)
    }
}

impl<P, A: Allocator + Clone> MessageCommonAlloc for MessageCommon<P, A> {
    type Alloc = A;

    #[inline]
    fn alloc(&self) -> &A {
        &self.alloc
    }
}

/// Inherent bit helpers — same as [`MessageCommonBits`], for call sites that
/// already have a concrete [`MessageCommon`].
impl<P, A: Allocator> MessageCommon<P, A> {
    /// Returns whether bit `bit` is set.
    #[inline]
    pub fn is_bit_set(&self, bit: usize) -> bool
    where
        Self: MessageCommonBits,
    {
        MessageCommonBits::is_bit_set(self, bit)
    }

    /// Sets or clears bit `bit`.
    #[inline]
    pub fn set_bit(&mut self, bit: usize, value: bool)
    where
        Self: MessageCommonBits,
    {
        MessageCommonBits::set_bit(self, bit, value);
    }

    /// Returns a mutable handle to bit `bit` (`DerefMut<Target = bool>`).
    #[inline]
    pub fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0>
    where
        Self: MessageCommonBits,
    {
        MessageCommonBits::bit_mut(self, bit)
    }
}

// ---------------------------------------------------------------------------
// Allocator-aware construction (`DefaultIn`) / empty check (`ProtoEmpty`)
// ---------------------------------------------------------------------------

/// Generalizes [`Default`] for payloads whose empty form may need an allocator
/// (`UnmanagedString`, `UnmanagedVec`). Allocator-less scalars ignore `alloc`.
///
/// The allocator is a **trait parameter** chosen by the caller (not an
/// associated type), matching the wg-allocators direction: scalars like `i32`
/// can implement `DefaultIn<A>` for every `A`, while `UnmanagedString<A>` only
/// implements `DefaultIn<A>` for its own `A`.
///
/// Teardown uses [`unmanaged::DeallocateIn`] directly (not a local trait).
pub trait DefaultIn<A: Allocator + Clone> {
    /// Builds the empty / type-zero value, using `alloc` when heap-backed.
    fn default_in(alloc: A) -> Self;
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
impl ProtoEmpty for bool {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        !*self
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

impl<A: Allocator> ProtoEmpty for ::unmanaged::UnmanagedVec<u8, A> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.is_empty()
    }
}
