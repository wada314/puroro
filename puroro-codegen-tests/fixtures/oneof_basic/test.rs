//! Real oneof: string / scalar / bool / message variants.

use crate::oneof_basic::holder::{Choice, ChoiceCase};
use crate::oneof_basic::{Holder, Peer};
use ::puroro::{Message, OneofView, OneofViewMut};

#[test]
fn unset_encodes_empty() {
    let msg = Holder::new();
    assert!(msg.choice().case().is_none());
    assert!(!msg.email().is_set());
    assert!(!msg.code().is_set());
    assert!(!msg.urgent().is_set());
    assert!(msg.peer().is_none());
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
        None => panic!("expected Email, got None"),
    }
    msg.choice_mut().clear();
    assert!(msg.choice().case().is_none());
}
