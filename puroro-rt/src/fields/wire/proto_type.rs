//! Unified wire/storage semantics for singular (non-repeated) field type markers.
//!
//! Each implementor is a **protobuf type marker** (`ProtoInt32`, `ProtoBool`,
//! [`ProtoMessage`](super::proto_message::ProtoMessage), …). The physical field
//! slot is the associated [`Slot`](ProtoType::Slot) (`Self` for addressable
//! wrappers and bit-packed [`ProtoBool`]; [`UnmanagedBox`] for messages).
//!
//! IMPLICIT omit uses [`is_proto_empty`](ProtoType::is_proto_empty) on the
//! type marker (slot [`ProtoEmpty`](crate::fields::shared::ProtoEmpty) for
//! payload-bearing types; bit read for [`ProtoBool`]). Wire `encoded_len` /
//! `encode` stay on the marker because multiple markers can share the same
//! `Ref` type (e.g. [`ProtoInt32`] and [`ProtoSint32`] both use `i32`).
//!
//! [`merge`](ProtoType::merge) owns replace vs merge-into semantics: the default
//! is decode-then-write (last wins); nested messages override to recursive merge.
//!
//! [`VarintProtoType`](super::varint::VarintProtoType) and
//! [`LenProtoType`](super::len::LenProtoType) remain for **repeated** fields,
//! which keep storing the inner [`VarintProtoType::Value`] /
//! [`LenProtoType::Storage`] so public slices stay `&[i32]` / `&[UnmanagedString]`.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::ops::{Deref, DerefMut};

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::decode;
use crate::encode;
use crate::fields::shared::{
    DefaultIn, MessageCommon, PresenceBits, ProtoEmpty,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};

use super::len::{LenProtoType, ProtoBytes, ProtoString};
use super::varint::{ProtoBool, VarintProtoType};

/// Wire + accessor semantics for a singular (non-repeated) protobuf type.
///
/// The implementor is the **type marker** (not necessarily what sits in the
/// field struct). Physical storage is [`Slot`](Self::Slot):
/// - addressable wrappers: `Slot = Self` (thin payload in the field)
/// - [`ProtoBool`]: `Slot = Self` (ZST; logical `bool` in [`MessageCommon`] bitvec)
/// - nested messages: `Slot = UnmanagedBox<M, A>` via [`ProtoMessage`](super::proto_message::ProtoMessage)
///
/// Interim: [`ProtoBool`] still carries `VALUE_BIT` as a const generic for
/// codegen stability; a future cleanup should move that index to the field /
/// layout side so the type marker is bit-index-free.
pub trait ProtoType: Sized {
    /// Allocator type retained by the marker and its physical slot.
    type Alloc: Allocator + Clone;

    /// Physical value stored in the singular field slot (excluding
    /// [`MessageCommon`] bits).
    type Slot: AddressableSlot<SlotAlloc = Self::Alloc> + DefaultIn<Alloc = Self::Alloc>;

    /// Borrowed / by-value view returned by getters (`i32`, `&str`, `bool`, …).
    type Ref<'a>
    where
        Self: 'a;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, `StringGuard`,
    /// bit handle, …).
    type Mut<'a>: DerefMut
    where
        Self: 'a;

    /// Value accepted by `set` / produced by [`decode`](Self::decode).
    type Written;

    /// Expected wire type for a singular occurrence of this field.
    const WIRE_TYPE: WireType;

    /// `true` when the field holds protobuf empty / type-zero (IMPLICIT omit).
    ///
    /// Addressable payload slots delegate to [`ProtoEmpty`]; bit-packed
    /// [`ProtoBool`] reads the value bit from `common`.
    fn is_proto_empty<Pb: PresenceBits>(
        slot: &Self::Slot,
        common: &MessageCommon<Pb, Self::Alloc>,
    ) -> bool;

