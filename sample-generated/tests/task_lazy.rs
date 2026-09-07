//! `TaskLazy` ingest: scanner, catalog merge, WireOrSso, nested lazy children.

use ::puroro::{BytesMut, DecodeError, MapMut, MapRef, Message, RepeatedStringMut, StringMut};
use ::puroro_rt::INLINE_CAP;
use ::puroro_rt::Varint;
use ::puroro_rt::encode::{encode_varint_field, field_number_const};
use ::puroro_sample_generated::task::{
    FIELD_ASSIGNEE, FIELD_ATTRIBUTES, FIELD_LABELS, FIELD_ORIGIN, FIELD_OWNER_ID, FIELD_PAYLOAD,
    FIELD_SCORE, FIELD_SCORES, FIELD_TAG_IDS, FIELD_TITLE, FIELD_WATCHERS,
};
use ::puroro_sample_generated::{Address, Point, Priority, Status, Task, TaskLazy};

fn encode_u64_varint(mut v: u64, buf: &mut Vec<u8>) {
    loop {
        let mut byte = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if v == 0 {
            break;
        }
    }
}

fn encode_packed_int32_field(field_number: u32, values: &[i32], buf: &mut Vec<u8>) {
    let mut payload = Vec::new();
    for &v in values {
        encode_u64_varint(v as u64, &mut payload);
    }
    encode_u64_varint(u64::from(field_number) << 3 | 2, buf);
    encode_u64_varint(payload.len() as u64, buf);
    buf.extend_from_slice(&payload);
}

fn encode_len_bytes(field_number: u32, payload: &[u8], buf: &mut Vec<u8>) {
    encode_u64_varint(u64::from(field_number) << 3 | 2, buf);
    encode_u64_varint(payload.len() as u64, buf);
    buf.extend_from_slice(payload);
}

#[test]
fn empty_message_is_implicit_zero() {
    let lazy = TaskLazy::new();
    assert_eq!(lazy.score().unwrap(), 0);
    assert!(!lazy.max_retries().unwrap().is_set());
    assert_eq!(lazy.max_retries().unwrap().get(), 3);
    assert!(lazy.tag_ids().unwrap().is_empty());
    assert!(lazy.scores().unwrap().is_empty());
    assert!(!lazy.status().unwrap().is_set());
    assert!(!lazy.priority().unwrap().is_set());
    assert!(!lazy.done().unwrap());
    assert!(!lazy.flag().unwrap().is_set());
    assert!(lazy.votes().unwrap().is_empty());
    assert!(!lazy.has_title().unwrap());
    assert!(!lazy.title().unwrap().is_set());
    assert_eq!(lazy.title().unwrap().get(), "");
    assert!(!lazy.has_owner_id().unwrap());
    assert!(!lazy.has_payload().unwrap());
    assert_eq!(lazy.payload().unwrap().get(), b"" as &[u8]);
    assert!(lazy.assignee().unwrap().is_none());
    assert!(lazy.origin().unwrap().is_none());
    assert!(lazy.watchers().unwrap().is_empty());
    assert!(lazy.labels().unwrap().is_empty());
    assert!(lazy.attributes().unwrap().is_empty());
}

#[test]
fn last_wins_scalar_in_one_message() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(1),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(9),
        &mut bytes,
    );

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.score().unwrap(), 9);
}

#[test]
fn second_merge_from_is_protobuf_merge() {
    let mut first = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(1),
        &mut first,
    );
    encode_varint_field(
        field_number_const::<FIELD_TAG_IDS>(),
        Varint::from_int32(10),
        &mut first,
    );

    let mut second = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(2),
        &mut second,
    );
    encode_varint_field(
        field_number_const::<FIELD_TAG_IDS>(),
        Varint::from_int32(20),
        &mut second,
    );

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut first.as_slice()).unwrap();
    lazy.merge_from(&mut second.as_slice()).unwrap();
    assert_eq!(lazy.score().unwrap(), 2);
    assert_eq!(lazy.tag_ids().unwrap(), &[10, 20]);
}

