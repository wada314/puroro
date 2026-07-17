//! Wire semantics for singular (non-repeated) field type markers.
//!
//! Each implementor is a **protobuf type marker** (`ProtoInt32`, `ProtoBool`,
//! [`ProtoMessage`](super::proto_message::ProtoMessage), …). The physical field
//! slot is the associated [`Slot`](ProtoType::Slot).
//!
//! **Storage access** (get / write / clear / merge) lives on
//! [`PayloadAccess`] for inline payloads, or on
//! [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout)
//! (`BitPacked`) for singular / oneof `bool`. [`SingularField`](crate::fields::singular::field::SingularField)
//! always goes through `ValueLayout`.
//!
//! Repeated fields use [`RepeatedItems`](super::repeated_items::RepeatedItems)
//! (`Element` storage). [`VarintProtoType`](super::varint::VarintProtoType) and
//! [`LenProtoType`](super::len::LenProtoType) remain as wire/storage helpers.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
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

/// Wire + type-identity for a singular protobuf type marker.
///
/// Physical storage is [`Slot`](Self::Slot):
/// - addressable wrappers: `Slot = Self`
/// - [`ProtoBool`]: `Slot = Self` (ZST; logical `bool` via [`BitPacked`](crate::fields::shared::value_layout::BitPacked))
/// - nested messages: `Slot = UnmanagedBox<M, A>` via [`ProtoMessage`](super::proto_message::ProtoMessage)
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

    /// Wire byte length of one tagged occurrence for `value`.
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
}

/// Inline payload access for markers whose value lives in [`ProtoType::Slot`].
///
/// Not implemented for [`ProtoBool`] — use
/// [`BitPacked`](crate::fields::shared::value_layout::BitPacked) instead.
pub trait PayloadAccess: ProtoType {
    /// `true` when the field holds protobuf empty / type-zero (IMPLICIT omit).
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

impl<T> PayloadAccess for T
where
    T: VarintProtoType + AddressableSlot + From<T::Value> + ProtoEmpty,
    T: Deref<Target = T::Value> + DerefMut,
{
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
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, Self::Alloc>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
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

impl<A: Allocator + Clone> PayloadAccess for ProtoString<A> {
    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(slot: &Self::Slot, _common: &MessageCommon<Pb, A>) -> bool {
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

impl<A: Allocator + Clone> PayloadAccess for ProtoBytes<A> {
    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(slot: &Self::Slot, _common: &MessageCommon<Pb, A>) -> bool {
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
}

// ---------------------------------------------------------------------------
// Bit-packed bool marker (Slot = Self, ZST; value via BitPacked layout)
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> ProtoType for ProtoBool<A> {
    type Alloc = A;
    /// ZST slot: presence/init layout only. Logical `bool` lives in
    /// [`MessageCommon`] at the index carried by [`BitPacked`](crate::fields::shared::value_layout::BitPacked).
    type Slot = Self;
    type Ref<'a>
        = bool
    where
        Self: 'a;
    type Mut<'a>
        = BitRef<'a, Mut, u8, Lsb0>
    where
        Self: 'a;
    type Written = bool;
    const WIRE_TYPE: WireType = WireType::Varint;

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
    fn decode<B: Buf>(wire_type: WireType, buf: &mut B, _alloc: A) -> Result<bool, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        Self::decode_wire(raw)
    }
}
