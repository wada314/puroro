//! Unified singular scalar field — varint and LEN share one wrapper.
//!
//! **Singular** here means **non-repeated**: both presence-tracked fields
//! (`EXPLICIT` / “optional”) and non-presence-tracked fields (`IMPLICIT`) use
//! this type. Cardinality (singular vs repeated) is separate from presence
//! ([`FieldPresence`](crate::fields::shared::field_presence::FieldPresence)).
//!
//! Parametrised by protobuf type marker `T: ProtoType`, [`FieldPresence`],
//! proto field number `FIELD`, value [`ValueLayout`] `L` (`Inline` or
//! [`BitPacked`] for bool), and compile-time default marker `D`. Physical
//! storage is `P::ValueSlot<T::Slot>` (`T` itself — including ZST
//! [`ProtoBool`](crate::ProtoBool)). Heap payloads are
//! wrapped in [`ManuallyDrop`] so message / oneof `Drop` can release them
//! through [`deallocate`](SingularField::deallocate) without an implicit panic
//! from `UnmanagedString` / `UnmanagedVec`. Copy scalars / ZST bool slots use the
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
    MessageCommon, PresenceBits,
    field_presence::{
        FieldPresence, Implicit, LegacyRequired, NonOneof, Oneof, RequiredFieldPresence,
    },
    slot_init::{AlwaysInitialized, SlotInitView},
    value_layout::{Inline, ValueLayout},
    value_slot::{ValueSlot, ValueSlotRefAccess},
};
use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::proto_type::ProtoType;
use ::puroro::Message;
use ::unmanaged::UnmanagedBox;

/// Singular (non-repeated) scalar field — varint or LEN, selected by type marker `T`.
///
/// `T` is the protobuf type ([`ProtoType`]); the field stores
/// `P::ValueSlot<T::Slot>`. Covers both `IMPLICIT` and `EXPLICIT` /
/// `LEGACY_REQUIRED` presence via `P`. Value storage layout is `L`
/// ([`Inline`] or [`BitPacked`](crate::fields::shared::value_layout::BitPacked)).
pub struct SingularField<
    T: ProtoType,
    P: FieldPresence,
    const FIELD: u32,
    L: ValueLayout<T> = Inline,
    D = ProtoDefault,
> where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    value: ManuallyDrop<P::ValueSlot<T::Slot>>,
    _marker: PhantomData<(T, P, L, D)>,
}

impl<T: ProtoType, P: FieldPresence, const FIELD: u32, L: ValueLayout<T>, D> Clone
    for SingularField<T, P, FIELD, L, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot> + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ProtoType, P: FieldPresence, const FIELD: u32, L: ValueLayout<T>, D> Copy
    for SingularField<T, P, FIELD, L, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot> + Copy,
{
}

impl<T: ProtoType, P: FieldPresence, const FIELD: u32, L: ValueLayout<T>, D> fmt::Debug
    for SingularField<T, P, FIELD, L, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SingularField")
            .field("value", &*self.value)
            .finish()
    }
}

impl<T: ProtoType, P: FieldPresence, const FIELD: u32, L: ValueLayout<T>, D>
    SingularField<T, P, FIELD, L, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Creates a field with an empty value slot.
    #[inline]
    pub fn new_in(alloc: T::Alloc) -> Self {
        Self {
            value: ManuallyDrop::new(ValueSlot::new_in(alloc)),
            _marker: PhantomData,
        }
    }

    /// Binds this field to `common` for read access.
    #[inline]
    pub fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> SingularFieldRef<'a, T, P, FIELD, L, D, Pb, T::Alloc> {
        SingularFieldRef::new(self, common)
    }

    /// Binds this field to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, T::Alloc>,
    ) -> SingularFieldMut<'f, 'c, T, P, FIELD, L, D, Pb, T::Alloc> {
        SingularFieldMut::new(self, common)
    }

    pub fn encoded_len<Pb>(&self, common: &MessageCommon<Pb, T::Alloc>) -> usize
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

    pub fn encode_raw<Pb, B: BufMut>(&self, common: &MessageCommon<Pb, T::Alloc>, buf: &mut B)
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

impl<T: ProtoType, P: FieldPresence, const FIELD: u32, L: ValueLayout<T>, D, Pb: PresenceBits>
    FieldDeallocate<Pb, T::Alloc> for SingularField<T, P, FIELD, L, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Releases the payload through `common`'s allocator.
    ///
    /// No-op for copy scalars / unit bool slots whose [`DeallocateIn`](crate::fields::shared::DeallocateIn)
    /// does nothing.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, T::Alloc>) {
        let init = P::slot_init_view();
        let initialized = init.is_initialized(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(initialized, alloc);
    }
}

impl<T: ProtoType, const FIELD: u32, L: ValueLayout<T>, D> SingularField<T, Implicit, FIELD, L, D>
where
    <Implicit as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Low-level borrow of the always-initialized slot's logical value.
    #[inline]
    pub fn value<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> T::Ref<'a> {
        let slot = self
            .value
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot");
        L::get(slot, common)
    }
}

