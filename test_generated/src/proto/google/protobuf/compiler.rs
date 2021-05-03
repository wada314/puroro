#![allow(unused_variables)]
#![allow(unused_imports)]

pub trait CodeGeneratorResponseTrait {
    type FileType: self::code_generator_response::FileTrait;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a>: ::std::iter::Iterator<Item=&'a self::code_generator_response::File>;
    fn error(&'_ self) -> ::std::option::Option<&'_ str>;
    fn supported_features(&'_ self) -> ::std::option::Option<u64>;
    fn for_each_file<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::FileType);
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileType>>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_>;
}

#[derive(Debug, Clone)]
pub struct CodeGeneratorResponse {
    pub error: ::std::option::Option<::std::string::String>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::std::vec::Vec<code_generator_response::File>,
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.error, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<u64> as DeserializableFieldFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::deser(&mut self.supported_features, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec<code_generator_response::File> as DeserializableFieldFromIter<
                    tags::Message<code_generator_response::File>, 
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.error, serializer, 1)?;
        <::std::option::Option<u64> as SerializableField<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.supported_features, serializer, 2)?;
        <::std::vec::Vec<code_generator_response::File> as SerializableField<
                tags::Message<code_generator_response::File>, 
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
    type FileType = code_generator_response::File;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FileType>;
    fn error(&'_ self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&'_ self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FileType) {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileType>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for CodeGeneratorResponse<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct CodeGeneratorResponseBumpalo<'bump> {
    pub error: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::bumpalo::collections::Vec<'bump, code_generator_response::FileBumpalo<'bump>>,
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.error, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<u64> as DeserializableFieldFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::deser(&mut self.supported_features, field, ::std::default::Default::default)?;
            }
            15 => {
                <::bumpalo::collections::Vec<'bump, code_generator_response::FileBumpalo<'bump>> as DeserializableFieldFromIter<
                    tags::Message<code_generator_response::FileBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.file, field, || code_generator_response::FileBumpalo::new_in(self.puroro_internal.bumpalo()))?;
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.error, serializer, 1)?;
        <::std::option::Option<u64> as SerializableField<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.supported_features, serializer, 2)?;
        <::bumpalo::collections::Vec<'bump, code_generator_response::FileBumpalo<'bump>> as SerializableField<
                tags::Message<code_generator_response::FileBumpalo<'bump>>, 
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
    type FileType = code_generator_response::FileBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FileType>;
    fn error(&'_ self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&'_ self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FileType) {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileType>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
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
    type GeneratedCodeInfoType: self::super::super::GeneratedCodeInfoTrait;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn insertion_point(&'_ self) -> ::std::option::Option<&'_ str>;
    fn content(&'_ self) -> ::std::option::Option<&'_ str>;
    fn generated_code_info(&'_ self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfoType>;
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: ::std::option::Option<::std::string::String>,
    pub insertion_point: ::std::option::Option<::std::string::String>,
    pub content: ::std::option::Option<::std::string::String>,
    pub generated_code_info: ::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>>,
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.insertion_point, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.content, field, ::std::default::Default::default)?;
            }
            16 => {
                <::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>> as DeserializableFieldFromIter<
                    tags::Message<super::super::GeneratedCodeInfo>, 
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::std::string::String> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.insertion_point, serializer, 2)?;
        <::std::option::Option<::std::string::String> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.content, serializer, 15)?;
        <::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>> as SerializableField<
                tags::Message<super::super::GeneratedCodeInfo>, 
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
    type GeneratedCodeInfoType = super::super::GeneratedCodeInfo;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&'_ self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&'_ self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    fn generated_code_info(&'_ self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfoType> {
        self.generated_code_info.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for File<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FileBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub insertion_point: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub content: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub generated_code_info: ::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>>,
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.insertion_point, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            15 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.content, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            16 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>> as DeserializableFieldFromIter<
                    tags::Message<super::super::GeneratedCodeInfoBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.generated_code_info, field, || ::bumpalo::boxed::Box::new_in(super::super::GeneratedCodeInfoBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.insertion_point, serializer, 2)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.content, serializer, 15)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>> as SerializableField<
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileTrait for FileBumpalo<'bump> {
    type GeneratedCodeInfoType = super::super::GeneratedCodeInfoBumpalo<'bump>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&'_ self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&'_ self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    fn generated_code_info(&'_ self) -> ::std::option::Option<&'_ Self::GeneratedCodeInfoType> {
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
    type FileDescriptorProtoType: self::super::FileDescriptorProtoTrait;
    type VersionType: self::VersionTrait;
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a>: ::std::iter::Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a>: ::std::iter::Iterator<Item=&'a self::super::FileDescriptorProto>;
    fn for_each_file_to_generate<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn file_to_generate_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_>;
    fn parameter(&'_ self) -> ::std::option::Option<&'_ str>;
    fn for_each_proto_file<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::FileDescriptorProtoType);
    fn proto_file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_>;
    fn compiler_version(&'_ self) -> ::std::option::Option<&'_ Self::VersionType>;
}

