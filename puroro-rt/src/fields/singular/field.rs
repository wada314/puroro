//! Unified singular scalar field — varint and LEN share one wrapper.
//!
//! **Singular** here means **non-repeated**: both presence-tracked fields
//! (`EXPLICIT` / “optional”) and non-presence-tracked fields (`IMPLICIT`) use
//! this type. Cardinality (singular vs repeated) is separate from presence
//! ([`FieldPresence`](crate::fields::shared::field_presence::FieldPresence)).
//!
//! Parametrised by [`ScalarProtoType`] `T` (a **thin wrapper** stored in the
//! field — `ProtoInt32(i32)`, `ProtoString(UnmanagedString)`, …),
//! [`FieldPresence`], proto field number `FIELD`, and compile-time default
//! marker `D`. Heap payloads are wrapped in [`ManuallyDrop`] so message / oneof
//! `Drop` can release them through [`deallocate`](SingularField::deallocate)
//! without an implicit panic from `UnmanagedString` / `UnmanagedVec`. Copy
//! scalars use the same layout; their `DeallocateIn` is a no-op.

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
    slot_init::AlwaysInitialized,
    value_slot::{ValueSlot, ValueSlotMutAccess, ValueSlotRefAccess},
};
use crate::fields::shared::FieldDeallocate;
use crate::fields::wire::scalar::ScalarProtoType;
use crate::fields::wire::varint::{self, VarintProtoType};

/// Singular (non-repeated) scalar field — varint or LEN, selected by `T`.
///
/// `T` is stored directly (thin wrapper). Covers both `IMPLICIT` and
/// `EXPLICIT` / `LEGACY_REQUIRED` presence via `P`.
pub struct SingularField<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D = ProtoDefault>
where
    P::ValueSlot<T>: ValueSlot<T>,
{
    value: ManuallyDrop<P::ValueSlot<T>>,
    _marker: PhantomData<(P, D)>,
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> Clone
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T> + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> Copy
    for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T> + Copy,
{
}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> ::core::fmt::Debug
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

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D> SingularField<T, P, FIELD, D>
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

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        if P::should_emit(common, || P::payload_is_empty(&self.value)) {
            let init = P::slot_init_view(common);
            let v = self
                .value
                .with(&init)
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
            let init = P::slot_init_view(common);
            let v = self
                .value
                .with(&init)
                .get()
                .expect("should_emit implies initialized slot");
            v.encode(FIELD, buf);
        }
    }

}

impl<T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    FieldDeallocate<Pb, A> for SingularField<T, P, FIELD, D>
where
    P::ValueSlot<T>: ValueSlot<T>,
    A: Clone,
{
    /// Releases the payload through `common`'s allocator.
    ///
    /// No-op for copy scalars whose [`DeallocateIn`](crate::fields::shared::DeallocateIn)
    /// does nothing.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        let init = P::slot_init_view(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(&init, alloc);
    }
}

impl<T: ScalarProtoType, const FIELD: u32, D> SingularField<T, Implicit, FIELD, D> {
    /// Low-level borrow of the always-initialized slot (no `common`).
    ///
    /// Generated message getters go through [`SingularFieldRef::value`] instead.
    #[inline]
    pub fn value(&self) -> T::Ref<'_> {
        self.value
            .with(&AlwaysInitialized)
            .get()
            .expect("always-initialized slot")
            .get()
    }
}

impl<T: ScalarProtoType, const FIELD: u32, D> SingularField<T, Oneof, FIELD, D> {
    /// Low-level borrow of the always-initialized oneof-variant slot (no `common`).
    ///
    /// Used by oneof storage projections; generated getters go through
    /// [`SingularFieldRef::value`] / [`SingularFieldRef::optional`].
    #[inline]
    pub fn value(&self) -> T::Ref<'_> {
        self.value
            .with(&AlwaysInitialized)
            .get()
            .expect("always-initialized slot")
            .get()
    }

    /// Mutable accessor for a oneof variant (slot is always initialized).
    pub fn value_mut<A: Allocator + Clone>(&mut self, alloc: A) -> T::Mut<'_, A> {
        ValueSlot::with_mut(&mut *self.value, AlwaysInitialized, alloc.clone())
            .get_mut()
            .with_mut(alloc)
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
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a singular field to its message common state,
/// produced by [`SingularAccess::bind`](crate::fields::singular::SingularAccess::bind).
///
/// Mirrors [`SingularFieldMut`] for the read path. Generated getters always go
/// through this view — `field.bind(&common).optional()` / `.value()` — even when
/// a particular accessor does not consult `common`.
pub struct SingularFieldRef<
    'a,
    T: ScalarProtoType,
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

impl<'a, T: ScalarProtoType, P: FieldPresence, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
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
        let init = P::slot_init_view(self.common);
        let v = if P::is_set(self.common, || P::payload_is_empty(&self.field.value)) {
            Some(
                self.field
                    .value
                    .with(&init)
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
        self.field.value()
    }
}

impl<'a, T: ScalarProtoType, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    SingularFieldRef<'a, T, Oneof, FIELD, D, Pb, A>
{
    #[inline]
    pub fn value(self) -> T::Ref<'a> {
        self.field.value()
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
    P::ValueSlot<T>: ValueSlot<T>,
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
    P::ValueSlot<T>: ValueSlot<T>,
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
    {
        let alloc = self.common.alloc.clone();
        ValueSlot::with_mut(
            &mut *self.field.value,
            P::slot_init_mut(self.common),
            alloc.clone(),
        )
        .get_mut()
        .with_mut(alloc)
    }

    #[inline]
    pub fn set(self, v: T)
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        ValueSlot::with_mut(&mut *self.field.value, P::slot_init_mut(self.common), alloc).set(v);
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        let new = T::decode(wire_type, buf, self.common.alloc.clone())?;
        let alloc = self.common.alloc.clone();
        ValueSlot::with_mut(&mut *self.field.value, P::slot_init_mut(self.common), alloc).set(new);
        Ok(())
    }

    /// Resets the value slot and clears explicit presence when applicable.
    pub fn clear(self)
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        ValueSlot::with_mut(&mut *self.field.value, P::slot_init_mut(self.common), alloc).clear();
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
    ///
    /// `is_known` is called with the decoded `i32` wire value before it is stored.
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
        let alloc = self.common.alloc.clone();
        ValueSlot::with_mut(&mut *self.field.value, P::slot_init_mut(self.common), alloc)
            .set(value);
        Ok(())
    }
}
