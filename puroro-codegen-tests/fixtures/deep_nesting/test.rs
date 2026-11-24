//! Three-level nested message path + round-trip.
//!
//! Also a compile gate: nested module path `demo::l1::l2::L3` resolves via the
//! `use` below.

use crate::deep_nesting::demo::l1::l2::L3;
use crate::deep_nesting::demo::L1;
use ::puroro::{Message, StringMut};

#[test]
fn nested_path_round_trip() {
    let mut msg = L1::new();
    msg.mid_mut().leaf_mut().name_mut().push_str("deep");

    let decoded: L1 = L1::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.mid().unwrap().leaf().unwrap().name().get(), "deep");

    // Keep a value use so the nested path stays referenced without a
    // type-only `let _: T = T::new()` smoke assert.
    assert!(L3::new().encode_to_vec().is_empty());
}
