// A generated source code by puroro library
// package google.protobuf.compiler

pub mod puroro_root {
    pub use super::super::super::super::*;
}


pub mod puroro_structs {

    pub mod puroro_root {
        pub use super::super::puroro_root::*;
    }
    
    #[allow(unused)]
    pub struct Version<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where Self: ::puroro_internal::GetImplStruct;
    
    impl<ImplTag> ::puroro::Message for Version<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
    {
    }
    
    impl<ImplTag> self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait for Version<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait
    {
        /// optional int32 major = 1;
        fn major<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait
            >::major(&self.0)
        }
        /// optional int32 minor = 2;
        fn minor<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait
            >::minor(&self.0)
        }
        /// optional int32 patch = 3;
        fn patch<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait
            >::patch(&self.0)
        }
        /// optional string suffix = 4;
        fn suffix<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait
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
    
    
    #[allow(unused)]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct Version_SimpleImpl {
        pub major: ::std::option::Option<i32>,
        pub minor: ::std::option::Option<i32>,
        pub patch: ::std::option::Option<i32>,
        pub suffix: ::std::option::Option<::std::string::String>,
    }
    
    impl ::puroro_internal::GetImplStruct for Version::<::puroro::tags::SimpleImpl> {
        type Type = Version_SimpleImpl;
    }
    
    impl ::puroro::Message for Version_SimpleImpl {}
    
    impl self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait for Version_SimpleImpl {
        /// optional int32 major = 1;
        fn major<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional int32 minor = 2;
        fn minor<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional int32 patch = 3;
        fn patch<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional string suffix = 4;
        fn suffix<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
    }
    
    impl ::puroro::DeserFromBytesIter for Version_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }
    
    impl ::puroro_internal::de::DeserFieldsFromBytesIter for Version_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.major, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.minor, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.patch, data),
                4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.suffix, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }
    
    impl ::puroro::SerToIoWrite for Version_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::ser_field(&self.major, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::ser_field(&self.minor, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::ser_field(&self.patch, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::ser_field(&self.suffix, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }
    
    #[allow(unused)]
    pub struct CodeGeneratorRequest<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where Self: ::puroro_internal::GetImplStruct;
    
    impl<ImplTag> ::puroro::Message for CodeGeneratorRequest<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
    {
    }
    
    impl<ImplTag> self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequest<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
    {
        type Field1RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
        >::Field1RepeatedType::<'this>;
    
        /// repeated string file_to_generate = 1;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
            >::file_to_generate(&self.0)
        }
        /// optional string parameter = 2;
        fn parameter<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
            >::parameter(&self.0)
        }
        type Field15MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
        >::Field15MessageType::<'this>;
        type Field15RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
        >::Field15RepeatedType::<'this>;
    
        /// repeated FileDescriptorProto proto_file = 15;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
            >::proto_file(&self.0)
        }
        type Field3MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
        >::Field3MessageType::<'this>;
        /// optional Version compiler_version = 3;
        fn compiler_version<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait
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
    
    
    #[allow(unused)]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct CodeGeneratorRequest_SimpleImpl {
        pub file_to_generate: ::std::vec::Vec<::std::string::String>,
        pub parameter: ::std::option::Option<::std::string::String>,
        pub proto_file: ::std::vec::Vec<self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>,
        pub compiler_version: ::std::option::Option<::std::boxed::Box<self::puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>>>,
    }
    
    impl ::puroro_internal::GetImplStruct for CodeGeneratorRequest::<::puroro::tags::SimpleImpl> {
        type Type = CodeGeneratorRequest_SimpleImpl;
    }
    
    impl ::puroro::Message for CodeGeneratorRequest_SimpleImpl {}
    
    impl self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequest_SimpleImpl {
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;
    
        /// repeated string file_to_generate = 1;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.file_to_generate)
        }
        /// optional string parameter = 2;
        fn parameter<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field15MessageType<'this> = self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>;
    
        /// repeated FileDescriptorProto proto_file = 15;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.proto_file)
        }
        type Field3MessageType<'this> = self::puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>;
        /// optional Version compiler_version = 3;
        fn compiler_version<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>> {
            todo!()
        }
    }
    
    impl ::puroro::DeserFromBytesIter for CodeGeneratorRequest_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }
    
    impl ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorRequest_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.file_to_generate, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.parameter, data),
                15 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.proto_file, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.compiler_version, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }
    
    impl ::puroro::SerToIoWrite for CodeGeneratorRequest_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::ser_field(&self.file_to_generate, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::ser_field(&self.parameter, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.proto_file, 15, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::puroro_root::google::protobuf::compiler::Version<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.compiler_version, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }
    
    #[allow(unused)]
    pub struct CodeGeneratorResponse<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where Self: ::puroro_internal::GetImplStruct;
    
    impl<ImplTag> ::puroro::Message for CodeGeneratorResponse<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
    {
    }
    
    impl<ImplTag> self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponse<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait
    {
        /// optional string error = 1;
        fn error<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait
            >::error(&self.0)
        }
        /// optional uint64 supported_features = 2;
        fn supported_features<'this>(&'this self) -> ::std::option::Option<u64> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait
            >::supported_features(&self.0)
        }
        type Field15MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait
        >::Field15MessageType::<'this>;
        type Field15RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait
        >::Field15RepeatedType::<'this>;
    
        /// repeated File file = 15;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait
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
    
    
    #[allow(unused)]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct CodeGeneratorResponse_SimpleImpl {
        pub error: ::std::option::Option<::std::string::String>,
        pub supported_features: ::std::option::Option<u64>,
        pub file: ::std::vec::Vec<self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>,
    }
    
    impl ::puroro_internal::GetImplStruct for CodeGeneratorResponse::<::puroro::tags::SimpleImpl> {
        type Type = CodeGeneratorResponse_SimpleImpl;
    }
    
    impl ::puroro::Message for CodeGeneratorResponse_SimpleImpl {}
    
    impl self::puroro_root::google::protobuf::compiler::puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponse_SimpleImpl {
        /// optional string error = 1;
        fn error<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional uint64 supported_features = 2;
        fn supported_features<'this>(&'this self) -> ::std::option::Option<u64> {
            todo!()
        }
        type Field15MessageType<'this> = self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>;
        type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>;
    
        /// repeated File file = 15;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.file)
        }
    }
    
    impl ::puroro::DeserFromBytesIter for CodeGeneratorResponse_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }
    
    impl ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorResponse_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.error, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt64
                >::deser_field(&mut self.supported_features, data),
                15 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.file, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }
    
    impl ::puroro::SerToIoWrite for CodeGeneratorResponse_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::ser_field(&self.error, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::ser_field(&self.supported_features, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.file, 15, out)?;
            ::std::result::Result::Ok(())
        }
    }

} // mod puroro_structs
pub type Version<ImplTag = ::puroro::tags::SimpleImpl> = self::puroro_structs::Version<ImplTag>;
pub type CodeGeneratorRequest<ImplTag = ::puroro::tags::SimpleImpl> = self::puroro_structs::CodeGeneratorRequest<ImplTag>;
pub type CodeGeneratorResponse<ImplTag = ::puroro::tags::SimpleImpl> = self::puroro_structs::CodeGeneratorResponse<ImplTag>;

