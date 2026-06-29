//! Singular nested message field — LEN wire type with merge semantics.
//!
//! Presence is `Option<Box<M, A>>` (not the message bitfield).

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::boxed::Box as ABox;

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

/// Singular embedded message field (`Option<Box<Child, A>>`).
pub struct NestedMessageField<M, A: Allocator> {
    child: Option<ABox<M, A>>,
}

impl<M, A: Allocator + Clone> NestedMessageField<M, A> {
    /// Creates an absent nested message field.
    pub fn new() -> Self {
        Self { child: None }
    }

    /// Returns the child when present.
    #[inline]
    pub fn get(&self) -> Option<&M> {
        self.child.as_deref()
    }

    /// Returns a mutable child reference, inserting a default instance if absent.
    pub fn get_mut<P: PresenceBits>(
        &mut self,
        common: &MessageCommon<P, A>,
    ) -> &mut M
    where
        M: NestedMessage<A>,
    {
        self.child.get_or_insert_with(|| {
            ABox::new_in(M::new_in(common.alloc.clone()), common.alloc.clone())
        })
    }

    /// Replaces the whole child (`None` clears).
    pub fn set<P: PresenceBits>(&mut self, common: &MessageCommon<P, A>, value: Option<M>) {
        self.child = value.map(|m| ABox::new_in(m, common.alloc.clone()));
    }

    /// Sets the child from an owned value.
    pub fn set_child<P: PresenceBits>(&mut self, common: &MessageCommon<P, A>, value: M) {
        self.child = Some(ABox::new_in(value, common.alloc.clone()));
    }

    /// Clears the nested message.
    #[inline]
    pub fn clear(&mut self) {
        self.child = None;
    }

    /// Returns whether the field is present.
    #[inline]
    pub fn is_present(&self) -> bool {
        self.child.is_some()
    }

    /// Wire byte length when the child is present.
    pub fn encoded_len<const FIELD: u32>(&self) -> usize
    where
        M: MessageEncode,
    {
        match self.child.as_deref() {
            Some(child) => {
                let payload_len = child.encoded_len();
                encode::encoded_len_len_field(FIELD, payload_len)
            }
            None => 0,
        }
    }

    /// Encodes tag + length + child body when present.
    pub fn encode_raw<B: BufMut, const FIELD: u32>(&self, buf: &mut B)
    where
        M: MessageEncode,
    {
        if let Some(child) = self.child.as_deref() {
            let payload_len = child.encoded_len();
            encode::encode_tag(FIELD, WireType::Len, buf);
            encode::encode_varint(payload_len as u64, buf);
            child.encode_raw(buf);
        }
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
        let child = self.child.get_or_insert_with(|| {
            ABox::new_in(M::new_in(common.alloc.clone()), common.alloc.clone())
        });
        child.merge_from(&mut sub)?;
        Ok(())
    }
}

impl<M, A: Allocator + Clone> Default for NestedMessageField<M, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M, A: Allocator + Clone> Clone for NestedMessageField<M, A>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            child: self.child.clone(),
        }
    }
}

impl<M: PartialEq, A: Allocator + Clone> PartialEq for NestedMessageField<M, A> {
    fn eq(&self, other: &Self) -> bool {
        self.child == other.child
    }
}

impl<M: Eq, A: Allocator + Clone> Eq for NestedMessageField<M, A> {}

impl<M: ::core::fmt::Debug, A: Allocator + Clone> ::core::fmt::Debug for NestedMessageField<M, A> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NestedMessageField")
            .field("child", &self.child)
            .finish()
    }
}
