//! Recursion-limit enforcement for nested-message decode.
//!
//! Wire payloads are hand-built so the test does not need to *encode* a
//! self-referential tree (which can overflow rustc's trait solver). Decode still
//! uses a self-referential [`Nest`] message type.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bitvec::ptr::{BitRef, Mut};
use ::bytes::{Buf, BufMut};
use ::puroro::{DecodeError, Message, RECURSION_LIMIT};
use ::puroro_rt::decode::{decode_tag, skip_field_and_save};
use ::puroro_rt::{
    FieldDeallocate, MessageCommon, NonOneof, PresenceBits, ProtoMessage, SingularField,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct NestPresence(BitArray<[u8; 1], Lsb0>);

impl PresenceBits for NestPresence {
    fn is_set(&self, bit: usize) -> bool {
        self.0[bit]
    }

    fn set(&mut self, bit: usize, present: bool) {
        self.0.set(bit, present);
    }

    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        self.0.get_mut(bit).expect("presence bit index in range")
    }
}

/// Self-referential message: optional `child` of the same type (field 1).
struct Nest<A: Allocator + Clone = Global> {
    _common: MessageCommon<NestPresence, A>,
    child: SingularField<ProtoMessage<Nest<A>>, NonOneof, 1, A>,
}

impl<A: Allocator + Clone> Nest<A> {
    fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(NestPresence::default(), alloc.clone()),
            child: SingularField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone + Default> Default for Nest<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> Drop for Nest<A> {
    fn drop(&mut self) {
        self.child.deallocate(&self._common);
        self._common.deallocate();
    }
}

impl<A: Allocator + Clone> ::unmanaged::CloneIn<A> for Nest<A> {
    fn clone_in(&self, alloc: A) -> Self {
        Self {
            _common: self._common.clone_in(alloc.clone()),
            child: self.child.clone_in(&self._common, alloc),
        }
    }
}

impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for Nest<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        drop(self);
    }
}

impl<A: Allocator + Clone> Message for Nest<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encoded_len(&self) -> usize {
        let c = &self._common;
        self.child.encoded_len(c) + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.child.encode_raw(c, buf);
        buf.put_slice(&c.unknown_fields);
    }

    fn merge_from_with_depth<B: Buf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number {
                1 => self
                    .child
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                _ => skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                    self._common.alloc.clone(),
                )?,
            }
        }
        Ok(())
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}

/// Builds `levels` nested empty `Nest` children as wire bytes (field 1, LEN).
fn encode_nest_levels(levels: usize) -> Vec<u8> {
    let mut payload = Vec::new();
    for _ in 0..levels {
        let inner = payload;
        let mut outer = Vec::new();
        outer.push(10); // tag: field 1, WireType::Len
        // length varint (always small here)
        let len = inner.len() as u64;
        let mut v = len;
        loop {
            let mut byte = (v & 0x7f) as u8;
            v >>= 7;
            if v != 0 {
                byte |= 0x80;
            }
            outer.push(byte);
            if v == 0 {
                break;
            }
        }
        outer.extend_from_slice(&inner);
        payload = outer;
    }
    payload
}

#[test]
fn recursion_within_limit_decodes() {
    let bytes = encode_nest_levels(RECURSION_LIMIT - 1);
    let decoded = Nest::<Global>::decode(&bytes[..]).expect("within limit");
    let mut cur = &decoded;
    for _ in 0..RECURSION_LIMIT - 1 {
        cur = cur.child.bind(&cur._common).get().expect("child present");
    }
    assert!(cur.child.bind(&cur._common).get().is_none());
}

#[test]
fn recursion_at_limit_fails_decode() {
    let bytes = encode_nest_levels(RECURSION_LIMIT);
    match Nest::<Global>::decode(&bytes[..]) {
        Err(DecodeError::RecursionLimitExceeded) => {}
        Err(_) => panic!("expected RecursionLimitExceeded"),
        Ok(_) => panic!("expected decode to fail at recursion limit"),
    }
}
