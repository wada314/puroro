//! Unified singular scalar field — varint and LEN share one wrapper.
//!
//! **Singular** here means **non-repeated**: both presence-tracked fields
//! (`EXPLICIT` / “optional”) and non-presence-tracked fields (`IMPLICIT`) use
//! this type. Cardinality (singular vs repeated) is separate from presence
//! ([`FieldPresence`](crate::fields::shared::field_presence::FieldPresence)).
//!
//! Parametrised by protobuf type marker `T: ProtoType`, [`FieldPresence`],
//! proto field number `FIELD`, allocator `A`, value [`ValueLayout`] `L`
//! (`Inline` or [`BitPacked`] for bool), and compile-time default marker `D`.
//! Physical storage is `P::ValueSlot<T::Slot<A>>`. Heap payloads are wrapped in
//! [`ManuallyDrop`] so message / oneof `Drop` can release them through
//! [`deallocate`](SingularField::deallocate) without an implicit panic from
//! `UnmanagedString` / `UnmanagedVec`. Copy scalars / ZST bool slots use the
//! same layout; their `DeallocateIn` is a no-op.

use ::core::fmt::{self, Debug, Formatter, Result as FmtResult};
use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::defaults::ProtoDefault;
use ::puroro::DecodeError;
use ::puroro::WireType;
use ::puroro::{HasDefault, Optional};

use crate::fields::shared::FieldDeallocate;
use crate::fields::shared::{
    DeallocateIn, DefaultIn, MessageCommon, PresenceBits,
    field_presence::{
        FieldPresence, Implicit, LegacyRequired, NonOneof, Oneof, RequiredFieldPresence,
    },
    slot_init::{AlwaysInitialized, SlotInitView},
    value_layout::{Inline, ValueLayout},
    value_slot::{AddressableSlot, ValueSlot, ValueSlotRefAccess},
};
use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::proto_type::ProtoType;
use ::puroro::Message;
use ::unmanaged::UnmanagedBox;

/// Singular (non-repeated) scalar field — varint or LEN, selected by type marker `T`.
///
/// Parameter order: `T`, `P`, `FIELD`, `A`, `L = Inline`, `D = ProtoDefault`.
pub struct SingularField<
    T: ProtoType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator + Clone,
    L: ValueLayout<T, A> = Inline,
    D = ProtoDefault,
> where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    value: ManuallyDrop<P::ValueSlot<T::Slot<A>>>,
    _marker: PhantomData<(T, P, A, L, D)>,
}

impl<T, P, const FIELD: u32, A, L, D> Clone for SingularField<T, P, FIELD, A, L, D>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A> + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, P, const FIELD: u32, A, L, D> Copy for SingularField<T, P, FIELD, A, L, D>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A> + Copy,
{
}

impl<T, P, const FIELD: u32, A, L, D> fmt::Debug for SingularField<T, P, FIELD, A, L, D>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SingularField")
            .field("value", &*self.value)
            .finish()
    }
}

impl<T, P, const FIELD: u32, A, L, D> SingularField<T, P, FIELD, A, L, D>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    /// Creates a field with an empty value slot.
    #[inline]
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: ManuallyDrop::new(ValueSlot::new_in(alloc)),
            _marker: PhantomData,
        }
    }

    /// Binds this field to `common` for read access.
    #[inline]
    pub fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> SingularFieldRef<'a, T, P, FIELD, A, L, D, Pb> {
        SingularFieldRef::new(self, common)
    }

    /// Binds this field to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularFieldMut<'f, 'c, T, P, FIELD, A, L, D, Pb> {
        SingularFieldMut::new(self, common)
    }

    pub fn encoded_len<Pb>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            match self.value.with(init, common).get() {
                Some(slot) => L::is_proto_empty(slot, common),
                None => true,
            }
        }) {
            let init = P::slot_init_view();
            let slot = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            T::encoded_len(L::get(slot, common), FIELD)
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            match self.value.with(init, common).get() {
                Some(slot) => L::is_proto_empty(slot, common),
                None => true,
            }
        }) {
            let init = P::slot_init_view();
            let slot = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            T::encode(L::get(slot, common), FIELD, buf);
        }
    }
}

impl<T, P, const FIELD: u32, A, L, D, Pb> FieldDeallocate<Pb, A>
    for SingularField<T, P, FIELD, A, L, D>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    Pb: PresenceBits,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    /// Releases the payload through `common`'s allocator.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        let init = P::slot_init_view();
        let initialized = init.is_initialized(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(initialized, alloc);
    }
}

impl<T, const FIELD: u32, A, L, D> SingularField<T, Implicit, FIELD, A, L, D>
where
    T: ProtoType,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    <Implicit as FieldPresence>::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    /// Low-level borrow of the always-initialized slot's logical value.
    #[inline]
    pub fn value<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> T::Ref<'a, A> {
        let slot = self
            .value
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot");
        L::get(slot, common)
    }
}

impl<T, const FIELD: u32, A, L, D> SingularField<T, Oneof, FIELD, A, L, D>
where
    T: ProtoType,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    <Oneof as FieldPresence>::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    /// Low-level borrow of the always-initialized oneof-variant slot's logical value.
    #[inline]
    pub fn value<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> T::Ref<'a, A> {
        let slot = self
            .value
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot");
        L::get(slot, common)
    }

    /// Mutable accessor for a oneof variant (slot is always initialized).
    pub fn value_mut<'a, Pb: PresenceBits>(
        &'a mut self,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> T::Mut<'a, A> {
        L::with_mut(&mut *self.value, AlwaysInitialized, common)
    }
}

