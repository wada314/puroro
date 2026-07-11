//! Singular nested message field — LEN wire type with merge semantics.
//!
//! Generic parameter order matches the other singular field wrappers:
//! message type `M`, presence policy `P`, field number `FIELD`, allocator `A`.
//!
//! Storage is chosen by a [`MessagePresence`] marker:
//!
//! - [`Singular`] — `Option<UnmanagedBox<M>>`, for ordinary nested message
//!   fields whose presence is tracked by the field itself (absent vs present).
//! - [`Oneof`](crate::fields::shared::field_presence::Oneof) — a bare `UnmanagedBox<M>`, for oneof
//!   message variants: the enclosing `OneofSlot` tracks presence, so the box is
//!   *always* there and the field behaves like a scalar (`value` / `value_mut`, no
//!   `Option`, no per-access `unwrap`).
//!
//! Either way the box is allocator-less; it is freed explicitly (via
//! [`deallocate`](NestedMessageField::deallocate)) — from the owning message's
//! `Drop` for [`Singular`], or through `OneofDeallocate` for [`Oneof`](crate::fields::shared::field_presence::Oneof).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::unmanaged::UnmanagedBox;

use ::puroro::{DecodeError, MessageDecode, MessageEncode, WireType};

use crate::decode;
use crate::encode;

use crate::fields::shared::{
    Bindable, BindableMut, FieldDeallocate, MessageCommon, PresenceBits, field_presence::Oneof,
};
use crate::fields::wire::len;

/// Trait for child message types stored in [`NestedMessageField`].
pub trait NestedMessage<A: Allocator + Clone>: MessageEncode + MessageDecode + Sized {
    /// Creates an empty child message with the given allocator.
    fn new_in(alloc: A) -> Self;
}

/// Storage strategy for [`NestedMessageField`] — whether the child box is
/// wrapped in `Option` (optional presence) or always present.
pub trait MessagePresence {
    /// The stored container: `Option<UnmanagedBox<M>>` for [`Singular`], a bare
    /// `UnmanagedBox<M>` for [`Oneof`].
    type Store<M, A: Allocator>;

    /// Borrows the child when present (`None` only for [`Singular`] when absent).
    fn as_ref<M, A: Allocator>(store: &Self::Store<M, A>) -> Option<&M>;

    /// Returns a mutable child for merge, inserting an empty one when absent
    /// ([`Singular`] only).
    fn as_mut_for_merge<M, A: Allocator>(store: &mut Self::Store<M, A>, alloc: A) -> &mut M
    where
        M: NestedMessage<A>,
        A: Clone;
}

/// Ordinary nested message field: `Option<UnmanagedBox<M>>`.
pub struct Singular;
impl MessagePresence for Singular {
    type Store<M, A: Allocator> = Option<UnmanagedBox<M>>;

    fn as_ref<M, A: Allocator>(store: &Self::Store<M, A>) -> Option<&M> {
        store.as_deref()
    }

    fn as_mut_for_merge<M, A: Allocator>(store: &mut Self::Store<M, A>, alloc: A) -> &mut M
    where
        M: NestedMessage<A>,
        A: Clone,
    {
        if store.is_none() {
            let m = M::new_in(alloc.clone());
            *store = Some(UnmanagedBox::new_in(m, alloc));
        }
        store.as_deref_mut().unwrap()
    }
}

impl MessagePresence for Oneof {
    type Store<M, A: Allocator> = UnmanagedBox<M>;

    fn as_ref<M, A: Allocator>(store: &Self::Store<M, A>) -> Option<&M> {
        Some(store)
    }

    fn as_mut_for_merge<M, A: Allocator>(store: &mut Self::Store<M, A>, _: A) -> &mut M
    where
        M: NestedMessage<A>,
        A: Clone,
    {
        store
    }
}

/// Singular embedded message field.
///
/// Parametrised like the other singular wrappers: message type `M`, presence
/// policy `P` ([`Singular`] / [`Oneof`]), proto field number `FIELD`, allocator
/// `A`.
pub struct NestedMessageField<M, P: MessagePresence, const FIELD: u32, A: Allocator> {
    store: P::Store<M, A>,
}

impl<M, P: MessagePresence, const FIELD: u32, A: Allocator> NestedMessageField<M, P, FIELD, A> {
    /// Low-level borrow of the child when present.
    ///
    /// Generated message getters go through [`NestedMessageFieldRef::get`] instead.
    #[inline]
    pub fn get(&self) -> Option<&M> {
        P::as_ref(&self.store)
    }

