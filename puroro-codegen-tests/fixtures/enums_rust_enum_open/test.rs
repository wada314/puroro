//! Open enum emitted as a native Rust `enum` (`(puroro.enum_repr) = RUST_ENUM`).

use crate::enums_rust_enum_open::demo::{Holder, Status};
use ::core::convert::TryFrom;
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
fn unknown_open_value_is_stored_as_unknown_variant() {
    // field 1 varint 99
    let wire = [0x08_u8, 99];
    let msg: Holder = Holder::decode(&wire[..]).expect("decode");
    assert!(msg.status().is_set());
    assert_eq!(msg.status().get(), Status::_Unknown(99));
    assert_eq!(i32::try_from(msg.status().get()), Err(99));
    assert_eq!(msg.encode_to_vec(), wire);
}

#[test]
fn match_covers_known_and_unknown() {
    match Status::from(1) {
        Status::UNSPECIFIED => panic!("expected PENDING"),
        Status::PENDING => {}
        Status::DONE => panic!("expected PENDING"),
        Status::_Unknown(_) => panic!("expected PENDING"),
    }
}
