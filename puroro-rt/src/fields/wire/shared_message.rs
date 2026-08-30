//! Body-only nested message catalog (`Slot` = body, `View`/`Mut` = window + body).
//!
//! [`ProtoMessage`](super::proto_message::ProtoMessage) still stores a full `M`
//! (`Slot = M`, `View = &M`). This marker is the shared-`MessageCommon` path:
//! the parent slot holds only the field body; getters bind a [`Window`].

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};
use ::core::marker::PhantomData;
use ::core::ops::Deref;
use ::protobuf_core::{FieldNumber, Varint};
use ::puroro::{DecodeBuf, DecodeError, WireType};
use ::unmanaged::DeallocateIn;

use crate::decode;
use crate::encode::{encode_varint, encoded_len_varint};
use crate::fields::shared::{
    DefaultIn, InlinedMessageParent, MessageBindingMut, Window, WindowMut,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::fields::wire::encode_type::EncodeType;
use crate::fields::wire::proto_ref_ops::{ProtoRefDebug, ProtoRefEq};
use crate::fields::wire::singular_type::{PayloadAccess, PayloadMerge, SingularType};
use crate::message_encode::EncodeCtx;

/// Field body of an inlined (or later boxed) nested message.
///
/// Implemented by hand-written / generated `*Body` types. The owned message
/// (`Point`) keeps its own [`crate::MessageCommon`]; views bind through a window.
pub trait SharedMessageBody: Sized {
    /// Shared getter view (`Window` + `&Body`). Must be `Copy`.
    type View<'a>: Copy + PartialEq + Debug
    where
        Self: 'a;

    /// Mutable getter view (`WindowMut` + `&mut Body`).
    type Mut<'a>: Deref
    where
        Self: 'a;

    fn bind_view<'a, Ax: Allocator>(body: &'a Self, window: Window<'a, Ax>) -> Self::View<'a>
    where
        Self: 'a;

    fn bind_mut<'a, Ax: Allocator>(body: &'a mut Self, window: WindowMut<'a, Ax>) -> Self::Mut<'a>
    where
        Self: 'a;

    fn body_len(view: Self::View<'_>, ctx: &mut EncodeCtx) -> usize;

    fn encode_body<B: BufMut>(view: Self::View<'_>, ctx: &mut EncodeCtx, buf: &mut B);

    fn merge_from<Ax, Buf>(
        body: &mut Self,
        window: &mut WindowMut<'_, Ax>,
        buf: &mut Buf,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        Ax: Allocator + Clone,
        Buf: DecodeBuf;

    fn default_in<Ax: Allocator + Clone>(alloc: Ax) -> Self;

    /// Releases field payloads. Does not run a child [`crate::MessageCommon::deallocate`].
    ///
    /// # Safety
    ///
    /// `alloc` must own every unmanaged buffer in `body`.
    unsafe fn deallocate_in<Ax: Allocator>(body: Self, alloc: &Ax);
}

/// Type marker for a shared-common nested message whose slot is `B` (body only).
///
/// `FIELD` is the parent field number (unknown-store child key). `BIT_BASE` is
/// the parent bit index of this child's local bit 0 (0 when the child has no bits).
pub struct SharedMessage<B, const FIELD: u32, const BIT_BASE: usize = 0>(PhantomData<fn() -> B>);

impl<B, const FIELD: u32, const BIT_BASE: usize> Default for SharedMessage<B, FIELD, BIT_BASE> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<B, const FIELD: u32, const BIT_BASE: usize> Clone for SharedMessage<B, FIELD, BIT_BASE> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B, const FIELD: u32, const BIT_BASE: usize> Copy for SharedMessage<B, FIELD, BIT_BASE> {}

impl<B: SharedMessageBody, const FIELD: u32, const BIT_BASE: usize> SingularType
    for SharedMessage<B, FIELD, BIT_BASE>
{
}

impl<B: SharedMessageBody, const FIELD: u32, const BIT_BASE: usize> EncodeType
    for SharedMessage<B, FIELD, BIT_BASE>
{
    type View<'a, A: Allocator>
        = B::View<'a>
    where
        Self: 'a,
        A: 'a,
        B: 'a;

    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn payload_len<'a, A: Allocator>(value: B::View<'a>, ctx: &mut EncodeCtx) -> usize
    where
        Self: 'a,
    {
        let n = B::body_len(value, ctx);
        encoded_len_varint(Varint::from_uint64(n as u64)) + n
    }

    #[inline]
    fn encode_payload<'a, A, Buf>(value: B::View<'a>, ctx: &mut EncodeCtx, buf: &mut Buf)
    where
        Self: 'a,
        A: Allocator,
        Buf: BufMut,
    {
        let n = B::body_len(value, ctx);
        encode_varint(Varint::from_uint64(n as u64), buf);
        B::encode_body(value, ctx, buf);
    }
}

impl<B, const FIELD: u32, const BIT_BASE: usize> PayloadAccess for SharedMessage<B, FIELD, BIT_BASE>
where
    B: SharedMessageBody,
{
    type Slot<A: Allocator> = B;
    type Mut<'a, A: Allocator>
        = B::Mut<'a>
    where
        Self: 'a,
        A: 'a,
        B: 'a;
    type Written<A: Allocator> = B;

    #[inline]
    fn is_proto_empty<A: Allocator, Cx>(_slot: &B, _common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        false
    }

    #[inline]
    fn get<'a, A: Allocator + 'a, Cx>(slot: &'a B, common: &'a Cx) -> B::View<'a>
    where
        Cx: MessageBindingMut<A>,
        B: 'a,
    {
        B::bind_view(slot, common.child_window(BIT_BASE, FIELD))
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> B::Mut<'a>
    where
        A: Allocator + Clone + 'a,
        B: AddressableSlot + DefaultIn<A> + 'a,
        VS: ValueSlot<B, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A> + MessageBindingMut<A>,
        Self: 'a,
    {
        let body = ValueSlot::with_mut(slot, init, common).get_mut();
        // Slot and common are disjoint; `get_mut` ties both to `'a`.
        let body = body as *mut B;
        let window = common.child_window_mut(BIT_BASE, FIELD);
        // SAFETY: `body` is the field slot; `window` borrows only `common`.
        B::bind_mut(unsafe { &mut *body }, window)
    }

    #[inline]
    fn write<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, value: B)
    where
        A: Allocator + Clone,
        B: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<B, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        let alloc = common.clone_alloc();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
            // SAFETY: `write` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, &alloc) };
        }
    }

    #[inline]
    fn clear<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        A: Allocator + Clone,
        B: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<B, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        let alloc = common.clone_alloc();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            // SAFETY: `clear` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, &alloc) };
        }
        common.unknown_fields_mut().remove_child(FIELD, &alloc);
    }
}