    /// Wire byte length of this field occurrence (ignores `common`; inline / oneof
    /// presence is handled by the field or enclosing slot).
    pub fn encoded_len<Pb>(&self, _common: &MessageCommon<Pb, A>) -> usize
    where
        M: MessageEncode,
    {
        P::as_ref(&self.store)
            .map(|child| encode::encoded_len_len_field(FIELD, child.encoded_len()))
            .unwrap_or(0)
    }

    /// Encodes this field occurrence (ignores `common`).
    pub fn encode_raw<Pb, B: BufMut>(&self, _common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        M: MessageEncode,
    {
        if let Some(child) = P::as_ref(&self.store) {
            let payload_len = child.encoded_len();
            encode::encode_tag(FIELD, WireType::Len, buf);
            encode::encode_varint(payload_len as u64, buf);
            child.encode_raw(buf);
        }
    }
}

impl<M, const FIELD: u32, A: Allocator> NestedMessageField<M, Singular, FIELD, A> {
    /// Creates an absent nested message field, ignoring `alloc` (codegen uses
    /// `new_in` uniformly).
    #[inline]
    pub fn new_in(_alloc: A) -> Self {
        Self { store: None }
    }

    /// Returns whether the field is present.
    #[inline]
    pub fn is_present(&self) -> bool {
        self.store.is_some()
    }

    /// Releases the child through `common.alloc`, if present. Terminal; called
    /// from the owning message's `Drop`. Generated `clear_*` accessors go through
    /// the bound view ([`NestedMessageFieldMut::clear`]) instead.
    pub fn deallocate<Pb: PresenceBits>(&mut self, common: &MessageCommon<Pb, A>)
    where
        A: Clone,
    {
        if let Some(b) = self.store.take() {
            // SAFETY: an owned clone of the message allocator owns the box's
            // allocation; dropping the child runs its own `Drop`, which
            // recursively frees its fields.
            unsafe { b.deallocate(common.alloc.clone()) };
        }
    }
}

impl<M, const FIELD: u32, A: Allocator, Pb: PresenceBits> FieldDeallocate<Pb, A>
    for NestedMessageField<M, Singular, FIELD, A>
where
    A: Clone,
{
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        if let Some(b) = self.store.take() {
            // SAFETY: see inherent `deallocate`.
            unsafe { b.deallocate(common.alloc.clone()) };
        }
    }
}

impl<M, const FIELD: u32, A: Allocator> Default for NestedMessageField<M, Singular, FIELD, A> {
    fn default() -> Self {
        Self { store: None }
    }
}

impl<M, const FIELD: u32, A: Allocator> NestedMessageField<M, Oneof, FIELD, A> {
    /// Low-level borrow of the always-present child (no `common`).
    ///
    /// Used by oneof storage projections; generated getters go through
    /// [`NestedMessageFieldRef::value`].
    #[inline]
    pub fn value(&self) -> &M {
        self.get().unwrap()
    }

    /// Mutably borrows the always-present child.
    #[inline]
    pub fn value_mut(&mut self) -> &mut M {
        &mut self.store
    }

    /// Builds an always-present field holding a fresh, empty child.
    pub fn with_message_in(alloc: A) -> Self
    where
        A: Clone,
        M: NestedMessage<A>,
    {
        let m = M::new_in(alloc.clone());
        Self {
            store: UnmanagedBox::new_in(m, alloc),
        }
    }
}

impl<M, const FIELD: u32, A: Allocator + Clone> NestedMessageField<M, Oneof, FIELD, A> {
    /// Releases the child through `common.alloc`. Consumes the field by value
    /// (there is no `Option` to null out); the enclosing oneof calls this via
    /// `OneofDeallocate`, which already owns the variant by value.
    pub fn deallocate<Pb: PresenceBits>(self, common: &MessageCommon<Pb, A>) {
        // SAFETY: an owned clone of the message allocator owns the box's
        // allocation; dropping the child runs its own `Drop` recursively.
        unsafe { self.store.deallocate(common.alloc.clone()) };
    }
}

impl<'a, M, P: MessagePresence, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits>
    Bindable<&'a MessageCommon<Pb, A>> for &'a NestedMessageField<M, P, FIELD, A>
{
    type Bound = NestedMessageFieldRef<'a, M, P, FIELD, A, Pb>;

    #[inline]
    fn bind(self, common: &'a MessageCommon<Pb, A>) -> Self::Bound {
        NestedMessageFieldRef::new(self, common)
    }
}

