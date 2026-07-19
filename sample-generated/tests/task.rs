//! Integration tests for the sample `Task` / `Address` messages.
//!
//! Grouped by what they exercise: encode/decode roundtrips, repeated-field
//! wire/merge behaviour, oneof accessors (no wire), and enum merge /
//! unknown-field handling.

use ::puroro::{
    MapMut, MapRef, Message, OneofView, OneofViewMut, RepeatedStringMut, UnknownPayload,
};
use ::puroro_rt::encode::encode_varint_field;
use ::puroro_sample_generated::task::{Notification, NotificationCase};
use ::puroro_sample_generated::{Address, Priority, Status, Task};

/// Appends `v` as a base-128 varint (test helper; mirrors wire encoding).
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

/// Writes a packed repeated int32 LEN record (tag + length + concatenated varints).
fn encode_packed_int32_field(field_number: u32, values: &[i32], buf: &mut Vec<u8>) {
    let mut payload = Vec::new();
    for &v in values {
        encode_u64_varint(v as u64, &mut payload);
    }
    encode_u64_varint(u64::from(field_number) << 3 | 2, buf); // WireType::Len
    encode_u64_varint(payload.len() as u64, buf);
    buf.extend_from_slice(&payload);
}

// ---------------------------------------------------------------------------
// Encode / decode roundtrips
// ---------------------------------------------------------------------------

#[test]
fn task_clone_and_eq() {
    let mut task = Task::new();
    task.title_mut().push_str("Clone me");
    *task.score_mut() = 7;
    task.owner_id_mut().push_str("u");
    *task.done_mut() = true;
    let mut addr = Address::new();
    addr.street_mut().push_str("St");
    *addr.latitude_mut() = 1.5;
    *task.assignee_mut() = addr;

    let mut cloned = task.clone();
    assert_eq!(task, cloned);
    assert_eq!(format!("{:?}", task), format!("{:?}", cloned));

    *cloned.score_mut() = 8;
    assert_ne!(task, cloned);
}

#[test]
fn address_clone_and_eq() {
    let mut a = Address::new();
    a.city_mut().push_str("Tokyo");
    *a.postal_code_mut() = 100;
    *a.latitude_mut() = 35.0;
    let b = a.clone();
    assert_eq!(a, b);
    a.clear_city();
    assert_ne!(a, b);
}

#[test]
fn task_fields_roundtrip() {
    let mut task = Task::new();
    task.title_mut().push_str("Write docs");
    *task.score_mut() = 42;
    *task.max_retries_mut() = 5;
    task.owner_id_mut().push_str("user-1");
    task.payload_mut().extend_from_slice(b"data");
    task.tag_ids_mut().push(10);
    task.tag_ids_mut().push(20);
    task.scores_mut().push(1);
    task.labels_mut().push().push_str("urgent");
    *task.status_mut() = Status::PENDING;
    *task.priority_mut() = Priority::HIGH;
    *task.done_mut() = true;
    *task.flag_mut() = false;
    task.email_address_mut().push_str("a@example.com");

    let mut assignee = Address::new();
    assignee.street_mut().push_str("1 Main St");
    assignee.city_mut().push_str("Tokyo");
    *assignee.postal_code_mut() = 1000001;
    *assignee.latitude_mut() = 35.6812;
    *task.assignee_mut() = assignee;

    let mut watcher = Address::new();
    watcher.street_mut().push_str("2 Side Rd");
    watcher.city_mut().push_str("Osaka");
    *watcher.postal_code_mut() = 5300001;
    *watcher.latitude_mut() = 34.6937;
    task.watchers_mut().push(watcher);
    task.votes_mut().push(true);
    task.votes_mut().push(false);
    task.attributes_mut().insert_in("region", 81).unwrap();
    task.attributes_mut().insert_in("tier", 2).unwrap();

    task.validate().unwrap();

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();

    assert_eq!(decoded.title().get(), "Write docs");
    assert_eq!(decoded.score(), 42);
    assert_eq!(decoded.max_retries().get(), 5);
    assert!(decoded.owner_id().is_set());
    assert_eq!(decoded.payload().get(), b"data");
    assert_eq!(decoded.tag_ids(), &[10, 20]);
    assert_eq!(decoded.scores(), &[1]);
    assert_eq!(decoded.labels().len(), 1);
    assert_eq!(&*decoded.labels()[0], "urgent");
    assert_eq!(decoded.status().get(), Status::PENDING);
    assert!(decoded.status().is_set());
    assert_eq!(decoded.priority().get(), Priority::HIGH);
    assert!(decoded.priority().is_set());
    assert!(decoded.done());
    assert!(decoded.flag().is_set());
    assert!(!decoded.flag().get());
    assert!(matches!(
        decoded.notification().as_ref(),
        Some(Notification::EmailAddress(s)) if s == "a@example.com"
    ));
    let a = decoded.assignee().unwrap();
    assert_eq!(a.street().get(), "1 Main St");
    assert_eq!(a.city().get(), "Tokyo");
    assert_eq!(a.postal_code().get(), 1000001);
    assert!((a.latitude().get() - 35.6812).abs() < 1e-9);
    assert_eq!(decoded.watchers().len(), 1);
    assert_eq!(decoded.watchers()[0].street().get(), "2 Side Rd");
    assert_eq!(decoded.watchers()[0].city().get(), "Osaka");
    assert_eq!(decoded.votes(), &[true, false]);
    assert_eq!(decoded.attributes().len(), 2);
    assert_eq!(decoded.attributes().get("region").copied(), Some(81));
    assert_eq!(decoded.attributes().get("tier").copied(), Some(2));
}