impl<T, const FIELD: u32, A, L, D> SingularField<T, NonOneof, FIELD, A, L, D>
where
    T: ProtoType,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    <NonOneof as FieldPresence>::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    /// Returns whether the pointer-present slot holds a value.
    #[inline]
    pub fn is_present(&self) -> bool {
        (*self.value).is_some()
    }
}

impl<M, const FIELD: u32, A, L, D> SingularField<ProtoMessage<M>, Oneof, FIELD, A, L, D>
where
    M: Message<Alloc = A>,
    A: Allocator + Clone,
    L: ValueLayout<ProtoMessage<M>, A>,
    <Oneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>, A>,
{
    /// Builds an always-present nested-message oneof variant with an empty child.
    #[inline]
    pub fn with_message_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<T, const BIT: usize, const FIELD: u32, A, L, D>
    SingularField<T, LegacyRequired<BIT>, FIELD, A, L, D>
where
    T: ProtoType,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    pub fn validate_required<Pb>(&self, common: &MessageCommon<Pb, A>) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
    {
        LegacyRequired::<BIT>::validate_present(common, FIELD, || {
            let init = <LegacyRequired<BIT> as FieldPresence>::slot_init_view();
            match self.value.with(init, common).get() {
                Some(slot) => L::is_proto_empty(slot, common),
                None => true,
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a singular field to its message common state,
/// produced by [`SingularField::bind`].
pub struct SingularFieldRef<
    'a,
    T: ProtoType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    D,
    Pb: PresenceBits,
> where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    field: &'a SingularField<T, P, FIELD, A, L, D>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, T, P, const FIELD: u32, A, L, D, Pb> SingularFieldRef<'a, T, P, FIELD, A, L, D, Pb>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    Pb: PresenceBits,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    #[inline]
    pub(crate) fn new(
        field: &'a SingularField<T, P, FIELD, A, L, D>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns the logical value when the field is present.
    pub fn get(self) -> Option<T::Ref<'a, A>> {
        if P::is_set(self.common, || {
            match self
                .field
                .value
                .with(P::slot_init_view(), self.common)
                .get()
            {
                Some(slot) => L::is_proto_empty(slot, self.common),
                None => true,
            }
        }) {
            let slot = self
                .field
                .value
                .with(P::slot_init_view(), self.common)
                .get()
                .expect("is_set implies initialized slot");
            Some(L::get(slot, self.common))
        } else {
            None
        }
    }
}

impl<'a, T, P, const FIELD: u32, A, L, D, Pb> SingularFieldRef<'a, T, P, FIELD, A, L, D, Pb>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    Pb: PresenceBits,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
    T::Ref<'a, A>: Copy,
    D: HasDefault<T::Ref<'a, A>>,
{
    pub fn optional(self) -> Optional<T::Ref<'a, A>, D> {
        Optional::new(self.get())
    }
}

impl<'a, T, const FIELD: u32, A, L, D, Pb> SingularFieldRef<'a, T, Implicit, FIELD, A, L, D, Pb>
where
    T: ProtoType,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    Pb: PresenceBits,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    <Implicit as FieldPresence>::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    #[inline]
    pub fn value(self) -> T::Ref<'a, A> {
        self.field.value(self.common)
    }
}

impl<'a, T, const FIELD: u32, A, L, D, Pb> SingularFieldRef<'a, T, Oneof, FIELD, A, L, D, Pb>
where
    T: ProtoType,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    Pb: PresenceBits,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    <Oneof as FieldPresence>::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    #[inline]
    pub fn value(self) -> T::Ref<'a, A> {
        self.field.value(self.common)
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a singular field to its message common state,
/// produced by [`SingularField::bind_mut`].
pub struct SingularFieldMut<
    'f,
    'c,
    T: ProtoType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    D,
    Pb: PresenceBits,
> where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    field: &'f mut SingularField<T, P, FIELD, A, L, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, T, P, const FIELD: u32, A, L, D, Pb> SingularFieldMut<'f, 'c, T, P, FIELD, A, L, D, Pb>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    Pb: PresenceBits,
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>, A>,
{
    #[inline]
    pub(crate) fn new(
        field: &'f mut SingularField<T, P, FIELD, A, L, D>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns a mutable accessor, lazy-initializing the slot when needed.
    #[inline]
    pub fn value_mut(self) -> T::Mut<'f, A>
    where
        'c: 'f,
    {
        L::with_mut(&mut *self.field.value, P::slot_init_mut(), self.common)
    }

    /// Alias of [`value_mut`](Self::value_mut) for nested-message call sites.
    #[inline]
    pub fn get_mut(self) -> T::Mut<'f, A>
    where
        'c: 'f,
    {
        self.value_mut()
    }

    #[inline]
    pub fn set(self, v: T::Written<A>) {
        L::write(&mut *self.field.value, P::slot_init_mut(), self.common, v);
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
        L::merge(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
            wire_type,
            buf,
            FIELD,
        )
    }

    /// Resets the value slot and clears explicit presence when applicable.
    pub fn clear(self) {
        L::clear(&mut *self.field.value, P::slot_init_mut(), self.common);
    }
}
