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
//! **Storage access** (get / write / clear) lives on [`PayloadAccess`]
//! for inline payloads, or on
//! [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout)
//! (`BitPacked` / `InlineOrHeap`) for packed bool and SSO string / bytes. Singular wire
//! decode is **merge-into only** ([`PayloadMerge`] / layout merge);
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

use crate::decode;
use crate::fields::shared::{
    CloneBound, DeallocateBound, DefaultIn, InlinedMessageParent, MessageBindingMut,
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
///   bytes (`UnmanagedVec`), inlined nested messages (`M`)
/// - [`Boxed`](crate::fields::shared::value_layout::Boxed): nested message
///   [`UnmanagedBox<M, A>`](::unmanaged::UnmanagedBox)
/// - [`BitPacked`](crate::fields::shared::value_layout::BitPacked): `()` + bit handle
///   (packed singular / oneof `bool`)
/// - [`InlineOrHeap`](crate::fields::shared::value_layout::InlineOrHeap):
///   [`SsoString`](super::sso_string::SsoString) / [`SsoBytes`](super::sso_bytes::SsoBytes)
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
    type Slot<A: Allocator>: AddressableSlot;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, `StringGuard`, …).
    ///
    /// Bound is [`Deref`] only so mutators need not expose `DerefMut`.
    type Mut<'a, A: Allocator>: Deref
    where
        Self: 'a,
        A: 'a;

    /// Value accepted by [`write`](Self::write).
    type Written<A: Allocator>;

    /// `true` when the field holds protobuf empty / type-zero (IMPLICIT omit).
    ///
    /// Numerics / enums compare to [`Default`]; string / bytes use `is_empty`;
    /// a present nested message is never empty (absence is the presence layer).
    fn is_proto_empty<A: Allocator, Cx>(slot: &Self::Slot<A>, common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>;

    /// Reads the logical getter view from the slot and/or `common`.
    fn get<'a, A: Allocator + 'a, Cx>(slot: &'a Self::Slot<A>, common: &'a Cx) -> Self::View<'a, A>
    where
        Cx: MessageBindingMut<A>;

    /// Ensures the slot is present and returns a mutable accessor handle.
    fn with_mut<'a, A, VS, I, Cx>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut Cx,
    ) -> Self::Mut<'a, A>
    where
        A: Allocator + Clone + 'a,
        Self::Slot<A>: AddressableSlot + DefaultIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
        Self: 'a;

    /// Writes `value`, ensuring slot presence when applicable.
    ///
    /// Any previous payload is released with `common.alloc`. `common` must be
    /// the parent [`MessageCommon`] that owns the existing slot (same pairing
    /// as [`ValueLayout::deallocate_slot`](crate::fields::shared::value_layout::ValueLayout::deallocate_slot)).
    fn write<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, value: Self::Written<A>)
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>;

    /// Clears the logical value and slot presence / payload.
    ///
    /// Releases the previous payload with `common.alloc`. `common` must be the
    /// parent [`MessageCommon`] that owns the slot.
    fn clear<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>;

    /// Releases an extracted inline slot. `common` is the field's parent binding.
    fn deallocate_payload<A, Cx>(slot: Self::Slot<A>, common: &Cx)
    where
        A: Allocator,
        Cx: MessageBindingMut<A>,
        Self::Slot<A>: DeallocateBound<A>;

    /// Deep-copies an extracted inline slot using `common` for bits / tags.
    fn clone_payload<A, Cx>(slot: &Self::Slot<A>, common: &Cx, alloc: A) -> Self::Slot<A>
    where
        A: Allocator + Clone,
        Cx: MessageBindingMut<A>,
        Self::Slot<A>: CloneBound<A>;
}

/// Wire-decode merge for inline payloads. Kept off [`PayloadAccess`] so nested
/// messages can be *named* as [`PayloadAccess`] without [`crate::MessageMerge`].
pub trait PayloadMerge: PayloadAccess {
    /// Merges one wire occurrence into the slot after the tag has been read.
    ///
    /// This is the **only** singular wire-decode entry for inline payloads.
    /// Scalars / string / bytes typically last-win [`PayloadAccess::write`];
    /// nested messages merge recursively into the present child. Closed-enum
    /// unknowns are parked in `common.unknown_fields`.
    fn merge<A, VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut B,
        field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
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
    type Slot<A: Allocator> = C::NativeType;
    type Mut<'a, A: Allocator>
        = &'a mut C::NativeType
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator> = C::NativeType;

    #[inline]
    fn is_proto_empty<A: Allocator, Cx>(slot: &C::NativeType, _common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        // Type-zero / default: floats treat `-0.0` as empty and `NaN` as non-empty.
        *slot == C::NativeType::default()
    }

