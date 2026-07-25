//! Proto field names that are Rust keywords → raw idents.

use crate::keyword_field::KeywordFields;
use ::puroro::Message;

#[test]
fn keyword_accessors_round_trip() {
    let mut msg = KeywordFields::new();
    *msg.r#type_mut() = 7;
    msg.r#match_mut().push_str("x");

    assert!(msg.r#type().is_set());
    assert_eq!(msg.r#type().get(), 7);
    assert_eq!(msg.r#match().get(), "x");

    let decoded: KeywordFields = KeywordFields::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.r#type().get(), 7);
    assert_eq!(decoded.r#match().get(), "x");

    msg.clear_type();
    msg.clear_match();
    assert!(!msg.r#type().is_set());
    assert!(!msg.r#match().is_set());
}
