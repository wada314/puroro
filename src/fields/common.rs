//! Shared per-message state (`MessageCommon`) and views passed into field ops.
//!
//! Except for [`OneofSlot`](super::oneof::OneofSlot), each protobuf field's
//! getter/setter/encode/decode/clone/eq touches only its own field struct plus
//! a [`MessageParts`] or [`MessagePartsMut`] view of this common state.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AVec;

use super::presence::PresenceBits;

/// Infrastructure fields shared by every field in a generated message.
///
/// Singular field types receive [`MessageParts`] / [`MessagePartsMut`] rather
/// than holding a back-pointer to the parent message.
pub struct MessageCommon<P, A: Allocator> {
    pub presence: P,
    pub unknown_fields: AVec<u8, A>,
    pub alloc: A,
}

/// Immutable common state for field getters and encode.
pub struct MessageParts<'a, P, A: Allocator> {
    pub presence: &'a P,
    pub alloc: &'a A,
}

/// Mutable common state for field setters, merge, and closed-enum unknown diversion.
pub struct MessagePartsMut<'a, P, A: Allocator> {
    pub presence: &'a mut P,
    pub unknown_fields: &'a mut AVec<u8, A>,
    pub alloc: &'a A,
}

impl<P, A: Allocator> MessageCommon<P, A> {
    /// Creates common parts with the given presence bitfield and allocator.
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

    /// Immutable view for read-only field operations.
    #[inline]
    pub fn parts(&self) -> MessageParts<'_, P, A> {
        MessageParts {
            presence: &self.presence,
            alloc: &self.alloc,
        }
    }

    /// Mutable view for field mutation and merge.
    #[inline]
    pub fn parts_mut(&mut self) -> MessagePartsMut<'_, P, A> {
        MessagePartsMut {
            presence: &mut self.presence,
            unknown_fields: &mut self.unknown_fields,
            alloc: &self.alloc,
        }
    }
}

impl<P: PresenceBits, A: Allocator> MessagePartsMut<'_, P, A> {
    /// Sets or clears a presence bit through the shared bitfield.
    #[inline]
    pub fn set_presence(&mut self, bit: usize, present: bool) {
        self.presence.set(bit, present);
    }

    /// Returns whether a presence bit is set.
    #[inline]
    pub fn is_present(&self, bit: usize) -> bool {
        self.presence.is_set(bit)
    }
}