pub mod puroro_traits {
    mod puroro_root {
        pub use super::super::puroro_root::*;
    }
    
    
    pub trait VersionTrait: ::std::clone::Clone {
        /// optional int32 major = 1;
        fn major<'this>(&'this self) -> ::std::option::Option<i32>;
        /// optional int32 minor = 2;
        fn minor<'this>(&'this self) -> ::std::option::Option<i32>;
        /// optional int32 patch = 3;
        fn patch<'this>(&'this self) -> ::std::option::Option<i32>;
        /// optional string suffix = 4;
        fn suffix<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
    }
    
    
    pub trait CodeGeneratorRequestTrait: ::std::clone::Clone {
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, str>>;
    
        /// repeated string file_to_generate = 1;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
        /// optional string parameter = 2;
        fn parameter<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field15MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait;
        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, Self::Field15MessageType<'this>>>;
    
        /// repeated FileDescriptorProto proto_file = 15;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
        type Field3MessageType<'this>: 'this + self::puroro_root::google::protobuf::compiler::puroro_traits::VersionTrait;
        /// optional Version compiler_version = 3;
        fn compiler_version<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
    }
    
    
    pub trait CodeGeneratorResponseTrait: ::std::clone::Clone {
        /// optional string error = 1;
        fn error<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional uint64 supported_features = 2;
        fn supported_features<'this>(&'this self) -> ::std::option::Option<u64>;
        type Field15MessageType<'this>: 'this + self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait;
        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this, ::std::borrow::Cow<'this, Self::Field15MessageType<'this>>>;
    
        /// repeated File file = 15;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
    }

} // mod puroro_traits

