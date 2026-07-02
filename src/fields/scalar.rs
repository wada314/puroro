//! Singular varint field wrapper — generic over wire type and presence policy.
//!
//! Fixed-width scalars will follow the same `Singular*Field<T, P>` pattern with
//! [`Fixed32ProtoType`](super::fixed32::Fixed32ProtoType) /
//! [`Fixed64ProtoType`](super::fixed64::Fixed64ProtoType).

use ::core::marker::PhantomData;

use ::bytes::{Buf, BufMut};

use crate::decode;
use crate::encode;
use crate::error::DecodeError;
use crate::optional::{HasDefault, Optional};
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::field_presence::{ExplicitFieldPresence, FieldPresence};
use super::presence::PresenceBits;
use super::varint::{self, VarintProtoType};

/// Singular scalar on the wire as VARINT — parametrised by protobuf type `T` and
/// presence policy `P` ([`Implicit`](super::field_presence::Implicit) /
/// [`Explicit`](super::field_presence::Explicit)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SingularVarintField<T: VarintProtoType, P: FieldPresence> {
    value: T::Value,
    _presence: PhantomData<P>,
}

impl<T: VarintProtoType, P: FieldPresence> SingularVarintField<T, P> {
    /// Creates a field with the protobuf type-zero in the value slot.
    pub fn new() -> Self {
        Self {
            value: T::proto_zero(),
            _presence: PhantomData,
        }
    }

    /// Raw stored value (use for IMPLICIT public getters).
    #[inline]
    pub fn value(&self) -> T::Value {
        self.value
    }

    /// Stores `v`, applying the presence policy (`on_set` for EXPLICIT).
    pub fn set<Pb, A>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        bit: usize,
        v: T::Value,
    ) where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        P::on_set(common, bit);
        self.value = v;
    }

    /// Returns a mutable reference to the value, applying the presence policy
    /// (`on_set` for EXPLICIT). Backs generated `*_mut` accessors.
    pub fn value_mut<Pb, A>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        bit: usize,
    ) -> &mut T::Value
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        P::on_set(common, bit);
        &mut self.value
    }

    /// Resets the value slot to type-zero (does not touch the bitfield).
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = T::proto_zero();
    }

    pub fn encoded_len<Pb, A>(
        &self,
        common: &MessageCommon<Pb, A>,
        field: u32,
        bit: usize,
    ) -> usize
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let empty = self.value == T::proto_zero();
        if P::should_emit(common, bit, empty) {
            encode::encoded_len_varint_field(field, T::encode_wire(self.value))
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(
        &self,
        common: &MessageCommon<Pb, A>,
        field: u32,
        bit: usize,
        buf: &mut B,
    ) where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let empty = self.value == T::proto_zero();
        if P::should_emit(common, bit, empty) {
            encode::encode_varint_field(field, T::encode_wire(self.value), buf);
        }
    }

    pub fn merge<Pb, A, B: Buf>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        bit: usize,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        P::on_set(common, bit);
        self.value = T::decode_wire(raw)?;
        Ok(())
    }
}

impl<T: VarintProtoType, P: FieldPresence> Default for SingularVarintField<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: VarintProtoType, P: ExplicitFieldPresence> SingularVarintField<T, P> {
    #[inline]
    pub fn has<Pb, A>(&self, common: &MessageCommon<Pb, A>, bit: usize) -> bool
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        common.is_present(bit)
    }

    pub fn optional<Pb, A, D>(
        &self,
        common: &MessageCommon<Pb, A>,
        bit: usize,
        default: D,
    ) -> Optional<T::Value, D>
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
        D: HasDefault<T::Value>,
        T::Value: Copy,
    {
        let v = if common.is_present(bit) {
            Some(self.value)
        } else {
            None
        };
        Optional::new(v, default)
    }

    pub fn clear<Pb, A>(&mut self, common: &mut MessageCommon<Pb, A>, bit: usize)
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        P::on_clear(common, bit);
        self.value = T::proto_zero();
    }

    /// Merges a closed-enum occurrence; unknown values go to `common.unknown_fields`.
    pub fn merge_closed<Pb, A, B: Buf>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        field: u32,
        bit: usize,
        wire_type: WireType,
        buf: &mut B,
        is_known: impl FnOnce(T::Value) -> bool,
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
        T::Value: Copy,
    {
        if wire_type != varint::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        let value = T::decode_wire(raw)?;
        if !is_known(value) {
            decode::save_unknown_varint_field(
                field,
                raw,
                &mut common.unknown_fields,
                &common.alloc,
            );
            return Ok(());
        }
        P::on_set(common, bit);
        self.value = value;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type SingularVarint<T, P> = SingularVarintField<T, P>;

pub type ImplicitVarintField<T> = SingularVarintField<T, super::field_presence::Implicit>;
pub type ExplicitVarintField<T> = SingularVarintField<T, super::field_presence::Explicit>;

pub type ImplicitVarint<T> = ImplicitVarintField<T>;
pub type ExplicitVarint<T> = ExplicitVarintField<T>;

pub type ImplicitInt32 = ImplicitVarintField<varint::ProtoInt32>;
pub type ExplicitInt32 = ExplicitVarintField<varint::ProtoInt32>;
pub type ImplicitEnum = ImplicitVarintField<varint::ProtoEnum>;
pub type ExplicitEnum = ExplicitVarintField<varint::ProtoEnum>;
