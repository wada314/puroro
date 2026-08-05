//! Singular (non-repeated) field semantics for protobuf **type** markers
//! (e.g. `int32` / [`ProtoInt32`](super::numerical::ProtoInt32), `string` /
//! [`ProtoString`](super::len::ProtoString) — not wire shapes like Varint / Len).
//!
//! Markers are allocator-free. Physical storage / views are GATs parametrised by `A`.
//!
//! Tagged encode is **not** on this trait — catalog code calls
//! [`encode_field`](super::encode_type::encode_field) with
//! [`EncodeType::View`](super::encode_type::EncodeType::View) after omit checks.
//!
//! **Storage access** (get / write / clear / merge) lives on [`PayloadAccess`]
//! for inline payloads, or on
//! [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout)
//! (`BitPacked`) for singular / oneof `bool`. Singular wire decode is
//! **merge-into only** (`PayloadAccess::merge` / `BitPacked::merge`); there is
//! no `SingularType::decode → Written`. [`SingularField`](crate::fields::singular::field::SingularField)
//! always goes through `ValueLayout`.
//!
//! Repeated fields use [`RepeatedElement`](super::repeated_element::RepeatedElement)
//! (`Element` storage). Numerical markers (including `ProtoBool`) share
//! [`NumericalType`](super::numerical::NumericalType) for `NativeType` ↔ `WireBody`
//! mapping; inline slot storage stays on [`PayloadAccess`] (`NativeType: AddressableSlot`).
//! Singular [`ProtoBool`](super::numerical::ProtoBool) uses `Slot = ()` + [`BitPacked`].

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::core::ops::{Deref, DerefMut};

use ::puroro::{DecodeBuf, DecodeError, WireType};

use ::unmanaged::DeallocateIn;

use crate::decode;
use crate::fields::shared::{
    DefaultIn, MessageCommon, MessageCommonBits,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};

use super::encode_type::EncodeType;
use super::len::{LenCodec, LenScalar};
use super::numerical::{Numerical, NumericalType, ProtoBool};
use super::wire_payload::CopyWirePayload;

/// Singular protobuf **type** marker (e.g. `ProtoInt32`, `ProtoString`) with
/// storage GATs on top of [`EncodeType`].
///
/// Physical storage is the GAT [`Slot`](Self::Slot):
/// - numerics / enums: bare `i32` / `E` / …
/// - [`ProtoBool`]: `()` (ZST; logical `bool` via [`BitPacked`](crate::fields::shared::value_layout::BitPacked))
/// - string / bytes: `UnmanagedString` / `UnmanagedVec`
/// - nested messages: `UnmanagedBox<M, A>` via [`ProtoMessage`](super::proto_message::ProtoMessage)
///
/// Getter views use [`EncodeType::View`] (same type as tagged encode).
pub trait SingularType: EncodeType {
    /// Physical value stored in the singular field slot (excluding
    /// [`MessageCommon`] bits).
    ///
    /// Call sites ([`SingularField`](crate::fields::singular::field::SingularField))
    /// require `Slot<A>: AddressableSlot<SlotAlloc = A> + DefaultIn<Alloc = A>`.
    /// Nested messages use `UnmanagedBox<M, A>`; call sites require
    /// `M: Message<Alloc = A> + unmanaged::DeallocateIn<A>` via `DefaultIn` /
    /// `DeallocateIn` on that box.
    type Slot<A: Allocator + Clone>;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, `StringGuard`,
    /// bit handle, …).
    type Mut<'a, A: Allocator + Clone>: DerefMut
    where
        Self: 'a,
        A: 'a;

    /// Value accepted by [`PayloadAccess::write`] / field `set`.
    type Written<A: Allocator + Clone>;
}

