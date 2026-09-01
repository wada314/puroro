//! Unified nested-message catalog: getters are always a [`Window`] + body.
//!
//! [`SharedMessage<M, FIELD>`] is parameterized by the **owned** message `M`.
//! [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout) picks the slot:
//!
//! - [`Inline`](crate::fields::shared::value_layout::Inline): slot = [`NestedMessage::Body`]
//!   (no child `MessageCommon`). The window looks into the **parent** (`bit_base`,
//!   `child(FIELD)` unknowns).
//! - [`Boxed`](crate::fields::shared::value_layout::Boxed): slot = `UnmanagedBox<M>`.
//!   The window looks into the **child’s** own common (`bit_base = 0`).
//!
//! [`ProtoMessage`](super::proto_message::ProtoMessage) (`View = &M`) remains for
//! generated fixtures. Repeated / map elements still store a full `M` (own
//! `MessageCommon`); sample getters project [`NestedMessage::View`] /
//! [`NestedMessage::Mut`].

use crate::decode;
use crate::encode::{encode_varint, encoded_len_varint};
use crate::fields::shared::{
    CloneBound, DeallocateBound, DefaultIn, InlinedMessageParent, MessageBindingMut, Window,
    WindowMut,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::fields::wire::encode_type::EncodeType;
use crate::fields::wire::proto_ref_ops::{ProtoRefDebug, ProtoRefEq};
use crate::fields::wire::singular_type::{PayloadAccess, PayloadMerge, SingularType};
use crate::message_encode::EncodeCtx;
use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};
use ::core::marker::PhantomData;
use ::core::mem;
use ::core::ops::Deref;
use ::protobuf_core::{FieldNumber, Varint};
use ::puroro::{DecodeBuf, DecodeError, WireType};

/// Owned nested message that can bind a [`Window`] over its body.
///
/// `Alloc` is this message’s allocator (same `A` as the parent field).
pub trait NestedMessage: Sized {
    /// Allocator stored in the owned message / body field wrappers.
    type Alloc: Allocator;

    /// Field wrappers only (no [`crate::MessageCommon`]).
    type Body;

    /// Shared getter (`Window` + `&Body`). Must be `Copy`.
    type View<'a>: Copy + PartialEq + Debug
    where
        Self: 'a;

    /// Mutable getter (`WindowMut` + `&mut Body`).
    type Mut<'a>: Deref
    where
        Self: 'a;

    /// Window onto this owned message’s own common (`bit_base = 0`).
    fn as_view(&self) -> Self::View<'_>;

    /// Mutable window onto this owned message’s own common.
    fn as_mut(&mut self) -> Self::Mut<'_>;

    fn bind_view<'a>(body: &'a Self::Body, window: Window<'a, Self::Alloc>) -> Self::View<'a>
    where
        Self: 'a;

    fn bind_mut<'a>(body: &'a mut Self::Body, window: WindowMut<'a, Self::Alloc>) -> Self::Mut<'a>
    where
        Self: 'a;

