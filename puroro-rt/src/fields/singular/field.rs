//! Unified singular scalar field — varint and LEN share one wrapper.
//!
//! **Singular** here means **non-repeated**: both presence-tracked fields
//! (`EXPLICIT` / “optional”) and non-presence-tracked fields (`IMPLICIT`) use
//! this type. Cardinality (singular vs repeated) is separate from presence
//! ([`FieldPresence`](crate::fields::shared::field_presence::FieldPresence)).
//!
//! Parametrised by protobuf type marker `T: ScalarProtoType`, [`FieldPresence`],
//! proto field number `FIELD`, and compile-time default marker `D`. Physical
//! storage is `P::ValueSlot<T::Slot>` (`T` itself for addressable scalars;
//! `()` for bit-packed [`ProtoBool`](crate::ProtoBool)). Heap payloads are
//! wrapped in [`ManuallyDrop`] so message / oneof `Drop` can release them
//! through [`deallocate`](SingularField::deallocate) without an implicit panic
//! from `UnmanagedString` / `UnmanagedVec`. Copy scalars / unit slots use the
//! same layout; their `DeallocateIn` is a no-op.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::decode;
use crate::defaults::ProtoDefault;
use ::puroro::DecodeError;
use ::puroro::WireType;
use ::puroro::{HasDefault, Optional};

use crate::fields::shared::{
    MessageCommon, PresenceBits,
    field_presence::{FieldPresence, Implicit, LegacyRequired, Oneof, RequiredFieldPresence},
    slot_init::{AlwaysInitialized, SlotInitView},
    value_slot::{ValueSlot, ValueSlotRefAccess},
};
use crate::fields::shared::FieldDeallocate;
use crate::fields::wire::scalar::ScalarProtoType;
use crate::fields::wire::varint::{self, VarintProtoType};

/// Singular (non-repeated) scalar field — varint or LEN, selected by type marker `T`.
///
/// `T` is the protobuf type ([`ScalarProtoType`]); the field stores
/// `P::ValueSlot<T::Slot>`. Covers both `IMPLICIT` and `EXPLICIT` /
/// `LEGACY_REQUIRED` presence via `P`.
pub struct SingularField<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D = ProtoDefault>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    value: ManuallyDrop<P::ValueSlot<T::Slot>>,
    _marker: PhantomData<(T, P, D)>,
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> Clone
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot> + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> Copy
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot> + Copy,
{
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> ::core::fmt::Debug
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot> + ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SingularField")
            .field("value", &*self.value)
            .finish()
    }
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Creates a field with an empty value slot.
    #[inline]
    pub fn new_in<A: Allocator>(alloc: A) -> Self {
        Self {
            value: ManuallyDrop::new(ValueSlot::new_in(alloc)),
            _marker: PhantomData,
        }
    }

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            match self.value.with(init, common).get() {
                Some(slot) => T::is_proto_empty(slot, common),
                None => true,
            }
        }) {
            let init = P::slot_init_view();
            let slot = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            T::encoded_len(T::get(slot, common), FIELD)
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            match self.value.with(init, common).get() {
                Some(slot) => T::is_proto_empty(slot, common),
                None => true,
            }
        }) {
            let init = P::slot_init_view();
            let slot = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            T::encode(T::get(slot, common), FIELD, buf);
        }
    }
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    FieldDeallocate<Pb, A> for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
    A: Clone,
{
    /// Releases the payload through `common`'s allocator.
    ///
    /// No-op for copy scalars / unit bool slots whose [`DeallocateIn`](crate::fields::shared::DeallocateIn)
    /// does nothing.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        let init = P::slot_init_view();
        let initialized = init.is_initialized(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(initialized, alloc);
    }
}

impl<T: ScalarProtoType, const FIELD: u32, D> SingularField<T, Implicit, FIELD, D>
where
    <Implicit as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Low-level borrow of the always-initialized slot's logical value.
    #[inline]
    pub fn value<'a, Pb: PresenceBits, A: Allocator>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> T::Ref<'a> {
        let slot = self
            .value
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot");
        T::get(slot, common)
    }
}

