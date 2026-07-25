//! Open enum (edition 2023 default) with IMPLICIT presence.

use crate::enums_open::demo::{Holder, Status};
use ::puroro::Message;

#[test]
fn default_is_unset_zero() {
    let msg = Holder::new();
    assert!(!msg.status().is_set());
    assert_eq!(msg.status().get(), Status::UNSPECIFIED);
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn known_value_round_trip() {
    let mut msg = Holder::new();
    *msg.status_mut() = Status::PENDING;
    assert!(msg.status().is_set());
    assert_eq!(msg.status().get(), Status::PENDING);

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.status().get(), Status::PENDING);
    assert!(decoded.status().is_set());
}

#[test]
fn unknown_open_value_is_stored() {
    // field 1 varint 99
    let wire = [0x08_u8, 99];
    let msg: Holder = Holder::decode(&wire[..]).expect("decode");
    assert!(msg.status().is_set());
    assert_eq!(msg.status().get(), Status::from(99));
    assert_eq!(msg.encode_to_vec(), wire);
}
