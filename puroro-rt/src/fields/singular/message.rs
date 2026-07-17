//! Singular nested message field — LEN wire type with merge-into semantics.
//!
//! Generic parameter order matches the other singular field wrappers:
//! message type `M`, presence policy `P`, field number `FIELD`, allocator `A`.
//!
//! Storage is `ManuallyDrop<P::ValueSlot<UnmanagedBox<M, A>>>`, selected by
//! [`FieldPresence`]:
//!
//! - [`NonOneof`](crate::fields::shared::field_presence::NonOneof) —
//!   `Option<UnmanagedBox<M, A>>`, for ordinary nested message fields whose
//!   presence is the option discriminant (absent vs present).
//! - [`Oneof`](crate::fields::shared::field_presence::Oneof) —
//!   `UnmanagedBox<M, A>` always present under `ManuallyDrop` for oneof message
//!   variants (the enclosing `OneofSlot` tracks case presence).
//!
//! Wire merge / encode / clear delegate to
//! [`ProtoMessage`](crate::fields::wire::proto_message::ProtoMessage) /
//! [`ProtoType`](crate::fields::wire::proto_type::ProtoType).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;
use ::core::ops::{Deref, DerefMut};
use ::unmanaged::UnmanagedBox;

use ::puroro::{DecodeError, Message, WireType};

use crate::fields::shared::{
    FieldDeallocate, MessageCommon, PresenceBits,
    field_presence::{FieldPresence, NonOneof, Oneof},
    slot_init::SlotInitView,
    value_slot::{ValueSlot, ValueSlotRefAccess},
};
use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::proto_type::ProtoType;

/// Singular embedded message field.
///
/// Parametrised like the other singular wrappers: message type `M`, presence
/// policy `P` ([`NonOneof`] / [`Oneof`]), proto field number `FIELD`, allocator
/// `A`.
pub struct NestedMessageField<M, P: FieldPresence, const FIELD: u32, A: Allocator + Clone>
where
    M: Message<Alloc = A>,
    P::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    value: ManuallyDrop<P::ValueSlot<UnmanagedBox<M, A>>>,
    _alloc: PhantomData<A>,
}

impl<M, P: FieldPresence, const FIELD: u32, A: Allocator + Clone> NestedMessageField<M, P, FIELD, A>
where
    M: Message<Alloc = A>,
    P::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    /// Low-level borrow of the child when present.
    ///
    /// Generated message getters go through [`NestedMessageFieldRef::get`] instead.
    #[inline]
    pub fn get<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> Option<&'a M> {
        let init = P::slot_init_view();
        self.value
            .with(init, common)
            .get()
            .map(|b| Deref::deref(b))
    }

    /// Wire byte length of this field occurrence.
    pub fn encoded_len<Pb>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            self.value.with(init, common).get().is_none()
        }) {
            let init = P::slot_init_view();
            let slot = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            <ProtoMessage<M, A> as ProtoType>::encoded_len(Deref::deref(slot), FIELD)
        } else {
            0
        }
    }

    /// Encodes this field occurrence.
    pub fn encode_raw<Pb, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            self.value.with(init, common).get().is_none()
        }) {
            let init = P::slot_init_view();
            let slot = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            <ProtoMessage<M, A> as ProtoType>::encode(Deref::deref(slot), FIELD, buf);
        }
    }
}

impl<M, const FIELD: u32, A: Allocator + Clone> NestedMessageField<M, NonOneof, FIELD, A>
where
    M: Message<Alloc = A>,
    <NonOneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    /// Creates an absent nested message field.
    #[inline]
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: ManuallyDrop::new(ValueSlot::new_in(alloc)),
            _alloc: PhantomData,
        }
    }

    /// Returns whether the field is present.
    #[inline]
    pub fn is_present(&self) -> bool {
        // NonOneof stores `Option<UnmanagedBox<M, A>>`.
        (*self.value).is_some()
    }
}

impl<M, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits> FieldDeallocate<Pb, A>
    for NestedMessageField<M, NonOneof, FIELD, A>
where
    M: Message<Alloc = A>,
    <NonOneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        let init = <NonOneof as FieldPresence>::slot_init_view();
        let initialized = init.is_initialized(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(initialized, alloc);
    }
}

impl<M, const FIELD: u32, A: Allocator + Clone> Default for NestedMessageField<M, NonOneof, FIELD, A>
where
    M: Message<Alloc = A>,
    <NonOneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    fn default() -> Self {
        Self {
            value: ManuallyDrop::new(None),
            _alloc: PhantomData,
        }
    }
}

impl<M, const FIELD: u32, A: Allocator + Clone> NestedMessageField<M, Oneof, FIELD, A>
where
    M: Message<Alloc = A>,
    <Oneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    /// Low-level borrow of the always-present child (no `common`).
    #[inline]
    pub fn value(&self) -> &M {
        Deref::deref(&*self.value)
    }

    /// Mutably borrows the always-present child.
    #[inline]
    pub fn value_mut(&mut self) -> &mut M {
        DerefMut::deref_mut(&mut *self.value)
    }

    /// Builds an always-present field holding a fresh, empty child.
    pub fn with_message_in(alloc: A) -> Self {
        let m = M::new_in(alloc.clone());
        Self {
            value: ManuallyDrop::new(UnmanagedBox::new_in(m, alloc)),
            _alloc: PhantomData,
        }
    }
}

impl<M, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits> FieldDeallocate<Pb, A>
    for NestedMessageField<M, Oneof, FIELD, A>
