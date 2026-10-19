//! Closed enum emitted as a native Rust `enum` (`(puroro.enum_repr) = RUST_ENUM`).

use crate::enums_rust_enum_closed::demo::{Holder, Priority};
use ::core::convert::TryFrom;
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
    assert!(Priority::try_from(99).is_err());
}

#[test]
fn match_is_exhaustive_for_known_values() {
    match Priority::HIGH {
        Priority::UNSPECIFIED => panic!("expected HIGH"),
        Priority::LOW => panic!("expected HIGH"),
        Priority::HIGH => {}
    }
}
