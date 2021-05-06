#![allow(unused_variables)]
#![allow(unused_imports)]

pub trait CodeGeneratorResponseTrait {
    type File_Type: self::code_generator_response::FileTrait;
    fn error(&self) -> ::std::option::Option<&'_ str>;
    fn supported_features(&self) -> ::std::option::Option<u64>;
    type File_RepeatedType: ::puroro::RepeatedField<Self::File_Type>;
    fn file(&self) -> &Self::File_RepeatedType;
}

#[derive(Debug)]
pub struct CodeGeneratorResponse {
    pub error: ::std::option::Option<::std::string::String>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::std::vec::Vec<self::code_generator_response::File>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl CodeGeneratorResponse {
    pub fn new() -> Self {
        Self {
            error: ::puroro_internal::helpers::FieldNew::new(),
            supported_features: ::puroro_internal::helpers::FieldNew::new(),
            file: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for CodeGeneratorResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for CodeGeneratorResponse {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            error: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.error),
            supported_features: <::std::option::Option<u64> as FieldClone>::clone(&self.supported_features),
            file: <::std::vec::Vec<self::code_generator_response::File> as FieldClone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for CodeGeneratorResponse {
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
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.error, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<u64> as FieldDeserFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::deser(&mut self.supported_features, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec<self::code_generator_response::File> as FieldDeserFromIter<
                    tags::Message<self::code_generator_response::File>, 
                    tags::Repeated>>
                ::deser(&mut self.file, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for CodeGeneratorResponse {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::ser::Serializable for CodeGeneratorResponse {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.error, serializer, 1)?;
        <::std::option::Option<u64> as FieldSer<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.supported_features, serializer, 2)?;
        <::std::vec::Vec<self::code_generator_response::File> as FieldSer<
                tags::Message<self::code_generator_response::File>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 15)?;
        Ok(())
    }
}

impl ::puroro::Serializable for CodeGeneratorResponse {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl CodeGeneratorResponseTrait for CodeGeneratorResponse {
    type File_Type = self::code_generator_response::File;
    fn error(&self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    type File_RepeatedType = ::std::vec::Vec<self::code_generator_response::File>;
    fn file(&self) -> &Self::File_RepeatedType {
        &self.file
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for CodeGeneratorResponse<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct CodeGeneratorResponseBumpalo<'bump> {
    pub error: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::bumpalo::collections::Vec<'bump, self::code_generator_response::FileBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorResponseBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            error: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            supported_features: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for CodeGeneratorResponseBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            error: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.error, self.puroro_internal.bumpalo()),
            supported_features: <::std::option::Option<u64> as FieldClone>::clone_in_bumpalo(&self.supported_features, self.puroro_internal.bumpalo()),
            file: <::bumpalo::collections::Vec<'bump, self::code_generator_response::FileBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.file, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for CodeGeneratorResponseBumpalo<'bump> {
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
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.error, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<u64> as FieldDeserFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::deser(&mut self.supported_features, field, ::std::default::Default::default)?;
            }
            15 => {
                <::bumpalo::collections::Vec<'bump, self::code_generator_response::FileBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::code_generator_response::FileBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.file, field, || self::code_generator_response::FileBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for CodeGeneratorResponseBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for CodeGeneratorResponseBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.error, serializer, 1)?;
        <::std::option::Option<u64> as FieldSer<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.supported_features, serializer, 2)?;
        <::bumpalo::collections::Vec<'bump, self::code_generator_response::FileBumpalo<'bump>> as FieldSer<
                tags::Message<self::code_generator_response::FileBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 15)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for CodeGeneratorResponseBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorResponseTrait for CodeGeneratorResponseBumpalo<'bump> {
    type File_Type = self::code_generator_response::FileBumpalo<'bump>;
    fn error(&self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    type File_RepeatedType = ::bumpalo::collections::Vec<'bump, self::code_generator_response::FileBumpalo<'bump>>;
    fn file(&self) -> &Self::File_RepeatedType {
        &self.file
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for CodeGeneratorResponseBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub mod code_generator_response {
#[derive(Debug, Clone)]
pub enum Feature {
    FeatureNone = 0,
    FeatureProto3Optional = 1,
}
impl ::std::convert::TryFrom<i32> for Feature {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            0 => Ok(Self::FeatureNone),
            1 => Ok(Self::FeatureProto3Optional),
            x => Err(x),
        }
    }
}
impl ::std::convert::From<Feature> for i32 {
    fn from(val: Feature) -> i32 {
        val as i32
    }
}
pub trait FileTrait {
    type GeneratedCodeInfo_Type: super::super::GeneratedCodeInfoTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    fn insertion_point(&self) -> ::std::option::Option<&'_ str>;
    fn content(&self) -> ::std::option::Option<&'_ str>;
    fn generated_code_info(&self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfo_Type>;
}

#[derive(Debug)]
pub struct File {
    pub name: ::std::option::Option<::std::string::String>,
    pub insertion_point: ::std::option::Option<::std::string::String>,
    pub content: ::std::option::Option<::std::string::String>,
    pub generated_code_info: ::std::option::Option<::std::boxed::Box<self::super::super::GeneratedCodeInfo>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl File {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            insertion_point: ::puroro_internal::helpers::FieldNew::new(),
            content: ::puroro_internal::helpers::FieldNew::new(),
            generated_code_info: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for File {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for File {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            insertion_point: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.insertion_point),
            content: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.content),
            generated_code_info: <::std::option::Option<::std::boxed::Box<self::super::super::GeneratedCodeInfo>> as FieldClone>::clone(&self.generated_code_info),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for File {
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
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.insertion_point, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.content, field, ::std::default::Default::default)?;
            }
            16 => {
                <::std::option::Option<::std::boxed::Box<self::super::super::GeneratedCodeInfo>> as FieldDeserFromIter<
                    tags::Message<self::super::super::GeneratedCodeInfo>, 
                    tags::Optional2>>
                ::deser(&mut self.generated_code_info, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for File {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::ser::Serializable for File {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.insertion_point, serializer, 2)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.content, serializer, 15)?;
        <::std::option::Option<::std::boxed::Box<self::super::super::GeneratedCodeInfo>> as FieldSer<
                tags::Message<self::super::super::GeneratedCodeInfo>, 
                tags::Optional2>>
            ::ser(&self.generated_code_info, serializer, 16)?;
        Ok(())
    }
}

impl ::puroro::Serializable for File {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileTrait for File {
    type GeneratedCodeInfo_Type = self::super::super::GeneratedCodeInfo;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    fn generated_code_info(&self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfo_Type> {
        self.generated_code_info.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for File<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct FileBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub insertion_point: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub content: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub generated_code_info: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::super::super::GeneratedCodeInfoBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            insertion_point: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            content: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            generated_code_info: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for FileBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            insertion_point: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.insertion_point, self.puroro_internal.bumpalo()),
            content: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.content, self.puroro_internal.bumpalo()),
            generated_code_info: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::super::super::GeneratedCodeInfoBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.generated_code_info, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileBumpalo<'bump> {
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
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.insertion_point, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            15 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.content, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            16 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::super::super::GeneratedCodeInfoBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::super::super::GeneratedCodeInfoBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.generated_code_info, field, || ::bumpalo::boxed::Box::new_in(self::super::super::GeneratedCodeInfoBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for FileBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for FileBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.insertion_point, serializer, 2)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.content, serializer, 15)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::super::super::GeneratedCodeInfoBumpalo<'bump>>> as FieldSer<
                tags::Message<self::super::super::GeneratedCodeInfoBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.generated_code_info, serializer, 16)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileTrait for FileBumpalo<'bump> {
    type GeneratedCodeInfo_Type = self::super::super::GeneratedCodeInfoBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    fn generated_code_info(&self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfo_Type> {
        self.generated_code_info.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for FileBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
} // mod code_generator_response
pub trait CodeGeneratorRequestTrait {
    type FileDescriptorProto_Type: super::FileDescriptorProtoTrait;
    type Version_Type: self::VersionTrait;
    type FileToGenerate_RepeatedType: ::puroro::RepeatedField<str>;
    fn file_to_generate(&self) -> &Self::FileToGenerate_RepeatedType;
    fn parameter(&self) -> ::std::option::Option<&'_ str>;
    type ProtoFile_RepeatedType: ::puroro::RepeatedField<Self::FileDescriptorProto_Type>;
    fn proto_file(&self) -> &Self::ProtoFile_RepeatedType;
    fn compiler_version(&self) -> ::std::option::Option<&'_ Self::Version_Type>;
}

#[derive(Debug)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: ::std::vec::Vec<::std::string::String>,
    pub parameter: ::std::option::Option<::std::string::String>,
    pub proto_file: ::std::vec::Vec<self::super::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option<::std::boxed::Box<self::Version>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl CodeGeneratorRequest {
    pub fn new() -> Self {
        Self {
            file_to_generate: ::puroro_internal::helpers::FieldNew::new(),
            parameter: ::puroro_internal::helpers::FieldNew::new(),
            proto_file: ::puroro_internal::helpers::FieldNew::new(),
            compiler_version: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for CodeGeneratorRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for CodeGeneratorRequest {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file_to_generate: <::std::vec::Vec<::std::string::String> as FieldClone>::clone(&self.file_to_generate),
            parameter: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.parameter),
            proto_file: <::std::vec::Vec<self::super::FileDescriptorProto> as FieldClone>::clone(&self.proto_file),
            compiler_version: <::std::option::Option<::std::boxed::Box<self::Version>> as FieldClone>::clone(&self.compiler_version),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for CodeGeneratorRequest {
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
                <::std::vec::Vec<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.file_to_generate, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.parameter, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec<self::super::FileDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::super::FileDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.proto_file, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<self::Version>> as FieldDeserFromIter<
                    tags::Message<self::Version>, 
                    tags::Optional2>>
                ::deser(&mut self.compiler_version, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for CodeGeneratorRequest {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::ser::Serializable for CodeGeneratorRequest {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<::std::string::String> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.file_to_generate, serializer, 1)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.parameter, serializer, 2)?;
        <::std::vec::Vec<self::super::FileDescriptorProto> as FieldSer<
                tags::Message<self::super::FileDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.proto_file, serializer, 15)?;
        <::std::option::Option<::std::boxed::Box<self::Version>> as FieldSer<
                tags::Message<self::Version>, 
                tags::Optional2>>
            ::ser(&self.compiler_version, serializer, 3)?;
        Ok(())
    }
}

impl ::puroro::Serializable for CodeGeneratorRequest {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl CodeGeneratorRequestTrait for CodeGeneratorRequest {
    type FileDescriptorProto_Type = self::super::FileDescriptorProto;
    type Version_Type = self::Version;
    type FileToGenerate_RepeatedType = ::std::vec::Vec<::std::string::String>;
    fn file_to_generate(&self) -> &Self::FileToGenerate_RepeatedType {
        &self.file_to_generate
    }
    fn parameter(&self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    type ProtoFile_RepeatedType = ::std::vec::Vec<self::super::FileDescriptorProto>;
    fn proto_file(&self) -> &Self::ProtoFile_RepeatedType {
        &self.proto_file
    }
    fn compiler_version(&self) -> ::std::option::Option<&'_ Self::Version_Type> {
        self.compiler_version.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for CodeGeneratorRequest<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct CodeGeneratorRequestBumpalo<'bump> {
    pub file_to_generate: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    pub parameter: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub proto_file: ::bumpalo::collections::Vec<'bump, self::super::FileDescriptorProtoBumpalo<'bump>>,
    pub compiler_version: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::VersionBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorRequestBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            file_to_generate: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            parameter: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            proto_file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            compiler_version: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for CodeGeneratorRequestBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file_to_generate: <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.file_to_generate, self.puroro_internal.bumpalo()),
            parameter: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.parameter, self.puroro_internal.bumpalo()),
            proto_file: <::bumpalo::collections::Vec<'bump, self::super::FileDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.proto_file, self.puroro_internal.bumpalo()),
            compiler_version: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::VersionBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.compiler_version, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for CodeGeneratorRequestBumpalo<'bump> {
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
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.file_to_generate, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.parameter, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            15 => {
                <::bumpalo::collections::Vec<'bump, self::super::FileDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::super::FileDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.proto_file, field, || self::super::FileDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::VersionBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::VersionBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.compiler_version, field, || ::bumpalo::boxed::Box::new_in(self::VersionBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for CodeGeneratorRequestBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for CodeGeneratorRequestBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.file_to_generate, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.parameter, serializer, 2)?;
        <::bumpalo::collections::Vec<'bump, self::super::FileDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::super::FileDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.proto_file, serializer, 15)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::VersionBumpalo<'bump>>> as FieldSer<
                tags::Message<self::VersionBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.compiler_version, serializer, 3)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for CodeGeneratorRequestBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorRequestTrait for CodeGeneratorRequestBumpalo<'bump> {
    type FileDescriptorProto_Type = self::super::FileDescriptorProtoBumpalo<'bump>;
    type Version_Type = self::VersionBumpalo<'bump>;
    type FileToGenerate_RepeatedType = ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>;
    fn file_to_generate(&self) -> &Self::FileToGenerate_RepeatedType {
        &self.file_to_generate
    }
    fn parameter(&self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    type ProtoFile_RepeatedType = ::bumpalo::collections::Vec<'bump, self::super::FileDescriptorProtoBumpalo<'bump>>;
    fn proto_file(&self) -> &Self::ProtoFile_RepeatedType {
        &self.proto_file
    }
    fn compiler_version(&self) -> ::std::option::Option<&'_ Self::Version_Type> {
        self.compiler_version.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for CodeGeneratorRequestBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait VersionTrait {
    fn major(&self) -> ::std::option::Option<i32>;
    fn minor(&self) -> ::std::option::Option<i32>;
    fn patch(&self) -> ::std::option::Option<i32>;
    fn suffix(&self) -> ::std::option::Option<&'_ str>;
}

#[derive(Debug)]
pub struct Version {
    pub major: ::std::option::Option<i32>,
    pub minor: ::std::option::Option<i32>,
    pub patch: ::std::option::Option<i32>,
    pub suffix: ::std::option::Option<::std::string::String>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: ::puroro_internal::helpers::FieldNew::new(),
            minor: ::puroro_internal::helpers::FieldNew::new(),
            patch: ::puroro_internal::helpers::FieldNew::new(),
            suffix: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for Version {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            major: <::std::option::Option<i32> as FieldClone>::clone(&self.major),
            minor: <::std::option::Option<i32> as FieldClone>::clone(&self.minor),
            patch: <::std::option::Option<i32> as FieldClone>::clone(&self.patch),
            suffix: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.suffix),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for Version {
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
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.major, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.minor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.patch, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.suffix, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}

impl ::puroro::DeserializableFromIter for Version {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::ser::Serializable for Version {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.major, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.minor, serializer, 2)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.patch, serializer, 3)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.suffix, serializer, 4)?;
        Ok(())
    }
}

impl ::puroro::Serializable for Version {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl VersionTrait for Version {
    fn major(&self) -> ::std::option::Option<i32> {
        self.major.clone()
    }
    fn minor(&self) -> ::std::option::Option<i32> {
        self.minor.clone()
    }
    fn patch(&self) -> ::std::option::Option<i32> {
        self.patch.clone()
    }
    fn suffix(&self) -> ::std::option::Option<&'_ str> {
        self.suffix.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Version<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct VersionBumpalo<'bump> {
    pub major: ::std::option::Option<i32>,
    pub minor: ::std::option::Option<i32>,
    pub patch: ::std::option::Option<i32>,
    pub suffix: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> VersionBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            major: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            minor: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            patch: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            suffix: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for VersionBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            major: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.major, self.puroro_internal.bumpalo()),
            minor: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.minor, self.puroro_internal.bumpalo()),
            patch: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.patch, self.puroro_internal.bumpalo()),
            suffix: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.suffix, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for VersionBumpalo<'bump> {
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
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.major, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.minor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.patch, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.suffix, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for VersionBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::Serializable for VersionBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.major, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.minor, serializer, 2)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.patch, serializer, 3)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.suffix, serializer, 4)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for VersionBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> VersionTrait for VersionBumpalo<'bump> {
    fn major(&self) -> ::std::option::Option<i32> {
        self.major.clone()
    }
    fn minor(&self) -> ::std::option::Option<i32> {
        self.minor.clone()
    }
    fn patch(&self) -> ::std::option::Option<i32> {
        self.patch.clone()
    }
    fn suffix(&self) -> ::std::option::Option<&'_ str> {
        self.suffix.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for VersionBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
