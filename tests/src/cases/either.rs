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

#[test]
fn test_get_i32_unlabeled_field() {
    let val_0 = Msg {
        i32_unlabeled: 0,
        ..Default::default()
    };
    let val_3 = Msg {
        i32_unlabeled: 3,
        ..Default::default()
    };
    assert_eq!(0, Either::<&Msg, &Msg>::Left(&val_0).i32_unlabeled());
    assert_eq!(0, Either::<&Msg, &Msg>::Right(&val_0).i32_unlabeled());
    assert_eq!(3, Either::<&Msg, &Msg>::Left(&val_3).i32_unlabeled());
    assert_eq!(3, Either::<&Msg, &Msg>::Right(&val_3).i32_unlabeled());
}

#[test]
fn test_get_i32_repeated_field() {
    let empty = Msg {
        i32_repeated: [].iter().cloned().collect(),
        ..Default::default()
    };
    let val_1_2_5 = Msg {
        i32_repeated: [1, 2, 5].iter().cloned().collect(),
        ..Default::default()
    };
    assert_eq!(
        Either::<&Msg, &Msg>::Left(&empty)
            .i32_repeated()
            .into_iter()
            .collect::<Vec<_>>(),
        [],
    );
    assert_eq!(
        Either::<&Msg, &Msg>::Right(&empty)
            .i32_repeated()
            .into_iter()
            .collect::<Vec<_>>(),
        []
    );
    assert_eq!(
        Either::<&Msg, &Msg>::Left(&val_1_2_5)
            .i32_repeated()
            .into_iter()
            .collect::<Vec<_>>(),
        [1, 2, 5]
    );
    assert_eq!(
        Either::<&Msg, &Msg>::Right(&val_1_2_5)
            .i32_repeated()
            .into_iter()
            .collect::<Vec<_>>(),
        [1, 2, 5]
    );
}
