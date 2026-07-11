//! Unified singular scalar field — varint and LEN share one wrapper.
//!
//! **Singular** here means **non-repeated**: both presence-tracked fields
//! (`EXPLICIT` / “optional”) and non-presence-tracked fields (`IMPLICIT`) use
//! this type. Cardinality (singular vs repeated) is separate from presence
//! ([`FieldPresence`](crate::fields::shared::field_presence::FieldPresence)).
//!
//! Parametrised by payload type `T` (addressable [`ScalarProtoType`] wrappers, or
//! bit-packed [`ProtoBool`]), [`FieldPresence`], proto field number `FIELD`, and
//! compile-time default marker `D`. Heap payloads are wrapped in [`ManuallyDrop`]
//! so message / oneof `Drop` can release them through
//! [`deallocate`](SingularField::deallocate) without an implicit panic from
//! `UnmanagedString` / `UnmanagedVec`. Copy scalars use the same layout; their
//! `DeallocateIn` is a no-op.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;
use ::core::ops::DerefMut;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::decode;
use crate::defaults::ProtoDefault;
use crate::encode;
use ::puroro::DecodeError;
use ::puroro::WireType;
use ::puroro::{HasDefault, Optional};

use crate::fields::shared::{
    DeallocateIn, DefaultIn, MessageCommon, PresenceBits,
    field_presence::{FieldPresence, Implicit, LegacyRequired, Oneof, RequiredFieldPresence},
    slot_init::{AlwaysInitialized, SlotInitView},
    value_slot::{ValueSlot, ValueSlotMutAccess, ValueSlotRefAccess},
};
use crate::fields::shared::FieldDeallocate;
use crate::fields::wire::scalar::ScalarProtoType;
use crate::fields::wire::varint::{self, ProtoBool, VarintProtoType};

/// Singular (non-repeated) scalar field — varint or LEN, selected by `T`.
///
/// `T` is stored directly (thin wrapper or [`ProtoBool`] ZST). Covers both
/// `IMPLICIT` and `EXPLICIT` / `LEGACY_REQUIRED` presence via `P`.
pub struct SingularField<T: DefaultIn + DeallocateIn, P: FieldPresence, const FIELD: u32, D = ProtoDefault>
where
    P::ValueSlot<T>: ValueSlot<T>,
{
    value: ManuallyDrop<P::ValueSlot<T>>,
    _marker: PhantomData<(P, D)>,
}

impl<T: DefaultIn + DeallocateIn, P: FieldPresence, const FIELD: u32, D> Clone
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T> + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: DefaultIn + DeallocateIn, P: FieldPresence, const FIELD: u32, D> Copy
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T> + Copy,
{
}

impl<T: DefaultIn + DeallocateIn, P: FieldPresence, const FIELD: u32, D> ::core::fmt::Debug
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T> + ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SingularField")
            .field("value", &*self.value)
            .finish()
    }
}

impl<T: DefaultIn + DeallocateIn, P: FieldPresence, const FIELD: u32, D> SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T>,
{
    /// Creates a field with an empty value slot.
    #[inline]
    pub fn new_in<A: Allocator>(alloc: A) -> Self {
        Self {
            value: ManuallyDrop::new(ValueSlot::new_in(alloc)),
            _marker: PhantomData,
        }
    }
}

// ---------------------------------------------------------------------------
// Addressable ScalarProtoType encode / views
// ---------------------------------------------------------------------------

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T>,
{
    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || P::payload_is_empty(&self.value)) {
            let init = P::slot_init_view();
            let v = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            v.encoded_len(FIELD)
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || P::payload_is_empty(&self.value)) {
            let init = P::slot_init_view();
            let v = self
                .value
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            v.encode(FIELD, buf);
        }
    }
}

impl<T: DefaultIn + DeallocateIn, P: FieldPresence, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    FieldDeallocate<Pb, A> for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T>,
    A: Clone,
{
    /// Releases the payload through `common`'s allocator.
    ///
    /// No-op for copy scalars / [`ProtoBool`] whose [`DeallocateIn`] does nothing.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        let init = P::slot_init_view();
        let initialized = init.is_initialized(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(initialized, alloc);
    }
}

impl<T: ScalarProtoType, const FIELD: u32, D> SingularField<T, Implicit, FIELD, D> {
    /// Low-level borrow of the always-initialized slot.
    #[inline]
    pub fn value<'a, Pb: PresenceBits, A: Allocator>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> T::Ref<'a> {
        self.value
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot")
            .get()
    }
}