    /// Reads the logical getter view from the slot and/or `common`.
    fn get<'a, Pb: PresenceBits>(
        slot: &'a Self::Slot,
        common: &'a MessageCommon<Pb, Self::Alloc>,
    ) -> Self::Ref<'a>;

    /// Ensures the slot is present and returns a mutable accessor handle.
    ///
    /// Heap-backed mutators clone the allocator from `common.alloc`.
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, Self::Alloc>,
    ) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a;

    /// Writes `value`, ensuring slot presence when applicable.
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, Self::Alloc>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits;

    /// Clears the logical value and slot presence / payload.
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, Self::Alloc>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits;

    /// Wire byte length of one tagged occurrence for `value`.
    ///
    /// Kept on the type marker so int32 vs sint32 can share `Ref = i32` with
    /// different wire encodings.
    fn encoded_len<'a>(value: Self::Ref<'a>, field: u32) -> usize
    where
        Self: 'a;

    /// Encodes one tagged occurrence for `value`.
    fn encode<'a, B: BufMut>(value: Self::Ref<'a>, field: u32, buf: &mut B)
    where
        Self: 'a;

    /// Decodes one occurrence after the tag has been read (`wire_type` checked here).
    fn decode<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: Self::Alloc,
    ) -> Result<Self::Written, DecodeError>;

    /// Merges one wire occurrence into the slot after the tag has been read.
    ///
    /// Default: decode then [`write`](Self::write) (last wins). Closed-enum
    /// unknowns are parked in `common.unknown_fields`. Nested messages override
    /// this for recursive merge-into.
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, Self::Alloc>,
        wire_type: WireType,
        buf: &mut B,
        field: u32,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf,
    {
        match Self::decode(wire_type, buf, common.alloc.clone()) {
            Ok(new) => {
                Self::write(slot, init, common, new);
                Ok(())
            }
            // Closed enum, unrecognized value: park in unknown fields (not a
            // decode failure). See https://protobuf.dev/programming-guides/enum/
            Err(DecodeError::UnknownClosedEnum { raw }) => {
                decode::save_unknown_varint_field(
                    field,
                    raw,
                    &mut common.unknown_fields,
                    common.alloc.clone(),
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

// ---------------------------------------------------------------------------
// Addressable varint wrappers (Slot = Self): numerics + ProtoEnum
// ---------------------------------------------------------------------------

impl<T> ProtoType for T
where
    T: VarintProtoType + AddressableSlot + From<T::Value> + ProtoEmpty,
    T: Deref<Target = T::Value> + DerefMut,
{
    type Alloc = T::SlotAlloc;
    type Slot = Self;
    type Ref<'a>
        = T::Value
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut T::Value
    where
        Self: 'a;
    type Written = Self;
    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(
        slot: &Self::Slot,
        _common: &MessageCommon<Pb, Self::Alloc>,
    ) -> bool {
        slot.is_proto_empty()
    }

    #[inline]
    fn get<'a, Pb: PresenceBits>(
        slot: &'a Self::Slot,
        _common: &'a MessageCommon<Pb, Self::Alloc>,
    ) -> Self::Ref<'a> {
        *Deref::deref(slot)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, Self::Alloc>,
    ) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        DerefMut::deref_mut(ValueSlot::with_mut(slot, init, common).get_mut())
    }

    #[inline]
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, Self::Alloc>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, Self::Alloc>,
    )
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len<'a>(value: Self::Ref<'a>, field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_varint_field(field, T::encode_wire(value))
    }

    #[inline]
    fn encode<'a, B: BufMut>(value: Self::Ref<'a>, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        encode::encode_varint_field(field, T::encode_wire(value), buf);
    }

    #[inline]
    fn decode<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: Self::Alloc,
    ) -> Result<Self::Written, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        Ok(Self::from(T::decode_wire(raw)?))
    }
}