#[test]
fn packed_and_expanded_mixed() {
    let mut bytes = Vec::new();
    encode_packed_int32_field(FIELD_TAG_IDS, &[1, 2], &mut bytes);
    encode_varint_field(
        field_number_const::<FIELD_TAG_IDS>(),
        Varint::from_int32(3),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<FIELD_SCORES>(),
        Varint::from_int32(10),
        &mut bytes,
    );
    encode_packed_int32_field(FIELD_SCORES, &[20, 30], &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.tag_ids().unwrap(), &[1, 2, 3]);
    assert_eq!(lazy.scores().unwrap(), &[10, 20, 30]);
}

#[test]
fn truncated_input_errors_on_finish() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(150),
        &mut bytes,
    );
    assert!(bytes.len() >= 2);

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes[..bytes.len() - 1]).unwrap();
    assert_eq!(lazy.finish(), Err(DecodeError::TruncatedMessage));
    assert_eq!(lazy.score(), Err(DecodeError::TruncatedMessage));
}

#[test]
fn getter_before_finish_is_unfinished() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(4),
        &mut bytes,
    );

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes).unwrap();
    assert_eq!(lazy.score(), Err(DecodeError::UnfinishedMessage));
    lazy.finish().unwrap();
    assert_eq!(lazy.score().unwrap(), 4);
}

#[test]
fn incremental_chunks_join_one_record() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(150),
        &mut bytes,
    );
    assert!(bytes.len() >= 3);

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes[..1]).unwrap();
    lazy.push(&bytes[1..2]).unwrap();
    lazy.push(&bytes[2..]).unwrap();
    lazy.finish().unwrap();
    assert_eq!(lazy.score().unwrap(), 150);
}

#[test]
fn skips_len_records() {
    let mut bytes = Vec::new();
    // Field 99 is not a known `Task` field — still skipped (oneof is too).
    encode_len_bytes(99, b"ignored", &mut bytes);
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(7),
        &mut bytes,
    );

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.score().unwrap(), 7);
}

#[test]
fn enums_and_bools() {
    let mut bytes = Vec::new();
    encode_varint_field(field_number_const::<9>(), Varint::from_int32(1), &mut bytes);
    encode_varint_field(
        field_number_const::<10>(),
        Varint::from_int32(2),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<16>(),
        Varint::from_bool(true),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<17>(),
        Varint::from_bool(false),
        &mut bytes,
    );
    encode_packed_int32_field(20, &[1, 0, 1], &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.status().unwrap().get(), Status::PENDING);
    assert_eq!(lazy.priority().unwrap().get(), Priority::HIGH);
    assert!(lazy.done().unwrap());
    assert!(lazy.flag().unwrap().is_set());
    assert!(!lazy.flag().unwrap().get());
    assert_eq!(lazy.votes().unwrap(), &[true, false, true]);
}

#[test]
fn last_wins_string() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_TITLE, b"first", &mut bytes);
    encode_len_bytes(FIELD_TITLE, b"second", &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert!(lazy.has_title().unwrap());
    assert_eq!(lazy.title().unwrap().get(), "second");
}

#[test]
fn invalid_utf8_before_optional() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_TITLE, &[0xff, 0xfe], &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert!(lazy.has_title().unwrap());
    assert_eq!(lazy.title().err(), Some(DecodeError::InvalidUtf8));
    // Failed is sticky: a second get does not re-read `_wire`.
    assert_eq!(lazy.title().err(), Some(DecodeError::InvalidUtf8));
}

#[test]
fn missing_string_and_bytes() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(1),
        &mut bytes,
    );

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert!(!lazy.has_title().unwrap());
    assert!(!lazy.title().unwrap().is_set());
    assert!(!lazy.has_owner_id().unwrap());
    assert!(!lazy.has_payload().unwrap());
}

#[test]
fn empty_string_is_present() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_TITLE, b"", &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert!(lazy.has_title().unwrap());
    assert!(lazy.title().unwrap().is_set());
    assert_eq!(lazy.title().unwrap().get(), "");
}

#[test]
fn second_merge_from_last_wins_string() {
    let mut first = Vec::new();
    encode_len_bytes(FIELD_TITLE, b"alpha", &mut first);
    let mut second = Vec::new();
    encode_len_bytes(FIELD_TITLE, b"beta", &mut second);

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut first.as_slice()).unwrap();
    lazy.merge_from(&mut second.as_slice()).unwrap();
    assert_eq!(lazy.title().unwrap().get(), "beta");
}

