#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use ::puroro::apply::ApplyToField;
use ::puroro::tags;

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

impl<MTag, MType> ApplyToField<tags::Repeated, tags::Message<MTag>, Vec<MType>, (), ()> for Visitor
where
    MTag: ::puroro::MessageTag,
    MType: ::puroro::IsMessageImplOfTag<MTag>,
{
    fn apply(&mut self, field: &Vec<MType>, _: usize) -> Result<(), ()> {
        for item in field {}
        Ok(())
    }
}

impl Msg {
    fn apply_to_fields<F, E>(&self, f: &mut F) -> Result<(), E>
    where
        F: AppliableToMsgFields<E>,
    {
        f.apply(&self.the_map, 1)?;
        Ok(())
    }
}

impl TheMapEntry {
    fn apply_to_fields<F, E>(&self, f: &mut F) -> Result<(), E>
    where
        F: AppliableToTheMapEntryFields<E>,
    {
        f.apply(&self.key, 1)?;
        f.apply(&self.value, 2)?;
        Ok(())
    }
}

trait AppliableToMsgFields<E>:
    ApplyToField<tags::Repeated, tags::Message<TheMapEntryTag>, Vec<TheMapEntry>, (), E>
{
}

trait AppliableToTheMapEntryFields<E>:
    ApplyToField<tags::Optional3, tags::Message<SubMsgTag>, Option<Box<SubMsg>>, (), E>
    + ApplyToField<tags::Optional3, tags::String, String, (), E>
{
}

trait AppliableToSubMsgFields<E>:
    ApplyToField<tags::Optional3, tags::Message<SubMsgTag>, Option<SubMsg>, (), E>
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
}
