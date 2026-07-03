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

    /// Presence-agnostic mutable access to the child, if present.
    ///
    /// Unlike [`NestedMessageFieldMut::get_mut`] this never inserts and needs no
    /// `MessageCommon`; used by oneof variants, which are always constructed with
    /// the child present (their presence is tracked by the enclosing `OneofSlot`).
    #[inline]
    pub fn get_present_mut(&mut self) -> Option<&mut M> {
        self.child.as_deref_mut()
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

    /// Releases the child through the owned `alloc`, if present. Terminal; called
    /// from the owning message's `Drop`. Generated `clear_*` accessors go through
    /// the bound view ([`NestedMessageFieldMut::clear`]) instead.
    pub fn deallocate(&mut self, alloc: A) {
        if let Some(b) = self.child.take() {
            // SAFETY: an owned clone of the message allocator owns the box's
            // allocation; dropping the child runs its own `Drop`, which
            // recursively frees its fields.
            unsafe { b.deallocate(alloc) };
        }
    }
}

impl<M, A: Allocator + Clone> NestedMessageField<M, A> {
    /// Builds a field holding a fresh, empty child (presence-agnostic).
    ///
    /// Used by oneof variants (via a per-variant `_mut` accessor) so the variant
    /// is always constructed with the child present.
    pub fn with_message_in(alloc: A) -> Self
    where
        M: NestedMessage<A>,
    {
        let m = M::new_in(alloc.clone());
        Self {
            child: Some(UnmanagedBox::new_in(m, alloc)),
            _marker: ::core::marker::PhantomData,
        }
    }

    /// Binds this field to its message `common` state (for the allocator),
    /// producing a short-lived [`NestedMessageFieldMut`] view.
    ///
    /// The nested-message analogue of the scalar / LEN / repeated `bind`, so
    /// generated code merges and mutates a nested message the same way
    /// (`field.bind(common).merge(…)` / `.get_mut()`) instead of threading
    /// `common` into each method by hand. A nested message has no presence bit
    /// (presence is the inline `Option<Box>`), so — like a repeated field — the
    /// view needs only `common` and `bind` takes no `bit`.
    #[inline]
    pub fn bind<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> NestedMessageFieldMut<'f, 'c, M, Pb, A> {
        NestedMessageFieldMut::new(self, common)
    }

    /// Binds a **oneof variant** nested-message field for mutation.
    ///
    /// Identical to [`bind`](Self::bind) (a nested message has no presence bit to
    /// drop), and provided only so generated oneof code reads the same for every
    /// variant kind: `field.bind_oneof(common).merge(…)` — matching
    /// [`SingularLenField::bind_oneof`](super::len_field::SingularLenField::bind_oneof)
    /// and [`SingularVarintField::bind_oneof`](super::scalar::SingularVarintField::bind_oneof).
    #[inline]
    pub fn bind_oneof<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> NestedMessageFieldMut<'f, 'c, M, Pb, A> {
        self.bind(common)
    }
}

impl<M, A: Allocator> Default for NestedMessageField<M, A> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a nested message field to its message common state,
/// produced by [`NestedMessageField::bind`].
///
/// Bundles the field with the allocator context so a generated accessor can
/// express a whole mutation as a single call, mirroring the bound-view idiom of
/// the other field families. A nested message has no presence bit, so the view
/// carries only `common` (for the allocator). Every method consumes the view, so
/// a fresh `bind` precedes each mutation.
pub struct NestedMessageFieldMut<'f, 'c, M, Pb: PresenceBits, A: Allocator> {
    field: &'f mut NestedMessageField<M, A>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, M, Pb: PresenceBits, A: Allocator + Clone> NestedMessageFieldMut<'f, 'c, M, Pb, A> {
    #[inline]
    fn new(field: &'f mut NestedMessageField<M, A>, common: &'c mut MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }

    /// Returns a mutable child reference, inserting a default instance if absent.
    /// Borrows only the field (`'f`), so `common` is free once this returns.
    pub fn get_mut(self) -> &'f mut M
    where
        M: NestedMessage<A>,
    {
        if self.field.child.is_none() {
            let m = M::new_in(self.common.alloc.clone());
            self.field.child = Some(UnmanagedBox::new_in(m, self.common.alloc.clone()));
        }
        self.field.child.as_deref_mut().unwrap()
    }

    /// Merges one LEN occurrence into the child (creates the child on first
    /// merge, then merges subsequent occurrences into it).
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
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
        if self.field.child.is_none() {
            let m = M::new_in(self.common.alloc.clone());
            self.field.child = Some(UnmanagedBox::new_in(m, self.common.alloc.clone()));
        }
        let child = self.field.child.as_deref_mut().unwrap();
        child.merge_from(&mut sub)?;
        Ok(())
    }

    /// Clears the nested message, freeing it through the message allocator.
    pub fn clear(self) {
        self.field.deallocate(self.common.alloc.clone());
    }
}
