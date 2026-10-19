//! `Marker<A>` bakes [`DiscardUnknowns`]: unrecognized tags are dropped and
//! the store word is omitted. Impls stay `impl<A>` (no `U:` on the message).

use ::allocator_api2::alloc::Global;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::core::mem::size_of;
use ::puroro::Message;
use ::puroro_rt::encode::{encode_varint_field, field_number_const};
use ::puroro_rt::{DiscardUnknowns, MessageCommon, UnknownFields, Varint};
use ::puroro_sample_generated::{Marker, Point};

#[test]
fn discard_store_is_zst_on_common() {
    type Bits = BitArray<[u8; 1], Lsb0>;
    assert_eq!(size_of::<DiscardUnknowns>(), 0);
    assert_eq!(
        size_of::<MessageCommon<Bits, Global, DiscardUnknowns>>(),
        size_of::<Bits>()
    );
    assert!(
        size_of::<MessageCommon<Bits, Global, UnknownFields<Global>>>()
            > size_of::<MessageCommon<Bits, Global, DiscardUnknowns>>()
    );
    assert!(size_of::<Point>() > size_of::<Marker>());
}

#[test]
fn marker_drops_unknowns_and_encodes_known_only() {
    let mut wire = Vec::new();
    encode_varint_field(field_number_const::<1>(), Varint::from_int32(3), &mut wire);
    encode_varint_field(
        field_number_const::<99>(),
        Varint::from_uint64(5),
        &mut wire,
    );

    let decoded: Marker = Marker::decode(&wire[..]).expect("decode");
    assert_eq!(decoded.n(), 3);
    assert!(decoded.unknown_fields().next().is_none());

    let encoded = decoded.encode_to_vec();
    assert_ne!(encoded, wire, "discard encode must omit the unknown tag");
    assert_eq!(encoded, {
        let mut known = Vec::new();
        encode_varint_field(field_number_const::<1>(), Varint::from_int32(3), &mut known);
        known
    });
}

#[test]
fn point_still_preserves_unknowns() {
    let mut wire = Vec::new();
    encode_varint_field(field_number_const::<1>(), Varint::from_int32(3), &mut wire);
    encode_varint_field(
        field_number_const::<99>(),
        Varint::from_uint64(5),
        &mut wire,
    );
    encode_varint_field(field_number_const::<2>(), Varint::from_int32(7), &mut wire);

    let preserved: Point = Point::decode(&wire[..]).expect("preserve decode");
    assert_eq!(preserved.x(), 3);
    assert_eq!(preserved.y(), 7);
    assert_eq!(preserved.unknown_fields().count(), 1);

    // Known fields encode first; the unknown blob is appended, so tag order
    // on the wire is not preserved.
    let again: Point = Point::decode(&preserved.encode_to_vec()[..]).expect("redecode");
    assert_eq!(again.x(), 3);
    assert_eq!(again.y(), 7);
    assert_eq!(again.unknown_fields().count(), 1);
}
