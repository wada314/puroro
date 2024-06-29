//! THIS FILE IS A GENERATED FILE! DO NOT EDIT!
//! Source(s):
//!   person.proto
pub struct PhoneNumberStruct<
    #[cfg(allocator)]
    A: ::std::alloc::Allocator = ::std::alloc::Global,
> {
    pub number: ::std::vec::Vec<::puroro::string::String<A>>,
    pub phone_type: ::std::option::Option<
        crate::com::example::tutorial::Person::PhoneType,
    >,
}
impl<#[cfg(allocator)] A: ::std::alloc::Allocator> ::puroro::MessageLite<A>
for self::PhoneNumberStruct<A> {
    fn merge_from_bufread<R: ::std::io::BufRead>(
        &mut self,
        _bufread: &mut R,
    ) -> ::puroro::Result<Self> {
        use ::puroro::Result;
        use ::puroro::internal::deser::{
            DeserMessageHandlerBase, DeserMessageHandlerForRead,
        };
        use ::puroro::variant::Variant;
        impl DeserMessageHandlerBase for Self {
            fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
                todo!()
            }
            fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
                todo!()
            }
            fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
                todo!()
            }
            fn is_message_field(&self, num: i32) -> bool {
                todo!()
            }
            fn start_message(&mut self, num: i32) -> Result<()> {
                todo!()
            }
            fn end_message(&mut self) -> Result<()> {
                todo!()
            }
        }
        impl<R: ::std::io::Read> DeserMessageHandlerForRead<R> for Self {
            fn parse_len(&mut self, num: i32, read: &mut R) -> Result<usize> {
                todo!()
            }
        }
        todo!()
    }
    fn write<W: ::std::io::Write>(&self, _write: &mut W) -> Result<usize> {
        unimplemented!()
    }
}
pub enum PhoneType {
    Mobile = 0i32,
    Home = 1i32,
    Work = 2i32,
}
impl ::std::convert::TryFrom<i32> for self::PhoneType {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, Self::Error> {
        match value {
            0i32 => Ok(Self::Mobile),
            1i32 => Ok(Self::Home),
            2i32 => Ok(Self::Work),
            _ => Err(value),
        }
    }
}
impl ::std::convert::From<self::PhoneType> for i32 {
    fn from(value: self::PhoneType) -> i32 {
        match value {
            Self::Mobile => 0i32,
            Self::Home => 1i32,
            Self::Work => 2i32,
        }
    }
}
