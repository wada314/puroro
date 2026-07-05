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
use super::field_presence::{ExplicitFieldPresence, FieldPresence, Implicit, Oneof};
use super::presence::PresenceBits;
use super::varint::{self, VarintProtoType};

/// Singular scalar on the wire as VARINT — parametrised by protobuf type `T`,
/// presence policy `P` ([`Implicit`] / [`Explicit`](super::field_presence::Explicit)
/// / [`LegacyRequired`](super::field_presence::LegacyRequired)), and proto field
/// number `FIELD`, and compile-time default marker `D` ([`HasDefault`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SingularVarintField<
    T: VarintProtoType,
    P: FieldPresence,
    const FIELD: u32,
    D = ProtoDefault,
> {
    value: T::Value,
    _marker: PhantomData<(P, D)>,
}

impl<T: VarintProtoType, P: FieldPresence, const FIELD: u32, D> SingularVarintField<T, P, FIELD, D> {
    /// Creates a field with the protobuf type-zero in the value slot.
    pub fn new() -> Self {
        Self {
            value: T::proto_zero(),
            _marker: PhantomData,
        }
    }

    /// Raw stored value (use for IMPLICIT public getters).
    #[inline]
    pub fn value(&self) -> T::Value {
        self.value
    }

    /// Presence-agnostic mutable access to the value slot.
    ///
    /// Regular message fields mutate through [`bind`](Self::bind) (to fold in the
    /// presence bit); [`Oneof`] variants use [`bind`](Self::bind) too, but the
    /// marker's `on_set` is a no-op because the enclosing `OneofSlot` tracks
    /// presence.
    #[inline]
    pub fn value_mut(&mut self) -> &mut T::Value {
        &mut self.value
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

    /// Resets the value slot to type-zero (does not touch the bitfield).
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = T::proto_zero();
    }

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let empty = self.value == T::proto_zero();
        if P::should_emit(common, empty) {
            encode::encoded_len_varint_field(FIELD, T::encode_wire(self.value))
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let empty = self.value == T::proto_zero();
        if P::should_emit(common, empty) {
            encode::encode_varint_field(FIELD, T::encode_wire(self.value), buf);
        }
    }
}

impl<T: VarintProtoType, const FIELD: u32, D> SingularVarintField<T, Implicit, FIELD, D>
where
    D: HasDefault<T::Value>,
    T::Value: Copy,
{
    /// `Optional` getter (`is_set` when the stored value is not [`VarintProtoType::proto_zero`]).
    pub fn optional<Pb, A>(&self, _common: &MessageCommon<Pb, A>) -> Optional<T::Value, D>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        let v = if self.value == T::proto_zero() {
            None
        } else {
            Some(self.value)
        };
        Optional::new(v)
    }

    /// Wire byte length without `MessageCommon` (IMPLICIT presence only).
    pub fn encoded_len_wire(&self) -> usize {
        let empty = self.value == T::proto_zero();
        if empty {
            0
        } else {
            encode::encoded_len_varint_field(FIELD, T::encode_wire(self.value))
        }
    }

    /// Encodes without `MessageCommon` (IMPLICIT presence only).
    pub fn encode_raw_wire<B: BufMut>(&self, buf: &mut B) {
        let empty = self.value == T::proto_zero();
        if !empty {
            encode::encode_varint_field(FIELD, T::encode_wire(self.value), buf);
        }
    }
}

impl<T: VarintProtoType, const FIELD: u32, D> SingularVarintField<T, Oneof, FIELD, D> {
    /// Wire byte length without `MessageCommon` (active oneof variant only).
    pub fn encoded_len_wire(&self) -> usize {
        encode::encoded_len_varint_field(FIELD, T::encode_wire(self.value))
    }

    /// Encodes without `MessageCommon` (active oneof variant only).
    pub fn encode_raw_wire<B: BufMut>(&self, buf: &mut B) {
        encode::encode_varint_field(FIELD, T::encode_wire(self.value), buf);
    }
}

impl<T: VarintProtoType, P: FieldPresence, const FIELD: u32, D> Default
    for SingularVarintField<T, P, FIELD, D>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: VarintProtoType, P: ExplicitFieldPresence, const FIELD: u32, D> SingularVarintField<T, P, FIELD, D> {
    #[inline]
    pub fn has<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> bool
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        common.is_present(P::BIT)
    }
}

impl<T: VarintProtoType, P: ExplicitFieldPresence, const FIELD: u32, D> SingularVarintField<T, P, FIELD, D>
where
    D: HasDefault<T::Value>,
    T::Value: Copy,
{
    pub fn optional<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> Optional<T::Value, D>
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let v = if common.is_present(P::BIT) {
            Some(self.value)
        } else {
            None
        };
        Optional::new(v)
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
        P::on_set(self.common);
        &mut self.field.value
    }

    #[inline]
    pub fn set(self, v: T::Value) {
        P::on_set(self.common);
        self.field.value = v;
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        P::on_set(self.common);
        self.field.value = T::decode_wire(raw)?;
        Ok(())
    }
}

impl<
        'f,
        'c,
        T: VarintProtoType,
        P: ExplicitFieldPresence,
        const FIELD: u32,
        D,
        Pb: PresenceBits,
        A: Allocator,
    > SingularVarintFieldMut<'f, 'c, T, P, FIELD, D, Pb, A>
{
    pub fn clear(self) {
        P::on_clear(self.common);
        self.field.value = T::proto_zero();
    }

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
        P::on_set(self.common);
        self.field.value = T::decode_wire(raw)?;
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
