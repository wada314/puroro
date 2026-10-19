//! `(puroro.bytes_layout) = HEAP` uses heap bytes layout (`DerefMut` to `Vec`).

use crate::bytes_layout::HeapBytes;
use ::allocator_api2::alloc::Global;
use ::allocator_api2::vec::Vec as AllocVec;
use ::puroro::{BytesMut, Message};

#[test]
fn heap_field_deref_mut_round_trip() {
    let mut msg = HeapBytes::new();
    let mut body = AllocVec::new_in(Global);
    body.extend_from_slice(b"hello");
    *msg.body_mut() = body;
    assert!(msg.body().is_set());
    assert_eq!(msg.body().get(), b"hello");

    let decoded: HeapBytes = HeapBytes::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.body().get(), b"hello");
}

#[test]
fn default_bytes_still_uses_bytes_mut() {
    let mut msg = HeapBytes::new();
    msg.title_mut().set(b"sso");
    assert_eq!(msg.title().get(), b"sso");
}
