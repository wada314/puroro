//! `(puroro.message_layout)` and the auto-inline heuristic.

use crate::message_layout::{Address, Holder, Point, Recurse, RecurseOneof, Tiny};
use ::core::mem::size_of;
use ::puroro::{Message, StringMut};

#[test]
fn auto_inlined_origin_round_trip() {
    let mut holder = Holder::new();
    assert!(holder.origin().is_none());
    assert!(holder.encode_to_vec().is_empty());

    *holder.origin_mut().x_mut() = 3;
    *holder.origin_mut().y_mut() = 7;
    assert_eq!(holder.origin().unwrap().x(), 3);
    assert_eq!(holder.origin().unwrap().y(), 7);

    let decoded: Holder = Holder::decode(&holder.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.origin().unwrap().x(), 3);
    assert_eq!(decoded.origin().unwrap().y(), 7);

    holder.clear_origin();
    assert!(holder.origin().is_none());
    assert!(holder.encode_to_vec().is_empty());
}

#[test]
fn boxed_assignee_still_optional() {
    let mut holder = Holder::new();
    holder.assignee_mut().street_mut().push_str("Main");
    assert_eq!(holder.assignee().unwrap().street().get(), "Main");
    holder.clear_assignee();
    assert!(holder.assignee().is_none());
}

#[test]
fn force_boxed_tiny_and_force_inlined_address() {
    let mut holder = Holder::new();
    *holder.forced_box_mut().n_mut() = 9;
    holder.forced_inline_mut().city_mut().push_str("Kyoto");

    let decoded: Holder = Holder::decode(&holder.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.forced_box().unwrap().n(), 9);
    assert_eq!(decoded.forced_inline().unwrap().city().get(), "Kyoto");
}

#[test]
fn recursive_inline_hint_still_round_trips() {
    let mut nest = Recurse::new();
    let _ = nest.child_mut().child_mut();
    let decoded: Recurse = Recurse::decode(&nest.encode_to_vec()[..]).expect("decode");
    assert!(decoded.child().is_some());
    assert!(decoded.child().unwrap().child().is_some());
    assert!(decoded.child().unwrap().child().unwrap().child().is_none());
}

#[test]
fn oneof_postal_auto_inlines_and_round_trips() {
    let mut holder = Holder::new();
    *holder.postal_mut().x_mut() = 1;
    assert!(holder.postal().is_some());
    let decoded: Holder = Holder::decode(&holder.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.postal().unwrap().x(), 1);

    holder.boxed_postal_mut().city_mut().push_str("Osaka");
    let decoded: Holder = Holder::decode(&holder.encode_to_vec()[..]).expect("decode");
    assert!(decoded.postal().is_none());
    assert_eq!(decoded.boxed_postal().unwrap().city().get(), "Osaka");
}

#[test]
fn oneof_recursive_inline_hint_still_round_trips() {
    let mut nest = RecurseOneof::new();
    let _ = nest.child_mut().child_mut();
    let decoded: RecurseOneof = RecurseOneof::decode(&nest.encode_to_vec()[..]).expect("decode");
    assert!(decoded.child().is_some());
    assert!(decoded.child().unwrap().child().is_some());
    assert!(decoded.child().unwrap().child().unwrap().child().is_none());
}

#[test]
fn inlined_origin_embeds_child_size() {
    // Boxed singular message is a pointer-sized `Option<UnmanagedBox>`.
    // Inlined `Point` embeds the child (plus a presence bit in `_common`).
    assert!(size_of::<Holder>() >= size_of::<Point>() + size_of::<*const ()>());
    let _ = (size_of::<Address>(), size_of::<Tiny>());
}
