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

use ::allocator_api2::alloc::Allocator;
use ::core::marker::PhantomData;
use ::protobuf_core::FieldNumber;

use ::puroro::{DecodeBuf, DecodeError, WireType};

use crate::decode;
use crate::message_encode::MessageEncode;
use crate::message_merge::MessageMerge;
use ::unmanaged::DeallocateIn;

use crate::fields::shared::{
    DefaultIn, MessageCommon, MessageCommonBits,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::fields::wire::singular_type::{PayloadAccess, PayloadMerge, SingularType};

/// Type marker for a singular nested message `M`.
///
/// [`PayloadAccess::Slot`] is `M` (inlined). Heap boxing is
/// [`Boxed`](crate::fields::shared::value_layout::Boxed).
pub struct ProtoMessage<M>(PhantomData<fn() -> M>);

impl<M> Default for ProtoMessage<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M> Clone for ProtoMessage<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for ProtoMessage<M> {}

impl<M: MessageEncode> SingularType for ProtoMessage<M> {}

impl<M: MessageEncode> PayloadAccess for ProtoMessage<M> {
    type Slot<A: Allocator> = M;
    type Mut<'a, A: Allocator>
        = &'a mut M
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator> = M;

    #[inline]
    fn is_proto_empty<A: Allocator, Pb>(_slot: &M, _common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        false
    }

    #[inline]
    fn get<'a, A: Allocator + 'a, Pb>(slot: &'a M, _common: &'a MessageCommon<Pb, A>) -> &'a M
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        slot
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> &'a mut M
    where
        A: Allocator + Clone + 'a,
        M: AddressableSlot + DefaultIn<A>,
        VS: ValueSlot<M, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        Self: 'a,
    {
        ValueSlot::with_mut(slot, init, common).get_mut()
    }

    #[inline]
    fn write<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>, value: M)
    where
        A: Allocator + Clone,
        M: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<M, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let alloc = common.alloc.clone();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).replace(value) {
            // SAFETY: `write` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, &alloc) };
        }
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        M: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<M, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let alloc = common.alloc.clone();
        if let Some(old) = ValueSlot::with_mut(slot, init, common).take_clear() {
            // SAFETY: `clear` contract — `common` is this field's parent.
            unsafe { DeallocateIn::deallocate_in(old, &alloc) };
        }
    }
}

impl<M: MessageEncode + MessageMerge> PayloadMerge for ProtoMessage<M> {
    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        _field: FieldNumber,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        Self::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<Self::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
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
