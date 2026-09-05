//! Read a `Task` only through [`TaskMessageFallible`].

use ::puroro::{BytesMut, MapMut, MapRef, OneofView, RepeatedStringMut, StringMut};
use ::puroro_sample_generated::task::NotificationCase;
use ::puroro_sample_generated::{Address, Point, Priority, Status, Task, TaskMessageFallible};

/// Touches every `TaskMessageFallible` getter. Must not call inherent `Task`
/// accessors (except through the trait impl).
fn read_via_trait<M: TaskMessageFallible>(m: &M) -> Result<usize, M::Error> {
    let mut n = 0usize;
    n = n.wrapping_add(m.title()?.get().len());
    n = n.wrapping_add(m.score()? as usize);
    n = n.wrapping_add(m.max_retries()?.get() as usize);
    n = n.wrapping_add(m.owner_id()?.get().len());
    n = n.wrapping_add(m.payload()?.get().len());
    n = n.wrapping_add(m.tag_ids()?.iter().copied().sum::<i32>() as usize);
    n = n.wrapping_add(m.scores()?.len());
    n = n.wrapping_add(m.labels()?.iter().map(|s| s.len()).sum());
    n = n.wrapping_add(m.status()?.is_set() as usize);
    n = n.wrapping_add(m.priority()?.is_set() as usize);
    if let Some(addr) = m.assignee()? {
        n = n.wrapping_add(addr.street().get().len());
    }
    n = n.wrapping_add(m.done()? as usize);
    n = n.wrapping_add(m.flag()?.is_set() as usize);
    n = n.wrapping_add(m.watchers()?.len());
    n = n.wrapping_add(m.votes()?.len());
    n = n.wrapping_add(m.attributes()?.len());
    if let Some(origin) = m.origin()? {
        n = n.wrapping_add(origin.x() as usize);
    }
    n = n.wrapping_add(m.notification()?.case().is_some() as usize);
    n = n.wrapping_add(m.email_address()?.get().len());
    n = n.wrapping_add(m.phone_number()?.get().len());
    n = n.wrapping_add(m.webhook_id()?.get() as usize);
    if let Some(postal) = m.postal()? {
        n = n.wrapping_add(postal.city().get().len());
    }
    n = n.wrapping_add(m.urgent()?.is_set() as usize);
    Ok(n)
}

#[test]
fn eager_task_readable_through_fallible_trait() {
    let mut task = Task::new();
    task.title_mut().set("lazy-step-1");
    *task.score_mut() = 3;
    *task.max_retries_mut() = 2;
    task.owner_id_mut().set("owner");
    task.payload_mut().set(b"xy");
    task.tag_ids_mut().extend_from_slice(&[1, 2]);
    task.scores_mut().push(9);
    task.labels_mut().push().push_str("a");
    *task.status_mut() = Status::PENDING;
    *task.priority_mut() = Priority::HIGH;
    let mut addr = Address::new();
    addr.street_mut().set("St");
    task.set_assignee(addr);
    *task.done_mut() = true;
    *task.flag_mut() = true;
    task.watchers_mut().push(Address::new());
    task.votes_mut().push(true);
    *task.attributes_mut().entry_mut("k") = 1;
    let mut origin = Point::new();
    *origin.x_mut() = 4;
    task.set_origin(origin);
    task.email_address_mut().set("e@x");

    let n = read_via_trait(&task).unwrap_or_else(|e| match e {});
    assert!(n > 0);
    assert_eq!(
        TaskMessageFallible::title(&task)
            .unwrap_or_else(|e| match e {})
            .get(),
        "lazy-step-1"
    );
    assert_eq!(
        TaskMessageFallible::notification(&task)
            .unwrap_or_else(|e| match e {})
            .case(),
        Some(NotificationCase::EmailAddress)
    );
}
