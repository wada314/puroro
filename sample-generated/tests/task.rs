//! Integration tests for the sample `Task` / `Address` messages.
//!
//! Grouped by what they exercise: encode/decode roundtrips, repeated-field
//! wire/merge behaviour, oneof accessors (no wire), and enum merge /
//! unknown-field handling.

use ::allocator_api2::alloc::Global;
use ::puroro::{
    BytesMut, MapMut, MapRef, Message, OneofView, OneofViewMut, RepeatedStringMut,
    String as AllocString, StringMut, UnknownPayload,
};
use ::puroro_rt::INLINE_CAP;
use ::puroro_rt::Varint;
use ::puroro_rt::encode::{encode_varint_field, field_number_const};
use ::puroro_sample_generated::task::{Notification, NotificationCase};
use ::puroro_sample_generated::{Address, Point, Priority, Status, Task};

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
    task.title_mut().set("Clone me");
    *task.score_mut() = 7;
    task.owner_id_mut().set("u");
    *task.done_mut() = true;
    let mut addr = Address::new();
    addr.street_mut().set("St");
    *addr.latitude_mut() = 1.5;
    task.set_assignee(addr);

    let mut cloned = task.clone();
    assert_eq!(task, cloned);
    assert_eq!(format!("{:?}", task), format!("{:?}", cloned));

    *cloned.score_mut() = 8;
    assert_ne!(task, cloned);
}

#[test]
fn address_clone_and_eq() {
    let mut a = Address::new();
    a.city_mut().set("Tokyo");
    *a.postal_code_mut() = 100;
    *a.latitude_mut() = 35.0;
    let b = a.clone();
    assert_eq!(a, b);
    a.clear_city();
    assert_ne!(a, b);
}

#[test]
fn string_set_and_set_string() {
    let mut addr = Address::new();
    addr.street_mut().set("short");
    assert_eq!(addr.street().get(), "short");

    let long = "x".repeat(INLINE_CAP + 8);
    addr.street_mut()
        .set_string(AllocString::from_str_in(&long, Global));
    assert_eq!(addr.street().get(), long);

    // Moving a short `AllocString` still works (may demote to inline).
    addr.city_mut()
        .set_string(AllocString::from_str_in("Osaka", Global));
    assert_eq!(addr.city().get(), "Osaka");
}

#[test]
fn task_eq_with_reference_allocator_and_oneof_message() {
    // `A = &Global` is a non-'static reference allocator (same shape as `&Bump`).
    // Oneof `PartialEq` must not require `for<'a> Ref<'a>: PartialEq` / `A: 'static`.
    let alloc = &Global;
    let mut a = Task::new_in(alloc);
    a.owner_id_mut().set("user-1");
    a.postal_mut().street_mut().set("1 Ref St");

    let mut b = Task::new_in(alloc);
    b.owner_id_mut().set("user-1");
    b.postal_mut().street_mut().set("1 Ref St");
    assert_eq!(a, b);

    b.postal_mut().street_mut().set("x");
    assert_ne!(a, b);
}

#[test]
fn task_fields_roundtrip() {
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

    let mut watcher = Address::new();
    watcher.street_mut().set("2 Side Rd");
    watcher.city_mut().set("Osaka");
    *watcher.postal_code_mut() = 5300001;
    *watcher.latitude_mut() = 34.6937;
    task.watchers_mut().push(watcher);
    task.votes_mut().push(true);
    task.votes_mut().push(false);
    *task.attributes_mut().entry_mut("region") = 81;
    *task.attributes_mut().entry_mut("tier") = 2;

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
    assert_eq!(
        decoded.watchers().first().unwrap().street().get(),
        "2 Side Rd"
    );
    assert_eq!(decoded.watchers().first().unwrap().city().get(), "Osaka");
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
        *attrs.entry_mut("k") = 1;
        *attrs.entry_mut("k") = 2;
        *attrs.get_mut("k").unwrap() = 3;
    }
    assert_eq!(task.attributes().get("k").copied(), Some(3));
    assert_eq!(task.attributes().len(), 1);

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(decoded.attributes().get("k").copied(), Some(3));
}

