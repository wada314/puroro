//! Maps: all legal keys × scalar / string / bytes / enum / message values.

use crate::map_basic::{Holder, Kind, Peer};
use ::puroro::{MapMut, MapRef, Message};

#[test]
fn defaults_empty() {
    let msg = Holder::new();
    assert!(msg.attributes().is_empty());
    assert!(msg.flags().is_empty());
    assert!(msg.labels().is_empty());
    assert!(msg.peers().is_empty());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn string_key_entry_mut_get_and_round_trip() {
    let mut msg = Holder::new();
    {
        let mut attrs = msg.attributes_mut();
        *attrs.entry_mut("region") = 81;
        *attrs.entry_mut("tier") = 2;
    }
    assert_eq!(msg.attributes().get("region").copied(), Some(81));

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.attributes().get("region").copied(), Some(81));
    assert_eq!(decoded.attributes().get("tier").copied(), Some(2));
}

#[test]
fn string_key_entry_mut_same_key_last_wins() {
    let mut msg = Holder::new();
    {
        let mut attrs = msg.attributes_mut();
        *attrs.entry_mut("k") = 1;
        *attrs.entry_mut("k") = 2;
        *attrs.get_mut("k").unwrap() = 3;
    }
    assert_eq!(msg.attributes().get("k").copied(), Some(3));

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.attributes().get("k").copied(), Some(3));
}

#[test]
fn clear_empties_string_map() {
    let mut msg = Holder::new();
    *msg.attributes_mut().entry_mut("a") = 1;
    msg.clear_attributes();
    assert!(msg.attributes().is_empty());
}

#[test]
fn decode_merge_last_wins_for_duplicate_string_keys() {
    let mut first = Holder::new();
    *first.attributes_mut().entry_mut("k") = 1;
    let mut second = Holder::new();
    *second.attributes_mut().entry_mut("k") = 9;

    let mut bytes = first.encode_to_vec();
    bytes.extend_from_slice(&second.encode_to_vec());

    let decoded: Holder = Holder::decode(&bytes[..]).expect("decode");
    assert_eq!(decoded.attributes().get("k").copied(), Some(9));
}

#[test]
fn int32_key_entry_mut_and_round_trip() {
    let mut msg = Holder::new();
    {
        let mut flags = msg.flags_mut();
        *flags.entry_mut(1) = true;
        *flags.entry_mut(2) = false;
        *flags.entry_mut(1) = false;
    }
    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.flags().get(&1).copied(), Some(false));
    assert_eq!(decoded.flags().get(&2).copied(), Some(false));
}

#[test]
fn bool_key_uint64_value_round_trip() {
    let mut msg = Holder::new();
    *msg.counters_mut().entry_mut(true) = 99;
    *msg.counters_mut().entry_mut(false) = 7;
    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.counters().get(&true).copied(), Some(99));
    assert_eq!(decoded.counters().get(&false).copied(), Some(7));
}

#[test]
fn clear_sized_key_map() {
    let mut msg = Holder::new();
    *msg.flags_mut().entry_mut(3) = true;
    msg.clear_flags();
    assert!(msg.flags().is_empty());
}

#[test]
fn string_value_entry_mut_round_trip() {
    let mut msg = Holder::new();
    msg.labels_mut().entry_mut(1).push_str("hello");
    msg.labels_mut().entry_mut(2).push_str("world");
    assert_eq!(msg.labels().get(&1), Some("hello"));

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.labels().get(&1), Some("hello"));
    assert_eq!(decoded.labels().get(&2), Some("world"));
}

#[test]
fn bytes_value_entry_mut_round_trip() {
    let mut msg = Holder::new();
    msg.blobs_mut().entry_mut(7).extend_from_slice(b"abc");
    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.blobs().get(&7), Some(b"abc".as_slice()));
}

#[test]
fn enum_value_entry_mut_round_trip() {
    let mut msg = Holder::new();
    *msg.kinds_mut().entry_mut(1) = Kind::A;
    *msg.kinds_mut().entry_mut(2) = Kind::B;
    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.kinds().get(&1).copied(), Some(Kind::A));
    assert_eq!(decoded.kinds().get(&2).copied(), Some(Kind::B));
}

#[test]
fn message_value_entry_mut_round_trip() {
    let mut msg = Holder::new();
    msg.peers_mut()
        .entry_mut(9)
        .name_mut()
        .push_str("alice");
    assert_eq!(msg.peers().get(&9).unwrap().name().get(), "alice");

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.peers().get(&9).unwrap().name().get(), "alice");
    let _: Peer = Peer::new();
}

#[test]
fn string_to_string_entry_mut_round_trip() {
    let mut msg = Holder::new();
    msg.aliases_mut().entry_mut("k").push_str("v");
    assert_eq!(msg.aliases().get("k"), Some("v"));

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.aliases().get("k"), Some("v"));
}
