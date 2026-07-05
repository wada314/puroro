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
            value: ValueSlot::new_in(T::proto_zero()),
            _marker: PhantomData,
        }
    }

    /// Raw stored value (use for IMPLICIT public getters).
    ///
    /// # Safety
    ///
    /// For explicit-presence fields, the slot may be uninitialized when the
    /// presence bit is clear — callers must use [`optional`](Self::optional) or
    /// check `has_*` first.
    #[inline]
    pub fn value(&self) -> T::Value {
        *unsafe { ValueSlot::read_unchecked(&self.value) }
    }

    /// Presence-agnostic mutable access to the value slot.
    ///
    /// Regular message fields mutate through [`bind`](Self::bind) (to fold in the
    /// presence bit); [`Oneof`] variants use [`bind`](Self::bind) too, but the
    /// marker's `on_set` is a no-op because the enclosing `OneofSlot` tracks
    /// presence.
    #[inline]
    pub fn value_mut(&mut self) -> &mut T::Value {
        unsafe { ValueSlot::mut_unchecked(&mut self.value) }
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

    /// Resets the value slot (does not touch the bitfield).
    ///
    /// `was_set` must reflect whether the field was present before clearing
    /// (explicit / legacy required); implicit callers may pass any value.
    #[inline]
    pub fn clear_value(&mut self, was_set: bool) {
        ValueSlot::clear(&mut self.value, T::proto_zero(), was_set);
    }

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let proto_zero = T::proto_zero();
        if P::should_emit(common, || P::payload_is_empty(&self.value, proto_zero)) {
            let v = *unsafe { ValueSlot::read_unchecked(&self.value) };
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
        let proto_zero = T::proto_zero();
        if P::should_emit(common, || P::payload_is_empty(&self.value, proto_zero)) {
            let v = *unsafe { ValueSlot::read_unchecked(&self.value) };
            encode::encode_varint_field(FIELD, T::encode_wire(v), buf);
        }
    }

    #[inline]
    pub fn has<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> bool
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let proto_zero = T::proto_zero();
        P::is_set(common, || P::payload_is_empty(&self.value, proto_zero))
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
        let proto_zero = T::proto_zero();
        let v = if P::is_set(common, || P::payload_is_empty(&self.value, proto_zero)) {
            Some(*unsafe { ValueSlot::read_unchecked(&self.value) })
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<T: VarintProtoType, P: FieldPresence, const FIELD: u32, D> Default
    for SingularVarintField<T, P, FIELD, D>
where
    P::ValueSlot<T::Value>: ValueSlot<T::Value>,
{
    fn default() -> Self {
        Self {
            value: ValueSlot::new_in(T::proto_zero()),
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
        let proto_zero = T::proto_zero();
        let replacing =
            P::is_set(self.common, || P::payload_is_empty(&self.field.value, proto_zero));
        P::on_set(self.common);
        if !replacing {
            ValueSlot::write(&mut self.field.value, proto_zero, false);
        }
        unsafe { ValueSlot::mut_unchecked(&mut self.field.value) }
    }

    #[inline]
    pub fn set(self, v: T::Value) {
        let proto_zero = T::proto_zero();
        let replacing =
            P::is_set(self.common, || P::payload_is_empty(&self.field.value, proto_zero));
        P::on_set(self.common);
        ValueSlot::write(&mut self.field.value, v, replacing);
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        let proto_zero = T::proto_zero();
        let replacing =
            P::is_set(self.common, || P::payload_is_empty(&self.field.value, proto_zero));
        P::on_set(self.common);
        ValueSlot::write(&mut self.field.value, T::decode_wire(raw)?, replacing);
        Ok(())
    }

    /// Resets the value slot and clears explicit presence when applicable.
    ///
    /// For [`Implicit`](super::field_presence::Implicit) fields this omits the field on
    /// the wire (equivalent to assigning the type-zero); `on_clear` is a no-op.
    pub fn clear(self) {
        let proto_zero = T::proto_zero();
        let was_set =
            P::is_set(self.common, || P::payload_is_empty(&self.field.value, proto_zero));
        P::on_clear(self.common);
        self.field.clear_value(was_set);
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
        let proto_zero = T::proto_zero();
        let replacing =
            P::is_set(self.common, || P::payload_is_empty(&self.field.value, proto_zero));
        P::on_set(self.common);
        ValueSlot::write(&mut self.field.value, T::decode_wire(raw)?, replacing);
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
