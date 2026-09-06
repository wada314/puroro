//! Value-storage layout for singular fields (`Inline`, `Boxed` nested message,
//! bit-packed bool, SSO string / bytes).
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
use ::core::ops::{Deref, DerefMut};
use ::protobuf_core::FieldNumber;
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::unmanaged::{CloneIn, DeallocateIn, UnmanagedBox, UnmanagedString, UnmanagedVec};

use super::{
    CloneBound, DeallocateBound, DefaultIn, MessageBindingMut,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::decode;
use crate::fields::wire::len::{BytesLikeLenCodec, LenScalar, ProtoString};
use crate::fields::wire::numerical::ProtoBool;
use crate::fields::wire::numerical::{BoolCodec, NumericalType};
use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::singular_type::{PayloadAccess, PayloadMerge, SingularType};
use crate::fields::wire::sso_buf::pack_inline;
use crate::fields::wire::sso_bytes::{SsoBytes, SsoBytesMut, pack_written as pack_written_bytes};
use crate::fields::wire::sso_string::{
    INLINE_CAP, SsoString, SsoStringMut, pack_inline_utf8, pack_written,
};
use crate::fields::wire::wire_payload::{CopyWirePayload, VarintPayload};
use crate::message_encode::MessageEncode;
use crate::message_merge::MessageMerge;

/// Where a singular field's logical value is stored.
///
/// Associated [`Slot`](Self::Slot) is the physical storage for this `(T, L)`
/// pair. Infallible views live on [`ValueLayoutGet`]; mutable `_mut` handles
/// live on [`ValueLayoutMut`].
pub trait ValueLayout<T: SingularType, A: Allocator>: Copy {
    /// Physical value stored in the singular field slot (excluding
    /// [`MessageCommon`] bits).
    type Slot: AddressableSlot;

    fn is_proto_empty<Cx>(slot: &Self::Slot, common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>;

    /// Clears the logical value and slot presence / payload.
    ///
    /// `common` must be this field's parent [`MessageCommon`] (same pairing as
    /// [`deallocate_slot`](Self::deallocate_slot)).
    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>;

    /// Message / oneof teardown for this field's value slot.
    ///
    /// `common` must be the parent [`MessageCommon`] that allocated `slot`
    /// (same instance generated `Drop` passes as `&self._common`). The type
    /// system does not prove this pairing; a different message's common is
    /// unsound. SSO layouts also read the heap bit from this `common`.
    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, common: &Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>;
}

/// Infallible slot view. Kept off [`ValueLayout`] so layouts that need an
/// island buffer ([`WireOrSso`](crate::WireOrSso)) promote via `try_str` /
/// `try_bytes` instead of returning a dummy empty view.
pub trait ValueLayoutGet<T: SingularType, A: Allocator>: ValueLayout<T, A> {
    fn get<'a, Cx>(slot: &'a Self::Slot, common: &'a Cx) -> T::View<'a, A>
    where
        Cx: MessageBindingMut<A>;
}

/// Wire-decode merge for a layout. Kept off [`ValueLayout`] so nested-message
/// [`Inline`] naming does not require [`crate::MessageMerge`].
pub trait ValueLayoutMerge<T: SingularType, A: Allocator>: ValueLayout<T, A> {
    fn merge<VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
        A: Clone,
        Self::Slot: DefaultIn<A>;
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
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, common: &Cx, alloc: A) -> VS
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>;
}

/// `_mut` handle for a layout. Kept off [`ValueLayout`] so read-only layouts
/// ([`WireOrSso`](crate::WireOrSso)) do not provide a placeholder mutator.
pub trait ValueLayoutMut<T: SingularType, A: Allocator>: ValueLayout<T, A> {
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

    fn with_mut<'a, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        T: 'a,
        A: 'a + Clone,
        Self::Slot: DefaultIn<A>;
}

/// Value lives in the field slot payload ([`PayloadAccess::Slot`]).
///
/// For [`ProtoMessage`](crate::ProtoMessage) this is the nested message `M`
/// itself (inlined child). Heap boxing uses [`Boxed`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Inline;

/// Nested message stored behind [`UnmanagedBox`](::unmanaged::UnmanagedBox).
///
/// Singular fields pair this with [`Message`](super::field_presence::Message)
/// presence (`Option<UnmanagedBox<M, A>>`). Oneof variants use
/// [`Oneof`](super::field_presence::Oneof) (always-present box). Contrast
/// [`Inline`], which stores `M` in the slot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Boxed;

impl<T: PayloadAccess, A: Allocator> ValueLayout<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DeallocateBound<A>,
{
    type Slot = T::Slot<A>;

    #[inline]
    fn is_proto_empty<Cx>(slot: &T::Slot<A>, common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        T::is_proto_empty(slot, common)
    }

    #[inline]
    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        T::Slot<A>: DefaultIn<A>,
    {
        T::clear(slot, init, common);
    }

    #[inline]
    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, common: &Cx)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        Cx: MessageBindingMut<A>,
    {
        if let Some(v) = slot.take_value(initialized) {
            T::deallocate_payload(v, common);
        }
    }
}

