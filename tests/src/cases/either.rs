use ::puroro::{Either, Message};
use ::tests_pb::full_coverage3::{Msg, MsgTrait};

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
    assert_eq!(None, Either::<&Msg, &Msg>::Left(&none).i32_optional());
    assert_eq!(None, Either::<&Msg, &Msg>::Right(&none).i32_optional());
    assert_eq!(Some(3), Either::<&Msg, &Msg>::Left(&some_3).i32_optional());
    assert_eq!(Some(3), Either::<&Msg, &Msg>::Right(&some_3).i32_optional());
}
