//! Behavioural tests for an empty message under a non-empty protobuf package.
//!
//! Checks that `package example.v1` maps to nested Rust modules and that the
//! message type is re-exported from the package leaf (`example::v1::Empty`).

use crate::packaged_empty::example::v1::Empty;
use puroro::Message;

#[test]
fn package_path_resolves() {
    let msg = Empty::new();
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn round_trip_unknown_field_under_package() {
    let wire = [0x08_u8, 0x96, 0x01];
    let msg: Empty = Empty::decode(&wire[..]).expect("decode unknown");
    assert_eq!(msg.encode_to_vec(), wire);
}