#[derive(Debug, Clone)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: ::std::vec::Vec<::std::string::String>,
    pub parameter: ::std::option::Option<::std::string::String>,
    pub proto_file: ::std::vec::Vec<super::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option<::std::boxed::Box<Version>>,
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::vec::Vec<::std::string::String> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.file_to_generate, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.parameter, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec<super::FileDescriptorProto> as DeserializableFieldFromIter<
                    tags::Message<super::FileDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.proto_file, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<Version>> as DeserializableFieldFromIter<
                    tags::Message<Version>, 
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::std::vec::Vec<::std::string::String> as SerializableField<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.file_to_generate, serializer, 1)?;
        <::std::option::Option<::std::string::String> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.parameter, serializer, 2)?;
        <::std::vec::Vec<super::FileDescriptorProto> as SerializableField<
                tags::Message<super::FileDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.proto_file, serializer, 15)?;
        <::std::option::Option<::std::boxed::Box<Version>> as SerializableField<
                tags::Message<Version>, 
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
    type FileDescriptorProtoType = super::FileDescriptorProto;
    type VersionType = Version;
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FileDescriptorProtoType>;
    fn for_each_file_to_generate<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_> {
        self.file_to_generate.iter().map(|v| v.as_ref())
    }
    fn parameter(&'_ self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FileDescriptorProtoType) {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileDescriptorProtoType>> {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_> {
        self.proto_file.iter()
    }
    fn compiler_version(&'_ self) -> ::std::option::Option<&'_ Self::VersionType> {
        self.compiler_version.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for CodeGeneratorRequest<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct CodeGeneratorRequestBumpalo<'bump> {
    pub file_to_generate: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    pub parameter: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub proto_file: ::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>>,
    pub compiler_version: ::std::option::Option<::bumpalo::boxed::Box<'bump, VersionBumpalo<'bump>>>,
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.file_to_generate, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.parameter, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            15 => {
                <::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>> as DeserializableFieldFromIter<
                    tags::Message<super::FileDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.proto_file, field, || super::FileDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, VersionBumpalo<'bump>>> as DeserializableFieldFromIter<
                    tags::Message<VersionBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.compiler_version, field, || ::bumpalo::boxed::Box::new_in(VersionBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as SerializableField<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.file_to_generate, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as SerializableField<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.parameter, serializer, 2)?;
        <::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>> as SerializableField<
                tags::Message<super::FileDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.proto_file, serializer, 15)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, VersionBumpalo<'bump>>> as SerializableField<
                tags::Message<VersionBumpalo<'bump>>, 
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
    type FileDescriptorProtoType = super::FileDescriptorProtoBumpalo<'bump>;
    type VersionType = VersionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FileDescriptorProtoType>;
    fn for_each_file_to_generate<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_> {
        self.file_to_generate.iter().map(|v| v.as_ref())
    }
    fn parameter(&'_ self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FileDescriptorProtoType) {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileDescriptorProtoType>> {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_> {
        self.proto_file.iter()
    }
    fn compiler_version(&'_ self) -> ::std::option::Option<&'_ Self::VersionType> {
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
    fn major(&'_ self) -> ::std::option::Option<i32>;
    fn minor(&'_ self) -> ::std::option::Option<i32>;
    fn patch(&'_ self) -> ::std::option::Option<i32>;
    fn suffix(&'_ self) -> ::std::option::Option<&'_ str>;
}

#[derive(Debug, Clone)]
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.major, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.minor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.patch, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as SerializableField<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.major, serializer, 1)?;
        <::std::option::Option<i32> as SerializableField<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.minor, serializer, 2)?;
        <::std::option::Option<i32> as SerializableField<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.patch, serializer, 3)?;
        <::std::option::Option<::std::string::String> as SerializableField<
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
    fn major(&'_ self) -> ::std::option::Option<i32> {
        self.major.clone()
    }
    fn minor(&'_ self) -> ::std::option::Option<i32> {
        self.minor.clone()
    }
    fn patch(&'_ self) -> ::std::option::Option<i32> {
        self.patch.clone()
    }
    fn suffix(&'_ self) -> ::std::option::Option<&'_ str> {
        self.suffix.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Version<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
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
        use ::puroro_internal::helpers::DeserializableFieldFromIter;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.major, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.minor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.patch, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.suffix, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
        use ::puroro_internal::helpers::SerializableField;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as SerializableField<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.major, serializer, 1)?;
        <::std::option::Option<i32> as SerializableField<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.minor, serializer, 2)?;
        <::std::option::Option<i32> as SerializableField<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.patch, serializer, 3)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as SerializableField<
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
    fn major(&'_ self) -> ::std::option::Option<i32> {
        self.major.clone()
    }
    fn minor(&'_ self) -> ::std::option::Option<i32> {
        self.minor.clone()
    }
    fn patch(&'_ self) -> ::std::option::Option<i32> {
        self.patch.clone()
    }
    fn suffix(&'_ self) -> ::std::option::Option<&'_ str> {
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
