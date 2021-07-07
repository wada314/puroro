use crate::tests_pb::proto3_defaults::Msg;
use ::puroro::DeserFromBytesIter;

const INPUT_FIELD1_I32_ZERO: &[u8] = &[0x08, 0x00];
const INPUT_FIELD1_I32_ONE: &[u8] = &[0x08, 0x01];
const INPUT_FIELD4_F32_ZERO: &[u8] = &[0x25, 0x00, 0x00, 0x00, 0x00];
const INPUT_FIELD4_F32_ONE: &[u8] = &[0x25, 0x00, 0x00, 0x80, 0x3F]; // 0x3F80_0000 == 1.0f32
const INPUT_FIELD3_I32_ZERO: &[u8] = &[0x18, 0x00];
const INPUT_FIELD3_I32_ONE: &[u8] = &[0x18, 0x01];
const INPUT_FIELD3_I32_PACKED_ZERO_TO_THREE: &[u8] = &[0x1A, 0x04, 0x00, 0x01, 0x02, 0x03];

#[test]
fn test_i32_scalar() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    msg.i32_scalar = 10;
    assert_eq!(10, msg.i32_scalar);

    // Deser 0 into the field, but it is a default value so it should be ignored
    msg.deser(INPUT_FIELD1_I32_ZERO.bytes()).unwrap();
    assert_eq!(10, msg.i32_scalar);

    // Deser 1 into the field. Should overwrite the value.
    msg.deser(INPUT_FIELD1_I32_ONE.bytes()).unwrap();
    assert_eq!(1, msg.i32_scalar);
}

#[test]
fn test_f32_scalar() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    msg.f32_scalar = 10.0;
    assert_eq!(10.0, msg.f32_scalar);

    // Deser 0 into the field, but it is a default value so it should be ignored
    msg.deser(INPUT_FIELD4_F32_ZERO.bytes()).unwrap();
    assert_eq!(10.0, msg.f32_scalar);

    // Deser 1 into the field. Should overwrite the value.
    msg.deser(INPUT_FIELD4_F32_ONE.bytes()).unwrap();
    assert_eq!(1.0, msg.f32_scalar);
}

#[test]
fn test_i32_repeated() {
    use std::io::Read as _;
    let mut msg = <Msg as Default>::default();
    msg.i32_repeated = vec![10, 20];
    assert_eq!(&vec![10, 20], &msg.i32_repeated);

    msg.deser(INPUT_FIELD3_I32_ZERO.bytes()).unwrap();
    assert_eq!(&vec![10, 20, 0], &msg.i32_repeated);

    msg.deser(INPUT_FIELD3_I32_ONE.bytes()).unwrap();
    assert_eq!(&vec![10, 20, 0, 1], &msg.i32_repeated);

    msg.deser(INPUT_FIELD3_I32_PACKED_ZERO_TO_THREE.bytes())
        .unwrap();
    assert_eq!(&vec![10, 20, 0, 1, 0, 1, 2, 3], &msg.i32_repeated);
}