impl<T: PayloadAccess, A: Allocator> ValueLayoutGet<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DeallocateBound<A>,
{
    #[inline]
    fn get<'a, Cx>(slot: &'a T::Slot<A>, common: &'a Cx) -> T::View<'a, A>
    where
        Cx: MessageBindingMut<A>,
    {
        T::get(slot, common)
    }
}

impl<T: PayloadAccess, A: Allocator> ValueLayoutMut<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DeallocateBound<A>,
{
    type Mut<'a>
        = T::Mut<'a, A>
    where
        T: 'a,
        A: 'a;

    #[inline]
    fn with_mut<'a, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> T::Mut<'a, A>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        T: 'a,
        A: 'a + Clone,
        T::Slot<A>: DefaultIn<A>,
    {
        T::with_mut(slot, init, common)
    }
}

impl<T: PayloadAccess, A: Allocator + Clone> ValueLayoutClone<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + CloneBound<A> + DeallocateBound<A>,
{
    #[inline]
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, common: &Cx, alloc: A) -> VS
    where
        VS: ValueSlot<T::Slot<A>, A>,
        Cx: MessageBindingMut<A>,
    {
        VS::from_optional(
            slot.get_value(initialized)
                .map(|v| T::clone_payload(v, common, alloc)),
        )
    }
}

impl<T: PayloadMerge, A: Allocator> ValueLayoutMerge<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DeallocateBound<A>,
{
    #[inline]
    fn merge<VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
        A: Clone,
        T::Slot<A>: DefaultIn<A>,
    {
        T::merge(slot, init, common, wire_type, buf, field, depth)
    }
}

impl<M, A: Allocator> ValueLayout<ProtoMessage<M>, A> for Boxed
where
    M: MessageEncode + DeallocateIn<A>,
{
    type Slot = UnmanagedBox<M, A>;

    #[inline]
    fn is_proto_empty<Cx>(_slot: &UnmanagedBox<M, A>, _common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        false
    }

    #[inline]
    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        UnmanagedBox<M, A>: DefaultIn<A>,
    {
        let alloc = common.clone_alloc();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            // SAFETY: `clear` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, &alloc) };
        }
    }

    #[inline]
    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, common: &Cx)
    where
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        Cx: MessageBindingMut<A>,
    {
        if let Some(v) = slot.take_value(initialized) {
            // SAFETY: `deallocate_slot` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(v, common.alloc()) };
        }
    }
}

impl<M, A: Allocator> ValueLayoutGet<ProtoMessage<M>, A> for Boxed
where
    M: MessageEncode + DeallocateIn<A>,
{
    #[inline]
    fn get<'a, Cx>(slot: &'a UnmanagedBox<M, A>, _common: &'a Cx) -> &'a M
    where
        Cx: MessageBindingMut<A>,
    {
        Deref::deref(slot)
    }
}

