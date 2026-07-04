//! Singular varint field wrapper — generic over wire type and presence policy.
//!
//! Fixed-width scalars will follow the same `Singular*Field<T, P>` pattern with
//! [`Fixed32ProtoType`](super::fixed32::Fixed32ProtoType) /
//! [`Fixed64ProtoType`](super::fixed64::Fixed64ProtoType).

use ::core::marker::PhantomData;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::decode;
use crate::encode;
use crate::error::DecodeError;
use crate::optional::{HasDefault, Optional};
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::field_presence::{ExplicitFieldPresence, FieldPresence, Implicit};
use super::presence::PresenceBits;
use super::varint::{self, VarintProtoType};

/// Singular scalar on the wire as VARINT — parametrised by protobuf type `T` and
/// presence policy `P` ([`Implicit`] / [`Explicit`](super::field_presence::Explicit)
/// / [`LegacyRequired`](super::field_presence::LegacyRequired)).
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

    /// Presence-agnostic mutable access to the value slot.
    ///
    /// Regular message fields mutate through [`bind`](Self::bind) (to fold in the
    /// presence bit); this bypass exists for oneof variants, whose presence is
    /// tracked by the enclosing `OneofSlot` rather than a bit.
    #[inline]
    pub fn value_mut(&mut self) -> &mut T::Value {
        &mut self.value
    }

    /// Binds this field to its message `common` state (presence), producing a
    /// short-lived [`SingularVarintFieldMut`] view that carries the whole
    /// mutation context.
    ///
    /// This is the entry point for every mutation (`value_mut` / `set` /
    /// `merge` / `clear` / `merge_closed`): generated accessors call
    /// `field.bind(&mut common).…()` instead of threading `common` through
    /// each method. Scalars store no allocator, but the view still carries
    /// `common` for presence and (closed-enum) unknown-field handling.
    #[inline]
    pub fn bind<'f, 'c, Pb: PresenceBits, A: Allocator>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularVarintFieldMut<'f, 'c, T, P, Pb, A> {
        SingularVarintFieldMut::new(self, common)
    }

    /// Resets the value slot to type-zero (does not touch the bitfield).
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = T::proto_zero();
    }

    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>, field: u32) -> usize
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let empty = self.value == T::proto_zero();
        if P::should_emit(common, empty) {
            encode::encoded_len_varint_field(field, T::encode_wire(self.value))
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, A, B: BufMut>(
        &self,
        common: &MessageCommon<Pb, A>,
        field: u32,
        buf: &mut B,
    ) where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        let empty = self.value == T::proto_zero();
        if P::should_emit(common, empty) {
            encode::encode_varint_field(field, T::encode_wire(self.value), buf);
        }
    }
}

impl<T: VarintProtoType> SingularVarintField<T, Implicit> {
    /// Binds an `Implicit`-presence **oneof variant** field for mutation,
    /// yielding the same [`SingularVarintFieldMut`] view as [`bind`](Self::bind)
    /// so generated oneof code merges through
    /// `field.bind_oneof(common).merge(…)` — the field's own bind idiom — rather
    /// than a bespoke helper on the oneof enum.
    ///
    /// A oneof carries no presence bit (the enclosing `OneofSlot` tracks which
    /// variant is set). Restricting the method to `Implicit` keeps no
    /// `Explicit` bitfield from being touched.
    #[inline]
    pub fn bind_oneof<'f, 'c, Pb: PresenceBits, A: Allocator>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularVarintFieldMut<'f, 'c, T, Implicit, Pb, A> {
        self.bind(common)
    }
}

impl<T: VarintProtoType, P: FieldPresence> Default for SingularVarintField<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: VarintProtoType, P: ExplicitFieldPresence> SingularVarintField<T, P> {
    #[inline]
    pub fn has<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> bool
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
    {
        common.is_present(P::BIT)
    }

