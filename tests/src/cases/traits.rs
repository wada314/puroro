use crate::tests_pb::full_coverage3::msg::{Submsg, SubmsgTrait};
use crate::tests_pb::full_coverage3::{Enum, Msg, MsgTrait};

#[test]
fn test_read_trait() {
    let mut msg: Msg = Msg::default();
    msg.i32_unlabeled = 10;
    assert_eq!(10, msg.i32_unlabeled());
}
