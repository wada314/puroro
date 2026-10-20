//! Sample messages and wire payloads for the generated-path benches.

use ::std::sync::LazyLock;

use ::allocator_api2::alloc::Global;
use ::puroro::{BytesMut, MapMut, MapRef, Message, OneofView, RepeatedStringMut, StringMut};
use ::puroro_rt::encode::encode_varint_field;
use ::puroro_rt::{FieldNumber, Varint};
use ::puroro_sample_generated::task::NotificationCase;
use ::puroro_sample_generated::{
    Address, AddressLazy, Marker, Point, PointLazy, Priority, Status, Task, TaskLazy,
};

/// Map entries in the map-heavy `Task` payload.
pub const MAP_LEN: usize = 64;

/// Unrecognized tags appended after a known field-1 varint.
pub const UNKNOWN_LEN: usize = 64;

pub static TYPICAL_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| typical_task().encode_to_vec());

pub static MAP_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| map_heavy_task().encode_to_vec());

pub static UNKNOWN_BYTES: LazyLock<Vec<u8>> = LazyLock::new(unknown_heavy_bytes);

pub static MAP_KEYS: LazyLock<Vec<String>> =
    LazyLock::new(|| (0..MAP_LEN).map(|i| format!("attr-{i:04}")).collect());

pub static ONEOF_EMAIL_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| oneof_email_task().encode_to_vec());

pub static ONEOF_WEBHOOK_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| oneof_webhook_task().encode_to_vec());

pub static ONEOF_POSTAL_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| oneof_postal_task().encode_to_vec());

fn address(street: &str, city: &str, postal_code: u32, latitude: f64) -> Address {
    let mut a = Address::new();
    a.street_mut().set(street);
    a.city_mut().set(city);
    *a.postal_code_mut() = postal_code;
    *a.latitude_mut() = latitude;
    a
}

/// Mixed-field `Task` used as the generated-message baseline.
pub fn typical_task() -> Task {
    let mut task = Task::new();
    task.title_mut().set("Write docs");
    *task.score_mut() = 42;
    *task.max_retries_mut() = 5;
    task.owner_id_mut().set("user-1");
    task.payload_mut()
        .extend_from_slice(b"payload-bytes-xxxxxxxx");
    task.tag_ids_mut().extend_from_slice(&[10, 20, 30, 40]);
    task.scores_mut().extend_from_slice(&[1, 2, 3]);
    task.labels_mut().push().push_str("urgent");
    task.labels_mut().push().push_str("docs");
    *task.status_mut() = Status::PENDING;
    *task.priority_mut() = Priority::HIGH;
    *task.done_mut() = true;
    *task.flag_mut() = false;
    task.email_address_mut().set("a@example.com");
    task.set_assignee(address("1 Main St", "Tokyo", 1_000_001, 35.6812));
    task.watchers_mut()
        .push(address("2 Side Rd", "Osaka", 5_300_001, 34.6937));
    task.watchers_mut()
        .push(address("3 Hill Ave", "Kyoto", 6_000_001, 35.0116));
    task.votes_mut().extend_from_slice(&[true, false, true]);
    *task.attributes_mut().entry_mut("region") = 81;
    *task.attributes_mut().entry_mut("tier") = 2;
    *task.origin_mut().x_mut() = 10;
    *task.origin_mut().y_mut() = 20;
    task
}

fn oneof_required() -> Task {
    let mut task = Task::new();
    task.owner_id_mut().set("user-1");
    task
}

/// Oneof LEN string variant only.
pub fn oneof_email_task() -> Task {
    let mut task = oneof_required();
    task.email_address_mut().set("a@example.com");
    task
}

/// Oneof VARINT variant only.
pub fn oneof_webhook_task() -> Task {
    let mut task = oneof_required();
    *task.webhook_id_mut() = 7;
    task
}

/// Oneof nested-message variant only.
pub fn oneof_postal_task() -> Task {
    let mut task = oneof_required();
    let postal = task.postal_mut();
    postal.street_mut().set("1 Main St");
    postal.city_mut().set("Tokyo");
    *postal.postal_code_mut() = 1_000_001;
    task
}

/// Cycle the notification group through the cheap variants (clear last).
pub fn switch_oneof(task: &mut Task) -> usize {
    task.email_address_mut().set("a@example.com");
    let mut n = usize::from(task.notification().case() == Some(NotificationCase::EmailAddress));
    task.phone_number_mut().set("555");
    n = n.wrapping_add(usize::from(
        task.notification().case() == Some(NotificationCase::PhoneNumber),
    ));
    *task.webhook_id_mut() = 9;
    n = n.wrapping_add(task.webhook_id().get() as usize);
    *task.urgent_mut() = true;
    n = n.wrapping_add(usize::from(task.urgent().get()));
    task.clear_notification();
    n.wrapping_add(usize::from(task.notification().case().is_some()))
}

