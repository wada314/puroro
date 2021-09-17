use crate::tests_pb::ser_tests2::msg::Submsg as Submsg2;
use crate::tests_pb::ser_tests2::{Enum as Enum2, Msg as Msg2};
use crate::tests_pb::ser_tests3::msg::Submsg as Submsg3;
use crate::tests_pb::ser_tests3::{Enum as Enum3, Msg as Msg3};
use ::puroro::DeserializableMessageFromBytesIterator;
use ::puroro::SerToIoWrite;

#[test]
fn test_empty2() {
    let msg: Msg2 = Msg2::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert!(buf.is_empty());
}

#[test]
fn test_empty3() {
    let msg: Msg3 = Msg3::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert!(buf.is_empty());
}

#[test]
fn test_i32_optional2() {
    let mut msg: Msg2 = Msg2::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.i32_optional = Some(10);
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(1 << 3) | 0, 10], buf.as_slice());
}

#[test]
fn test_i32_unlabeled3() {
    let mut msg: Msg3 = Msg3::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.i32_unlabeled = 10;
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(1 << 3) | 0, 10], buf.as_slice());
}

#[test]
fn test_submsg_optional_empty2() {
    let mut msg: Msg2 = Msg2::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.submsg_optional = Some(Box::new(Submsg2::default()));
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(7 << 3) | 2, 0], buf.as_slice());
}

#[test]
fn test_submsg_unlabeled_empty3() {
    let mut msg: Msg3 = Msg3::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.submsg_unlabeled = Some(Box::new(Submsg3::default()));
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(7 << 3) | 2, 0], buf.as_slice());
}

#[test]
fn test_submsg_optional_filled2() {
    let mut msg: Msg2 = Msg2::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.submsg_optional = Some(Box::new(Submsg2::default()));
    msg.submsg_optional.as_mut().unwrap().i32_optional = Some(10);
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(7 << 3) | 2, 2, (1 << 3) | 0, 10], buf.as_slice());
}

#[test]
fn test_submsg_unlabeled_filled3() {
    let mut msg: Msg3 = Msg3::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.submsg_unlabeled = Some(Box::new(Submsg3::default()));
    msg.submsg_unlabeled.as_mut().unwrap().i32_unlabeled = 10;
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(7 << 3) | 2, 2, (1 << 3) | 0, 10], buf.as_slice());
}

#[test]
fn test_ser_and_then_deser() {
    let mut msg: Msg3 = Msg3::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.i32_unlabeled = 10;
    msg.i32_repeated.extend(vec![10, 20].into_iter());
    msg.float_unlabeled = 10.0;
    msg.float_repeated.extend(vec![10.0, 20.0].into_iter());
    msg.string_unlabeled = "test".into();
    msg.string_repeated
        .extend(vec!["abc".into(), "def".into()].into_iter());
    msg.submsg_unlabeled = Some(Box::new(Submsg3::default()));
    msg.submsg_unlabeled.as_mut().unwrap().i32_unlabeled = 100;
    msg.submsg_repeated
        .extend(vec![100, 200].into_iter().map(|i| {
            let mut submsg = Submsg3::default();
            submsg.i32_unlabeled = i;
            submsg
        }));
    msg.enum_unlabeled = Enum3::First;
    msg.enum_repeated.extend(vec![Enum3::Zeroth, Enum3::Tenth]);
    msg.very_large_field_number = ((1i64 << 31) - 1) as i32;

    msg.ser(&mut buf).unwrap();
    use ::std::io::Read as _;
    let mut new_msg: Msg3 = Msg3::default();
    new_msg.deser(buf.bytes()).unwrap();

    assert_eq!(msg, new_msg);
}
