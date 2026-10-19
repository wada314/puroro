//! `repeated string` / `repeated bytes` (Expanded LEN payload).

use crate::repeated_len::Labels;
use ::puroro::{Message, RepeatedBytesMut, RepeatedStringMut};

#[test]
fn defaults_empty() {
    let msg = Labels::new();
    assert!(msg.labels().is_empty());
    assert!(msg.blobs().is_empty());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn push_and_round_trip() {
    let mut msg = Labels::new();
    msg.labels_mut().push().push_str("a");
    msg.labels_mut().push().push_str("b");
    msg.blobs_mut().push().extend_from_slice(b"xy");
    msg.blobs_mut().push().extend_from_slice(b"z");

    let decoded: Labels = Labels::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.labels().len(), 2);
    assert_eq!(&*decoded.labels()[0], "a");
    assert_eq!(&*decoded.labels()[1], "b");
    assert_eq!(decoded.blobs().len(), 2);
    assert_eq!(&*decoded.blobs()[0], b"xy");
    assert_eq!(&*decoded.blobs()[1], b"z");
}

#[test]
fn empty_elements_round_trip() {
    let mut msg = Labels::new();
    msg.labels_mut().push(); // ""
    msg.blobs_mut().push(); // empty bytes

    let decoded: Labels = Labels::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.labels().len(), 1);
    assert_eq!(&*decoded.labels()[0], "");
    assert_eq!(decoded.blobs().len(), 1);
    assert_eq!(&*decoded.blobs()[0], b"");
}

#[test]
fn merge_appends() {
    let mut msg = Labels::new();
    msg.labels_mut().push().push_str("a");
    msg.blobs_mut().push().extend_from_slice(b"1");

    let mut other = Labels::new();
    other.labels_mut().push().push_str("b");
    other.blobs_mut().push().extend_from_slice(b"2");
    let bytes = other.encode_to_vec();
    msg.merge_from(&mut &bytes[..]).expect("merge");

    assert_eq!(msg.labels().len(), 2);
    assert_eq!(&*msg.labels()[0], "a");
    assert_eq!(&*msg.labels()[1], "b");
    assert_eq!(msg.blobs().len(), 2);
    assert_eq!(&*msg.blobs()[0], b"1");
    assert_eq!(&*msg.blobs()[1], b"2");
}
