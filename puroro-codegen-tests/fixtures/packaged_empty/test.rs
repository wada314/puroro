//! Behavioural tests for an empty message under a non-empty protobuf package.
//!
//! Checks that `package example.v1` maps to nested Rust modules and that the
//! message type is reachable as `example::v1::Empty`. Unknown-field preserve
//! for empty messages is covered by the `empty` fixture.

use crate::packaged_empty::example::v1::Empty;
use puroro::Message;

#[test]
fn package_path_resolves() {
    let msg = Empty::new();
    assert!(msg.encode_to_vec().is_empty());
}
