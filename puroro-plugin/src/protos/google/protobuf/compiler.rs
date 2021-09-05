// A generated source code by puroro library
// package google.protobuf.compiler

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}


#[allow(unused)]
pub struct Version<ImplTag = ::puroro::tags::SimpleImpl>(<Self as ::puroro_internal::GetImplStruct>::Type)
where Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for Version<ImplTag>
where Self: ::puroro_internal::GetImplStruct
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for Version<ImplTag>
where Self: ::puroro_internal::GetImplStruct
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for Version<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for Version<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for Version<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.0, formatter)
    }
}
impl<ImplTag> ::std::cmp::PartialEq for Version<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for Version<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
{
}

impl<ImplTag> self::_puroro_traits::VersionTrait for Version<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
{
    fn major<'this>(&'this self) -> ::std::option::Option<i32> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
        >::major(&self.0)
    }
    fn minor<'this>(&'this self) -> ::std::option::Option<i32> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
        >::minor(&self.0)
    }
    fn patch<'this>(&'this self) -> ::std::option::Option<i32> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
        >::patch(&self.0)
    }
    fn suffix<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
        >::suffix(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for Version<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
{
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
    }
}
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Version<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro_internal::de::DeserFieldsFromBytesIter,
{
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(&mut self.0, field_number, data)
    }
}

impl<ImplTag> ::puroro::SerToIoWrite for Version<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::puroro::SerToIoWrite::ser(&self.0, out)
    }
}

#[allow(unused)]
pub struct CodeGeneratorRequest<ImplTag = ::puroro::tags::SimpleImpl>(<Self as ::puroro_internal::GetImplStruct>::Type)
where Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for CodeGeneratorRequest<ImplTag>
where Self: ::puroro_internal::GetImplStruct
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for CodeGeneratorRequest<ImplTag>
where Self: ::puroro_internal::GetImplStruct
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for CodeGeneratorRequest<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for CodeGeneratorRequest<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for CodeGeneratorRequest<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.0, formatter)
    }
}
impl<ImplTag> ::std::cmp::PartialEq for CodeGeneratorRequest<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for CodeGeneratorRequest<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
{
}

impl<ImplTag> self::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequest<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
{
    type Field1RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
    >::Field1RepeatedType::<'this>;

    fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
        >::file_to_generate(&self.0)
    }
    fn parameter<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
        >::parameter(&self.0)
    }
    type Field15MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
    >::Field15MessageType::<'this>;
    type Field15RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
    >::Field15RepeatedType::<'this>;

    fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
        >::proto_file(&self.0)
    }
    type Field3MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
    >::Field3MessageType::<'this>;
    fn compiler_version<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorRequestTrait
        >::compiler_version(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for CodeGeneratorRequest<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
{
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
    }
}
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorRequest<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro_internal::de::DeserFieldsFromBytesIter,
{
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(&mut self.0, field_number, data)
    }
}

impl<ImplTag> ::puroro::SerToIoWrite for CodeGeneratorRequest<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::puroro::SerToIoWrite::ser(&self.0, out)
    }
}

#[allow(unused)]
pub struct CodeGeneratorResponse<ImplTag = ::puroro::tags::SimpleImpl>(<Self as ::puroro_internal::GetImplStruct>::Type)
where Self: ::puroro_internal::GetImplStruct;

impl<ImplTag> ::std::ops::Deref for CodeGeneratorResponse<ImplTag>
where Self: ::puroro_internal::GetImplStruct
{
    type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<ImplTag> ::std::ops::DerefMut for CodeGeneratorResponse<ImplTag>
where Self: ::puroro_internal::GetImplStruct
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<ImplTag> ::std::clone::Clone for CodeGeneratorResponse<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<ImplTag> ::std::default::Default for CodeGeneratorResponse<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
{
    fn default() -> Self {
        Self(::std::default::Default::default())
    }
}
impl<ImplTag> ::std::fmt::Debug for CodeGeneratorResponse<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(&self.0, formatter)
    }
}
impl<ImplTag> ::std::cmp::PartialEq for CodeGeneratorResponse<ImplTag>
where 
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
    }
}

impl<ImplTag> ::puroro::Message for CodeGeneratorResponse<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
{
}