#[test]
fn incremental_string_chunks() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_TITLE, b"hello", &mut bytes);
    assert!(bytes.len() >= 3);

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes[..1]).unwrap();
    lazy.push(&bytes[1..3]).unwrap();
    lazy.push(&bytes[3..]).unwrap();
    lazy.finish().unwrap();
    assert_eq!(lazy.title().unwrap().get(), "hello");
}

#[test]
fn owner_id_and_payload() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_OWNER_ID, b"user-1", &mut bytes);
    encode_len_bytes(FIELD_PAYLOAD, b"\x00\x01\xff", &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.owner_id().unwrap().get(), "user-1");
    assert_eq!(lazy.payload().unwrap().get(), &[0, 1, 0xff]);
}

#[test]
fn title_rejects_non_len_wire() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_TITLE>(),
        Varint::from_int32(1),
        &mut bytes,
    );

    assert_eq!(
        TaskLazy::decode(&bytes[..]).err(),
        Some(DecodeError::InvalidTag)
    );
}

#[test]
fn second_get_does_not_revalidate() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_TITLE, b"hello", &mut bytes);
    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.title().unwrap().get(), "hello");
    assert_eq!(lazy.title().unwrap().get(), "hello");
}

#[test]
fn heap_string_promotes_and_drops() {
    let long = "z".repeat(INLINE_CAP + 8);
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_TITLE, long.as_bytes(), &mut bytes);
    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.title().unwrap().get(), long);
    drop(lazy);
}

#[test]
fn failed_then_merge_from_retries() {
    let mut bad = Vec::new();
    encode_len_bytes(FIELD_TITLE, &[0xff], &mut bad);
    let mut good = Vec::new();
    encode_len_bytes(FIELD_TITLE, b"ok", &mut good);

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut bad.as_slice()).unwrap();
    assert_eq!(lazy.title().err(), Some(DecodeError::InvalidUtf8));
    lazy.merge_from(&mut good.as_slice()).unwrap();
    assert_eq!(lazy.title().unwrap().get(), "ok");
}

fn encode_message_field(field_number: u32, payload: &[u8], buf: &mut Vec<u8>) {
    encode_len_bytes(field_number, payload, buf);
}

