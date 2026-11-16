//! Plugin acceptance for the handwritten sample's schema.
//!
//! Does not depend on `puroro-sample-generated`. The opt-in comparison lives
//! in `sample_parity` and is not compiled unless `--features sample-parity`.

use crate::reference_schema::example::Task;
use puroro::{Message, StringMut};

#[test]
fn title_and_owner_round_trip() {
    let mut msg = Task::new();
    msg.title_mut().set("docs");
    msg.owner_id_mut().set("user-1");
    let bytes = msg.encode_to_vec();
    let decoded: Task = Task::decode(&bytes[..]).expect("decode");
    assert_eq!(decoded.title().get(), "docs");
    assert_eq!(decoded.owner_id().get(), "user-1");
}
