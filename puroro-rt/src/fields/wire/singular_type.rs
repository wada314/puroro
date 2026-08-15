//! Singular (non-repeated) field semantics for protobuf **type** markers
//! (e.g. `int32` / [`ProtoInt32`](super::numerical::ProtoInt32), `string` /
//! [`ProtoString`](super::len::ProtoString) — not wire shapes like Varint / Len).
//!
//! Markers are allocator-free. Getter views stay on [`EncodeType`]. Physical
//! slot / mutator types live on [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout)
//! (`Inline` aliases [`PayloadAccess`]; `BitPacked` / `InlineOrHeap` choose
//! their own).
//!
//! Tagged encode is **not** on this trait — catalog code calls
//! [`encode_field`](super::encode_type::encode_field) with
//! [`EncodeType::View`](super::encode_type::EncodeType::View) after omit checks.
//!
//! **Storage access** (get / write / clear / merge) lives on [`PayloadAccess`]
//! for inline payloads, or on
//! [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout)
//! (`BitPacked` / `InlineOrHeap`) for packed bool and SSO string. Singular wire
//! decode is **merge-into only** (`PayloadAccess::merge` / layout `merge`);
//! there is no `decode → Written`. [`SingularField`](crate::fields::singular::field::SingularField)
//! always goes through `ValueLayout`.
//!
//! Repeated fields use [`RepeatedElement`](super::repeated_element::RepeatedElement)
//! (`Element` storage). Numerical markers (including `ProtoBool`) share
//! [`NumericalType`](super::numerical::NumericalType) for `NativeType` ↔ `WireBody`
//! mapping; inline slot storage stays on [`PayloadAccess`] (`NativeType: AddressableSlot`).
//! Singular [`ProtoBool`](super::numerical::ProtoBool) uses `Inline` (`Slot = bool`)
//! or [`BitPacked`](crate::fields::shared::value_layout::BitPacked) (`Slot = ()`).

use ::allocator_api2::alloc::Allocator;
use ::core::ops::Deref;
use ::protobuf_core::FieldNumber;

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
use super::numerical::{Numerical, NumericalType};
use super::wire_payload::CopyWirePayload;

/// Singular protobuf **type** marker (e.g. `ProtoInt32`, `ProtoString`).
///
/// Extends [`EncodeType`] so [`SingularField`](crate::fields::singular::field::SingularField)
/// can require a proto type without taking a storage layout. Slot / mutator
/// types are on [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout):
/// - [`Inline`](crate::fields::shared::value_layout::Inline) + [`PayloadAccess`]:
///   numerics / enums / bool (`i32` / `E` / `bool`), heap string (`UnmanagedString`),
///   bytes (`UnmanagedVec`), nested messages (`UnmanagedBox<M, A>`)
/// - [`BitPacked`](crate::fields::shared::value_layout::BitPacked): `()` + bit handle
///   (packed singular / oneof `bool`)
/// - [`InlineOrHeap`](crate::fields::shared::value_layout::InlineOrHeap):
///   [`SsoString`](super::sso_string::SsoString)
///
/// Getter views use [`EncodeType::View`] (same type as tagged encode).
pub trait SingularType: EncodeType {}

/// Inline payload access for markers whose value lives in the field slot.
///
/// Public because [`Inline`](crate::fields::shared::value_layout::Inline) aliases
/// these associated types on the public [`ValueLayout`](crate::ValueLayout)
/// impl. Not intended for generated code — prefer `ValueLayout::Slot` /
/// `ValueLayout::Mut`. [`ProtoBool`](super::numerical::ProtoBool) implements
/// this for [`Inline`](crate::fields::shared::value_layout::Inline) (`Slot = bool`);
/// packed singular / oneof bool uses
/// [`BitPacked`](crate::fields::shared::value_layout::BitPacked) instead.
pub trait PayloadAccess: SingularType {
    /// Physical value stored in the singular field slot (excluding
    /// [`MessageCommon`] bits).
    type Slot<A: Allocator + Clone>: AddressableSlot;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, `StringGuard`, …).
    ///
    /// Bound is [`Deref`] only so mutators need not expose `DerefMut`.
    type Mut<'a, A: Allocator + Clone>: Deref
    where
        Self: 'a,
        A: 'a;

    /// Value accepted by [`write`](Self::write).
    type Written<A: Allocator + Clone>;

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
        Self::Slot<A>: AddressableSlot + DefaultIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        Self: 'a;

    /// Writes `value`, ensuring slot presence when applicable.
    ///
    /// Any previous payload is released with `common.alloc`. `common` must be
    /// the parent [`MessageCommon`] that owns the existing slot (same pairing
    /// as [`ValueLayout::deallocate_slot`](crate::fields::shared::value_layout::ValueLayout::deallocate_slot)).
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
    ///
    /// Releases the previous payload with `common.alloc`. `common` must be the
    /// parent [`MessageCommon`] that owns the slot.
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
        field: FieldNumber,
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

impl<C: NumericalType> SingularType for Numerical<C> {}

impl<C> PayloadAccess for Numerical<C>
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
        C::NativeType: AddressableSlot + DefaultIn<A>,
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
        let alloc = common.alloc.clone();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
            // SAFETY: `write` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, alloc) };
        }
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
        let alloc = common.alloc.clone();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            // SAFETY: `clear` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, alloc) };
        }
    }

    #[inline]
    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        field: FieldNumber,
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
// LEN scalars — heap `UnmanagedString` / `UnmanagedVec` via [`PayloadAccess`] +
// [`Inline`]. Singular `string` SSO uses [`InlineOrHeap`] instead.
// ---------------------------------------------------------------------------

impl<C: LenCodec> SingularType for LenScalar<C> {}

impl<C: LenCodec> PayloadAccess for LenScalar<C> {
    type Slot<A: Allocator + Clone> = C::Slot<A>;
    type Mut<'a, A: Allocator + Clone>
        = C::Mut<'a, A>
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = C::Slot<A>;

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
        let alloc = common.alloc.clone();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
            // SAFETY: `write` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, alloc) };
        }
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let alloc = common.alloc.clone();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            // SAFETY: `clear` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, alloc) };
        }
    }

    #[inline]
    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        _field: FieldNumber,
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
