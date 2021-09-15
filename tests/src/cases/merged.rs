use ::itertools::Itertools;
use ::tests_pb::full_coverage3::msg::{Submsg, SubmsgTrait as _};
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

#[test]
fn test_get_i32_repeated_field() {
    let empty = Msg {
        i32_repeated: vec![],
        ..Default::default()
    };
    let msg_1 = Msg {
        i32_repeated: vec![1],
        ..Default::default()
    };
    let msg_3_5 = Msg {
        i32_repeated: vec![3, 5],
        ..Default::default()
    };
    assert_eq!(
        vec![] as Vec<i32>,
        (&empty, &empty).i32_repeated().into_iter().collect_vec()
    );
    assert_eq!(
        vec![1],
        (&empty, &msg_1).i32_repeated().into_iter().collect_vec()
    );
    assert_eq!(
        vec![1],
        (&msg_1, &empty).i32_repeated().into_iter().collect_vec()
    );
    assert_eq!(
        vec![1, 3, 5],
        (&msg_1, &msg_3_5).i32_repeated().into_iter().collect_vec()
    );
}

#[test]
fn test_get_msg_optional_field() {
    let submsg_3 = Submsg { i32_unlabeled: 3 };
    let none = Msg {
        submsg_optional: None,
        ..Default::default()
    };
    let msg_3 = Msg {
        submsg_optional: Some(Box::new(submsg_3.clone())),
        ..Default::default()
    };
    assert_eq!(None, (&none, &none).submsg_optional());
    assert!((&msg_3, &none).submsg_optional().is_some());
    assert_eq!(
        3,
        (&msg_3, &none).submsg_optional().unwrap().i32_unlabeled()
    );
}
