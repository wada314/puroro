//! THIS FILE IS A GENERATED FILE! DO NOT EDIT!
//! Source(s):
//!   p2-example.proto
pub struct Test1Struct<
    #[cfg(allocator)]
    A: ::std::alloc::Allocator = ::std::alloc::Global,
> {
    pub a: ::std::option::Option<i32>,
}
impl<#[cfg(allocator)] A: ::std::alloc::Allocator> ::puroro::MessageLite<A>
for self::Test1Struct<A> {
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