impl<ImplTag> self::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponse<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorResponseTrait
{
    fn error<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorResponseTrait
        >::error(&self.0)
    }
    fn supported_features<'this>(&'this self) -> ::std::option::Option<u64> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorResponseTrait
        >::supported_features(&self.0)
    }
    type Field15MessageType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorResponseTrait
    >::Field15MessageType::<'this>;
    type Field15RepeatedType<'this> = <
        <Self as ::puroro_internal::GetImplStruct>::Type
        as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorResponseTrait
    >::Field15RepeatedType::<'this>;

    fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
        <<Self as ::puroro_internal::GetImplStruct>::Type
            as self::_puroro_root::google::protobuf::compiler::_puroro_traits::CodeGeneratorResponseTrait
        >::file(&self.0)
    }
}

impl<ImplTag> ::puroro::DeserFromBytesIter for CodeGeneratorResponse<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
{
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
    }
}
impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorResponse<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro_internal::de::DeserFieldsFromBytesIter,
{
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(&mut self.0, field_number, data)
    }
}

impl<ImplTag> ::puroro::SerToIoWrite for CodeGeneratorResponse<ImplTag>
where
    Self: ::puroro_internal::GetImplStruct,
    <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::puroro::SerToIoWrite::ser(&self.0, out)
    }
}

mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

impl ::puroro_internal::GetImplStruct for super::Version<::puroro::tags::SimpleImpl> {
    type Type = Version_Simple;
}

#[allow(non_camel_case_types)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Version_Simple {
    
    pub major: ::std::option::Option<i32>,
    
    
    pub minor: ::std::option::Option<i32>,
    
    
    pub patch: ::std::option::Option<i32>,
    
    
    pub suffix: ::std::option::Option<::std::string::String>,
    
}
impl ::puroro::Message for Version_Simple {}

impl super::_puroro_traits::VersionTrait for Version_Simple {
    fn major<'this>(&'this self) -> ::std::option::Option<i32> {
        ::std::clone::Clone::clone(&self.major)
    }
    fn minor<'this>(&'this self) -> ::std::option::Option<i32> {
        ::std::clone::Clone::clone(&self.minor)
    }
    fn patch<'this>(&'this self) -> ::std::option::Option<i32> {
        ::std::clone::Clone::clone(&self.patch)
    }
    fn suffix<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        self.suffix.as_ref().map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
    }
}

impl ::puroro::DeserFromBytesIter for Version_Simple {
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::de::DeserFieldsFromBytesIter for Version_Simple {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.major, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.minor, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.patch, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.suffix, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
    }
}

impl ::puroro::SerToIoWrite for Version_Simple {
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Int32
        >::ser_field(&self.major, 1, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Int32
        >::ser_field(&self.minor, 2, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Int32
        >::ser_field(&self.patch, 3, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::String
        >::ser_field(&self.suffix, 4, out)?;
        ::std::result::Result::Ok(())
    }
}

impl ::puroro_internal::GetImplStruct for super::CodeGeneratorRequest<::puroro::tags::SimpleImpl> {
    type Type = CodeGeneratorRequest_Simple;
}

#[allow(non_camel_case_types)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct CodeGeneratorRequest_Simple {
    
    pub file_to_generate: ::std::vec::Vec<::std::string::String>,
    
    
    pub parameter: ::std::option::Option<::std::string::String>,
    
    
    pub proto_file: ::std::vec::Vec<self::_puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>,
    
    
    pub compiler_version: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>>>,
    
}
impl ::puroro::Message for CodeGeneratorRequest_Simple {}

impl super::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequest_Simple {
    type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;

    fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecCowWrapper::new(&self.file_to_generate)
    }
    fn parameter<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        self.parameter.as_ref().map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
    }
    type Field15MessageType<'this> = self::_puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>;
    type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, self::_puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>;

    fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecCowWrapper::new(&self.proto_file)
    }
    type Field3MessageType<'this> = self::_puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>;
    fn compiler_version<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>> {
        self.compiler_version.as_ref().map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
    }
}

