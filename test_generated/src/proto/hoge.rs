#![allow(unused_variables)]
#![allow(unused_imports)]

pub trait MsgTrait {
    type TheMapEntryType: self::msg::TheMapEntryTrait;
    type SubMsgType: self::msg::SubMsgTrait;
    #[cfg(feature = "puroro-nightly")]
    type TheMapIter<'a>: ::std::iter::Iterator<Item=&'a Self::TheMapEntryType>
        where Self: 'a, Self::TheMapEntryType: 'a;
    fn the_map(&self) -> &::std::collections::HashMap<str, Self::SubMsgType>;
}

#[derive(Debug)]
pub struct Msg {
    pub the_map: ::std::collections::HashMap<::std::string::String, ::std::option::Option<::std::boxed::Box<self::msg::SubMsg>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl Msg {
    pub fn new() -> Self {
        Self {
            the_map: ::puroro_internal::helpers::FieldNew::new(),
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
            the_map: <::std::collections::HashMap<::std::string::String, ::std::option::Option<::std::boxed::Box<self::msg::SubMsg>>> as FieldClone>::clone(&self.the_map),
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
        let puroro_internal = &self.puroro_internal;
        match field_number {
            8 => {
                <::std::collections::HashMap<::std::string::String, ::std::option::Option<::std::boxed::Box<self::msg::SubMsg>>> as FieldDeserFromIter<
                    tags::Message<self::msg::TheMapEntry>, 
                    tags::Repeated>>
                ::deser(&mut self.the_map, field, ::std::default::Default::default)?;
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
        <::std::collections::HashMap<::std::string::String, ::std::option::Option<::std::boxed::Box<self::msg::SubMsg>>> as FieldSer<
                tags::Message<self::msg::TheMapEntry>, 
                tags::Repeated>>
            ::ser(&self.the_map, serializer, 8)?;
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
    type TheMapEntryType = self::msg::TheMapEntry;
    type SubMsgType = self::msg::SubMsg;
    #[cfg(feature = "puroro-nightly")]
    type TheMapIter<'a> where Self: 'a = 
        impl ::std::iter::Iterator<Item = &'a Self::TheMapEntryType>;
    fn the_map(&self) -> &::std::collections::HashMap<str, Self::SubMsgType> {
        &self.the_map
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
    pub the_map: ::std::collections::HashMap<::bumpalo::collections::String<'bump>, ::std::option::Option<::bumpalo::boxed::Box<'bump, self::msg::SubMsgBumpalo<'bump>>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MsgBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            the_map: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
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
            the_map: <::std::collections::HashMap<::bumpalo::collections::String<'bump>, ::std::option::Option<::bumpalo::boxed::Box<'bump, self::msg::SubMsgBumpalo<'bump>>>> as FieldClone>::clone_in_bumpalo(&self.the_map, self.puroro_internal.bumpalo()),
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
        let puroro_internal = &self.puroro_internal;
        match field_number {
            8 => {
                <::std::collections::HashMap<::bumpalo::collections::String<'bump>, ::std::option::Option<::bumpalo::boxed::Box<'bump, self::msg::SubMsgBumpalo<'bump>>>> as FieldDeserFromIter<
                    tags::Message<self::msg::TheMapEntryBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.the_map, field, || self::msg::TheMapEntryBumpalo::new_in(puroro_internal.bumpalo()))?;
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
        <::std::collections::HashMap<::bumpalo::collections::String<'bump>, ::std::option::Option<::bumpalo::boxed::Box<'bump, self::msg::SubMsgBumpalo<'bump>>>> as FieldSer<
                tags::Message<self::msg::TheMapEntryBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.the_map, serializer, 8)?;
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
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MsgTrait for MsgBumpalo<'bump> {
    type TheMapEntryType = self::msg::TheMapEntryBumpalo<'bump>;
    type SubMsgType = self::msg::SubMsgBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type TheMapIter<'a> where Self: 'a = 
        impl ::std::iter::Iterator<Item = &'a Self::TheMapEntryType>;
    fn the_map(&self) -> &::std::collections::HashMap<str, Self::SubMsgType> {
        &self.the_map
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
pub trait TheMapEntryTrait {
    type SubMsgType: self::SubMsgTrait;
    fn key(&self) -> &'_ str;
    fn value(&self) -> ::std::option::Option<&'_ Self::SubMsgType>;
}

#[derive(Debug)]
pub struct TheMapEntry {
    pub key: ::std::string::String,
    pub value: ::std::option::Option<::std::boxed::Box<self::SubMsg>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl TheMapEntry {
    pub fn new() -> Self {
        Self {
            key: ::puroro_internal::helpers::FieldNew::new(),
            value: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for TheMapEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for TheMapEntry {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            key: <::std::string::String as FieldClone>::clone(&self.key),
            value: <::std::option::Option<::std::boxed::Box<self::SubMsg>> as FieldClone>::clone(&self.value),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for TheMapEntry {
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
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::string::String as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional3>>
                ::deser(&mut self.key, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::boxed::Box<self::SubMsg>> as FieldDeserFromIter<
                    tags::Message<self::SubMsg>, 
                    tags::Optional3>>
                ::deser(&mut self.value, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for TheMapEntry {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::helpers::MapEntry for self::TheMapEntry {
    type KeyType = ::std::string::String;
    type ValueType = ::std::option::Option<::std::boxed::Box<self::SubMsg>>;
    fn into_tuple(self) -> (Self::KeyType, Self::ValueType) {
        (self.key, self.value)
    }
    fn ser_kv<T: ::puroro_internal::ser::MessageSerializer>(
        key: &Self::KeyType,
        value: &Self::ValueType,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::string::String as FieldSer<
            tags::String, 
            tags::Optional3>>
            ::ser(key, serializer, 1)?;
        <::std::option::Option<::std::boxed::Box<self::SubMsg>> as FieldSer<
            tags::Message<self::SubMsg>, 
            tags::Optional3>>
            ::ser(value, serializer, 2)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::Serializable for TheMapEntry {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::string::String as FieldSer<
                tags::String, 
                tags::Optional3>>
            ::ser(&self.key, serializer, 1)?;
        <::std::option::Option<::std::boxed::Box<self::SubMsg>> as FieldSer<
                tags::Message<self::SubMsg>, 
                tags::Optional3>>
            ::ser(&self.value, serializer, 2)?;
        Ok(())
    }
}

impl ::puroro::Serializable for TheMapEntry {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl TheMapEntryTrait for TheMapEntry {
    type SubMsgType = self::SubMsg;
    fn key(&self) -> &'_ str {
        self.key.as_ref()
    }
    fn value(&self) -> ::std::option::Option<&'_ Self::SubMsgType> {
        self.value.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for TheMapEntry<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct TheMapEntryBumpalo<'bump> {
    pub key: ::bumpalo::collections::String<'bump>,
    pub value: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::SubMsgBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> TheMapEntryBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            key: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for TheMapEntryBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            key: <::bumpalo::collections::String<'bump> as FieldClone>::clone_in_bumpalo(&self.key, self.puroro_internal.bumpalo()),
            value: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::SubMsgBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.value, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for TheMapEntryBumpalo<'bump> {
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
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::bumpalo::collections::String<'bump> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional3>>
                ::deser(&mut self.key, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::SubMsgBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::SubMsgBumpalo<'bump>>, 
                    tags::Optional3>>
                ::deser(&mut self.value, field, || ::bumpalo::boxed::Box::new_in(self::SubMsgBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for TheMapEntryBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::MapEntry for self::TheMapEntryBumpalo<'bump> {
    type KeyType = ::bumpalo::collections::String<'bump>;
    type ValueType = ::std::option::Option<::bumpalo::boxed::Box<'bump, self::SubMsgBumpalo<'bump>>>;
    fn into_tuple(self) -> (Self::KeyType, Self::ValueType) {
        (self.key, self.value)
    }
    fn ser_kv<T: ::puroro_internal::ser::MessageSerializer>(
        key: &Self::KeyType,
        value: &Self::ValueType,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::String<'bump> as FieldSer<
            tags::String, 
            tags::Optional3>>
            ::ser(key, serializer, 1)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::SubMsgBumpalo<'bump>>> as FieldSer<
            tags::Message<self::SubMsgBumpalo<'bump>>, 
            tags::Optional3>>
            ::ser(value, serializer, 2)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for TheMapEntryBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::String<'bump> as FieldSer<
                tags::String, 
                tags::Optional3>>
            ::ser(&self.key, serializer, 1)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::SubMsgBumpalo<'bump>>> as FieldSer<
                tags::Message<self::SubMsgBumpalo<'bump>>, 
                tags::Optional3>>
            ::ser(&self.value, serializer, 2)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for TheMapEntryBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> TheMapEntryTrait for TheMapEntryBumpalo<'bump> {
    type SubMsgType = self::SubMsgBumpalo<'bump>;
    fn key(&self) -> &'_ str {
        self.key.as_ref()
    }
    fn value(&self) -> ::std::option::Option<&'_ Self::SubMsgType> {
        self.value.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for TheMapEntryBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
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
        let puroro_internal = &self.puroro_internal;
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
        let puroro_internal = &self.puroro_internal;
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
} // mod msg
