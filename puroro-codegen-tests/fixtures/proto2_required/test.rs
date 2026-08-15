//! proto2 `required` → `LegacyRequired` / `Message::validate`.

use crate::proto2_required::RequiredOwner;
use ::puroro::{DecodeError, Message, StringMut};

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
    msg.owner_mut().push_str("alice");
    *msg.score_mut() = 7;
    msg.validate().expect("required set");

    let decoded: RequiredOwner = RequiredOwner::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.owner().get(), "alice");
    assert_eq!(decoded.score().get(), 7);
    decoded.validate().expect("decoded still valid");
}

#[test]
fn clear_fails_validate_again() {
    let mut msg = RequiredOwner::new();
    msg.owner_mut().push_str("alice");
    msg.validate().expect("set");
    msg.clear_owner();
    assert!(!msg.owner().is_set());
    assert!(matches!(
        msg.validate(),
        Err(DecodeError::MissingRequiredField { field_number: 1 })
    ));
}