// ---------------------------------------------------------------------------
// LEN wrappers (Slot = Self)
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> ProtoType for ProtoString<A> {
    type Alloc = A;
    type Slot = Self;
    type Ref<'a>
        = &'a str
    where
        Self: 'a;
    type Mut<'a>
        = <Self as LenProtoType>::Mut<'a>
    where
        Self: 'a;
    type Written = Self;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(
        slot: &Self::Slot,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        slot.is_proto_empty()
    }

    #[inline]
    fn get<'a, Pb: PresenceBits>(
        slot: &'a Self::Slot,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a str {
        &slot.0
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        let alloc = common.alloc.clone();
        <Self as LenProtoType>::with_alloc(
            &mut ValueSlot::with_mut(slot, init, common).get_mut().0,
            alloc,
        )
    }

    #[inline]
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len<'a>(value: Self::Ref<'a>, field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_len_field(field, value.as_bytes().len())
    }

    #[inline]
    fn encode<'a, B: BufMut>(value: Self::Ref<'a>, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        encode::encode_len_field(field, value.as_bytes(), buf);
    }

    #[inline]
    fn decode<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Written, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        Ok(Self(<Self as LenProtoType>::decode(buf, alloc)?))
    }
}

impl<A: Allocator + Clone> ProtoType for ProtoBytes<A> {
    type Alloc = A;
    type Slot = Self;
    type Ref<'a>
        = &'a [u8]
    where
        Self: 'a;
    type Mut<'a>
        = <Self as LenProtoType>::Mut<'a>
    where
        Self: 'a;
    type Written = Self;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(
        slot: &Self::Slot,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        slot.is_proto_empty()
    }

    #[inline]
    fn get<'a, Pb: PresenceBits>(
        slot: &'a Self::Slot,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a [u8] {
        &slot.0
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        let alloc = common.alloc.clone();
        <Self as LenProtoType>::with_alloc(
            &mut ValueSlot::with_mut(slot, init, common).get_mut().0,
            alloc,
        )
    }

    #[inline]
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len<'a>(value: Self::Ref<'a>, field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_len_field(field, value.len())
    }

    #[inline]
    fn encode<'a, B: BufMut>(value: Self::Ref<'a>, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        encode::encode_len_field(field, value, buf);
    }

    #[inline]
    fn decode<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Written, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        Ok(Self(<Self as LenProtoType>::decode(buf, alloc)?))
    }
}

// ---------------------------------------------------------------------------
// Bit-packed bool (Slot = Self, ZST; value in MessageCommon)
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone, const VALUE_BIT: usize> ProtoType
    for ProtoBool<A, VALUE_BIT>
{
    type Alloc = A;
    /// ZST slot: presence/init layout only. Logical `bool` lives at `VALUE_BIT`
    /// in [`MessageCommon`].
    type Slot = Self;
    type Ref<'a>
        = bool
    where
        Self: 'a;
    type Mut<'a>
        = ::bitvec::ptr::BitRef<'a, ::bitvec::ptr::Mut, u8, ::bitvec::order::Lsb0>
    where
        Self: 'a;
    type Written = bool;
    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(
        _slot: &Self::Slot,
        common: &MessageCommon<Pb, A>,
    ) -> bool {
        !common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn get<'a, Pb: PresenceBits>(
        _slot: &'a Self::Slot,
        common: &'a MessageCommon<Pb, A>,
    ) -> bool {
        common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.bit_mut(VALUE_BIT)
    }

    #[inline]
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: bool,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.set_bit(VALUE_BIT, value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        common.set_bit(VALUE_BIT, false);
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len<'a>(value: bool, field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_varint_field(field, Self::encode_wire(value))
    }

    #[inline]
    fn encode<'a, B: BufMut>(value: bool, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        encode::encode_varint_field(field, Self::encode_wire(value), buf);
    }

    #[inline]
    fn decode<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: A,
    ) -> Result<bool, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        Self::decode_wire(raw)
    }
}