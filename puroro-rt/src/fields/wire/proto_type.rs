//! Wire semantics for singular (non-repeated) field type markers.
//!
//! Markers (`ProtoInt32`, `ProtoBool`, [`ProtoMessage`](super::proto_message::ProtoMessage), …)
//! are allocator-free. Physical storage / views are GATs parametrised by `A`.
//!
//! **Storage access** (get / write / clear / merge) lives on [`PayloadAccess`]
//! for inline payloads, or on
//! [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout)
//! (`BitPacked`) for singular / oneof `bool`. Singular wire decode is
//! **merge-into only** (`PayloadAccess::merge` / `BitPacked::merge`); there is
//! no `ProtoType::decode → Written`. [`SingularField`](crate::fields::singular::field::SingularField)
//! always goes through `ValueLayout`.
//!
//! Repeated fields use [`RepeatedElement`](super::repeated_element::RepeatedElement)
//! (`Element` storage). Copy-inline numerics / enums share
//! [`NumericalType`](super::numerical::NumericalType).

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::bytes::{Buf, BufMut};
use ::core::ops::DerefMut;
use ::unmanaged::string::StringGuard;
use ::unmanaged::vec::VecGuard;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use ::puroro::DecodeError;
use ::puroro::WireType;

use ::unmanaged::DeallocateIn;

use crate::decode;
use crate::encode;
use crate::fields::shared::{
    DefaultIn, MessageCommon, PresenceBits, ProtoEmpty,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};

use super::len::{ProtoBytes, ProtoString};
use super::numerical::NumericalType;
use super::varint::ProtoBool;

/// Wire + type-identity for a singular protobuf type marker.
///
/// Physical storage is the GAT [`Slot`](Self::Slot):
/// - numerics / enums: bare `i32` / `E` / …
/// - [`ProtoBool`]: `()` (ZST; logical `bool` via [`BitPacked`](crate::fields::shared::value_layout::BitPacked))
/// - string / bytes: `UnmanagedString` / `UnmanagedVec`
/// - nested messages: `UnmanagedBox<M, A>` via [`ProtoMessage`](super::proto_message::ProtoMessage)
pub trait ProtoType: Sized {
    /// Physical value stored in the singular field slot (excluding
    /// [`MessageCommon`] bits).
    ///
    /// Call sites ([`SingularField`](crate::fields::singular::field::SingularField))
    /// require `Slot<A>: AddressableSlot<SlotAlloc = A> + DefaultIn<Alloc = A>`.
    /// Nested messages use `UnmanagedBox<M, A>`; call sites require
    /// `M: Message<Alloc = A> + unmanaged::DeallocateIn<A>` via `DefaultIn` /
    /// `DeallocateIn` on that box.
    type Slot<A: Allocator + Clone>;

    /// Borrowed / by-value view returned by getters (`i32`, `&str`, `bool`, …).
    type Ref<'a, A: Allocator + Clone>
    where
        Self: 'a,
        A: 'a;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, `StringGuard`,
    /// bit handle, …).
    type Mut<'a, A: Allocator + Clone>: DerefMut
    where
        Self: 'a,
        A: 'a;

    /// Value accepted by [`PayloadAccess::write`] / field `set`.
    type Written<A: Allocator + Clone>;

    /// Wire byte length of one tagged occurrence for `value`.
    fn encoded_len<'a, A: Allocator + Clone + 'a>(value: Self::Ref<'a, A>, field: u32) -> usize
    where
        Self: 'a;

    /// Encodes one tagged occurrence for `value`.
    fn encode<'a, A: Allocator + Clone + 'a, B: BufMut>(
        value: Self::Ref<'a, A>,
        field: u32,
        buf: &mut B,
    ) where
        Self: 'a;
}

/// Inline payload access for markers whose value lives in [`ProtoType::Slot`].
///
/// Not implemented for [`ProtoBool`] — use
/// [`BitPacked`](crate::fields::shared::value_layout::BitPacked) instead.
pub trait PayloadAccess: ProtoType {
    /// `true` when the field holds protobuf empty / type-zero (IMPLICIT omit).
    fn is_proto_empty<A: Allocator + Clone, Pb: PresenceBits>(
        slot: &Self::Slot<A>,
        common: &MessageCommon<Pb, A>,
    ) -> bool;