#[test]
fn map_message_values_are_owned_messages() {
    use ::bitvec::array::BitArray;
    use ::bitvec::order::Lsb0;
    use ::puroro_rt::{FieldDeallocate, MapField, MessageCommon, ProtoMessage, ProtoString};

    let mut common = MessageCommon::<BitArray<[u8; 1], Lsb0>, _>::new_in(BitArray::ZERO, Global);
    let mut field = MapField::<ProtoString, ProtoMessage<Address>, 1, _>::new_in(Global);
    {
        let mut map = field.bind_mut(&mut common);
        map.entry_mut("home").street_mut().set("1 Main");
    }
    let homes = field.bind(&common);
    assert_eq!(homes.len(), 1);
    assert_eq!(homes.get("home").unwrap().street().get(), "1 Main");
    field.deallocate(&common);
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
    encode_varint_field(
        field_number_const::<6>(),
        Varint::from_uint64(10),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<6>(),
        Varint::from_uint64(20),
        &mut bytes,
    );

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
    encode_varint_field(
        field_number_const::<6>(),
        Varint::from_uint64(3),
        &mut bytes,
    );
    encode_varint_field(
        field_number_const::<7>(),
        Varint::from_uint64(10),
        &mut bytes,
    );
    encode_packed_int32_field(7, &[20, 30], &mut bytes);
    // votes = 20 (PACKED declared): expanded then packed
    encode_varint_field(
        field_number_const::<20>(),
        Varint::from_uint64(1),
        &mut bytes,
    );
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
    assert_eq!(task.watchers().first().unwrap().city().get(), "A");
    assert_eq!(task.watchers().get(1).unwrap().city().get(), "B");
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
    assert_eq!(decoded.watchers().first().unwrap().street().get(), "1 Main");
    assert_eq!(decoded.watchers().first().unwrap().city().get(), "Tokyo");
    assert_eq!(decoded.watchers().get(1).unwrap().street().get(), "2 Oak");
    assert_eq!(decoded.watchers().get(1).unwrap().city().get(), "Kyoto");
}

// ---------------------------------------------------------------------------
// Oneof accessors / views (no encode/decode)
// ---------------------------------------------------------------------------

#[test]
fn max_retries_uses_custom_default_when_unset() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    assert!(!task.max_retries().is_set());
    assert_eq!(task.max_retries().get(), 3);
    *task.max_retries_mut() = 0;
    assert!(task.max_retries().is_set());
    assert_eq!(task.max_retries().get(), 0);
    task.clear_max_retries();
    assert!(!task.max_retries().is_set());
    assert_eq!(task.max_retries().get(), 3);
}

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
fn oneof_inlined_postal_heap_string_switch_and_clone() {
    // AlwaysInitialized oneof + leftover HEAP_BIT would SIGSEGV on the next
    // empty-inline body without after_deallocate wiping the Address bit range.
    let heap_street = "S".repeat(INLINE_CAP + 8);
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    task.postal_mut().street_mut().set(&heap_street);
    assert_eq!(task.postal().unwrap().street().get(), heap_street);

    let cloned = task.clone();
    assert_eq!(cloned.postal().unwrap().street().get(), heap_street);

    task.email_address_mut().set("a@example.com");
    assert!(task.postal().is_none());
    assert_eq!(task.email_address().get(), "a@example.com");

    {
        let postal = task.postal_mut();
        assert!(!postal.street().is_set());
        postal.street_mut().set(&heap_street);
    }
    assert_eq!(task.postal().unwrap().street().get(), heap_street);

    *task.urgent_mut() = true;
    assert!(task.postal().is_none());
    assert!(task.urgent().get());
    *task.webhook_id_mut() = 1;
    assert!(!task.urgent().is_set());
    assert!(!*task.urgent_mut());
}

#[test]
fn oneof_heap_email_switch_clears_sso_bit() {
    let heap_email = "a".repeat(INLINE_CAP + 8);
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    task.email_address_mut().set(&heap_email);
    *task.webhook_id_mut() = 7;
    assert_eq!(task.email_address().get(), "");
    task.email_address_mut().set(&heap_email);
    assert_eq!(task.email_address().get(), heap_email);
}

