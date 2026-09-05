//! `(puroro.unknown_fields) = DISCARD` bakes `DiscardUnknowns` into the message.

use crate::unknown_fields::{Discard, DiscardChoice, Keep};
use ::core::mem::size_of;
use ::puroro::Message;

#[test]
fn keep_preserves_unknown_tag() {
    let wire = [0x08_u8, 0x03, 0x50, 0x05]; // n=3, field 10 = 5
    let msg: Keep = Keep::decode(&wire[..]).expect("decode");
    assert_eq!(msg.n().get(), 3);
    assert_eq!(msg.unknown_fields().count(), 1);
    assert_eq!(msg.encode_to_vec(), wire);
}

#[test]
fn discard_drops_unknown_tag() {
    let wire = [0x08_u8, 0x03, 0x50, 0x05]; // n=3, field 10 = 5
    let msg: Discard = Discard::decode(&wire[..]).expect("decode");
    assert_eq!(msg.n().get(), 3);
    assert!(msg.unknown_fields().next().is_none());
    assert_eq!(msg.encode_to_vec(), [0x08, 0x03]);
}

#[test]
fn discard_message_is_smaller() {
    assert!(size_of::<Keep>() > size_of::<Discard>());
}

#[test]
fn discard_oneof_drops_unknown_and_keeps_variant() {
    let mut wire = Vec::new();
    // choice.a = 7 (field 1 varint)
    wire.extend_from_slice(&[0x08, 0x07]);
    // unknown field 10 = 5
    wire.extend_from_slice(&[0x50, 0x05]);

    let msg: DiscardChoice = DiscardChoice::decode(&wire[..]).expect("decode");
    assert_eq!(msg.a().get(), 7);
    assert!(msg.unknown_fields().next().is_none());
    assert_eq!(msg.encode_to_vec(), [0x08, 0x07]);
}
