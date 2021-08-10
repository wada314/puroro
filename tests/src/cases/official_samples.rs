/// The sample cases documented in the Protobuf official encoding document:
/// https://developers.google.com/protocol-buffers/docs/encoding
use ::puroro::DeserFromBytesIter;
use ::std::default::Default;

const TEST1_INPUT: &[u8] = &[0x08, 0x96, 0x01];
const TEST2_INPUT: &[u8] = &[0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
const TEST3_INPUT: &[u8] = &[0x1a, 0x03, 0x08, 0x96, 0x01];
const TEST4_INPUT: &[u8] = &[0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];

const TEST1_EXPECTED: i32 = 150;
const TEST2_EXPECTED: &str = "testing";
const TEST3_EXPECTED: i32 = 150;
const TEST4_EXPECTED: &[i32] = &[3, 270, 86942];

#[test]
fn test1_simple() {
    use std::io::Read as _;
    let mut t1 = <tests_pb::official_samples::Test1 as Default>::default();
    t1.deser(TEST1_INPUT.bytes()).unwrap();
    assert_eq!(Some(TEST1_EXPECTED), t1.a);
}

#[test]
fn test2_simple() {
    use std::io::Read as _;
    use std::ops::Deref as _;
    let mut t2 = <tests_pb::official_samples::Test2 as Default>::default();
    t2.deser(TEST2_INPUT.bytes()).unwrap();
    assert_eq!(Some(TEST2_EXPECTED.into()), t2.b);
}

#[test]
fn test3_simple() {
    use std::io::Read as _;
    use std::ops::Deref as _;
    let mut t3 = <tests_pb::official_samples::Test3 as Default>::default();
    t3.deser(TEST3_INPUT.bytes()).unwrap();
    assert!(t3.c.is_some());
    assert_eq!(Some(TEST3_EXPECTED), t3.c.as_ref().unwrap().a);
}

#[test]
fn test4_simple() {
    use std::io::Read as _;
    let mut t4 = <tests_pb::official_samples::Test4 as Default>::default();
    t4.deser(TEST4_INPUT.bytes()).unwrap();
    assert_eq!(TEST4_EXPECTED, &t4.d);
}
