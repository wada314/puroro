use ::tests_pb::full_coverage3::{Msg, MsgTrait as _};
use ::tests_pb::oneofs3::{Msg as OneofMsg, MsgTrait as _};

#[test]
fn test_get_i32_optional_field() {
    let none = Msg {
        i32_optional: None,
        ..Default::default()
    };
    let some_3 = Msg {
        i32_optional: Some(3),
        ..Default::default()
    };
    let some_7 = Msg {
        i32_optional: Some(7),
        ..Default::default()
    };
    assert_eq!(None, (&none, &none).i32_optional());
    assert_eq!(Some(3), (&none, &some_3).i32_optional());
    assert_eq!(Some(3), (&some_3, &none).i32_optional());
    assert_eq!(Some(7), (&some_3, &some_7).i32_optional());
}

#[test]
fn test_get_i32_unlabeled_field() {
    let msg_0 = Msg {
        i32_unlabeled: 0,
        ..Default::default()
    };
    let msg_3 = Msg {
        i32_unlabeled: 3,
        ..Default::default()
    };
    let msg_7 = Msg {
        i32_unlabeled: 7,
        ..Default::default()
    };
    assert_eq!(0, (&msg_0, &msg_0).i32_unlabeled());
    assert_eq!(3, (&msg_0, &msg_3).i32_unlabeled());
    assert_eq!(3, (&msg_3, &msg_0).i32_unlabeled());
    assert_eq!(7, (&msg_3, &msg_7).i32_unlabeled());
}
