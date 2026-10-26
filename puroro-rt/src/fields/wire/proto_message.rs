//! Nested-message type marker for [`SingularType`](super::singular_type::SingularType).
//!
//! [`ProtoMessage`] supplies merge-into wire semantics. Physical storage is
//! chosen by [`ValueLayout`](crate::fields::shared::value_layout::ValueLayout):
//! [`Inline`](crate::fields::shared::value_layout::Inline) stores `M` in the
//! slot (inlined child); [`Boxed`](crate::fields::shared::value_layout::Boxed)
//! stores [`UnmanagedBox<M, A>`](::unmanaged::UnmanagedBox). Presence policy
//! (`Message` / `Explicit` / `Oneof`) lives on
//! [`FieldPresence`](crate::fields::shared::field_presence::FieldPresence).
//!
//! The marker is allocator-free; `M` typically still mentions `A` (e.g.
//! `Address<A>`). Singular fields pass the same `A` so the box allocator matches.
//!
//! Wire framing is selected by the second type parameter: [`LengthPrefixed`]
//! (default, `LEN`) or [`GroupDelimited`] (`SGroup` … `EGroup`). Alias
//! [`ProtoGroup`] for the latter.

use ::allocator_api2::alloc::Allocator;
use ::core::marker::PhantomData;
use ::protobuf_core::FieldNumber;

use ::puroro::{DecodeBuf, DecodeError, ScopedBuf, WireType};

use crate::decode;
use crate::fields::shared::{
    CloneBound, DeallocateBound, DefaultIn, MessageBindingMut,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::fields::wire::singular_type::{PayloadAccess, PayloadMerge, SingularType};
use crate::message_encode::MessageEncode;
use crate::message_merge::MessageMerge;

/// Length-prefixed nested message framing (`WireType::Len`).
#[derive(Debug, Default, Clone, Copy)]
pub struct LengthPrefixed;

/// Group-delimited nested message framing (`SGroup` … `EGroup`).
#[derive(Debug, Default, Clone, Copy)]
pub struct GroupDelimited;

/// Marker bound for [`ProtoMessage`]'s framing parameter.
pub trait MessageFraming: 'static + Copy + Default {}

impl MessageFraming for LengthPrefixed {}
impl MessageFraming for GroupDelimited {}

/// Type marker for a singular nested message `M`.
///
/// [`PayloadAccess::Slot`] is `M` (inlined). Heap boxing is
/// [`Boxed`](crate::fields::shared::value_layout::Boxed).
///
/// `W` selects wire framing ([`LengthPrefixed`] by default, or
/// [`GroupDelimited`] / [`ProtoGroup`]).
pub struct ProtoMessage<M, W: MessageFraming = LengthPrefixed>(PhantomData<(fn() -> M, W)>);

/// Nested message with group start/end framing (classic `TYPE_GROUP` /
/// Editions `DELIMITED`).
pub type ProtoGroup<M> = ProtoMessage<M, GroupDelimited>;

impl<M, W: MessageFraming> Default for ProtoMessage<M, W> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M, W: MessageFraming> Clone for ProtoMessage<M, W> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M, W: MessageFraming> Copy for ProtoMessage<M, W> {}

macro_rules! impl_payload_access_proto_message {
    ($W:ty) => {
        impl<M: MessageEncode> SingularType for ProtoMessage<M, $W> {}

        impl<M: MessageEncode> PayloadAccess for ProtoMessage<M, $W> {
            type Slot<A: Allocator> = M;
            type Mut<'a, A: Allocator>
                = &'a mut M
            where
                Self: 'a,
                A: 'a;
            type Written<A: Allocator> = M;

            #[inline]
            fn is_proto_empty<A: Allocator, Cx>(_slot: &M, _common: &Cx) -> bool
            where
                Cx: MessageBindingMut<A>,
            {
                false
            }

            #[inline]
            fn get<'a, A: Allocator + 'a, Cx>(slot: &'a M, _common: &'a Cx) -> &'a M
            where
                Cx: MessageBindingMut<A>,
            {
                slot
            }

            #[inline]
            fn with_mut<'a, A, VS, I, Cx>(
                slot: &'a mut VS,
                init: I,
                common: &'a mut Cx,
            ) -> &'a mut M
            where
                A: Allocator + Clone + 'a,
                M: AddressableSlot + DefaultIn<A>,
                VS: ValueSlot<M, A>,
                I: SlotInitMut,
                Cx: MessageBindingMut<A>,
                Self: 'a,
            {
                ValueSlot::with_mut(slot, init, common).get_mut()
            }

            #[inline]
            fn write<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, value: M)
            where
                A: Allocator + Clone,
                M: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
                VS: ValueSlot<M, A>,
                I: SlotInitMut,
                Cx: MessageBindingMut<A>,
            {
                if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
                    old.deallocate_bound(common);
                }
            }

            #[inline]
            fn clear<A, VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
            where
                A: Allocator + Clone,
                M: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
                VS: ValueSlot<M, A>,
                I: SlotInitMut,
                Cx: MessageBindingMut<A>,
            {
                if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
                    old.deallocate_bound(common);
                }
            }

            #[inline]
            fn deallocate_payload<A, Cx>(slot: M, common: &Cx)
            where
                A: Allocator,
                Cx: MessageBindingMut<A>,
                M: DeallocateBound<A>,
            {
                slot.deallocate_bound(common);
            }

            #[inline]
            fn clone_payload<A, Cx>(slot: &M, common: &Cx, alloc: A) -> M
            where
                A: Allocator + Clone,
                Cx: MessageBindingMut<A>,
                M: CloneBound<A>,
            {
                slot.clone_bound(common, alloc)
            }
        }
    };
}

impl_payload_access_proto_message!(LengthPrefixed);
impl_payload_access_proto_message!(GroupDelimited);

impl<M: MessageEncode + MessageMerge> PayloadMerge for ProtoMessage<M, LengthPrefixed> {
    fn merge<A, VS, I, Cx, B>(
        slot: &mut VS,
        init: I,
        common: &mut Cx,
        wire_type: WireType,
        buf: &mut B,
        _field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateBound<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        // Same `DecodeBuf` type at every depth: push a LEN scope instead of
        // `Take<…>` monomorphization or `copy_to_bytes`.
        let mut guard = buf.push_limit_guard(len)?;
        let child = ValueSlot::with_mut(slot, init, common).get_mut();
        MessageMerge::merge_from_with_depth(child, &mut *guard, depth + 1)
    }
}

impl<M: MessageEncode + MessageMerge> PayloadMerge for ProtoMessage<M, GroupDelimited> {
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
        Cx: MessageBindingMut<A>,
        B: DecodeBuf,
    {
        if wire_type != WireType::SGroup {
            return Err(DecodeError::InvalidTag);
        }
        // Copy body then merge: same `take_group_body` path as unknown preserve.
        let mut body = ::allocator_api2::vec::Vec::new_in(common.clone_alloc());
        decode::take_group_body(field, buf, &mut body)?;
        let mut slice: &[u8] = body.as_slice();
        let mut scoped = ScopedBuf::new(&mut slice);
        let child = ValueSlot::with_mut(slot, init, common).get_mut();
        MessageMerge::merge_from_with_depth(child, &mut scoped, depth + 1)
    }
}
