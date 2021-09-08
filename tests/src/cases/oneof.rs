use ::tests_pb::oneofs2::msg::{
    GroupOne as GroupOne2, GroupThree as GroupThree2, GroupTwo as GroupTwo2,
};
use ::tests_pb::oneofs2::{Msg as Msg2, MsgTrait as _};
use ::tests_pb::oneofs3::{Msg as Msg3, MsgTrait as _};

#[test]
fn test_oneof_simple2() {
    let mut msg = Msg2::default();
    assert!(msg.group_one.is_none());
    assert!(msg.group_two.is_none());
    assert!(msg.group_three.is_none());
    assert!(msg.group_one().is_none());
    assert!(msg.group_two().is_none());
    assert!(msg.group_three().is_none());
    msg.group_one = Some(GroupOne2::G1Int32(100));
    assert!(matches!(msg.group_one(), Some(GroupOne2::G1Int32(100))));
    assert!(matches!(msg.g1_int32(), Some(100)));
    assert!(msg.g1_string().is_none());
    msg.group_one = Some(GroupOne2::G1String("Test".into()));
    assert!(matches!(msg.group_one(), Some(GroupOne2::G1String("Test"))));
    assert!(matches!(msg.g1_string(), Some("Test")));
    assert!(msg.g1_int32().is_none());
}
