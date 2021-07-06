use crate::tests_pb::proto3_defaults::Msg;
use ::puroro::DeserFromBytesIter;

const INPUT_FIELD1_I32_ZERO: &[u8] = &[0x08, 0x00];

#[test]
fn test_i32_scalar() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    msg.i32_scalar = 10;
    assert_eq!(10, msg.i32_scalar);
    // Deser 0 into the field, but it is a default value so it should be ignored
    msg.deser(INPUT_FIELD1_I32_ZERO.bytes()).unwrap();
    assert_eq!(10, msg.i32_scalar);
}
