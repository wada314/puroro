// A generated source code by puroro library
pub mod compiler;
// package google.protobuf

pub mod puroro_root {
    pub use super::super::super::*;
}

pub mod puroro_structs {

    pub mod puroro_root {
        pub use super::super::puroro_root::*;
    }

    #[allow(unused)]
    pub struct FileDescriptorSet<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::FileDescriptorSetTrait
        for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::FileDescriptorSetTrait,
    {
        type Field1MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorSetTrait
        >::Field1MessageType::<'this>;
        type Field1RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorSetTrait
        >::Field1RepeatedType::<'this>;

        /// repeated FileDescriptorProto file = 1;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorSetTrait
            >::file(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for FileDescriptorSet<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for FileDescriptorSet<ImplTag>
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
    pub struct FileDescriptorSet_SimpleImpl {
        pub file: ::std::vec::Vec<
            self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for FileDescriptorSet<::puroro::tags::SimpleImpl> {
        type Type = FileDescriptorSet_SimpleImpl;
    }

    impl ::puroro::Message for FileDescriptorSet_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::FileDescriptorSetTrait
        for FileDescriptorSet_SimpleImpl
    {
        type Field1MessageType<'this> =
            self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::FileDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated FileDescriptorProto file = 1;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.file)
        }
    }

    impl ::puroro::DeserFromBytesIter for FileDescriptorSet_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FileDescriptorSet_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::FileDescriptorProto<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.file, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for FileDescriptorSet_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::FileDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.file, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct FileDescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::name(&self.0)
        }
        /// optional string package = 2;
        fn package<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::package(&self.0)
        }
        type Field3RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field3RepeatedType::<'this>;

        /// repeated string dependency = 3;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::dependency(&self.0)
        }
        type Field10RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field10RepeatedType::<'this>;

        /// repeated int32 public_dependency = 10;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::public_dependency(&self.0)
        }
        type Field11RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field11RepeatedType::<'this>;

        /// repeated int32 weak_dependency = 11;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::weak_dependency(&self.0)
        }
        type Field4MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field4MessageType::<'this>;
        type Field4RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field4RepeatedType::<'this>;

        /// repeated DescriptorProto message_type = 4;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::message_type(&self.0)
        }
        type Field5MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field5MessageType::<'this>;
        type Field5RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field5RepeatedType::<'this>;

        /// repeated EnumDescriptorProto enum_type = 5;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::enum_type(&self.0)
        }
        type Field6MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field6MessageType::<'this>;
        type Field6RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field6RepeatedType::<'this>;

        /// repeated ServiceDescriptorProto service = 6;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::service(&self.0)
        }
        type Field7MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field7MessageType::<'this>;
        type Field7RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field7RepeatedType::<'this>;

        /// repeated FieldDescriptorProto extension = 7;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::extension(&self.0)
        }
        type Field8MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field8MessageType::<'this>;
        /// optional FileOptions options = 8;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::options(&self.0)
        }
        type Field9MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        >::Field9MessageType::<'this>;
        /// optional SourceCodeInfo source_code_info = 9;
        fn source_code_info<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field9MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::source_code_info(&self.0)
        }
        /// optional string syntax = 12;
        fn syntax<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
            >::syntax(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for FileDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for FileDescriptorProto<ImplTag>
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
    pub struct FileDescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub package: ::std::option::Option<::std::string::String>,
        pub dependency: ::std::vec::Vec<::std::string::String>,
        pub public_dependency: ::std::vec::Vec<i32>,
        pub weak_dependency: ::std::vec::Vec<i32>,
        pub message_type: ::std::vec::Vec<
            self::puroro_root::google::protobuf::DescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub enum_type: ::std::vec::Vec<
            self::puroro_root::google::protobuf::EnumDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub service: ::std::vec::Vec<
            self::puroro_root::google::protobuf::ServiceDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub extension: ::std::vec::Vec<
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::FileOptions<::puroro::tags::SimpleImpl>,
            >,
        >,
        pub source_code_info: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::SourceCodeInfo<::puroro::tags::SimpleImpl>,
            >,
        >,
        pub syntax: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro_internal::GetImplStruct for FileDescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = FileDescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for FileDescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string package = 2;
        fn package<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;

        /// repeated string dependency = 3;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.dependency)
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

        /// repeated int32 public_dependency = 10;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.public_dependency)
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;

        /// repeated int32 weak_dependency = 11;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.weak_dependency)
        }
        type Field4MessageType<'this> =
            self::puroro_root::google::protobuf::DescriptorProto<::puroro::tags::SimpleImpl>;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::DescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated DescriptorProto message_type = 4;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.message_type)
        }
        type Field5MessageType<'this> =
            self::puroro_root::google::protobuf::EnumDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::EnumDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated EnumDescriptorProto enum_type = 5;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.enum_type)
        }
        type Field6MessageType<'this> =
            self::puroro_root::google::protobuf::ServiceDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::ServiceDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated ServiceDescriptorProto service = 6;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.service)
        }
        type Field7MessageType<'this> =
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field7RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated FieldDescriptorProto extension = 7;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.extension)
        }
        type Field8MessageType<'this> =
            self::puroro_root::google::protobuf::FileOptions<::puroro::tags::SimpleImpl>;
        /// optional FileOptions options = 8;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>
        {
            todo!()
        }
        type Field9MessageType<'this> =
            self::puroro_root::google::protobuf::SourceCodeInfo<::puroro::tags::SimpleImpl>;
        /// optional SourceCodeInfo source_code_info = 9;
        fn source_code_info<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field9MessageType<'this>>>
        {
            todo!()
        }
        /// optional string syntax = 12;
        fn syntax<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
    }

    impl ::puroro::DeserFromBytesIter for FileDescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FileDescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.package, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::String,
                >::deser_field(&mut self.dependency, data),
                10 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.public_dependency, data),
                11 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.weak_dependency, data),
                4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::DescriptorProto<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.message_type, data),
                5 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::EnumDescriptorProto<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.enum_type, data),
                6 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::ServiceDescriptorProto<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.service, data),
                7 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::FieldDescriptorProto<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.extension, data),
                8 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::FileOptions<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.options, data),
                9 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::SourceCodeInfo<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.source_code_info, data),
                12 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.syntax, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for FileDescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.package, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.dependency, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.public_dependency, 10, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.weak_dependency, 11, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::DescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.message_type, 4, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::EnumDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.enum_type, 5, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::ServiceDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.service, 6, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::FieldDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.extension, 7, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::FileOptions<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.options, 8, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::SourceCodeInfo<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.source_code_info, 9, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.syntax, 12, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct DescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::name(&self.0)
        }
        type Field2MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field2MessageType::<'this>;
        type Field2RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field2RepeatedType::<'this>;

        /// repeated FieldDescriptorProto field = 2;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::field(&self.0)
        }
        type Field6MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field6MessageType::<'this>;
        type Field6RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field6RepeatedType::<'this>;

        /// repeated FieldDescriptorProto extension = 6;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::extension(&self.0)
        }
        type Field3MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field3MessageType::<'this>;
        type Field3RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field3RepeatedType::<'this>;

        /// repeated DescriptorProto nested_type = 3;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::nested_type(&self.0)
        }
        type Field4MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field4MessageType::<'this>;
        type Field4RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field4RepeatedType::<'this>;

        /// repeated EnumDescriptorProto enum_type = 4;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::enum_type(&self.0)
        }
        type Field5MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field5MessageType::<'this>;
        type Field5RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field5RepeatedType::<'this>;

        /// repeated ExtensionRange extension_range = 5;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::extension_range(&self.0)
        }
        type Field8MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field8MessageType::<'this>;
        type Field8RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field8RepeatedType::<'this>;

        /// repeated OneofDescriptorProto oneof_decl = 8;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::oneof_decl(&self.0)
        }
        type Field7MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field7MessageType::<'this>;
        /// optional MessageOptions options = 7;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::options(&self.0)
        }
        type Field9MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field9MessageType::<'this>;
        type Field9RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field9RepeatedType::<'this>;

        /// repeated ReservedRange reserved_range = 9;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::reserved_range(&self.0)
        }
        type Field10RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        >::Field10RepeatedType::<'this>;

        /// repeated string reserved_name = 10;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
            >::reserved_name(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for DescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for DescriptorProto<ImplTag>
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
    pub struct DescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub field: ::std::vec::Vec<
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub extension: ::std::vec::Vec<
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub nested_type: ::std::vec::Vec<
            self::puroro_root::google::protobuf::DescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub enum_type: ::std::vec::Vec<
            self::puroro_root::google::protobuf::EnumDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub extension_range: ::std::vec::Vec<
            self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<
                ::puroro::tags::SimpleImpl,
            >,
        >,
        pub oneof_decl: ::std::vec::Vec<
            self::puroro_root::google::protobuf::OneofDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::MessageOptions<::puroro::tags::SimpleImpl>,
            >,
        >,
        pub reserved_range: ::std::vec::Vec<
            self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<
                ::puroro::tags::SimpleImpl,
            >,
        >,
        pub reserved_name: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro_internal::GetImplStruct for DescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = DescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for DescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait
        for DescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field2MessageType<'this> =
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated FieldDescriptorProto field = 2;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.field)
        }
        type Field6MessageType<'this> =
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated FieldDescriptorProto extension = 6;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.extension)
        }
        type Field3MessageType<'this> =
            self::puroro_root::google::protobuf::DescriptorProto<::puroro::tags::SimpleImpl>;
        type Field3RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::DescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated DescriptorProto nested_type = 3;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.nested_type)
        }
        type Field4MessageType<'this> =
            self::puroro_root::google::protobuf::EnumDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::EnumDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated EnumDescriptorProto enum_type = 4;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.enum_type)
        }
        type Field5MessageType<'this> =
            self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<
                ::puroro::tags::SimpleImpl,
            >;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<
                ::puroro::tags::SimpleImpl,
            >,
        >;

        /// repeated ExtensionRange extension_range = 5;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.extension_range)
        }
        type Field8MessageType<'this> =
            self::puroro_root::google::protobuf::OneofDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::OneofDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated OneofDescriptorProto oneof_decl = 8;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.oneof_decl)
        }
        type Field7MessageType<'this> =
            self::puroro_root::google::protobuf::MessageOptions<::puroro::tags::SimpleImpl>;
        /// optional MessageOptions options = 7;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>
        {
            todo!()
        }
        type Field9MessageType<'this> =
            self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<
                ::puroro::tags::SimpleImpl,
            >;
        type Field9RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<
                ::puroro::tags::SimpleImpl,
            >,
        >;

        /// repeated ReservedRange reserved_range = 9;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.reserved_range)
        }
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;

        /// repeated string reserved_name = 10;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.reserved_name)
        }
    }

    impl ::puroro::DeserFromBytesIter for DescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for DescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.field, data),
                6 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::FieldDescriptorProto<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.extension, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::DescriptorProto<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.nested_type, data),
                4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::EnumDescriptorProto<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.enum_type, data),
                5 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.extension_range, data),
                8 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::OneofDescriptorProto<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.oneof_decl, data),
                7 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::puroro_root::google::protobuf::MessageOptions<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.options, data),
                9 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.reserved_range, data),
                10 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.reserved_name, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for DescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::FieldDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.field, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::FieldDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.extension, 6, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::DescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.nested_type, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::EnumDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.enum_type, 4, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.extension_range, 5, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::OneofDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.oneof_decl, 8, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::MessageOptions<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.options, 7, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.reserved_range, 9, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.reserved_name, 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct ExtensionRangeOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::ExtensionRangeOptionsTrait
        for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::ExtensionRangeOptionsTrait,
    {
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::ExtensionRangeOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::ExtensionRangeOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::ExtensionRangeOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for ExtensionRangeOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for ExtensionRangeOptions<ImplTag>
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
    pub struct ExtensionRangeOptions_SimpleImpl {
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for ExtensionRangeOptions<::puroro::tags::SimpleImpl> {
        type Type = ExtensionRangeOptions_SimpleImpl;
    }

    impl ::puroro::Message for ExtensionRangeOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::ExtensionRangeOptionsTrait
        for ExtensionRangeOptions_SimpleImpl
    {
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for ExtensionRangeOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for ExtensionRangeOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::UninterpretedOption<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for ExtensionRangeOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct FieldDescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::name(&self.0)
        }
        /// optional int32 number = 3;
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::number(&self.0)
        }
        /// optional Label label = 4;
        fn label<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label,
        > {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::label(&self.0)
        }
        /// optional Type type = 5;
        fn type_<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type,
        > {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::type_(&self.0)
        }
        /// optional string type_name = 6;
        fn type_name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::type_name(&self.0)
        }
        /// optional string extendee = 2;
        fn extendee<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::extendee(&self.0)
        }
        /// optional string default_value = 7;
        fn default_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::default_value(&self.0)
        }
        /// optional int32 oneof_index = 9;
        fn oneof_index<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::oneof_index(&self.0)
        }
        /// optional string json_name = 10;
        fn json_name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::json_name(&self.0)
        }
        type Field8MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
        >::Field8MessageType::<'this>;
        /// optional FieldOptions options = 8;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::options(&self.0)
        }
        /// optional bool proto3_optional = 17;
        fn proto3_optional<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
            >::proto3_optional(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for FieldDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for FieldDescriptorProto<ImplTag>
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
    pub struct FieldDescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub number: ::std::option::Option<i32>,
        pub label: ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label,
        >,
        pub type_: ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type,
        >,
        pub type_name: ::std::option::Option<::std::string::String>,
        pub extendee: ::std::option::Option<::std::string::String>,
        pub default_value: ::std::option::Option<::std::string::String>,
        pub oneof_index: ::std::option::Option<i32>,
        pub json_name: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::FieldOptions<::puroro::tags::SimpleImpl>,
            >,
        >,
        pub proto3_optional: ::std::option::Option<bool>,
    }

    impl ::puroro_internal::GetImplStruct for FieldDescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = FieldDescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for FieldDescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional int32 number = 3;
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional Label label = 4;
        fn label<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label,
        > {
            todo!()
        }
        /// optional Type type = 5;
        fn type_<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type,
        > {
            todo!()
        }
        /// optional string type_name = 6;
        fn type_name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string extendee = 2;
        fn extendee<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string default_value = 7;
        fn default_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional int32 oneof_index = 9;
        fn oneof_index<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional string json_name = 10;
        fn json_name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field8MessageType<'this> =
            self::puroro_root::google::protobuf::FieldOptions<::puroro::tags::SimpleImpl>;
        /// optional FieldOptions options = 8;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>
        {
            todo!()
        }
        /// optional bool proto3_optional = 17;
        fn proto3_optional<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
    }

    impl ::puroro::DeserFromBytesIter for FieldDescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FieldDescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.name, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.number, data),
                4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Enum2<self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label>
                >::deser_field(&mut self.label, data),
                5 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Enum2<self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type>
                >::deser_field(&mut self.type_, data),
                6 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.type_name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.extendee, data),
                7 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.default_value, data),
                9 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::deser_field(&mut self.oneof_index, data),
                10 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.json_name, data),
                8 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::puroro_root::google::protobuf::FieldOptions<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.options, data),
                17 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.proto3_optional, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for FieldDescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.number, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label>
            >::ser_field(&self.label, 4, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type>
            >::ser_field(&self.type_, 5, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.type_name, 6, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.extendee, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.default_value, 7, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.oneof_index, 9, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.json_name, 10, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::FieldOptions<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.options, 8, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.proto3_optional, 17, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct OneofDescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::OneofDescriptorProtoTrait
        for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::OneofDescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::OneofDescriptorProtoTrait
            >::name(&self.0)
        }
        type Field2MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::OneofDescriptorProtoTrait
        >::Field2MessageType::<'this>;
        /// optional OneofOptions options = 2;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field2MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::OneofDescriptorProtoTrait
            >::options(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for OneofDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for OneofDescriptorProto<ImplTag>
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
    pub struct OneofDescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::OneofOptions<::puroro::tags::SimpleImpl>,
            >,
        >,
    }

    impl ::puroro_internal::GetImplStruct for OneofDescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = OneofDescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for OneofDescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::OneofDescriptorProtoTrait
        for OneofDescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field2MessageType<'this> =
            self::puroro_root::google::protobuf::OneofOptions<::puroro::tags::SimpleImpl>;
        /// optional OneofOptions options = 2;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field2MessageType<'this>>>
        {
            todo!()
        }
    }

    impl ::puroro::DeserFromBytesIter for OneofDescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for OneofDescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::OneofOptions<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.options, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for OneofDescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::OneofOptions<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.options, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct EnumDescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
            >::name(&self.0)
        }
        type Field2MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        >::Field2MessageType::<'this>;
        type Field2RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        >::Field2RepeatedType::<'this>;

        /// repeated EnumValueDescriptorProto value = 2;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
            >::value(&self.0)
        }
        type Field3MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        >::Field3MessageType::<'this>;
        /// optional EnumOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
            >::options(&self.0)
        }
        type Field4MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        >::Field4MessageType::<'this>;
        type Field4RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        >::Field4RepeatedType::<'this>;

        /// repeated EnumReservedRange reserved_range = 4;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
            >::reserved_range(&self.0)
        }
        type Field5RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        >::Field5RepeatedType::<'this>;

        /// repeated string reserved_name = 5;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
            >::reserved_name(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for EnumDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for EnumDescriptorProto<ImplTag>
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
    pub struct EnumDescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub value: ::std::vec::Vec<self::puroro_root::google::protobuf::EnumValueDescriptorProto<::puroro::tags::SimpleImpl>>,
        pub options: ::std::option::Option<::std::boxed::Box<self::puroro_root::google::protobuf::EnumOptions<::puroro::tags::SimpleImpl>>>,
        pub reserved_range: ::std::vec::Vec<self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<::puroro::tags::SimpleImpl>>,
        pub reserved_name: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro_internal::GetImplStruct for EnumDescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = EnumDescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for EnumDescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait
        for EnumDescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field2MessageType<'this> =
            self::puroro_root::google::protobuf::EnumValueDescriptorProto<
                ::puroro::tags::SimpleImpl,
            >;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::EnumValueDescriptorProto<
                ::puroro::tags::SimpleImpl,
            >,
        >;

        /// repeated EnumValueDescriptorProto value = 2;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.value)
        }
        type Field3MessageType<'this> =
            self::puroro_root::google::protobuf::EnumOptions<::puroro::tags::SimpleImpl>;
        /// optional EnumOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            todo!()
        }
        type Field4MessageType<'this> = self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<::puroro::tags::SimpleImpl>;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<::puroro::tags::SimpleImpl>>;

        /// repeated EnumReservedRange reserved_range = 4;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.reserved_range)
        }
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;

        /// repeated string reserved_name = 5;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.reserved_name)
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumDescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumDescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::EnumValueDescriptorProto<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.value, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::puroro_root::google::protobuf::EnumOptions<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.options, data),
                4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.reserved_range, data),
                5 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::deser_field(&mut self.reserved_name, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for EnumDescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::EnumValueDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.value, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::EnumOptions<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.reserved_range, 4, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.reserved_name, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct EnumValueDescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait
        for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait
            >::name(&self.0)
        }
        /// optional int32 number = 2;
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait
            >::number(&self.0)
        }
        type Field3MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait
        >::Field3MessageType::<'this>;
        /// optional EnumValueOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait
            >::options(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for EnumValueDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for EnumValueDescriptorProto<ImplTag>
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
    pub struct EnumValueDescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub number: ::std::option::Option<i32>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::EnumValueOptions<::puroro::tags::SimpleImpl>,
            >,
        >,
    }

    impl ::puroro_internal::GetImplStruct for EnumValueDescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = EnumValueDescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for EnumValueDescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait
        for EnumValueDescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional int32 number = 2;
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        type Field3MessageType<'this> =
            self::puroro_root::google::protobuf::EnumValueOptions<::puroro::tags::SimpleImpl>;
        /// optional EnumValueOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            todo!()
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumValueDescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumValueDescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.number, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::EnumValueOptions<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.options, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for EnumValueDescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.number, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::EnumValueOptions<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct ServiceDescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
        for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
            >::name(&self.0)
        }
        type Field2MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
        >::Field2MessageType::<'this>;
        type Field2RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
        >::Field2RepeatedType::<'this>;

        /// repeated MethodDescriptorProto method = 2;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
            >::method(&self.0)
        }
        type Field3MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
        >::Field3MessageType::<'this>;
        /// optional ServiceOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
            >::options(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for ServiceDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for ServiceDescriptorProto<ImplTag>
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
    pub struct ServiceDescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub method: ::std::vec::Vec<
            self::puroro_root::google::protobuf::MethodDescriptorProto<::puroro::tags::SimpleImpl>,
        >,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::ServiceOptions<::puroro::tags::SimpleImpl>,
            >,
        >,
    }

    impl ::puroro_internal::GetImplStruct for ServiceDescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = ServiceDescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for ServiceDescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait
        for ServiceDescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field2MessageType<'this> =
            self::puroro_root::google::protobuf::MethodDescriptorProto<::puroro::tags::SimpleImpl>;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::MethodDescriptorProto<::puroro::tags::SimpleImpl>,
        >;

        /// repeated MethodDescriptorProto method = 2;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.method)
        }
        type Field3MessageType<'this> =
            self::puroro_root::google::protobuf::ServiceOptions<::puroro::tags::SimpleImpl>;
        /// optional ServiceOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>
        {
            todo!()
        }
    }

    impl ::puroro::DeserFromBytesIter for ServiceDescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for ServiceDescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::MethodDescriptorProto<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.method, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::ServiceOptions<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.options, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for ServiceDescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::MethodDescriptorProto<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.method, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::ServiceOptions<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct MethodDescriptorProto<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
        for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait,
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
            >::name(&self.0)
        }
        /// optional string input_type = 2;
        fn input_type<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
            >::input_type(&self.0)
        }
        /// optional string output_type = 3;
        fn output_type<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
            >::output_type(&self.0)
        }
        type Field4MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
        >::Field4MessageType::<'this>;
        /// optional MethodOptions options = 4;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field4MessageType<'this>>>
        {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
            >::options(&self.0)
        }
        /// optional bool client_streaming = 5;
        fn client_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
            >::client_streaming(&self.0)
        }
        /// optional bool server_streaming = 6;
        fn server_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
            >::server_streaming(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for MethodDescriptorProto<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for MethodDescriptorProto<ImplTag>
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
    pub struct MethodDescriptorProto_SimpleImpl {
        pub name: ::std::option::Option<::std::string::String>,
        pub input_type: ::std::option::Option<::std::string::String>,
        pub output_type: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::puroro_root::google::protobuf::MethodOptions<::puroro::tags::SimpleImpl>,
            >,
        >,
        pub client_streaming: ::std::option::Option<bool>,
        pub server_streaming: ::std::option::Option<bool>,
    }

    impl ::puroro_internal::GetImplStruct for MethodDescriptorProto<::puroro::tags::SimpleImpl> {
        type Type = MethodDescriptorProto_SimpleImpl;
    }

    impl ::puroro::Message for MethodDescriptorProto_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait
        for MethodDescriptorProto_SimpleImpl
    {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string input_type = 2;
        fn input_type<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string output_type = 3;
        fn output_type<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field4MessageType<'this> =
            self::puroro_root::google::protobuf::MethodOptions<::puroro::tags::SimpleImpl>;
        /// optional MethodOptions options = 4;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field4MessageType<'this>>>
        {
            todo!()
        }
        /// optional bool client_streaming = 5;
        fn client_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool server_streaming = 6;
        fn server_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
    }

    impl ::puroro::DeserFromBytesIter for MethodDescriptorProto_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MethodDescriptorProto_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.input_type, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.output_type, data),
                4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::MethodOptions<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.options, data),
                5 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.client_streaming, data),
                6 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.server_streaming, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for MethodDescriptorProto_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.input_type, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.output_type, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::MethodOptions<::puroro::tags::SimpleImpl>,
                >,
            >::ser_field(&self.options, 4, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.client_streaming, 5, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.server_streaming, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct FileOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
        for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait,
    {
        /// optional string java_package = 1;
        fn java_package<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::java_package(&self.0)
        }
        /// optional string java_outer_classname = 8;
        fn java_outer_classname<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::java_outer_classname(&self.0)
        }
        /// optional bool java_multiple_files = 10;
        fn java_multiple_files<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::java_multiple_files(&self.0)
        }
        /// optional bool java_generate_equals_and_hash = 20;
        fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::java_generate_equals_and_hash(&self.0)
        }
        /// optional bool java_string_check_utf8 = 27;
        fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::java_string_check_utf8(&self.0)
        }
        /// optional OptimizeMode optimize_for = 9;
        fn optimize_for<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
        > {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::optimize_for(&self.0)
        }
        /// optional string go_package = 11;
        fn go_package<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::go_package(&self.0)
        }
        /// optional bool cc_generic_services = 16;
        fn cc_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::cc_generic_services(&self.0)
        }
        /// optional bool java_generic_services = 17;
        fn java_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::java_generic_services(&self.0)
        }
        /// optional bool py_generic_services = 18;
        fn py_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::py_generic_services(&self.0)
        }
        /// optional bool php_generic_services = 42;
        fn php_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::php_generic_services(&self.0)
        }
        /// optional bool deprecated = 23;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::deprecated(&self.0)
        }
        /// optional bool cc_enable_arenas = 31;
        fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::cc_enable_arenas(&self.0)
        }
        /// optional string objc_class_prefix = 36;
        fn objc_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::objc_class_prefix(&self.0)
        }
        /// optional string csharp_namespace = 37;
        fn csharp_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::csharp_namespace(&self.0)
        }
        /// optional string swift_prefix = 39;
        fn swift_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::swift_prefix(&self.0)
        }
        /// optional string php_class_prefix = 40;
        fn php_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::php_class_prefix(&self.0)
        }
        /// optional string php_namespace = 41;
        fn php_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::php_namespace(&self.0)
        }
        /// optional string php_metadata_namespace = 44;
        fn php_metadata_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::php_metadata_namespace(&self.0)
        }
        /// optional string ruby_package = 45;
        fn ruby_package<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::ruby_package(&self.0)
        }
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for FileOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for FileOptions<ImplTag>
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
    pub struct FileOptions_SimpleImpl {
        pub java_package: ::std::option::Option<::std::string::String>,
        pub java_outer_classname: ::std::option::Option<::std::string::String>,
        pub java_multiple_files: ::std::option::Option<bool>,
        pub java_generate_equals_and_hash: ::std::option::Option<bool>,
        pub java_string_check_utf8: ::std::option::Option<bool>,
        pub optimize_for: ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
        >,
        pub go_package: ::std::option::Option<::std::string::String>,
        pub cc_generic_services: ::std::option::Option<bool>,
        pub java_generic_services: ::std::option::Option<bool>,
        pub py_generic_services: ::std::option::Option<bool>,
        pub php_generic_services: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub cc_enable_arenas: ::std::option::Option<bool>,
        pub objc_class_prefix: ::std::option::Option<::std::string::String>,
        pub csharp_namespace: ::std::option::Option<::std::string::String>,
        pub swift_prefix: ::std::option::Option<::std::string::String>,
        pub php_class_prefix: ::std::option::Option<::std::string::String>,
        pub php_namespace: ::std::option::Option<::std::string::String>,
        pub php_metadata_namespace: ::std::option::Option<::std::string::String>,
        pub ruby_package: ::std::option::Option<::std::string::String>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for FileOptions<::puroro::tags::SimpleImpl> {
        type Type = FileOptions_SimpleImpl;
    }

    impl ::puroro::Message for FileOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait
        for FileOptions_SimpleImpl
    {
        /// optional string java_package = 1;
        fn java_package<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string java_outer_classname = 8;
        fn java_outer_classname<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional bool java_multiple_files = 10;
        fn java_multiple_files<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool java_generate_equals_and_hash = 20;
        fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool java_string_check_utf8 = 27;
        fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional OptimizeMode optimize_for = 9;
        fn optimize_for<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
        > {
            todo!()
        }
        /// optional string go_package = 11;
        fn go_package<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional bool cc_generic_services = 16;
        fn cc_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool java_generic_services = 17;
        fn java_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool py_generic_services = 18;
        fn py_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool php_generic_services = 42;
        fn php_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool deprecated = 23;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool cc_enable_arenas = 31;
        fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional string objc_class_prefix = 36;
        fn objc_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string csharp_namespace = 37;
        fn csharp_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string swift_prefix = 39;
        fn swift_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string php_class_prefix = 40;
        fn php_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string php_namespace = 41;
        fn php_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string php_metadata_namespace = 44;
        fn php_metadata_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string ruby_package = 45;
        fn ruby_package<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for FileOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FileOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.java_package, data),
                8 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.java_outer_classname, data),
                10 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.java_multiple_files, data),
                20 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.java_generate_equals_and_hash, data),
                27 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.java_string_check_utf8, data),
                9 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Enum2<self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode>
                >::deser_field(&mut self.optimize_for, data),
                11 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.go_package, data),
                16 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.cc_generic_services, data),
                17 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.java_generic_services, data),
                18 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.py_generic_services, data),
                42 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.php_generic_services, data),
                23 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.deprecated, data),
                31 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.cc_enable_arenas, data),
                36 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.objc_class_prefix, data),
                37 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.csharp_namespace, data),
                39 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.swift_prefix, data),
                40 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.php_class_prefix, data),
                41 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.php_namespace, data),
                44 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.php_metadata_namespace, data),
                45 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.ruby_package, data),
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for FileOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.java_package, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.java_outer_classname, 8, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_multiple_files, 10, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_generate_equals_and_hash, 20, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_string_check_utf8, 27, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
                >,
            >::ser_field(&self.optimize_for, 9, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.go_package, 11, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.cc_generic_services, 16, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_generic_services, 17, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.py_generic_services, 18, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.php_generic_services, 42, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 23, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.cc_enable_arenas, 31, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.objc_class_prefix, 36, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.csharp_namespace, 37, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.swift_prefix, 39, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.php_class_prefix, 40, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.php_namespace, 41, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.php_metadata_namespace, 44, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.ruby_package, 45, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct MessageOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
        for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait,
    {
        /// optional bool message_set_wire_format = 1;
        fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
            >::message_set_wire_format(&self.0)
        }
        /// optional bool no_standard_descriptor_accessor = 2;
        fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
            >::no_standard_descriptor_accessor(&self.0)
        }
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
            >::deprecated(&self.0)
        }
        /// optional bool map_entry = 7;
        fn map_entry<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
            >::map_entry(&self.0)
        }
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for MessageOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for MessageOptions<ImplTag>
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
    pub struct MessageOptions_SimpleImpl {
        pub message_set_wire_format: ::std::option::Option<bool>,
        pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub map_entry: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for MessageOptions<::puroro::tags::SimpleImpl> {
        type Type = MessageOptions_SimpleImpl;
    }

    impl ::puroro::Message for MessageOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait
        for MessageOptions_SimpleImpl
    {
        /// optional bool message_set_wire_format = 1;
        fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool no_standard_descriptor_accessor = 2;
        fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool map_entry = 7;
        fn map_entry<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for MessageOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MessageOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.message_set_wire_format, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.no_standard_descriptor_accessor, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.deprecated, data),
                7 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.map_entry, data),
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::UninterpretedOption<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for MessageOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.message_set_wire_format, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.no_standard_descriptor_accessor, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.map_entry, 7, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct FieldOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
        for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait,
    {
        /// optional CType ctype = 1;
        fn ctype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
        > {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
            >::ctype(&self.0)
        }
        /// optional bool packed = 2;
        fn packed<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
            >::packed(&self.0)
        }
        /// optional JSType jstype = 6;
        fn jstype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
        > {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
            >::jstype(&self.0)
        }
        /// optional bool lazy = 5;
        fn lazy<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
            >::lazy(&self.0)
        }
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
            >::deprecated(&self.0)
        }
        /// optional bool weak = 10;
        fn weak<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
            >::weak(&self.0)
        }
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for FieldOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for FieldOptions<ImplTag>
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
    pub struct FieldOptions_SimpleImpl {
        pub ctype: ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
        >,
        pub packed: ::std::option::Option<bool>,
        pub jstype: ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
        >,
        pub lazy: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub weak: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for FieldOptions<::puroro::tags::SimpleImpl> {
        type Type = FieldOptions_SimpleImpl;
    }

    impl ::puroro::Message for FieldOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait
        for FieldOptions_SimpleImpl
    {
        /// optional CType ctype = 1;
        fn ctype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
        > {
            todo!()
        }
        /// optional bool packed = 2;
        fn packed<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional JSType jstype = 6;
        fn jstype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
        > {
            todo!()
        }
        /// optional bool lazy = 5;
        fn lazy<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool weak = 10;
        fn weak<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for FieldOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FieldOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Enum2<
                        self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
                    >,
                >::deser_field(&mut self.ctype, data),
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.packed, data),
                6 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Enum2<
                        self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
                    >,
                >::deser_field(&mut self.jstype, data),
                5 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.lazy, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.deprecated, data),
                10 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.weak, data),
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::UninterpretedOption<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for FieldOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
                >,
            >::ser_field(&self.ctype, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.packed, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
                >,
            >::ser_field(&self.jstype, 6, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.lazy, 5, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.weak, 10, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct OneofOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::OneofOptionsTrait
        for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::OneofOptionsTrait,
    {
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::OneofOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::OneofOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::OneofOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for OneofOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for OneofOptions<ImplTag>
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
    pub struct OneofOptions_SimpleImpl {
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for OneofOptions<::puroro::tags::SimpleImpl> {
        type Type = OneofOptions_SimpleImpl;
    }

    impl ::puroro::Message for OneofOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::OneofOptionsTrait
        for OneofOptions_SimpleImpl
    {
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for OneofOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for OneofOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::UninterpretedOption<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for OneofOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct EnumOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait
        for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait,
    {
        /// optional bool allow_alias = 2;
        fn allow_alias<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait
            >::allow_alias(&self.0)
        }
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait
            >::deprecated(&self.0)
        }
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for EnumOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for EnumOptions<ImplTag>
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
    pub struct EnumOptions_SimpleImpl {
        pub allow_alias: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for EnumOptions<::puroro::tags::SimpleImpl> {
        type Type = EnumOptions_SimpleImpl;
    }

    impl ::puroro::Message for EnumOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait
        for EnumOptions_SimpleImpl
    {
        /// optional bool allow_alias = 2;
        fn allow_alias<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.allow_alias, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.deprecated, data),
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::UninterpretedOption<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for EnumOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.allow_alias, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct EnumValueOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait
        for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait,
    {
        /// optional bool deprecated = 1;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait
            >::deprecated(&self.0)
        }
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for EnumValueOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for EnumValueOptions<ImplTag>
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
    pub struct EnumValueOptions_SimpleImpl {
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for EnumValueOptions<::puroro::tags::SimpleImpl> {
        type Type = EnumValueOptions_SimpleImpl;
    }

    impl ::puroro::Message for EnumValueOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait
        for EnumValueOptions_SimpleImpl
    {
        /// optional bool deprecated = 1;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumValueOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumValueOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.deprecated, data),
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::UninterpretedOption<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for EnumValueOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 1, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct ServiceOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait
        for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait,
    {
        /// optional bool deprecated = 33;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait
            >::deprecated(&self.0)
        }
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for ServiceOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for ServiceOptions<ImplTag>
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
    pub struct ServiceOptions_SimpleImpl {
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for ServiceOptions<::puroro::tags::SimpleImpl> {
        type Type = ServiceOptions_SimpleImpl;
    }

    impl ::puroro::Message for ServiceOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait
        for ServiceOptions_SimpleImpl
    {
        /// optional bool deprecated = 33;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for ServiceOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for ServiceOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                33 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.deprecated, data),
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Message<
                        self::puroro_root::google::protobuf::UninterpretedOption<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for ServiceOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 33, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct MethodOptions<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait
        for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait,
    {
        /// optional bool deprecated = 33;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait
            >::deprecated(&self.0)
        }
        /// optional IdempotencyLevel idempotency_level = 34;
        fn idempotency_level<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel,
        > {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait
            >::idempotency_level(&self.0)
        }
        type Field999MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait
        >::Field999MessageType::<'this>;
        type Field999RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait
        >::Field999RepeatedType::<'this>;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait
            >::uninterpreted_option(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for MethodOptions<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for MethodOptions<ImplTag>
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
    pub struct MethodOptions_SimpleImpl {
        pub deprecated: ::std::option::Option<bool>,
        pub idempotency_level: ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel,
        >,
        pub uninterpreted_option: ::std::vec::Vec<
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >,
    }

    impl ::puroro_internal::GetImplStruct for MethodOptions<::puroro::tags::SimpleImpl> {
        type Type = MethodOptions_SimpleImpl;
    }

    impl ::puroro::Message for MethodOptions_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait
        for MethodOptions_SimpleImpl
    {
        /// optional bool deprecated = 33;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
        /// optional IdempotencyLevel idempotency_level = 34;
        fn idempotency_level<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel,
        > {
            todo!()
        }
        type Field999MessageType<'this> =
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.uninterpreted_option)
        }
    }

    impl ::puroro::DeserFromBytesIter for MethodOptions_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MethodOptions_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                33 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bool
                >::deser_field(&mut self.deprecated, data),
                34 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Enum2<self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel>
                >::deser_field(&mut self.idempotency_level, data),
                999 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::UninterpretedOption<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.uninterpreted_option, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for MethodOptions_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 33, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel>
            >::ser_field(&self.idempotency_level, 34, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::UninterpretedOption<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct UninterpretedOption<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
        for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait,
    {
        type Field2MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
        >::Field2MessageType::<'this>;
        type Field2RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
        >::Field2RepeatedType::<'this>;

        /// repeated NamePart name = 2;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
            >::name(&self.0)
        }
        /// optional string identifier_value = 3;
        fn identifier_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
            >::identifier_value(&self.0)
        }
        /// optional uint64 positive_int_value = 4;
        fn positive_int_value<'this>(&'this self) -> ::std::option::Option<u64> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
            >::positive_int_value(&self.0)
        }
        /// optional int64 negative_int_value = 5;
        fn negative_int_value<'this>(&'this self) -> ::std::option::Option<i64> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
            >::negative_int_value(&self.0)
        }
        /// optional double double_value = 6;
        fn double_value<'this>(&'this self) -> ::std::option::Option<f64> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
            >::double_value(&self.0)
        }
        /// optional bytes string_value = 7;
        fn string_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, [u8]>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
            >::string_value(&self.0)
        }
        /// optional string aggregate_value = 8;
        fn aggregate_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
            >::aggregate_value(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for UninterpretedOption<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for UninterpretedOption<ImplTag>
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
    pub struct UninterpretedOption_SimpleImpl {
        pub name: ::std::vec::Vec<
            self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<
                ::puroro::tags::SimpleImpl,
            >,
        >,
        pub identifier_value: ::std::option::Option<::std::string::String>,
        pub positive_int_value: ::std::option::Option<u64>,
        pub negative_int_value: ::std::option::Option<i64>,
        pub double_value: ::std::option::Option<f64>,
        pub string_value: ::std::option::Option<::std::vec::Vec<u8>>,
        pub aggregate_value: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro_internal::GetImplStruct for UninterpretedOption<::puroro::tags::SimpleImpl> {
        type Type = UninterpretedOption_SimpleImpl;
    }

    impl ::puroro::Message for UninterpretedOption_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait
        for UninterpretedOption_SimpleImpl
    {
        type Field2MessageType<'this> =
            self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<
                ::puroro::tags::SimpleImpl,
            >;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<
                ::puroro::tags::SimpleImpl,
            >,
        >;

        /// repeated NamePart name = 2;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.name)
        }
        /// optional string identifier_value = 3;
        fn identifier_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional uint64 positive_int_value = 4;
        fn positive_int_value<'this>(&'this self) -> ::std::option::Option<u64> {
            todo!()
        }
        /// optional int64 negative_int_value = 5;
        fn negative_int_value<'this>(&'this self) -> ::std::option::Option<i64> {
            todo!()
        }
        /// optional double double_value = 6;
        fn double_value<'this>(&'this self) -> ::std::option::Option<f64> {
            todo!()
        }
        /// optional bytes string_value = 7;
        fn string_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, [u8]>> {
            todo!()
        }
        /// optional string aggregate_value = 8;
        fn aggregate_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
    }

    impl ::puroro::DeserFromBytesIter for UninterpretedOption_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for UninterpretedOption_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.name, data),
                3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.identifier_value, data),
                4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::UInt64
                >::deser_field(&mut self.positive_int_value, data),
                5 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Int64
                >::deser_field(&mut self.negative_int_value, data),
                6 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Double
                >::deser_field(&mut self.double_value, data),
                7 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::Bytes
                >::deser_field(&mut self.string_value, data),
                8 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::deser_field(&mut self.aggregate_value, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for UninterpretedOption_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.name, 2, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.identifier_value, 3, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(&self.positive_int_value, 4, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(&self.negative_int_value, 5, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >::ser_field(&self.double_value, 6, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(&self.string_value, 7, out)?;
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.aggregate_value, 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct SourceCodeInfo<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::SourceCodeInfoTrait
        for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::SourceCodeInfoTrait,
    {
        type Field1MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::SourceCodeInfoTrait
        >::Field1MessageType::<'this>;
        type Field1RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::SourceCodeInfoTrait
        >::Field1RepeatedType::<'this>;

        /// repeated Location location = 1;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::SourceCodeInfoTrait
            >::location(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for SourceCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for SourceCodeInfo<ImplTag>
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
    pub struct SourceCodeInfo_SimpleImpl {
        pub location: ::std::vec::Vec<
            self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                ::puroro::tags::SimpleImpl,
            >,
        >,
    }

    impl ::puroro_internal::GetImplStruct for SourceCodeInfo<::puroro::tags::SimpleImpl> {
        type Type = SourceCodeInfo_SimpleImpl;
    }

    impl ::puroro::Message for SourceCodeInfo_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::SourceCodeInfoTrait
        for SourceCodeInfo_SimpleImpl
    {
        type Field1MessageType<'this> =
            self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                ::puroro::tags::SimpleImpl,
            >;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                ::puroro::tags::SimpleImpl,
            >,
        >;

        /// repeated Location location = 1;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.location)
        }
    }

    impl ::puroro::DeserFromBytesIter for SourceCodeInfo_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for SourceCodeInfo_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.location, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for SourceCodeInfo_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                        ::puroro::tags::SimpleImpl,
                    >,
                >,
            >::ser_field(&self.location, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    #[allow(unused)]
    pub struct GeneratedCodeInfo<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
    where
        Self: ::puroro_internal::GetImplStruct;

    impl<ImplTag> ::puroro::Message for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
    {
    }

    impl<ImplTag> self::puroro_root::google::protobuf::puroro_traits::GeneratedCodeInfoTrait
        for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            self::puroro_root::google::protobuf::puroro_traits::GeneratedCodeInfoTrait,
    {
        type Field1MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::GeneratedCodeInfoTrait
        >::Field1MessageType::<'this>;
        type Field1RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_traits::GeneratedCodeInfoTrait
        >::Field1RepeatedType::<'this>;

        /// repeated Annotation annotation = 1;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_traits::GeneratedCodeInfoTrait
            >::annotation(&self.0)
        }
    }

    impl<ImplTag> ::puroro::DeserFromBytesIter for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
    {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
        }
    }
    impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type:
            ::puroro_internal::de::DeserFieldsFromBytesIter,
    {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                &mut self.0,
                field_number,
                data,
            )
        }
    }

    impl<ImplTag> ::puroro::SerToIoWrite for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro::SerToIoWrite::ser(&self.0, out)
        }
    }

    impl<ImplTag> ::std::ops::Deref for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<ImplTag> ::std::ops::DerefMut for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<ImplTag> ::std::clone::Clone for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
    {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<ImplTag> ::std::default::Default for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
    {
        fn default() -> Self {
            Self(::std::default::Default::default())
        }
    }
    impl<ImplTag> ::std::fmt::Debug for GeneratedCodeInfo<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
    {
        fn fmt(
            &self,
            formatter: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, formatter)
        }
    }
    impl<ImplTag> ::std::cmp::PartialEq for GeneratedCodeInfo<ImplTag>
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
    pub struct GeneratedCodeInfo_SimpleImpl {
        pub annotation: ::std::vec::Vec<
            self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<
                ::puroro::tags::SimpleImpl,
            >,
        >,
    }

    impl ::puroro_internal::GetImplStruct for GeneratedCodeInfo<::puroro::tags::SimpleImpl> {
        type Type = GeneratedCodeInfo_SimpleImpl;
    }

    impl ::puroro::Message for GeneratedCodeInfo_SimpleImpl {}

    impl self::puroro_root::google::protobuf::puroro_traits::GeneratedCodeInfoTrait
        for GeneratedCodeInfo_SimpleImpl
    {
        type Field1MessageType<'this> =
            self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<
                ::puroro::tags::SimpleImpl,
            >;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<
            'this,
            self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<
                ::puroro::tags::SimpleImpl,
            >,
        >;

        /// repeated Annotation annotation = 1;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.annotation)
        }
    }

    impl ::puroro::DeserFromBytesIter for GeneratedCodeInfo_SimpleImpl {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for GeneratedCodeInfo_SimpleImpl {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                    ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<::puroro::tags::SimpleImpl>>
                >::deser_field(&mut self.annotation, data),
                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for GeneratedCodeInfo_SimpleImpl {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<::puroro::tags::SimpleImpl>>
            >::ser_field(&self.annotation, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }
} // mod puroro_structs
pub type FileDescriptorSet<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::FileDescriptorSet<ImplTag>;
pub type FileDescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::FileDescriptorProto<ImplTag>;
pub type DescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::DescriptorProto<ImplTag>;
pub type ExtensionRangeOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::ExtensionRangeOptions<ImplTag>;
pub type FieldDescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::FieldDescriptorProto<ImplTag>;
pub type OneofDescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::OneofDescriptorProto<ImplTag>;
pub type EnumDescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::EnumDescriptorProto<ImplTag>;
pub type EnumValueDescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::EnumValueDescriptorProto<ImplTag>;
pub type ServiceDescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::ServiceDescriptorProto<ImplTag>;
pub type MethodDescriptorProto<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::MethodDescriptorProto<ImplTag>;
pub type FileOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::FileOptions<ImplTag>;
pub type MessageOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::MessageOptions<ImplTag>;
pub type FieldOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::FieldOptions<ImplTag>;
pub type OneofOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::OneofOptions<ImplTag>;
pub type EnumOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::EnumOptions<ImplTag>;
pub type EnumValueOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::EnumValueOptions<ImplTag>;
pub type ServiceOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::ServiceOptions<ImplTag>;
pub type MethodOptions<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::MethodOptions<ImplTag>;
pub type UninterpretedOption<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::UninterpretedOption<ImplTag>;
pub type SourceCodeInfo<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::SourceCodeInfo<ImplTag>;
pub type GeneratedCodeInfo<ImplTag = ::puroro::tags::SimpleImpl> =
    self::puroro_structs::GeneratedCodeInfo<ImplTag>;

pub mod puroro_traits {
    mod puroro_root {
        pub use super::super::puroro_root::*;
    }

    pub trait FileDescriptorSetTrait: ::std::clone::Clone {
        type Field1MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::FileDescriptorProtoTrait;
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field1MessageType<'this>>,
        >;

        /// repeated FileDescriptorProto file = 1;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    pub trait FileDescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string package = 2;
        fn package<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, str>,
        >;

        /// repeated string dependency = 3;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

        /// repeated int32 public_dependency = 10;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        type Field11RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

        /// repeated int32 weak_dependency = 11;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this>;
        type Field4MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field4MessageType<'this>>,
        >;

        /// repeated DescriptorProto message_type = 4;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field5MessageType<'this>>,
        >;

        /// repeated EnumDescriptorProto enum_type = 5;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field6MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::ServiceDescriptorProtoTrait;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field6MessageType<'this>>,
        >;

        /// repeated ServiceDescriptorProto service = 6;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait;
        type Field7RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field7MessageType<'this>>,
        >;

        /// repeated FieldDescriptorProto extension = 7;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this>;
        type Field8MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::FileOptionsTrait;
        /// optional FileOptions options = 8;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
        type Field9MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::SourceCodeInfoTrait;
        /// optional SourceCodeInfo source_code_info = 9;
        fn source_code_info<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field9MessageType<'this>>>;
        /// optional string syntax = 12;
        fn syntax<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
    }

    pub trait DescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field2MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field2MessageType<'this>>,
        >;

        /// repeated FieldDescriptorProto field = 2;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field6MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::FieldDescriptorProtoTrait;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field6MessageType<'this>>,
        >;

        /// repeated FieldDescriptorProto extension = 6;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field3MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::DescriptorProtoTrait;
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field3MessageType<'this>>,
        >;

        /// repeated DescriptorProto nested_type = 3;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        type Field4MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::EnumDescriptorProtoTrait;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field4MessageType<'this>>,
        >;

        /// repeated EnumDescriptorProto enum_type = 4;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field5MessageType<'this>>,
        >;

        /// repeated ExtensionRange extension_range = 5;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field8MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::OneofDescriptorProtoTrait;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field8MessageType<'this>>,
        >;

        /// repeated OneofDescriptorProto oneof_decl = 8;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        type Field7MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::MessageOptionsTrait;
        /// optional MessageOptions options = 7;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field7MessageType<'this>>>;
        type Field9MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ReservedRangeTrait;
        type Field9RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field9MessageType<'this>>,
        >;

        /// repeated ReservedRange reserved_range = 9;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this>;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, str>,
        >;

        /// repeated string reserved_name = 10;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
    }

    pub trait ExtensionRangeOptionsTrait: ::std::clone::Clone {
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait FieldDescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional int32 number = 3;
        fn number<'this>(&'this self) -> ::std::option::Option<i32>;
        /// optional Label label = 4;
        fn label<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label,
        >;
        /// optional Type type = 5;
        fn type_<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type,
        >;
        /// optional string type_name = 6;
        fn type_name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string extendee = 2;
        fn extendee<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string default_value = 7;
        fn default_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional int32 oneof_index = 9;
        fn oneof_index<'this>(&'this self) -> ::std::option::Option<i32>;
        /// optional string json_name = 10;
        fn json_name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field8MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::FieldOptionsTrait;
        /// optional FieldOptions options = 8;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field8MessageType<'this>>>;
        /// optional bool proto3_optional = 17;
        fn proto3_optional<'this>(&'this self) -> ::std::option::Option<bool>;
    }

    pub trait OneofDescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field2MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::OneofOptionsTrait;
        /// optional OneofOptions options = 2;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field2MessageType<'this>>>;
    }

    pub trait EnumDescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field2MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::EnumValueDescriptorProtoTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field2MessageType<'this>>,
        >;

        /// repeated EnumValueDescriptorProto value = 2;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::EnumOptionsTrait;
        /// optional EnumOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
        type Field4MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::puroro_traits::EnumReservedRangeTrait;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field4MessageType<'this>>,
        >;

        /// repeated EnumReservedRange reserved_range = 4;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, str>,
        >;

        /// repeated string reserved_name = 5;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
    }

    pub trait EnumValueDescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional int32 number = 2;
        fn number<'this>(&'this self) -> ::std::option::Option<i32>;
        type Field3MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::EnumValueOptionsTrait;
        /// optional EnumValueOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
    }

    pub trait ServiceDescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field2MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::MethodDescriptorProtoTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field2MessageType<'this>>,
        >;

        /// repeated MethodDescriptorProto method = 2;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::ServiceOptionsTrait;
        /// optional ServiceOptions options = 3;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
    }

    pub trait MethodDescriptorProtoTrait: ::std::clone::Clone {
        /// optional string name = 1;
        fn name<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string input_type = 2;
        fn input_type<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string output_type = 3;
        fn output_type<'this>(&'this self)
            -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field4MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::MethodOptionsTrait;
        /// optional MethodOptions options = 4;
        fn options<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field4MessageType<'this>>>;
        /// optional bool client_streaming = 5;
        fn client_streaming<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool server_streaming = 6;
        fn server_streaming<'this>(&'this self) -> ::std::option::Option<bool>;
    }

    pub trait FileOptionsTrait: ::std::clone::Clone {
        /// optional string java_package = 1;
        fn java_package<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string java_outer_classname = 8;
        fn java_outer_classname<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional bool java_multiple_files = 10;
        fn java_multiple_files<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool java_generate_equals_and_hash = 20;
        fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool java_string_check_utf8 = 27;
        fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional OptimizeMode optimize_for = 9;
        fn optimize_for<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
        >;
        /// optional string go_package = 11;
        fn go_package<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional bool cc_generic_services = 16;
        fn cc_generic_services<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool java_generic_services = 17;
        fn java_generic_services<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool py_generic_services = 18;
        fn py_generic_services<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool php_generic_services = 42;
        fn php_generic_services<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool deprecated = 23;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool cc_enable_arenas = 31;
        fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional string objc_class_prefix = 36;
        fn objc_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string csharp_namespace = 37;
        fn csharp_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string swift_prefix = 39;
        fn swift_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string php_class_prefix = 40;
        fn php_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string php_namespace = 41;
        fn php_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string php_metadata_namespace = 44;
        fn php_metadata_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional string ruby_package = 45;
        fn ruby_package<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait MessageOptionsTrait: ::std::clone::Clone {
        /// optional bool message_set_wire_format = 1;
        fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool no_standard_descriptor_accessor = 2;
        fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool map_entry = 7;
        fn map_entry<'this>(&'this self) -> ::std::option::Option<bool>;
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait FieldOptionsTrait: ::std::clone::Clone {
        /// optional CType ctype = 1;
        fn ctype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
        >;
        /// optional bool packed = 2;
        fn packed<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional JSType jstype = 6;
        fn jstype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
        >;
        /// optional bool lazy = 5;
        fn lazy<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool weak = 10;
        fn weak<'this>(&'this self) -> ::std::option::Option<bool>;
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait OneofOptionsTrait: ::std::clone::Clone {
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait EnumOptionsTrait: ::std::clone::Clone {
        /// optional bool allow_alias = 2;
        fn allow_alias<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional bool deprecated = 3;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool>;
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait EnumValueOptionsTrait: ::std::clone::Clone {
        /// optional bool deprecated = 1;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool>;
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait ServiceOptionsTrait: ::std::clone::Clone {
        /// optional bool deprecated = 33;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool>;
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait MethodOptionsTrait: ::std::clone::Clone {
        /// optional bool deprecated = 33;
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool>;
        /// optional IdempotencyLevel idempotency_level = 34;
        fn idempotency_level<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel,
        >;
        type Field999MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field999MessageType<'this>>,
        >;

        /// repeated UninterpretedOption uninterpreted_option = 999;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    pub trait UninterpretedOptionTrait: ::std::clone::Clone {
        type Field2MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::puroro_traits::NamePartTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field2MessageType<'this>>,
        >;

        /// repeated NamePart name = 2;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        /// optional string identifier_value = 3;
        fn identifier_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
        /// optional uint64 positive_int_value = 4;
        fn positive_int_value<'this>(&'this self) -> ::std::option::Option<u64>;
        /// optional int64 negative_int_value = 5;
        fn negative_int_value<'this>(&'this self) -> ::std::option::Option<i64>;
        /// optional double double_value = 6;
        fn double_value<'this>(&'this self) -> ::std::option::Option<f64>;
        /// optional bytes string_value = 7;
        fn string_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, [u8]>>;
        /// optional string aggregate_value = 8;
        fn aggregate_value<'this>(
            &'this self,
        ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
    }

    pub trait SourceCodeInfoTrait: ::std::clone::Clone {
        type Field1MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait;
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field1MessageType<'this>>,
        >;

        /// repeated Location location = 1;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    pub trait GeneratedCodeInfoTrait: ::std::clone::Clone {
        type Field1MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait;
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<
            'this,
            ::std::borrow::Cow<'this, Self::Field1MessageType<'this>>,
        >;

        /// repeated Annotation annotation = 1;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }
} // mod puroro_traits

pub use self::puroro_traits::*;

pub mod puroro_nested {

    pub mod descriptor_proto {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            #[allow(unused)]
            pub struct ExtensionRange<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
            where
                Self: ::puroro_internal::GetImplStruct;

            impl<ImplTag> ::puroro::Message for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
            {
            }

            impl<ImplTag> self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait for ExtensionRange<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait
    {
        /// optional int32 start = 1;
        fn start<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait
            >::start(&self.0)
        }
        /// optional int32 end = 2;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait
            >::end(&self.0)
        }
        type Field3MessageType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait
        >::Field3MessageType::<'this>;
        /// optional ExtensionRangeOptions options = 3;
        fn options<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait
            >::options(&self.0)
        }
    }

            impl<ImplTag> ::puroro::DeserFromBytesIter for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
            {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
                }
            }
            impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type:
                    ::puroro_internal::de::DeserFieldsFromBytesIter,
            {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                        &mut self.0,
                        field_number,
                        data,
                    )
                }
            }

            impl<ImplTag> ::puroro::SerToIoWrite for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::SerToIoWrite::ser(&self.0, out)
                }
            }

            impl<ImplTag> ::std::ops::Deref for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<ImplTag> ::std::ops::DerefMut for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl<ImplTag> ::std::clone::Clone for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl<ImplTag> ::std::default::Default for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
            {
                fn default() -> Self {
                    Self(::std::default::Default::default())
                }
            }
            impl<ImplTag> ::std::fmt::Debug for ExtensionRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
            {
                fn fmt(
                    &self,
                    formatter: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::result::Result<(), ::std::fmt::Error> {
                    ::std::fmt::Debug::fmt(&self.0, formatter)
                }
            }
            impl<ImplTag> ::std::cmp::PartialEq for ExtensionRange<ImplTag>
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
            pub struct ExtensionRange_SimpleImpl {
                pub start: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
                pub options: ::std::option::Option<
                    ::std::boxed::Box<
                        self::puroro_root::google::protobuf::ExtensionRangeOptions<
                            ::puroro::tags::SimpleImpl,
                        >,
                    >,
                >,
            }

            impl ::puroro_internal::GetImplStruct for ExtensionRange<::puroro::tags::SimpleImpl> {
                type Type = ExtensionRange_SimpleImpl;
            }

            impl ::puroro::Message for ExtensionRange_SimpleImpl {}

            impl self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ExtensionRangeTrait for ExtensionRange_SimpleImpl {
        /// optional int32 start = 1;
        fn start<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional int32 end = 2;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        type Field3MessageType<'this> = self::puroro_root::google::protobuf::ExtensionRangeOptions<::puroro::tags::SimpleImpl>;
        /// optional ExtensionRangeOptions options = 3;
        fn options<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>> {
            todo!()
        }
    }

            impl ::puroro::DeserFromBytesIter for ExtensionRange_SimpleImpl {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for ExtensionRange_SimpleImpl {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.start, data),
                        2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),
                        3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Message<
                                self::puroro_root::google::protobuf::ExtensionRangeOptions<
                                    ::puroro::tags::SimpleImpl,
                                >,
                            >,
                        >::deser_field(&mut self.options, data),
                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for ExtensionRange_SimpleImpl {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.start, 1, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 2, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Message<
                            self::puroro_root::google::protobuf::ExtensionRangeOptions<
                                ::puroro::tags::SimpleImpl,
                            >,
                        >,
                    >::ser_field(&self.options, 3, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            #[allow(unused)]
            pub struct ReservedRange<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
            where
                Self: ::puroro_internal::GetImplStruct;

            impl<ImplTag> ::puroro::Message for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
            {
            }

            impl<ImplTag> self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ReservedRangeTrait for ReservedRange<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ReservedRangeTrait
    {
        /// optional int32 start = 1;
        fn start<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ReservedRangeTrait
            >::start(&self.0)
        }
        /// optional int32 end = 2;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ReservedRangeTrait
            >::end(&self.0)
        }
    }

            impl<ImplTag> ::puroro::DeserFromBytesIter for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
            {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
                }
            }
            impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type:
                    ::puroro_internal::de::DeserFieldsFromBytesIter,
            {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                        &mut self.0,
                        field_number,
                        data,
                    )
                }
            }

            impl<ImplTag> ::puroro::SerToIoWrite for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::SerToIoWrite::ser(&self.0, out)
                }
            }

            impl<ImplTag> ::std::ops::Deref for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<ImplTag> ::std::ops::DerefMut for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl<ImplTag> ::std::clone::Clone for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl<ImplTag> ::std::default::Default for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
            {
                fn default() -> Self {
                    Self(::std::default::Default::default())
                }
            }
            impl<ImplTag> ::std::fmt::Debug for ReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
            {
                fn fmt(
                    &self,
                    formatter: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::result::Result<(), ::std::fmt::Error> {
                    ::std::fmt::Debug::fmt(&self.0, formatter)
                }
            }
            impl<ImplTag> ::std::cmp::PartialEq for ReservedRange<ImplTag>
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
            pub struct ReservedRange_SimpleImpl {
                pub start: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }

            impl ::puroro_internal::GetImplStruct for ReservedRange<::puroro::tags::SimpleImpl> {
                type Type = ReservedRange_SimpleImpl;
            }

            impl ::puroro::Message for ReservedRange_SimpleImpl {}

            impl self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::puroro_traits::ReservedRangeTrait for ReservedRange_SimpleImpl {
        /// optional int32 start = 1;
        fn start<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional int32 end = 2;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
    }

            impl ::puroro::DeserFromBytesIter for ReservedRange_SimpleImpl {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for ReservedRange_SimpleImpl {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.start, data),
                        2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),
                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for ReservedRange_SimpleImpl {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.start, 1, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }
        } // mod puroro_structs
        pub type ExtensionRange<ImplTag = ::puroro::tags::SimpleImpl> =
            self::puroro_structs::ExtensionRange<ImplTag>;
        pub type ReservedRange<ImplTag = ::puroro::tags::SimpleImpl> =
            self::puroro_structs::ReservedRange<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            pub trait ExtensionRangeTrait: ::std::clone::Clone {
                /// optional int32 start = 1;
                fn start<'this>(&'this self) -> ::std::option::Option<i32>;
                /// optional int32 end = 2;
                fn end<'this>(&'this self) -> ::std::option::Option<i32>;
                type Field3MessageType<'this>: 'this + self::puroro_root::google::protobuf::puroro_traits::ExtensionRangeOptionsTrait;
                /// optional ExtensionRangeOptions options = 3;
                fn options<'this>(
                    &'this self,
                ) -> ::std::option::Option<::std::borrow::Cow<'this, Self::Field3MessageType<'this>>>;
            }

            pub trait ReservedRangeTrait: ::std::clone::Clone {
                /// optional int32 start = 1;
                fn start<'this>(&'this self) -> ::std::option::Option<i32>;
                /// optional int32 end = 2;
                fn end<'this>(&'this self) -> ::std::option::Option<i32>;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod descriptor_proto

    pub mod field_descriptor_proto {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum Type {
            TypeDouble,
            TypeFloat,
            TypeInt64,
            TypeUint64,
            TypeInt32,
            TypeFixed64,
            TypeFixed32,
            TypeBool,
            TypeString,
            TypeGroup,
            TypeMessage,
            TypeBytes,
            TypeUint32,
            TypeEnum,
            TypeSfixed32,
            TypeSfixed64,
            TypeSint32,
            TypeSint64,
        }
        impl ::puroro::Enum2 for Type {}
        impl ::std::convert::TryFrom<i32> for Type {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    1 => Type::TypeDouble,
                    2 => Type::TypeFloat,
                    3 => Type::TypeInt64,
                    4 => Type::TypeUint64,
                    5 => Type::TypeInt32,
                    6 => Type::TypeFixed64,
                    7 => Type::TypeFixed32,
                    8 => Type::TypeBool,
                    9 => Type::TypeString,
                    10 => Type::TypeGroup,
                    11 => Type::TypeMessage,
                    12 => Type::TypeBytes,
                    13 => Type::TypeUint32,
                    14 => Type::TypeEnum,
                    15 => Type::TypeSfixed32,
                    16 => Type::TypeSfixed64,
                    17 => Type::TypeSint32,
                    18 => Type::TypeSint64,
                    _ => Err(value)?,
                })
            }
        }

        impl ::std::convert::From<Type> for i32 {
            fn from(value: Type) -> i32 {
                match value {
                    Type::TypeDouble => 1,
                    Type::TypeFloat => 2,
                    Type::TypeInt64 => 3,
                    Type::TypeUint64 => 4,
                    Type::TypeInt32 => 5,
                    Type::TypeFixed64 => 6,
                    Type::TypeFixed32 => 7,
                    Type::TypeBool => 8,
                    Type::TypeString => 9,
                    Type::TypeGroup => 10,
                    Type::TypeMessage => 11,
                    Type::TypeBytes => 12,
                    Type::TypeUint32 => 13,
                    Type::TypeEnum => 14,
                    Type::TypeSfixed32 => 15,
                    Type::TypeSfixed64 => 16,
                    Type::TypeSint32 => 17,
                    Type::TypeSint64 => 18,
                }
            }
        }

        impl ::std::default::Default for Type {
            fn default() -> Self {
                Type::TypeDouble
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum Label {
            LabelOptional,
            LabelRequired,
            LabelRepeated,
        }
        impl ::puroro::Enum2 for Label {}
        impl ::std::convert::TryFrom<i32> for Label {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    1 => Label::LabelOptional,
                    2 => Label::LabelRequired,
                    3 => Label::LabelRepeated,
                    _ => Err(value)?,
                })
            }
        }

        impl ::std::convert::From<Label> for i32 {
            fn from(value: Label) -> i32 {
                match value {
                    Label::LabelOptional => 1,
                    Label::LabelRequired => 2,
                    Label::LabelRepeated => 3,
                }
            }
        }

        impl ::std::default::Default for Label {
            fn default() -> Self {
                Label::LabelOptional
            }
        }
    } // mod field_descriptor_proto

    pub mod enum_descriptor_proto {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            #[allow(unused)]
            pub struct EnumReservedRange<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
            where
                Self: ::puroro_internal::GetImplStruct;

            impl<ImplTag> ::puroro::Message for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
            {
            }

            impl<ImplTag> self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::puroro_traits::EnumReservedRangeTrait for EnumReservedRange<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::puroro_traits::EnumReservedRangeTrait
    {
        /// optional int32 start = 1;
        fn start<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::puroro_traits::EnumReservedRangeTrait
            >::start(&self.0)
        }
        /// optional int32 end = 2;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::puroro_traits::EnumReservedRangeTrait
            >::end(&self.0)
        }
    }

            impl<ImplTag> ::puroro::DeserFromBytesIter for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
            {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
                }
            }
            impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type:
                    ::puroro_internal::de::DeserFieldsFromBytesIter,
            {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                        &mut self.0,
                        field_number,
                        data,
                    )
                }
            }

            impl<ImplTag> ::puroro::SerToIoWrite for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::SerToIoWrite::ser(&self.0, out)
                }
            }

            impl<ImplTag> ::std::ops::Deref for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<ImplTag> ::std::ops::DerefMut for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl<ImplTag> ::std::clone::Clone for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl<ImplTag> ::std::default::Default for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
            {
                fn default() -> Self {
                    Self(::std::default::Default::default())
                }
            }
            impl<ImplTag> ::std::fmt::Debug for EnumReservedRange<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
            {
                fn fmt(
                    &self,
                    formatter: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::result::Result<(), ::std::fmt::Error> {
                    ::std::fmt::Debug::fmt(&self.0, formatter)
                }
            }
            impl<ImplTag> ::std::cmp::PartialEq for EnumReservedRange<ImplTag>
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
            pub struct EnumReservedRange_SimpleImpl {
                pub start: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }

            impl ::puroro_internal::GetImplStruct for EnumReservedRange<::puroro::tags::SimpleImpl> {
                type Type = EnumReservedRange_SimpleImpl;
            }

            impl ::puroro::Message for EnumReservedRange_SimpleImpl {}

            impl self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::puroro_traits::EnumReservedRangeTrait for EnumReservedRange_SimpleImpl {
        /// optional int32 start = 1;
        fn start<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional int32 end = 2;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
    }

            impl ::puroro::DeserFromBytesIter for EnumReservedRange_SimpleImpl {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumReservedRange_SimpleImpl {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.start, data),
                        2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),
                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for EnumReservedRange_SimpleImpl {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.start, 1, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }
        } // mod puroro_structs
        pub type EnumReservedRange<ImplTag = ::puroro::tags::SimpleImpl> =
            self::puroro_structs::EnumReservedRange<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            pub trait EnumReservedRangeTrait: ::std::clone::Clone {
                /// optional int32 start = 1;
                fn start<'this>(&'this self) -> ::std::option::Option<i32>;
                /// optional int32 end = 2;
                fn end<'this>(&'this self) -> ::std::option::Option<i32>;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod enum_descriptor_proto

    pub mod file_options {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum OptimizeMode {
            Speed,
            CodeSize,
            LiteRuntime,
        }
        impl ::puroro::Enum2 for OptimizeMode {}
        impl ::std::convert::TryFrom<i32> for OptimizeMode {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    1 => OptimizeMode::Speed,
                    2 => OptimizeMode::CodeSize,
                    3 => OptimizeMode::LiteRuntime,
                    _ => Err(value)?,
                })
            }
        }

        impl ::std::convert::From<OptimizeMode> for i32 {
            fn from(value: OptimizeMode) -> i32 {
                match value {
                    OptimizeMode::Speed => 1,
                    OptimizeMode::CodeSize => 2,
                    OptimizeMode::LiteRuntime => 3,
                }
            }
        }

        impl ::std::default::Default for OptimizeMode {
            fn default() -> Self {
                OptimizeMode::Speed
            }
        }
    } // mod file_options

    pub mod field_options {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum Ctype {
            String,
            Cord,
            StringPiece,
        }
        impl ::puroro::Enum2 for Ctype {}
        impl ::std::convert::TryFrom<i32> for Ctype {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    0 => Ctype::String,
                    1 => Ctype::Cord,
                    2 => Ctype::StringPiece,
                    _ => Err(value)?,
                })
            }
        }

        impl ::std::convert::From<Ctype> for i32 {
            fn from(value: Ctype) -> i32 {
                match value {
                    Ctype::String => 0,
                    Ctype::Cord => 1,
                    Ctype::StringPiece => 2,
                }
            }
        }

        impl ::std::default::Default for Ctype {
            fn default() -> Self {
                Ctype::String
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum Jstype {
            JsNormal,
            JsString,
            JsNumber,
        }
        impl ::puroro::Enum2 for Jstype {}
        impl ::std::convert::TryFrom<i32> for Jstype {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    0 => Jstype::JsNormal,
                    1 => Jstype::JsString,
                    2 => Jstype::JsNumber,
                    _ => Err(value)?,
                })
            }
        }

        impl ::std::convert::From<Jstype> for i32 {
            fn from(value: Jstype) -> i32 {
                match value {
                    Jstype::JsNormal => 0,
                    Jstype::JsString => 1,
                    Jstype::JsNumber => 2,
                }
            }
        }

        impl ::std::default::Default for Jstype {
            fn default() -> Self {
                Jstype::JsNormal
            }
        }
    } // mod field_options

    pub mod method_options {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum IdempotencyLevel {
            IdempotencyUnknown,
            NoSideEffects,
            Idempotent,
        }
        impl ::puroro::Enum2 for IdempotencyLevel {}
        impl ::std::convert::TryFrom<i32> for IdempotencyLevel {
            type Error = i32;
            fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
                ::std::result::Result::Ok(match value {
                    0 => IdempotencyLevel::IdempotencyUnknown,
                    1 => IdempotencyLevel::NoSideEffects,
                    2 => IdempotencyLevel::Idempotent,
                    _ => Err(value)?,
                })
            }
        }

        impl ::std::convert::From<IdempotencyLevel> for i32 {
            fn from(value: IdempotencyLevel) -> i32 {
                match value {
                    IdempotencyLevel::IdempotencyUnknown => 0,
                    IdempotencyLevel::NoSideEffects => 1,
                    IdempotencyLevel::Idempotent => 2,
                }
            }
        }

        impl ::std::default::Default for IdempotencyLevel {
            fn default() -> Self {
                IdempotencyLevel::IdempotencyUnknown
            }
        }
    } // mod method_options

    pub mod uninterpreted_option {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            #[allow(unused)]
            pub struct NamePart<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
            where
                Self: ::puroro_internal::GetImplStruct;

            impl<ImplTag> ::puroro::Message for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
            {
            }

            impl<ImplTag> self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::puroro_traits::NamePartTrait for NamePart<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::puroro_traits::NamePartTrait
    {
        /// required string name_part = 1;
        fn name_part<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::puroro_traits::NamePartTrait
            >::name_part(&self.0)
        }
        /// required bool is_extension = 2;
        fn is_extension<'this>(&'this self) -> ::std::option::Option<bool> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::puroro_traits::NamePartTrait
            >::is_extension(&self.0)
        }
    }

            impl<ImplTag> ::puroro::DeserFromBytesIter for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
            {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
                }
            }
            impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type:
                    ::puroro_internal::de::DeserFieldsFromBytesIter,
            {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                        &mut self.0,
                        field_number,
                        data,
                    )
                }
            }

            impl<ImplTag> ::puroro::SerToIoWrite for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::SerToIoWrite::ser(&self.0, out)
                }
            }

            impl<ImplTag> ::std::ops::Deref for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<ImplTag> ::std::ops::DerefMut for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl<ImplTag> ::std::clone::Clone for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl<ImplTag> ::std::default::Default for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
            {
                fn default() -> Self {
                    Self(::std::default::Default::default())
                }
            }
            impl<ImplTag> ::std::fmt::Debug for NamePart<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
            {
                fn fmt(
                    &self,
                    formatter: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::result::Result<(), ::std::fmt::Error> {
                    ::std::fmt::Debug::fmt(&self.0, formatter)
                }
            }
            impl<ImplTag> ::std::cmp::PartialEq for NamePart<ImplTag>
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
            pub struct NamePart_SimpleImpl {
                pub name_part: ::std::option::Option<::std::string::String>,
                pub is_extension: ::std::option::Option<bool>,
            }

            impl ::puroro_internal::GetImplStruct for NamePart<::puroro::tags::SimpleImpl> {
                type Type = NamePart_SimpleImpl;
            }

            impl ::puroro::Message for NamePart_SimpleImpl {}

            impl self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::puroro_traits::NamePartTrait for NamePart_SimpleImpl {
        /// required string name_part = 1;
        fn name_part<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// required bool is_extension = 2;
        fn is_extension<'this>(&'this self) -> ::std::option::Option<bool> {
            todo!()
        }
    }

            impl ::puroro::DeserFromBytesIter for NamePart_SimpleImpl {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for NamePart_SimpleImpl {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::String,
                        >::deser_field(&mut self.name_part, data),
                        2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::Bool,
                        >::deser_field(&mut self.is_extension, data),
                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for NamePart_SimpleImpl {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Required,
                        ::puroro::tags::String,
                    >::ser_field(&self.name_part, 1, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Required,
                        ::puroro::tags::Bool,
                    >::ser_field(&self.is_extension, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }
        } // mod puroro_structs
        pub type NamePart<ImplTag = ::puroro::tags::SimpleImpl> =
            self::puroro_structs::NamePart<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            pub trait NamePartTrait: ::std::clone::Clone {
                /// required string name_part = 1;
                fn name_part<'this>(
                    &'this self,
                ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
                /// required bool is_extension = 2;
                fn is_extension<'this>(&'this self) -> ::std::option::Option<bool>;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod uninterpreted_option

    pub mod source_code_info {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            #[allow(unused)]
            pub struct Location<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
            where
                Self: ::puroro_internal::GetImplStruct;

            impl<ImplTag> ::puroro::Message for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
            {
            }

            impl<ImplTag> self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait for Location<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
    {
        type Field1RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
        >::Field1RepeatedType::<'this>;
    
        /// repeated int32 path = 1;
        fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
            >::path(&self.0)
        }
        type Field2RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
        >::Field2RepeatedType::<'this>;
    
        /// repeated int32 span = 2;
        fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
            >::span(&self.0)
        }
        /// optional string leading_comments = 3;
        fn leading_comments<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
            >::leading_comments(&self.0)
        }
        /// optional string trailing_comments = 4;
        fn trailing_comments<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
            >::trailing_comments(&self.0)
        }
        type Field6RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
        >::Field6RepeatedType::<'this>;
    
        /// repeated string leading_detached_comments = 6;
        fn leading_detached_comments<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait
            >::leading_detached_comments(&self.0)
        }
    }

            impl<ImplTag> ::puroro::DeserFromBytesIter for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
            {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
                }
            }
            impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type:
                    ::puroro_internal::de::DeserFieldsFromBytesIter,
            {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                        &mut self.0,
                        field_number,
                        data,
                    )
                }
            }

            impl<ImplTag> ::puroro::SerToIoWrite for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::SerToIoWrite::ser(&self.0, out)
                }
            }

            impl<ImplTag> ::std::ops::Deref for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<ImplTag> ::std::ops::DerefMut for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl<ImplTag> ::std::clone::Clone for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl<ImplTag> ::std::default::Default for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
            {
                fn default() -> Self {
                    Self(::std::default::Default::default())
                }
            }
            impl<ImplTag> ::std::fmt::Debug for Location<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
            {
                fn fmt(
                    &self,
                    formatter: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::result::Result<(), ::std::fmt::Error> {
                    ::std::fmt::Debug::fmt(&self.0, formatter)
                }
            }
            impl<ImplTag> ::std::cmp::PartialEq for Location<ImplTag>
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
            pub struct Location_SimpleImpl {
                pub path: ::std::vec::Vec<i32>,
                pub span: ::std::vec::Vec<i32>,
                pub leading_comments: ::std::option::Option<::std::string::String>,
                pub trailing_comments: ::std::option::Option<::std::string::String>,
                pub leading_detached_comments: ::std::vec::Vec<::std::string::String>,
            }

            impl ::puroro_internal::GetImplStruct for Location<::puroro::tags::SimpleImpl> {
                type Type = Location_SimpleImpl;
            }

            impl ::puroro::Message for Location_SimpleImpl {}

            impl self::puroro_root::google::protobuf::puroro_nested::source_code_info::puroro_traits::LocationTrait for Location_SimpleImpl {
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;
    
        /// repeated int32 path = 1;
        fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.path)
        }
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;
    
        /// repeated int32 span = 2;
        fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.span)
        }
        /// optional string leading_comments = 3;
        fn leading_comments<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional string trailing_comments = 4;
        fn trailing_comments<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::VecCowWrapper<'this, str>;
    
        /// repeated string leading_detached_comments = 6;
        fn leading_detached_comments<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecCowWrapper::new(&self.leading_detached_comments)
        }
    }

            impl ::puroro::DeserFromBytesIter for Location_SimpleImpl {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for Location_SimpleImpl {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.path, data),
                        2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.span, data),
                        3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.leading_comments, data
                        ),
                        4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.trailing_comments, data
                        ),
                        6 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.leading_detached_comments, data
                        ),
                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for Location_SimpleImpl {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.path, 1, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.span, 2, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(&self.leading_comments, 3, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(&self.trailing_comments, 4, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >::ser_field(&self.leading_detached_comments, 6, out)?;
                    ::std::result::Result::Ok(())
                }
            }
        } // mod puroro_structs
        pub type Location<ImplTag = ::puroro::tags::SimpleImpl> =
            self::puroro_structs::Location<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            pub trait LocationTrait: ::std::clone::Clone {
                type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

                /// repeated int32 path = 1;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
                type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

                /// repeated int32 span = 2;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
                /// optional string leading_comments = 3;
                fn leading_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
                /// optional string trailing_comments = 4;
                fn trailing_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
                type Field6RepeatedType<'this>: ::puroro::RepeatedField<
                    'this,
                    ::std::borrow::Cow<'this, str>,
                >;

                /// repeated string leading_detached_comments = 6;
                fn leading_detached_comments<'this>(&'this self)
                    -> Self::Field6RepeatedType<'this>;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod source_code_info

    pub mod generated_code_info {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            #[allow(unused)]
            pub struct Annotation<ImplTag>(<Self as ::puroro_internal::GetImplStruct>::Type)
            where
                Self: ::puroro_internal::GetImplStruct;

            impl<ImplTag> ::puroro::Message for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::Message,
            {
            }

            impl<ImplTag> self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait for Annotation<ImplTag>
    where
        Self: ::puroro_internal::GetImplStruct,
        <Self as ::puroro_internal::GetImplStruct>::Type: self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait
    {
        type Field1RepeatedType<'this> = <
            <Self as ::puroro_internal::GetImplStruct>::Type
            as self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait
        >::Field1RepeatedType::<'this>;
    
        /// repeated int32 path = 1;
        fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait
            >::path(&self.0)
        }
        /// optional string source_file = 2;
        fn source_file<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait
            >::source_file(&self.0)
        }
        /// optional int32 begin = 3;
        fn begin<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait
            >::begin(&self.0)
        }
        /// optional int32 end = 4;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            <
                <Self as ::puroro_internal::GetImplStruct>::Type
                as self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait
            >::end(&self.0)
        }
    }

            impl<ImplTag> ::puroro::DeserFromBytesIter for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::DeserFromBytesIter,
            {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    <<Self as ::puroro_internal::GetImplStruct>::Type as ::puroro::DeserFromBytesIter>::deser(self, iter)
                }
            }
            impl<ImplTag> ::puroro_internal::de::DeserFieldsFromBytesIter for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type:
                    ::puroro_internal::de::DeserFieldsFromBytesIter,
            {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::DeserFieldsFromBytesIter::deser_field(
                        &mut self.0,
                        field_number,
                        data,
                    )
                }
            }

            impl<ImplTag> ::puroro::SerToIoWrite for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::puroro::SerToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro::SerToIoWrite::ser(&self.0, out)
                }
            }

            impl<ImplTag> ::std::ops::Deref for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                type Target = <Self as ::puroro_internal::GetImplStruct>::Type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<ImplTag> ::std::ops::DerefMut for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl<ImplTag> ::std::clone::Clone for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::clone::Clone,
            {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl<ImplTag> ::std::default::Default for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::default::Default,
            {
                fn default() -> Self {
                    Self(::std::default::Default::default())
                }
            }
            impl<ImplTag> ::std::fmt::Debug for Annotation<ImplTag>
            where
                Self: ::puroro_internal::GetImplStruct,
                <Self as ::puroro_internal::GetImplStruct>::Type: ::std::fmt::Debug,
            {
                fn fmt(
                    &self,
                    formatter: &mut ::std::fmt::Formatter<'_>,
                ) -> ::std::result::Result<(), ::std::fmt::Error> {
                    ::std::fmt::Debug::fmt(&self.0, formatter)
                }
            }
            impl<ImplTag> ::std::cmp::PartialEq for Annotation<ImplTag>
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
            pub struct Annotation_SimpleImpl {
                pub path: ::std::vec::Vec<i32>,
                pub source_file: ::std::option::Option<::std::string::String>,
                pub begin: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }

            impl ::puroro_internal::GetImplStruct for Annotation<::puroro::tags::SimpleImpl> {
                type Type = Annotation_SimpleImpl;
            }

            impl ::puroro::Message for Annotation_SimpleImpl {}

            impl self::puroro_root::google::protobuf::puroro_nested::generated_code_info::puroro_traits::AnnotationTrait for Annotation_SimpleImpl {
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::VecWrapper<'this, i32>;
    
        /// repeated int32 path = 1;
        fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::VecWrapper::new(&self.path)
        }
        /// optional string source_file = 2;
        fn source_file<'this>(&'this self) -> ::std::option::Option<::std::borrow::Cow<'this, str>> {
            todo!()
        }
        /// optional int32 begin = 3;
        fn begin<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
        /// optional int32 end = 4;
        fn end<'this>(&'this self) -> ::std::option::Option<i32> {
            todo!()
        }
    }

            impl ::puroro::DeserFromBytesIter for Annotation_SimpleImpl {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for Annotation_SimpleImpl {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro_internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    match field_number {
                        1 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.path, data),
                        2 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(&mut self.source_file, data),
                        3 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.begin, data),
                        4 => ::puroro_internal::impls::simple::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),
                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for Annotation_SimpleImpl {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.path, 1, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(&self.source_file, 2, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.begin, 3, out)?;
                    ::puroro_internal::impls::simple::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 4, out)?;
                    ::std::result::Result::Ok(())
                }
            }
        } // mod puroro_structs
        pub type Annotation<ImplTag = ::puroro::tags::SimpleImpl> =
            self::puroro_structs::Annotation<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }

            pub trait AnnotationTrait: ::std::clone::Clone {
                type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this, i32>;

                /// repeated int32 path = 1;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
                /// optional string source_file = 2;
                fn source_file<'this>(
                    &'this self,
                ) -> ::std::option::Option<::std::borrow::Cow<'this, str>>;
                /// optional int32 begin = 3;
                fn begin<'this>(&'this self) -> ::std::option::Option<i32>;
                /// optional int32 end = 4;
                fn end<'this>(&'this self) -> ::std::option::Option<i32>;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod generated_code_info
} // mod puroro_nested

pub use self::puroro_nested::*;