    /// Reads the logical getter view from the slot and/or `common`.
    fn get<'a, A: Allocator + Clone + 'a, Pb: PresenceBits>(
        slot: &'a Self::Slot<A>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self::Ref<'a, A>;

    /// Ensures the slot is present and returns a mutable accessor handle.
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a, A>
    where
        A: Allocator + Clone + 'a,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a;

    /// Writes `value`, ensuring slot presence when applicable.
    fn write<A, VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written<A>,
    ) where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits;

    /// Clears the logical value and slot presence / payload.
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits;

    /// Merges one wire occurrence into the slot after the tag has been read.
    ///
    /// This is the **only** singular wire-decode entry for inline payloads.
    /// Scalars / string / bytes typically last-win [`write`](Self::write);
    /// nested messages merge recursively into the present child. Closed-enum
    /// unknowns are parked in `common.unknown_fields`.
    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        field: u32,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf;
}

// ---------------------------------------------------------------------------
// Numerical markers (Slot = bare wire value) — one blanket for all families
// ---------------------------------------------------------------------------

impl<T> ProtoType for T
where
    T: NumericalType,
{
    type Slot<A: Allocator + Clone> = T::Value;
    type Ref<'a, A: Allocator + Clone>
        = T::Value
    where
        Self: 'a,
        A: 'a;
    type Mut<'a, A: Allocator + Clone>
        = &'a mut T::Value
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = T::Value;

    #[inline]
    fn encoded_len<'a, A: Allocator + Clone + 'a>(value: T::Value, field: u32) -> usize
    where
        Self: 'a,
    {
        T::encoded_len_field(value, field)
    }

    #[inline]
    fn encode<'a, A: Allocator + Clone + 'a, B: BufMut>(value: T::Value, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        T::encode_field(value, field, buf);
    }
}

impl<T> PayloadAccess for T
where
    T: NumericalType,
{
    #[inline]
    fn is_proto_empty<A: Allocator + Clone, Pb: PresenceBits>(
        slot: &T::Value,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        slot.is_proto_empty()
    }

    #[inline]
    fn get<'a, A: Allocator + Clone + 'a, Pb: PresenceBits>(
        slot: &'a T::Value,
        _common: &'a MessageCommon<Pb, A>,
    ) -> T::Value {
        *slot
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> &'a mut T::Value
    where
        A: Allocator + Clone + 'a,
        T::Value: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<T::Value, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        ValueSlot::with_mut(slot, init, common).get_mut()
    }

    #[inline]
    fn write<A, VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: T::Value,
    ) where
        A: Allocator + Clone,
        T::Value: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<T::Value, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        T::Value: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<T::Value, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        field: u32,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        T::Value: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<T::Value, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf,
    {
        match T::decode_wire_value(wire_type, buf) {
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
// LEN markers
// ---------------------------------------------------------------------------

impl ProtoType for ProtoString {
    type Slot<A: Allocator + Clone> = UnmanagedString<A>;
    type Ref<'a, A: Allocator + Clone>
        = &'a str
    where
        Self: 'a,
        A: 'a;
    type Mut<'a, A: Allocator + Clone>
        = StringGuard<'a, A>
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = UnmanagedString<A>;

    #[inline]
    fn encoded_len<'a, A: Allocator + Clone + 'a>(value: &'a str, field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_len_field(field, value.len())
    }

    #[inline]
    fn encode<'a, A: Allocator + Clone + 'a, B: BufMut>(value: &'a str, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        encode::encode_len_field(field, value.as_bytes(), buf);
    }
}

impl PayloadAccess for ProtoString {
    #[inline]
    fn is_proto_empty<A: Allocator + Clone, Pb: PresenceBits>(
        slot: &UnmanagedString<A>,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        slot.is_proto_empty()
    }

    #[inline]
    fn get<'a, A: Allocator + Clone + 'a, Pb: PresenceBits>(
        slot: &'a UnmanagedString<A>,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a str {
        slot
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> StringGuard<'a, A>
    where
        A: Allocator + Clone + 'a,
        VS: ValueSlot<UnmanagedString<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        let alloc = common.alloc.clone();
        // SAFETY: message allocator owns this string buffer.
        unsafe {
            ValueSlot::with_mut(slot, init, common)
                .get_mut()
                .with_alloc(alloc)
        }
    }

    #[inline]
    fn write<A, VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: UnmanagedString<A>,
    ) where
        A: Allocator + Clone,
        VS: ValueSlot<UnmanagedString<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        VS: ValueSlot<UnmanagedString<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        _field: u32,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        VS: ValueSlot<UnmanagedString<A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let new = decode::decode_string_in(buf, common.alloc.clone())?;
        Self::write(slot, init, common, new);
        Ok(())
    }
}

impl ProtoType for ProtoBytes {
    type Slot<A: Allocator + Clone> = UnmanagedVec<u8, A>;
    type Ref<'a, A: Allocator + Clone>
        = &'a [u8]
    where
        Self: 'a,
        A: 'a;
    type Mut<'a, A: Allocator + Clone>
        = VecGuard<'a, u8, A>
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = UnmanagedVec<u8, A>;

