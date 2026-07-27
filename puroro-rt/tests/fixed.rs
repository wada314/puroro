//! Round-trip tests for fixed-width scalar catalog types.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};
use ::puroro::DecodeError;
use ::puroro::Message;
use ::puroro_rt::decode::{decode_tag, skip_field_and_save};
use ::puroro_rt::{
    Expanded, Explicit, FieldDeallocate, FieldEncode, MessageCommon, Packed, ProtoDouble,
    ProtoFixed32, ProtoFloat, RepeatedField, SingularField,
};

/// Minimal message exercising singular + packed + expanded fixed fields.
struct FixedDemo<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    code: SingularField<ProtoFixed32, Explicit<0>, 1, A>,
    altitude: SingularField<ProtoFloat, Explicit<1>, 2, A>,
    samples: RepeatedField<ProtoDouble, Packed, 3, A>,
    tags: RepeatedField<ProtoFixed32, Expanded, 4, A>,
}

impl FixedDemo<Global> {
    fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A: Allocator + Clone> FixedDemo<A> {
    fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            code: SingularField::new_in(alloc.clone()),
            altitude: SingularField::new_in(alloc.clone()),
            samples: RepeatedField::new_in(alloc.clone()),
            tags: RepeatedField::new_in(alloc),
        }
    }
}

impl<A: Allocator + Clone + Default> Default for FixedDemo<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> Drop for FixedDemo<A> {
    fn drop(&mut self) {
        self.code.deallocate(&self._common);
        self.altitude.deallocate(&self._common);
        self.samples.deallocate(&self._common);
        self.tags.deallocate(&self._common);
        self._common.deallocate();
    }
}

impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for FixedDemo<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        drop(self);
    }
}

impl<A: Allocator + Clone> Message for FixedDemo<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encoded_len(&self) -> usize {
        let c = &self._common;
        self.code.encoded_len(c)
            + self.altitude.encoded_len(c)
            + self.samples.encoded_len(c)
            + self.tags.encoded_len(c)
            + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.code.encode_raw(c, buf);
        self.altitude.encode_raw(c, buf);
        self.samples.encode_raw(c, buf);
        self.tags.encode_raw(c, buf);
        buf.put_slice(&c.unknown_fields);
    }

    fn merge_from_with_depth<B: Buf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number {
                1 => self
                    .code
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                2 => self
                    .altitude
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                3 => self
                    .samples
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, depth)?,
                4 => self
                    .tags
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

#[test]
fn fixed_singular_and_repeated_roundtrip() {
    let mut msg = FixedDemo::new();
    *msg.code.bind_mut(&mut msg._common).value_mut() = 0xA1B2_C3D4;
    *msg.altitude.bind_mut(&mut msg._common).value_mut() = 12.5;
    {
        let mut samples = msg.samples.bind_mut(&mut msg._common).values_mut();
        samples.push(1.25);
        samples.push(-2.5);
        samples.push(3.0);
    }
    {
        let mut tags = msg.tags.bind_mut(&mut msg._common).values_mut();
        tags.push(7);
        tags.push(8);
        tags.push(9);
    }

    let bytes = msg.encode_to_vec();
    let decoded = FixedDemo::<Global>::decode(&bytes[..]).unwrap();

    assert_eq!(
        decoded.code.bind(&decoded._common).optional().get(),
        0xA1B2_C3D4
    );
    assert_eq!(
        decoded.altitude.bind(&decoded._common).optional().get(),
        12.5
    );
    assert_eq!(
        decoded.samples.bind(&decoded._common).as_slice(),
        &[1.25, -2.5, 3.0]
    );
    assert_eq!(decoded.tags.bind(&decoded._common).as_slice(), &[7, 8, 9]);
}

#[test]
fn packed_fixed32_dual_form_decode() {
    // Expanded fixed32 (wire type 5) for field 4: values 1, 2
    let mut bytes = Vec::new();
    // tag = (4 << 3) | 5 = 37
    bytes.push(37);
    bytes.extend_from_slice(&1u32.to_le_bytes());
    bytes.push(37);
    bytes.extend_from_slice(&2u32.to_le_bytes());
    // Packed LEN for field 3 (double): one value 4.0
    // tag = (3 << 3) | 2 = 26
    bytes.push(26);
    bytes.push(8); // length
    bytes.extend_from_slice(&4.0f64.to_le_bytes());

    let decoded = FixedDemo::<Global>::decode(&bytes[..]).unwrap();
    assert_eq!(decoded.tags.bind(&decoded._common).as_slice(), &[1, 2]);
    assert_eq!(decoded.samples.bind(&decoded._common).as_slice(), &[4.0]);
}
