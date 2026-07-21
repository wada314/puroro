//! Behavioural tests for the fake empty-message generator.

use ::puroro::Message;
use ::puroro_codegen_tests::empty::Empty;

#[test]
fn default_encodes_to_empty() {
    let msg: Empty = Empty::new();
    assert_eq!(Message::encoded_len(&msg), 0);
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn decode_empty_bytes() {
    let msg: Empty = Empty::decode(&[][..]).expect("decode empty");
    assert_eq!(Message::encoded_len(&msg), 0);
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn round_trip_unknown_field() {
    // field 1, varint 150
    let wire = [0x08_u8, 0x96, 0x01];
    let msg: Empty = Empty::decode(&wire[..]).expect("decode unknown");
    assert_eq!(msg.encode_to_vec(), wire);
    assert_eq!(msg.unknown_fields().count(), 1);
}

#[test]
fn clone_and_eq() {
    let a: Empty = Empty::new();
    let b = a.clone();
    assert_eq!(a, b);

    let wire = [0x08_u8, 0x01];
    let with_unknown: Empty = Empty::decode(&wire[..]).unwrap();
    assert_ne!(a, with_unknown);
    assert_eq!(with_unknown.clone(), with_unknown);
}