impl<T: ProtoType, const FIELD: u32, L: ValueLayout<T>, D> SingularField<T, Oneof, FIELD, L, D>
where
    <Oneof as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Low-level borrow of the always-initialized oneof-variant slot's logical value.
    #[inline]
    pub fn value<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> T::Ref<'a> {
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
        common: &'a mut MessageCommon<Pb, T::Alloc>,
    ) -> T::Mut<'a> {
        L::with_mut(&mut *self.value, AlwaysInitialized, common)
    }
}

impl<T: ProtoType, const FIELD: u32, L: ValueLayout<T>, D> SingularField<T, NonOneof, FIELD, L, D>
where
    <NonOneof as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Returns whether the pointer-present slot holds a value.
    #[inline]
    pub fn is_present(&self) -> bool {
        (*self.value).is_some()
    }
}

impl<M, const FIELD: u32, L: ValueLayout<ProtoMessage<M, A>>, D, A: Allocator + Clone>
    SingularField<ProtoMessage<M, A>, Oneof, FIELD, L, D>
where
    M: Message<Alloc = A>,
    <Oneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>>,
{
    /// Builds an always-present nested-message oneof variant with an empty child.
    #[inline]
    pub fn with_message_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<T: ProtoType, const BIT: usize, const FIELD: u32, L: ValueLayout<T>, D>
    SingularField<T, LegacyRequired<BIT>, FIELD, L, D>
where
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    pub fn validate_required<Pb>(
        &self,
        common: &MessageCommon<Pb, T::Alloc>,
    ) -> Result<(), DecodeError>
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
    L: ValueLayout<T>,
    D,
    Pb: PresenceBits,
    A: Allocator,
> where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    field: &'a SingularField<T, P, FIELD, L, D>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, T: ProtoType, P: FieldPresence, const FIELD: u32, L: ValueLayout<T>, D, Pb: PresenceBits>
    SingularFieldRef<'a, T, P, FIELD, L, D, Pb, T::Alloc>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    #[inline]
    pub(crate) fn new(
        field: &'a SingularField<T, P, FIELD, L, D>,
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns the logical value when the field is present.
    ///
    /// Unlike [`optional`](Self::optional), this does not require `Ref: Copy`,
    /// so nested messages (`Ref = &M`) can use it.
    pub fn get(self) -> Option<T::Ref<'a>> {
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

impl<'a, T: ProtoType, P: FieldPresence, const FIELD: u32, L: ValueLayout<T>, D, Pb: PresenceBits>
    SingularFieldRef<'a, T, P, FIELD, L, D, Pb, T::Alloc>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
    T::Ref<'a>: Copy,
    D: HasDefault<T::Ref<'a>>,
{
    pub fn optional(self) -> Optional<T::Ref<'a>, D> {
        Optional::new(self.get())
    }
}

impl<'a, T: ProtoType, const FIELD: u32, L: ValueLayout<T>, D, Pb: PresenceBits>
    SingularFieldRef<'a, T, Implicit, FIELD, L, D, Pb, T::Alloc>
where
    <Implicit as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    #[inline]
    pub fn value(self) -> T::Ref<'a> {
        self.field.value(self.common)
    }
}

impl<'a, T: ProtoType, const FIELD: u32, L: ValueLayout<T>, D, Pb: PresenceBits>
    SingularFieldRef<'a, T, Oneof, FIELD, L, D, Pb, T::Alloc>
where
    <Oneof as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    #[inline]
    pub fn value(self) -> T::Ref<'a> {
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
    L: ValueLayout<T>,
    D,
    Pb: PresenceBits,
    A: Allocator,
> where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    field: &'f mut SingularField<T, P, FIELD, L, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<
    'f,
    'c,
    T: ProtoType,
    P: FieldPresence,
    const FIELD: u32,
    L: ValueLayout<T>,
    D,
    Pb: PresenceBits,
> SingularFieldMut<'f, 'c, T, P, FIELD, L, D, Pb, T::Alloc>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    #[inline]
    pub(crate) fn new(
        field: &'f mut SingularField<T, P, FIELD, L, D>,
        common: &'c mut MessageCommon<Pb, T::Alloc>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns a mutable accessor, lazy-initializing the slot when needed.
    #[inline]
    pub fn value_mut(self) -> T::Mut<'f>
    where
        'c: 'f,
    {
        L::with_mut(&mut *self.field.value, P::slot_init_mut(), self.common)
    }

    /// Alias of [`value_mut`](Self::value_mut) for nested-message call sites.
    #[inline]
    pub fn get_mut(self) -> T::Mut<'f>
    where
        'c: 'f,
    {
        self.value_mut()
    }

    #[inline]
    pub fn set(self, v: T::Written) {
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