    #[inline]
    fn encoded_len<'a, A: Allocator + Clone + 'a>(value: &'a [u8], field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_len_field(field, value.len())
    }

    #[inline]
    fn encode<'a, A: Allocator + Clone + 'a, B: BufMut>(value: &'a [u8], field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        encode::encode_len_field(field, value, buf);
    }
}

impl PayloadAccess for ProtoBytes {
    #[inline]
    fn is_proto_empty<A: Allocator + Clone, Pb: PresenceBits>(
        slot: &UnmanagedVec<u8, A>,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        slot.is_proto_empty()
    }

    #[inline]
    fn get<'a, A: Allocator + Clone + 'a, Pb: PresenceBits>(
        slot: &'a UnmanagedVec<u8, A>,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a [u8] {
        slot
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> VecGuard<'a, u8, A>
    where
        A: Allocator + Clone + 'a,
        VS: ValueSlot<UnmanagedVec<u8, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        let alloc = common.alloc.clone();
        // SAFETY: message allocator owns this bytes buffer.
        unsafe {
            ValueSlot::with_mut(slot, init, common)
                .get_mut()
                .with_alloc(alloc)
        }
    }

    #[inline]
    fn write<A, VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: UnmanagedVec<u8, A>,
    ) where
        A: Allocator + Clone,
        VS: ValueSlot<UnmanagedVec<u8, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        VS: ValueSlot<UnmanagedVec<u8, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        _field: u32,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        VS: ValueSlot<UnmanagedVec<u8, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let new = decode::decode_bytes_in(buf, common.alloc.clone())?;
        Self::write(slot, init, common, new);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Bit-packed bool marker (Slot = (); value via BitPacked layout)
// ---------------------------------------------------------------------------

impl ProtoType for ProtoBool {
    type Slot<A: Allocator + Clone> = ();
    type Ref<'a, A: Allocator + Clone>
        = bool
    where
        Self: 'a,
        A: 'a;
    type Mut<'a, A: Allocator + Clone>
        = BitRef<'a, Mut, u8, Lsb0>
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = bool;

    #[inline]
    fn encoded_len<'a, A: Allocator + Clone + 'a>(value: bool, field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_varint_field(field, Self::encode_wire(value))
    }

    #[inline]
    fn encode<'a, A: Allocator + Clone + 'a, B: BufMut>(value: bool, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        encode::encode_varint_field(field, Self::encode_wire(value), buf);
    }
}