impl<'f, 'c, M, P: MessagePresence, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits>
    BindableMut<&'c mut MessageCommon<Pb, A>> for &'f mut NestedMessageField<M, P, FIELD, A>
{
    type BoundMut = NestedMessageFieldMut<'f, 'c, M, P, FIELD, A, Pb>;

    #[inline]
    fn bind_mut(self, common: &'c mut MessageCommon<Pb, A>) -> Self::BoundMut {
        NestedMessageFieldMut::new(self, common)
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a nested message field to its message common
/// state, produced by [`Bindable::bind`](crate::fields::shared::Bindable::bind).
///
/// Mirrors [`NestedMessageFieldMut`] for the read path. Generated getters always
/// go through this view — `field.bind(&common).get()` / `.value()` — even when
/// the accessor does not consult `common`.
pub struct NestedMessageFieldRef<
    'a,
    M,
    P: MessagePresence,
    const FIELD: u32,
    A: Allocator,
    Pb: PresenceBits,
> {
    field: &'a NestedMessageField<M, P, FIELD, A>,
    /// Bound for symmetry with [`NestedMessageFieldMut`]; unused by current getters.
    #[allow(dead_code)]
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, M, P: MessagePresence, const FIELD: u32, A: Allocator, Pb: PresenceBits>
    NestedMessageFieldRef<'a, M, P, FIELD, A, Pb>
{
    #[inline]
    fn new(
        field: &'a NestedMessageField<M, P, FIELD, A>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns the child when present.
    #[inline]
    pub fn get(self) -> Option<&'a M> {
        self.field.get()
    }
}

impl<'a, M, const FIELD: u32, A: Allocator, Pb: PresenceBits>
    NestedMessageFieldRef<'a, M, Oneof, FIELD, A, Pb>
{
    /// Borrows the always-present child.
    #[inline]
    pub fn value(self) -> &'a M {
        self.field.value()
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a nested message field to its message common state,
/// produced by [`BindableMut::bind_mut`](crate::fields::shared::BindableMut::bind_mut).
///
/// Bundles the field with the allocator context so a generated accessor can
/// express a whole mutation as a single call, mirroring the bound-view idiom of
/// the other field families. Every method consumes the view.
pub struct NestedMessageFieldMut<
    'f,
    'c,
    M,
    P: MessagePresence,
    const FIELD: u32,
    A: Allocator,
    Pb: PresenceBits,
> {
    field: &'f mut NestedMessageField<M, P, FIELD, A>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, M, P: MessagePresence, const FIELD: u32, A: Allocator, Pb: PresenceBits>
    NestedMessageFieldMut<'f, 'c, M, P, FIELD, A, Pb>
{
    #[inline]
    fn new(
        field: &'f mut NestedMessageField<M, P, FIELD, A>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Merges one LEN occurrence into the child (creates the child on first
    /// merge for [`Singular`], then merges subsequent occurrences into it).
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        M: NestedMessage<A>,
        A: Clone,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut sub = buf.take(len);
        P::as_mut_for_merge(&mut self.field.store, self.common.alloc.clone())
            .merge_from(&mut sub)?;
        Ok(())
    }
}

impl<'f, 'c, M, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits>
    NestedMessageFieldMut<'f, 'c, M, Singular, FIELD, A, Pb>
{
    /// Returns a mutable child reference, inserting a default instance if absent.
    /// Borrows only the field (`'f`), so `common` is free once this returns.
    pub fn get_mut(self) -> &'f mut M
    where
        M: NestedMessage<A>,
    {
        Singular::as_mut_for_merge(&mut self.field.store, self.common.alloc.clone())
    }

    /// Clears the nested message, freeing it through the message allocator.
    pub fn clear(self) {
        self.field.deallocate(self.common);
    }
}

impl<'f, 'c, M, const FIELD: u32, A: Allocator, Pb: PresenceBits>
    NestedMessageFieldMut<'f, 'c, M, Oneof, FIELD, A, Pb>
{
    /// Mutably borrows the always-present child.
    /// Borrows only the field (`'f`), so `common` is free once this returns.
    pub fn value_mut(self) -> &'f mut M {
        self.field.value_mut()
    }
}