    #[inline]
    fn get<'a, A: Allocator + 'a, Cx>(slot: &'a C::NativeType, _common: &'a Cx) -> C::NativeType
    where
        Cx: MessageBindingMut<A>,
    {
        *slot
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Cx>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut Cx,
    ) -> &'a mut C::NativeType
    where
        A: Allocator + Clone + 'a,
        C::NativeType: AddressableSlot + DefaultIn<A>,
        VS: ValueSlot<C::NativeType, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
        Self: 'a,
    {
        ValueSlot::with_mut(slot, init, common).get_mut()
    }

    #[inline]
    fn write<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, value: C::NativeType)
    where
        A: Allocator + Clone,
        C::NativeType: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<C::NativeType, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
            old.deallocate_bound(common);
        }
    }

    #[inline]
    fn clear<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        A: Allocator + Clone,
        C::NativeType: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<C::NativeType, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            old.deallocate_bound(common);
        }
    }

    #[inline]
    fn deallocate_payload<A, Cx>(slot: C::NativeType, common: &Cx)
    where
        A: Allocator,
        Cx: MessageBindingMut<A>,
        C::NativeType: DeallocateBound<A>,
    {
        slot.deallocate_bound(common);
    }

    #[inline]
    fn clone_payload<A, Cx>(slot: &C::NativeType, common: &Cx, alloc: A) -> C::NativeType
    where
        A: Allocator + Clone,
        Cx: MessageBindingMut<A>,
        C::NativeType: CloneBound<A>,
    {
        slot.clone_bound(common, alloc)
    }
}

impl<C> PayloadMerge for Numerical<C>
where
    C: NumericalType,
    C::NativeType: AddressableSlot,
{
    #[inline]
    fn merge<A, VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut B,
        field: FieldNumber,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
        B: DecodeBuf,
    {
        match C::from_wire_body(C::WireBody::decode(wire_type, buf)?) {
            Ok(new) => {
                Self::write(slot, init, common, new);
                Ok(())
            }
            Err(DecodeError::UnknownClosedEnum { raw }) => {
                let alloc = common.clone_alloc();
                decode::save_unknown_varint_field(field, raw, common.unknown_fields_mut(), alloc);
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
    type Slot<A: Allocator> = C::Slot<A>;
    type Mut<'a, A: Allocator>
        = C::Mut<'a, A>
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator> = C::Slot<A>;

    #[inline]
    fn is_proto_empty<A: Allocator, Cx>(slot: &C::Slot<A>, _common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        C::as_wire_bytes(Deref::deref(slot)).is_empty()
    }

    #[inline]
    fn get<'a, A: Allocator + 'a, Cx>(slot: &'a C::Slot<A>, _common: &'a Cx) -> &'a C::RefView
    where
        Cx: MessageBindingMut<A>,
    {
        Deref::deref(slot)
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> C::Mut<'a, A>
    where
        A: Allocator + Clone + 'a,
        C::Slot<A>: AddressableSlot + DefaultIn<A>,
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
        Self: 'a,
    {
        let alloc = common.clone_alloc();
        // SAFETY: message allocator owns this LEN scalar buffer.
        unsafe { C::slot_with_alloc(ValueSlot::with_mut(slot, init, common).get_mut(), alloc) }
    }

    #[inline]
    fn write<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, value: C::Slot<A>)
    where
        A: Allocator + Clone,
        C::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
            old.deallocate_bound(common);
        }
    }

    #[inline]
    fn clear<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        A: Allocator + Clone,
        C::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<C::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            old.deallocate_bound(common);
        }
    }

    #[inline]
    fn deallocate_payload<A, Cx>(slot: C::Slot<A>, common: &Cx)
    where
        A: Allocator,
        Cx: MessageBindingMut<A>,
        C::Slot<A>: DeallocateBound<A>,
    {
        slot.deallocate_bound(common);
    }

    #[inline]
    fn clone_payload<A, Cx>(slot: &C::Slot<A>, common: &Cx, alloc: A) -> C::Slot<A>
    where
        A: Allocator + Clone,
        Cx: MessageBindingMut<A>,
        C::Slot<A>: CloneBound<A>,
    {
        slot.clone_bound(common, alloc)
    }
}

impl<C: LenCodec> PayloadMerge for LenScalar<C> {
    #[inline]
    fn merge<A, VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut B,
        _field: FieldNumber,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
        B: DecodeBuf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let new = C::decode_in(buf, common.clone_alloc())?;
        Self::write(slot, init, common, new);
        Ok(())
    }
}
