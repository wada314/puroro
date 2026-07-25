//! Editions `repeated_field_encoding = EXPANDED` for a packable scalar.

use crate::repeated_expanded::demo::ExpandedIds;
use ::puroro::Message;
use ::puroro_rt::encode::encode_varint_field;

fn encode_u64_varint(mut v: u64, buf: &mut Vec<u8>) {
    loop {
        let mut byte = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if v == 0 {
            break;
        }
    }
}

fn encode_packed_int32_field(field_number: u32, values: &[i32], buf: &mut Vec<u8>) {
    let mut payload = Vec::new();
    for &v in values {
        encode_u64_varint(v as u64, &mut payload);
    }
    encode_u64_varint(u64::from(field_number) << 3 | 2, buf); // WireType::Len
    encode_u64_varint(payload.len() as u64, buf);
    buf.extend_from_slice(&payload);
}

#[test]
fn encode_uses_expanded_tags() {
    let mut msg = ExpandedIds::new();
    msg.ids_mut().extend_from_slice(&[1, 2, 3]);

    // field 1 varint: tag 0x08 per element → 08 01 08 02 08 03
    assert_eq!(msg.encode_to_vec(), vec![0x08, 1, 0x08, 2, 0x08, 3]);
}

#[test]
fn round_trip() {
    let mut msg = ExpandedIds::new();
    msg.ids_mut().extend_from_slice(&[10, 20]);
    let decoded: ExpandedIds = ExpandedIds::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.ids(), &[10, 20]);
}

#[test]
fn accepts_packed_wire_on_decode() {
    let mut bytes = Vec::new();
    encode_packed_int32_field(1, &[1, 2, 3], &mut bytes);
    let msg: ExpandedIds = ExpandedIds::decode(&bytes[..]).expect("decode packed");
    assert_eq!(msg.ids(), &[1, 2, 3]);
}

#[test]
fn accepts_mixed_wire_forms() {
    let mut bytes = Vec::new();
    encode_varint_field(1, 1, &mut bytes);
    encode_packed_int32_field(1, &[2, 3], &mut bytes);
    encode_varint_field(1, 4, &mut bytes);
    let msg: ExpandedIds = ExpandedIds::decode(&bytes[..]).expect("decode mixed");
    assert_eq!(msg.ids(), &[1, 2, 3, 4]);
}