impl<M, A: Allocator> ValueLayoutMut<ProtoMessage<M>, A> for Boxed
where
    M: MessageEncode + DeallocateIn<A>,
{
    type Mut<'a>
        = &'a mut M
    where
        M: 'a,
        A: 'a;

    #[inline]
    fn with_mut<'a, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> &'a mut M
    where
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        M: 'a,
        A: 'a + Clone,
        UnmanagedBox<M, A>: DefaultIn<A>,
    {
        DerefMut::deref_mut(ValueSlot::with_mut(slot, init, common).get_mut())
    }
}

impl<M, A: Allocator> ValueLayoutMerge<ProtoMessage<M>, A> for Boxed
where
    M: MessageEncode + MessageMerge + DeallocateIn<A>,
{
    #[inline]
    fn merge<VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        _field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
        A: Clone,
        UnmanagedBox<M, A>: DefaultIn<A>,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        let mut guard = buf.push_limit_guard(len)?;
        let child = ValueSlot::with_mut(slot, init, common).get_mut();
        MessageMerge::merge_from_with_depth(DerefMut::deref_mut(child), &mut *guard, depth + 1)
    }
}

impl<M, A: Allocator + Clone> ValueLayoutClone<ProtoMessage<M>, A> for Boxed
where
    M: MessageEncode + DeallocateIn<A> + CloneIn<A>,
{
    #[inline]
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, _common: &Cx, alloc: A) -> VS
    where
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        Cx: MessageBindingMut<A>,
    {
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

impl<A: Allocator, const VALUE_BIT: usize> ValueLayout<ProtoBool, A> for BitPacked<VALUE_BIT> {
    type Slot = ();

    #[inline]
    fn is_proto_empty<Cx>(_slot: &(), common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        !common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        common.set_bit(VALUE_BIT, false);
        let _ = ValueSlot::with_mut(slot, init, common).take_clear();
    }

    #[inline]
    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, _common: &Cx)
    where
        VS: ValueSlot<(), A>,
        Cx: MessageBindingMut<A>,
    {
        let _ = slot.take_value(initialized);
    }
}

impl<A: Allocator, const VALUE_BIT: usize> ValueLayoutGet<ProtoBool, A> for BitPacked<VALUE_BIT> {
    #[inline]
    fn get<'a, Cx>(_slot: &'a (), common: &'a Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        common.is_bit_set(VALUE_BIT)
    }
}

impl<A: Allocator, const VALUE_BIT: usize> ValueLayoutMut<ProtoBool, A> for BitPacked<VALUE_BIT> {
    type Mut<'a>
        = BitRef<'a, Mut, u8, Lsb0>
    where
        A: 'a;

    #[inline]
    fn with_mut<'a, VS, I, Cx>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut Cx,
    ) -> BitRef<'a, Mut, u8, Lsb0>
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        ProtoBool: 'a,
        A: 'a + Clone,
        Self::Slot: DefaultIn<A>,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.bit_mut(VALUE_BIT)
    }
}

impl<A: Allocator, const VALUE_BIT: usize> ValueLayoutMerge<ProtoBool, A> for BitPacked<VALUE_BIT> {
    #[inline]
    fn merge<VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut B,
        field: FieldNumber,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        match BoolCodec::from_wire_body(VarintPayload::decode(wire_type, buf)?) {
            Ok(new) => {
                let _ = ValueSlot::with_mut(slot, init, common).get_mut();
                common.set_bit(VALUE_BIT, new);
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

impl<A: Allocator + Clone, const VALUE_BIT: usize> ValueLayoutClone<ProtoBool, A>
    for BitPacked<VALUE_BIT>
{
    #[inline]
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, _common: &Cx, _alloc: A) -> VS
    where
        VS: ValueSlot<(), A>,
        Cx: MessageBindingMut<A>,
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
    fn is_heap<Cx, A: Allocator>(common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        common.is_bit_set(HEAP_BIT)
    }

    /// Writes `HEAP_BIT` (`is_heap == `[`SSO_HEAP`] / [`SSO_INLINE`]).
    #[inline]
    fn set_heap<Cx, A: Allocator>(common: &mut Cx, is_heap: bool)
    where
        Cx: MessageBindingMut<A>,
    {
        common.set_bit(HEAP_BIT, is_heap);
    }
}

impl<A: Allocator, const HEAP_BIT: usize> ValueLayout<ProtoString, A> for InlineOrHeap<HEAP_BIT> {
    type Slot = SsoString<A>;

    #[inline]
    fn is_proto_empty<Cx>(slot: &SsoString<A>, common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        slot.is_empty(Self::is_heap(common))
    }

    #[inline]
    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<SsoString<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if !init.is_initialized(|b| common.is_bit_set(b)) {
            Self::set_heap(common, SSO_INLINE);
            return;
        }
        let old_is_heap = Self::is_heap(common);
        let alloc = common.clone_alloc();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            // SAFETY: HEAP_BIT matches the live arm.
            unsafe {
                s.replace_packed(SsoString::empty_inline(), SSO_INLINE, old_is_heap, &alloc);
            }
        }
        Self::set_heap(common, SSO_INLINE);
        // Presence clear is a no-op for always-initialized slots.
        init.set_initialized(|b, v| common.set_bit(b, v), false);
    }

