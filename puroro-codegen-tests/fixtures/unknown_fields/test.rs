//! `(puroro.unknown_fields)` preserve, discard, and reject.

use crate::unknown_fields::{
    Discard, DiscardChoice, Keep, KeepParent, Reject, RejectChoice, RejectClosed, RejectLazy,
    RejectParent, Status,
};
use ::core::mem::size_of;
use ::puroro::{DecodeError, Message};

#[test]
fn keep_preserves_unknown_tag() {
    let wire = [0x08_u8, 0x03, 0x50, 0x05]; // n=3, field 10 = 5
    let msg: Keep = Keep::decode(&wire[..]).expect("decode");
    assert_eq!(msg.n().get(), 3);
    assert_eq!(msg.unknown_fields().count(), 1);
    assert_eq!(msg.encode_to_vec(), wire);
}

#[test]
fn discard_drops_unknown_tag() {
    let wire = [0x08_u8, 0x03, 0x50, 0x05]; // n=3, field 10 = 5
    let msg: Discard = Discard::decode(&wire[..]).expect("decode");
    assert_eq!(msg.n().get(), 3);
    assert!(msg.unknown_fields().next().is_none());
    assert_eq!(msg.encode_to_vec(), [0x08, 0x03]);
}

#[test]
fn discard_message_is_smaller() {
    assert!(size_of::<Keep>() > size_of::<Discard>());
}

#[test]
fn discard_oneof_drops_unknown_and_keeps_variant() {
    let mut wire = Vec::new();
    // choice.a = 7 (field 1 varint)
    wire.extend_from_slice(&[0x08, 0x07]);
    // unknown field 10 = 5
    wire.extend_from_slice(&[0x50, 0x05]);

    let msg: DiscardChoice = DiscardChoice::decode(&wire[..]).expect("decode");
    assert_eq!(msg.a().get(), 7);
    assert!(msg.unknown_fields().next().is_none());
    assert_eq!(msg.encode_to_vec(), [0x08, 0x07]);
}

#[test]
fn reject_fails_unknown_tag_and_keeps_known_field_round_trip() {
    let known = [0x08_u8, 0x03];
    let msg: Reject = Reject::decode(&known[..]).expect("decode");
    assert_eq!(msg.n().get(), 3);
    assert!(msg.unknown_fields().next().is_none());
    assert_eq!(msg.encode_to_vec(), known);

    let wire = [0x08_u8, 0x03, 0x50, 0x05]; // n=3, field 10 = 5
    let result: Result<Reject, _> = Reject::decode(&wire[..]);
    assert_eq!(
        result.unwrap_err(),
        DecodeError::UnrecognizedField { field_number: 10 }
    );
}

#[test]
fn reject_message_matches_discard_size() {
    assert_eq!(size_of::<Reject>(), size_of::<Discard>());
    assert!(size_of::<Keep>() > size_of::<Reject>());
}

#[test]
fn reject_oneof_keeps_variant_and_fails_unknown() {
    let known = [0x08_u8, 0x07];
    let msg: RejectChoice = RejectChoice::decode(&known[..]).expect("decode");
    assert_eq!(msg.a().get(), 7);
    assert_eq!(msg.encode_to_vec(), known);

    let mut wire = known.to_vec();
    wire.extend_from_slice(&[0x50, 0x05]);
    let result: Result<RejectChoice, _> = RejectChoice::decode(&wire[..]);
    assert_eq!(
        result.unwrap_err(),
        DecodeError::UnrecognizedField { field_number: 10 }
    );
}

#[test]
fn reject_lazy_fails_unknown_tag() {
    let known = [0x08_u8, 0x03];
    let lazy = RejectLazy::decode(&known[..]).expect("lazy decode");
    assert_eq!(lazy.n().expect("n").get(), 3);

    let wire = [0x08_u8, 0x03, 0x50, 0x05];
    let result: Result<RejectLazy, _> = RejectLazy::decode(&wire[..]);
    assert_eq!(
        result.unwrap_err(),
        DecodeError::UnrecognizedField { field_number: 10 }
    );
}

#[test]
fn reject_parent_keeps_child_unknowns() {
    // child = { n = 3, field 10 = 5 }
    let wire = [0x0a_u8, 0x04, 0x08, 0x03, 0x50, 0x05];
    let msg: RejectParent = RejectParent::decode(&wire[..]).expect("decode");
    let child = msg.child().expect("child");
    assert_eq!(child.n().get(), 3);
    assert_eq!(child.unknown_fields().count(), 1);
    assert!(msg.unknown_fields().next().is_none());
    assert_eq!(msg.encode_to_vec(), wire);
}

#[test]
fn reject_child_fails_parent_decode() {
    let wire = [0x0a_u8, 0x04, 0x08, 0x03, 0x50, 0x05];
    let result: Result<KeepParent, _> = KeepParent::decode(&wire[..]);
    assert_eq!(
        result.unwrap_err(),
        DecodeError::UnrecognizedField { field_number: 10 }
    );
}

#[test]
fn reject_closed_enum_unknown_is_not_an_error() {
    // field 1 varint 99
    let wire = [0x08_u8, 99];
    let msg: RejectClosed = RejectClosed::decode(&wire[..]).expect("decode");
    assert!(!msg.status().is_set());
    assert_eq!(msg.status().get(), Status::UNSPECIFIED);
    assert!(msg.unknown_fields().next().is_none());
    assert!(msg.encode_to_vec().is_empty());
}