    fn view_len(view: Self::View<'_>, ctx: &mut EncodeCtx) -> usize;

    fn encode_view<B: BufMut>(view: Self::View<'_>, ctx: &mut EncodeCtx, buf: &mut B);

    fn merge_inline<Ax, Buf>(
        body: &mut Self::Body,
        window: &mut WindowMut<'_, Ax>,
        buf: &mut Buf,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        Ax: Allocator + Clone,
        Buf: DecodeBuf;

    /// Number of parent bits this inlined body occupies, starting at the
    /// field's `BIT_BASE` (this message's local bits plus nested inlined
    /// children). Boxed children do not add to the parent count.
    ///
    /// After [`DeallocateBound`] on the body (which **reads** SSO / presence
    /// bits), [`clear`](PayloadAccess::clear) and a first
    /// [`with_mut`](PayloadAccess::with_mut) on an empty slot zero this range
    /// so a fresh empty-inline body is not paired with a leftover `HEAP_BIT`.
    const BIT_COUNT: usize;
}

/// Type marker for a nested message whose getters are [`NestedMessage::View`].
///
/// `M` is the **owned** message. `FIELD` is the parent field number (inline
/// unknown-store child key). `BIT_BASE` is the parent bit index of this child’s
/// local bit 0 (0 when the child has no bits, and for boxed).
pub struct SharedMessage<M, const FIELD: u32, const BIT_BASE: usize = 0>(PhantomData<fn() -> M>);

/// Clears `[bit_base, bit_base + count)` on `common` (parent indices, or a
/// [`WindowMut`] local index that already applies `bit_base`).
#[inline]
fn clear_inlined_bits<A, Cx>(common: &mut Cx, bit_base: usize, count: usize)
where
    A: Allocator,
    Cx: MessageBindingMut<A>,
{
    for i in 0..count {
        common.set_bit(bit_base + i, false);
    }
}

impl<M: NestedMessage, const FIELD: u32, const BIT_BASE: usize> SharedMessage<M, FIELD, BIT_BASE> {
    /// After an inlined oneof body has been [`DeallocateBound`], zero
    /// `[BIT_BASE, BIT_BASE + BIT_COUNT)` and drop the unknown subtree for
    /// `FIELD`.
    ///
    /// Oneof presence is always-initialized, so [`with_mut`](PayloadAccess::with_mut)
    /// never treats a newly selected variant as `fresh` and will not wipe leftover
    /// `HEAP_BIT`s itself.
    pub fn after_oneof_release<A, Cx>(common: &mut Cx)
    where
        A: Allocator + Clone,
        Cx: MessageBindingMut<A>,
    {
        let alloc = common.clone_alloc();
        clear_inlined_bits(common, BIT_BASE, M::BIT_COUNT);
        common.unknown_fields_mut().remove_child(FIELD, &alloc);
    }
}

impl<M, const FIELD: u32, const BIT_BASE: usize> Default for SharedMessage<M, FIELD, BIT_BASE> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M, const FIELD: u32, const BIT_BASE: usize> Clone for SharedMessage<M, FIELD, BIT_BASE> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M, const FIELD: u32, const BIT_BASE: usize> Copy for SharedMessage<M, FIELD, BIT_BASE> {}

impl<M: NestedMessage, const FIELD: u32, const BIT_BASE: usize> SingularType
    for SharedMessage<M, FIELD, BIT_BASE>
{
}

impl<M: NestedMessage, const FIELD: u32, const BIT_BASE: usize> EncodeType
    for SharedMessage<M, FIELD, BIT_BASE>
{
    type View<'a, A: Allocator>
        = M::View<'a>
    where
        Self: 'a,
        A: 'a,
        M: 'a;

    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn payload_len<'a, A: Allocator>(value: M::View<'a>, ctx: &mut EncodeCtx) -> usize
    where
        Self: 'a,
    {
        let n = M::view_len(value, ctx);
        encoded_len_varint(Varint::from_uint64(n as u64)) + n
    }

    #[inline]
    fn encode_payload<'a, A, Buf>(value: M::View<'a>, ctx: &mut EncodeCtx, buf: &mut Buf)
    where
        Self: 'a,
        A: Allocator,
        Buf: BufMut,
    {
        let n = M::view_len(value, ctx);
        encode_varint(Varint::from_uint64(n as u64), buf);
        M::encode_view(value, ctx, buf);
    }
}

impl<M, const FIELD: u32, const BIT_BASE: usize> PayloadAccess for SharedMessage<M, FIELD, BIT_BASE>
where
    M: NestedMessage,
{
    type Slot<A: Allocator> = M::Body;
    type Mut<'a, A: Allocator>
        = M::Mut<'a>
    where
        Self: 'a,
        A: 'a,
        M: 'a;
    type Written<A: Allocator> = M::Body;

    #[inline]
    fn is_proto_empty<A: Allocator, Cx>(_slot: &M::Body, _common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        false
    }

    #[inline]
    fn get<'a, A: Allocator + 'a, Cx>(slot: &'a M::Body, common: &'a Cx) -> M::View<'a>
    where
        Cx: MessageBindingMut<A>,
        M: 'a,
    {
        let window = common.child_window(BIT_BASE, FIELD);
        // Field `A` is `M::Alloc` at every generated call site.
        let window: Window<'a, M::Alloc> = unsafe { mem::transmute_copy(&window) };
        M::bind_view(slot, window)
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> M::Mut<'a>
    where
        A: Allocator + Clone + 'a,
        M: 'a,
        M::Body: AddressableSlot + DefaultIn<A>,
        VS: ValueSlot<M::Body, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A> + MessageBindingMut<A>,
        Self: 'a,
    {
        let fresh = !init.is_initialized(|b| common.is_bit_set(b));
        let body = ValueSlot::with_mut(slot, init, common).get_mut();
        let body = body as *mut M::Body;
        if fresh {
            clear_inlined_bits(common, BIT_BASE, M::BIT_COUNT);
        }
        let window = common.child_window_mut(BIT_BASE, FIELD);
        let window: WindowMut<'a, M::Alloc> = unsafe { mem::transmute(window) };
        // SAFETY: `body` is the field slot; `window` borrows only `common`.
        M::bind_mut(unsafe { &mut *body }, window)
    }

    #[inline]
    fn write<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, value: M::Body)
    where
        A: Allocator + Clone,
        M::Body: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<M::Body, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
            let window = common.child_window(BIT_BASE, FIELD);
            old.deallocate_bound(&window);
        }
    }

    #[inline]
    fn clear<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        A: Allocator + Clone,
        M::Body: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<M::Body, A>,
        I: SlotInitMut,
        Cx: InlinedMessageParent<A>,
    {
        let alloc = common.clone_alloc();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            let window = common.child_window(BIT_BASE, FIELD);
            old.deallocate_bound(&window);
        }
        // Presence is already clear; drop child SSO / nested presence bits so a
        // later DefaultIn body is not paired with a leftover HEAP_BIT.
        clear_inlined_bits(common, BIT_BASE, M::BIT_COUNT);
        common.unknown_fields_mut().remove_child(FIELD, &alloc);
    }

    #[inline]
    fn deallocate_payload<A, Cx>(slot: M::Body, common: &Cx)
    where
        A: Allocator,
        Cx: MessageBindingMut<A>,
        M::Body: DeallocateBound<A>,
    {
        let window = common.child_window(BIT_BASE, FIELD);
        slot.deallocate_bound(&window);
    }

    #[inline]
    fn clone_payload<A, Cx>(slot: &M::Body, common: &Cx, alloc: A) -> M::Body
    where
        A: Allocator + Clone,
        Cx: MessageBindingMut<A>,
        M::Body: CloneBound<A>,
    {
        let window = common.child_window(BIT_BASE, FIELD);
        slot.clone_bound(&window, alloc)
    }
}

