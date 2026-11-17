//! Group type marker for [`SingularType`](super::singular_type::SingularType).
//!
//! [`ProtoGroup`] is the group-framed sibling of
//! [`ProtoMessage`](super::proto_message::ProtoMessage). Storage is the same
//! (`Inline` stores `M`, [`Boxed`](crate::fields::shared::value_layout::Boxed)
//! stores [`UnmanagedBox<M, A>`](::unmanaged::UnmanagedBox)). The wire shape
//! is a start-group tag, the child fields, and a matching end-group tag. There
//! is no length prefix.
//!
//! Codegen does not emit this marker yet. A hand-written field is
//! `SingularField<ProtoGroup<M>, Message, FIELD, A, Inline>` (or `Boxed`).

use ::allocator_api2::alloc::Allocator;
use ::core::marker::PhantomData;
use ::protobuf_core::FieldNumber;

use ::puroro::{DecodeBuf, DecodeError, WireType};

use crate::decode;
use crate::fields::shared::{
    CloneBound, DeallocateBound, DefaultIn, MessageBindingMut,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::fields::wire::singular_type::{PayloadAccess, PayloadMerge, SingularType};
use crate::message_encode::MessageEncode;
use crate::message_merge::MessageMerge;

/// Type marker for a singular group whose body is message `M`.
///
/// [`PayloadAccess::Slot`] is `M` (inlined). Heap boxing is
/// [`Boxed`](crate::fields::shared::value_layout::Boxed).
pub struct ProtoGroup<M>(PhantomData<fn() -> M>);

impl<M> Default for ProtoGroup<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M> Clone for ProtoGroup<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for ProtoGroup<M> {}

impl<M: MessageEncode> SingularType for ProtoGroup<M> {}

impl<M: MessageEncode> PayloadAccess for ProtoGroup<M> {
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
    fn with_mut<'a, A, VS, I, Cx>(slot: &'a mut VS, init: I, common: &'a mut Cx) -> &'a mut M
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

impl<M: MessageEncode + MessageMerge> PayloadMerge for ProtoGroup<M> {
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
        decode::read_group_body(buf, wire_type, field, |body| {
            let child = ValueSlot::with_mut(slot, init, common).get_mut();
            MessageMerge::merge_from_slice(child, body, depth + 1)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ProtoGroup;
    use crate::decode::{decode_tag, decode_varint, read_group_body, skip_field};
    use crate::encode::{encode_tag, encode_varint, field_number_const};
    use crate::fields::repeated::{Expanded, RepeatedField};
    use crate::fields::shared::field_presence::Message;
    use crate::fields::shared::value_layout::{Boxed, Inline};
    use crate::fields::shared::{FieldDeallocate, FieldEncode, MessageBindingMut, MessageCommon};
    use crate::fields::singular::SingularField;
    use crate::fields::wire::encode_type::{encode_field, encoded_len_field};
    use crate::fields::wire::numerical::ProtoInt32;
    use crate::message_encode::{EncodeCtx, MessageEncode};
    use crate::message_merge::MessageMerge;
    use crate::{CloneBound, DeallocateBound, DefaultIn};
    use ::allocator_api2::alloc::{Allocator, Global};
    use ::bitvec::array::BitArray;
    use ::bitvec::order::Lsb0;
    use ::bytes::{Buf, BufMut};
    use ::protobuf_core::{FieldNumber, Varint};
    use ::puroro::{DecodeBuf, DecodeError, RECURSION_LIMIT, ScopedBuf, WireType};
    use ::std::vec::Vec;
    use ::unmanaged::DeallocateIn;

    type Bits1 = BitArray<[u8; 1], Lsb0>;
    type TestCommon = MessageCommon<Bits1, Global>;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Counts {
        a: i32,
        b: i32,
    }

    impl<A: Allocator> DefaultIn<A> for Counts {
        fn default_in(_alloc: A) -> Self {
            Self { a: 0, b: 0 }
        }
    }

    impl<A: Allocator> DeallocateBound<A> for Counts {
        fn deallocate_bound<Cx: MessageBindingMut<A>>(self, _common: &Cx) {}
    }

    impl<A: Allocator + Clone> CloneBound<A> for Counts {
        fn clone_bound<Cx: MessageBindingMut<A>>(&self, _common: &Cx, _alloc: A) -> Self {
            self.clone()
        }
    }

    impl<A: Allocator> DeallocateIn<A> for Counts {
        unsafe fn deallocate_in(self, _alloc: &A) {}
    }

    impl MessageEncode for Counts {
        fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
            let mut n = 0;
            if self.a != 0 {
                n +=
                    encoded_len_field::<ProtoInt32, Global>(self.a, field_number_const::<1>(), ctx);
            }
            if self.b != 0 {
                n +=
                    encoded_len_field::<ProtoInt32, Global>(self.b, field_number_const::<3>(), ctx);
            }
            n
        }

        fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
            if self.a != 0 {
                encode_field::<ProtoInt32, Global, B>(self.a, field_number_const::<1>(), ctx, buf);
            }
            if self.b != 0 {
                encode_field::<ProtoInt32, Global, B>(self.b, field_number_const::<3>(), ctx, buf);
            }
        }
    }

    impl MessageMerge for Counts {
        fn merge_from_with_depth<B: DecodeBuf>(
            &mut self,
            buf: &mut B,
            depth: usize,
        ) -> Result<(), DecodeError> {
            if depth >= RECURSION_LIMIT {
                return Err(DecodeError::RecursionLimitExceeded);
            }
            while buf.has_remaining() {
                let (number, wire) = decode_tag(buf)?;
                match (number.as_u32(), wire) {
                    (1, WireType::Varint) => {
                        self.a = i32::try_from(decode_varint(buf)?).expect("test varint fits i32");
                    }
                    (3, WireType::Varint) => {
                        self.b = i32::try_from(decode_varint(buf)?).expect("test varint fits i32");
                    }
                    (2, WireType::SGroup) => {
                        read_group_body(buf, wire, field_number_const::<2>(), |body| {
                            let mut inner = Counts { a: 0, b: 0 };
                            inner.merge_from_slice(body, depth + 1)?;
                            self.b = inner.a;
                            Ok(())
                        })?;
                    }
                    _ => skip_field(number, wire, buf)?,
                }
            }
            Ok(())
        }
    }

    fn num(n: u32) -> FieldNumber {
        FieldNumber::try_new(n).expect("test field number")
    }

    fn varint_num(number: u32, value: i32) -> Vec<u8> {
        let mut buf = Vec::new();
        encode_tag(num(number), WireType::Varint, &mut buf);
        encode_varint(Varint::from_int32(value), &mut buf);
        buf
    }

    /// Group payload after the start tag: `body` then the matching end tag.
    fn group_payload(number: u32, body: &[u8]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(body);
        encode_tag(num(number), WireType::EGroup, &mut buf);
        buf
    }

    fn full_group(number: u32, body: &[u8]) -> Vec<u8> {
        let mut buf = Vec::new();
        encode_tag(num(number), WireType::SGroup, &mut buf);
        buf.extend_from_slice(&group_payload(number, body));
        buf
    }

    struct Chunked<'a> {
        rest: &'a [u8],
        max_chunk: usize,
    }

    impl Buf for Chunked<'_> {
        fn remaining(&self) -> usize {
            self.rest.len()
        }

        fn chunk(&self) -> &[u8] {
            let n = self.max_chunk.min(self.rest.len());
            &self.rest[..n]
        }

        fn advance(&mut self, cnt: usize) {
            self.rest = &self.rest[cnt..];
        }
    }

    #[test]
    fn inline_group_merges_and_round_trips() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoGroup<Counts>, Message, 7, Global, Inline>::new_in(Global);
        let body = varint_num(1, 4);
        let wire = full_group(7, &body);
        let mut tag = Vec::new();
        encode_tag(num(7), WireType::SGroup, &mut tag);
        let mut payload = &wire[tag.len()..];

        let mut buf = ScopedBuf::new(&mut payload);
        field
            .bind_mut(&mut common)
            .merge(WireType::SGroup, &mut buf, 0)
            .expect("merge group");
        assert!(!buf.has_remaining());
        assert_eq!(field.bind(&common).get().map(|c| (c.a, c.b)), Some((4, 0)));

        let mut encoded = Vec::new();
        field.encode_raw(&common, &mut EncodeCtx::new(), &mut encoded);
        assert_eq!(encoded, wire);
        assert_eq!(
            field.encoded_len(&common, &mut EncodeCtx::new()),
            encoded.len()
        );

        let empty = group_payload(7, &[]);
        let mut empty_buf = &empty[..];
        let mut buf = ScopedBuf::new(&mut empty_buf);
        field
            .bind_mut(&mut common)
            .merge(WireType::SGroup, &mut buf, 0)
            .expect("merge empty into existing");
        assert_eq!(field.bind(&common).get().map(|c| (c.a, c.b)), Some((4, 0)));

        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn boxed_group_reads_a_nested_group_and_skips_an_unknown_one() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoGroup<Counts>, Message, 7, Global, Boxed>::new_in(Global);

        let mut body = Vec::new();
        body.extend_from_slice(&full_group(7, &varint_num(1, 1)));
        body.extend_from_slice(&varint_num(1, 2));
        body.extend_from_slice(&full_group(2, &varint_num(1, 9)));
        let payload = group_payload(7, &body);

        let mut chunked = Chunked {
            rest: &payload,
            max_chunk: 1,
        };
        let mut buf = ScopedBuf::new(&mut chunked);
        field
            .bind_mut(&mut common)
            .merge(WireType::SGroup, &mut buf, 0)
            .expect("merge split group");
        assert_eq!(field.bind(&common).get().map(|c| (c.a, c.b)), Some((2, 9)));

        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn repeated_groups_append_and_reject_len() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field = RepeatedField::<ProtoGroup<Counts>, Expanded, 7, Global>::new_in(Global);
        for value in [3, 5] {
            let payload = group_payload(7, &varint_num(1, value));
            let mut rest = &payload[..];
            let mut buf = ScopedBuf::new(&mut rest);
            field
                .bind_mut(&mut common)
                .merge(WireType::SGroup, &mut buf, 0)
                .expect("append group");
        }
        assert_eq!(
            field.as_slice().iter().map(|c| c.a).collect::<Vec<_>>(),
            vec![3, 5]
        );

        let mut encoded = Vec::new();
        field.encode_raw(&common, &mut EncodeCtx::new(), &mut encoded);
        assert_eq!(encoded, {
            let mut both = full_group(7, &varint_num(1, 3));
            both.extend_from_slice(&full_group(7, &varint_num(1, 5)));
            both
        });

        let mut bad: &[u8] = &[0];
        let mut buf = ScopedBuf::new(&mut bad);
        let err = field
            .bind_mut(&mut common)
            .merge(WireType::Len, &mut buf, 0)
            .unwrap_err();
        assert_eq!(err, DecodeError::InvalidTag);
        assert_eq!(bad, &[0]);

        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn mismatched_end_truncated_and_depth_limit() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoGroup<Counts>, Message, 7, Global, Inline>::new_in(Global);

        let mut mismatched = Vec::new();
        encode_tag(num(8), WireType::EGroup, &mut mismatched);
        let mut mismatched_buf = &mismatched[..];
        let mut buf = ScopedBuf::new(&mut mismatched_buf);
        let err = field
            .bind_mut(&mut common)
            .merge(WireType::SGroup, &mut buf, 0)
            .unwrap_err();
        assert_eq!(
            err,
            DecodeError::MismatchedEndGroup {
                expected: 7,
                got: 8
            }
        );

        let truncated = varint_num(1, 1);
        let mut truncated_buf = &truncated[..];
        let mut buf = ScopedBuf::new(&mut truncated_buf);
        let err = field
            .bind_mut(&mut common)
            .merge(WireType::SGroup, &mut buf, 0)
            .unwrap_err();
        assert_eq!(err, DecodeError::TruncatedMessage);
        assert_eq!(buf.remaining(), truncated.len());

        let payload = group_payload(7, &[]);
        let mut rest = &payload[..];
        let mut buf = ScopedBuf::new(&mut rest);
        let err = field
            .bind_mut(&mut common)
            .merge(WireType::SGroup, &mut buf, RECURSION_LIMIT)
            .unwrap_err();
        assert_eq!(err, DecodeError::RecursionLimitExceeded);
        assert!(!buf.has_remaining());

        field.deallocate(&common);
        common.deallocate();
    }
}
