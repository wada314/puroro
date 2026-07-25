//! Closed enum (`features.enum_type = CLOSED`) with EXPLICIT presence.

use crate::enums_closed::demo::{Holder, Priority};
use ::puroro::Message;

#[test]
fn default_is_unset() {
    let msg = Holder::new();
    assert!(!msg.priority().is_set());
    assert_eq!(msg.priority().get(), Priority::UNSPECIFIED);
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn known_value_round_trip() {
    let mut msg = Holder::new();
    *msg.priority_mut() = Priority::HIGH;
    assert!(msg.priority().is_set());
    assert_eq!(msg.priority().get(), Priority::HIGH);

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.priority().get(), Priority::HIGH);
    assert!(decoded.priority().is_set());
}

#[test]
fn unknown_closed_value_goes_to_unknown_fields() {
    // field 1 varint 99
    let wire = [0x08_u8, 99];
    let msg: Holder = Holder::decode(&wire[..]).expect("decode");
    assert!(!msg.priority().is_set());
    assert_eq!(msg.priority().get(), Priority::UNSPECIFIED);
    assert_eq!(msg.unknown_fields().count(), 1);
    assert_eq!(msg.encode_to_vec(), wire);
}