impl<T: ScalarProtoType, const FIELD: u32, D> SingularField<T, Oneof, FIELD, D> {
    /// Low-level borrow of the always-initialized oneof-variant slot.
    #[inline]
    pub fn value<'a, Pb: PresenceBits, A: Allocator>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> T::Ref<'a> {
        self.value
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot")
            .get()
    }

    /// Mutable accessor for a oneof variant (slot is always initialized).
    pub fn value_mut<'a, Pb: PresenceBits, A: Allocator + Clone>(
        &'a mut self,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> T::Mut<'a, A> {
        let alloc = common.alloc.clone();
        T::with_mut(
            ValueSlot::with_mut(&mut *self.value, AlwaysInitialized, common).get_mut(),
            alloc,
        )
    }
}

impl<T: ScalarProtoType, const BIT: usize, const FIELD: u32, D>
    SingularField<T, LegacyRequired<BIT>, FIELD, D>
where
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<T>: ValueSlot<T>,
{
    pub fn validate_required<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        LegacyRequired::<BIT>::validate_present(common, FIELD, || {
            LegacyRequired::<BIT>::payload_is_empty(&self.value)
        })
    }
}

// ---------------------------------------------------------------------------
// ProtoBool encode / views
// ---------------------------------------------------------------------------

impl<const VALUE_BIT: usize, P: FieldPresence, const FIELD: u32, D>
    SingularField<ProtoBool<VALUE_BIT>, P, FIELD, D>
where
    P::ValueSlot<ProtoBool<VALUE_BIT>>: ValueSlot<ProtoBool<VALUE_BIT>>,
{
    #[inline]
    fn read_value<Pb: PresenceBits, A: Allocator>(common: &MessageCommon<Pb, A>) -> bool {
        common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn value_is_empty<Pb: PresenceBits, A: Allocator>(common: &MessageCommon<Pb, A>) -> bool {
        !Self::read_value(common)
    }

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || Self::value_is_empty(common)) {
            encode::encoded_len_varint_field(
                FIELD,
                ProtoBool::<VALUE_BIT>::encode_wire(Self::read_value(common)),
            )
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || Self::value_is_empty(common)) {
            encode::encode_varint_field(
                FIELD,
                ProtoBool::<VALUE_BIT>::encode_wire(Self::read_value(common)),
                buf,
            );
        }
    }
}

impl<const VALUE_BIT: usize, const FIELD: u32, D>
    SingularField<ProtoBool<VALUE_BIT>, Implicit, FIELD, D>
{
    #[inline]
    pub fn value<Pb: PresenceBits, A: Allocator>(&self, common: &MessageCommon<Pb, A>) -> bool {
        Self::read_value(common)
    }
}

impl<const VALUE_BIT: usize, const FIELD: u32, D>
    SingularField<ProtoBool<VALUE_BIT>, Oneof, FIELD, D>
{
    #[inline]
    pub fn value<Pb: PresenceBits, A: Allocator>(&self, common: &MessageCommon<Pb, A>) -> bool {
        Self::read_value(common)
    }

    pub fn value_mut<'a, Pb: PresenceBits, A: Allocator + Clone>(
        &'a mut self,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> impl DerefMut<Target = bool> + 'a {
        let _ = ValueSlot::with_mut(&mut *self.value, AlwaysInitialized, common).get_mut();
        common.bit_mut(VALUE_BIT)
    }
}

impl<const VALUE_BIT: usize, const BIT: usize, const FIELD: u32, D>
    SingularField<ProtoBool<VALUE_BIT>, LegacyRequired<BIT>, FIELD, D>
