//! Shared infrastructure for generated message fields.
//!
//! [`MessageCommon`] holds per-message state. Catalog bounds use
//! [`MessageCommonBits`] / [`MessageCommonAlloc`] on that context.
//! [`DefaultIn`](::unmanaged::DefaultIn),
//! [`ValueSlot`](value_slot::ValueSlot),
//! [`SlotInitView`](slot_init::SlotInitView) / [`SlotInitMut`](slot_init::SlotInitMut),
//! and [`FieldPresence`](field_presence::FieldPresence) govern singular scalar
//! storage and init state. Slot teardown goes through
//! [`ValueLayout`](value_layout::ValueLayout) (`take_value` + payload release).
//! IMPLICIT empty / type-zero checks live on
//! [`PayloadAccess`](crate::fields::wire::singular_type::PayloadAccess) /
//! [`ValueLayout`](value_layout::ValueLayout).

pub(crate) mod field_deallocate;
pub(crate) mod field_inspect;
pub(crate) mod field_presence;
pub(crate) mod slot_bound;
pub(crate) mod slot_init;
pub(crate) mod value_layout;
pub(crate) mod value_slot;

pub use crate::unknown_fields::{DiscardUnknowns, UnknownFields, UnknownStore};
pub use ::unmanaged::DefaultIn;
pub use field_deallocate::FieldDeallocate;
pub use field_inspect::{
    CloneFieldsVisitor, DebugStructVisitor, EncodeRawVisitor, EncodedLenVisitor, FieldCloneIn,
    FieldDeallocVisitor, FieldEncode, FieldEqVisitor, FieldPairVisitor, FieldPairVisitorMut,
    FieldVisitor, FieldVisitorMut,
};
pub use slot_bound::{CloneBound, DeallocateBound};
pub use value_layout::{BitPacked, Boxed, Inline, InlineOrHeap, SSO_HEAP, SSO_INLINE, ValueLayout};
pub use value_slot::AddressableSlot;
// MessageBinding / MessageBindingMut are defined in this module.

use ::core::cell::UnsafeCell;
use ::core::mem::ManuallyDrop;

use crate::decode::{UnknownFieldsIter, iter_unknown_fields};
use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    array::BitArray,
    order::Lsb0,
    ptr::{BitRef, Mut},
    slice::BitSlice,
};

// ---------------------------------------------------------------------------
// Private bit-storage helper (not a catalog bound)
// ---------------------------------------------------------------------------

/// Read/write interface to the packed bit array stored in [`MessageCommon`].
///
/// Catalog code bounds [`MessageCommonBits`] on the common context instead.
pub(crate) trait BitStorage {
    fn is_set(&self, bit: usize) -> bool;
    fn set(&mut self, bit: usize, value: bool);
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0>;
}

impl<const N: usize> BitStorage for BitArray<[u8; N], Lsb0> {
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
        self.get_mut(bit).expect("common bit index in range")
    }
}

/// [`BitArray`] behind [`UnsafeCell`] so lazy promotion can update arm bits
/// through `&self`.
///
/// Catalog `bit_mut` still requires `&mut self`. Shared writes go through
/// [`set_shared`](Self::set_shared).
pub struct InteriorBitArray<const N: usize> {
    bits: UnsafeCell<BitArray<[u8; N], Lsb0>>,
}

impl<const N: usize> InteriorBitArray<N> {
    pub fn zero() -> Self {
        Self {
            bits: UnsafeCell::new(BitArray::ZERO),
        }
    }

    #[inline]
    pub fn is_set(&self, bit: usize) -> bool {
        // SAFETY: only bit cells are written; readers see a single-byte store.
        unsafe { (&(*self.bits.get()))[bit] }
    }

    #[inline]
    pub fn set_shared(&self, bit: usize, value: bool) {
        // SAFETY: callers serialize with `&self` promotion or exclusive `&mut`.
        unsafe {
            BitSlice::set(&mut *self.bits.get(), bit, value);
        }
    }
}

impl<const N: usize> Clone for InteriorBitArray<N> {
    fn clone(&self) -> Self {
        Self {
            bits: UnsafeCell::new(unsafe { *self.bits.get() }),
        }
    }
}

impl<const N: usize> Default for InteriorBitArray<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> BitStorage for InteriorBitArray<N> {
    #[inline]
    fn is_set(&self, bit: usize) -> bool {
        InteriorBitArray::is_set(self, bit)
    }

    #[inline]
    fn set(&mut self, bit: usize, value: bool) {
        self.set_shared(bit, value);
    }

    #[inline]
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        // SAFETY: exclusive `&mut self`.
        unsafe {
            (*self.bits.get())
                .get_mut(bit)
                .expect("common bit index in range")
        }
    }
}

impl<B: BitStorage + ?Sized> MessageCommonBits for B {
    #[inline]
    fn is_bit_set(&self, bit: usize) -> bool {
        self.is_set(bit)
    }

    #[inline]
    fn set_bit(&mut self, bit: usize, value: bool) {
        BitStorage::set(self, bit, value);
    }

    #[inline]
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        BitStorage::bit_mut(self, bit)
    }
}

// ---------------------------------------------------------------------------
// MessageCommon capabilities (catalog bounds)
// ---------------------------------------------------------------------------

