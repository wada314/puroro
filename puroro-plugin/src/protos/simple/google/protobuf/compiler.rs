#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct CodeGeneratorResponse {
    pub error: ::std::option::Option::<::std::string::String>,
    pub supported_features: ::std::option::Option::<u64>,
    pub file: ::std::vec::Vec::<self::code_generator_response::File>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl CodeGeneratorResponse {
    pub fn new() -> Self {
        Self {
            error: ::puroro_internal::FieldNew::new(),
            supported_features: ::puroro_internal::FieldNew::new(),
            file: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for CodeGeneratorResponse {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            error: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.error),
            supported_features: <::std::option::Option::<u64> as Clone>::clone(&self.supported_features),
            file: <::std::vec::Vec::<self::code_generator_response::File> as Clone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for CodeGeneratorResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for CodeGeneratorResponse {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.error, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<u64> as FieldMergeFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::merge(&mut self.supported_features, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec::<self::code_generator_response::File> as FieldMergeFromIter<
                    tags::Message::<self::code_generator_response::File>, 
                    tags::Repeated>>
                ::merge(&mut self.file, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for CodeGeneratorResponse {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for CodeGeneratorResponse {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for CodeGeneratorResponse {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.error, serializer, 1)?;
        <::std::option::Option::<u64> as FieldSer<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.supported_features, serializer, 2)?;
        <::std::vec::Vec::<self::code_generator_response::File> as FieldSer<
                tags::Message::<self::code_generator_response::File>, 
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

impl super::super::super::super::traits::google::protobuf::compiler::CodeGeneratorResponseTrait for CodeGeneratorResponse {
    fn error<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.error.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn supported_features<'this>(&'this self) -> ::std::option::Option::<u64> {
        self.supported_features.clone()
    }
    type FileElement<'this> where Self: 'this = self::code_generator_response::File;
    type FileRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::code_generator_response::File>;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this> {
        &self.file
    }
}

impl ::puroro::Message for CodeGeneratorResponse {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::CodeGeneratorResponseTag> for CodeGeneratorResponse {
}

impl<'a> ::puroro_internal::FieldNew<'a> for CodeGeneratorResponse {
    fn new() -> Self {
        Default::default()
    }
}

pub mod code_generator_response {
#[derive(Debug)]
pub struct File {
    pub name: ::std::option::Option::<::std::string::String>,
    pub insertion_point: ::std::option::Option::<::std::string::String>,
    pub content: ::std::option::Option::<::std::string::String>,
    pub generated_code_info: ::std::option::Option::<::std::boxed::Box::<super::super::GeneratedCodeInfo>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl File {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            insertion_point: ::puroro_internal::FieldNew::new(),
            content: ::puroro_internal::FieldNew::new(),
            generated_code_info: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for File {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            insertion_point: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.insertion_point),
            content: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.content),
            generated_code_info: <::std::option::Option::<::std::boxed::Box::<super::super::GeneratedCodeInfo>> as Clone>::clone(&self.generated_code_info),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for File {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for File {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.insertion_point, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.content, field, ::std::default::Default::default)?;
            }
            16 => {
                <::std::option::Option::<::std::boxed::Box::<super::super::GeneratedCodeInfo>> as FieldMergeFromIter<
                    tags::Message::<super::super::GeneratedCodeInfo>, 
                    tags::Optional2>>
                ::merge(&mut self.generated_code_info, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for File {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for File {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for File {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.insertion_point, serializer, 2)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.content, serializer, 15)?;
        <::std::option::Option::<::std::boxed::Box::<super::super::GeneratedCodeInfo>> as FieldSer<
                tags::Message::<super::super::GeneratedCodeInfo>, 
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

impl super::super::super::super::super::traits::google::protobuf::compiler::code_generator_response::FileTrait for File {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn insertion_point<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.insertion_point.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn content<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.content.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type GeneratedCodeInfoType<'this> where Self: 'this = super::super::GeneratedCodeInfo;
    fn generated_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::GeneratedCodeInfoType::<'this>>> {
        self.generated_code_info.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for File {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::super::tags::google::protobuf::compiler::code_generator_response::FileTag> for File {
}

impl<'a> ::puroro_internal::FieldNew<'a> for File {
    fn new() -> Self {
        Default::default()
    }
}

} // mod code_generator_response
#[derive(Debug)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: ::std::vec::Vec::<::std::string::String>,
    pub parameter: ::std::option::Option::<::std::string::String>,
    pub proto_file: ::std::vec::Vec::<super::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option::<::std::boxed::Box::<self::Version>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl CodeGeneratorRequest {
    pub fn new() -> Self {
        Self {
            file_to_generate: ::puroro_internal::FieldNew::new(),
            parameter: ::puroro_internal::FieldNew::new(),
            proto_file: ::puroro_internal::FieldNew::new(),
            compiler_version: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for CodeGeneratorRequest {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            file_to_generate: <::std::vec::Vec::<::std::string::String> as Clone>::clone(&self.file_to_generate),
            parameter: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.parameter),
            proto_file: <::std::vec::Vec::<super::FileDescriptorProto> as Clone>::clone(&self.proto_file),
            compiler_version: <::std::option::Option::<::std::boxed::Box::<self::Version>> as Clone>::clone(&self.compiler_version),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for CodeGeneratorRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for CodeGeneratorRequest {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.file_to_generate, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.parameter, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec::<super::FileDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<super::FileDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.proto_file, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::boxed::Box::<self::Version>> as FieldMergeFromIter<
                    tags::Message::<self::Version>, 
                    tags::Optional2>>
                ::merge(&mut self.compiler_version, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for CodeGeneratorRequest {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for CodeGeneratorRequest {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for CodeGeneratorRequest {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.file_to_generate, serializer, 1)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.parameter, serializer, 2)?;
        <::std::vec::Vec::<super::FileDescriptorProto> as FieldSer<
                tags::Message::<super::FileDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.proto_file, serializer, 15)?;
        <::std::option::Option::<::std::boxed::Box::<self::Version>> as FieldSer<
                tags::Message::<self::Version>, 
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

impl super::super::super::super::traits::google::protobuf::compiler::CodeGeneratorRequestTrait for CodeGeneratorRequest {
    type FileToGenerateRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<::std::string::String>;
    fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeated::<'this> {
        &self.file_to_generate
    }
    fn parameter<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.parameter.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ProtoFileElement<'this> where Self: 'this = super::FileDescriptorProto;
    type ProtoFileRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<super::FileDescriptorProto>;
    fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeated::<'this> {
        &self.proto_file
    }
    type CompilerVersionType<'this> where Self: 'this = self::Version;
    fn compiler_version<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::CompilerVersionType::<'this>>> {
        self.compiler_version.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for CodeGeneratorRequest {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::CodeGeneratorRequestTag> for CodeGeneratorRequest {
}

impl<'a> ::puroro_internal::FieldNew<'a> for CodeGeneratorRequest {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct Version {
    pub major: ::std::option::Option::<i32>,
    pub minor: ::std::option::Option::<i32>,
    pub patch: ::std::option::Option::<i32>,
    pub suffix: ::std::option::Option::<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: ::puroro_internal::FieldNew::new(),
            minor: ::puroro_internal::FieldNew::new(),
            patch: ::puroro_internal::FieldNew::new(),
            suffix: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for Version {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            major: <::std::option::Option::<i32> as Clone>::clone(&self.major),
            minor: <::std::option::Option::<i32> as Clone>::clone(&self.minor),
            patch: <::std::option::Option::<i32> as Clone>::clone(&self.patch),
            suffix: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.suffix),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for Version {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.major, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.minor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.patch, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.suffix, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for Version {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for Version {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for Version {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.major, serializer, 1)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.minor, serializer, 2)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.patch, serializer, 3)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
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

impl super::super::super::super::traits::google::protobuf::compiler::VersionTrait for Version {
    fn major<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.major.clone()
    }
    fn minor<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.minor.clone()
    }
    fn patch<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.patch.clone()
    }
    fn suffix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.suffix.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for Version {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::VersionTag> for Version {
}

impl<'a> ::puroro_internal::FieldNew<'a> for Version {
    fn new() -> Self {
        Default::default()
    }
}