pub fn read_oneof_case(task: &Task) -> Option<NotificationCase> {
    task.notification().case()
}

/// `Task` whose payload is almost only `map<string, int32> attributes`.
pub fn map_heavy_task() -> Task {
    let mut task = Task::new();
    task.owner_id_mut().set("user-1");
    for (i, key) in MAP_KEYS.iter().enumerate() {
        *task.attributes_mut().entry_mut(key.as_str()) = i as i32;
    }
    task
}

fn unknown_heavy_bytes() -> Vec<u8> {
    let mut wire = Vec::new();
    encode_varint_field(
        FieldNumber::try_new(1).expect("field 1"),
        Varint::from_int32(3),
        &mut wire,
    );
    for i in 0..UNKNOWN_LEN {
        let field = FieldNumber::try_new(100 + i as u32).expect("unknown field");
        encode_varint_field(field, Varint::from_uint64(i as u64), &mut wire);
    }
    wire
}

fn touch_address(addr: &Address) -> usize {
    addr.street().get().len()
        + addr.city().get().len()
        + addr.postal_code().get() as usize
        + addr.latitude().get().to_bits() as usize
}

/// `assignee` is pinned lazy (`AddressLazy`) on both eager and lazy `Task`.
fn touch_address_lazy(addr: &AddressLazy) -> usize {
    addr.street().expect("street").get().len()
        + addr.city().expect("city").get().len()
        + addr.postal_code().expect("postal_code").get() as usize
        + addr.latitude().expect("latitude").get().to_bits() as usize
}

fn touch_point(point: &Point) -> usize {
    (point.x() as usize).wrapping_add(point.y() as usize)
}

fn touch_point_lazy(point: &PointLazy) -> usize {
    (point.x().expect("x") as usize).wrapping_add(point.y().expect("y") as usize)
}

/// Sparse access: one explicit string (the lazy-path contrast).
pub fn read_one(task: &Task) -> usize {
    task.title().get().len()
}

/// Same getter as [`read_one`], on the lazy alias (materialises that string).
pub fn read_one_lazy(task: &TaskLazy) -> usize {
    task.title().expect("title").get().len()
}

/// Touch every generated getter on the typical `Task`.
pub fn read_all(task: &Task) -> usize {
    let mut n = 0usize;
    n = n.wrapping_add(task.title().get().len());
    n = n.wrapping_add(task.score() as usize);
    n = n.wrapping_add(task.max_retries().get() as usize);
    n = n.wrapping_add(task.owner_id().get().len());
    n = n.wrapping_add(task.payload().get().len());
    n = n.wrapping_add(task.tag_ids().iter().copied().sum::<i32>() as usize);
    n = n.wrapping_add(task.scores().len());
    n = n.wrapping_add(task.labels().iter().map(|s| s.len()).sum());
    n = n.wrapping_add(task.status().is_set() as usize);
    n = n.wrapping_add(task.priority().is_set() as usize);
    if let Some(assignee) = task.assignee() {
        n = n.wrapping_add(touch_address_lazy(assignee));
    }
    n = n.wrapping_add(task.done() as usize);
    n = n.wrapping_add(task.flag().is_set() as usize);
    for watcher in task.watchers() {
        n = n.wrapping_add(touch_address(watcher));
    }
    n = n.wrapping_add(task.votes().len());
    n = n.wrapping_add(task.attributes().len());
    n = n.wrapping_add(task.attributes().get("region").copied().unwrap_or(0) as usize);
    if let Some(origin) = task.origin() {
        n = n.wrapping_add(touch_point(origin));
    }
    n = n.wrapping_add(task.email_address().get().len());
    n = n.wrapping_add(task.notification().case().is_some() as usize);
    n
}

