//! Sample messages and wire payloads for the generated-path benches.

use ::std::sync::LazyLock;

use ::allocator_api2::alloc::Global;
use ::puroro::{BytesMut, MapMut, MapRef, Message, OneofView, RepeatedStringMut, StringMut};
use ::puroro_rt::encode::encode_varint_field;
use ::puroro_rt::{FieldNumber, Varint};
use ::puroro_sample_generated::{Address, Marker, Point, Priority, Status, Task};

/// Map entries in the map-heavy `Task` payload.
pub const MAP_LEN: usize = 64;

/// Unrecognized tags appended after a known field-1 varint.
pub const UNKNOWN_LEN: usize = 64;

pub static TYPICAL_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| typical_task().encode_to_vec());

pub static MAP_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| map_heavy_task().encode_to_vec());

pub static UNKNOWN_BYTES: LazyLock<Vec<u8>> = LazyLock::new(unknown_heavy_bytes);

pub static MAP_KEYS: LazyLock<Vec<String>> =
    LazyLock::new(|| (0..MAP_LEN).map(|i| format!("attr-{i:04}")).collect());

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

/// Sparse access: one explicit string (the lazy-path contrast).
pub fn read_one(task: &Task) -> usize {
    task.title().get().len()
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
        n = n.wrapping_add(touch_address(assignee));
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
        n = n.wrapping_add(origin.x() as usize);
        n = n.wrapping_add(origin.y() as usize);
    }
    n = n.wrapping_add(task.email_address().get().len());
    n = n.wrapping_add(task.notification().case().is_some() as usize);
    n
}

pub fn lookup_map(task: &Task) -> i32 {
    MAP_KEYS
        .iter()
        .map(|key| task.attributes().get(key.as_str()).copied().unwrap_or(0))
        .sum()
}

pub fn assert_payloads() {
    let typical = typical_task();
    typical.validate().expect("typical validate");
    let decoded = Task::<Global>::decode(TYPICAL_BYTES.as_slice()).expect("typical decode");
    assert_eq!(decoded.encode_to_vec(), *TYPICAL_BYTES);
    assert_eq!(read_all(&decoded), read_all(&typical));
    assert_eq!(read_one(&decoded), "Write docs".len());

    let map = Task::<Global>::decode(MAP_BYTES.as_slice()).expect("map decode");
    assert_eq!(map.attributes().len(), MAP_LEN);
    assert_eq!(lookup_map(&map), (0..MAP_LEN as i32).sum::<i32>());

    let preserved = Point::<Global>::decode(UNKNOWN_BYTES.as_slice()).expect("point decode");
    assert_eq!(preserved.x(), 3);
    assert_eq!(preserved.unknown_fields().count(), UNKNOWN_LEN);

    let discarded = Marker::<Global>::decode(UNKNOWN_BYTES.as_slice()).expect("marker decode");
    assert_eq!(discarded.n(), 3);
    assert!(discarded.unknown_fields().next().is_none());
    assert_ne!(discarded.encode_to_vec(), *UNKNOWN_BYTES);
}
