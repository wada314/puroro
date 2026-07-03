//! Singular nested message field — LEN wire type with merge semantics.
//!
//! Storage is chosen by a [`MessagePresence`] marker:
//!
//! - [`Optional`] (the default) — `Option<UnmanagedBox<M>>`, for ordinary message
//!   fields whose presence is inline (absent vs present).
//! - [`Present`] — a bare `UnmanagedBox<M>`, for oneof message variants: the
//!   enclosing `OneofSlot` tracks presence, so the box is *always* there and the
//!   field behaves like a scalar (`value` / `value_mut`, no `Option`, no
//!   per-access `unwrap`).
//!
//! Either way the box is allocator-less; it is freed explicitly (via
//! [`deallocate`](NestedMessageField::deallocate)) — from the owning message's
//! `Drop` for [`Optional`], or through `OneofDeallocate` for [`Present`].

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
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

/// Storage strategy for [`NestedMessageField`] — whether the child box is
/// wrapped in `Option` (optional presence) or always present.
pub trait MessagePresence {
    /// The stored container: `Option<UnmanagedBox<M>>` for [`Optional`], a bare
    /// `UnmanagedBox<M>` for [`Present`].
    type Store<M, A: Allocator>;
}

/// Optional presence: `Option<UnmanagedBox<M>>`. Used by ordinary message fields.
pub struct Optional;
impl MessagePresence for Optional {
    type Store<M, A: Allocator> = Option<UnmanagedBox<M>>;
}

/// Always-present: a bare `UnmanagedBox<M>`. Used by oneof message variants,
/// whose presence is tracked by the enclosing `OneofSlot`.
pub struct Present;
impl MessagePresence for Present {
    type Store<M, A: Allocator> = UnmanagedBox<M>;
}

/// Singular embedded message field.
///
/// `P` selects storage (see [`MessagePresence`]): [`Optional`] (default) wraps
/// the child in `Option` for ordinary fields; [`Present`] holds a bare box for
/// oneof variants.
pub struct NestedMessageField<M, A: Allocator, P: MessagePresence = Optional> {
    store: P::Store<M, A>,
}

// ---------------------------------------------------------------------------
// Optional — ordinary message field (`Option<UnmanagedBox<M>>`)
// ---------------------------------------------------------------------------

impl<M, A: Allocator> NestedMessageField<M, A, Optional> {
    /// Creates an absent nested message field.
    pub fn new() -> Self {
        Self { store: None }
    }

    /// Returns the child when present.
    #[inline]
    pub fn get(&self) -> Option<&M> {
        self.store.as_deref()
    }

    /// Returns whether the field is present.
    #[inline]
    pub fn is_present(&self) -> bool {
        self.store.is_some()
    }

    /// Wire byte length when the child is present.
    pub fn encoded_len(&self, field: u32) -> usize
    where
        M: MessageEncode,
    {
        match self.store.as_deref() {
            Some(child) => encode::encoded_len_len_field(field, child.encoded_len()),
            None => 0,
        }
    }

    /// Encodes tag + length + child body when present.
    pub fn encode_raw<B: BufMut>(&self, field: u32, buf: &mut B)
    where
        M: MessageEncode,
    {
        if let Some(child) = self.store.as_deref() {
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
        if let Some(b) = self.store.take() {
            // SAFETY: an owned clone of the message allocator owns the box's
            // allocation; dropping the child runs its own `Drop`, which
            // recursively frees its fields.
            unsafe { b.deallocate(alloc) };
        }
    }
}

impl<M, A: Allocator + Clone> NestedMessageField<M, A, Optional> {
    /// Binds this field to its message `common` state (for the allocator),
    /// producing a short-lived [`NestedMessageFieldMut`] view.
    ///
    /// The nested-message analogue of the scalar / LEN / repeated `bind`, so
    /// generated code merges and mutates a nested message the same way
    /// (`field.bind(common).merge(…)` / `.get_mut()`). A nested message has no
    /// presence bit (presence is the inline `Option<Box>`), so — like a repeated
    /// field — the view needs only `common` and `bind` takes no `bit`.
    #[inline]
    pub fn bind<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> NestedMessageFieldMut<'f, 'c, M, Pb, A, Optional> {
        NestedMessageFieldMut::new(self, common)
    }
}

impl<M, A: Allocator> Default for NestedMessageField<M, A, Optional> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Present — oneof message variant (bare `UnmanagedBox<M>`, always present)
// ---------------------------------------------------------------------------

impl<M, A: Allocator> NestedMessageField<M, A, Present> {
    /// Borrows the always-present child.
    #[inline]
    pub fn value(&self) -> &M {
        &self.store
    }

    /// Mutably borrows the always-present child.
    ///
    /// Presence-agnostic and allocator-free (the box is already there), so a
    /// oneof `_mut` accessor reads `field.value_mut()` exactly like a scalar's.
    #[inline]
    pub fn value_mut(&mut self) -> &mut M {
        &mut self.store
    }

