//! proto2 `required` → `LegacyRequired` / boxed message `validate()`.

use crate::proto2_required::RequiredOwner;
use ::puroro::{BytesMut, DecodeError, Message};

fn set_all_required(msg: &mut RequiredOwner) {
    msg.owner_mut().set(b"alice");
    let _ = msg.addr_mut();
    let _ = msg.pos_mut();
}

#[test]
fn unset_fails_validate() {
    let msg = RequiredOwner::new();
    assert!(!msg.owner().is_set());
    match msg.validate() {
        Err(DecodeError::MissingRequiredField { field_number }) => {
            assert_eq!(field_number, 1);
        }
        other => panic!("expected MissingRequiredField, got {other:?}"),
    }
}

#[test]
fn set_passes_validate_and_round_trips() {
    let mut msg = RequiredOwner::new();
    set_all_required(&mut msg);
    *msg.score_mut() = 7;
    msg.validate().expect("required set");

    let decoded: RequiredOwner = RequiredOwner::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.owner().get(), b"alice");
    assert_eq!(decoded.score().get(), 7);
    assert!(decoded.addr().is_some());
    assert!(decoded.pos().is_some());
    decoded.validate().expect("decoded still valid");
}

#[test]
fn clear_fails_validate_again() {
    let mut msg = RequiredOwner::new();
    set_all_required(&mut msg);
    msg.validate().expect("set");
    msg.clear_owner();
    assert!(!msg.owner().is_set());
    assert!(matches!(
        msg.validate(),
        Err(DecodeError::MissingRequiredField { field_number: 1 })
    ));
}

#[test]
fn boxed_required_message_unset_fails_validate() {
    let mut msg = RequiredOwner::new();
    msg.owner_mut().set(b"alice");
    let _ = msg.pos_mut();
    assert!(msg.addr().is_none());
    assert!(matches!(
        msg.validate(),
        Err(DecodeError::MissingRequiredField { field_number: 3 })
    ));

    let _ = msg.addr_mut();
    msg.validate().expect("empty present Address is set");
    msg.clear_addr();
    assert!(matches!(
        msg.validate(),
        Err(DecodeError::MissingRequiredField { field_number: 3 })
    ));
}

#[test]
fn inlined_required_message_unset_fails_validate() {
    let mut msg = RequiredOwner::new();
    msg.owner_mut().set(b"alice");
    let _ = msg.addr_mut();
    assert!(msg.pos().is_none());
    assert!(matches!(
        msg.validate(),
        Err(DecodeError::MissingRequiredField { field_number: 4 })
    ));

    let _ = msg.pos_mut();
    msg.validate().expect("empty present Point is set");
    msg.clear_pos();
    assert!(matches!(
        msg.validate(),
        Err(DecodeError::MissingRequiredField { field_number: 4 })
    ));
}
