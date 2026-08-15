//! Cross-file message import (`task.proto` → `address.proto`).

use crate::message_import::demo::{Address, Task};
use ::puroro::{Message, StringMut};

#[test]
fn imported_address_round_trip() {
    let mut task = Task::new();
    task.assignee_mut().street_mut().push_str("Side");
    let decoded: Task = Task::decode(&task.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.assignee().unwrap().street().get(), "Side");

    // Type is reachable as the shared package peer.
    let _: Address = Address::new();
}
