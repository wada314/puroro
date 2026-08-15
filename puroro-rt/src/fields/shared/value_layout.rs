//! Value-storage layout for singular fields (`Inline`, bit-packed bool, SSO string / bytes).
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
use ::core::ops::Deref;
use ::protobuf_core::FieldNumber;
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::unmanaged::{CloneIn, DeallocateIn, UnmanagedString, UnmanagedVec};

use super::{
    DefaultIn, MessageCommon, MessageCommonBits,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::decode;
use crate::fields::wire::len::{ProtoBytes, ProtoString};
use crate::fields::wire::numerical::ProtoBool;
use crate::fields::wire::numerical::{BoolCodec, NumericalType};
use crate::fields::wire::singular_type::{PayloadAccess, SingularType};
use crate::fields::wire::sso_buf::pack_inline;
use crate::fields::wire::sso_bytes::{SsoBytes, SsoBytesMut, pack_written as pack_written_bytes};
use crate::fields::wire::sso_string::{
    INLINE_CAP, SsoString, SsoStringMut, pack_inline_utf8, pack_written,
};
use crate::fields::wire::wire_payload::{CopyWirePayload, VarintPayload};

/// Where a singular field's logical value is stored.
///
/// Associated [`Slot`](Self::Slot) / [`Mut`](Self::Mut) are the physical storage
/// and `_mut` handle for this `(T, L)` pair — not properties of `T` alone.
pub trait ValueLayout<T: SingularType, A: Allocator + Clone>: Copy {
    /// Physical value stored in the singular field slot (excluding
    /// [`MessageCommon`] bits).
    type Slot: AddressableSlot + DefaultIn<A>;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, SSO mutator,
    /// bit handle, …).
    ///
    /// Bound is [`Deref`] only so SSO string mutators need not expose
    /// `DerefMut` (edits go through inherent / trait methods). Concrete handles
    /// such as `&mut T` / `BitRef` still implement `DerefMut`.
    type Mut<'a>: Deref
    where
        T: 'a,
        A: 'a,
        Self: 'a;

    fn is_proto_empty<Pb>(slot: &Self::Slot, common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    fn get<'a, Pb>(slot: &'a Self::Slot, common: &'a MessageCommon<Pb, A>) -> T::View<'a, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        T: 'a,
        A: 'a;

    /// Clears the logical value and slot presence / payload.
    ///
    /// `common` must be this field's parent [`MessageCommon`] (same pairing as
    /// [`deallocate_slot`](Self::deallocate_slot)).
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot, A>,
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
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf;

    /// Message / oneof teardown for this field's value slot.
    ///
    /// `common` must be the parent [`MessageCommon`] that allocated `slot`
    /// (same instance generated `Drop` passes as `&self._common`). The type
    /// system does not prove this pairing; a different message's common is
    /// unsound. SSO layouts also read the heap bit from this `common`.
    fn deallocate_slot<VS, Pb>(slot: VS, initialized: bool, common: &MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot, A>,
        MessageCommon<Pb, A>: MessageCommonBits;
}

/// Slot deep-copy for layouts whose payload can be cloned.
///
/// Kept off [`ValueLayout`] so getter paths do not require [`CloneIn`] (nested
/// messages) and SSO can clone via the heap bit instead of a blind `CloneIn`.
pub trait ValueLayoutClone<T: SingularType, A: Allocator + Clone>: ValueLayout<T, A> {
    /// Deep-copies this field's value slot into `alloc`.
    ///
    /// `common` must be the **source** field's parent [`MessageCommon`] (bits /
    /// heap-vs-inline tag). `alloc` is the **destination** allocator and need
    /// not be `common.alloc`. The type system does not prove that `slot`
    /// belongs to `common`.
    fn clone_slot<VS, Pb>(
        slot: &VS,
        initialized: bool,
        common: &MessageCommon<Pb, A>,
        alloc: A,
    ) -> VS
    where
        VS: ValueSlot<Self::Slot, A>,
        MessageCommon<Pb, A>: MessageCommonBits;
}

/// Value lives in the field slot payload ([`PayloadAccess::Slot`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Inline;

impl<T: PayloadAccess, A: Allocator + Clone> ValueLayout<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    type Slot = T::Slot<A>;
    type Mut<'a>
        = T::Mut<'a, A>
    where
        T: 'a,
        A: 'a;

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

    #[inline]
    fn deallocate_slot<VS, Pb>(slot: VS, initialized: bool, common: &MessageCommon<Pb, A>)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        if let Some(v) = slot.take_value(initialized) {
            // SAFETY: `deallocate_slot` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(v, common.alloc.clone()) };
        }
    }
}

impl<T: PayloadAccess, A: Allocator + Clone> ValueLayoutClone<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A> + CloneIn<A>,
{
    #[inline]
    fn clone_slot<VS, Pb>(
        slot: &VS,
        initialized: bool,
        common: &MessageCommon<Pb, A>,
        alloc: A,
    ) -> VS
    where
        VS: ValueSlot<T::Slot<A>, A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let _ = common;
        VS::from_optional(
            slot.get_value(initialized)
                .map(|v| CloneIn::clone_in(v, alloc)),
        )
    }
}

