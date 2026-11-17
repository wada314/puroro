//! `TaskView` indexes one buffer. It does not copy field bytes and does not
//! rescan on get.

use ::allocator_api2::alloc::Global;
use ::puroro::{BytesMut, DecodeError, MapMut, Message, RepeatedStringMut, StringMut};
use ::puroro_sample_generated::task::NotificationCase;
use ::puroro_sample_generated::{
    Address, AddressView, Point, PointView, Priority, Status, Task, TaskView,
};
use ::std::vec::Vec;

fn task_with_owner(id: &str) -> Task {
    let mut task = Task::new();
    task.owner_id_mut().set(id);
    task
}

#[test]
fn empty_view_is_implicit_zero() {
    let view = TaskView::parse(&[]).unwrap();
    assert_eq!(view.title().unwrap(), None);
    assert!(!view.has_title());
    assert_eq!(view.score(), 0);
    assert_eq!(view.max_retries(), None);
    assert_eq!(view.owner_id().unwrap(), None);
    assert_eq!(view.payload().unwrap(), None);
    assert!(view.tag_ids().is_empty());
    assert!(view.scores().is_empty());
    assert!(view.labels().next().is_none());
    assert_eq!(view.status(), Status::UNSPECIFIED);
    assert_eq!(view.priority(), None);
    assert!(view.assignee().is_none());
    assert!(!view.done());
    assert_eq!(view.flag(), None);
    assert!(view.watchers().is_empty());
    assert!(view.votes().is_empty());
    assert_eq!(view.attribute("region").unwrap(), None);
    assert!(view.origin().is_none());
    assert_eq!(view.notification(), None);
}

#[test]
fn roundtrip_matches_eager_decode() {
    let mut task = task_with_owner("user-1");
    task.title_mut().set("Write docs");
    *task.score_mut() = 42;
    *task.max_retries_mut() = 5;
    task.payload_mut().extend_from_slice(b"data");
    task.tag_ids_mut().push(10);
    task.tag_ids_mut().push(20);
    task.scores_mut().push(1);
    task.labels_mut().push().push_str("urgent");
    *task.status_mut() = Status::PENDING;
    *task.priority_mut() = Priority::HIGH;
    *task.done_mut() = true;
    *task.flag_mut() = false;
    task.email_address_mut().set("a@example.com");

    let mut assignee = Address::new();
    assignee.street_mut().set("1 Main St");
    assignee.city_mut().set("Tokyo");
    *assignee.postal_code_mut() = 1_000_001;
    *assignee.latitude_mut() = 35.5;
    task.set_assignee(assignee);

    let mut watcher = Address::new();
    watcher.street_mut().set("2 Side Rd");
    task.watchers_mut().push(watcher);
    task.votes_mut().push(true);
    task.votes_mut().push(false);
    *task.attributes_mut().entry_mut("region") = 81;

    let mut origin = Point::new();
    *origin.x_mut() = 3;
    *origin.y_mut() = 4;
    task.set_origin(origin);

    let bytes = task.encode_to_vec();
    let view = TaskView::parse(&bytes).unwrap();
    assert_eq!(view.as_bytes(), bytes.as_slice());
    assert_eq!(view.title().unwrap(), Some("Write docs"));
    assert!(view.has_title());
    assert_eq!(view.score(), 42);
    assert_eq!(view.max_retries(), Some(5));
    assert_eq!(view.owner_id().unwrap(), Some("user-1"));
    assert_eq!(view.payload().unwrap(), Some(b"data".as_slice()));
    assert_eq!(view.tag_ids(), &[10, 20]);
    assert_eq!(view.scores(), &[1]);
    assert_eq!(
        view.labels().collect::<Result<Vec<_>, _>>().unwrap(),
        ["urgent"]
    );
    assert_eq!(view.status(), Status::PENDING);
    assert_eq!(view.priority(), Some(Priority::HIGH));
    assert!(view.done());
    assert_eq!(view.flag(), Some(false));
    assert_eq!(view.notification(), Some(NotificationCase::EmailAddress));
    assert_eq!(view.email_address().unwrap(), Some("a@example.com"));
    assert_eq!(view.phone_number().unwrap(), None);
    assert_eq!(view.webhook_id(), None);
    let assignee = view.assignee().unwrap();
    assert_eq!(assignee.street().unwrap(), Some("1 Main St"));
    assert_eq!(assignee.city().unwrap(), Some("Tokyo"));
    assert_eq!(assignee.postal_code(), Some(1_000_001));
    assert_eq!(assignee.latitude(), Some(35.5));
    assert_eq!(view.watchers().len(), 1);
    assert_eq!(view.watchers()[0].street().unwrap(), Some("2 Side Rd"));
    assert_eq!(view.votes(), &[true, false]);
    assert_eq!(view.attribute("region").unwrap(), Some(81));
    let origin = view.origin().unwrap();
    assert_eq!(origin.x(), 3);
    assert_eq!(origin.y(), 4);

    let owned = view.to_owned(Global).unwrap();
    assert_eq!(owned.title().get(), "Write docs");
    assert_eq!(owned.score(), 42);
    assert_eq!(
        owned.assignee().unwrap().street().unwrap().get(),
        "1 Main St"
    );
}

