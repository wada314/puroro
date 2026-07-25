//! Cross-edition enum import: edition-2023 open + edition-2024 closed.

use crate::enum_import::demo::{Holder, Priority, Status};
use ::puroro::Message;

#[test]
fn open_and_closed_round_trip() {
    let mut msg = Holder::new();
    *msg.status_mut() = Status::PENDING;
    *msg.priority_mut() = Priority::HIGH;

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.status().get(), Status::PENDING);
    assert!(decoded.status().is_set());
    assert_eq!(decoded.priority().get(), Priority::HIGH);
    assert!(decoded.priority().is_set());
}

#[test]
fn imported_open_enum_keeps_unknown_value() {
    // field 1 = 99
    let wire = [0x08_u8, 99];
    let msg: Holder = Holder::decode(&wire[..]).expect("decode");
    assert!(msg.status().is_set());
    assert_eq!(msg.status().get(), Status::from(99));
    assert!(!msg.priority().is_set());
    assert_eq!(msg.unknown_fields().count(), 0);
}

#[test]
fn imported_closed_enum_diverts_unknown_value() {
    // field 2 = 99 → tag (2<<3)|0 = 16
    let wire = [0x10_u8, 99];
    let msg: Holder = Holder::decode(&wire[..]).expect("decode");
    assert!(!msg.priority().is_set());
    assert_eq!(msg.priority().get(), Priority::UNSPECIFIED);
    assert!(!msg.status().is_set());
    assert_eq!(msg.unknown_fields().count(), 1);
    assert_eq!(msg.encode_to_vec(), wire);
}
