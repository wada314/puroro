//! Shared per-message state (`MessageCommon`) passed into field operations.
//!
//! Except for [`OneofSlot`](super::oneof::OneofSlot), each protobuf field's
//! getter/setter/encode/decode touches only its own field struct plus
//! `&MessageCommon` / `&mut MessageCommon`.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AVec;

use super::presence::PresenceBits;

/// Infrastructure fields shared by every field in a generated message.
///
/// Field types take `&Self` or `&mut Self` rather than a back-pointer to the
/// parent message struct.
pub struct MessageCommon<P, A: Allocator> {
    pub presence: P,
    pub unknown_fields: AVec<u8, A>,
    pub alloc: A,
}

impl<P, A: Allocator> MessageCommon<P, A> {
    /// Creates common state with the given presence bitfield and allocator.
    pub fn new_in(presence: P, alloc: A) -> Self
    where
        A: Clone,
    {
        Self {
            presence,
            unknown_fields: AVec::new_in(alloc.clone()),
            alloc,
        }
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
