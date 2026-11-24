//! Proto field names that are Rust keywords → raw idents.
//!
//! Keywords that cannot be raw identifiers (`self` / `Self` / `crate` / `super`)
//! are out of scope for this fixture.

use crate::keyword_field::KeywordFields;
use ::puroro::{BytesMut, Message, StringMut};

#[test]
fn keyword_accessors_round_trip() {
    let mut msg = KeywordFields::new();
    *msg.r#type_mut() = 7;
    msg.r#match_mut().push_str("x");
    *msg.r#async_mut() = true;
    *msg.r#for_mut() = -9;
    msg.r#mod_mut().extend_from_slice(b"yz");

    assert!(msg.r#type().is_set());
    assert_eq!(msg.r#type().get(), 7);
    assert_eq!(msg.r#match().get(), "x");
    assert!(msg.r#async().is_set());
    assert!(msg.r#async().get());
    assert_eq!(msg.r#for().get(), -9);
    assert_eq!(msg.r#mod().get(), b"yz");

    let decoded: KeywordFields = KeywordFields::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.r#type().get(), 7);
    assert_eq!(decoded.r#match().get(), "x");
    assert!(decoded.r#async().get());
    assert_eq!(decoded.r#for().get(), -9);
    assert_eq!(decoded.r#mod().get(), b"yz");

    msg.clear_type();
    msg.clear_match();
    msg.clear_async();
    msg.clear_for();
    msg.clear_mod();
    assert!(!msg.r#type().is_set());
    assert!(!msg.r#match().is_set());
    assert!(!msg.r#async().is_set());
    assert!(!msg.r#for().is_set());
    assert!(!msg.r#mod().is_set());
}
