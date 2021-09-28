use ::itertools::Itertools;

#[test]
fn test_builder() {
    use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait};
    let msg = MsgBuilder::new()
        .append_i32_unlabeled(10)
        .append_string_optional(Some("Test"))
        .build();
    assert_eq!(10, msg.i32_unlabeled());
    assert_eq!(Some("Test"), msg.string_optional().as_deref());
    assert_eq!(0, msg.i64_unlabeled());
    assert!(::std::mem::size_of_val(&msg) < ::std::mem::size_of::<Msg>());
    dbg!(::std::mem::size_of_val(&msg));
    dbg!(::std::mem::size_of::<Msg>());
}

#[test]
fn test_builder_append_repeated() {
    use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait};
    let msg = MsgBuilder::new()
        .append_i32_repeated(vec![1, 1])
        .append_i32_repeated(vec![])
        .append_i32_repeated(vec![2, 3, 5])
        .build();
    assert_eq!(
        vec![1, 1, 2, 3, 5],
        msg.i32_repeated().into_iter().collect_vec()
    );
}

#[test]
fn test_oneof_field_builder() {
    use ::std::ops::Deref;
    use ::tests_pb::oneofs3::{Msg, MsgBuilder, MsgTrait};
    let msg1 = MsgBuilder::new().append_g1_int32(10).build();
    assert_eq!(Some(10), msg1.g1_int32());
    assert_eq!(None, msg1.g1_string());

    let msg2 = MsgBuilder::new().append_g1_string("test").build();
    assert_eq!(None, msg2.g1_int32());
    assert_eq!(Some("test"), msg2.g1_string().as_ref().map(|v| v.deref()));

    let msg3 = MsgBuilder::new()
        .append_g1_string("test")
        .append_g1_int32(10)
        .build();
    assert_eq!(Some(10), msg3.g1_int32());
    assert_eq!(None, msg3.g1_string());
}
