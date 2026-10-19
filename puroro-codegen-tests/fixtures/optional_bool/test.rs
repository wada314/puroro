//! EXPLICIT `optional bool` (presence bit + BitPacked value bit).

use crate::optional_bool::FlagHolder;
use ::puroro::Message;

#[test]
fn unset_encodes_empty() {
    let msg = FlagHolder::new();
    assert!(!msg.flag().is_set());
    assert!(!msg.flag().get());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn set_true_round_trip() {
    let mut msg = FlagHolder::new();
    *msg.flag_mut() = true;
    assert!(msg.flag().is_set());
    assert!(msg.flag().get());

    let decoded: FlagHolder = FlagHolder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert!(decoded.flag().is_set());
    assert!(decoded.flag().get());
}

#[test]
fn set_false_is_distinct_from_unset() {
    let mut msg = FlagHolder::new();
    *msg.flag_mut() = false;
    assert!(msg.flag().is_set());
    assert!(!msg.flag().get());

    let bytes = msg.encode_to_vec();
    assert!(!bytes.is_empty());
    let decoded: FlagHolder = FlagHolder::decode(&bytes[..]).expect("decode");
    assert!(decoded.flag().is_set());
    assert!(!decoded.flag().get());
}

#[test]
fn clear_returns_to_unset() {
    let mut msg = FlagHolder::new();
    *msg.flag_mut() = true;
    msg.clear_flag();
    assert!(!msg.flag().is_set());
    assert!(msg.encode_to_vec().is_empty());
}
