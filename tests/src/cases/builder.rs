use ::tests_pb::full_coverage3::{Msg, MsgBuilder, MsgTrait};

#[test]
fn test_builder() {
    let builder = MsgBuilder::new();
    let msg = builder
        .append_i32_unlabeled(10)
        .append_string_optional(Some("Test".into()))
        .build();
    assert_eq!(10, msg.i32_unlabeled());
    assert_eq!(Some("Test"), msg.string_optional().as_deref());
    assert_eq!(0, msg.i64_unlabeled());
    assert!(::std::mem::size_of_val(&msg) < ::std::mem::size_of::<Msg>());
    dbg!(::std::mem::size_of_val(&msg));
    dbg!(::std::mem::size_of::<Msg>());
}
