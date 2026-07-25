//! `map<string, int32>` insert / get / clear / merge last-wins.

use crate::map_basic::Holder;
use ::puroro::{MapMut, MapRef, Message};

#[test]
fn defaults_empty() {
    let msg = Holder::new();
    assert!(msg.attributes().is_empty());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn insert_get_and_round_trip() {
    let mut msg = Holder::new();
    {
        let mut attrs = msg.attributes_mut();
        attrs.insert_in("region", 81).unwrap();
        attrs.insert_in("tier", 2).unwrap();
    }
    assert_eq!(msg.attributes().len(), 2);
    assert_eq!(msg.attributes().get("region").copied(), Some(81));
    assert_eq!(msg.attributes().get("tier").copied(), Some(2));

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.attributes().len(), 2);
    assert_eq!(decoded.attributes().get("region").copied(), Some(81));
    assert_eq!(decoded.attributes().get("tier").copied(), Some(2));
}

#[test]
fn insert_same_key_last_wins() {
    let mut msg = Holder::new();
    {
        let mut attrs = msg.attributes_mut();
        attrs.insert_in("k", 1).unwrap();
        attrs.insert_in("k", 2).unwrap();
        *attrs.get_mut("k").unwrap() = 3;
    }
    assert_eq!(msg.attributes().get("k").copied(), Some(3));
    assert_eq!(msg.attributes().len(), 1);

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.attributes().get("k").copied(), Some(3));
}

#[test]
fn clear_empties_map() {
    let mut msg = Holder::new();
    msg.attributes_mut().insert_in("a", 1).unwrap();
    msg.clear_attributes();
    assert!(msg.attributes().is_empty());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn decode_merge_last_wins_for_duplicate_keys() {
    // Two wire entries for the same key: later value wins on merge/decode.
    let mut first = Holder::new();
    first.attributes_mut().insert_in("k", 1).unwrap();
    let mut second = Holder::new();
    second.attributes_mut().insert_in("k", 9).unwrap();

    let mut bytes = first.encode_to_vec();
    bytes.extend_from_slice(&second.encode_to_vec());

    let decoded: Holder = Holder::decode(&bytes[..]).expect("decode");
    assert_eq!(decoded.attributes().len(), 1);
    assert_eq!(decoded.attributes().get("k").copied(), Some(9));
}
