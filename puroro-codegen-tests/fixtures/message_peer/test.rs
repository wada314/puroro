//! Same-file peer message field (`Task.assignee: Address`).

use crate::message_peer::demo::{Address, Task};
use ::puroro::Message;

#[test]
fn assignee_starts_unset() {
    let task = Task::new();
    assert!(task.assignee().is_none());
    assert!(task.encode_to_vec().is_empty());
}

#[test]
fn assignee_mut_creates_and_clear_removes() {
    let mut task = Task::new();
    task.assignee_mut().street_mut().push_str("Main");
    assert!(task.assignee().is_some());
    assert_eq!(task.assignee().unwrap().street().get(), "Main");

    task.clear_assignee();
    assert!(task.assignee().is_none());
    assert!(task.encode_to_vec().is_empty());
}

#[test]
fn empty_child_still_encodes_presence() {
    let mut task = Task::new();
    let _ = task.assignee_mut();
    let bytes = task.encode_to_vec();
    // tag for field 1 LEN = (1<<3)|2 = 10, length 0
    assert_eq!(bytes, [0x0a, 0x00]);

    let decoded: Task = Task::decode(&bytes[..]).expect("decode");
    assert!(decoded.assignee().is_some());
    assert!(!decoded.assignee().unwrap().street().is_set());
}

#[test]
fn child_fields_merge_across_occurrences() {
    // Two LEN records for field 1:
    // 1) street="A"  → tag 0x0a, len, then street field tag 0x0a "A"
    // 2) street="B"  → last-wins for string, so final street is "B"
    // Build via encode helpers on Address instead of raw bytes for clarity.
    let mut first = Address::new();
    first.street_mut().push_str("A");
    let mut second = Address::new();
    second.street_mut().push_str("B");

    let mut wire = Vec::new();
    // field 1 LEN wrapping first Address bytes
    let a = first.encode_to_vec();
    wire.push(0x0a);
    wire.push(a.len() as u8);
    wire.extend_from_slice(&a);
    let b = second.encode_to_vec();
    wire.push(0x0a);
    wire.push(b.len() as u8);
    wire.extend_from_slice(&b);

    let task: Task = Task::decode(&wire[..]).expect("decode");
    assert_eq!(task.assignee().unwrap().street().get(), "B");
}

#[test]
fn round_trip() {
    let mut task = Task::new();
    task.assignee_mut().street_mut().push_str("Oak");
    let decoded: Task = Task::decode(&task.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.assignee().unwrap().street().get(), "Oak");
    assert_eq!(decoded, task);
}
