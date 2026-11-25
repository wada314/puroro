//! Message names that snake_case to Rust keywords → raw module idents.
//!
//! Compile gate: nested path `r#struct::Match` (and companion `r#struct::r#match`)
//! must resolve. Keywords that cannot be raw identifiers (`self` / `Self` /
//! `crate` / `super`) stay out of scope (same as `keyword_field`).

use crate::keyword_type::r#struct::Match;
use crate::keyword_type::{Holder, Struct, Type};
use ::puroro::{Message, StringMut};

#[test]
fn keyword_type_modules_round_trip() {
    let mut holder = Holder::new();
    *holder.payload_mut().nest_mut().value_mut() = 11;
    holder.payload_mut().r#type_mut().push_str("kw");
    *holder.wrapper_mut().r#for_mut() = -3;

    assert_eq!(holder.payload().unwrap().nest().unwrap().value().get(), 11);
    assert_eq!(holder.payload().unwrap().r#type().get(), "kw");
    assert_eq!(holder.wrapper().unwrap().r#for().get(), -3);

    let decoded: Holder = Holder::decode(&holder.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.payload().unwrap().nest().unwrap().value().get(), 11);
    assert_eq!(decoded.payload().unwrap().r#type().get(), "kw");
    assert_eq!(decoded.wrapper().unwrap().r#for().get(), -3);

    // Keep a value use so the nested raw-ident path stays referenced.
    assert!(Match::new().encode_to_vec().is_empty());
    assert!(Struct::new().encode_to_vec().is_empty());
    assert!(Type::new().encode_to_vec().is_empty());
}
