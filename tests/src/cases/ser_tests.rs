use crate::tests_pb::ser_tests::{msg::Submsg, Msg};
use ::puroro::SerToIoWrite;

#[test]
fn test_empty() {
    let msg = Msg::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert!(buf.is_empty());
}

#[test]
fn test_i32_unlabeled() {
    let mut msg = Msg::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.i32_unlabeled = 10;
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(1 << 3) | 0, 10], buf.as_slice());
}

#[test]
fn test_submsg_unlabeled() {
    let mut msg = Msg::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.submsg_unlabeled = Some(Box::new(Submsg::default()));
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(7 << 3) | 2, 0], buf.as_slice());
}