#[test]
fn map_attributes_last_wins_on_merge() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("u");
    {
        let mut attrs = task.attributes_mut();
        attrs.insert_in("k", 1).unwrap();
        attrs.insert_in("k", 2).unwrap();
        *attrs.get_mut("k").unwrap() = 3;
    }
    assert_eq!(task.attributes().get("k").copied(), Some(3));
    assert_eq!(task.attributes().len(), 1);

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(decoded.attributes().get("k").copied(), Some(3));
}

#[test]
fn oneof_varint_variant_roundtrip() {
    // A oneof that resolves to a VARINT variant (`int32 webhook_id = 14`).
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1"); // LEGACY_REQUIRED
    *task.webhook_id_mut() = 4321;
    task.validate().unwrap();

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();

    assert_eq!(
        decoded.notification().case(),
        Some(NotificationCase::WebhookId)
    );
    assert!(matches!(
        decoded.notification().as_ref(),
        Some(Notification::WebhookId(4321))
    ));
    assert!(decoded.webhook_id().is_set());
    assert_eq!(decoded.webhook_id().get(), 4321);
}

#[test]
fn oneof_message_variant_roundtrip() {
    // A oneof that resolves to a nested-MESSAGE variant (`Address postal = 15`).
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    {
        let postal = task.postal_mut();
        postal.street_mut().push_str("5 Oak Ave");
        postal.city_mut().push_str("Kyoto");
    }
    task.validate().unwrap();

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();

    assert_eq!(
        decoded.notification().case(),
        Some(NotificationCase::Postal)
    );
    let Some(Notification::Postal(addr)) = decoded.notification().as_ref() else {
        panic!("expected postal variant");
    };
    assert_eq!(addr.street().get(), "5 Oak Ave");
    assert_eq!(addr.city().get(), "Kyoto");
}

#[test]
fn oneof_bool_variant_roundtrip() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    *task.urgent_mut() = true;
    task.validate().unwrap();

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();

    assert_eq!(
        decoded.notification().case(),
        Some(NotificationCase::Urgent)
    );
    assert!(matches!(
        decoded.notification().as_ref(),
        Some(Notification::Urgent(true))
    ));
    assert!(decoded.urgent().is_set());
    assert!(decoded.urgent().get());

    // Type-default false is still emitted when the oneof case is selected.
    *task.urgent_mut() = false;
    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(
        decoded.notification().case(),
        Some(NotificationCase::Urgent)
    );
    assert!(matches!(
        decoded.notification().as_ref(),
        Some(Notification::Urgent(false))
    ));
}

#[test]
fn implicit_clear_omits_from_wire() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    *task.score_mut() = 42;
    *task.status_mut() = Status::PENDING;

    task.clear_score();
    task.clear_status();

    assert_eq!(task.score(), 0);
    assert!(!task.status().is_set());

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(decoded.score(), 0);
    assert!(!decoded.status().is_set());
}

#[test]
fn implicit_bool_omits_false_on_wire() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    *task.done_mut() = true;
    task.clear_done();
    assert!(!task.done());

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert!(!decoded.done());
}

#[test]
fn explicit_bool_preserves_false() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    *task.flag_mut() = false;
    assert!(task.flag().is_set());
    assert!(!task.flag().get());

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert!(decoded.flag().is_set());
    assert!(!decoded.flag().get());

    task.clear_flag();
    assert!(!task.flag().is_set());
    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert!(!decoded.flag().is_set());
}

