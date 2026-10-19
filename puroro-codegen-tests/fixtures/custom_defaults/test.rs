//! Custom `[default = …]` on singular fields and oneof members (proto2).

use crate::custom_defaults::{Holder, Kind};
use ::puroro::{BytesMut, Message, OneofView};

#[test]
fn singular_custom_defaults_when_unset() {
    let msg = Holder::new();
    assert!(!msg.max_retries().is_set());
    assert_eq!(msg.max_retries().get(), 3);

    assert!(!msg.zero_int().is_set());
    assert_eq!(msg.zero_int().get(), 0);

    assert!(!msg.flag().is_set());
    assert!(msg.flag().get());

    assert!(!msg.title().is_set());
    assert_eq!(msg.title().get(), b"hi");

    assert!(!msg.payload().is_set());
    assert_eq!(msg.payload().get(), b"a\0b");

    assert!(!msg.kind().is_set());
    assert_eq!(msg.kind().get(), Kind::B);
}

#[test]
fn singular_set_and_clear_round_trip() {
    let mut msg = Holder::new();
    *msg.max_retries_mut() = 9;
    assert!(msg.max_retries().is_set());
    assert_eq!(msg.max_retries().get(), 9);

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert!(decoded.max_retries().is_set());
    assert_eq!(decoded.max_retries().get(), 9);

    msg.clear_max_retries();
    assert!(!msg.max_retries().is_set());
    assert_eq!(msg.max_retries().get(), 3);
}

#[test]
fn oneof_custom_default_when_unset_or_other_variant() {
    let mut msg = Holder::new();
    assert!(msg.choice().case().is_none());
    assert!(!msg.webhook_id().is_set());
    assert_eq!(msg.webhook_id().get(), -1);

    msg.note_mut().set(b"hello");
    assert!(!msg.webhook_id().is_set());
    assert_eq!(msg.webhook_id().get(), -1);

    // `_mut` installs the type default (0), not the custom default (-1).
    assert_eq!(*msg.webhook_id_mut(), 0);
    assert!(msg.webhook_id().is_set());
    assert_eq!(msg.webhook_id().get(), 0);
}
