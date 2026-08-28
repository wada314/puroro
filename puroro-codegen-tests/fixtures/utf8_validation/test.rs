//! Editions `utf8_validation=VERIFY` vs `NONE` on string fields.

use crate::utf8_validation::Utf8Fields;
use puroro::{BytesMut, DecodeError, Message};

fn len_field(number: u32, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let tag = (number << 3) | 2;
    out.push(tag as u8);
    out.push(payload.len() as u8);
    out.extend_from_slice(payload);
    out
}

#[test]
fn verify_rejects_invalid_utf8() {
    let result: Result<Utf8Fields, _> = Utf8Fields::decode(&len_field(1, &[0xff])[..]);
    assert_eq!(result.unwrap_err(), DecodeError::InvalidUtf8);
}

#[test]
fn none_sso_accepts_invalid_utf8_and_round_trips() {
    let msg: Utf8Fields = Utf8Fields::decode(&len_field(2, &[0xff])[..]).expect("NONE decode");
    assert!(msg.raw().is_set());
    assert_eq!(msg.raw().get(), b"\xff");

    let encoded = msg.encode_to_vec();
    let again: Utf8Fields = Utf8Fields::decode(&encoded[..]).expect("NONE re-decode");
    assert_eq!(again.raw().get(), b"\xff");
}

#[test]
fn none_repeated_accepts_invalid_utf8() {
    let msg: Utf8Fields =
        Utf8Fields::decode(&len_field(3, &[0xff])[..]).expect("NONE repeated decode");
    assert_eq!(msg.raw_list().len(), 1);
    assert_eq!(&*msg.raw_list()[0], b"\xff");
}

#[test]
fn none_still_round_trips_valid_utf8() {
    let mut msg = Utf8Fields::new();
    msg.raw_mut().set(b"ok");
    let decoded: Utf8Fields = Utf8Fields::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.raw().get(), b"ok");
}
