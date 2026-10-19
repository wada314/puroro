//! Behavior of the handwritten [`AddressView`] spike.

use ::allocator_api2::alloc::Global;
use ::puroro::{DecodeError, Message, StringMut};
use ::puroro_sample_generated::{Address, AddressView};

fn encoded(street: &str, city: &str, postal_code: u32, latitude: f64) -> Vec<u8> {
    let mut msg = Address::new();
    msg.street_mut().set(street);
    msg.city_mut().set(city);
    *msg.postal_code_mut() = postal_code;
    *msg.latitude_mut() = latitude;
    msg.encode_to_vec()
}

fn push_string(buf: &mut Vec<u8>, field: u32, value: &str) {
    buf.push(((field << 3) | 2) as u8);
    buf.push(value.len() as u8);
    buf.extend_from_slice(value.as_bytes());
}

#[test]
fn fields_match_eager_and_strings_borrow_the_input() {
    let buf = encoded("Main St", "Oakland", 94612, 37.5);
    let view = AddressView::parse(&buf).unwrap();
    assert_eq!(view.street().unwrap(), Some("Main St"));
    assert_eq!(view.city().unwrap(), Some("Oakland"));
    assert_eq!(view.postal_code(), Some(94612));
    assert_eq!(view.latitude(), Some(37.5));

    let street = view.street().unwrap().unwrap();
    let start = buf.as_ptr() as usize;
    let end = start + buf.len();
    let ptr = street.as_ptr() as usize;
    assert!(ptr >= start && ptr + street.len() <= end);
}

#[test]
fn last_string_wins_and_unknowns_are_skipped() {
    let mut buf = Vec::new();
    push_string(&mut buf, 1, "old");
    buf.push((9 << 3) as u8); // unknown varint field 9
    buf.push(7);
    push_string(&mut buf, 1, "new");
    let view = AddressView::parse(&buf).unwrap();
    assert_eq!(view.street().unwrap(), Some("new"));
    assert_eq!(view.city().unwrap(), None);
}

#[test]
fn empty_string_is_present_and_missing_field_is_absent() {
    let mut buf = Vec::new();
    push_string(&mut buf, 1, "");
    let view = AddressView::parse(&buf).unwrap();
    assert_eq!(view.street().unwrap(), Some(""));
    assert_eq!(view.postal_code(), None);

    let empty = AddressView::parse(&[]).unwrap();
    assert_eq!(empty.street().unwrap(), None);
    assert_eq!(empty.latitude(), None);
}

#[test]
fn truncated_len_and_wrong_wire_type_fail_the_parse() {
    let mut truncated = Vec::new();
    truncated.push((1 << 3) | 2);
    truncated.push(5);
    truncated.extend_from_slice(b"ab");
    assert_eq!(
        AddressView::parse(&truncated).unwrap_err(),
        DecodeError::TruncatedMessage
    );

    // street encoded as a varint
    let wrong = vec![1 << 3, 1];
    assert_eq!(
        AddressView::parse(&wrong).unwrap_err(),
        DecodeError::InvalidTag
    );

    // unknown start-group
    let group = vec![(9 << 3) | 3];
    assert_eq!(
        AddressView::parse(&group).unwrap_err(),
        DecodeError::InvalidTag
    );
}

#[test]
fn invalid_utf8_is_reported_on_get() {
    let buf = vec![1 << 3 | 2, 1, 0xff];
    let view = AddressView::parse(&buf).unwrap();
    assert_eq!(view.street().unwrap_err(), DecodeError::InvalidUtf8);
    assert_eq!(view.street().unwrap_err(), DecodeError::InvalidUtf8);
}

#[test]
fn to_owned_matches_eager_decode_including_unknowns() {
    let mut buf = encoded("Main St", "Oakland", 94612, 37.5);
    buf.push(9 << 3);
    buf.push(1);
    let view = AddressView::parse(&buf).unwrap();
    let owned = view.to_owned(Global).unwrap();
    let eager = Address::<Global>::decode(buf.as_slice()).unwrap();
    assert_eq!(owned, eager);
    assert_eq!(owned.street().get(), "Main St");
}