#[test]
fn assignee_occurrences_merge_during_scan() {
    let mut first = Address::new();
    first.street_mut().set("Oak");
    let mut second = Address::new();
    second.city_mut().set("Kyoto");
    *second.postal_code_mut() = 600;
    *second.latitude_mut() = 35.0;

    let mut bytes = Vec::new();
    encode_message_field(FIELD_ASSIGNEE, &first.encode_to_vec(), &mut bytes);
    encode_message_field(FIELD_ASSIGNEE, &second.encode_to_vec(), &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let addr = lazy.assignee().unwrap().expect("assignee present");
    assert_eq!(addr.street().unwrap().get(), "Oak");
    assert_eq!(addr.city().unwrap().get(), "Kyoto");
    assert_eq!(addr.postal_code().unwrap().get(), 600);
    assert_eq!(addr.latitude().unwrap().get(), 35.0);
}

#[test]
fn assignee_street_last_wins_across_occurrences() {
    let mut first = Address::new();
    first.street_mut().set("old");
    let mut second = Address::new();
    second.street_mut().set("new");

    let mut bytes = Vec::new();
    encode_message_field(FIELD_ASSIGNEE, &first.encode_to_vec(), &mut bytes);
    encode_message_field(FIELD_ASSIGNEE, &second.encode_to_vec(), &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(
        lazy.assignee().unwrap().unwrap().street().unwrap().get(),
        "new"
    );
}

#[test]
fn watchers_append_one_child_per_occurrence() {
    let mut a = Address::new();
    a.street_mut().set("one");
    let mut b = Address::new();
    b.street_mut().set("two");

    let mut bytes = Vec::new();
    encode_message_field(FIELD_WATCHERS, &a.encode_to_vec(), &mut bytes);
    encode_message_field(FIELD_WATCHERS, &b.encode_to_vec(), &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let watchers = lazy.watchers().unwrap();
    assert_eq!(watchers.len(), 2);
    assert_eq!(watchers[0].street().unwrap().get(), "one");
    assert_eq!(watchers[1].street().unwrap().get(), "two");
}

#[test]
fn origin_point_numericals() {
    let mut point = Point::new();
    *point.x_mut() = 3;
    *point.y_mut() = 4;

    let mut bytes = Vec::new();
    encode_message_field(FIELD_ORIGIN, &point.encode_to_vec(), &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let origin = lazy.origin().unwrap().expect("origin present");
    assert_eq!(origin.x().unwrap(), 3);
    assert_eq!(origin.y().unwrap(), 4);
}

#[test]
fn incremental_nested_len_chunks() {
    let mut addr = Address::new();
    addr.street_mut().set("split");
    let mut bytes = Vec::new();
    encode_message_field(FIELD_ASSIGNEE, &addr.encode_to_vec(), &mut bytes);
    assert!(bytes.len() >= 3);

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes[..1]).unwrap();
    lazy.push(&bytes[1..3]).unwrap();
    lazy.push(&bytes[3..]).unwrap();
    lazy.finish().unwrap();
    assert_eq!(
        lazy.assignee().unwrap().unwrap().street().unwrap().get(),
        "split"
    );
}

#[test]
fn second_parent_merge_from_merges_child() {
    let mut first = Address::new();
    first.street_mut().set("A");
    let mut second = Address::new();
    second.city_mut().set("B");

    let mut wire1 = Vec::new();
    encode_message_field(FIELD_ASSIGNEE, &first.encode_to_vec(), &mut wire1);
    let mut wire2 = Vec::new();
    encode_message_field(FIELD_ASSIGNEE, &second.encode_to_vec(), &mut wire2);

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut wire1.as_slice()).unwrap();
    lazy.merge_from(&mut wire2.as_slice()).unwrap();
    let addr = lazy.assignee().unwrap().unwrap();
    assert_eq!(addr.street().unwrap().get(), "A");
    assert_eq!(addr.city().unwrap().get(), "B");
}

fn encode_map_string_i32(key: &str, value: i32, buf: &mut Vec<u8>) {
    let mut entry = Vec::new();
    encode_len_bytes(1, key.as_bytes(), &mut entry);
    encode_varint_field(
        field_number_const::<2>(),
        Varint::from_int32(value),
        &mut entry,
    );
    encode_len_bytes(FIELD_ATTRIBUTES, &entry, buf);
}

#[test]
fn attributes_empty_map() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_SCORE>(),
        Varint::from_int32(1),
        &mut bytes,
    );

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert!(lazy.attributes().unwrap().is_empty());
    assert_eq!(lazy.attributes().unwrap().get("k"), None);
}

#[test]
fn attributes_last_wins_per_key() {
    let mut bytes = Vec::new();
    encode_map_string_i32("k", 1, &mut bytes);
    encode_map_string_i32("k", 2, &mut bytes);
    encode_map_string_i32("other", 9, &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let attrs = lazy.attributes().unwrap();
    assert_eq!(attrs.len(), 2);
    assert_eq!(attrs.get("k").copied(), Some(2));
    assert_eq!(attrs.get("other").copied(), Some(9));
}

#[test]
fn attributes_missing_key_and_value_defaults() {
    let mut value_only = Vec::new();
    encode_varint_field(
        field_number_const::<2>(),
        Varint::from_int32(7),
        &mut value_only,
    );
    let mut key_only = Vec::new();
    encode_len_bytes(1, b"solo", &mut key_only);

    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_ATTRIBUTES, &value_only, &mut bytes);
    encode_len_bytes(FIELD_ATTRIBUTES, &key_only, &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let attrs = lazy.attributes().unwrap();
    assert_eq!(attrs.len(), 2);
    assert_eq!(attrs.get("").copied(), Some(7));
    assert_eq!(attrs.get("solo").copied(), Some(0));
}

#[test]
fn attributes_second_get_uses_materialised_map() {
    let mut bytes = Vec::new();
    encode_map_string_i32("region", 81, &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.attributes().unwrap().get("region").copied(), Some(81));
    assert_eq!(lazy.attributes().unwrap().get("region").copied(), Some(81));
}

#[test]
fn attributes_merge_from_after_first_get() {
    let mut first = Vec::new();
    encode_map_string_i32("a", 1, &mut first);
    let mut second = Vec::new();
    encode_map_string_i32("a", 2, &mut second);
    encode_map_string_i32("b", 3, &mut second);

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut first.as_slice()).unwrap();
    assert_eq!(lazy.attributes().unwrap().get("a").copied(), Some(1));
    lazy.merge_from(&mut second.as_slice()).unwrap();
    let attrs = lazy.attributes().unwrap();
    assert_eq!(attrs.len(), 2);
    assert_eq!(attrs.get("a").copied(), Some(2));
    assert_eq!(attrs.get("b").copied(), Some(3));
}

