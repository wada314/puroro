//! Cross-file message import (`task.proto` → `address.proto`).
//!
//! Compile gate: imported peer `Address` resolves via the `use` below.

use crate::message_import::demo::{Address, Task};
use ::puroro::{Message, StringMut};

#[test]
fn imported_address_round_trip() {
    let mut task = Task::new();
    task.assignee_mut().street_mut().push_str("Side");
    let decoded: Task = Task::decode(&task.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.assignee().unwrap().street().get(), "Side");

    assert!(Address::new().encode_to_vec().is_empty());
}
