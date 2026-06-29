use ::puroro::{MessageDecode, MessageEncode};
use ::puroro_sample_generated::{Address, Priority, Status, Task};

#[test]
fn task_roundtrip() {
    let mut task = Task::new();
    task.set_title("Write docs");
    task.set_score(42);
    task.set_max_retries(5);
    task.set_owner_id("user-1");
    task.set_payload(b"data").unwrap();
    task.push_tag_id(10);
    task.push_tag_id(20);
    task.push_score(1);
    task.push_label("urgent");
    task.set_status(Status::Pending);
    task.set_priority(Priority::High);
    task.set_email_address("a@example.com");

    let mut assignee = Address::new();
    assignee.set_street("1 Main St");
    assignee.set_city("Tokyo");
    task.set_assignee(assignee);

    task.validate().unwrap();

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();

    assert_eq!(decoded.title().get(), "Write docs");
    assert_eq!(decoded.score(), 42);
    assert_eq!(decoded.max_retries().get(), 5);
    assert!(decoded.has_owner_id());
    assert_eq!(decoded.payload().get(), b"data");
    assert_eq!(decoded.tag_ids(), &[10, 20]);
    assert_eq!(decoded.scores(), &[1]);
    assert_eq!(decoded.labels().len(), 1);
    assert_eq!(&*decoded.labels()[0], "urgent");
    assert_eq!(decoded.status().unwrap(), Status::Pending);
    assert_eq!(decoded.priority().unwrap().unwrap(), Priority::High);
    assert!(matches!(
        decoded.notification(),
        Some(puroro_sample_generated::task::Notification::EmailAddress(s)) if s.as_ref() == "a@example.com"
    ));
    let a = decoded.assignee().unwrap();
    assert_eq!(a.street().get(), "1 Main St");
    assert_eq!(a.city().get(), "Tokyo");
}

#[test]
fn closed_enum_unknown_goes_to_unknown_fields() {
    let mut task = Task::new();
    task.set_owner_id("x");

    // field 10 (priority) = 99 (unknown closed enum value), wire: tag + varint
    let mut bytes = Vec::new();
    ::puroro::encode::encode_varint_field(10, 99, &mut bytes);

    task.merge_from(&mut &bytes[..]).unwrap();

    assert!(task.priority().is_none());
    assert!(!task.unknown_fields().is_empty());
}
