#![allow(unused_variables)]
#![allow(unused_imports)]

pub trait MsgTrait {
    type SubMsgType: self::msg::SubMsgTrait;
    #[cfg(feature = "puroro-nightly")]
    type RsubmsgIter<'a>: ::std::iter::Iterator<Item=&'a Self::SubMsgType> where Self::SubMsgType: 'a;
    fn for_each_rsubmsg<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::SubMsgType);
    fn rsubmsg_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::SubMsgType>>;
    #[cfg(feature = "puroro-nightly")]
    fn rsubmsg_iter(&self) -> Self::RsubmsgIter<'_>;
}

#[derive(Debug)]
pub struct Msg {
    pub rsubmsg: ::std::vec::Vec<msg::SubMsg>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl Msg {
    pub fn new() -> Self {
        Self {
            rsubmsg: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for Msg {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for Msg {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            rsubmsg: <::std::vec::Vec<msg::SubMsg> as FieldClone>::clone(&self.rsubmsg),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for Msg {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            6 => {
                <::std::vec::Vec<msg::SubMsg> as FieldDeserFromIter<
                    tags::Message<msg::SubMsg>, 
                    tags::Repeated>>
                ::deser(&mut self.rsubmsg, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for Msg {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::ser::Serializable for Msg {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<msg::SubMsg> as FieldSer<
                tags::Message<msg::SubMsg>, 
                tags::Repeated>>
            ::ser(&self.rsubmsg, serializer, 6)?;
        Ok(())
    }
}

impl ::puroro::Serializable for Msg {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl MsgTrait for Msg {
    type SubMsgType = msg::SubMsg;
    #[cfg(feature = "puroro-nightly")]
    type RsubmsgIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::SubMsgType>;
    fn for_each_rsubmsg<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::SubMsgType) {
        for item in (self.rsubmsg).iter() {
            (f)(item);
        }
    }
    fn rsubmsg_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::SubMsgType>> {
        ::std::boxed::Box::new(self.rsubmsg.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    fn rsubmsg_iter(&self) -> Self::RsubmsgIter<'_> {
        self.rsubmsg.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Msg<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct MsgBumpalo<'bump> {
    pub rsubmsg: ::bumpalo::collections::Vec<'bump, msg::SubMsgBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MsgBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            rsubmsg: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for MsgBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            rsubmsg: <::bumpalo::collections::Vec<'bump, msg::SubMsgBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.rsubmsg, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for MsgBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let bumpalo = self.puroro_internal.bumpalo();
        match field_number {
            6 => {
                <::bumpalo::collections::Vec<'bump, msg::SubMsgBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<msg::SubMsgBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.rsubmsg, field, || msg::SubMsgBumpalo::new_in(bumpalo))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for MsgBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for MsgBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, msg::SubMsgBumpalo<'bump>> as FieldSer<
                tags::Message<msg::SubMsgBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.rsubmsg, serializer, 6)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MsgBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
trait Hoge {
    type RsubmsgType<'a>;
    type RsubmsgIter<'a>: ::std::iter::Iterator<Item = &'a Self::RsubmsgType<'a>> 
    where Self::RsubmsgType<'a>: 'a, Self: 'a;
    fn rsubmsg_iter(&self) -> Self::RsubmsgIter<'_>;
}
impl<'bump> Hoge for MsgBumpalo<'bump> {
    type RsubmsgType<'a> = msg::SubMsgBumpalo<'bump>;
    type RsubmsgIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::RsubmsgType<'a>>;
    fn rsubmsg_iter(&self) -> Self::RsubmsgIter<'_> {
        self.rsubmsg.iter()
    }
}

#[cfg(feature = "puroro-bumpalo2")]
impl<'bump> MsgTrait for MsgBumpalo<'bump> {
    type SubMsgType = msg::SubMsgBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type RsubmsgIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::SubMsgType>;
    fn for_each_rsubmsg<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::SubMsgType) {
        for item in (self.rsubmsg).iter() {
            (f)(item);
        }
    }
    fn rsubmsg_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::SubMsgType>> {
        ::std::boxed::Box::new(self.rsubmsg.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    fn rsubmsg_iter(&self) -> Self::RsubmsgIter<'_> {
        self.rsubmsg.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for MsgBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub mod msg {
pub trait SubMsgTrait {
}

#[derive(Debug)]
pub struct SubMsg {
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl SubMsg {
    pub fn new() -> Self {
        Self {
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for SubMsg {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for SubMsg {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for SubMsg {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for SubMsg {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::ser::Serializable for SubMsg {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        Ok(())
    }
}

impl ::puroro::Serializable for SubMsg {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl SubMsgTrait for SubMsg {
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for SubMsg<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct SubMsgBumpalo<'bump> {
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SubMsgBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for SubMsgBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for SubMsgBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for SubMsgBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for SubMsgBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for SubMsgBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SubMsgTrait for SubMsgBumpalo<'bump> {
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for SubMsgBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub mod sub_msg {
pub trait SubSubMsgTrait {
    fn ival(&'_ self) -> i64;
}

#[derive(Debug)]
pub struct SubSubMsg {
    pub ival: i64,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl SubSubMsg {
    pub fn new() -> Self {
        Self {
            ival: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for SubSubMsg {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for SubSubMsg {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            ival: <i64 as FieldClone>::clone(&self.ival),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for SubSubMsg {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <i64 as FieldDeserFromIter<
                    tags::Int64, 
                    tags::Optional3>>
                ::deser(&mut self.ival, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for SubSubMsg {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::ser::Serializable for SubSubMsg {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <i64 as FieldSer<
                tags::Int64, 
                tags::Optional3>>
            ::ser(&self.ival, serializer, 1)?;
        Ok(())
    }
}

impl ::puroro::Serializable for SubSubMsg {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl SubSubMsgTrait for SubSubMsg {
    fn ival(&'_ self) -> i64 {
        self.ival.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for SubSubMsg<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct SubSubMsgBumpalo<'bump> {
    pub ival: i64,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SubSubMsgBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            ival: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for SubSubMsgBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            ival: <i64 as FieldClone>::clone_in_bumpalo(&self.ival, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for SubSubMsgBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <i64 as FieldDeserFromIter<
                    tags::Int64, 
                    tags::Optional3>>
                ::deser(&mut self.ival, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for SubSubMsgBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for SubSubMsgBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <i64 as FieldSer<
                tags::Int64, 
                tags::Optional3>>
            ::ser(&self.ival, serializer, 1)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for SubSubMsgBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SubSubMsgTrait for SubSubMsgBumpalo<'bump> {
    fn ival(&'_ self) -> i64 {
        self.ival.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for SubSubMsgBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
} // mod sub_msg
} // mod msg
