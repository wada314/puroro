//! `(puroro.string_layout) = HEAP` uses heap string layout (`DerefMut` to `String`).

use crate::string_layout::HeapString;
use ::allocator_api2::alloc::Global;
use ::puroro::{Message, String as PuroroString, StringMut};

#[test]
fn heap_field_deref_mut_round_trip() {
    let mut msg = HeapString::new();
    *msg.body_mut() = PuroroString::from_str_in("hello", Global);
    assert!(msg.body().is_set());
    assert_eq!(msg.body().get(), "hello");

    let decoded: HeapString = HeapString::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.body().get(), "hello");
}

#[test]
fn default_string_still_uses_string_mut() {
    let mut msg = HeapString::new();
    msg.title_mut().set("sso");
    assert_eq!(msg.title().get(), "sso");
}
