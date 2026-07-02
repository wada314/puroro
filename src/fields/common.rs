//! Shared per-message state (`MessageCommon`) passed into field operations.
//!
//! Except for [`OneofSlot`](super::oneof::OneofSlot), each protobuf field's
//! getter/setter/encode/decode touches only its own field struct plus
//! `&MessageCommon` / `&mut MessageCommon`.

use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedVec;

use super::presence::PresenceBits;

/// Infrastructure fields shared by every field in a generated message.
///
/// Field types take `&Self` or `&mut Self` rather than a back-pointer to the
/// parent message struct.
///
/// The allocator `alloc` is the single canonical copy for the whole message:
/// unmanaged field payloads borrow it (`&alloc`) for every operation that
/// (de)allocates. `unknown_fields` is an allocator-less [`UnmanagedVec`] wrapped
/// in [`ManuallyDrop`], so it never frees itself implicitly; the owning message
/// releases it via [`deallocate`](Self::deallocate) in its `Drop`.
pub struct MessageCommon<P, A: Allocator> {
    pub presence: P,
    pub unknown_fields: ManuallyDrop<UnmanagedVec<u8>>,
    pub alloc: A,
}

impl<P, A: Allocator> MessageCommon<P, A> {
    /// Creates common state with the given presence bitfield and allocator.
    pub fn new_in(presence: P, alloc: A) -> Self {
        // `UnmanagedVec::new` does not allocate; it only needs an allocator to
        // decompose an empty `Vec`. Borrow `alloc` so it stays owned for the
        // canonical `alloc` slot below.
        let unknown_fields = ManuallyDrop::new(UnmanagedVec::new(&alloc));
        Self {
            presence,
            unknown_fields,
            alloc,
        }
    }

    /// Releases the unknown-field buffer. Must be called exactly once from the
    /// owning message's `Drop`; afterwards `self` must not be used.
    pub fn deallocate(&mut self) {
        // SAFETY: called once from the message `Drop`; `unknown_fields` is not
        // touched again, and `self.alloc` is the allocator that owns the buffer.
        let uf = unsafe { ManuallyDrop::take(&mut self.unknown_fields) };
        unsafe { uf.deallocate(&self.alloc) };
    }
}

impl<P: PresenceBits, A: Allocator> MessageCommon<P, A> {
    /// Returns whether a presence bit is set.
    #[inline]
    pub fn is_present(&self, bit: usize) -> bool {
        self.presence.is_set(bit)
    }

    /// Sets or clears a presence bit.
    #[inline]
    pub fn set_presence(&mut self, bit: usize, present: bool) {
        self.presence.set(bit, present);
    }
}