impl ::puroro::DeserFromBytesIter for CodeGeneratorRequest_Simple {
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorRequest_Simple {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.file_to_generate, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.parameter, data),
            15 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>
            >::deser_field(&mut self.proto_file, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>>
            >::deser_field(&mut self.compiler_version, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
    }
}

impl ::puroro::SerToIoWrite for CodeGeneratorRequest_Simple {
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::String
        >::ser_field(&self.file_to_generate, 1, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::String
        >::ser_field(&self.parameter, 2, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>
        >::ser_field(&self.proto_file, 15, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>>
        >::ser_field(&self.compiler_version, 3, out)?;
        ::std::result::Result::Ok(())
    }
}

impl ::puroro_internal::GetImplStruct for super::CodeGeneratorResponse<::puroro::tags::SimpleImpl> {
    type Type = CodeGeneratorResponse_Simple;
}

#[allow(non_camel_case_types)]
#[derive(Clone, Default, PartialEq, Debug)]
pub struct CodeGeneratorResponse_Simple {
    
    pub error: ::std::option::Option<::std::string::String>,
    
    
    pub supported_features: ::std::option::Option<u64>,
    
    
    pub file: ::std::vec::Vec<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>,
    
}
impl ::puroro::Message for CodeGeneratorResponse_Simple {}

impl super::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponse_Simple {
    fn error<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
        self.error.as_ref().map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
    }
    fn supported_features<'this>(&'this self) -> ::std::option::Option<u64> {
        ::std::clone::Clone::clone(&self.supported_features)
    }
    type Field15MessageType<'this> = self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>;
    type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>;

    fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
        ::puroro_internal::impls::simple::VecCowWrapper::new(&self.file)
    }
}

impl ::puroro::DeserFromBytesIter for CodeGeneratorResponse_Simple {
    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
    }
}

impl ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorResponse_Simple {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
    ) -> ::puroro::Result<()>
    where
        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
    {
        match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.error, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.supported_features, data),
            15 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>
            >::deser_field(&mut self.file, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
    }
}

