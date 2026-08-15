//! Nested message/enum declarations + repeated fields.

use crate::nested_repeated::demo::outer::{Inner, Kind};
use crate::nested_repeated::demo::Outer;
use ::puroro::{Message, StringMut};

#[test]
fn defaults() {
    let msg = Outer::new();
    assert!(msg.tag_ids().is_empty());
    assert!(msg.inners().is_empty());
    assert!(!msg.kind().is_set());
    assert!(msg.child().is_none());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn repeated_scalars_and_messages_round_trip() {
    let mut msg = Outer::new();
    msg.tag_ids_mut().extend_from_slice(&[1, 2, 3]);
    let mut inner = Inner::new();
    inner.name_mut().push_str("x");
    msg.inners_mut().push(inner);
    *msg.kind_mut() = Kind::A;
    msg.child_mut().name_mut().push_str("y");

    let decoded: Outer = Outer::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.tag_ids(), &[1, 2, 3]);
    assert_eq!(decoded.inners().len(), 1);
    assert_eq!(decoded.inners()[0].name().get(), "x");
    assert_eq!(decoded.kind().get(), Kind::A);
    assert_eq!(decoded.child().unwrap().name().get(), "y");
}