/// Inline payload access for markers whose value lives in [`SingularType::Slot`].
///
/// Not implemented for [`ProtoBool`] — use
/// [`BitPacked`](crate::fields::shared::value_layout::BitPacked) instead.
pub(crate) trait PayloadAccess: SingularType {
    /// `true` when the field holds protobuf empty / type-zero (IMPLICIT omit).
    ///
    /// Numerics / enums compare to [`Default`]; string / bytes use `is_empty`;
    /// a present nested message is never empty (absence is the presence layer).
    fn is_proto_empty<A: Allocator + Clone, Pb>(
        slot: &Self::Slot<A>,
        common: &MessageCommon<Pb, A>,
    ) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    /// Reads the logical getter view from the slot and/or `common`.
    fn get<'a, A: Allocator + Clone + 'a, Pb>(
        slot: &'a Self::Slot<A>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self::View<'a, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits;

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
        MessageCommon<Pb, A>: MessageCommonBits,
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
        MessageCommon<Pb, A>: MessageCommonBits;

    /// Clears the logical value and slot presence / payload.
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits;

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
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf;
}

// ---------------------------------------------------------------------------
// Numerical markers (Slot = NativeType; storage via AddressableSlot on methods)
// ---------------------------------------------------------------------------

impl<C> SingularType for Numerical<C>
where
    C: NumericalType,
    C::NativeType: AddressableSlot,
{
    type Slot<A: Allocator + Clone> = C::NativeType;
    type Mut<'a, A: Allocator + Clone>
        = &'a mut C::NativeType
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = C::NativeType;
}

impl<C> PayloadAccess for Numerical<C>
where
    C: NumericalType,
    C::NativeType: AddressableSlot,
{
    #[inline]
    fn is_proto_empty<A: Allocator + Clone, Pb>(
        slot: &C::NativeType,
        _common: &MessageCommon<Pb, A>,
    ) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        // Type-zero / default: floats treat `-0.0` as empty and `NaN` as non-empty.
        *slot == C::NativeType::default()
    }

    #[inline]
    fn get<'a, A: Allocator + Clone + 'a, Pb>(
        slot: &'a C::NativeType,
        _common: &'a MessageCommon<Pb, A>,
    ) -> C::NativeType
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        *slot
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> &'a mut C::NativeType
    where
        A: Allocator + Clone + 'a,
        C::NativeType: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<C::NativeType, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        Self: 'a,
    {
        ValueSlot::with_mut(slot, init, common).get_mut()
    }

    #[inline]
    fn write<A, VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: C::NativeType,
    ) where
        A: Allocator + Clone,
        C::NativeType: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<C::NativeType, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        C::NativeType: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<C::NativeType, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
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
        C::NativeType: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<C::NativeType, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        match C::from_wire_body(C::WireBody::decode(wire_type, buf)?) {
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
// LEN scalars (`LenScalar<C>`)
// ---------------------------------------------------------------------------

impl<C: LenCodec> SingularType for LenScalar<C> {
    type Slot<A: Allocator + Clone> = C::Slot<A>;
    type Mut<'a, A: Allocator + Clone>
        = C::Mut<'a, A>
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = C::Slot<A>;
}

impl<C: LenCodec> PayloadAccess for LenScalar<C> {
    #[inline]
    fn is_proto_empty<A: Allocator + Clone, Pb>(
        slot: &C::Slot<A>,
        _common: &MessageCommon<Pb, A>,
    ) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        C::as_wire_bytes(Deref::deref(slot)).is_empty()
    }

    #[inline]
    fn get<'a, A: Allocator + Clone + 'a, Pb>(
        slot: &'a C::Slot<A>,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a C::RefView
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        Deref::deref(slot)
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> C::Mut<'a, A>
    where
        A: Allocator + Clone + 'a,
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        Self: 'a,
    {
        let alloc = common.alloc.clone();
        // SAFETY: message allocator owns this LEN scalar buffer.
        unsafe { C::slot_with_alloc(ValueSlot::with_mut(slot, init, common).get_mut(), alloc) }
    }

    #[inline]
    fn write<A, VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: C::Slot<A>,
    ) where
        A: Allocator + Clone,
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
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
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let new = C::decode_in(buf, common.alloc.clone())?;
        Self::write(slot, init, common, new);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Bit-packed bool marker (Slot = (); value via BitPacked layout)
// ---------------------------------------------------------------------------

impl SingularType for ProtoBool {
    type Slot<A: Allocator + Clone> = ();
    type Mut<'a, A: Allocator + Clone>
        = BitRef<'a, Mut, u8, Lsb0>
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = bool;
}