pub use self::puroro_traits::*;

pub mod puroro_nested {

pub mod code_generator_response {
mod puroro_root {
    pub use super::super::super::puroro_root::*;
}


pub mod puroro_structs {

    pub mod puroro_root {
        pub use super::super::puroro_root::*;
    }
    
    #[allow(unused)]
    pub struct File<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where Self: ::puroro_internal::GetImplStruct;
    
    impl<ImplTag> ::puroro::Message for File<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message
    {
    }
    
    impl<ImplTag> self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait for File<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait
            >::name(&self.0)
        }
        /// optional string insertion_point = 2;
        fn insertion_point<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait
            >::insertion_point(&self.0)
        }
        /// optional string content = 15;
        fn content<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait
            >::content(&self.0)
        }
        type Field16MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait
        >::Field16MessageType::<'this>;
        /// optional GeneratedCodeInfo generated_code_info = 16;
        fn generated_code_info<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field16MessageType<'this>>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait
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
    
    
    #[allow(unused)]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Default, PartialEq, Debug)]
    pub struct File_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub insertion_point: ::std::option::Option<::std::string::String>,
        pub content: ::std::option::Option<::std::string::String>,
        pub generated_code_info: ::std::option::Option<::std::boxed::Box<self::puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>>>,
    }
    
    impl ::puroro_internal::GetImplStruct for File::<::puroro::tags::SimpleImpl> {
        type Type = File_SimpleImpl;
    }
    
    impl ::puroro::Message for File_SimpleImpl {}
    
    impl self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::puroro_traits::FileTrait for File_SimpleImpl {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string insertion_point = 2;
        fn insertion_point<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string content = 15;
        fn content<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field16MessageType<'this> = self::puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>;
        /// optional GeneratedCodeInfo generated_code_info = 16;
        fn generated_code_info<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field16MessageType<'this>>> {
            todo!()
        }
    }
    
    impl ::puroro::DeserFromBytesIter for File_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }
    
    impl ::puroro_internal::de::DeserFieldsFromBytesIter for File_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.insertion_point, data),
                15 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.content, data),
                16 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.generated_code_info, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }
    
    impl ::puroro::SerToIoWrite for File_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::ser_field(&self.insertion_point, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::ser_field(&self.content, 15, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::puroro_root::google::protobuf::GeneratedCodeInfo<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.generated_code_info, 16, out)?;
            ::std::result::Result::Ok(())
        }
    }

} // mod puroro_structs
pub type File<ImplTag = ::puroro::tags::SimpleImpl> = self::puroro_structs::File<ImplTag>;

pub mod puroro_traits {
    mod puroro_root {
        pub use super::super::puroro_root::*;
    }
    
    
    pub trait FileTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string insertion_point = 2;
        fn insertion_point<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string content = 15;
        fn content<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field16MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::GeneratedCodeInfoTrait;
        /// optional GeneratedCodeInfo generated_code_info = 16;
        fn generated_code_info<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field16MessageType<'this>>>;
    }

} // mod puroro_traits

pub use self::puroro_traits::*;


#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    FeatureNone,
    FeatureProto3Optional,
}
impl ::puroro::Enum2 for Feature {
}
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



} // mod code_generator_response

} // mod puroro_nested

pub use self::puroro_nested::*;