#[test]
fn incremental_map_entry_chunks() {
    let mut bytes = Vec::new();
    encode_map_string_i32("split", 5, &mut bytes);
    assert!(bytes.len() >= 3);

    let mut lazy = TaskLazy::new();
    lazy.push(&bytes[..1]).unwrap();
    lazy.push(&bytes[1..3]).unwrap();
    lazy.push(&bytes[3..]).unwrap();
    lazy.finish().unwrap();
    assert_eq!(lazy.attributes().unwrap().get("split").copied(), Some(5));
}

#[test]
fn attributes_rejects_non_len_wire() {
    let mut bytes = Vec::new();
    encode_varint_field(
        field_number_const::<FIELD_ATTRIBUTES>(),
        Varint::from_int32(1),
        &mut bytes,
    );

    assert_eq!(
        TaskLazy::decode(&bytes[..]).err(),
        Some(DecodeError::InvalidTag)
    );
}

#[test]
fn labels_append_and_empty_string() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_LABELS, b"urgent", &mut bytes);
    encode_len_bytes(FIELD_LABELS, b"", &mut bytes);
    encode_len_bytes(FIELD_LABELS, b"docs", &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let labels = lazy.labels().unwrap();
    assert_eq!(labels.len(), 3);
    assert_eq!(&*labels[0], "urgent");
    assert_eq!(&*labels[1], "");
    assert_eq!(&*labels[2], "docs");
    assert_eq!(&*lazy.labels().unwrap()[0], "urgent");
}

#[test]
fn labels_second_merge_from_appends() {
    let mut first = Vec::new();
    encode_len_bytes(FIELD_LABELS, b"a", &mut first);
    let mut second = Vec::new();
    encode_len_bytes(FIELD_LABELS, b"b", &mut second);

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut first.as_slice()).unwrap();
    lazy.merge_from(&mut second.as_slice()).unwrap();
    let labels = lazy.labels().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(&*labels[0], "a");
    assert_eq!(&*labels[1], "b");
}

#[test]
fn labels_merge_from_after_first_get() {
    let mut first = Vec::new();
    encode_len_bytes(FIELD_LABELS, b"a", &mut first);
    let mut second = Vec::new();
    encode_len_bytes(FIELD_LABELS, b"b", &mut second);

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut first.as_slice()).unwrap();
    assert_eq!(lazy.labels().unwrap().len(), 1);
    lazy.merge_from(&mut second.as_slice()).unwrap();
    let labels = lazy.labels().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(&*labels[0], "a");
    assert_eq!(&*labels[1], "b");
}

#[test]
fn labels_invalid_utf8() {
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_LABELS, b"ok", &mut bytes);
    encode_len_bytes(FIELD_LABELS, &[0xff, 0xfe], &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.labels().err(), Some(DecodeError::InvalidUtf8));
    assert_eq!(lazy.labels().err(), Some(DecodeError::InvalidUtf8));
}

#[test]
fn attributes_invalid_utf8_key() {
    let mut entry = Vec::new();
    encode_len_bytes(1, &[0xff, 0xfe], &mut entry);
    encode_varint_field(field_number_const::<2>(), Varint::from_int32(1), &mut entry);
    let mut bytes = Vec::new();
    encode_len_bytes(FIELD_ATTRIBUTES, &entry, &mut bytes);

    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.attributes().err(), Some(DecodeError::InvalidUtf8));
}

