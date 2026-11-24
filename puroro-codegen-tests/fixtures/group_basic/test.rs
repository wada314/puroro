//! Proto2 `group` fields: singular + repeated, eager and lazy.

use crate::group_basic::demo::envelope::{Header, Item};
use crate::group_basic::demo::{Envelope, EnvelopeLazy};
use ::allocator_api2::alloc::Global;
use ::puroro::{BytesMut, Message};

#[test]
fn header_round_trip_eager() {
    let mut env = Envelope::new();
    env.header_mut().name_mut().set(b"hdr");
    *env.header_mut().id_mut() = 7;
    env.note_mut().set(b"n");

    let bytes = env.encode_to_vec();
    // Field 1 SGroup = (1<<3)|3 = 0x0b; EGroup = 0x0c.
    assert_eq!(bytes[0], 0x0b);
    assert!(bytes.contains(&0x0c));

    let decoded: Envelope = Envelope::decode(bytes.as_slice()).unwrap();
    assert_eq!(decoded.header().unwrap().name().get(), b"hdr");
    assert_eq!(decoded.header().unwrap().id().get(), 7);
    assert_eq!(decoded.note().get(), b"n");
    assert_eq!(decoded, env);
}

#[test]
fn empty_header_still_encodes_group_frame() {
    let mut env = Envelope::new();
    let _ = env.header_mut();
    let bytes = env.encode_to_vec();
    assert_eq!(bytes, [0x0b, 0x0c]);

    let decoded: Envelope = Envelope::decode(bytes.as_slice()).unwrap();
    assert!(decoded.header().is_some());
    assert!(!decoded.header().unwrap().name().is_set());
}

#[test]
fn repeated_item_appends() {
    let mut env = Envelope::new();
    let mut first = Item::new();
    *first.qty_mut() = 1;
    env.item_mut().push(first);
    let mut second = Item::new();
    *second.qty_mut() = 2;
    env.item_mut().push(second);
    assert_eq!(env.item().len(), 2);

    let decoded: Envelope = Envelope::decode(env.encode_to_vec().as_slice()).unwrap();
    assert_eq!(decoded.item().len(), 2);
    assert_eq!(decoded.item()[0].qty().get(), 1);
    assert_eq!(decoded.item()[1].qty().get(), 2);
}

#[test]
fn lazy_header_matches_eager() {
    let mut env = Envelope::new();
    env.header_mut().name_mut().set(b"lazy");
    *env.header_mut().id_mut() = 3;
    let mut item = Item::new();
    *item.qty_mut() = 9;
    env.item_mut().push(item);

    let lazy = EnvelopeLazy::<Global>::decode(env.encode_to_vec().as_slice()).unwrap();
    assert_eq!(
        lazy.header().unwrap().unwrap().name().unwrap().get(),
        b"lazy"
    );
    assert_eq!(lazy.header().unwrap().unwrap().id().unwrap().get(), 3);
    assert_eq!(lazy.item().unwrap().len(), 1);
    assert_eq!(lazy.item().unwrap()[0].qty().unwrap().get(), 9);
    assert_eq!(lazy.into_eager().unwrap(), env);
}

#[test]
fn nested_types_are_named() {
    let _: Header<Global> = Header::new();
    let _: Item<Global> = Item::new();
}
