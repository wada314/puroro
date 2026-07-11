use ::puroro::{MessageDecode, MessageEncode};
use ::puroro_sample_generated::task::{NotificationCase, NotificationMut, NotificationRef};
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
    *task.status_mut() = Status::PENDING;
    *task.priority_mut() = Priority::HIGH;
    *task.done_mut() = true;
    *task.flag_mut() = false;
    task.email_address_mut().push_str("a@example.com");

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
        Some(NotificationRef::EmailAddress(s)) if s == "a@example.com"
    ));
    let a = decoded.assignee().unwrap();
    assert_eq!(a.street().get(), "1 Main St");
    assert_eq!(a.city().get(), "Tokyo");
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
        decoded.notification_case(),
        Some(NotificationCase::WebhookId)
    );
    assert!(matches!(
        decoded.notification().as_ref(),
        Some(NotificationRef::WebhookId(4321))
    ));
    assert!(decoded.webhook_id().is_set());
    assert_eq!(decoded.webhook_id().get(), 4321);
}

#[test]
fn oneof_scalar_getter_uses_custom_default_when_unset() {
    // Official const-getter contract: unset / other variant → custom default,
    // without selecting the variant. `webhook_id = 14 [default = -1]`.
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");

    assert!(task.notification_case().is_none());
    assert!(!task.webhook_id().is_set());
    assert_eq!(task.webhook_id().get(), -1);

    task.email_address_mut().push_str("a@example.com");
    assert_eq!(
        task.notification_case(),
        Some(NotificationCase::EmailAddress)
    );
    assert!(!task.webhook_id().is_set());
    assert_eq!(task.webhook_id().get(), -1);
    // String members without [default] still fall back to the type default.
    assert!(!task.phone_number().is_set());
    assert_eq!(task.phone_number().get(), "");
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

    assert_eq!(decoded.notification_case(), Some(NotificationCase::Postal));
    let Some(NotificationRef::Postal(addr)) = decoded.notification().as_ref() else {
        panic!("expected postal variant");
    };
    assert_eq!(addr.street().get(), "5 Oak Ave");
    assert_eq!(addr.city().get(), "Kyoto");
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
        Some(NotificationRef::WebhookId(3))
    ));
}

#[test]
fn oneof_group_bound_views_when_unset() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");

    // Bound views are always available, including when the group is unset.
    let view = task.notification();
    assert!(view.case().is_none());
    assert!(view.as_ref().is_none());

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
        assert!(matches!(
            view_mut.as_view().as_ref(),
            Some(NotificationRef::EmailAddress(s)) if s == "a@example.com"
        ));
        assert!(matches!(
            view_mut.as_mut(),
            Some(NotificationMut::EmailAddress(_))
        ));
    }

    task.notification_mut().clear();
    assert!(task.notification_case().is_none());
    assert!(task.notification().as_ref().is_none());
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
fn closed_enum_unknown_goes_to_unknown_fields() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("x");

    // field 10 (priority) = 99 (unknown closed enum value), wire: tag + varint
    let mut bytes = Vec::new();
    ::puroro_rt::encode::encode_varint_field(10, 99, &mut bytes);

    task.merge_from(&mut &bytes[..]).unwrap();

    assert!(!task.priority().is_set());
    assert!(!task.unknown_fields().is_empty());
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

#[test]
fn oneof_bool_variant_roundtrip() {
    let mut task = Task::new();
    task.owner_id_mut().push_str("user-1");
    *task.urgent_mut() = true;
    task.validate().unwrap();

    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();

    assert_eq!(
        decoded.notification_case(),
        Some(NotificationCase::Urgent)
    );
    assert!(matches!(
        decoded.notification().as_ref(),
        Some(NotificationRef::Urgent(true))
    ));
    assert!(decoded.urgent().is_set());
    assert!(decoded.urgent().get());

    // Type-default false is still emitted when the oneof case is selected.
    *task.urgent_mut() = false;
    let bytes = task.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).unwrap();
    assert_eq!(
        decoded.notification_case(),
        Some(NotificationCase::Urgent)
    );
    assert!(matches!(
        decoded.notification().as_ref(),
        Some(NotificationRef::Urgent(false))
    ));
}
