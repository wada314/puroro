//! Real oneof: string / scalar / bool / message / enum / bytes, plus a second group.

use crate::oneof_basic::holder::{Alt, AltCase, Choice, ChoiceCase};
use crate::oneof_basic::{Holder, Kind, Peer};
use ::puroro::{Message, OneofView, OneofViewMut, StringMut};

#[test]
fn unset_encodes_empty() {
    let msg = Holder::new();
    assert!(msg.choice().case().is_none());
    assert!(msg.alt().case().is_none());
    assert!(!msg.email().is_set());
    assert!(!msg.code().is_set());
    assert!(!msg.urgent().is_set());
    assert!(msg.peer().is_none());
    assert!(!msg.kind().is_set());
    assert!(!msg.blob().is_set());
    assert!(!msg.note().is_set());
    assert!(!msg.rank().is_set());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn string_variant_round_trip() {
    let mut msg = Holder::new();
    msg.email_mut().push_str("a@b.c");
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Email));
    assert_eq!(msg.email().get(), "a@b.c");

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.choice().case(), Some(ChoiceCase::Email));
    assert_eq!(decoded.email().get(), "a@b.c");
}

#[test]
fn scalar_variant_round_trip() {
    let mut msg = Holder::new();
    *msg.code_mut() = 7;
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Code));
    assert_eq!(msg.code().get(), 7);

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.code().get(), 7);
}

#[test]
fn bool_false_distinct_from_unset() {
    let mut msg = Holder::new();
    *msg.urgent_mut() = false;
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Urgent));
    assert!(msg.urgent().is_set());
    assert!(!msg.urgent().get());
    assert!(!msg.encode_to_vec().is_empty());

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert!(decoded.urgent().is_set());
    assert!(!decoded.urgent().get());
}

#[test]
fn message_variant_round_trip() {
    let mut msg = Holder::new();
    msg.peer_mut().name_mut().push_str("p");
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Peer));
    assert_eq!(msg.peer().unwrap().name().get(), "p");

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.peer().unwrap().name().get(), "p");
    let _: Peer = Peer::new();
}

#[test]
fn empty_message_variant_still_selects_case() {
    // Selecting a message oneof variant with an empty child must:
    // - expose Some(peer) / Case::Peer from getters
    // - encode a LEN record with length 0 (presence on the wire)
    // - decode back to an active empty peer (not unset)
    let mut msg = Holder::new();
    let _ = msg.peer_mut();
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Peer));
    assert!(msg.peer().is_some());
    assert!(!msg.peer().unwrap().name().is_set());

    // field 4 LEN: tag = (4 << 3) | 2 = 0x22, length 0
    let bytes = msg.encode_to_vec();
    assert_eq!(bytes, [0x22, 0x00]);

    let decoded: Holder = Holder::decode(&bytes[..]).expect("decode empty peer");
    assert_eq!(decoded.choice().case(), Some(ChoiceCase::Peer));
    assert!(decoded.peer().is_some());
    assert!(!decoded.peer().unwrap().name().is_set());
    assert_eq!(decoded.encode_to_vec(), [0x22, 0x00]);
}

#[test]
fn switching_variants_clears_previous() {
    let mut msg = Holder::new();
    msg.email_mut().push_str("x");
    assert!(msg.email().is_set());
    *msg.code_mut() = 1;
    assert!(!msg.email().is_set());
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Code));
    assert_eq!(msg.code().get(), 1);
}

#[test]
fn clear_group_unsets() {
    let mut msg = Holder::new();
    *msg.code_mut() = 3;
    msg.clear_choice();
    assert!(msg.choice().case().is_none());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn shape_projection_via_as_ref() {
    let mut msg = Holder::new();
    msg.email_mut().push_str("z");
    match msg.choice().as_ref() {
        Some(Choice::Email(s)) => assert_eq!(s, "z"),
        Some(Choice::Code(_)) => panic!("expected Email, got Code"),
        Some(Choice::Urgent(_)) => panic!("expected Email, got Urgent"),
        Some(Choice::Peer(_)) => panic!("expected Email, got Peer"),
        Some(Choice::Kind(_)) => panic!("expected Email, got Kind"),
        Some(Choice::Blob(_)) => panic!("expected Email, got Blob"),
        None => panic!("expected Email, got None"),
    }
    msg.choice_mut().clear();
    assert!(msg.choice().case().is_none());
}

#[test]
fn merge_switches_oneof_case() {
    let mut msg = Holder::new();
    msg.email_mut().push_str("a@b.c");
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Email));

    let mut other = Holder::new();
    *other.code_mut() = 7;
    let bytes = other.encode_to_vec();
    msg.merge_from(&mut &bytes[..]).expect("merge");

    assert!(!msg.email().is_set());
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Code));
    assert_eq!(msg.code().get(), 7);
}

#[test]
fn type_default_int32_zero_selects_case() {
    let mut msg = Holder::new();
    *msg.code_mut() = 0;
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Code));
    assert!(msg.code().is_set());
    assert_eq!(msg.code().get(), 0);
    // field 2 varint 0: tag = (2 << 3) | 0 = 0x10
    assert_eq!(msg.encode_to_vec(), [0x10, 0x00]);

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.choice().case(), Some(ChoiceCase::Code));
    assert!(decoded.code().is_set());
    assert_eq!(decoded.code().get(), 0);
}

#[test]
fn enum_and_bytes_variants_round_trip() {
    let mut msg = Holder::new();
    *msg.kind_mut() = Kind::A;
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Kind));
    assert_eq!(msg.kind().get(), Kind::A);

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode kind");
    assert_eq!(decoded.choice().case(), Some(ChoiceCase::Kind));
    assert_eq!(decoded.kind().get(), Kind::A);

    msg.blob_mut().extend_from_slice(b"xy");
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Blob));
    assert!(!msg.kind().is_set());
    assert_eq!(msg.blob().get(), b"xy");

    let decoded: Holder = Holder::decode(&msg.encode_to_vec()[..]).expect("decode blob");
    assert_eq!(decoded.choice().case(), Some(ChoiceCase::Blob));
    assert_eq!(decoded.blob().get(), b"xy");
}

#[test]
fn multiple_oneofs_are_independent() {
    let mut msg = Holder::new();
    msg.email_mut().push_str("a");
    msg.note_mut().push_str("n");
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Email));
    assert_eq!(msg.alt().case(), Some(AltCase::Note));
    assert!(matches!(msg.alt().as_ref(), Some(Alt::Note(s)) if s == "n"));

    *msg.rank_mut() = 2;
    assert_eq!(msg.choice().case(), Some(ChoiceCase::Email));
    assert_eq!(msg.email().get(), "a");
    assert_eq!(msg.alt().case(), Some(AltCase::Rank));
    assert_eq!(msg.rank().get(), 2);

    msg.clear_choice();
    assert!(msg.choice().case().is_none());
    assert_eq!(msg.alt().case(), Some(AltCase::Rank));
    assert_eq!(msg.rank().get(), 2);
}
