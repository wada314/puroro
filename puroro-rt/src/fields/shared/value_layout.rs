//! Value-storage layout for singular fields (`Inline`, bit-packed bool, SSO string).
//!
//! Orthogonal to [`FieldPresence`](super::field_presence::FieldPresence):
//! presence decides *whether* a field is set; layout decides *where the value
//! bytes / bits live*.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::bytes::Buf;
use ::protobuf_core::FieldNumber;
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::unmanaged::{CloneIn, DeallocateIn, UnmanagedString, UnmanagedVec};

use super::{
    DefaultIn, MessageCommon, MessageCommonBits,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::decode;
use crate::fields::wire::len::ProtoString;
use crate::fields::wire::numerical::ProtoBool;
use crate::fields::wire::numerical::{BoolCodec, NumericalType};
use crate::fields::wire::singular_type::{PayloadAccess, SingularType};
use crate::fields::wire::sso_string::{
    INLINE_CAP, SsoString, SsoStringMut, pack_inline_utf8, pack_written,
};
use crate::fields::wire::wire_payload::{CopyWirePayload, VarintPayload};

/// Where a singular field's logical value is stored.
pub trait ValueLayout<T: SingularType, A: Allocator + Clone>: Copy
where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    fn is_proto_empty<Pb>(slot: &T::Slot<A>, common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    fn get<'a, Pb>(slot: &'a T::Slot<A>, common: &'a MessageCommon<Pb, A>) -> T::View<'a, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> T::Mut<'a, A>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        T: 'a,
        A: 'a;

    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits;

    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf;

    /// Message / oneof teardown for this field's value slot.
    fn deallocate_slot<VS, Pb>(slot: VS, initialized: bool, common: &MessageCommon<Pb, A>)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        if let Some(v) = slot.take_value(initialized) {
            // SAFETY: message allocator owns `v`.
            unsafe { DeallocateIn::deallocate_in(v, common.alloc.clone()) };
        }
    }

    /// Deep-copies this field's value slot into `alloc`.
    fn clone_slot<VS, Pb>(
        slot: &VS,
        initialized: bool,
        common: &MessageCommon<Pb, A>,
        alloc: A,
    ) -> VS
    where
        VS: ValueSlot<T::Slot<A>, A>,
        T::Slot<A>: CloneIn<A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let _ = common;
        VS::from_optional(
            slot.get_value(initialized)
                .map(|v| CloneIn::clone_in(v, alloc)),
        )
    }
}

/// Value lives in the field slot payload (`T::Slot<A>`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Inline;

impl<T: PayloadAccess, A: Allocator + Clone> ValueLayout<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    #[inline]
    fn is_proto_empty<Pb>(slot: &T::Slot<A>, common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        T::is_proto_empty(slot, common)
    }

    #[inline]
    fn get<'a, Pb>(slot: &'a T::Slot<A>, common: &'a MessageCommon<Pb, A>) -> T::View<'a, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        T::get(slot, common)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> T::Mut<'a, A>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        T: 'a,
        A: 'a,
    {
        T::with_mut(slot, init, common)
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        T::clear(slot, init, common);
    }

    #[inline]
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        T::merge(slot, init, common, wire_type, buf, field, depth)
    }
}

/// Logical `bool` packed at `VALUE_BIT` in [`MessageCommon`]'s bitvec.
///
/// Used only with [`ProtoBool`]. The field slot remains a ZST for presence/init.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BitPacked<const VALUE_BIT: usize>;