    #[inline]
    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, common: &Cx)
    where
        VS: ValueSlot<SsoString<A>, A>,
        Cx: MessageBindingMut<A>,
    {
        let is_heap = Self::is_heap(common);
        if let Some(s) = slot.take_value(initialized) {
            // SAFETY: HEAP_BIT / `alloc` come from this field's parent `common`.
            unsafe { s.deallocate(is_heap, common.alloc()) };
        }
    }
}

impl<A: Allocator, const HEAP_BIT: usize> ValueLayoutGet<ProtoString, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn get<'a, Cx>(slot: &'a SsoString<A>, common: &'a Cx) -> &'a str
    where
        Cx: MessageBindingMut<A>,
    {
        slot.as_str(Self::is_heap(common))
    }
}

impl<A: Allocator, const HEAP_BIT: usize> ValueLayoutMut<ProtoString, A>
    for InlineOrHeap<HEAP_BIT>
{
    type Mut<'a>
        = SsoStringMut<'a, A>
    where
        A: 'a;

    #[inline]
    fn with_mut<'a, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> SsoStringMut<'a, A>
    where
        VS: ValueSlot<SsoString<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        ProtoString: 'a,
        A: 'a + Clone,
        Self::Slot: DefaultIn<A>,
    {
        let alloc = common.clone_alloc();
        // Ensure presence/init, then split the slot pointer from `common` so we
        // can hand out both `&mut SsoString` and a heap-bit `BitRef`.
        let slot_ptr: *mut SsoString<A> = ValueSlot::with_mut(slot, init, common).get_mut();
        let tag = common.bit_mut(HEAP_BIT);
        // SAFETY: `tag` borrows only the bitfield in `common`; `slot_ptr` is the
        // distinct field payload and remains valid for `'a`.
        SsoStringMut::new(unsafe { &mut *slot_ptr }, tag, alloc)
    }
}

impl<A: Allocator, const HEAP_BIT: usize> ValueLayoutMerge<ProtoString, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn merge<VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut B,
        _field: FieldNumber,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<SsoString<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let (new, new_is_heap) = decode_sso_packed(buf, common.clone_alloc())?;
        let old_is_heap = Self::is_heap(common);
        let alloc = common.clone_alloc();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            // SAFETY: message allocator owns any previous heap buffer; bit matches arm.
            unsafe { s.replace_packed(new, new_is_heap, old_is_heap, &alloc) };
        }
        Self::set_heap(common, new_is_heap);
        Ok(())
    }
}

impl<A: Allocator + Clone, const HEAP_BIT: usize> ValueLayoutClone<ProtoString, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, common: &Cx, alloc: A) -> VS
    where
        VS: ValueSlot<SsoString<A>, A>,
        Cx: MessageBindingMut<A>,
    {
        let is_heap = Self::is_heap(common);
        VS::from_optional(
            slot.get_value(initialized)
                .map(|s| s.clone_packed(is_heap, alloc).0),
        )
    }
}

