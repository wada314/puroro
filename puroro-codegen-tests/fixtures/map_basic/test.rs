//! Maps: string / int32 / bool keys with Copy scalar values.

use crate::map_basic::Holder;
use ::puroro::{MapEntryMut, MapMut, MapRef, Message};

#[test]
fn defaults_empty() {
    let msg = Holder::new();
    assert!(msg.attributes().is_empty());
    assert!(msg.flags().is_empty());
    assert!(msg.counters().is_empty());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn string_key_insert_get_and_round_trip() {
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
fn string_key_insert_same_key_last_wins() {
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
fn clear_empties_string_map() {
    let mut msg = Holder::new();
    msg.attributes_mut().insert_in("a", 1).unwrap();
    msg.clear_attributes();
    assert!(msg.attributes().is_empty());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn decode_merge_last_wins_for_duplicate_string_keys() {
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

#[test]
fn int32_key_insert_and_round_trip() {
    let mut msg = Holder::new();
    {
        let mut flags = msg.flags_mut();
        flags.insert(1, true);
        flags.insert(2, false);
        flags.insert(1, false);
    }
    assert_eq!(msg.flags().len(), 2);
    assert_eq!(msg.flags().get(&1).copied(), Some(false));
    assert_eq!(msg.flags().get(&2).copied(), Some(false));

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.flags().get(&1).copied(), Some(false));
    assert_eq!(decoded.flags().get(&2).copied(), Some(false));
}

#[test]
fn bool_key_uint64_value_round_trip() {
    let mut msg = Holder::new();
    msg.counters_mut().insert(true, 99);
    msg.counters_mut().insert(false, 7);
    assert_eq!(msg.counters().get(&true).copied(), Some(99));
    assert_eq!(msg.counters().get(&false).copied(), Some(7));

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.counters().get(&true).copied(), Some(99));
    assert_eq!(decoded.counters().get(&false).copied(), Some(7));
}

#[test]
fn clear_sized_key_map() {
    let mut msg = Holder::new();
    msg.flags_mut().insert(3, true);
    msg.clear_flags();
    assert!(msg.flags().is_empty());
}