// ---------------------------------------------------------------------------
// Repeated fields (dual-form decode, merge append, clear omit)
// ---------------------------------------------------------------------------

#[test]
fn packed_declared_field_accepts_expanded_wire() {
    // `tag_ids` is PACKED on encode, but must accept expanded (per-element) wire.
    let mut bytes = Vec::new();
    encode_varint_field(6, 10, &mut bytes);
    encode_varint_field(6, 20, &mut bytes);

    let task: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(task.tag_ids(), &[10, 20]);
}

#[test]
fn expanded_declared_field_accepts_packed_wire() {
    // `scores` is EXPANDED on encode, but must accept a packed LEN record.
    let mut bytes = Vec::new();
    encode_packed_int32_field(7, &[1, 2, 3], &mut bytes);

    let task: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(task.scores(), &[1, 2, 3]);
}

#[test]
fn packable_repeated_accepts_mixed_wire_forms() {
    // Spec: multiple occurrences append; packed and expanded may be mixed.
    let mut bytes = Vec::new();
    encode_packed_int32_field(6, &[1, 2], &mut bytes);
    encode_varint_field(6, 3, &mut bytes);
    encode_varint_field(7, 10, &mut bytes);
    encode_packed_int32_field(7, &[20, 30], &mut bytes);
    // votes = 20 (PACKED declared): expanded then packed
    encode_varint_field(20, 1, &mut bytes);
    encode_packed_int32_field(20, &[0, 1], &mut bytes);

    let task: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(task.tag_ids(), &[1, 2, 3]);
    assert_eq!(task.scores(), &[10, 20, 30]);
    assert_eq!(task.votes(), &[true, false, true]);
}

#[test]
fn repeated_merge_appends() {
    let mut task = Task::new();
    task.tag_ids_mut().push(1);
    task.scores_mut().push(10);
    task.labels_mut().push().push_str("a");
    let mut w0 = Address::new();
    w0.city_mut().push_str("A");
    task.watchers_mut().push(w0);
    task.votes_mut().push(true);

    let mut other = Task::new();
    other.tag_ids_mut().push(2);
    other.tag_ids_mut().push(3);
    other.scores_mut().push(20);
    other.labels_mut().push().push_str("b");
    let mut w1 = Address::new();
    w1.city_mut().push_str("B");
    other.watchers_mut().push(w1);
    other.votes_mut().push(false);
    let bytes = other.encode_to_vec();

    task.merge_from(&mut &bytes[..]).unwrap();

    assert_eq!(task.tag_ids(), &[1, 2, 3]);
    assert_eq!(task.scores(), &[10, 20]);
    assert_eq!(task.labels().len(), 2);
    assert_eq!(&*task.labels()[0], "a");
    assert_eq!(&*task.labels()[1], "b");
    assert_eq!(task.watchers().len(), 2);
    assert_eq!(task.watchers()[0].city().get(), "A");
    assert_eq!(task.watchers()[1].city().get(), "B");
    assert_eq!(task.votes(), &[true, false]);
}

#[test]
fn repeated_clear_omits_from_wire() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    task.tag_ids_mut().push(1);
    task.scores_mut().push(2);
    task.labels_mut().push().push_str("x");
    let mut w = Address::new();
    w.street_mut().push_str("gone");
    task.watchers_mut().push(w);
    task.votes_mut().push(true);

    task.clear_tag_ids();
    task.clear_scores();
    task.clear_labels();
    task.clear_watchers();
    task.clear_votes();

    assert!(task.tag_ids().is_empty());
    assert!(task.scores().is_empty());
    assert!(task.labels().is_empty());
    assert!(task.watchers().is_empty());
    assert!(task.votes().is_empty());

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert!(decoded.tag_ids().is_empty());
    assert!(decoded.scores().is_empty());
    assert!(decoded.labels().is_empty());
    assert!(decoded.watchers().is_empty());
    assert!(decoded.votes().is_empty());
}

#[test]
fn repeated_bool_roundtrip() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    task.votes_mut().push(true);
    task.votes_mut().push(false);
    task.votes_mut().push(true);

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(decoded.votes(), &[true, false, true]);
}

#[test]
fn repeated_message_roundtrip() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");

    let mut a = Address::new();
    a.street_mut().push_str("1 Main");
    a.city_mut().push_str("Tokyo");
    let mut b = Address::new();
    b.street_mut().push_str("2 Oak");
    b.city_mut().push_str("Kyoto");
    task.watchers_mut().push(a);
    task.watchers_mut().push(b);

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();

    assert_eq!(decoded.watchers().len(), 2);
    assert_eq!(decoded.watchers()[0].street().get(), "1 Main");
    assert_eq!(decoded.watchers()[0].city().get(), "Tokyo");
    assert_eq!(decoded.watchers()[1].street().get(), "2 Oak");
    assert_eq!(decoded.watchers()[1].city().get(), "Kyoto");
}

