#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use ::puroro::apply::ApplyToField;
use ::puroro::tags;
use msg::{SubMsg, SubMsgTag, TheMapEntry, TheMapEntryTag};

pub struct Msg {
    pub the_map: ::std::vec::Vec<self::msg::TheMapEntry>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}
pub struct MsgTag();
impl ::puroro::MessageTag for MsgTag {}
impl ::puroro::IsMessageImplOfTag<MsgTag> for Msg {}

pub mod msg {
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
} // mod msg

struct Visitor();

impl ApplyToField<tags::Repeated, tags::Message<MsgTag>, Vec<Msg>, (), ()> for Visitor {
    fn apply(&mut self, _: &Vec<Msg>, _: usize) -> Result<(), ()> {
        Ok(())
    }
}

impl Msg {
    fn apply_to_fields<F, E>(&self, f: &mut F) -> Result<(), E>
    where
        F: ApplyToField<tags::Repeated, tags::Message<TheMapEntryTag>, Vec<TheMapEntry>, (), E>,
    {
        f.apply(&self.the_map, 1)?;
        Ok(())
    }
}

trait AppliableToMsgFields<E>:
    ApplyToField<tags::Repeated, tags::Message<TheMapEntryTag>, Vec<TheMapEntry>, (), E>
{
}

trait AppliableToTheMapEntryFields<E>:
    ApplyToField<tags::Optional3, tags::Message<SubMsgTag>, Option<SubMsg>, (), E>
    + ApplyToField<tags::Optional3, tags::String, String, (), E>
{
}

trait AppliableToSubMsgFields<E>:
    ApplyToField<tags::Optional3, tags::Message<SubMsgTag>, Option<SubMsg>, (), E>
{
}

trait RecursivelyAppliableToSubMsgFields<E>: RecursivelyAppliableToSubMsgFields<E> {}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