where
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<ProtoBool<VALUE_BIT>>:
        ValueSlot<ProtoBool<VALUE_BIT>>,
{
    pub fn validate_required<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        LegacyRequired::<BIT>::validate_present(common, FIELD, || Self::value_is_empty(common))
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a singular field to its message common state,
/// produced by [`SingularAccess::bind`](crate::fields::singular::SingularAccess::bind).
pub struct SingularFieldRef<
    'a,
    T: DefaultIn + DeallocateIn,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> where
    P::ValueSlot<T>: ValueSlot<T>,
{
    field: &'a SingularField<T, P, FIELD, D>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, T: DefaultIn + DeallocateIn, P: FieldPresence, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T>: ValueSlot<T>,
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
    P::ValueSlot<T>: ValueSlot<T>,
    for<'b> T::Ref<'b>: Copy,
    D: for<'b> HasDefault<T::Ref<'b>>,
{
    pub fn optional(self) -> Optional<T::Ref<'a>, D> {
        let init = P::slot_init_view();
        let v = if P::is_set(self.common, || P::payload_is_empty(&self.field.value)) {
            Some(
                self.field
                    .value
                    .with(init, self.common)
                    .get()
                    .expect("is_set implies initialized slot")
                    .get(),
            )
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<'a, T: ScalarProtoType, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, Implicit, FIELD, D, Pb, A>
{
    #[inline]
    pub fn value(self) -> T::Ref<'a> {
        self.field.value(self.common)
    }
}

impl<'a, T: ScalarProtoType, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, Oneof, FIELD, D, Pb, A>
{
    #[inline]
    pub fn value(self) -> T::Ref<'a> {
        self.field.value(self.common)
    }
}

impl<
    'a,
    const VALUE_BIT: usize,
    P: FieldPresence,
    const FIELD: u32,
    D: HasDefault<bool>,
    Pb: PresenceBits,
    A: Allocator,
> SingularFieldRef<'a, ProtoBool<VALUE_BIT>, P, FIELD, D, Pb, A>
where
    P::ValueSlot<ProtoBool<VALUE_BIT>>: ValueSlot<ProtoBool<VALUE_BIT>>,
{
    pub fn optional(self) -> Optional<bool, D> {
        let v = if P::is_set(self.common, || {
            SingularField::<ProtoBool<VALUE_BIT>, P, FIELD, D>::value_is_empty(self.common)
        }) {
            Some(SingularField::<ProtoBool<VALUE_BIT>, P, FIELD, D>::read_value(
                self.common,
            ))
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<'a, const VALUE_BIT: usize, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, ProtoBool<VALUE_BIT>, Implicit, FIELD, D, Pb, A>
{
    #[inline]
    pub fn value(self) -> bool {
        self.field.value(self.common)
    }
}

impl<'a, const VALUE_BIT: usize, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, ProtoBool<VALUE_BIT>, Oneof, FIELD, D, Pb, A>
{
    #[inline]
    pub fn value(self) -> bool {
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
    T: DefaultIn + DeallocateIn,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> where
    P::ValueSlot<T>: ValueSlot<T>,
{
    field: &'f mut SingularField<T, P, FIELD, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<
    'f,
    'c,
    T: DefaultIn + DeallocateIn,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> SingularFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T>: ValueSlot<T>,
{
    #[inline]
    pub(crate) fn new(
        field: &'f mut SingularField<T, P, FIELD, D>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }
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
    P::ValueSlot<T>: ValueSlot<T>,
{
    /// Returns a mutable accessor, lazy-initializing the slot when needed.
    #[inline]
    pub fn value_mut(self) -> T::Mut<'f, A>
    where
        A: Clone,
        'c: 'f,
    {
        let alloc = self.common.alloc.clone();
        T::with_mut(
            ValueSlot::with_mut(&mut *self.field.value, P::slot_init_mut(), self.common).get_mut(),
            alloc,
        )
    }

    #[inline]
    pub fn set(self, v: T)
    where
        A: Clone,
    {
        ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .set(v);
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        let new = T::decode(wire_type, buf, self.common.alloc.clone())?;
        ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .set(new);
        Ok(())
    }

    /// Resets the value slot and clears explicit presence when applicable.
    pub fn clear(self)
    where
        A: Clone,
    {
        ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .clear();
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
    P::ValueSlot<T>: ValueSlot<T>,
    T: From<<T as VarintProtoType>::Value>,
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
        let value = T::from(<T as VarintProtoType>::decode_wire(raw)?);
        ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .set(value);
        Ok(())
    }
}

impl<
    'f,
    'c,
    const VALUE_BIT: usize,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> SingularFieldMut<'f, 'c, ProtoBool<VALUE_BIT>, P, FIELD, D, Pb, A>
where
    P::ValueSlot<ProtoBool<VALUE_BIT>>: ValueSlot<ProtoBool<VALUE_BIT>>,
{
    /// Ensures presence (when applicable) and returns a mutable handle to the value bit.
    #[inline]
    pub fn value_mut(self) -> impl DerefMut<Target = bool> + 'c
    where
        A: Clone,
    {
        let _ = ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .get_mut();
        self.common.bit_mut(VALUE_BIT)
    }

    #[inline]
    pub fn set(self, v: bool)
    where
        A: Clone,
    {
        let _ = ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .get_mut();
        self.common.set_bit(VALUE_BIT, v);
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        let v = ProtoBool::<VALUE_BIT>::decode_wire(raw)?;
        let _ = ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .get_mut();
        self.common.set_bit(VALUE_BIT, v);
        Ok(())
    }

    pub fn clear(self)
    where
        A: Clone,
    {
        ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
        )
        .clear();
    }
}