/// Touch the same fields as [`read_all`] on [`TaskLazy`] (materialises LEN slots).
pub fn read_all_lazy(task: &TaskLazy) -> usize {
    let mut n = 0usize;
    n = n.wrapping_add(task.title().expect("title").get().len());
    n = n.wrapping_add(task.score().expect("score") as usize);
    n = n.wrapping_add(task.max_retries().expect("max_retries").get() as usize);
    n = n.wrapping_add(task.owner_id().expect("owner_id").get().len());
    n = n.wrapping_add(task.payload().expect("payload").get().len());
    n = n.wrapping_add(
        task.tag_ids()
            .expect("tag_ids")
            .iter()
            .copied()
            .sum::<i32>() as usize,
    );
    n = n.wrapping_add(task.scores().expect("scores").len());
    n = n.wrapping_add(task.labels().expect("labels").iter().map(|s| s.len()).sum());
    n = n.wrapping_add(task.status().expect("status").is_set() as usize);
    n = n.wrapping_add(task.priority().expect("priority").is_set() as usize);
    if let Some(assignee) = task.assignee().expect("assignee") {
        n = n.wrapping_add(touch_address_lazy(assignee));
    }
    n = n.wrapping_add(task.done().expect("done") as usize);
    n = n.wrapping_add(task.flag().expect("flag").is_set() as usize);
    for watcher in task.watchers().expect("watchers") {
        n = n.wrapping_add(touch_address_lazy(watcher));
    }
    n = n.wrapping_add(task.votes().expect("votes").len());
    let attributes = task.attributes().expect("attributes");
    n = n.wrapping_add(attributes.len());
    n = n.wrapping_add(attributes.get("region").copied().unwrap_or(0) as usize);
    if let Some(origin) = task.origin().expect("origin") {
        n = n.wrapping_add(touch_point_lazy(origin));
    }
    n = n.wrapping_add(task.email_address().expect("email_address").get().len());
    n = n.wrapping_add(task.notification().expect("notification").is_some() as usize);
    n
}

pub fn lookup_map(task: &Task) -> i32 {
    MAP_KEYS
        .iter()
        .map(|key| task.attributes().get(key.as_str()).copied().unwrap_or(0))
        .sum()
}

pub fn lookup_map_lazy(task: &TaskLazy) -> i32 {
    let attributes = task.attributes().expect("attributes");
    MAP_KEYS
        .iter()
        .map(|key| attributes.get(key.as_str()).copied().unwrap_or(0))
        .sum()
}

pub fn assert_payloads() {
    let typical = typical_task();
    typical.validate().expect("typical validate");
    let decoded = Task::<Global>::decode(TYPICAL_BYTES.as_slice()).expect("typical decode");
    // Map encode order is hash iteration, so wire bytes are not stable.
    assert_eq!(decoded, typical);
    assert_eq!(read_all(&decoded), read_all(&typical));
    assert_eq!(read_one(&decoded), "Write docs".len());

    let lazy = TaskLazy::<Global>::decode(TYPICAL_BYTES.as_slice()).expect("typical lazy decode");
    let lazy_bytes = lazy.encode_to_vec().expect("lazy encode");
    let from_lazy_wire = Task::<Global>::decode(lazy_bytes.as_slice()).expect("decode lazy encode");
    assert_eq!(from_lazy_wire, typical);
    assert_eq!(read_all_lazy(&lazy), read_all(&typical));
    assert_eq!(read_one_lazy(&lazy), "Write docs".len());
    assert_eq!(lazy.into_eager().expect("into_eager"), typical);

    let map = Task::<Global>::decode(MAP_BYTES.as_slice()).expect("map decode");
    assert_eq!(map.attributes().len(), MAP_LEN);
    assert_eq!(lookup_map(&map), (0..MAP_LEN as i32).sum::<i32>());

    let map_lazy = TaskLazy::<Global>::decode(MAP_BYTES.as_slice()).expect("map lazy decode");
    assert_eq!(map_lazy.attributes().expect("map lazy len").len(), MAP_LEN);
    assert_eq!(lookup_map_lazy(&map_lazy), (0..MAP_LEN as i32).sum::<i32>());

    let preserved = Point::<Global>::decode(UNKNOWN_BYTES.as_slice()).expect("point decode");
    assert_eq!(preserved.x(), 3);
    assert_eq!(preserved.unknown_fields().count(), UNKNOWN_LEN);

    let discarded = Marker::<Global>::decode(UNKNOWN_BYTES.as_slice()).expect("marker decode");
    assert_eq!(discarded.n(), 3);
    assert!(discarded.unknown_fields().next().is_none());
    assert_ne!(discarded.encode_to_vec(), *UNKNOWN_BYTES);

    let email = Task::<Global>::decode(ONEOF_EMAIL_BYTES.as_slice()).expect("oneof email decode");
    assert_eq!(email, oneof_email_task());
    assert_eq!(
        read_oneof_case(&email),
        Some(NotificationCase::EmailAddress)
    );

    let webhook =
        Task::<Global>::decode(ONEOF_WEBHOOK_BYTES.as_slice()).expect("oneof webhook decode");
    assert_eq!(webhook, oneof_webhook_task());
    assert_eq!(read_oneof_case(&webhook), Some(NotificationCase::WebhookId));

    let postal =
        Task::<Global>::decode(ONEOF_POSTAL_BYTES.as_slice()).expect("oneof postal decode");
    assert_eq!(postal, oneof_postal_task());
    assert_eq!(read_oneof_case(&postal), Some(NotificationCase::Postal));

    let mut switched = oneof_required();
    assert_eq!(switch_oneof(&mut switched), 1 + 1 + 9 + 1);
    assert!(read_oneof_case(&switched).is_none());
}
