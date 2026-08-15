//! Behavioural tests for singular scalar / string / bytes fields.

use crate::scalars::Scalars;
use puroro::{Message, StringMut};

#[test]
fn default_scalars_are_zeroish() {
    let msg = Scalars::new();
    assert_eq!(msg.score(), 0);
    assert!(!msg.title().is_set());
    assert!(!msg.done());
    assert!(!msg.postal_code().is_set());
    assert!(!msg.latitude().is_set());
    assert_eq!(msg.ratio(), 0.0);
    assert_eq!(msg.big_score(), 0);
    assert_eq!(msg.count(), 0);
    assert_eq!(msg.big_count(), 0);
    assert_eq!(msg.zigzag32(), 0);
    assert_eq!(msg.zigzag64(), 0);
    assert_eq!(msg.fixed_id(), 0);
    assert!(!msg.sfixed32_val().is_set());
    assert!(!msg.sfixed64_val().is_set());
    assert!(!msg.payload().is_set());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn set_and_round_trip() {
    let mut msg = Scalars::new();
    *msg.score_mut() = 42;
    msg.title_mut().clear();
    msg.title_mut().push_str("hello");
    *msg.done_mut() = true;
    *msg.postal_code_mut() = 100;
    *msg.latitude_mut() = 1.5;
    *msg.ratio_mut() = 0.25;
    *msg.big_score_mut() = -9_000_000_000;
    *msg.count_mut() = 7;
    *msg.big_count_mut() = 9_000_000_000;
    *msg.zigzag32_mut() = -3;
    *msg.zigzag64_mut() = -4;
    *msg.fixed_id_mut() = 0x1122_3344_5566_7788;
    *msg.sfixed32_val_mut() = -100;
    *msg.sfixed64_val_mut() = -200;
    msg.payload_mut().extend_from_slice(b"xyz");

    assert_eq!(msg.score(), 42);
    assert!(msg.title().is_set());
    assert_eq!(msg.title().get(), "hello");
    assert!(msg.done());
    assert_eq!(msg.postal_code().get(), 100);
    assert_eq!(msg.latitude().get(), 1.5);
    assert_eq!(msg.ratio(), 0.25);
    assert_eq!(msg.big_score(), -9_000_000_000);
    assert_eq!(msg.count(), 7);
    assert_eq!(msg.big_count(), 9_000_000_000);
    assert_eq!(msg.zigzag32(), -3);
    assert_eq!(msg.zigzag64(), -4);
    assert_eq!(msg.fixed_id(), 0x1122_3344_5566_7788);
    assert_eq!(msg.sfixed32_val().get(), -100);
    assert_eq!(msg.sfixed64_val().get(), -200);
    assert_eq!(msg.payload().get(), b"xyz");

    let bytes = msg.encode_to_vec();
    let decoded = Scalars::decode(&bytes[..]).expect("decode");
    assert_eq!(decoded.score(), 42);
    assert_eq!(decoded.title().get(), "hello");
    assert!(decoded.done());
    assert_eq!(decoded.postal_code().get(), 100);
    assert_eq!(decoded.latitude().get(), 1.5);
    assert_eq!(decoded.ratio(), 0.25);
    assert_eq!(decoded.big_score(), -9_000_000_000);
    assert_eq!(decoded.count(), 7);
    assert_eq!(decoded.big_count(), 9_000_000_000);
    assert_eq!(decoded.zigzag32(), -3);
    assert_eq!(decoded.zigzag64(), -4);
    assert_eq!(decoded.fixed_id(), 0x1122_3344_5566_7788);
    assert_eq!(decoded.sfixed32_val().get(), -100);
    assert_eq!(decoded.sfixed64_val().get(), -200);
    assert_eq!(decoded.payload().get(), b"xyz");
    assert_eq!(decoded, msg);
}

#[test]
fn clear_omits_on_encode() {
    let mut msg = Scalars::new();
    *msg.score_mut() = 7;
    msg.clear_score();
    assert_eq!(msg.score(), 0);

    msg.payload_mut().extend_from_slice(b"x");
    msg.clear_payload();
    assert!(!msg.payload().is_set());

    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn unknown_field_preserved_with_knowns() {
    let mut msg = Scalars::new();
    *msg.score_mut() = 1;
    let mut bytes = msg.encode_to_vec();
    // field 20 varint 1: tag = 20*8+0 = 160 → 0xA0 0x01, value 0x01
    bytes.extend_from_slice(&[0xA0, 0x01, 0x01]);
    let decoded: Scalars = Scalars::decode(&bytes[..]).expect("decode");
    assert_eq!(decoded.score(), 1);
    assert_eq!(decoded.unknown_fields().count(), 1);
    let reencoded = decoded.encode_to_vec();
    assert!(reencoded.windows(3).any(|w| w == [0xA0, 0x01, 0x01]));
}

#[test]
fn optional_string_unset_vs_empty() {
    let unset = Scalars::new();
    assert!(!unset.title().is_set());
    assert!(unset.encode_to_vec().is_empty());

    let mut empty = Scalars::new();
    empty.title_mut().clear(); // set presence with empty payload
    assert!(empty.title().is_set());
    assert_eq!(empty.title().get(), "");
    assert!(!empty.encode_to_vec().is_empty());
}

#[test]
fn scalar_last_wins_on_merge() {
    let mut msg = Scalars::new();
    *msg.score_mut() = 1;
    // field 1 varint 2
    let second = [0x08_u8, 2];
    msg.merge_from(&mut &second[..]).expect("merge");
    assert_eq!(msg.score(), 2);
}