/// Logical `bool` packed at `VALUE_BIT` in [`MessageCommon`]'s bitvec.
///
/// Used only with [`ProtoBool`]. The field slot remains a ZST; the value lives
/// in bits. [`Inline`](Inline) + [`ProtoBool`] stores a plain `bool` in the slot
/// instead (same [`PayloadAccess`] path as other numerics).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BitPacked<const VALUE_BIT: usize>;

impl<A: Allocator + Clone, const VALUE_BIT: usize> ValueLayout<ProtoBool, A>
    for BitPacked<VALUE_BIT>
{
    type Slot = ();
    type Mut<'a>
        = BitRef<'a, Mut, u8, Lsb0>
    where
        A: 'a;

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
        let _ = ValueSlot::with_mut(slot, init, common).take_clear();
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

    #[inline]
    fn deallocate_slot<VS, Pb>(slot: VS, initialized: bool, _common: &MessageCommon<Pb, A>)
    where
        VS: ValueSlot<(), A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let _ = slot.take_value(initialized);
    }
}

impl<A: Allocator + Clone, const VALUE_BIT: usize> ValueLayoutClone<ProtoBool, A>
    for BitPacked<VALUE_BIT>
{
    #[inline]
    fn clone_slot<VS, Pb>(
        slot: &VS,
        initialized: bool,
        _common: &MessageCommon<Pb, A>,
        _alloc: A,
    ) -> VS
    where
        VS: ValueSlot<(), A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        VS::from_optional(slot.get_value(initialized).copied())
    }
}

/// Polarity of [`InlineOrHeap`]'s `HEAP_BIT` in [`MessageCommon`].
///
/// - [`SSO_HEAP`] (`true`): slot holds a heap [`UnmanagedString`]
/// - [`SSO_INLINE`] (`false`): slot holds the inline buffer
pub const SSO_HEAP: bool = true;
/// See [`SSO_HEAP`].
pub const SSO_INLINE: bool = false;

/// Singular `string` / `bytes` layout: SSO slot + heap/inline bit at `HEAP_BIT`.
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
    type Slot = SsoString<A>;
    type Mut<'a>
        = SsoStringMut<'a, A>
    where
        A: 'a;

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
            // SAFETY: HEAP_BIT / `alloc` come from this field's parent `common`.
            unsafe { s.deallocate(is_heap, alloc) };
        }
    }
}

impl<A: Allocator + Clone, const HEAP_BIT: usize> ValueLayoutClone<ProtoString, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn clone_slot<VS, Pb>(
        slot: &VS,
        initialized: bool,
        common: &MessageCommon<Pb, A>,
        alloc: A,
    ) -> VS
    where
        VS: ValueSlot<SsoString<A>, A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let is_heap = Self::is_heap(common);
        VS::from_optional(
            slot.get_value(initialized)
                .map(|s| s.clone_packed(is_heap, alloc).0),
        )
    }
}

impl<A: Allocator + Clone, const HEAP_BIT: usize> ValueLayout<ProtoBytes, A>
    for InlineOrHeap<HEAP_BIT>
{
    type Slot = SsoBytes<A>;
    type Mut<'a>
        = SsoBytesMut<'a, A>
    where
        A: 'a;

    #[inline]
    fn is_proto_empty<Pb>(slot: &SsoBytes<A>, common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        slot.is_empty(Self::is_heap(common))
    }

    #[inline]
    fn get<'a, Pb>(slot: &'a SsoBytes<A>, common: &'a MessageCommon<Pb, A>) -> &'a [u8]
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        slot.as_bytes(Self::is_heap(common))
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> SsoBytesMut<'a, A>
    where
        VS: ValueSlot<SsoBytes<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        ProtoBytes: 'a,
        A: 'a,
    {
        let alloc = common.alloc.clone();
        let slot_ptr: *mut SsoBytes<A> = ValueSlot::with_mut(slot, init, common).get_mut();
        let tag = common.bit_mut(HEAP_BIT);
        // SAFETY: `tag` borrows only the bitfield in `common`; `slot_ptr` is the
        // distinct field payload and remains valid for `'a`.
        SsoBytesMut::new(unsafe { &mut *slot_ptr }, tag, alloc)
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<SsoBytes<A>, A>,
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
                s.replace_packed(SsoBytes::empty_inline(), SSO_INLINE, old_is_heap, alloc);
            }
        }
        Self::set_heap(common, SSO_INLINE);
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
        VS: ValueSlot<SsoBytes<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let (new, new_is_heap) = decode_sso_bytes_packed(buf, common.alloc.clone())?;
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
        VS: ValueSlot<SsoBytes<A>, A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let is_heap = Self::is_heap(common);
        let alloc = common.alloc.clone();
        if let Some(s) = slot.take_value(initialized) {
            // SAFETY: HEAP_BIT / `alloc` come from this field's parent `common`.
            unsafe { s.deallocate(is_heap, alloc) };
        }
    }
}

