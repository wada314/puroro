//! Editions `message_encoding=DELIMITED` vs default LENGTH_PREFIXED.

use crate::groups_editions::groups_editions::{Inner, Outer};
use ::puroro::Message;

#[test]
fn delimited_and_prefixed_round_trip() {
    let mut msg = Outer::new();
    *msg.delimited_mut().n_mut() = 9;
    let mut item = Inner::new();
    *item.n_mut() = 4;
    msg.items_mut().push(item);
    *msg.prefixed_mut().n_mut() = 2;

    let decoded: Outer = Outer::decode(msg.encode_to_vec().as_slice()).expect("decode");
    assert_eq!(decoded.delimited().unwrap().n().get(), 9);
    assert_eq!(decoded.items().len(), 1);
    assert_eq!(decoded.items()[0].n().get(), 4);
    assert_eq!(decoded.prefixed().unwrap().n().get(), 2);
    assert_eq!(decoded, msg);
}

#[test]
fn delimited_field_uses_group_framing() {
    let mut msg = Outer::new();
    *msg.delimited_mut().n_mut() = 1;
    let bytes = msg.encode_to_vec();
    assert_eq!(
        bytes[0] & 0x07,
        3,
        "delimited field should be SGroup: {bytes:?}"
    );
}

#[test]
fn prefixed_field_uses_len_framing() {
    let mut msg = Outer::new();
    *msg.prefixed_mut().n_mut() = 1;
    let bytes = msg.encode_to_vec();
    assert_eq!(
        bytes[0] & 0x07,
        2,
        "prefixed field should be Len: {bytes:?}"
    );
}

#[test]
fn decodes_raw_delimited_group_wire() {
    // field 1 SGroup, field 1 = 5, field 1 EGroup
    let wire = [0x0b, 0x08, 0x05, 0x0c];
    let decoded: Outer = Outer::decode(&wire[..]).expect("decode");
    assert_eq!(decoded.delimited().unwrap().n().get(), 5);
}
