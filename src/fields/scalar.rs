//! Singular varint field wrapper — generic over wire type and presence policy.
//!
//! Fixed-width scalars will follow the same `Singular*Field<T, P, FIELD>` pattern with
//! [`Fixed32ProtoType`](super::fixed32::Fixed32ProtoType) /
//! [`Fixed64ProtoType`](super::fixed64::Fixed64ProtoType).

use ::core::marker::PhantomData;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::decode;
use crate::encode;
use crate::error::DecodeError;
use crate::defaults::ProtoDefault;
use crate::optional::{HasDefault, Optional};
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::field_presence::FieldPresence;
use super::presence::PresenceBits;
use super::slot_init::AlwaysInitialized;
use super::value_slot::ValueSlot;
use super::varint::{self, VarintProtoType};

/// Singular scalar on the wire as VARINT — parametrised by protobuf type `T`,
/// presence policy `P` ([`Implicit`] / [`Explicit`](super::field_presence::Explicit)
/// / [`LegacyRequired`](super::field_presence::LegacyRequired)), and proto field
/// number `FIELD`, and compile-time default marker `D` ([`HasDefault`]).
#[derive(Clone, Copy, Debug)]
pub struct SingularVarintField<
    T: VarintProtoType,
    P: FieldPresence,
    const FIELD: u32,
    D = ProtoDefault,
> {
    value: P::ValueSlot<T::Value>,
    _marker: PhantomData<(P, D)>,
}

impl<T: VarintProtoType, P: FieldPresence, const FIELD: u32, D> SingularVarintField<T, P, FIELD, D>
where
    P::ValueSlot<T::Value>: ValueSlot<T::Value>,
{
    /// Creates a field with an empty value slot.
    #[inline]
    pub fn new_in<A: Allocator>(_alloc: A) -> Self {
        Self {
            value: ValueSlot::new(),
            _marker: PhantomData,
        }
    }

    /// Binds this field to its message `common` state (presence), producing a
    /// short-lived [`SingularVarintFieldMut`] view that carries the whole
    /// mutation context.
    #[inline]
    pub fn bind<'f, 'c, Pb: PresenceBits, A: Allocator>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularVarintFieldMut<'f, 'c, T, P, FIELD, D, Pb, A> {
        SingularVarintFieldMut::new(self, common)
    }

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        if P::should_emit(common, || P::payload_is_empty(&self.value)) {
            let init = P::slot_init_view(common);
            let v = *self
                .value
                .as_ref(&init)
                .expect("should_emit implies initialized slot");
            encode::encoded_len_varint_field(FIELD, T::encode_wire(v))
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        if P::should_emit(common, || P::payload_is_empty(&self.value)) {
            let init = P::slot_init_view(common);
            let v = *self
                .value
                .as_ref(&init)
                .expect("should_emit implies initialized slot");
            encode::encode_varint_field(FIELD, T::encode_wire(v), buf);
        }
    }

    #[inline]
    pub fn has<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> bool
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        P::is_set(common, || P::payload_is_empty(&self.value))
    }
}

