use ::tests_pb::full_coverage3::{MsgBuilder, MsgTrait};

#[test]
fn test_builder() {
    let builder = MsgBuilder::new();
    let msg = builder.append_i32_unlabeled(10).build();
    assert_eq!(10, msg.i32_unlabeled());
}
