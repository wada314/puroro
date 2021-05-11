#![allow(unused_variables)]
#![allow(unused_imports)]

pub trait CodeGeneratorResponseTrait {
    type FileType: self::code_generator_response::FileTrait;
    fn error(&self) -> ::std::option::Option<&'_ str>;
    fn supported_features(&self) -> ::std::option::Option<u64>;
    type FileRepeated: ::puroro::RepeatedField<Self::FileType>;
    fn file(&self) -> &Self::FileRepeated;
}

#[derive(Debug)]
pub struct CodeGeneratorResponse {
    pub error: ::std::option::Option<::std::string::String>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::std::vec::Vec<self::code_generator_response::File>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl CodeGeneratorResponse {
    pub fn new() -> Self {
        Self {
            error: ::puroro_internal::helpers::FieldNew::new(),
            supported_features: ::puroro_internal::helpers::FieldNew::new(),
            file: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
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
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
        Ok(true)
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

impl ::puroro::DeserializableFromSlice for CodeGeneratorResponse {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for CodeGeneratorResponse {
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
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl CodeGeneratorResponseTrait for CodeGeneratorResponse {
    type FileType = self::code_generator_response::File;
    fn error(&self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    type FileRepeated = ::std::vec::Vec<self::code_generator_response::File>;
    fn file(&self) -> &Self::FileRepeated {
        &self.file
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for CodeGeneratorResponse {
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
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorResponseBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            error: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            supported_features: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
        Ok(true)
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
impl<'bump> ::puroro::DeserializableFromSlice for CodeGeneratorResponseBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for CodeGeneratorResponseBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorResponseTrait for CodeGeneratorResponseBumpalo<'bump> {
    type FileType = self::code_generator_response::FileBumpalo<'bump>;
    fn error(&self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    type FileRepeated = ::bumpalo::collections::Vec<'bump, self::code_generator_response::FileBumpalo<'bump>>;
    fn file(&self) -> &Self::FileRepeated {
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

#[derive(Debug)]
pub struct CodeGeneratorResponseSliceView<'slice, 'p> {
    error: ::std::option::Option<&'slice str>,
    supported_features: ::std::option::Option<u64>,
    file: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice, 'p> CodeGeneratorResponseSliceView<'slice, 'p> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            error: ::puroro_internal::helpers::FieldNew::new(),
            supported_features: ::puroro_internal::helpers::FieldNew::new(),
            file: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice, 'p> ::std::clone::Clone for CodeGeneratorResponseSliceView<'slice, 'p> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            error: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.error),
            supported_features: <::std::option::Option<u64> as FieldClone>::clone(&self.supported_features),
            file: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, 'p> ::puroro_internal::deser::DeserializableMessageFromSlice for CodeGeneratorResponseSliceView<'slice, 'p> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice, 'p> ::puroro_internal::ser::SerializableMessage for CodeGeneratorResponseSliceView<'slice, 'p> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice, 'p> ::puroro::Serializable for CodeGeneratorResponseSliceView<'slice, 'p> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
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
    type GeneratedCodeInfoType: super::super::GeneratedCodeInfoTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    fn insertion_point(&self) -> ::std::option::Option<&'_ str>;
    fn content(&self) -> ::std::option::Option<&'_ str>;
    fn generated_code_info(&self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfoType>;
}

#[derive(Debug)]
pub struct File {
    pub name: ::std::option::Option<::std::string::String>,
    pub insertion_point: ::std::option::Option<::std::string::String>,
    pub content: ::std::option::Option<::std::string::String>,
    pub generated_code_info: ::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl File {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            insertion_point: ::puroro_internal::helpers::FieldNew::new(),
            content: ::puroro_internal::helpers::FieldNew::new(),
            generated_code_info: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
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
            generated_code_info: <::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>> as FieldClone>::clone(&self.generated_code_info),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for File {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
                <::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>> as FieldDeserFromIter<
                    tags::Message<super::super::GeneratedCodeInfo>, 
                    tags::Optional2>>
                ::deser(&mut self.generated_code_info, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
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

impl ::puroro::DeserializableFromSlice for File {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for File {
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
        <::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>> as FieldSer<
                tags::Message<super::super::GeneratedCodeInfo>, 
                tags::Optional2>>
            ::ser(&self.generated_code_info, serializer, 16)?;
        Ok(())
    }
}

impl ::puroro::Serializable for File {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl FileTrait for File {
    type GeneratedCodeInfoType = super::super::GeneratedCodeInfo;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    fn generated_code_info(&self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfoType> {
        self.generated_code_info.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for File {
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
    pub generated_code_info: ::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            insertion_point: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            content: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            generated_code_info: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
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
            generated_code_info: <::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.generated_code_info, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
                <::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<super::super::GeneratedCodeInfoBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.generated_code_info, field, || ::bumpalo::boxed::Box::new_in(super::super::GeneratedCodeInfoBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
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
impl<'bump> ::puroro::DeserializableFromSlice for FileBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for FileBumpalo<'bump> {
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
        <::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>> as FieldSer<
                tags::Message<super::super::GeneratedCodeInfoBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.generated_code_info, serializer, 16)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileTrait for FileBumpalo<'bump> {
    type GeneratedCodeInfoType = super::super::GeneratedCodeInfoBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    fn generated_code_info(&self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfoType> {
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

#[derive(Debug)]
pub struct FileSliceView<'slice, 'p> {
    name: ::std::option::Option<&'slice str>,
    insertion_point: ::std::option::Option<&'slice str>,
    content: ::std::option::Option<&'slice str>,
    generated_code_info: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice, 'p> FileSliceView<'slice, 'p> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            insertion_point: ::puroro_internal::helpers::FieldNew::new(),
            content: ::puroro_internal::helpers::FieldNew::new(),
            generated_code_info: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice, 'p> ::std::clone::Clone for FileSliceView<'slice, 'p> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            insertion_point: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.insertion_point),
            content: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.content),
            generated_code_info: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.generated_code_info),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, 'p> ::puroro_internal::deser::DeserializableMessageFromSlice for FileSliceView<'slice, 'p> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice, 'p> ::puroro_internal::ser::SerializableMessage for FileSliceView<'slice, 'p> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice, 'p> ::puroro::Serializable for FileSliceView<'slice, 'p> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
} // mod code_generator_response
pub trait CodeGeneratorRequestTrait {
    type FileDescriptorProtoType: super::FileDescriptorProtoTrait;
    type VersionType: self::VersionTrait;
    type FileToGenerateRepeated: ::puroro::RepeatedField<str>;
    fn file_to_generate(&self) -> &Self::FileToGenerateRepeated;
    fn parameter(&self) -> ::std::option::Option<&'_ str>;
    type ProtoFileRepeated: ::puroro::RepeatedField<Self::FileDescriptorProtoType>;
    fn proto_file(&self) -> &Self::ProtoFileRepeated;
    fn compiler_version(&self) -> ::std::option::Option<&'_ Self::VersionType>;
}

#[derive(Debug)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: ::std::vec::Vec<::std::string::String>,
    pub parameter: ::std::option::Option<::std::string::String>,
    pub proto_file: ::std::vec::Vec<super::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option<::std::boxed::Box<self::Version>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl CodeGeneratorRequest {
    pub fn new() -> Self {
        Self {
            file_to_generate: ::puroro_internal::helpers::FieldNew::new(),
            parameter: ::puroro_internal::helpers::FieldNew::new(),
            proto_file: ::puroro_internal::helpers::FieldNew::new(),
            compiler_version: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
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
            proto_file: <::std::vec::Vec<super::FileDescriptorProto> as FieldClone>::clone(&self.proto_file),
            compiler_version: <::std::option::Option<::std::boxed::Box<self::Version>> as FieldClone>::clone(&self.compiler_version),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for CodeGeneratorRequest {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
                <::std::vec::Vec<super::FileDescriptorProto> as FieldDeserFromIter<
                    tags::Message<super::FileDescriptorProto>, 
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
        Ok(true)
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

impl ::puroro::DeserializableFromSlice for CodeGeneratorRequest {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for CodeGeneratorRequest {
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
        <::std::vec::Vec<super::FileDescriptorProto> as FieldSer<
                tags::Message<super::FileDescriptorProto>, 
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
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl CodeGeneratorRequestTrait for CodeGeneratorRequest {
    type FileDescriptorProtoType = super::FileDescriptorProto;
    type VersionType = self::Version;
    type FileToGenerateRepeated = ::std::vec::Vec<::std::string::String>;
    fn file_to_generate(&self) -> &Self::FileToGenerateRepeated {
        &self.file_to_generate
    }
    fn parameter(&self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    type ProtoFileRepeated = ::std::vec::Vec<super::FileDescriptorProto>;
    fn proto_file(&self) -> &Self::ProtoFileRepeated {
        &self.proto_file
    }
    fn compiler_version(&self) -> ::std::option::Option<&'_ Self::VersionType> {
        self.compiler_version.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for CodeGeneratorRequest {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct CodeGeneratorRequestBumpalo<'bump> {
    pub file_to_generate: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    pub parameter: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub proto_file: ::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>>,
    pub compiler_version: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::VersionBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorRequestBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            file_to_generate: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            parameter: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            proto_file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            compiler_version: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
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
            proto_file: <::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.proto_file, self.puroro_internal.bumpalo()),
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
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
                <::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<super::FileDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.proto_file, field, || super::FileDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::VersionBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::VersionBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.compiler_version, field, || ::bumpalo::boxed::Box::new_in(self::VersionBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
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
impl<'bump> ::puroro::DeserializableFromSlice for CodeGeneratorRequestBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for CodeGeneratorRequestBumpalo<'bump> {
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
        <::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<super::FileDescriptorProtoBumpalo<'bump>>, 
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
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorRequestTrait for CodeGeneratorRequestBumpalo<'bump> {
    type FileDescriptorProtoType = super::FileDescriptorProtoBumpalo<'bump>;
    type VersionType = self::VersionBumpalo<'bump>;
    type FileToGenerateRepeated = ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>;
    fn file_to_generate(&self) -> &Self::FileToGenerateRepeated {
        &self.file_to_generate
    }
    fn parameter(&self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    type ProtoFileRepeated = ::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>>;
    fn proto_file(&self) -> &Self::ProtoFileRepeated {
        &self.proto_file
    }
    fn compiler_version(&self) -> ::std::option::Option<&'_ Self::VersionType> {
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

#[derive(Debug)]
pub struct CodeGeneratorRequestSliceView<'slice, 'p> {
    file_to_generate: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    parameter: ::std::option::Option<&'slice str>,
    proto_file: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    compiler_version: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice, 'p> CodeGeneratorRequestSliceView<'slice, 'p> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            file_to_generate: ::puroro_internal::helpers::FieldNew::new(),
            parameter: ::puroro_internal::helpers::FieldNew::new(),
            proto_file: ::puroro_internal::helpers::FieldNew::new(),
            compiler_version: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice, 'p> ::std::clone::Clone for CodeGeneratorRequestSliceView<'slice, 'p> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file_to_generate: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.file_to_generate),
            parameter: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.parameter),
            proto_file: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.proto_file),
            compiler_version: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.compiler_version),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, 'p> ::puroro_internal::deser::DeserializableMessageFromSlice for CodeGeneratorRequestSliceView<'slice, 'p> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice, 'p> ::puroro_internal::ser::SerializableMessage for CodeGeneratorRequestSliceView<'slice, 'p> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice, 'p> ::puroro::Serializable for CodeGeneratorRequestSliceView<'slice, 'p> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
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
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: ::puroro_internal::helpers::FieldNew::new(),
            minor: ::puroro_internal::helpers::FieldNew::new(),
            patch: ::puroro_internal::helpers::FieldNew::new(),
            suffix: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
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
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
        Ok(true)
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

impl ::puroro::DeserializableFromSlice for Version {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for Version {
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
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
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
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Version {
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
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> VersionBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            major: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            minor: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            patch: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            suffix: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
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
        Ok(true)
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
impl<'bump> ::puroro::DeserializableFromSlice for VersionBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for VersionBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
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

#[derive(Debug)]
pub struct VersionSliceView<'slice, 'p> {
    major: ::std::option::Option<i32>,
    minor: ::std::option::Option<i32>,
    patch: ::std::option::Option<i32>,
    suffix: ::std::option::Option<&'slice str>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice, 'p> VersionSliceView<'slice, 'p> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            major: ::puroro_internal::helpers::FieldNew::new(),
            minor: ::puroro_internal::helpers::FieldNew::new(),
            patch: ::puroro_internal::helpers::FieldNew::new(),
            suffix: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice, 'p> ::std::clone::Clone for VersionSliceView<'slice, 'p> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            major: <::std::option::Option<i32> as FieldClone>::clone(&self.major),
            minor: <::std::option::Option<i32> as FieldClone>::clone(&self.minor),
            patch: <::std::option::Option<i32> as FieldClone>::clone(&self.patch),
            suffix: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.suffix),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, 'p> ::puroro_internal::deser::DeserializableMessageFromSlice for VersionSliceView<'slice, 'p> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice, 'p> ::puroro_internal::ser::SerializableMessage for VersionSliceView<'slice, 'p> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice, 'p> ::puroro::Serializable for VersionSliceView<'slice, 'p> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