impl ::puroro::SerToIoWrite for CodeGeneratorResponse_Simple {
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::String
        >::ser_field(&self.error, 1, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::UInt64
        >::ser_field(&self.supported_features, 2, out)?;
        ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>
        >::ser_field(&self.file, 15, out)?;
        ::std::result::Result::Ok(())
    }
}
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    
    pub trait VersionTrait: ::std::clone::Clone {
        fn major<'this>(&'this self) -> ::std::option::Option<i32>;
        fn minor<'this>(&'this self) -> ::std::option::Option<i32>;
        fn patch<'this>(&'this self) -> ::std::option::Option<i32>;
        fn suffix<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
    }
    pub trait CodeGeneratorRequestTrait: ::std::clone::Clone {
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, str>>;
    
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
        fn parameter<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field15MessageType<'this>: 'this + self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait;
        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, Self::Field15MessageType<'this>>>;
    
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
        type Field3MessageType<'this>: 'this + self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait;
        fn compiler_version<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
    }
    pub trait CodeGeneratorResponseTrait: ::std::clone::Clone {
        fn error<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        fn supported_features<'this>(&'this self) -> ::std::option::Option<u64>;
        type Field15MessageType<'this>: 'this + self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait;
        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, Self::Field15MessageType<'this>>>;
    
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod version {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
        
    }
    pub mod code_generator_request {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
        
    }
    pub mod code_generator_response {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
        
        
        #[allow(unused)]
        pub struct File<ImplTag = ::puroro::tags::SimpleImpl>(<Self as ::puroro_internal::GetImplStruct>::Type)
        where Self: ::puroro_internal::GetImplStruct;
        
        impl<ImplTag> ::std::ops::Deref for File<ImplTag>
        where Self: ::puroro_internal::GetImplStruct
        {
            type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<ImplTag> ::std::ops::DerefMut for File<ImplTag>
        where Self: ::puroro_internal::GetImplStruct
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<ImplTag> ::std::clone::Clone for File<ImplTag>
        where 
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        impl<ImplTag> ::std::default::Default for File<ImplTag>
        where 
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                Self(::std::default::Default::default())
            }
        }
        impl<ImplTag> ::std::fmt::Debug for File<ImplTag>
        where 
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, formatter)
            }
        }
        impl<ImplTag> ::std::cmp::PartialEq for File<ImplTag>
        where 
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::std::cmp::PartialEq,
        {
            fn eq(&self, rhs: &Self) -> bool {
                ::std::cmp::PartialEq::eq(&self.0, &rhs.0)
            }
        }
        
        impl<ImplTag> ::puroro::Message for File<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
        {
        }
        
        impl<ImplTag> self::_puroro_traits::FileTrait for File<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait
        {
            fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
                <<Self as ::puroro_internal::GetImplStruct>::Type
                    as self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait
                >::name(&self.0)
            }
            fn insertion_point<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
                <<Self as ::puroro_internal::GetImplStruct>::Type
                    as self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait
                >::insertion_point(&self.0)
            }
            fn content<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
                <<Self as ::puroro_internal::GetImplStruct>::Type
                    as self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait
                >::content(&self.0)
            }
            type Field16MessageType<'this> = <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait
            >::Field16MessageType::<'this>;
            fn generated_code_info<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field16MessageType<'this>>> {
                <<Self as ::puroro_internal::GetImplStruct>::Type
                    as self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait
                >::generated_code_info(&self.0)
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for File<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
            }
        }
        impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for File<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro_internal::de::DeserFieldsFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(&mut self.0, field_number, data)
            }
        }
        
        impl<ImplTag> ::puroro::SerToIoWrite for File<ImplTag>
        where
            Self: ::puroro_internal::GetImplStruct,
            <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::puroro::SerToIoWrite::ser(&self.0, out)
            }
        }
        
        mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
        
        impl ::puroro_internal::GetImplStruct for super::File<::puroro::tags::SimpleImpl> {
            type Type = File_Simple;
        }
        
        #[allow(non_camel_case_types)]
        #[derive(Clone, Default, PartialEq, Debug)]
        pub struct File_Simple {
            
            pub name: ::std::option::Option<::std::string::String>,
            
            
            pub insertion_point: ::std::option::Option<::std::string::String>,
            
            
            pub content: ::std::option::Option<::std::string::String>,
            
            
            pub generated_code_info: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>>>,
            
        }
        impl ::puroro::Message for File_Simple {}
        
        impl super::_puroro_traits::FileTrait for File_Simple {
            fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
                self.name.as_ref().map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
            }
            fn insertion_point<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
                self.insertion_point.as_ref().map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
            }
            fn content<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
                self.content.as_ref().map(|v| ::std::borrow::Cow::Borrowed(v.as_ref()))
            }
            type Field16MessageType<'this> = self::_puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>;
            fn generated_code_info<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field16MessageType<'this>>> {
                self.generated_code_info.as_ref().map(|boxed| ::std::borrow::Cow::Borrowed(boxed.as_ref()))
            }
        }
        
        impl ::puroro::DeserFromBytesIter for File_Simple {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }
        
        impl ::puroro_internal::de::DeserFieldsFromBytesIter for File_Simple {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                match field_number {
                    1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.name, data),
                    2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.insertion_point, data),
                    15 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.content, data),
                    16 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>>
                    >::deser_field(&mut self.generated_code_info, data),
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
            }
        }
        
        impl ::puroro::SerToIoWrite for File_Simple {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.name, 1, out)?;
                ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.insertion_point, 2, out)?;
                ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.content, 15, out)?;
                ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>>
                >::ser_field(&self.generated_code_info, 16, out)?;
                ::std::result::Result::Ok(())
            }
        }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            
            pub trait FileTrait: ::std::clone::Clone {
                fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
                fn insertion_point<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
                fn content<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
                type Field16MessageType<'this>: 'this + self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait;
                fn generated_code_info<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field16MessageType<'this>>>;
            }
        }
        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
        pub enum Feature {
            FeatureNone,
            FeatureProto3Optional,
        }
        
        impl ::puroro::Enum2 for Feature {}
        impl ::std::convert::TryFrom<i32> for Feature {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    0 => Feature::FeatureNone,
                    1 => Feature::FeatureProto3Optional,
                    _ => Err(value)?,
                })
            }
        }
        
        impl ::std::convert::From<Feature> for i32 {
            fn from(value: Feature) -> i32 {
                match value {
                    Feature::FeatureNone => 0,
                    Feature::FeatureProto3Optional => 1,
                }
            }
        }
        
        impl ::std::default::Default for Feature {
            fn default() -> Self {
                Feature::FeatureNone
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod file {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
                
            }
        }
    }
}