impl<T: VarintProtoType, P: FieldPresence, const FIELD: u32, D> SingularVarintField<T, P, FIELD, D>
where
    P::ValueSlot<T::Value>: ValueSlot<T::Value>,
    D: HasDefault<T::Value>,
    T::Value: Copy,
{
    pub fn optional<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> Optional<T::Value, D>
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let init = P::slot_init_view(common);
        let v = if P::is_set(common, || P::payload_is_empty(&self.value)) {
            Some(*self.value.as_ref(&init).expect("is_set implies initialized slot"))
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<T: VarintProtoType, const FIELD: u32, D> SingularVarintField<T, super::field_presence::Implicit, FIELD, D>
where
    T::Value: super::proto_zero::ProtoZero,
{
    /// Raw stored value for [`Implicit`] fields (always initialized).
    #[inline]
    pub fn value(&self) -> T::Value {
        *self
            .value
            .as_ref(&AlwaysInitialized)
            .expect("always-initialized slot")
    }
}

impl<T: VarintProtoType, const FIELD: u32, D> SingularVarintField<T, super::field_presence::Oneof, FIELD, D>
where
    T::Value: super::proto_zero::ProtoZero,
{
    /// Raw stored value for [`Oneof`] variants (always initialized).
    #[inline]
    pub fn value(&self) -> T::Value {
        *self
            .value
            .as_ref(&AlwaysInitialized)
            .expect("always-initialized slot")
    }

    /// Mutable access for [`Oneof`] variants — slot is always-initialized `T::Value`.
    #[inline]
    pub fn value_mut(&mut self) -> &mut T::Value {
        &mut self.value
    }
}

impl<T: VarintProtoType, P: FieldPresence, const FIELD: u32, D> Default
    for SingularVarintField<T, P, FIELD, D>
where
    P::ValueSlot<T::Value>: ValueSlot<T::Value>,
{
    fn default() -> Self {
        Self {
            value: ValueSlot::new(),
            _marker: PhantomData,
        }
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

pub struct SingularVarintFieldMut<
    'f,
    'c,
    T: VarintProtoType,
    P: FieldPresence,
    const FIELD: u32,
    D,
    Pb: PresenceBits,
    A: Allocator,
> {
    field: &'f mut SingularVarintField<T, P, FIELD, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<
        'f,
        'c,
        T: VarintProtoType,
        P: FieldPresence,
        const FIELD: u32,
        D,
        Pb: PresenceBits,
        A: Allocator,
    > SingularVarintFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T::Value>: ValueSlot<T::Value>,
{
    #[inline]
    fn new(
        field: &'f mut SingularVarintField<T, P, FIELD, D>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    #[inline]
    pub fn value_mut(self) -> &'f mut T::Value {
        let mut init = P::slot_init_mut(self.common);
        self.field.value.as_mut(&mut init)
    }

    #[inline]
    pub fn set(self, v: T::Value) {
        let mut init = P::slot_init_mut(self.common);
        self.field.value.set(&mut init, v);
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        let mut init = P::slot_init_mut(self.common);
        self.field
            .value
            .set(&mut init, T::decode_wire(raw)?);
        Ok(())
    }

    /// Resets the value slot and clears explicit presence when applicable.
    ///
    /// For [`Implicit`](super::field_presence::Implicit) fields this omits the field on
    /// the wire (equivalent to assigning the type-zero).
    pub fn clear(self) {
        let mut init = P::slot_init_mut(self.common);
        self.field.value.clear(&mut init);
    }
}

impl<
        'f,
        'c,
        T: VarintProtoType,
        P: FieldPresence,
        const FIELD: u32,
        D,
        Pb: PresenceBits,
        A: Allocator,
    > SingularVarintFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>
where
    P::ValueSlot<T::Value>: ValueSlot<T::Value>,
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
        T::Value: Copy,
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
        let mut init = P::slot_init_mut(self.common);
        self.field
            .value
            .set(&mut init, T::decode_wire(raw)?);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type SingularVarint<T, P, const FIELD: u32, D = ProtoDefault> = SingularVarintField<T, P, FIELD, D>;

pub type ImplicitVarintField<T, const FIELD: u32> =
    SingularVarintField<T, super::field_presence::Implicit, FIELD>;
pub type OneofVarintField<T, const FIELD: u32> =
    SingularVarintField<T, super::field_presence::Oneof, FIELD>;
pub type ExplicitVarintField<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    SingularVarintField<T, super::field_presence::Explicit<BIT>, FIELD, D>;
pub type LegacyRequiredVarintField<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    SingularVarintField<T, super::field_presence::LegacyRequired<BIT>, FIELD, D>;

pub type ImplicitVarint<T, const FIELD: u32> = ImplicitVarintField<T, FIELD>;
pub type ExplicitVarint<T, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitVarintField<T, BIT, FIELD, D>;

pub type ImplicitInt32<const FIELD: u32> = ImplicitVarintField<varint::ProtoInt32, FIELD>;
pub type ExplicitInt32<const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitVarintField<varint::ProtoInt32, BIT, FIELD, D>;
pub type ImplicitEnum<E, const FIELD: u32> = ImplicitVarintField<varint::ProtoEnum<E>, FIELD>;
pub type ExplicitEnum<E, const BIT: usize, const FIELD: u32, D = ProtoDefault> =
    ExplicitVarintField<varint::ProtoEnum<E>, BIT, FIELD, D>;
