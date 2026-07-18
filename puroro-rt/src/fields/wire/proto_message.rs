//! Nested-message type marker for [`ProtoType`](super::proto_type::ProtoType).
//!
//! [`ProtoMessage`] supplies merge-into wire semantics; physical storage is
//! [`UnmanagedBox<M, A>`]. Presence policy (`NonOneof` / `Oneof`) lives on
//! [`FieldPresence`](crate::fields::shared::field_presence::FieldPresence).
//!
//! The marker is allocator-free; `M` typically still mentions `A` (e.g.
//! `Address<A>`). Singular fields pass the same `A` so the box allocator matches.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::marker::PhantomData;
use ::core::ops::{Deref, DerefMut};
use ::unmanaged::UnmanagedBox;

use ::puroro::{DecodeError, Message, WireType};

use crate::decode;
use crate::encode;
use crate::fields::shared::{
    DeallocateIn, DefaultIn, MessageCommon, PresenceBits, ProtoEmpty,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::fields::wire::len;
use crate::fields::wire::proto_type::{PayloadAccess, ProtoType};

/// Type marker for a singular nested message `M`.
///
/// Physical slot is [`UnmanagedBox<M, A>`] via [`ProtoType::Slot`].
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

impl<M: Message<Alloc = A>, A: Allocator + Clone> DefaultIn<A> for UnmanagedBox<M, A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        UnmanagedBox::new_in(M::new_in(alloc.clone()), alloc)
    }
}

impl<M: Message<Alloc = A>, A: Allocator + Clone> DeallocateIn<A> for UnmanagedBox<M, A> {
    #[inline]
    unsafe fn deallocate_in(self, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { self.deallocate(alloc) };
    }
}

impl<M, A: Allocator> ProtoEmpty for UnmanagedBox<M, A> {
    /// A present nested message is never omitted for being "empty"; absence is
    /// expressed by the [`Option`](core::option::Option) / init layer.
    #[inline]
    fn is_proto_empty(&self) -> bool {
        false
    }
}

impl<M, A: Allocator> AddressableSlot for UnmanagedBox<M, A> {}

impl<M: Message> ProtoType for ProtoMessage<M> {
    type Slot<A: Allocator + Clone> = UnmanagedBox<M, A>;
    type Ref<'a, A: Allocator + Clone>
        = &'a M
    where
        Self: 'a,
        A: 'a;
    type Mut<'a, A: Allocator + Clone>
        = &'a mut M
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone> = UnmanagedBox<M, A>;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn encoded_len<'a, A: Allocator + Clone>(value: &'a M, field: u32) -> usize
    where
        Self: 'a,
        A: 'a,
    {
        encode::encoded_len_len_field(field, value.encoded_len())
    }

    #[inline]
    fn encode<'a, A: Allocator + Clone, B: BufMut>(value: &'a M, field: u32, buf: &mut B)
    where
        Self: 'a,
        A: 'a,
    {
        let payload_len = value.encoded_len();
        encode::encode_tag(field, WireType::Len, buf);
        encode::encode_varint(payload_len as u64, buf);
        value.encode_raw(buf);
    }
}

impl<M: Message> PayloadAccess for ProtoMessage<M> {
    #[inline]
    fn is_proto_empty<A: Allocator + Clone, Pb: PresenceBits>(
        _slot: &UnmanagedBox<M, A>,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        false
    }

    #[inline]
    fn get<'a, A: Allocator + Clone, Pb: PresenceBits>(
        slot: &'a UnmanagedBox<M, A>,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a M
    where
        A: 'a,
    {
        Deref::deref(slot)
    }

    #[inline]
    fn with_mut<'a, A, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> &'a mut M
    where
        A: Allocator + Clone + 'a,
        UnmanagedBox<M, A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        DerefMut::deref_mut(ValueSlot::with_mut(slot, init, common).get_mut())
    }

    #[inline]
    fn write<A, VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: UnmanagedBox<M, A>,
    ) where
        A: Allocator + Clone,
        UnmanagedBox<M, A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<A, VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        A: Allocator + Clone,
        UnmanagedBox<M, A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    fn merge<A, VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        _field: u32,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        UnmanagedBox<M, A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
        VS: ValueSlot<UnmanagedBox<M, A>, A>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut sub = buf.take(len);
        let child = ValueSlot::with_mut(slot, init, common).get_mut();
        DerefMut::deref_mut(child).merge_from(&mut sub)
    }
}