fn sample_task() -> Task {
    let mut task = Task::new();
    task.title_mut().set("Write docs");
    *task.score_mut() = 42;
    *task.max_retries_mut() = 5;
    task.owner_id_mut().set("user-1");
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
    *assignee.postal_code_mut() = 1000001;
    *assignee.latitude_mut() = 35.6812;
    task.set_assignee(assignee);

    let mut origin = Point::new();
    *origin.x_mut() = 3;
    *origin.y_mut() = 4;
    task.set_origin(origin);

    let mut watcher = Address::new();
    watcher.street_mut().set("2 Side Rd");
    task.watchers_mut().push(watcher);
    task.votes_mut().push(true);
    *task.attributes_mut().entry_mut("region") = 81;
    task
}

#[test]
fn into_eager_matches_eager_decode() {
    let task = sample_task();
    let bytes = task.encode_to_vec();
    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    assert_eq!(lazy.into_eager().unwrap(), task);
}

#[test]
fn encode_as_wire_matches_single_buffer() {
    let task = sample_task();
    let bytes = task.encode_to_vec();
    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let _ = lazy.title().unwrap();
    assert_eq!(lazy.encoded_len().unwrap(), bytes.len());
    assert_eq!(lazy.encode_to_vec().unwrap(), bytes);
}

#[test]
fn empty_into_eager_and_encode() {
    let lazy = TaskLazy::new();
    assert_eq!(lazy.encoded_len().unwrap(), 0);
    assert!(lazy.encode_to_vec().unwrap().is_empty());
    assert_eq!(lazy.into_eager().unwrap(), Task::new());
}

#[test]
fn into_eager_before_finish_errors() {
    let mut lazy = TaskLazy::new();
    lazy.push(&[0x08]).unwrap();
    assert_eq!(lazy.into_eager().err(), Some(DecodeError::TruncatedMessage));
}

#[test]
fn two_merge_from_into_eager_matches_eager_merge() {
    let mut first = Task::new();
    first.owner_id_mut().set("u");
    *first.score_mut() = 1;
    first.tag_ids_mut().push(1);
    let mut second = Task::new();
    second.owner_id_mut().set("u");
    *second.score_mut() = 2;
    second.tag_ids_mut().push(2);

    let w1 = first.encode_to_vec();
    let w2 = second.encode_to_vec();

    let mut expected = Task::new();
    expected.merge_from(&mut w1.as_slice()).unwrap();
    expected.merge_from(&mut w2.as_slice()).unwrap();

    let mut lazy = TaskLazy::new();
    lazy.merge_from(&mut w1.as_slice()).unwrap();
    lazy.merge_from(&mut w2.as_slice()).unwrap();
    assert_eq!(lazy.encoded_len().unwrap(), w1.len() + w2.len());
    assert_eq!(lazy.into_eager().unwrap(), expected);
}

#[test]
fn address_and_point_into_eager() {
    let mut addr = Address::new();
    addr.street_mut().set("Oak");
    *addr.postal_code_mut() = 100;
    let addr_bytes = addr.encode_to_vec();
    let addr_lazy = ::puroro_sample_generated::AddressLazy::decode(&addr_bytes[..]).unwrap();
    assert_eq!(addr_lazy.encode_to_vec().unwrap(), addr_bytes);
    assert_eq!(addr_lazy.into_eager().unwrap(), addr);

    let mut point = Point::new();
    *point.x_mut() = 1;
    *point.y_mut() = 2;
    let point_bytes = point.encode_to_vec();
    let point_lazy = ::puroro_sample_generated::PointLazy::decode(&point_bytes[..]).unwrap();
    assert_eq!(point_lazy.encode_to_vec().unwrap(), point_bytes);
    assert_eq!(point_lazy.into_eager().unwrap(), point);
}

#[test]
fn nested_child_encode_is_body_only() {
    let mut addr = Address::new();
    addr.street_mut().set("Oak");
    let mut task = Task::new();
    task.owner_id_mut().set("u");
    *task.score_mut() = 9;
    task.set_assignee(addr.clone());
    let bytes = task.encode_to_vec();
    let lazy = TaskLazy::decode(&bytes[..]).unwrap();
    let child = lazy.assignee().unwrap().unwrap();
    assert_eq!(child.encode_to_vec().unwrap(), addr.encode_to_vec());
    assert_ne!(child.encode_to_vec().unwrap(), bytes);
}
