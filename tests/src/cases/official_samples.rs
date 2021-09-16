/// The sample cases documented in the Protobuf official encoding document:
/// https://developers.google.com/protocol-buffers/docs/encoding
use ::puroro::DeserFromBytesIter;
use ::std::borrow::Cow;
use ::std::default::Default;
use ::tests_pb::official_samples2 as s2;
use ::tests_pb::official_samples3 as s3;

const TEST1_INPUT: &[u8] = &[0x08, 0x96, 0x01];
const TEST2_INPUT: &[u8] = &[0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
const TEST3_INPUT: &[u8] = &[0x1a, 0x03, 0x08, 0x96, 0x01];
const TEST4_INPUT: &[u8] = &[0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];

const TEST1_EXPECTED: i32 = 150;
const TEST2_EXPECTED: &str = "testing";
const TEST3_EXPECTED: i32 = 150;
const TEST4_EXPECTED: &[i32] = &[3, 270, 86942];

#[test]
fn proto2_test1_simple() {
    use std::io::Read as _;
    let mut t1 = s2::Test1::default();
    t1.deser(TEST1_INPUT.bytes()).unwrap();
    assert_eq!(Some(TEST1_EXPECTED), t1.a);
}

#[test]
fn proto3_test1_simple() {
    use std::io::Read as _;
    let mut t1 = s3::Test1::default();
    t1.deser(TEST1_INPUT.bytes()).unwrap();
    assert_eq!(TEST1_EXPECTED, t1.a);
}

#[test]
fn proto2_test2_simple() {
    use std::io::Read as _;
    let mut t2 = s2::Test2::default();
    t2.deser(TEST2_INPUT.bytes()).unwrap();
    assert_eq!(Some(Cow::Borrowed(TEST2_EXPECTED)), t2.b);
}

#[test]
fn proto3_test2_simple() {
    use std::io::Read as _;
    let mut t2 = s3::Test2::default();
    t2.deser(TEST2_INPUT.bytes()).unwrap();
    assert_eq!(TEST2_EXPECTED, t2.b);
}

#[test]
fn proto2_test3_simple() {
    use std::io::Read as _;
    let mut t3 = s2::Test3::default();
    t3.deser(TEST3_INPUT.bytes()).unwrap();
    assert!(t3.c.is_some());
    assert_eq!(Some(TEST3_EXPECTED), t3.c.unwrap().a);
}

#[test]
fn proto3_test3_simple() {
    use std::io::Read as _;
    let mut t3 = s3::Test3::default();
    t3.deser(TEST3_INPUT.bytes()).unwrap();
    assert!(t3.c.is_some());
    assert_eq!(TEST3_EXPECTED, t3.c.unwrap().a);
}

#[test]
fn proto2_test4_simple() {
    use std::io::Read as _;
    let mut t4 = s2::Test4::default();
    t4.deser(TEST4_INPUT.bytes()).unwrap();
    assert_eq!(TEST4_EXPECTED, &t4.d);
}

#[test]
fn proto3_test4_simple() {
    use std::io::Read as _;
    let mut t4 = s3::Test4::default();
    t4.deser(TEST4_INPUT.bytes()).unwrap();
    assert_eq!(TEST4_EXPECTED, &t4.d);
}