// ---------------------------------------------------------------------------
// Oneof accessors / views (no encode/decode)
// ---------------------------------------------------------------------------

#[test]
fn oneof_scalar_getter_uses_custom_default_when_unset() {
    // Official const-getter contract: unset / other variant → custom default,
    // without selecting the variant. `webhook_id = 14 [default = -1]`.
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");

    assert!(task.notification().case().is_none());
    assert!(!task.webhook_id().is_set());
    assert_eq!(task.webhook_id().get(), -1);

    task.email_address_mut().push_str("a@example.com");
    assert_eq!(
        task.notification().case(),
        Some(NotificationCase::EmailAddress)
    );
    assert!(!task.webhook_id().is_set());
    assert_eq!(task.webhook_id().get(), -1);
    // String members without [default] still fall back to the type default.
    assert!(!task.phone_number().is_set());
    assert_eq!(task.phone_number().get(), "");
}

#[test]
fn oneof_switching_frees_previous_variant() {
    // Switching across all three storage kinds must free the previous variant
    // (heap-owning LEN and message variants) without an implicit-drop panic.
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    task.email_address_mut().push_str("a@example.com"); // LEN (heap string)
    task.postal_mut().street_mut().push_str("St"); // -> message (frees string)
    *task.webhook_id_mut() = 3; // -> scalar (frees message)

    assert!(matches!(
        task.notification().as_ref(),
        Some(Notification::WebhookId(3))
    ));
}

#[test]
fn oneof_group_bound_views_when_unset() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");

    // Bound views are always available, including when the group is unset.
    {
        let view = task.notification();
        assert!(view.case().is_none());
        assert!(view.as_ref().is_none());
    }

    let view_mut = task.notification_mut();
    assert!(view_mut.as_view().case().is_none());
    assert!(view_mut.as_view().as_ref().is_none());
    assert!(view_mut.as_mut().is_none());
}

#[test]
fn oneof_group_view_mut_as_view_and_clear() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    task.email_address_mut().push_str("a@example.com");

    {
        let view_mut = task.notification_mut();
        // Shared getters work while holding the mut bound view.
        assert_eq!(
            view_mut.as_view().case(),
            Some(NotificationCase::EmailAddress)
        );
        assert!(view_mut.as_view().as_ref().is_some());
        assert!(matches!(
            view_mut.as_mut(),
            Some(Notification::EmailAddress(_))
        ));
    }
    assert!(matches!(
        task.notification().as_ref(),
        Some(Notification::EmailAddress(s)) if s == "a@example.com"
    ));

    task.notification_mut().clear();
    assert!(task.notification().case().is_none());
    assert!(task.notification().as_ref().is_none());
}

// ---------------------------------------------------------------------------
// Enum merge / unknown fields (decode into existing message)
// ---------------------------------------------------------------------------

#[test]
fn closed_enum_unknown_goes_to_unknown_fields() {
    // Spec: https://protobuf.dev/programming-guides/enum/
    // Closed: unrecognized value → unknown field set; field unset; default returned.
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");

    // field 10 (priority) = 99 (unknown closed enum value), wire: tag + varint
    let mut bytes = Vec::new();
    encode_varint_field(10, 99, &mut bytes);

    task.merge_from(&mut &bytes[..]).unwrap();

    assert!(!task.priority().is_set());
    assert_eq!(task.priority().get(), Priority::UNSPECIFIED);
    let unknowns: Vec<_> = task.unknown_fields().collect();
    assert_eq!(unknowns.len(), 1);
    assert_eq!(unknowns[0].number(), 10);
    assert_eq!(*unknowns[0].payload(), UnknownPayload::Varint(99));
}

#[test]
fn open_enum_unknown_stays_in_field() {
    // Spec: https://protobuf.dev/programming-guides/enum/
    // Open: unrecognized value → stored in the field; accessors report set.
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");

    // field 9 (status) = 99 (unknown open enum value)
    let mut bytes = Vec::new();
    encode_varint_field(9, 99, &mut bytes);

    task.merge_from(&mut &bytes[..]).unwrap();

    assert!(task.status().is_set());
    assert_eq!(task.status().get(), Status::from(99));
    assert!(task.unknown_fields().next().is_none());
}