#[test]
fn oneof_inlined_postal_unknown_cleared_on_switch() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");

    let mut postal_payload = Vec::new();
    let street = b"Oak";
    encode_u64_varint((1u64) << 3 | 2, &mut postal_payload);
    encode_u64_varint(street.len() as u64, &mut postal_payload);
    postal_payload.extend_from_slice(street);
    encode_varint_field(
        field_number_const::<99>(),
        Varint::from_uint64(5),
        &mut postal_payload,
    );

    let mut bytes = Vec::new();
    encode_u64_varint((15u64) << 3 | 2, &mut bytes);
    encode_u64_varint(postal_payload.len() as u64, &mut bytes);
    bytes.extend_from_slice(&postal_payload);

    task.merge_from(&mut &bytes[..]).unwrap();
    assert!(task.unknown_fields().next().is_none());
    assert_eq!(task.postal().unwrap().street().get(), "Oak");
    let encoded = task.encode_to_vec();
    let postal_len = find_len_field(&encoded, 15).expect("postal LEN on wire");
    assert!(
        contains_varint_field(postal_len, 99, 5),
        "unknown tag 99 must round-trip inside postal LEN, got {postal_len:?}"
    );

    task.email_address_mut().set("a@example.com");
    assert!(task.postal().is_none());
    assert!(task.unknown_fields().next().is_none());
    let switched = task.encode_to_vec();
    assert!(find_len_field(&switched, 15).is_none());
    assert!(!top_level_has_varint_field(&switched, 99));
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
        // `as_mut()` is opaque (no Mut in the RPIT bound); only presence is checked here.
        assert!(view_mut.as_mut().is_some());
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
    encode_varint_field(
        field_number_const::<10>(),
        Varint::from_uint64(99),
        &mut bytes,
    );

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
    encode_varint_field(
        field_number_const::<9>(),
        Varint::from_uint64(99),
        &mut bytes,
    );

    task.merge_from(&mut &bytes[..]).unwrap();

    assert!(task.status().is_set());
    assert_eq!(task.status().get(), Status::from(99));
    assert!(task.unknown_fields().next().is_none());
}

#[test]
fn inlined_origin_unset_omits_from_wire() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");
    assert!(task.origin().is_none());
    let decoded: Task = Task::decode(&task.encode_to_vec()[..]).expect("decode");
    assert!(decoded.origin().is_none());
}

#[test]
fn inlined_origin_roundtrip_and_assign() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");
    *task.origin_mut().x_mut() = 3;
    *task.origin_mut().y_mut() = 7;

    let decoded: Task = Task::decode(&task.encode_to_vec()[..]).expect("decode");
    let o = decoded.origin().expect("origin set");
    assert_eq!(o.x(), 3);
    assert_eq!(o.y(), 7);

    let mut p = Point::new();
    *p.x_mut() = 11;
    *p.y_mut() = 13;
    task.origin_mut().copy_from(&p);
    assert_eq!(task.origin().unwrap().x(), 11);
    assert_eq!(task.origin().unwrap().y(), 13);

    let mut p2 = Point::new();
    *p2.x_mut() = 21;
    *p2.y_mut() = 22;
    task.set_origin(p2);
    assert_eq!(task.origin().unwrap().x(), 21);
    assert_eq!(task.origin().unwrap().y(), 22);
}

#[test]
fn inlined_origin_merge_into_and_clear() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");
    *task.origin_mut().x_mut() = 1;

    let mut other = Task::new();
    other.owner_id_mut().push_str("x");
    *other.origin_mut().y_mut() = 2;

    task.merge_from(&mut &other.encode_to_vec()[..]).unwrap();
    let o = task.origin().unwrap();
    assert_eq!(o.x(), 1);
    assert_eq!(o.y(), 2);

    task.clear_origin();
    assert!(task.origin().is_none());
    let decoded: Task = Task::decode(&task.encode_to_vec()[..]).expect("decode");
    assert!(decoded.origin().is_none());
}

