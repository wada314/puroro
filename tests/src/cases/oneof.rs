use ::std::borrow::Cow;
use ::tests_pb::oneofs2::msg::{
    GroupOne as GroupOne2, GroupThree as GroupThree2, GroupTwo as GroupTwo2,
};
use ::tests_pb::oneofs2::{Msg as Msg2, MsgTrait as _, Submsg as Submsg2};
use ::tests_pb::oneofs3::msg::{
    GroupOne as GroupOne3, GroupThree as GroupThree3, GroupTwo as GroupTwo3,
};
use ::tests_pb::oneofs3::{Msg as Msg3, MsgTrait as _, Submsg as Submsg3};

#[test]
fn test_oneof_simple2() {
    let mut msg = Msg2::default();
    assert!(msg.group_one.is_none());
    assert!(msg.group_two.is_none());
    assert!(msg.group_three.is_none());
    assert!(msg.group_one().is_none());
    assert!(msg.group_two().is_none());
    assert!(msg.group_three().is_none());
    assert!(msg.g1_int32().is_none());
    assert!(msg.g1_string().is_none());
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_string().is_none());
    assert!(msg.g2_submsg().is_none());
    assert!(msg.g3_int32().is_none());

    msg.group_one = Some(GroupOne2::G1Int32(100));
    assert!(matches!(msg.group_one(), Some(GroupOne2::G1Int32(100))));
    assert!(matches!(msg.g1_int32(), Some(100)));
    assert!(msg.g1_string().is_none());
    msg.group_one = Some(GroupOne2::G1String("Test".to_string()));
    assert!(matches!(msg.group_one(), Some(GroupOne2::G1String("Test"))));
    assert!(matches!(msg.g1_string(), Some("Test")));
    assert!(msg.g1_int32().is_none());
    msg.group_one = None;
    assert!(msg.group_one().is_none());
    assert!(msg.g1_int32().is_none());
    assert!(msg.g1_string().is_none());

    msg.group_two = Some(GroupTwo2::G2F32(100.0));
    assert_eq!(msg.group_two(), Some(GroupTwo2::G2F32(100.0)));
    assert_eq!(msg.g2_f32(), Some(100.0));
    assert!(msg.g2_string().is_none());
    assert!(msg.g2_submsg().is_none());
    msg.group_two = Some(GroupTwo2::G2String("Test".to_string()));
    assert_eq!(msg.group_two(), Some(GroupTwo2::G2String("Test")));
    assert_eq!(msg.g2_string(), Some("Test"));
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_submsg().is_none());
    msg.group_two = Some(GroupTwo2::G2Submsg({
        let mut submsg = Submsg2::default();
        submsg.i32_optional = Some(100);
        Box::new(submsg)
    }));
    assert!(matches!(msg.group_two(), Some(GroupTwo2::G2Submsg(_))));
    assert!(msg.g2_submsg().is_some());
    assert_eq!(msg.g2_submsg().unwrap().i32_optional, Some(100));
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_string().is_none());
    msg.group_two = None;
    assert!(msg.group_two().is_none());
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_string().is_none());
    assert!(msg.g2_submsg().is_none());

    msg.group_three = Some(GroupThree2::G3Int32(100));
    assert!(matches!(msg.group_three(), Some(GroupThree2::G3Int32(100))));
    assert!(matches!(msg.g3_int32(), Some(100)));
    msg.group_three = None;
    assert!(msg.group_three().is_none());
    assert!(msg.g3_int32().is_none());
}

#[test]
fn test_oneof_simple3() {
    let mut msg = Msg3::default();
    assert!(msg.group_one.is_none());
    assert!(msg.group_two.is_none());
    assert!(msg.group_three.is_none());
    assert!(msg.group_one().is_none());
    assert!(msg.group_two().is_none());
    assert!(msg.group_three().is_none());
    assert!(msg.g1_int32().is_none());
    assert!(msg.g1_string().is_none());
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_string().is_none());
    assert!(msg.g2_submsg().is_none());
    assert!(msg.g3_int32().is_none());

    msg.group_one = Some(GroupOne3::G1Int32(100));
    assert!(matches!(msg.group_one(), Some(GroupOne3::G1Int32(100))));
    assert!(matches!(msg.g1_int32(), Some(100)));
    assert!(msg.g1_string().is_none());
    msg.group_one = Some(GroupOne3::G1String("Test".to_string()));
    assert!(matches!(msg.group_one(), Some(GroupOne3::G1String("Test"))));
    assert!(matches!(msg.g1_string(), Some("Test")));
    assert!(msg.g1_int32().is_none());
    msg.group_one = None;
    assert!(msg.group_one().is_none());
    assert!(msg.g1_int32().is_none());
    assert!(msg.g1_string().is_none());

    msg.group_two = Some(GroupTwo3::G2F32(100.0));
    assert_eq!(msg.group_two(), Some(GroupTwo3::G2F32(100.0)));
    assert_eq!(msg.g2_f32(), Some(100.0));
    assert!(msg.g2_string().is_none());
    assert!(msg.g2_submsg().is_none());
    msg.group_two = Some(GroupTwo3::G2String("Test".to_string()));
    assert_eq!(msg.group_two(), Some(GroupTwo3::G2String("Test")));
    assert_eq!(msg.g2_string(), Some("Test"));
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_submsg().is_none());
    msg.group_two = Some(GroupTwo3::G2Submsg({
        let mut submsg = Submsg3::default();
        submsg.i32_unlabeled = 100;
        Box::new(submsg)
    }));
    assert!(matches!(msg.group_two(), Some(GroupTwo3::G2Submsg(_))));
    assert!(msg.g2_submsg().is_some());
    assert_eq!(msg.g2_submsg().unwrap().i32_unlabeled, 100);
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_string().is_none());
    msg.group_two = None;
    assert!(msg.group_two().is_none());
    assert!(msg.g2_f32().is_none());
    assert!(msg.g2_string().is_none());
    assert!(msg.g2_submsg().is_none());

    msg.group_three = Some(GroupThree3::G3Int32(100));
    assert!(matches!(msg.group_three(), Some(GroupThree3::G3Int32(100))));
    assert!(matches!(msg.g3_int32(), Some(100)));
    msg.group_three = None;
    assert!(msg.group_three().is_none());
    assert!(msg.g3_int32().is_none());
}
