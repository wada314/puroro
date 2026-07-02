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
        // `UnmanagedVec::new` does not allocate; it only decomposes an empty
        // `Vec`, so the borrow here never establishes buffer ownership (this is
        // the documented no-op use of `UnmanagedVec::new(&alloc)`). Once the
        // buffer actually grows it is owned by an owned-`A` clone, and it is
        // freed with an owned-`A` clone in `deallocate`.
        let unknown_fields = ManuallyDrop::new(UnmanagedVec::new(&alloc));
        Self {
            presence,
            unknown_fields,
            alloc,
        }
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
