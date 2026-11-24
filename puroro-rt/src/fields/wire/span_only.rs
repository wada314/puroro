//! Span-only lazy LEN layout: payload is a [`WireSpan`] into the island buffer.
//!
//! Used by no-cache profiles ([`Borrowed`](crate::decode::Borrowed)). There is
//! no Inline / Heap / Failed arm and no kind bits — `KIND` on the profile's
//! associated type is ignored. UTF-8 / bytes views always re-slice the wire.

use ::allocator_api2::alloc::Allocator;
use ::core::str;
use ::puroro::DecodeError;
use ::unmanaged::DefaultIn;

use super::lazy_len::{LazyBytesGet, LazyLenStore, LazyStringGet};
use super::len::{BytesLikeLenCodec, LenScalar, ProtoString};
use crate::decode::WireSpan;
use crate::fields::shared::slot_init::SlotInitMut;
use crate::fields::shared::value_layout::{ValueLayout, ValueLayoutClone};
use crate::fields::shared::value_slot::{ValueSlot, ValueSlotMutAccess};
use crate::fields::shared::{MessageBindingMut, MessageCommonAlloc, MessageCommonSharedBits};

/// Singular `string` / `bytes` layout that stores only a wire span.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SpanOnly;

impl<A: Allocator> ValueLayout<ProtoString, A> for SpanOnly {
    type Slot = WireSpan;

    fn is_proto_empty<Cx>(slot: &Self::Slot, _common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        slot.len == 0
    }

    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if !init.is_initialized(|b| common.is_bit_set(b)) {
            return;
        }
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            *s = WireSpan { offset: 0, len: 0 };
        }
        init.set_initialized(|b, v| common.set_bit(b, v), false);
    }

    fn deallocate_slot<VS, Cx>(_slot: VS, _initialized: bool, _common: &Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
    }
}

impl<A: Allocator + Clone> ValueLayoutClone<ProtoString, A> for SpanOnly {
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, _common: &Cx, _alloc: A) -> VS
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
        VS::from_optional(slot.get_value(initialized).copied())
    }
}

impl<A: Allocator, C: BytesLikeLenCodec> ValueLayout<LenScalar<C>, A> for SpanOnly {
    type Slot = WireSpan;

    fn is_proto_empty<Cx>(slot: &Self::Slot, _common: &Cx) -> bool
    where
        Cx: MessageBindingMut<A>,
    {
        slot.len == 0
    }

    fn clear<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
        Self::Slot: DefaultIn<A>,
    {
        if !init.is_initialized(|b| common.is_bit_set(b)) {
            return;
        }
        {
            let s = ValueSlot::with_mut(slot, init, common).get_mut();
            *s = WireSpan { offset: 0, len: 0 };
        }
        init.set_initialized(|b, v| common.set_bit(b, v), false);
    }

    fn deallocate_slot<VS, Cx>(_slot: VS, _initialized: bool, _common: &Cx)
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
    }
}

impl<A: Allocator + Clone, C: BytesLikeLenCodec> ValueLayoutClone<LenScalar<C>, A> for SpanOnly {
    fn clone_slot<VS, Cx>(slot: &VS, initialized: bool, _common: &Cx, _alloc: A) -> VS
    where
        VS: ValueSlot<Self::Slot, A>,
        Cx: MessageBindingMut<A>,
    {
        VS::from_optional(slot.get_value(initialized).copied())
    }
}

impl<A: Allocator> LazyLenStore<ProtoString, A> for SpanOnly {
    fn store_len_span<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, span: WireSpan)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
    {
        let _ = ValueSlot::with_mut(slot, init, common).replace(span);
    }
}

impl<A: Allocator, C: BytesLikeLenCodec> LazyLenStore<LenScalar<C>, A> for SpanOnly {
    fn store_len_span<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, span: WireSpan)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone,
    {
        let _ = ValueSlot::with_mut(slot, init, common).replace(span);
    }
}

impl<A: Allocator> LazyStringGet<A> for SpanOnly {
    fn get_str<'msg, Cx>(
        slot: &'msg Self::Slot,
        _common: &'msg Cx,
        wire: &'msg [u8],
        _alloc: A,
    ) -> Result<&'msg str, DecodeError>
    where
        Cx: MessageCommonSharedBits + MessageCommonAlloc<Alloc = A>,
        A: Clone,
    {
        let bytes = slot.slice(wire)?;
        str::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)
    }
}

impl<C: BytesLikeLenCodec, A: Allocator> LazyBytesGet<C, A> for SpanOnly {
    fn get_bytes<'msg, Cx>(
        slot: &'msg Self::Slot,
        _common: &'msg Cx,
        wire: &'msg [u8],
        _alloc: A,
    ) -> Result<&'msg [u8], DecodeError>
    where
        Cx: MessageCommonSharedBits + MessageCommonAlloc<Alloc = A>,
        A: Clone,
    {
        slot.slice(wire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_slice_utf8() {
        let wire = b"hello";
        let span = WireSpan { offset: 1, len: 3 };
        assert_eq!(str::from_utf8(span.slice(wire).unwrap()).unwrap(), "ell");
    }
}