impl<A: Allocator + Clone, const HEAP_BIT: usize> ValueLayoutClone<ProtoBytes, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn clone_slot<VS, Pb>(
        slot: &VS,
        initialized: bool,
        common: &MessageCommon<Pb, A>,
        alloc: A,
    ) -> VS
    where
        VS: ValueSlot<SsoBytes<A>, A>,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let is_heap = Self::is_heap(common);
        VS::from_optional(
            slot.get_value(initialized)
                .map(|s| s.clone_packed(is_heap, alloc).0),
        )
    }
}

/// Bytes read from one LEN payload, still untyped (string vs bytes).
enum SsoLenRead<A: Allocator> {
    Inline { data: [u8; INLINE_CAP], len: usize },
    Heap(UnmanagedVec<u8, A>),
}

fn read_sso_len<B: Buf, A: Allocator + Clone>(
    buf: &mut B,
    alloc: A,
) -> Result<SsoLenRead<A>, DecodeError> {
    let len = decode::decode_varint(buf)? as usize;
    if buf.remaining() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    if len <= INLINE_CAP {
        let mut data = [0u8; INLINE_CAP];
        let mut remaining = len;
        let mut filled = 0usize;
        while remaining > 0 {
            let chunk = buf.chunk();
            let to_copy = chunk.len().min(remaining);
            data[filled..filled + to_copy].copy_from_slice(&chunk[..to_copy]);
            buf.advance(to_copy);
            filled += to_copy;
            remaining -= to_copy;
        }
        Ok(SsoLenRead::Inline { data, len })
    } else {
        let mut vec = AllocVec::<u8, A>::with_capacity_in(len, alloc);
        let mut remaining = len;
        while remaining > 0 {
            let chunk = buf.chunk();
            let to_copy = chunk.len().min(remaining);
            vec.extend_from_slice(&chunk[..to_copy]);
            buf.advance(to_copy);
            remaining -= to_copy;
        }
        Ok(SsoLenRead::Heap(UnmanagedVec::from_vec(vec)))
    }
}

/// Decodes one LEN string into a packed SSO slot, preferring inline when short.
fn decode_sso_packed<B: Buf, A: Allocator + Clone>(
    buf: &mut B,
    alloc: A,
) -> Result<(SsoString<A>, bool), DecodeError> {
    use ::core::str;

    match read_sso_len(buf, alloc.clone())? {
        SsoLenRead::Inline { data, len } => match str::from_utf8(&data[..len]) {
            Ok(s) => Ok((pack_inline_utf8(s.as_bytes()), false)),
            Err(_) => Err(DecodeError::InvalidUtf8),
        },
        SsoLenRead::Heap(bytes) => match UnmanagedString::from_utf8(bytes) {
            Ok(s) => Ok(pack_written(s, alloc)),
            Err(bytes) => {
                // SAFETY: `alloc` owns the buffer we just built.
                unsafe { bytes.deallocate(alloc) };
                Err(DecodeError::InvalidUtf8)
            }
        },
    }
}

/// Decodes one LEN bytes payload into a packed SSO slot, preferring inline when short.
fn decode_sso_bytes_packed<B: Buf, A: Allocator + Clone>(
    buf: &mut B,
    alloc: A,
) -> Result<(SsoBytes<A>, bool), DecodeError> {
    match read_sso_len(buf, alloc.clone())? {
        SsoLenRead::Inline { data, len } => Ok((pack_inline(&data[..len]), false)),
        SsoLenRead::Heap(bytes) => Ok(pack_written_bytes(bytes, alloc)),
    }
}
