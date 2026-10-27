//! Classic proto2 `group` fields: optional + repeated, round-trip and raw wire.
//!
//! Protoc lowercases the group field name (`OptionalItem` → `optionalitem`).

use crate::groups_proto2::groups_proto2::holder::{OptionalItem, RepeatedItem};
use crate::groups_proto2::groups_proto2::Holder;
use ::puroro::{BytesMut, Message};

#[test]
fn defaults() {
    let msg = Holder::new();
    assert!(msg.optionalitem().is_none());
    assert!(msg.repeateditem().is_empty());
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn optional_and_repeated_group_round_trip() {
    let mut msg = Holder::new();
    *msg.optionalitem_mut().id_mut() = 7;
    msg.optionalitem_mut().label_mut().set(b"x");
    let mut item = RepeatedItem::new();
    *item.n_mut() = 3;
    msg.repeateditem_mut().push(item);

    let decoded: Holder = Holder::decode(msg.encode_to_vec().as_slice()).expect("decode");
    assert_eq!(decoded.optionalitem().unwrap().id().get(), 7);
    assert_eq!(decoded.optionalitem().unwrap().label().get(), b"x");
    assert_eq!(decoded.repeateditem().len(), 1);
    assert_eq!(decoded.repeateditem()[0].n().get(), 3);
    assert_eq!(decoded, msg);
}

#[test]
fn decodes_raw_sgroup_egroup_wire() {
    // field 1 SGroup, field 1 = 42 (varint), field 1 EGroup
    let wire = [
        0x0b, // 1:SGroup
        0x08, 0x2a, // 1:varint = 42
        0x0c, // 1:EGroup
    ];
    let decoded: Holder = Holder::decode(&wire[..]).expect("decode group wire");
    assert_eq!(decoded.optionalitem().unwrap().id().get(), 42);
    let _ = OptionalItem::new();
}

#[test]
fn encode_uses_group_framing_not_len() {
    let mut msg = Holder::new();
    *msg.optionalitem_mut().id_mut() = 1;
    let bytes = msg.encode_to_vec();
    // First tag must be SGroup (wt=3), not Len (wt=2).
    assert_eq!(bytes[0] & 0x07, 3, "expected SGroup tag, got {bytes:?}");
    assert_eq!(*bytes.last().unwrap() & 0x07, 4, "expected trailing EGroup");
}