#[test]
fn inlined_origin_unknown_stays_inside_child_len() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");

    let mut origin_payload = Vec::new();
    encode_varint_field(
        field_number_const::<1>(),
        Varint::from_uint64(1),
        &mut origin_payload,
    );
    encode_varint_field(
        field_number_const::<99>(),
        Varint::from_uint64(5),
        &mut origin_payload,
    );

    let mut bytes = Vec::new();
    encode_u64_varint((22u64) << 3 | 2, &mut bytes);
    encode_u64_varint(origin_payload.len() as u64, &mut bytes);
    bytes.extend_from_slice(&origin_payload);

    task.merge_from(&mut &bytes[..]).unwrap();
    assert!(task.unknown_fields().next().is_none());
    let o = task.origin().expect("origin set");
    assert_eq!(o.x(), 1);

    let encoded = task.encode_to_vec();
    let origin_len = find_len_field(&encoded, 22).expect("origin LEN on wire");
    assert!(
        contains_varint_field(origin_len, 99, 5),
        "unknown tag 99 must round-trip inside origin LEN, got {origin_len:?}"
    );
    assert!(
        !top_level_has_varint_field(&encoded, 99),
        "origin unknown must not surface as a top-level Task field"
    );

    task.clear_origin();
    assert!(task.origin().is_none());
    assert!(task.unknown_fields().next().is_none());
    let cleared = task.encode_to_vec();
    assert!(find_len_field(&cleared, 22).is_none());
    assert!(!top_level_has_varint_field(&cleared, 99));
}

fn x_of(p: &Point) -> i32 {
    p.x()
}

fn street_of(a: &Address) -> &str {
    a.street().get()
}

#[test]
fn owned_and_inlined_point_share_the_same_type() {
    let mut owned = Point::new();
    *owned.x_mut() = 3;
    assert_eq!(x_of(&owned), 3);

    let mut task = Task::new();
    *task.origin_mut().x_mut() = 4;
    assert_eq!(x_of(task.origin().unwrap()), 4);
}

#[test]
fn owned_boxed_and_inlined_address_share_the_same_type() {
    let mut owned = Address::new();
    owned.street_mut().set("A");
    assert_eq!(street_of(&owned), "A");

    let mut task = Task::new();
    task.assignee_mut().street_mut().set("B");
    assert_eq!(street_of(task.assignee().unwrap()), "B");

    task.postal_mut().street_mut().set("C");
    assert_eq!(street_of(task.postal().unwrap()), "C");
}

/// Decodes a base-128 varint at `bytes[i..]`. Returns `(value, bytes_consumed)`.
fn decode_u64_varint_at(bytes: &[u8], i: usize) -> Option<(u64, usize)> {
    let mut v = 0u64;
    let mut shift = 0;
    for (n, &b) in bytes[i..].iter().enumerate() {
        v |= u64::from(b & 0x7f) << shift;
        if b & 0x80 == 0 {
            return Some((v, n + 1));
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
    None
}

/// Returns the LEN payload of the first occurrence of `field_number` (wire type 2).
fn find_len_field(bytes: &[u8], field_number: u32) -> Option<&[u8]> {
    let mut i = 0;
    while i < bytes.len() {
        let (tag, n) = decode_u64_varint_at(bytes, i)?;
        i += n;
        let num = (tag >> 3) as u32;
        let wt = tag & 7;
        match wt {
            0 => {
                let (_, n) = decode_u64_varint_at(bytes, i)?;
                i += n;
            }
            2 => {
                let (len, n) = decode_u64_varint_at(bytes, i)?;
                i += n;
                let end = i + usize::try_from(len).ok()?;
                if end > bytes.len() {
                    return None;
                }
                let payload = &bytes[i..end];
                i = end;
                if num == field_number {
                    return Some(payload);
                }
            }
            _ => return None,
        }
    }
    None
}

fn contains_varint_field(bytes: &[u8], field_number: u32, value: u64) -> bool {
    let mut encoded = Vec::new();
    encode_u64_varint(u64::from(field_number) << 3, &mut encoded);
    encode_u64_varint(value, &mut encoded);
    bytes
        .windows(encoded.len())
        .any(|w| w == encoded.as_slice())
}

fn top_level_has_varint_field(bytes: &[u8], field_number: u32) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        let Some((tag, n)) = decode_u64_varint_at(bytes, i) else {
            return false;
        };
        i += n;
        let num = (tag >> 3) as u32;
        let wt = tag & 7;
        match wt {
            0 => {
                let Some((_, n)) = decode_u64_varint_at(bytes, i) else {
                    return false;
                };
                i += n;
                if num == field_number {
                    return true;
                }
            }
            2 => {
                let Some((len, n)) = decode_u64_varint_at(bytes, i) else {
                    return false;
                };
                i += n + usize::try_from(len).unwrap_or(0);
            }
            _ => return false,
        }
    }
    false
}