/// Access to a message's common bits (presence, packed bool values, SSO heap).
///
/// Codegen assigns stable `bit` indices in one array for EXPLICIT /
/// LEGACY_REQUIRED presence, singular / oneof bool values, and string / bytes
/// SSO heap-arm bits.
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
    type Alloc: Allocator;

    /// Borrow the canonical message allocator.
    fn alloc(&self) -> &Self::Alloc;

    /// Clone the canonical message allocator.
    #[inline]
    fn clone_alloc(&self) -> Self::Alloc
    where
        Self::Alloc: Clone,
    {
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
/// unmanaged field payloads retain only its type. Growth / clone paths receive
/// cloned instances; teardown borrows `&self.alloc` (`Allocator::deallocate`
/// is `&self`). `unknown_fields` is wrapped in
/// [`ManuallyDrop`], so it never frees itself implicitly; the owning message
/// releases it via [`deallocate`](Self::deallocate) in its `Drop`.
///
/// `U` selects preserve ([`UnknownFields`], default) or discard
/// ([`DiscardUnknowns`]). Catalog bounds use [`MessageCommonBits`] /
/// [`MessageCommonAlloc`] on `&Self` rather than constraining `B` directly.
pub struct MessageCommon<B, A: Allocator, U: UnknownStore<A> = UnknownFields<A>> {
    /// Common bits (`BitArray` sized by codegen): presence, packed bool values,
    /// and string / bytes SSO heap-arm bits.
    pub bits: B,
    /// Unknown-field store (`ManuallyDrop` — freed by the message). Preserve
    /// empty is one word; discard is a ZST.
    pub unknown_fields: ManuallyDrop<U>,
    /// Canonical allocator for the whole message (cloned for growth / clone;
    /// teardown borrows `&self.alloc`).
    pub alloc: A,
}

impl<B, A: Allocator + Clone, U: UnknownStore<A>> MessageCommon<B, A, U> {
    /// Creates common state with the given common bits and allocator.
    pub fn new_in(bits: B, alloc: A) -> Self {
        let unknown_fields = ManuallyDrop::new(U::new());
        Self {
            bits,
            unknown_fields,
            alloc,
        }
    }

    /// Deep-copies common bits and unknown-field bytes into `alloc`.
    #[inline]
    pub fn clone_in(&self, alloc: A) -> Self
    where
        B: Clone,
    {
        Self {
            bits: self.bits.clone(),
            unknown_fields: ManuallyDrop::new(UnknownStore::clone_in(
                &*self.unknown_fields,
                alloc.clone(),
            )),
            alloc,
        }
    }
}

impl<B, A: Allocator, U: UnknownStore<A>> MessageCommon<B, A, U> {
    /// Iterates preserved unknown fields as structured views.
    ///
    /// Storage remains a contiguous wire blob when preserving; this only
    /// parses it for the public accessor shape. Discard yields an empty
    /// iterator.
    #[inline]
    pub fn iter_unknown_fields(&self) -> UnknownFieldsIter<'_> {
        iter_unknown_fields(self.unknown_fields.as_bytes())
    }

    /// Byte-equality of the unknown-field store (for message `PartialEq`).
    #[inline]
    pub fn unknown_fields_eq(&self, other: &Self) -> bool {
        self.unknown_fields.as_bytes() == other.unknown_fields.as_bytes()
    }

    /// Releases the unknown-field buffer. Must be called exactly once from the
    /// owning message's `Drop`; afterwards `self` must not be used.
    pub fn deallocate(&mut self) {
        // SAFETY: called once from the message `Drop`; `unknown_fields` is not
        // touched again. Reconstructs `Vec<u8, &A>` so teardown does not clone
        // `self.alloc`; clones used at growth must be interchangeable with this
        // borrow (`Allocator::deallocate` is `&self`).
        let uf = unsafe { ManuallyDrop::take(&mut self.unknown_fields) };
        unsafe { uf.deallocate(&self.alloc) };
    }
}

impl<B: BitStorage, A: Allocator, U: UnknownStore<A>> MessageCommonBits for MessageCommon<B, A, U> {
    #[inline]
    fn is_bit_set(&self, bit: usize) -> bool {
        self.bits.is_set(bit)
    }

    #[inline]
    fn set_bit(&mut self, bit: usize, value: bool) {
        self.bits.set(bit, value);
    }

    #[inline]
    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        self.bits.bit_mut(bit)
    }
}

impl<B, A: Allocator, U: UnknownStore<A>> MessageCommonAlloc for MessageCommon<B, A, U> {
    type Alloc = A;

    #[inline]
    fn alloc(&self) -> &A {
        &self.alloc
    }
}

/// Read-side catalog binding: bits, allocator, and unknown-field store.
///
/// [`MessageCommon`] implements this so field accessors can bind without
/// naming the bit-array type parameter.
pub trait MessageBinding<A: Allocator>: MessageCommonAlloc<Alloc = A> + MessageCommonBits {
    /// Unknown-field store for this binding (preserve or discard).
    type Unknown: UnknownStore<A>;

    /// Unknown-field store for this binding.
    fn unknown_fields(&self) -> &Self::Unknown;
}

/// Write-side catalog binding.
pub trait MessageBindingMut<A: Allocator>: MessageBinding<A> {
    /// Mutable unknown-field store for this binding.
    fn unknown_fields_mut(&mut self) -> &mut Self::Unknown;
}

impl<B, A: Allocator, U: UnknownStore<A>> MessageBinding<A> for MessageCommon<B, A, U>
where
    Self: MessageCommonBits,
{
    type Unknown = U;

    #[inline]
    fn unknown_fields(&self) -> &U {
        &self.unknown_fields
    }
}

impl<B, A: Allocator, U: UnknownStore<A>> MessageBindingMut<A> for MessageCommon<B, A, U>
where
    Self: MessageCommonBits,
{
    #[inline]
    fn unknown_fields_mut(&mut self) -> &mut U {
        &mut self.unknown_fields
    }
}

/// Inherent bit helpers — same as [`MessageCommonBits`], for call sites that
/// already have a concrete [`MessageCommon`].
impl<B, A: Allocator, U: UnknownStore<A>> MessageCommon<B, A, U> {
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