    pub fn optional<Pb, A, D>(
        &self,
        common: &MessageCommon<Pb, A>,
        default: D,
    ) -> Optional<T::Value, D>
    where
        Pb: PresenceBits,
        A: ::allocator_api2::alloc::Allocator,
        D: HasDefault<T::Value>,
        T::Value: Copy,
    {
        let v = if common.is_present(P::BIT) {
            Some(self.value)
        } else {
            None
        };
        Optional::new(v, default)
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a singular varint field to its message common state,
/// produced by [`SingularVarintField::bind`].
///
/// It bundles the value slot with the presence/allocator context so a generated
/// accessor can express a whole mutation as a single call (mirrors the
/// `SingularLenField` view). Scalars are inline (no heap payload), so the value
/// itself is `Copy`; the view exists purely to fold presence — and, for closed
/// enums, unknown-field capture — into one call. Two lifetimes keep the `&mut`
/// returned by `value_mut` tied to the field slot only (`'f`); the `common`
/// borrow (`'c`) is released as the method returns. Every method consumes the
/// view, so a fresh `bind` precedes each mutation.
pub struct SingularVarintFieldMut<
    'f,
    'c,
    T: VarintProtoType,
    P: FieldPresence,
    Pb: PresenceBits,
    A: Allocator,
> {
    field: &'f mut SingularVarintField<T, P>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, T: VarintProtoType, P: FieldPresence, Pb: PresenceBits, A: Allocator>
    SingularVarintFieldMut<'f, 'c, T, P, Pb, A>
{
    #[inline]
    fn new(
        field: &'f mut SingularVarintField<T, P>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Marks presence (per `P`) and returns a mutable reference to the value.
    #[inline]
    pub fn value_mut(self) -> &'f mut T::Value {
        P::on_set(self.common);
        &mut self.field.value
    }

    /// Stores `v`, applying the presence policy (`on_set` for EXPLICIT).
    #[inline]
    pub fn set(self, v: T::Value) {
        P::on_set(self.common);
        self.field.value = v;
    }

    /// Merges one VARINT occurrence and marks presence (per `P`).
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

impl<'f, 'c, T: VarintProtoType, P: ExplicitFieldPresence, Pb: PresenceBits, A: Allocator>
    SingularVarintFieldMut<'f, 'c, T, P, Pb, A>
{
    /// Clears presence and resets the value slot to type-zero.
    pub fn clear(self) {
        P::on_clear(self.common);
        self.field.value = T::proto_zero();
    }

    /// Merges a closed-enum occurrence; unknown values go to
    /// `common.unknown_fields`. `field` is the proto field number.
    pub fn merge_closed<B: Buf>(
        self,
        field: u32,
        wire_type: WireType,
        buf: &mut B,
        is_known: impl FnOnce(T::Value) -> bool,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
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
                &mut self.common.unknown_fields,
                self.common.alloc.clone(),
            );
            return Ok(());
        }
        P::on_set(self.common);
        self.field.value = value;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type SingularVarint<T, P> = SingularVarintField<T, P>;

pub type ImplicitVarintField<T> = SingularVarintField<T, super::field_presence::Implicit>;
pub type ExplicitVarintField<T, const BIT: usize> =
    SingularVarintField<T, super::field_presence::Explicit<BIT>>;
pub type LegacyRequiredVarintField<T, const BIT: usize> =
    SingularVarintField<T, super::field_presence::LegacyRequired<BIT>>;

pub type ImplicitVarint<T> = ImplicitVarintField<T>;
pub type ExplicitVarint<T, const BIT: usize> = ExplicitVarintField<T, BIT>;

pub type ImplicitInt32 = ImplicitVarintField<varint::ProtoInt32>;
pub type ExplicitInt32<const BIT: usize> = ExplicitVarintField<varint::ProtoInt32, BIT>;
pub type ImplicitEnum = ImplicitVarintField<varint::ProtoEnum>;
pub type ExplicitEnum<const BIT: usize> = ExplicitVarintField<varint::ProtoEnum, BIT>;
