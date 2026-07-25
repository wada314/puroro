//! Three-level nested message path + round-trip.

use crate::deep_nesting::demo::l1::l2::L3;
use crate::deep_nesting::demo::L1;
use ::puroro::Message;

#[test]
fn nested_path_round_trip() {
    let mut msg = L1::new();
    msg.mid_mut().leaf_mut().name_mut().push_str("deep");

    let decoded: L1 = L1::decode(&msg.encode_to_vec()[..]).expect("decode");
    assert_eq!(decoded.mid().unwrap().leaf().unwrap().name().get(), "deep");

    // Nested type is reachable via the generated module path.
    let _: L3 = L3::new();
}
