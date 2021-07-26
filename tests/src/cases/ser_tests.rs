use crate::tests_pb::ser_tests::{msg::Submsg, Msg};
use ::puroro::DeserFromBytesIter;
use ::puroro::SerToIoWrite;
use ::puroro_internal::SimpleImpl;

#[test]
fn test_empty() {
    let msg: Msg = Msg::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.ser(&mut buf).unwrap();
    assert!(buf.is_empty());
}

#[test]
fn test_i32_unlabeled() {
    let mut msg: Msg = Msg::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.i32_unlabeled = 10;
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(1 << 3) | 0, 10], buf.as_slice());
}

#[test]
fn test_submsg_unlabeled_empty() {
    let mut msg: Msg = Msg::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.submsg_unlabeled = Some(Box::new(Submsg::default()));
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(7 << 3) | 2, 0], buf.as_slice());
}

#[test]
fn test_submsg_unlabeled_filled() {
    let mut msg: Msg = Msg::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.submsg_unlabeled = Some(Box::new(Submsg::default()));
    msg.submsg_unlabeled.as_mut().unwrap().i32_unlabeled = 10;
    msg.ser(&mut buf).unwrap();
    assert_eq!(&[(7 << 3) | 2, 2, (1 << 3) | 0, 10], buf.as_slice());
}

#[test]
fn test_ser_and_then_deser() {
    let mut msg = Msg::<SimpleImpl>::default();
    let mut buf: Vec<u8> = Vec::new();
    msg.i32_unlabeled = 10;
    msg.i32_repeated.extend(vec![10, 20].into_iter());
    msg.float_unlabeled = 10.0;
    msg.float_repeated.extend(vec![10.0, 20.0].into_iter());
    msg.string_unlabeled = "test".to_string();
    msg.string_repeated
        .extend(vec!["abc".to_string(), "def".to_string()].into_iter());
    msg.submsg_unlabeled = Some(Box::new(Submsg::default()));
    msg.submsg_unlabeled.as_mut().unwrap().i32_unlabeled = 100;
    msg.submsg_repeated
        .extend(vec![100, 200].into_iter().map(|i| {
            let mut submsg = Submsg::default();
            submsg.i32_unlabeled = i;
            submsg
        }));
    msg.very_large_field_number = ((1i64 << 31) - 1) as i32;

    msg.ser(&mut buf).unwrap();
    use ::std::io::Read as _;
    let mut new_msg = Msg::default();
    new_msg.deser(buf.bytes()).unwrap();

    assert_eq!(msg, new_msg);
}
