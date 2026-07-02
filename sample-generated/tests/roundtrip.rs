use ::puroro::{MessageDecode, MessageEncode};
use ::puroro_sample_generated::{Address, Priority, Status, Task};

#[test]
fn task_roundtrip() {
    let mut task = Task::new();
    task.title_mut().push_str("Write docs");
    *task.score_mut() = 42;
    *task.max_retries_mut() = 5;
    task.owner_id_mut().push_str("user-1");
    task.payload_mut().extend_from_slice(b"data");
    task.tag_ids_mut().push(10);
    task.tag_ids_mut().push(20);
    task.scores_mut().push(1);
    task.push_label("urgent");
    *task.status_mut() = i32::from(Status::Pending);
    *task.priority_mut() = i32::from(Priority::High);
    task.set_email_address("a@example.com");

    let mut assignee = Address::new();
    assignee.street_mut().push_str("1 Main St");
    assignee.city_mut().push_str("Tokyo");
    *task.assignee_mut() = assignee;

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
        Some(puroro_sample_generated::task::Notification::EmailAddress(s)) if &**s == "a@example.com"
    ));
    let a = decoded.assignee().unwrap();
    assert_eq!(a.street().get(), "1 Main St");
    assert_eq!(a.city().get(), "Tokyo");
}

#[test]
fn closed_enum_unknown_goes_to_unknown_fields() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");

    // field 10 (priority) = 99 (unknown closed enum value), wire: tag + varint
    let mut bytes = Vec::new();
    ::puroro::encode::encode_varint_field(10, 99, &mut bytes);

    task.merge_from(&mut &bytes[..]).unwrap();

    assert!(task.priority().is_none());
    assert!(!task.unknown_fields().is_empty());
}
