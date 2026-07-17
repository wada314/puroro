//! Nested-message type marker for [`ProtoType`](super::proto_type::ProtoType).
//!
//! [`ProtoMessage`] supplies merge-into wire semantics; physical storage is
//! [`UnmanagedBox<M, A>`]. Presence policy (`NonOneof` / `Oneof`) lives on
//! [`FieldPresence`](crate::fields::shared::field_presence::FieldPresence).

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
/// Physical slot is [`UnmanagedBox<M, A>`]. Does not store a value itself.
pub struct ProtoMessage<M, A>(PhantomData<fn() -> (M, A)>);

impl<M, A> Default for ProtoMessage<M, A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M, A> Clone for ProtoMessage<M, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M, A> Copy for ProtoMessage<M, A> {}

impl<M: Message<Alloc = A>, A: Allocator + Clone> DefaultIn for UnmanagedBox<M, A> {
    type Alloc = A;

    #[inline]
    fn default_in(alloc: A) -> Self {
        UnmanagedBox::new_in(M::new_in(alloc.clone()), alloc)
    }
}

impl<M: Message<Alloc = A>, A: Allocator + Clone> DeallocateIn for UnmanagedBox<M, A> {
    type Alloc = A;

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

impl<M: Message<Alloc = A>, A: Allocator + Clone> AddressableSlot for UnmanagedBox<M, A> {
    type SlotAlloc = A;
}

impl<M: Message<Alloc = A>, A: Allocator + Clone> ProtoType for ProtoMessage<M, A> {
    type Alloc = A;
    type Slot = UnmanagedBox<M, A>;
    type Ref<'a>
        = &'a M
    where
        Self: 'a;
    type Mut<'a>
        = &'a mut M
    where
        Self: 'a;
    type Written = UnmanagedBox<M, A>;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn encoded_len<'a>(value: Self::Ref<'a>, field: u32) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_len_field(field, value.encoded_len())
    }

    #[inline]
    fn encode<'a, B: BufMut>(value: Self::Ref<'a>, field: u32, buf: &mut B)
    where
        Self: 'a,
    {
        let payload_len = value.encoded_len();
        encode::encode_tag(field, WireType::Len, buf);
        encode::encode_varint(payload_len as u64, buf);
        value.encode_raw(buf);
    }

    #[inline]
    fn decode<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Written, DecodeError> {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut sub = buf.take(len);
        let mut child = M::new_in(alloc.clone());
        child.merge_from(&mut sub)?;
        Ok(UnmanagedBox::new_in(child, alloc))
    }
}

impl<M: Message<Alloc = A>, A: Allocator + Clone> PayloadAccess for ProtoMessage<M, A> {
    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(
        _slot: &Self::Slot,
        _common: &MessageCommon<Pb, A>,
    ) -> bool {
        false
    }

    #[inline]
    fn get<'a, Pb: PresenceBits>(slot: &'a Self::Slot, _common: &'a MessageCommon<Pb, A>) -> &'a M {
        Deref::deref(slot)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        Self: 'a,
    {
        DerefMut::deref_mut(ValueSlot::with_mut(slot, init, common).get_mut())
    }

    #[inline]
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        _field: u32,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<Self::Slot>,
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
