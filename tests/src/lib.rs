#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use ::puroro::apply::FieldVisitor;
use ::puroro::tags;

use ::sample_pb;

pub struct Msg {
    pub the_map: ::std::vec::Vec<self::TheMapEntry>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}
pub struct MsgTag();
impl ::puroro::MessageTag for MsgTag {}
impl ::puroro::IsMessageImplOfTag<MsgTag> for Msg {}

#[derive(Debug)]
pub struct TheMapEntry {
    pub key: ::std::string::String,
    pub value: ::std::option::Option<::std::boxed::Box<self::SubMsg>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}
pub struct TheMapEntryTag();
impl ::puroro::MessageTag for TheMapEntryTag {}
impl ::puroro::IsMessageImplOfTag<TheMapEntryTag> for TheMapEntry {}

#[derive(Debug)]
pub struct SubMsg {
    pub sub_msg: ::std::option::Option<::std::boxed::Box<self::SubMsg>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}
pub struct SubMsgTag();
impl ::puroro::MessageTag for SubMsgTag {}
impl ::puroro::IsMessageImplOfTag<SubMsgTag> for SubMsgTag {}

struct Visitor();

impl<MTag, MType> FieldVisitor<tags::Repeated, tags::Message<MTag>, Vec<MType>, (), ()> for Visitor
where
    MTag: ::puroro::MessageTag,
    MType: ::puroro::IsMessageImplOfTag<MTag> + VisitFieldsAcceptor<Self, ()>,
{
    fn visit(&mut self, field: &Vec<MType>, _: usize) -> Result<(), ()> {
        for item in field {
            <MType as VisitFieldsAcceptor<Self, ()>>::visit_fields(item, self)?;
        }
        Ok(())
    }
}

trait VisitFieldsAcceptor<F, E> {
    fn visit_fields(&self, f: &mut F) -> Result<(), E>;
}

impl<F, E> VisitFieldsAcceptor<F, E> for Msg
where
    F: AppliableToMsgFields<E>,
{
    fn visit_fields(&self, f: &mut F) -> Result<(), E> {
        f.visit(&self.the_map, 1)?;
        Ok(())
    }
}

impl<F, E> VisitFieldsAcceptor<F, E> for TheMapEntry
where
    F: AppliableToTheMapEntryFields<E>,
{
    fn visit_fields(&self, f: &mut F) -> Result<(), E> {
        f.visit(&self.key, 1)?;
        f.visit(&self.value, 2)?;
        Ok(())
    }
}

impl<F, E> VisitFieldsAcceptor<F, E> for SubMsg
where
    F: AppliableToSubMsgFields<E>,
{
    fn visit_fields(&self, f: &mut F) -> Result<(), E> {
        f.visit(&self.sub_msg, 1)?;
        Ok(())
    }
}

trait AppliableToMsgFields<E>:
    FieldVisitor<tags::Repeated, tags::Message<TheMapEntryTag>, Vec<TheMapEntry>, (), E>
{
}

trait AppliableToTheMapEntryFields<E>:
    FieldVisitor<tags::Optional3, tags::Message<SubMsgTag>, Option<Box<SubMsg>>, (), E>
    + FieldVisitor<tags::Optional3, tags::String, String, (), E>
{
}

trait AppliableToSubMsgFields<E>:
    FieldVisitor<tags::Optional3, tags::Message<SubMsgTag>, Option<Box<SubMsg>>, (), E>
{
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test1() {
        use sample_pb::traits::sample2::Test1Trait;
        use std::convert::TryFrom;
        let slice = [0x08, 0x96, 0x01];

        let test1_simple = <sample_pb::simple::sample2::Test1 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert_eq!(Some(150), test1_simple.a());

        let test1_slice_view =
            sample_pb::slice_view::sample2::Test1::<_>::try_from(&slice as &[u8]).unwrap();
        assert_eq!(Some(150), test1_slice_view.a());
    }

    #[test]
    fn test2() {
        use sample_pb::traits::sample2::Test2Trait;
        use std::convert::TryFrom;
        let slice = [0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];

        let test2_simple = <sample_pb::simple::sample2::Test2 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert_eq!(Some("testing"), test2_simple.b().as_deref());

        let test2_slice_view =
            sample_pb::slice_view::sample2::Test2::<_>::try_from(&slice as &[u8]).unwrap();
        assert_eq!(Some("testing"), test2_slice_view.b().as_deref());
    }

    #[test]
    fn test3() {
        use sample_pb::traits::sample2::{Test1Trait, Test3Trait};
        use std::convert::TryFrom;
        let slice = [0x1a, 0x03, 0x08, 0x96, 0x01];

        let test3_simple = <sample_pb::simple::sample2::Test3 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert!(test3_simple.c().is_some());
        let test1_simple = test3_simple.c().unwrap();
        assert_eq!(Some(150), test1_simple.a());

        let test3_slice_view =
            sample_pb::slice_view::sample2::Test3::<_>::try_from(&slice as &[u8]).unwrap();
        assert!(test3_slice_view.c().is_some());
        let test1_slice_view = test3_slice_view.c().unwrap();
        assert_eq!(Some(150), test1_slice_view.a());
    }

    #[test]
    fn test4() {
        use ::puroro::RepeatedField;
        use sample_pb::traits::sample2::Test4Trait;
        use std::convert::TryFrom;
        let slice = [0x22, 0x06, 0x03, 0x8E, 0x02, 0x9E, 0xA7, 0x05];

        let test4_simple = <sample_pb::simple::sample2::Test4 as ::puroro::DeserializableFromSlice>::deser_from_slice(
            &slice,
        )
        .unwrap();
        assert_eq!(
            // WTF!
            <&Vec<i32> as ::puroro::RepeatedField<::puroro::tags::Int32, i32>>::iter(
                &test4_simple.d()
            )
            .collect::<Vec<_>>(),
            vec![3, 270, 86942]
        );

        let test4_slice_view =
            sample_pb::slice_view::sample2::Test4::<_>::try_from(&slice as &[u8]).unwrap();
        assert_eq!(
            test4_slice_view.d().iter().collect::<Vec<_>>(),
            vec![3, 270, 86942]
        );
    }
}
