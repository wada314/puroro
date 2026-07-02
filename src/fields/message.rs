//! Singular nested message field — LEN wire type with merge semantics.
//!
//! Presence is `Option<UnmanagedBox<M>>` (not the message bitfield). The box is
//! allocator-less; the owning message frees it via
//! [`deallocate`](NestedMessageField::deallocate) in its `Drop`.

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedBox;

use crate::decode::{self, MessageDecode};
use crate::encode::{self, MessageEncode};
use crate::error::DecodeError;
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::len;
use super::presence::PresenceBits;

/// Trait for child message types stored in [`NestedMessageField`].
pub trait NestedMessage<A: Allocator + Clone>: MessageEncode + MessageDecode + Sized {
    /// Creates an empty child message with the given allocator.
    fn new_in(alloc: A) -> Self;
}

/// Singular embedded message field (`Option<UnmanagedBox<Child>>`).
pub struct NestedMessageField<M, A: Allocator> {
    child: Option<UnmanagedBox<M>>,
    _marker: ::core::marker::PhantomData<A>,
}

impl<M, A: Allocator> NestedMessageField<M, A> {
    /// Creates an absent nested message field.
    pub fn new() -> Self {
        Self {
            child: None,
            _marker: ::core::marker::PhantomData,
        }
    }

    /// Returns the child when present.
    #[inline]
    pub fn get(&self) -> Option<&M> {
        self.child.as_deref()
    }

    /// Returns whether the field is present.
    #[inline]
    pub fn is_present(&self) -> bool {
        self.child.is_some()
    }

    /// Wire byte length when the child is present.
    pub fn encoded_len(&self, field: u32) -> usize
    where
        M: MessageEncode,
    {
        match self.child.as_deref() {
            Some(child) => {
                let payload_len = child.encoded_len();
                encode::encoded_len_len_field(field, payload_len)
            }
            None => 0,
        }
    }

    /// Encodes tag + length + child body when present.
    pub fn encode_raw<B: BufMut>(&self, field: u32, buf: &mut B)
    where
        M: MessageEncode,
    {
        if let Some(child) = self.child.as_deref() {
            let payload_len = child.encoded_len();
            encode::encode_tag(field, WireType::Len, buf);
            encode::encode_varint(payload_len as u64, buf);
            child.encode_raw(buf);
        }
    }

    /// Releases the child through `alloc`, if present. Also used by generated
    /// `clear_*` accessors.
    pub fn deallocate(&mut self, alloc: &A) {
        if let Some(b) = self.child.take() {
            // SAFETY: `alloc` owns the box's allocation; dropping the child runs
            // its own `Drop`, which recursively frees its fields.
            unsafe { b.deallocate(alloc) };
        }
    }
}

impl<M, A: Allocator + Clone> NestedMessageField<M, A> {
    /// Returns a mutable child reference, inserting a default instance if absent.
    pub fn get_mut<P: PresenceBits>(&mut self, common: &MessageCommon<P, A>) -> &mut M
    where
        M: NestedMessage<A>,
    {
        if self.child.is_none() {
            let m = M::new_in(common.alloc.clone());
            self.child = Some(UnmanagedBox::new_in(m, &common.alloc));
        }
        self.child.as_deref_mut().unwrap()
    }

    /// Clears the nested message, freeing it through `alloc`.
    #[inline]
    pub fn clear(&mut self, alloc: &A) {
        self.deallocate(alloc);
    }

    /// Merges one LEN occurrence into the child (creates child on first merge).
    pub fn merge<P: PresenceBits, B: Buf>(
        &mut self,
        common: &MessageCommon<P, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        M: NestedMessage<A>,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut sub = buf.take(len);
        if self.child.is_none() {
            let m = M::new_in(common.alloc.clone());
            self.child = Some(UnmanagedBox::new_in(m, &common.alloc));
        }
        let child = self.child.as_deref_mut().unwrap();
        child.merge_from(&mut sub)?;
        Ok(())
    }
}

impl<M, A: Allocator> Default for NestedMessageField<M, A> {
    fn default() -> Self {
        Self::new()
    }
}
