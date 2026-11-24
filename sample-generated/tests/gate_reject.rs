//! `Gate<A>` bakes [`RejectUnknowns`]: unrecognized tags fail decode.

use ::allocator_api2::alloc::Global;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::core::mem::size_of;
use ::puroro::{DecodeError, Message};
use ::puroro_rt::encode::{encode_varint_field, field_number_const};
use ::puroro_rt::{MessageCommon, RejectUnknowns, Varint};
use ::puroro_sample_generated::{Gate, Marker};

#[test]
fn reject_store_is_zst_like_discard() {
    type Bits = BitArray<[u8; 1], Lsb0>;
    assert_eq!(size_of::<RejectUnknowns>(), 0);
    assert_eq!(
        size_of::<MessageCommon<Bits, Global, RejectUnknowns>>(),
        size_of::<Bits>()
    );
    assert_eq!(size_of::<Gate>(), size_of::<Marker>());
}

#[test]
fn gate_accepts_known_fields() {
    let mut wire = Vec::new();
    encode_varint_field(field_number_const::<1>(), Varint::from_int32(3), &mut wire);

    let decoded: Gate = Gate::decode(&wire[..]).expect("decode");
    assert_eq!(decoded.n(), 3);
    assert!(decoded.unknown_fields().next().is_none());
    assert_eq!(decoded.encode_to_vec(), wire);
}

#[test]
fn gate_rejects_unknown_tag() {
    let mut wire = Vec::new();
    encode_varint_field(field_number_const::<1>(), Varint::from_int32(3), &mut wire);
    encode_varint_field(
        field_number_const::<99>(),
        Varint::from_uint64(5),
        &mut wire,
    );

    let result: Result<Gate, _> = Gate::decode(&wire[..]);
    assert_eq!(
        result.unwrap_err(),
        DecodeError::UnrecognizedField { field_number: 99 }
    );
}