    /// Wire byte length (the child is always present).
    pub fn encoded_len(&self, field: u32) -> usize
    where
        M: MessageEncode,
    {
        encode::encoded_len_len_field(field, self.store.encoded_len())
    }

    /// Encodes tag + length + child body (always present).
    pub fn encode_raw<B: BufMut>(&self, field: u32, buf: &mut B)
    where
        M: MessageEncode,
    {
        let payload_len = self.store.encoded_len();
        encode::encode_tag(field, WireType::Len, buf);
        encode::encode_varint(payload_len as u64, buf);
        self.store.encode_raw(buf);
    }

    /// Releases the child through the owned `alloc`. Consumes the field by value
    /// (there is no `Option` to null out); the enclosing oneof calls this via
    /// `OneofDeallocate`, which already owns the variant by value.
    pub fn deallocate(self, alloc: A) {
        // SAFETY: an owned clone of the message allocator owns the box's
        // allocation; dropping the child runs its own `Drop` recursively.
        unsafe { self.store.deallocate(alloc) };
    }
}

impl<M, A: Allocator + Clone> NestedMessageField<M, A, Present> {
    /// Builds an always-present field holding a fresh, empty child.
    ///
    /// Used by oneof variants (via a per-variant `bind_*_mut` accessor) so the
    /// variant is always constructed with the child present.
    pub fn with_message_in(alloc: A) -> Self
    where
        M: NestedMessage<A>,
    {
        let m = M::new_in(alloc.clone());
        Self {
            store: UnmanagedBox::new_in(m, alloc),
        }
    }

    /// Binds a **oneof variant** nested-message field for mutation.
    ///
    /// Provided so generated oneof code reads the same for every variant kind:
    /// `field.bind_oneof(common).merge(…)` — matching
    /// [`SingularLenField::bind_oneof`](super::len_field::SingularLenField::bind_oneof)
    /// and [`SingularVarintField::bind_oneof`](super::scalar::SingularVarintField::bind_oneof).
    /// The child is always present, so this only backs `merge`; the `_mut`
    /// accessor uses [`value_mut`](Self::value_mut) directly.
    #[inline]
    pub fn bind_oneof<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> NestedMessageFieldMut<'f, 'c, M, Pb, A, Present> {
        NestedMessageFieldMut::new(self, common)
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a nested message field to its message common state,
/// produced by [`NestedMessageField::bind`] / [`bind_oneof`].
///
/// Bundles the field with the allocator context so a generated accessor can
/// express a whole mutation as a single call, mirroring the bound-view idiom of
/// the other field families. A nested message has no presence bit, so the view
/// carries only `common` (for the allocator). Every method consumes the view.
///
/// [`bind_oneof`]: NestedMessageField::bind_oneof
pub struct NestedMessageFieldMut<'f, 'c, M, Pb: PresenceBits, A: Allocator, P: MessagePresence = Optional>
{
    field: &'f mut NestedMessageField<M, A, P>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, M, Pb: PresenceBits, A: Allocator, P: MessagePresence>
    NestedMessageFieldMut<'f, 'c, M, Pb, A, P>
{
    #[inline]
    fn new(field: &'f mut NestedMessageField<M, A, P>, common: &'c mut MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }
}

impl<'f, 'c, M, Pb: PresenceBits, A: Allocator + Clone>
    NestedMessageFieldMut<'f, 'c, M, Pb, A, Optional>
{
    /// Returns a mutable child reference, inserting a default instance if absent.
    /// Borrows only the field (`'f`), so `common` is free once this returns.
    pub fn get_mut(self) -> &'f mut M
    where
        M: NestedMessage<A>,
    {
        if self.field.store.is_none() {
            let m = M::new_in(self.common.alloc.clone());
            self.field.store = Some(UnmanagedBox::new_in(m, self.common.alloc.clone()));
        }
        self.field.store.as_deref_mut().unwrap()
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
        if self.field.store.is_none() {
            let m = M::new_in(self.common.alloc.clone());
            self.field.store = Some(UnmanagedBox::new_in(m, self.common.alloc.clone()));
        }
        self.field.store.as_deref_mut().unwrap().merge_from(&mut sub)?;
        Ok(())
    }

    /// Clears the nested message, freeing it through the message allocator.
    pub fn clear(self) {
        self.field.deallocate(self.common.alloc.clone());
    }
}

impl<'f, 'c, M, Pb: PresenceBits, A: Allocator + Clone>
    NestedMessageFieldMut<'f, 'c, M, Pb, A, Present>
{
    /// Merges one LEN occurrence into the always-present child.
    ///
    /// Takes `common` only so the call site stays uniform with the LEN / varint
    /// oneof variants (`field.bind_oneof(common).merge(…)`); the box is already
    /// present, so no allocator is needed here.
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        M: MessageDecode,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut sub = buf.take(len);
        self.field.store.merge_from(&mut sub)?;
        Ok(())
    }
}
