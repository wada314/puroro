#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct CodeGeneratorResponse<'bump> {
    pub error: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub supported_features: ::std::option::Option::<u64>,
    pub file: ::puroro::bumpalo::collections::Vec::<'bump, self::code_generator_response::File::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> CodeGeneratorResponse<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            error: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            supported_features: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            file: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for CodeGeneratorResponse<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            error: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.error),
            supported_features: <::std::option::Option::<u64> as Clone>::clone(&self.supported_features),
            file: <::puroro::bumpalo::collections::Vec::<'bump, self::code_generator_response::File::<'bump>> as Clone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for CodeGeneratorResponse<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.error, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<u64> as FieldMergeFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::merge(&mut self.supported_features, field, ::std::default::Default::default)?;
            }
            15 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::code_generator_response::File::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::code_generator_response::File::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.file, field, || self::code_generator_response::File::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for CodeGeneratorResponse<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for CodeGeneratorResponse<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.error, serializer, 1)?;
        <::std::option::Option::<u64> as FieldSer<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.supported_features, serializer, 2)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::code_generator_response::File::<'bump>> as FieldSer<
                tags::Message::<self::code_generator_response::File::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 15)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for CodeGeneratorResponse<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::compiler::CodeGeneratorResponseTrait for CodeGeneratorResponse<'bump> {
    fn error<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.error.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn supported_features<'this>(&'this self) -> ::std::option::Option::<u64> {
        self.supported_features.clone()
    }
    type FileElement<'this> where Self: 'this = self::code_generator_response::File::<'bump>;
    type FileRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::code_generator_response::File::<'bump>>;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this> {
        &self.file
    }
}

impl<'bump> ::puroro::Message for CodeGeneratorResponse<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::CodeGeneratorResponseTag> for CodeGeneratorResponse<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for CodeGeneratorResponse<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod code_generator_response {
#[derive(Debug)]
pub struct File<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub insertion_point: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub content: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub generated_code_info: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, super::super::GeneratedCodeInfo::<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> File<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            insertion_point: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            content: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            generated_code_info: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for File<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            insertion_point: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.insertion_point),
            content: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.content),
            generated_code_info: self.generated_code_info.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for File<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.insertion_point, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            15 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.content, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            16 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, super::super::GeneratedCodeInfo::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<super::super::GeneratedCodeInfo::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.generated_code_info, field, || super::super::GeneratedCodeInfo::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for File<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for File<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.insertion_point, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.content, serializer, 15)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, super::super::GeneratedCodeInfo::<'bump>>> as FieldSer<
                tags::Message::<super::super::GeneratedCodeInfo::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.generated_code_info, serializer, 16)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for File<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::super::traits::google::protobuf::compiler::code_generator_response::FileTrait for File<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn insertion_point<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.insertion_point.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn content<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.content.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type GeneratedCodeInfoType<'this> where Self: 'this = super::super::GeneratedCodeInfo::<'bump>;
    fn generated_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::GeneratedCodeInfoType::<'this>>> {
        self.generated_code_info.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl<'bump> ::puroro::Message for File<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::super::tags::google::protobuf::compiler::code_generator_response::FileTag> for File<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for File<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

} // mod code_generator_response
#[derive(Debug)]
pub struct CodeGeneratorRequest<'bump> {
    pub file_to_generate: ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>,
    pub parameter: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub proto_file: ::puroro::bumpalo::collections::Vec::<'bump, super::FileDescriptorProto::<'bump>>,
    pub compiler_version: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::Version::<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> CodeGeneratorRequest<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            file_to_generate: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            parameter: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            proto_file: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            compiler_version: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for CodeGeneratorRequest<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            file_to_generate: <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.file_to_generate),
            parameter: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.parameter),
            proto_file: <::puroro::bumpalo::collections::Vec::<'bump, super::FileDescriptorProto::<'bump>> as Clone>::clone(&self.proto_file),
            compiler_version: self.compiler_version.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for CodeGeneratorRequest<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.file_to_generate, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.parameter, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            15 => {
                <::puroro::bumpalo::collections::Vec::<'bump, super::FileDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<super::FileDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.proto_file, field, || super::FileDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            3 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::Version::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::Version::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.compiler_version, field, || self::Version::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for CodeGeneratorRequest<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for CodeGeneratorRequest<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.file_to_generate, serializer, 1)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.parameter, serializer, 2)?;
        <::puroro::bumpalo::collections::Vec::<'bump, super::FileDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<super::FileDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.proto_file, serializer, 15)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::Version::<'bump>>> as FieldSer<
                tags::Message::<self::Version::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.compiler_version, serializer, 3)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for CodeGeneratorRequest<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::compiler::CodeGeneratorRequestTrait for CodeGeneratorRequest<'bump> {
    type FileToGenerateRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>;
    fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeated::<'this> {
        &self.file_to_generate
    }
    fn parameter<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.parameter.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ProtoFileElement<'this> where Self: 'this = super::FileDescriptorProto::<'bump>;
    type ProtoFileRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, super::FileDescriptorProto::<'bump>>;
    fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeated::<'this> {
        &self.proto_file
    }
    type CompilerVersionType<'this> where Self: 'this = self::Version::<'bump>;
    fn compiler_version<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::CompilerVersionType::<'this>>> {
        self.compiler_version.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl<'bump> ::puroro::Message for CodeGeneratorRequest<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::CodeGeneratorRequestTag> for CodeGeneratorRequest<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for CodeGeneratorRequest<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct Version<'bump> {
    pub major: ::std::option::Option::<i32>,
    pub minor: ::std::option::Option::<i32>,
    pub patch: ::std::option::Option::<i32>,
    pub suffix: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> Version<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            major: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            minor: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            patch: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            suffix: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for Version<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            major: <::std::option::Option::<i32> as Clone>::clone(&self.major),
            minor: <::std::option::Option::<i32> as Clone>::clone(&self.minor),
            patch: <::std::option::Option::<i32> as Clone>::clone(&self.patch),
            suffix: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.suffix),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for Version<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.suffix, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for Version<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for Version<'bump> {
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
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.suffix, serializer, 4)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for Version<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::compiler::VersionTrait for Version<'bump> {
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

impl<'bump> ::puroro::Message for Version<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::VersionTag> for Version<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for Version<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