where
    M: Message<Alloc = A>,
    <Oneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        let init = <Oneof as FieldPresence>::slot_init_view();
        let initialized = init.is_initialized(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(initialized, alloc);
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a nested message field to its message common
/// state, produced by [`SingularAccess::bind`](crate::fields::singular::SingularAccess::bind).
pub struct NestedMessageFieldRef<
    'a,
    M,
    P: FieldPresence,
    const FIELD: u32,
    AField: Allocator + Clone,
    A: Allocator,
    Pb: PresenceBits,
> where
    M: Message<Alloc = AField>,
    P::ValueSlot<UnmanagedBox<M, AField>>: ValueSlot<UnmanagedBox<M, AField>>,
{
    field: &'a NestedMessageField<M, P, FIELD, AField>,
    common: &'a MessageCommon<Pb, A>,
}

impl<
    'a,
    M,
    P: FieldPresence,
    const FIELD: u32,
    AField: Allocator + Clone,
    A: Allocator,
    Pb: PresenceBits,
> NestedMessageFieldRef<'a, M, P, FIELD, AField, A, Pb>
where
    M: Message<Alloc = AField>,
    P::ValueSlot<UnmanagedBox<M, AField>>: ValueSlot<UnmanagedBox<M, AField>>,
{
    #[inline]
    pub(crate) fn new(
        field: &'a NestedMessageField<M, P, FIELD, AField>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }
}

impl<'a, M, P: FieldPresence, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits>
    NestedMessageFieldRef<'a, M, P, FIELD, A, A, Pb>
where
    M: Message<Alloc = A>,
    P::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    /// Returns the child when present.
    #[inline]
    pub fn get(self) -> Option<&'a M> {
        self.field.get(self.common)
    }
}

impl<'a, M, const FIELD: u32, AField: Allocator + Clone, A: Allocator, Pb: PresenceBits>
    NestedMessageFieldRef<'a, M, Oneof, FIELD, AField, A, Pb>
where
    M: Message<Alloc = AField>,
    <Oneof as FieldPresence>::ValueSlot<UnmanagedBox<M, AField>>:
        ValueSlot<UnmanagedBox<M, AField>>,
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
/// produced by [`SingularAccess::bind_mut`](crate::fields::singular::SingularAccess::bind_mut).
pub struct NestedMessageFieldMut<
    'f,
    'c,
    M,
    P: FieldPresence,
    const FIELD: u32,
    AField: Allocator + Clone,
    A: Allocator,
    Pb: PresenceBits,
> where
    M: Message<Alloc = AField>,
    P::ValueSlot<UnmanagedBox<M, AField>>: ValueSlot<UnmanagedBox<M, AField>>,
{
    field: &'f mut NestedMessageField<M, P, FIELD, AField>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<
    'f,
    'c,
    M,
    P: FieldPresence,
    const FIELD: u32,
    AField: Allocator + Clone,
    A: Allocator,
    Pb: PresenceBits,
> NestedMessageFieldMut<'f, 'c, M, P, FIELD, AField, A, Pb>
where
    M: Message<Alloc = AField>,
    P::ValueSlot<UnmanagedBox<M, AField>>: ValueSlot<UnmanagedBox<M, AField>>,
{
    #[inline]
    pub(crate) fn new(
        field: &'f mut NestedMessageField<M, P, FIELD, AField>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }
}

impl<'f, 'c, M, P: FieldPresence, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits>
    NestedMessageFieldMut<'f, 'c, M, P, FIELD, A, A, Pb>
where
    M: Message<Alloc = A>,
    P::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    /// Merges one LEN occurrence into the child (creates the child on first
    /// merge for [`NonOneof`], then merges subsequent occurrences into it).
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
        <ProtoMessage<M, A> as ProtoType>::merge(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
            wire_type,
            buf,
            FIELD,
        )
    }
}

impl<'f, 'c, M, const FIELD: u32, A: Allocator + Clone, Pb: PresenceBits>
    NestedMessageFieldMut<'f, 'c, M, NonOneof, FIELD, A, A, Pb>
where
    M: Message<Alloc = A>,
    <NonOneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
    'c: 'f,
{
    /// Returns a mutable child reference, inserting a default instance if absent.
    /// Borrows only the field (`'f`), so `common` is free once this returns.
    pub fn get_mut(self) -> &'f mut M {
        <ProtoMessage<M, A> as ProtoType>::with_mut(
            &mut *self.field.value,
            <NonOneof as FieldPresence>::slot_init_mut(),
            self.common,
        )
    }

    /// Clears the nested message, freeing it through the message allocator.
    pub fn clear(self) {
        <ProtoMessage<M, A> as ProtoType>::clear(
            &mut *self.field.value,
            <NonOneof as FieldPresence>::slot_init_mut(),
            self.common,
        );
    }
}

impl<'f, 'c, M, const FIELD: u32, AField: Allocator + Clone, A: Allocator, Pb: PresenceBits>
    NestedMessageFieldMut<'f, 'c, M, Oneof, FIELD, AField, A, Pb>
where
    M: Message<Alloc = AField>,
    <Oneof as FieldPresence>::ValueSlot<UnmanagedBox<M, AField>>:
        ValueSlot<UnmanagedBox<M, AField>>,
{
    /// Mutably borrows the always-present child.
    /// Borrows only the field (`'f`), so `common` is free once this returns.
    pub fn value_mut(self) -> &'f mut M {
        self.field.value_mut()
    }
}