impl<A: Allocator + Clone, const VALUE_BIT: usize> ValueLayout<ProtoBool, A>
    for BitPacked<VALUE_BIT>
{
    #[inline]
    fn is_proto_empty<Pb>(_slot: &(), common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        !common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn get<'a, Pb>(_slot: &'a (), common: &'a MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> BitRef<'a, Mut, u8, Lsb0>
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        ProtoBool: 'a,
        A: 'a,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.bit_mut(VALUE_BIT)
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        common.set_bit(VALUE_BIT, false);
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        field: FieldNumber,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        match BoolCodec::from_wire_body(VarintPayload::decode(wire_type, buf)?) {
            Ok(new) => {
                let _ = ValueSlot::with_mut(slot, init, common).get_mut();
                common.set_bit(VALUE_BIT, new);
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

/// Polarity of [`InlineOrHeap`]'s `HEAP_BIT` in [`MessageCommon`].
///
/// - [`SSO_HEAP`] (`true`): slot holds a heap [`UnmanagedString`]
/// - [`SSO_INLINE`] (`false`): slot holds the inline buffer
pub const SSO_HEAP: bool = true;
/// See [`SSO_HEAP`].
pub const SSO_INLINE: bool = false;

/// Singular `string` layout: [`SsoString`] slot + heap/inline bit at `HEAP_BIT`.
///
/// `MessageCommon` bit `HEAP_BIT` is the sole arm discriminant for the untagged
/// slot: [`SSO_HEAP`] when the heap arm is live, [`SSO_INLINE`] when inline.
/// Short / empty heap states are allowed when installed via
/// [`pack_heap`](crate::fields::wire::sso_string::pack_heap).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct InlineOrHeap<const HEAP_BIT: usize>;

impl<const HEAP_BIT: usize> InlineOrHeap<HEAP_BIT> {
    /// `true` when `HEAP_BIT` selects the heap arm ([`SSO_HEAP`]).
    #[inline]
    fn is_heap<Pb, A: Allocator + Clone>(common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        common.is_bit_set(HEAP_BIT)
    }

    /// Writes `HEAP_BIT` (`is_heap == `[`SSO_HEAP`] / [`SSO_INLINE`]).
    #[inline]
    fn set_heap<Pb, A: Allocator + Clone>(common: &mut MessageCommon<Pb, A>, is_heap: bool)
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        common.set_bit(HEAP_BIT, is_heap);
    }
}

impl<A: Allocator + Clone, const HEAP_BIT: usize> ValueLayout<ProtoString, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn is_proto_empty<Pb>(slot: &SsoString<A>, common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        slot.is_empty(Self::is_heap(common))
    }

    #[inline]
    fn get<'a, Pb>(slot: &'a SsoString<A>, common: &'a MessageCommon<Pb, A>) -> &'a str
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        slot.as_str(Self::is_heap(common))
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> SsoStringMut<'a, A>
    where
        VS: ValueSlot<SsoString<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        ProtoString: 'a,
        A: 'a,
    {
        let alloc = common.alloc.clone();
        // Ensure presence/init, then split the slot pointer from `common` so we
        // can hand out both `&mut SsoString` and a heap-bit `BitRef`.
        let slot_ptr: *mut SsoString<A> = ValueSlot::with_mut(slot, init, common).get_mut();
        let tag = common.bit_mut(HEAP_BIT);
        // SAFETY: `tag` borrows only the bitfield in `common`; `slot_ptr` is the
        // distinct field payload and remains valid for `'a`.
        SsoStringMut::new(unsafe { &mut *slot_ptr }, tag, alloc)
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<SsoString<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        if !init.is_initialized(|b| common.is_bit_set(b)) {
            Self::set_heap(common, SSO_INLINE);
            return;
        }
        let old_is_heap = Self::is_heap(common);
        let alloc = common.alloc.clone();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            // SAFETY: HEAP_BIT matches the live arm.
            unsafe {
                s.replace_packed(SsoString::empty_inline(), SSO_INLINE, old_is_heap, alloc);
            }
        }
        Self::set_heap(common, SSO_INLINE);
        // Presence clear is a no-op for always-initialized slots.
        init.set_initialized(|b, v| common.set_bit(b, v), false);
    }

    #[inline]
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        _field: FieldNumber,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<SsoString<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let (new, new_is_heap) = decode_sso_packed(buf, common.alloc.clone())?;
        let old_is_heap = Self::is_heap(common);
        let alloc = common.alloc.clone();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            // SAFETY: message allocator owns any previous heap buffer; bit matches arm.
            unsafe { s.replace_packed(new, new_is_heap, old_is_heap, alloc) };
        }
        Self::set_heap(common, new_is_heap);
        Ok(())
    }

    #[inline]
    fn deallocate_slot<VS, Pb>(slot: VS, initialized: bool, common: &MessageCommon<Pb, A>)
    where
        VS: ValueSlot<SsoString<A>, A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let is_heap = Self::is_heap(common);
        let alloc = common.alloc.clone();
        if let Some(s) = slot.take_value(initialized) {
            // SAFETY: HEAP_BIT is the sole discriminant for this field.
            unsafe { s.deallocate(is_heap, alloc) };
        }
    }

    #[inline]
    fn clone_slot<VS, Pb>(
        slot: &VS,
        initialized: bool,
        common: &MessageCommon<Pb, A>,
        alloc: A,
    ) -> VS
    where
        VS: ValueSlot<SsoString<A>, A>,
        SsoString<A>: CloneIn<A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let is_heap = Self::is_heap(common);
        VS::from_optional(
            slot.get_value(initialized)
                .map(|s| s.clone_packed(is_heap, alloc).0),
        )
    }
}

/// Decodes one LEN string into a packed SSO slot, preferring inline when short.
fn decode_sso_packed<B: Buf, A: Allocator + Clone>(
    buf: &mut B,
    alloc: A,
) -> Result<(SsoString<A>, bool), DecodeError> {
    use ::core::str;

    let len = decode::decode_varint(buf)? as usize;
    if buf.remaining() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    if len <= INLINE_CAP {
        let mut tmp = [0u8; INLINE_CAP];
        let mut remaining = len;
        let mut filled = 0usize;
        while remaining > 0 {
            let chunk = buf.chunk();
            let to_copy = chunk.len().min(remaining);
            tmp[filled..filled + to_copy].copy_from_slice(&chunk[..to_copy]);
            buf.advance(to_copy);
            filled += to_copy;
            remaining -= to_copy;
        }
        match str::from_utf8(&tmp[..len]) {
            Ok(s) => Ok((pack_inline_utf8(s.as_bytes()), false)),
            Err(_) => Err(DecodeError::InvalidUtf8),
        }
    } else {
        let mut vec = AllocVec::<u8, A>::with_capacity_in(len, alloc.clone());
        let mut remaining = len;
        while remaining > 0 {
            let chunk = buf.chunk();
            let to_copy = chunk.len().min(remaining);
            vec.extend_from_slice(&chunk[..to_copy]);
            buf.advance(to_copy);
            remaining -= to_copy;
        }
        let bytes = UnmanagedVec::from_vec(vec);
        match UnmanagedString::from_utf8(bytes) {
            Ok(s) => Ok(pack_written(s, alloc)),
            Err(bytes) => {
                // SAFETY: `alloc` owns the buffer we just built.
                unsafe { bytes.deallocate(alloc) };
                Err(DecodeError::InvalidUtf8)
            }
        }
    }
}