impl<B: SharedMessageBody, const FIELD: u32, const BIT_BASE: usize> PayloadMerge
    for SharedMessage<B, FIELD, BIT_BASE>
{
    fn merge<A, VS, I, Cx, Buf>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut Buf,
        _field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
        Buf: DecodeBuf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        let mut guard = buf.push_limit_guard(len)?;
        let body = ValueSlot::with_mut(slot, init, common).get_mut() as *mut B;
        let mut window = common.child_window_mut(BIT_BASE, FIELD);
        // SAFETY: `body` is the field slot; `window` borrows only `common`.
        B::merge_from(unsafe { &mut *body }, &mut window, &mut *guard, depth + 1)
    }
}

impl<B, A, const FIELD: u32, const BIT_BASE: usize> ProtoRefEq<A>
    for SharedMessage<B, FIELD, BIT_BASE>
where
    B: SharedMessageBody,
    A: Allocator,
{
    fn option_eq<'a>(lhs: Option<B::View<'a>>, rhs: Option<B::View<'a>>) -> bool
    where
        A: 'a,
        B: 'a,
    {
        lhs == rhs
    }
}

impl<B, A, const FIELD: u32, const BIT_BASE: usize> ProtoRefDebug<A>
    for SharedMessage<B, FIELD, BIT_BASE>
where
    B: SharedMessageBody,
    A: Allocator,
{
    fn fmt_ref<'a>(value: &B::View<'a>, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
        B: 'a,
    {
        Debug::fmt(value, f)
    }
}
