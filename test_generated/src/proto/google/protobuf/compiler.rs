#![allow(unused_variables)]
#![allow(unused_imports)]


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
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.error, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<u64> as DeserializableFieldFromIter<(
                    tags::UInt64, 
                    tags::Optional2)>>
                ::deser(&mut self.supported_features, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec<code_generator_response::File> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::google::protobuf::compiler::code_generator_response::File>, 
                    tags::Repeated)>>
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

impl ::puroro_internal::serializer::Serializable for CodeGeneratorResponse {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.error.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::UInt64, _>(
            2, 
            self.supported_features.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.file.iter_for_ser() {
            serializer.serialize_message_twice(15, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for CodeGeneratorResponse {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl CodeGeneratorResponseTrait for CodeGeneratorResponse {
    fn error(&self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    type FileType = code_generator_response::File;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ code_generator_response::File)
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self) ->
        ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ code_generator_response::File>>
    {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item=&'a code_generator_response::File>;
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
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.error, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<u64> as DeserializableFieldFromIter<(
                    tags::UInt64, 
                    tags::Optional2)>>
                ::deser(&mut self.supported_features, field, || ::std::default::Default::default)?;
            }
            15 => {
                <::bumpalo::collections::Vec<'bump, code_generator_response::FileBumpalo<'bump>> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::google::protobuf::compiler::code_generator_response::File>, 
                    tags::Repeated)>>
                ::deser(&mut self.file, field, || super::super::super::google::protobuf::compiler::code_generator_response::File::new_in(self.puroro_internal.bumpalo()))?;
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
impl<'bump> ::puroro_internal::serializer::Serializable for CodeGeneratorResponseBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.error.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::UInt64, _>(
            2, 
            self.supported_features.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.file.iter_for_ser() {
            serializer.serialize_message_twice(15, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for CodeGeneratorResponseBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorResponseTrait for CodeGeneratorResponseBumpalo<'bump> {
    fn error(&self) -> ::std::option::Option<&'_ str> {
        self.error.as_deref()
    }
    fn supported_features(&self) -> ::std::option::Option<u64> {
        self.supported_features.clone()
    }
    type FileType = code_generator_response::FileBumpalo<'bump>;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ code_generator_response::File)
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self) ->
        ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ code_generator_response::File>>
    {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item=&'a code_generator_response::File>;
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
pub trait CodeGeneratorResponseTrait {
    fn error(&'_ self) -> ::std::option::Option<&'_ str>;
    fn supported_features(&'_ self) -> ::std::option::Option<u64>;
    type FileType: code_generator_response::FileTrait;
    fn for_each_file<F>(&self, f: F)
    where
        F: FnMut(&'_ code_generator_response::File);
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ code_generator_response::File>>;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a>: Iterator<Item=&'a code_generator_response::File>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_>;
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
impl ::std::convert::Into<i32> for Feature {
    fn into(self) -> i32 {
        self as i32
    }
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
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.insertion_point, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.content, field, ::std::default::Default::default)?;
            }
            16 => {
                <::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::super::google::protobuf::GeneratedCodeInfo>, 
                    tags::Optional2)>>
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

impl ::puroro_internal::serializer::Serializable for File {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.insertion_point.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.content.iter_for_ser() {
            serializer.serialize_bytes_twice(15, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.generated_code_info.iter_for_ser() {
            serializer.serialize_message_twice(16, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for File {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileTrait for File {
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    type GeneratedCodeInfoType = super::super::GeneratedCodeInfo;
    fn generated_code_info(&self) -> ::std::option::Option<&'_ super::super::GeneratedCodeInfo> {
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
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.insertion_point, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            15 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.content, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            16 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::super::google::protobuf::GeneratedCodeInfo>, 
                    tags::Optional2)>>
                ::deser(&mut self.generated_code_info, field, || ::bumpalo::boxed::Box::new_in(super::super::super::super::google::protobuf::GeneratedCodeInfo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
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
impl<'bump> ::puroro_internal::serializer::Serializable for FileBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.insertion_point.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.content.iter_for_ser() {
            serializer.serialize_bytes_twice(15, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.generated_code_info.iter_for_ser() {
            serializer.serialize_message_twice(16, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileTrait for FileBumpalo<'bump> {
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn insertion_point(&self) -> ::std::option::Option<&'_ str> {
        self.insertion_point.as_deref()
    }
    fn content(&self) -> ::std::option::Option<&'_ str> {
        self.content.as_deref()
    }
    type GeneratedCodeInfoType = super::super::GeneratedCodeInfoBumpalo<'bump>;
    fn generated_code_info(&self) -> ::std::option::Option<&'_ super::super::GeneratedCodeInfo> {
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
pub trait FileTrait {
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn insertion_point(&'_ self) -> ::std::option::Option<&'_ str>;
    fn content(&'_ self) -> ::std::option::Option<&'_ str>;
    type GeneratedCodeInfoType: super::super::GeneratedCodeInfoTrait;
    fn generated_code_info(&'_ self) -> ::std::option::Option<&'_ super::super::GeneratedCodeInfo>;
}
} // mod code_generator_response

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
                <::std::vec::Vec<::std::string::String> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Repeated)>>
                ::deser(&mut self.file_to_generate, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.parameter, field, ::std::default::Default::default)?;
            }
            15 => {
                <::std::vec::Vec<super::FileDescriptorProto> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::google::protobuf::FileDescriptorProto>, 
                    tags::Repeated)>>
                ::deser(&mut self.proto_file, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<Version>> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::google::protobuf::compiler::Version>, 
                    tags::Optional2)>>
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

impl ::puroro_internal::serializer::Serializable for CodeGeneratorRequest {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.file_to_generate.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.parameter.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.proto_file.iter_for_ser() {
            serializer.serialize_message_twice(15, msg)?;
        }
        for msg in self.compiler_version.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for CodeGeneratorRequest {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl CodeGeneratorRequestTrait for CodeGeneratorRequest {
    fn for_each_file_to_generate<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str)
    {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self) ->
        ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>
    {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_> {
        self.file_to_generate.iter().map(|v| v.as_ref())
    }
    fn parameter(&self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    type ProtoFileType = super::FileDescriptorProto;
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ super::FileDescriptorProto)
    {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(&self) ->
        ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ super::FileDescriptorProto>>
    {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a> = impl Iterator<Item=&'a super::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_> {
        self.proto_file.iter()
    }
    type CompilerVersionType = Version;
    fn compiler_version(&self) -> ::std::option::Option<&'_ Version> {
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
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Repeated)>>
                ::deser(&mut self.file_to_generate, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
                ::deser(&mut self.parameter, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            15 => {
                <::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::google::protobuf::FileDescriptorProto>, 
                    tags::Repeated)>>
                ::deser(&mut self.proto_file, field, || super::super::super::google::protobuf::FileDescriptorProto::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, VersionBumpalo<'bump>>> as DeserializableFieldFromIter<(
                    tags::Message<super::super::super::google::protobuf::compiler::Version>, 
                    tags::Optional2)>>
                ::deser(&mut self.compiler_version, field, || ::bumpalo::boxed::Box::new_in(super::super::super::google::protobuf::compiler::Version::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
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
impl<'bump> ::puroro_internal::serializer::Serializable for CodeGeneratorRequestBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.file_to_generate.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.parameter.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.proto_file.iter_for_ser() {
            serializer.serialize_message_twice(15, msg)?;
        }
        for msg in self.compiler_version.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for CodeGeneratorRequestBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorRequestTrait for CodeGeneratorRequestBumpalo<'bump> {
    fn for_each_file_to_generate<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str)
    {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self) ->
        ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>
    {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_> {
        self.file_to_generate.iter().map(|v| v.as_ref())
    }
    fn parameter(&self) -> ::std::option::Option<&'_ str> {
        self.parameter.as_deref()
    }
    type ProtoFileType = super::FileDescriptorProtoBumpalo<'bump>;
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ super::FileDescriptorProto)
    {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(&self) ->
        ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ super::FileDescriptorProto>>
    {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a> = impl Iterator<Item=&'a super::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_> {
        self.proto_file.iter()
    }
    type CompilerVersionType = VersionBumpalo<'bump>;
    fn compiler_version(&self) -> ::std::option::Option<&'_ Version> {
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
pub trait CodeGeneratorRequestTrait {
    fn for_each_file_to_generate<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn file_to_generate_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a>: Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_>;
    fn parameter(&'_ self) -> ::std::option::Option<&'_ str>;
    type ProtoFileType: super::FileDescriptorProtoTrait;
    fn for_each_proto_file<F>(&self, f: F)
    where
        F: FnMut(&'_ super::FileDescriptorProto);
    fn proto_file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ super::FileDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a>: Iterator<Item=&'a super::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_>;
    type CompilerVersionType: VersionTrait;
    fn compiler_version(&'_ self) -> ::std::option::Option<&'_ Version>;
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
                <::std::option::Option<i32> as DeserializableFieldFromIter<(
                    tags::Int32, 
                    tags::Optional2)>>
                ::deser(&mut self.major, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<(
                    tags::Int32, 
                    tags::Optional2)>>
                ::deser(&mut self.minor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<(
                    tags::Int32, 
                    tags::Optional2)>>
                ::deser(&mut self.patch, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::std::string::String> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
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

impl ::puroro_internal::serializer::Serializable for Version {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            1, 
            self.major.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            2, 
            self.minor.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            3, 
            self.patch.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.suffix.iter_for_ser() {
            serializer.serialize_bytes_twice(4, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for Version {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
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
                <::std::option::Option<i32> as DeserializableFieldFromIter<(
                    tags::Int32, 
                    tags::Optional2)>>
                ::deser(&mut self.major, field, || ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<(
                    tags::Int32, 
                    tags::Optional2)>>
                ::deser(&mut self.minor, field, || ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as DeserializableFieldFromIter<(
                    tags::Int32, 
                    tags::Optional2)>>
                ::deser(&mut self.patch, field, || ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as DeserializableFieldFromIter<(
                    tags::String, 
                    tags::Optional2)>>
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
impl<'bump> ::puroro_internal::serializer::Serializable for VersionBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            1, 
            self.major.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            2, 
            self.minor.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            3, 
            self.patch.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.suffix.iter_for_ser() {
            serializer.serialize_bytes_twice(4, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for VersionBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
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
pub trait VersionTrait {
    fn major(&'_ self) -> ::std::option::Option<i32>;
    fn minor(&'_ self) -> ::std::option::Option<i32>;
    fn patch(&'_ self) -> ::std::option::Option<i32>;
    fn suffix(&'_ self) -> ::std::option::Option<&'_ str>;
}