impl<A: Allocator, const HEAP_BIT: usize, C: BytesLikeLenCodec> ValueLayout<LenScalar<C>, A>
    for InlineOrHeap<HEAP_BIT>
{
    type Slot = SsoBytes<A>;

    #[inline]
    fn is_proto_empty<Cx>(slot: &SsoBytes<A>, common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        slot.is_empty(Self::is_heap(common))
    }

    #[inline]
    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<SsoBytes<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if !init.is_initialized(|b| common.is_bit_set(b)) {
            Self::set_heap(common, SSO_INLINE);
            return;
        }
        let old_is_heap = Self::is_heap(common);
        let alloc = common.clone_alloc();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            // SAFETY: HEAP_BIT matches the live arm.
            unsafe {
                s.replace_packed(SsoBytes::empty_inline(), SSO_INLINE, old_is_heap, &alloc);
            }
        }
        Self::set_heap(common, SSO_INLINE);
        init.set_initialized(|b, v| common.set_bit(b, v), false);
    }

    #[inline]
    fn deallocate_slot<VS, Cx>(slot: VS, initialized: bool, common: &Cx)
    where
        VS: ValueSlot<SsoBytes<A>, A>,
        Cx: MessageBindingMut<A>,
    {
        let is_heap = Self::is_heap(common);
        if let Some(s) = slot.take_value(initialized) {
            // SAFETY: HEAP_BIT / `alloc` come from this field's parent `common`.
            unsafe { s.deallocate(is_heap, common.alloc()) };
        }
    }
}

impl<A: Allocator, const HEAP_BIT: usize, C: BytesLikeLenCodec> ValueLayoutGet<LenScalar<C>, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn get<'a, Cx>(slot: &'a SsoBytes<A>, common: &'a Cx) -> &'a [u8]
    where
        Cx: MessageBindingMut<A>,
    {
        slot.as_bytes(Self::is_heap(common))
    }
}

impl<A: Allocator, const HEAP_BIT: usize, C: BytesLikeLenCodec> ValueLayoutMut<LenScalar<C>, A>
    for InlineOrHeap<HEAP_BIT>
{
    type Mut<'a>
        = SsoBytesMut<'a, A>
    where
        A: 'a,
        C: 'a;

    #[inline]
    fn with_mut<'a, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> SsoBytesMut<'a, A>
    where
        VS: ValueSlot<SsoBytes<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        LenScalar<C>: 'a,
        A: 'a + Clone,
        Self::Slot: DefaultIn<A>,
    {
        let alloc = common.clone_alloc();
        let slot_ptr: *mut SsoBytes<A> = ValueSlot::with_mut(slot, init, common).get_mut();
        let tag = common.bit_mut(HEAP_BIT);
        // SAFETY: `tag` borrows only the bitfield in `common`; `slot_ptr` is the
        // distinct field payload and remains valid for `'a`.
        SsoBytesMut::new(unsafe { &mut *slot_ptr }, tag, alloc)
    }
}

impl<A: Allocator, const HEAP_BIT: usize, C: BytesLikeLenCodec> ValueLayoutMerge<LenScalar<C>, A>
    for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn merge<VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut B,
        _field: FieldNumber,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<SsoBytes<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let (new, new_is_heap) = decode_sso_bytes_packed(buf, common.clone_alloc())?;
        let old_is_heap = Self::is_heap(common);
        let alloc = common.clone_alloc();
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            // SAFETY: message allocator owns any previous heap buffer; bit matches arm.
            unsafe { s.replace_packed(new, new_is_heap, old_is_heap, &alloc) };
        }
        Self::set_heap(common, new_is_heap);
        Ok(())
    }
}

impl<A: Allocator + Clone, const HEAP_BIT: usize, C: BytesLikeLenCodec>
    ValueLayoutClone<LenScalar<C>, A> for InlineOrHeap<HEAP_BIT>
{
    #[inline]
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, common: &Cx, alloc: A) -> VS
    where
        VS: ValueSlot<SsoBytes<A>, A>,
        Cx: MessageBindingMut<A>,
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
                unsafe { bytes.deallocate(&alloc) };
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