impl<T: ScalarProtoType, const FIELD: u32, D> SingularField<T, Oneof, FIELD, D>
where
    <Oneof as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    /// Low-level borrow of the always-initialized oneof-variant slot's logical value.
    #[inline]
    pub fn value<'a, Pb: PresenceBits, A: Allocator>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> T::Ref<'a> {
        let slot = self
            .value
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot");
        T::get(slot, common)
    }

    /// Mutable accessor for a oneof variant (slot is always initialized).
    pub fn value_mut<'a, Pb: PresenceBits, A: Allocator + Clone>(
        &'a mut self,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> T::Mut<'a, A> {
        T::with_mut(&mut *self.value, AlwaysInitialized, common)
    }
}

impl<T: ScalarProtoType, const BIT: usize, const FIELD: u32, D>
    SingularField<T, LegacyRequired<BIT>, FIELD, D>
where
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    pub fn validate_required<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        LegacyRequired::<BIT>::validate_present(common, FIELD, || {
            let init = <LegacyRequired<BIT> as FieldPresence>::slot_init_view();
            match self.value.with(init, common).get() {
                Some(slot) => T::is_proto_empty(slot, common),
                None => true,
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a singular field to its message common state,
/// produced by [`SingularAccess::bind`](crate::fields::singular::SingularAccess::bind).
pub struct SingularFieldRef<
    'a,
    T: ScalarProtoType,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    field: &'a SingularField<T, P, FIELD, D>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    #[inline]
    pub(crate) fn new(
        field: &'a SingularField<T, P, FIELD, D>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }
}

impl<'a, T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
    for<'b> T::Ref<'b>: Copy,
    D: for<'b> HasDefault<T::Ref<'b>>,
{
    pub fn optional(self) -> Optional<T::Ref<'a>, D> {
        let v = if P::is_set(self.common, || {
            match self
                .field
                .value
                .with(P::slot_init_view(), self.common)
                .get()
            {
                Some(slot) => T::is_proto_empty(slot, self.common),
                None => true,
            }
        }) {
            let slot = self
                .field
                .value
                .with(P::slot_init_view(), self.common)
                .get()
                .expect("is_set implies initialized slot");
            Some(T::get(slot, self.common))
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<'a, T: ScalarProtoType, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, Implicit, FIELD, D, Pb, A>
where
    <Implicit as FieldPresence>::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    #[inline]
    pub fn value(self) -> T::Ref<'a> {
        self.field.value(self.common)
    }
}

impl<'a, T: ScalarProtoType, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, Oneof, FIELD, D, Pb, A>
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
/// produced by [`SingularAccess::bind_mut`](crate::fields::singular::SingularAccess::bind_mut).
pub struct SingularFieldMut<
    'f,
    'c,
    T: ScalarProtoType,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    field: &'f mut SingularField<T, P, FIELD, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<
    'f,
    'c,
    T: ScalarProtoType,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
{
    #[inline]
    pub(crate) fn new(
        field: &'f mut SingularField<T, P, FIELD, D>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns a mutable accessor, lazy-initializing the slot when needed.
    #[inline]
    pub fn value_mut(self) -> T::Mut<'f, A>
    where
        A: Clone,
        'c: 'f,
    {
        T::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
    }

    #[inline]
    pub fn set(self, v: T::Written)
    where
        A: Clone,
    {
        T::write(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
            v,
        );
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        let new = T::decode(wire_type, buf, self.common.alloc.clone())?;
        T::write(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
            new,
        );
        Ok(())
    }

    /// Resets the value slot and clears explicit presence when applicable.
    pub fn clear(self)
    where
        A: Clone,
    {
        T::clear(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        );
    }
}

impl<
    'f,
    'c,
    T: ScalarProtoType + VarintProtoType,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T::Slot>: ValueSlot<T::Slot>,
    T::Written: From<<T as VarintProtoType>::Value>,
    <T as VarintProtoType>::Value: Copy,
{
    /// Merges a closed-enum occurrence; unknown values go to `common.unknown_fields`.
    pub fn merge_closed<B: Buf>(
        self,
        wire_type: WireType,
        buf: &mut B,
        is_known: impl FnOnce(i32) -> bool,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        let wire = varint::ProtoInt32::decode_wire(raw)?;
        if !is_known(wire) {
            decode::save_unknown_varint_field(
                FIELD,
                raw,
                &mut self.common.unknown_fields,
                self.common.alloc.clone(),
            );
            return Ok(());
        }
        let value = T::Written::from(<T as VarintProtoType>::decode_wire(raw)?);
        T::write(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
            value,
        );
        Ok(())
    }
}
