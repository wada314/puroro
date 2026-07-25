//! Behavioural tests for singular scalar / string / bool fields.

use crate::scalars::Scalars;
use ::puroro::Message;

#[test]
fn default_scalars_are_zeroish() {
    let msg = Scalars::new();
    assert_eq!(msg.score(), 0);
    assert!(!msg.title().is_set());
    assert!(!msg.done());
    assert!(!msg.postal_code().is_set());
    assert!(!msg.latitude().is_set());
    assert_eq!(Message::encoded_len(&msg), 0);
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

    assert_eq!(msg.score(), 42);
    assert!(msg.title().is_set());
    assert_eq!(msg.title().get(), "hello");
    assert!(msg.done());
    assert_eq!(msg.postal_code().get(), 100);
    assert_eq!(msg.latitude().get(), 1.5);

    let bytes = msg.encode_to_vec();
    let decoded = Scalars::decode(&bytes[..]).expect("decode");
    assert_eq!(decoded.score(), 42);
    assert_eq!(decoded.title().get(), "hello");
    assert!(decoded.done());
    assert_eq!(decoded.postal_code().get(), 100);
    assert_eq!(decoded.latitude().get(), 1.5);
    assert_eq!(decoded, msg);
}

#[test]
fn clear_omits_on_encode() {
    let mut msg = Scalars::new();
    *msg.score_mut() = 7;
    msg.clear_score();
    assert_eq!(msg.score(), 0);
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn unknown_field_preserved_with_knowns() {
    let mut msg = Scalars::new();
    *msg.score_mut() = 1;
    let mut bytes = msg.encode_to_vec();
    // Append unknown field 20 varint 1 → tag (20<<3)|0 = 160 = 0xA0, then 0x01 length? 
    // Actually for field 20 varint: tag = 20*8+0 = 160 = 0xA0 0x01 as varint of 160, then value 0x01
    bytes.extend_from_slice(&[0xA0, 0x01, 0x01]);
    let decoded: Scalars = Scalars::decode(&bytes[..]).expect("decode");
    assert_eq!(decoded.score(), 1);
    assert_eq!(decoded.unknown_fields().count(), 1);
    let reencoded = decoded.encode_to_vec();
    assert!(reencoded.windows(3).any(|w| w == [0xA0, 0x01, 0x01]));
}