#[test]
fn last_wins_and_merged_children() {
    let mut first = task_with_owner("user-1");
    first.title_mut().set("a");
    let mut assignee = Address::new();
    assignee.street_mut().set("Oak");
    first.set_assignee(assignee);
    *first.attributes_mut().entry_mut("k") = 1;
    first.email_address_mut().set("a@example.com");

    let mut second = task_with_owner("user-1");
    second.title_mut().set("b");
    let mut assignee = Address::new();
    assignee.city_mut().set("Kyoto");
    second.set_assignee(assignee);
    *second.attributes_mut().entry_mut("k") = 2;
    *second.urgent_mut() = true;

    let mut bytes = first.encode_to_vec();
    bytes.extend(second.encode_to_vec());
    let view = TaskView::parse(&bytes).unwrap();
    assert_eq!(view.title().unwrap(), Some("b"));
    let assignee = view.assignee().unwrap();
    assert_eq!(assignee.street().unwrap(), Some("Oak"));
    assert_eq!(assignee.city().unwrap(), Some("Kyoto"));
    assert_eq!(view.attribute("k").unwrap(), Some(2));
    assert_eq!(view.notification(), Some(NotificationCase::Urgent));
    assert_eq!(view.urgent(), Some(true));
    assert_eq!(view.email_address().unwrap(), None);
}

#[test]
fn labels_stop_before_a_bad_element() {
    let mut task = task_with_owner("user-1");
    task.labels_mut().push().push_str("ok");
    let mut bytes = task.encode_to_vec();
    // field 8, LEN, one non-UTF-8 byte
    bytes.extend_from_slice(&[0x42, 0x01, 0xff]);
    let view = TaskView::parse(&bytes).unwrap();
    let mut labels = view.labels();
    assert_eq!(labels.next().unwrap().unwrap(), "ok");
    assert_eq!(
        labels.next().unwrap().unwrap_err(),
        DecodeError::InvalidUtf8
    );
}

#[test]
fn invalid_utf8_is_present_and_fails_the_getter() {
    let view = TaskView::parse(&[0x0a, 0x01, 0xff]).unwrap();
    assert!(view.has_title());
    assert_eq!(view.title().unwrap_err(), DecodeError::InvalidUtf8);
}

#[test]
fn wrong_wire_type_and_truncated_body_fail_the_scan() {
    assert!(matches!(
        TaskView::parse(&[0x08, 0x01]),
        Err(DecodeError::InvalidTag)
    ));
    assert!(matches!(
        TaskView::parse(&[0x0a, 0x05, b'a']),
        Err(DecodeError::TruncatedMessage)
    ));
}

#[test]
fn unknown_group_is_skipped() {
    // field 23, empty group (tag varints are two bytes), then score = 9
    let view = TaskView::parse(&[0xbb, 0x01, 0xbc, 0x01, 0x10, 0x09]).unwrap();
    assert_eq!(view.score(), 9);
    let owned = view.to_owned(Global).unwrap();
    assert_eq!(owned.score(), 9);
}

#[test]
fn unknown_closed_enum_does_not_replace_a_known_value() {
    let mut task = task_with_owner("user-1");
    *task.priority_mut() = Priority::HIGH;
    let mut bytes = task.encode_to_vec();
    // field 10, varint 9 (not a known Priority)
    bytes.extend_from_slice(&[0x50, 0x09]);
    let view = TaskView::parse(&bytes).unwrap();
    assert_eq!(view.priority(), Some(Priority::HIGH));
    let owned = view.to_owned(Global).unwrap();
    assert_eq!(owned.priority().get(), Priority::HIGH);

    let only_unknown = TaskView::parse(&[0x50, 0x09]).unwrap();
    assert_eq!(only_unknown.priority(), None);
}

#[test]
fn address_and_point_views_parse_alone() {
    let mut address = Address::new();
    address.street_mut().set("Oak");
    *address.postal_code_mut() = 7;
    let bytes = address.encode_to_vec();
    let view = AddressView::parse(&bytes).unwrap();
    assert_eq!(view.street().unwrap(), Some("Oak"));
    assert_eq!(view.postal_code(), Some(7));
    assert_eq!(view.city().unwrap(), None);
    assert_eq!(view.latitude(), None);

    let mut point = Point::new();
    *point.x_mut() = 8;
    let view = PointView::parse(&point.encode_to_vec()).unwrap();
    assert_eq!(view.x(), 8);
    assert_eq!(view.y(), 0);
}

#[test]
fn unpacked_tag_id_appends_after_packed() {
    let mut task = task_with_owner("user-1");
    task.tag_ids_mut().push(1);
    let mut bytes = task.encode_to_vec();
    // field 6, varint 7
    bytes.extend_from_slice(&[0x30, 0x07]);
    let view = TaskView::parse(&bytes).unwrap();
    assert_eq!(view.tag_ids(), &[1, 7]);
}