impl<M: NestedMessage, const FIELD: u32, const BIT_BASE: usize> PayloadMerge
    for SharedMessage<M, FIELD, BIT_BASE>
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
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
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
        let body = ValueSlot::with_mut(slot, init, common).get_mut() as *mut M::Body;
        let mut window = common.child_window_mut(BIT_BASE, FIELD);
        // SAFETY: `body` is the field slot; `window` borrows only `common`.
        M::merge_inline(unsafe { &mut *body }, &mut window, &mut *guard, depth + 1)
    }
}

impl<M, A, const FIELD: u32, const BIT_BASE: usize> ProtoRefEq<A>
    for SharedMessage<M, FIELD, BIT_BASE>
where
    M: NestedMessage,
    A: Allocator,
{
    fn option_eq<'a>(lhs: Option<M::View<'a>>, rhs: Option<M::View<'a>>) -> bool
    where
        A: 'a,
        M: 'a,
    {
        lhs == rhs
    }
}

impl<M, A, const FIELD: u32, const BIT_BASE: usize> ProtoRefDebug<A>
    for SharedMessage<M, FIELD, BIT_BASE>
where
    M: NestedMessage,
    A: Allocator,
{
    fn fmt_ref<'a>(value: &M::View<'a>, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
        M: 'a,
    {
        Debug::fmt(value, f)
    }
}
