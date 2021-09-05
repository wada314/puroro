// A generated source code by puroro library
// package google.protobuf
pub mod compiler;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::DescriptorProtoSimple as DescriptorProto;
pub use _puroro_impls::EnumDescriptorProtoSimple as EnumDescriptorProto;
pub use _puroro_impls::EnumOptionsSimple as EnumOptions;
pub use _puroro_impls::EnumValueDescriptorProtoSimple as EnumValueDescriptorProto;
pub use _puroro_impls::EnumValueOptionsSimple as EnumValueOptions;
pub use _puroro_impls::ExtensionRangeOptionsSimple as ExtensionRangeOptions;
pub use _puroro_impls::FieldDescriptorProtoSimple as FieldDescriptorProto;
pub use _puroro_impls::FieldOptionsSimple as FieldOptions;
pub use _puroro_impls::FileDescriptorProtoSimple as FileDescriptorProto;
pub use _puroro_impls::FileDescriptorSetSimple as FileDescriptorSet;
pub use _puroro_impls::FileOptionsSimple as FileOptions;
pub use _puroro_impls::GeneratedCodeInfoSimple as GeneratedCodeInfo;
pub use _puroro_impls::MessageOptionsSimple as MessageOptions;
pub use _puroro_impls::MethodDescriptorProtoSimple as MethodDescriptorProto;
pub use _puroro_impls::MethodOptionsSimple as MethodOptions;
pub use _puroro_impls::OneofDescriptorProtoSimple as OneofDescriptorProto;
pub use _puroro_impls::OneofOptionsSimple as OneofOptions;
pub use _puroro_impls::ServiceDescriptorProtoSimple as ServiceDescriptorProto;
pub use _puroro_impls::ServiceOptionsSimple as ServiceOptions;
pub use _puroro_impls::SourceCodeInfoSimple as SourceCodeInfo;
pub use _puroro_impls::UninterpretedOptionSimple as UninterpretedOption;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileDescriptorSetSimple {
        pub file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
        >,
    }
    impl ::puroro::Message for FileDescriptorSetSimple {}

    impl FileDescriptorSetTrait for FileDescriptorSetSimple {
        type Field1MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            >,
        >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.file.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for FileDescriptorSetSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FileDescriptorSetSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple>
            >::deser_field(&mut self.file, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for FileDescriptorSetSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
                >,
            >::ser_field(&self.file, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl FileDescriptorSetTrait for () {
        type Field1MessageType<'this> = ();
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> FileDescriptorSetTrait for (T, U)
    where
        T: FileDescriptorSetTrait,
        U: FileDescriptorSetTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorSetTrait>::Field1MessageType<'this>,
            <U as FileDescriptorSetTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
                <U as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
            >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorSetTrait>::file(&self.0),
                <U as FileDescriptorSetTrait>::file(&self.1),
            )
        }
    }
    impl<T, U> FileDescriptorSetTrait for ::puroro::Either<T, U>
    where
        T: FileDescriptorSetTrait,
        U: FileDescriptorSetTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorSetTrait>::Field1MessageType<'this>,
            <U as FileDescriptorSetTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
                <U as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
            >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorSetTrait>::file(t))
                    .map_right(|u| <U as FileDescriptorSetTrait>::file(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorSetSimpleField1 {
        file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for FileDescriptorSetSimpleField1 {}

    impl super::_puroro_traits::FileDescriptorSetTrait for FileDescriptorSetSimpleField1 {
        type Field1MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            >,
        >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.file.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileDescriptorProtoSimple {
        pub name: ::std::option::Option<::std::string::String>,
        pub package: ::std::option::Option<::std::string::String>,
        pub dependency: ::std::vec::Vec<::std::string::String>,
        pub public_dependency: ::std::vec::Vec<i32>,
        pub weak_dependency: ::std::vec::Vec<i32>,
        pub message_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
        >,
        pub enum_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
        >,
        pub service: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple,
        >,
        pub extension: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
        >,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::FileOptionsSimple,
            >,
        >,
        pub source_code_info: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::SourceCodeInfoSimple,
            >,
        >,
        pub syntax: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message for FileDescriptorProtoSimple {}

    impl FileDescriptorProtoTrait for FileDescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'this str;
        fn package<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.package.as_ref().map(|v| v.as_ref())
        }
        type Field3StringType<'this> = &'this str;
        type Field3RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.dependency.iter())
        }
        type Field10RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.public_dependency.iter().cloned()
        }
        type Field11RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            self.weak_dependency.iter().cloned()
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            >,
        >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.message_type.iter())
        }
        type Field5MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field6MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple;
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple,
            >,
        >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.service.iter())
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple;
        type Field7RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FileOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::SourceCodeInfoSimple;
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            self.source_code_info.as_ref().map(|v| v.as_ref())
        }
        type Field12StringType<'this> = &'this str;
        fn syntax<'this>(&'this self) -> Option<Self::Field12StringType<'this>> {
            self.syntax.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for FileDescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FileDescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.name, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.package, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.dependency, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.public_dependency, data),
            11 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.weak_dependency, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple>
            >::deser_field(&mut self.message_type, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple>
            >::deser_field(&mut self.enum_type, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple>
            >::deser_field(&mut self.service, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple>
            >::deser_field(&mut self.extension, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FileOptionsSimple>
            >::deser_field(&mut self.options, data),
            9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::SourceCodeInfoSimple>
            >::deser_field(&mut self.source_code_info, data),
            12 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.syntax, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for FileDescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.package, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.dependency, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.public_dependency, 10, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >::ser_field(&self.weak_dependency, 11, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
                >,
            >::ser_field(&self.message_type, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
                >,
            >::ser_field(&self.enum_type, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple>
        >::ser_field(&self.service, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
                >,
            >::ser_field(&self.extension, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::FileOptionsSimple,
                >,
            >::ser_field(&self.options, 8, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::SourceCodeInfoSimple,
                >,
            >::ser_field(&self.source_code_info, 9, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.syntax, 12, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl FileDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }
    impl<T, U> FileDescriptorProtoTrait for (T, U)
    where
        T: FileDescriptorProtoTrait,
        U: FileDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field1StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as FileDescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileDescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field2StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field2StringType<'this>,
        >;
        fn package<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            if let Some(right) = <U as FileDescriptorProtoTrait>::package(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileDescriptorProtoTrait>::package(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field3StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field3StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field3StringType<'this>,
        >;
        type Field3RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedLDField<
            <T as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedLDField::new(
                <T as FileDescriptorProtoTrait>::dependency(&self.0),
                <U as FileDescriptorProtoTrait>::dependency(&self.1),
            )
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedField<
            <T as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedField::new(
                <T as FileDescriptorProtoTrait>::public_dependency(&self.0),
                <U as FileDescriptorProtoTrait>::public_dependency(&self.1),
            )
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedField<
            <T as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
        >;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedField::new(
                <T as FileDescriptorProtoTrait>::weak_dependency(&self.0),
                <U as FileDescriptorProtoTrait>::weak_dependency(&self.1),
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::message_type(&self.0),
                <U as FileDescriptorProtoTrait>::message_type(&self.1),
            )
        }
        type Field5MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field5MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::enum_type(&self.0),
                <U as FileDescriptorProtoTrait>::enum_type(&self.1),
            )
        }
        type Field6MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field6MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::service(&self.0),
                <U as FileDescriptorProtoTrait>::service(&self.1),
            )
        }
        type Field7MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field7MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field7MessageType<'this>,
        >;
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::extension(&self.0),
                <U as FileDescriptorProtoTrait>::extension(&self.1),
            )
        }
        type Field8MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as FileDescriptorProtoTrait>::Field8MessageType<'this>,
                <U as FileDescriptorProtoTrait>::Field8MessageType<'this>,
            >,
            (
                <T as FileDescriptorProtoTrait>::Field8MessageType<'this>,
                <U as FileDescriptorProtoTrait>::Field8MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            match (
                <T as FileDescriptorProtoTrait>::options(&self.0),
                <U as FileDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        type Field9MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as FileDescriptorProtoTrait>::Field9MessageType<'this>,
                <U as FileDescriptorProtoTrait>::Field9MessageType<'this>,
            >,
            (
                <T as FileDescriptorProtoTrait>::Field9MessageType<'this>,
                <U as FileDescriptorProtoTrait>::Field9MessageType<'this>,
            ),
        >;
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            match (
                <T as FileDescriptorProtoTrait>::source_code_info(&self.0),
                <U as FileDescriptorProtoTrait>::source_code_info(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        type Field12StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field12StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field12StringType<'this>,
        >;
        fn syntax<'this>(&'this self) -> Option<Self::Field12StringType<'this>> {
            if let Some(right) = <U as FileDescriptorProtoTrait>::syntax(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileDescriptorProtoTrait>::syntax(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
    }
    impl<T, U> FileDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: FileDescriptorProtoTrait,
        U: FileDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field1StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileDescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileDescriptorProtoTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field2StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field2StringType<'this>,
        >;
        fn package<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileDescriptorProtoTrait>::package(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileDescriptorProtoTrait>::package(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field3StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field3StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field3StringType<'this>,
        >;
        type Field3RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedLDField<
            <T as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::dependency(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::dependency(u)),
            )
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::public_dependency(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::public_dependency(u)),
            )
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedField<
            <T as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
        >;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::weak_dependency(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::weak_dependency(u)),
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::message_type(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::message_type(u)),
            )
        }
        type Field5MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field5MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::enum_type(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::enum_type(u)),
            )
        }
        type Field6MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field6MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::service(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::service(u)),
            )
        }
        type Field7MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field7MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field7MessageType<'this>,
        >;
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::extension(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::extension(u)),
            )
        }
        type Field8MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field8MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.as_ref().either(
                |t| <T as FileDescriptorProtoTrait>::options(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileDescriptorProtoTrait>::options(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field9MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field9MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field9MessageType<'this>,
        >;
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FileDescriptorProtoTrait>::source_code_info(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FileDescriptorProtoTrait>::source_code_info(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field12StringType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field12StringType<'this>,
            <U as FileDescriptorProtoTrait>::Field12StringType<'this>,
        >;
        fn syntax<'this>(&'this self) -> Option<Self::Field12StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileDescriptorProtoTrait>::syntax(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileDescriptorProtoTrait>::syntax(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField2 {
        package: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'this str;
        fn package<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.package.as_ref().map(|v| v.as_ref())
        }
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField3 {
        dependency: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'this str;
        type Field3RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.dependency.iter())
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField10 {
        public_dependency: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField10 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField10 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.public_dependency.iter().cloned()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField11 {
        weak_dependency: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField11 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField11 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            self.weak_dependency.iter().cloned()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField4 {
        message_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            >,
        >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.message_type.iter())
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField5 {
        enum_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField6 {
        service: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField6 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField6 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple;
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::ServiceDescriptorProtoSimple,
            >,
        >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.service.iter())
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField7 {
        extension: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField7 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField7 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple;
        type Field7RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField8 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::FileOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField8 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField8 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FileOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField9 {
        source_code_info: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::SourceCodeInfoSimple,
            >,
        >,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField9 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField9 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::SourceCodeInfoSimple;
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            self.source_code_info.as_ref().map(|v| v.as_ref())
        }
        type Field12StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileDescriptorProtoSimpleField12 {
        syntax: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileDescriptorProtoSimpleField12 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField12 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'this str;
        fn syntax<'this>(&'this self) -> Option<Self::Field12StringType<'this>> {
            self.syntax.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct DescriptorProtoSimple {
    pub name: ::std::option::Option<::std::string::String>,
    pub field: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple>,
    pub extension: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple>,
    pub nested_type: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple>,
    pub enum_type: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple>,
    pub extension_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple>,
    pub oneof_decl: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple>,
    pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_impls::MessageOptionsSimple>>,
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
    impl ::puroro::Message for DescriptorProtoSimple {}

    impl DescriptorProtoTrait for DescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            >,
        >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.field.iter())
        }
        type Field6MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple;
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple;
        type Field3RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            >,
        >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.nested_type.iter())
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field5MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple>>;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.extension_range.iter())
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple,
            >,
        >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.oneof_decl.iter())
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::MessageOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple;
        type Field9RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple>>;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field10StringType<'this> = &'this str;
        type Field10RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for DescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for DescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.name, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple>
            >::deser_field(&mut self.field, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple>
            >::deser_field(&mut self.extension, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple>
            >::deser_field(&mut self.nested_type, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple>
            >::deser_field(&mut self.enum_type, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple>
            >::deser_field(&mut self.extension_range, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple>
            >::deser_field(&mut self.oneof_decl, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::MessageOptionsSimple>
            >::deser_field(&mut self.options, data),
            9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple>
            >::deser_field(&mut self.reserved_range, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.reserved_name, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for DescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
                >,
            >::ser_field(&self.field, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
                >,
            >::ser_field(&self.extension, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
                >,
            >::ser_field(&self.nested_type, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
                >,
            >::ser_field(&self.enum_type, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple>
        >::ser_field(&self.extension_range, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple,
                >,
            >::ser_field(&self.oneof_decl, 8, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::MessageOptionsSimple,
                >,
            >::ser_field(&self.options, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple>
        >::ser_field(&self.reserved_range, 9, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.reserved_name, 10, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl DescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> DescriptorProtoTrait for (T, U)
    where
        T: DescriptorProtoTrait,
        U: DescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field1StringType<'this>,
            <U as DescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as DescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as DescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field2MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field2MessageType<'this>,
            <U as DescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::field(&self.0),
                <U as DescriptorProtoTrait>::field(&self.1),
            )
        }
        type Field6MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field6MessageType<'this>,
            <U as DescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::extension(&self.0),
                <U as DescriptorProtoTrait>::extension(&self.1),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field3MessageType<'this>,
            <U as DescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field3RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field3RepeatedType<'this>,
            >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::nested_type(&self.0),
                <U as DescriptorProtoTrait>::nested_type(&self.1),
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field4MessageType<'this>,
            <U as DescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::enum_type(&self.0),
                <U as DescriptorProtoTrait>::enum_type(&self.1),
            )
        }
        type Field5MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field5MessageType<'this>,
            <U as DescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::extension_range(&self.0),
                <U as DescriptorProtoTrait>::extension_range(&self.1),
            )
        }
        type Field8MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field8MessageType<'this>,
            <U as DescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field8RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field8RepeatedType<'this>,
            >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::oneof_decl(&self.0),
                <U as DescriptorProtoTrait>::oneof_decl(&self.1),
            )
        }
        type Field7MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as DescriptorProtoTrait>::Field7MessageType<'this>,
                <U as DescriptorProtoTrait>::Field7MessageType<'this>,
            >,
            (
                <T as DescriptorProtoTrait>::Field7MessageType<'this>,
                <U as DescriptorProtoTrait>::Field7MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            match (
                <T as DescriptorProtoTrait>::options(&self.0),
                <U as DescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        type Field9MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field9MessageType<'this>,
            <U as DescriptorProtoTrait>::Field9MessageType<'this>,
        >;
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field9RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field9RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::reserved_range(&self.0),
                <U as DescriptorProtoTrait>::reserved_range(&self.1),
            )
        }
        type Field10StringType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field10StringType<'this>,
            <U as DescriptorProtoTrait>::Field10StringType<'this>,
        >;
        type Field10RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedLDField<
            <T as DescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedLDField::new(
                <T as DescriptorProtoTrait>::reserved_name(&self.0),
                <U as DescriptorProtoTrait>::reserved_name(&self.1),
            )
        }
    }
    impl<T, U> DescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: DescriptorProtoTrait,
        U: DescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field1StringType<'this>,
            <U as DescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as DescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as DescriptorProtoTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field2MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field2MessageType<'this>,
            <U as DescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::field(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::field(u)),
            )
        }
        type Field6MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field6MessageType<'this>,
            <U as DescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::extension(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::extension(u)),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field3MessageType<'this>,
            <U as DescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field3RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field3RepeatedType<'this>,
            >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::nested_type(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::nested_type(u)),
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field4MessageType<'this>,
            <U as DescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::enum_type(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::enum_type(u)),
            )
        }
        type Field5MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field5MessageType<'this>,
            <U as DescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::extension_range(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::extension_range(u)),
            )
        }
        type Field8MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field8MessageType<'this>,
            <U as DescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field8RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field8RepeatedType<'this>,
            >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::oneof_decl(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::oneof_decl(u)),
            )
        }
        type Field7MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field7MessageType<'this>,
            <U as DescriptorProtoTrait>::Field7MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.as_ref().either(
                |t| <T as DescriptorProtoTrait>::options(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as DescriptorProtoTrait>::options(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field9MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field9MessageType<'this>,
            <U as DescriptorProtoTrait>::Field9MessageType<'this>,
        >;
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field9RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field9RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::reserved_range(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::reserved_range(u)),
            )
        }
        type Field10StringType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field10StringType<'this>,
            <U as DescriptorProtoTrait>::Field10StringType<'this>,
        >;
        type Field10RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedLDField<
            <T as DescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::reserved_name(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::reserved_name(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField2 {
        field: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            >,
        >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.field.iter())
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField6 {
        extension: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField6 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField6 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple;
        type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FieldDescriptorProtoSimple,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField3 {
        nested_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple;
        type Field3RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::DescriptorProtoSimple,
            >,
        >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.nested_type.iter())
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField4 {
        enum_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::EnumDescriptorProtoSimple,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField5 {
    extension_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple>,
}

    impl ::puroro::Message for DescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ExtensionRangeSimple>>;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.extension_range.iter())
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField8 {
        oneof_decl: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField8 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField8 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple;
        type Field8RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::OneofDescriptorProtoSimple,
            >,
        >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.oneof_decl.iter())
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField7 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::MessageOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField7 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField7 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::MessageOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField9 {
    reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple>,
}

    impl ::puroro::Message for DescriptorProtoSimpleField9 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField9 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple;
        type Field9RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_impls::ReservedRangeSimple>>;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct DescriptorProtoSimpleField10 {
        reserved_name: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro::Message for DescriptorProtoSimpleField10 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField10 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'this str;
        type Field10RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ExtensionRangeOptionsSimple {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for ExtensionRangeOptionsSimple {}

    impl ExtensionRangeOptionsTrait for ExtensionRangeOptionsSimple {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for ExtensionRangeOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for ExtensionRangeOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for ExtensionRangeOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl ExtensionRangeOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> ExtensionRangeOptionsTrait for (T, U)
    where
        T: ExtensionRangeOptionsTrait,
        U: ExtensionRangeOptionsTrait,
    {
        type Field999MessageType<'this> = ::puroro::Either<
            <T as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
            <U as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
                <U as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as ExtensionRangeOptionsTrait>::uninterpreted_option(&self.0),
                <U as ExtensionRangeOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> ExtensionRangeOptionsTrait for ::puroro::Either<T, U>
    where
        T: ExtensionRangeOptionsTrait,
        U: ExtensionRangeOptionsTrait,
    {
        type Field999MessageType<'this> = ::puroro::Either<
            <T as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
            <U as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
                <U as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as ExtensionRangeOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as ExtensionRangeOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct ExtensionRangeOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for ExtensionRangeOptionsSimpleField999 {}

    impl super::_puroro_traits::ExtensionRangeOptionsTrait for ExtensionRangeOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FieldDescriptorProtoSimple {
        pub name: ::std::option::Option<::std::string::String>,
        pub number: ::std::option::Option<i32>,
        pub label: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        >,
        pub type_: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        >,
        pub type_name: ::std::option::Option<::std::string::String>,
        pub extendee: ::std::option::Option<::std::string::String>,
        pub default_value: ::std::option::Option<::std::string::String>,
        pub oneof_index: ::std::option::Option<i32>,
        pub json_name: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::FieldOptionsSimple,
            >,
        >,
        pub proto3_optional: ::std::option::Option<bool>,
    }
    impl ::puroro::Message for FieldDescriptorProtoSimple {}

    impl FieldDescriptorProtoTrait for FieldDescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        fn number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        fn label<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            Clone::clone(&self.label)
        }
        fn type_<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            Clone::clone(&self.type_)
        }
        type Field6StringType<'this> = &'this str;
        fn type_name<'this>(&'this self) -> Option<Self::Field6StringType<'this>> {
            self.type_name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'this str;
        fn extendee<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.extendee.as_ref().map(|v| v.as_ref())
        }
        type Field7StringType<'this> = &'this str;
        fn default_value<'this>(&'this self) -> Option<Self::Field7StringType<'this>> {
            self.default_value.as_ref().map(|v| v.as_ref())
        }
        fn oneof_index<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.oneof_index)
        }
        type Field10StringType<'this> = &'this str;
        fn json_name<'this>(&'this self) -> Option<Self::Field10StringType<'this>> {
            self.json_name.as_ref().map(|v| v.as_ref())
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        fn proto3_optional<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.proto3_optional)
        }
    }

    impl ::puroro::DeserFromBytesIter for FieldDescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FieldDescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.name, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.number, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label>
            >::deser_field(&mut self.label, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type>
            >::deser_field(&mut self.type_, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.type_name, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.extendee, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.default_value, data),
            9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.oneof_index, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.json_name, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FieldOptionsSimple>
            >::deser_field(&mut self.options, data),
            17 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.proto3_optional, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for FieldDescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.number, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label>
        >::ser_field(&self.label, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type>
        >::ser_field(&self.type_, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.type_name, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.extendee, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.default_value, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.oneof_index, 9, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.json_name, 10, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::FieldOptionsSimple,
                >,
            >::ser_field(&self.options, 8, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.proto3_optional, 17, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl FieldDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }
    impl<T, U> FieldDescriptorProtoTrait for (T, U)
    where
        T: FieldDescriptorProtoTrait,
        U: FieldDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field1StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as FieldDescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FieldDescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        fn number<'this>(&'this self) -> Option<i32> {
            <U as FieldDescriptorProtoTrait>::number(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::number(&self.0))
        }
        fn label<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            <U as FieldDescriptorProtoTrait>::label(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::label(&self.0))
        }
        fn type_<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            <U as FieldDescriptorProtoTrait>::type_(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::type_(&self.0))
        }
        type Field6StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field6StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field6StringType<'this>,
        >;
        fn type_name<'this>(&'this self) -> Option<Self::Field6StringType<'this>> {
            if let Some(right) = <U as FieldDescriptorProtoTrait>::type_name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FieldDescriptorProtoTrait>::type_name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field2StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field2StringType<'this>,
        >;
        fn extendee<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            if let Some(right) = <U as FieldDescriptorProtoTrait>::extendee(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FieldDescriptorProtoTrait>::extendee(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field7StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field7StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field7StringType<'this>,
        >;
        fn default_value<'this>(&'this self) -> Option<Self::Field7StringType<'this>> {
            if let Some(right) = <U as FieldDescriptorProtoTrait>::default_value(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FieldDescriptorProtoTrait>::default_value(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        fn oneof_index<'this>(&'this self) -> Option<i32> {
            <U as FieldDescriptorProtoTrait>::oneof_index(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::oneof_index(&self.0))
        }
        type Field10StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field10StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field10StringType<'this>,
        >;
        fn json_name<'this>(&'this self) -> Option<Self::Field10StringType<'this>> {
            if let Some(right) = <U as FieldDescriptorProtoTrait>::json_name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FieldDescriptorProtoTrait>::json_name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field8MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
                <U as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
            >,
            (
                <T as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
                <U as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            match (
                <T as FieldDescriptorProtoTrait>::options(&self.0),
                <U as FieldDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        fn proto3_optional<'this>(&'this self) -> Option<bool> {
            <U as FieldDescriptorProtoTrait>::proto3_optional(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::proto3_optional(&self.0))
        }
    }
    impl<T, U> FieldDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: FieldDescriptorProtoTrait,
        U: FieldDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field1StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FieldDescriptorProtoTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        fn number<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::number(t),
                |u| <U as FieldDescriptorProtoTrait>::number(u),
            )
        }
        fn label<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::label(t),
                |u| <U as FieldDescriptorProtoTrait>::label(u),
            )
        }
        fn type_<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::type_(t),
                |u| <U as FieldDescriptorProtoTrait>::type_(u),
            )
        }
        type Field6StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field6StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field6StringType<'this>,
        >;
        fn type_name<'this>(&'this self) -> Option<Self::Field6StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FieldDescriptorProtoTrait>::type_name(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FieldDescriptorProtoTrait>::type_name(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field2StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field2StringType<'this>,
        >;
        fn extendee<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FieldDescriptorProtoTrait>::extendee(t).map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FieldDescriptorProtoTrait>::extendee(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field7StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field7StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field7StringType<'this>,
        >;
        fn default_value<'this>(&'this self) -> Option<Self::Field7StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FieldDescriptorProtoTrait>::default_value(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FieldDescriptorProtoTrait>::default_value(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn oneof_index<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::oneof_index(t),
                |u| <U as FieldDescriptorProtoTrait>::oneof_index(u),
            )
        }
        type Field10StringType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field10StringType<'this>,
            <U as FieldDescriptorProtoTrait>::Field10StringType<'this>,
        >;
        fn json_name<'this>(&'this self) -> Option<Self::Field10StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FieldDescriptorProtoTrait>::json_name(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FieldDescriptorProtoTrait>::json_name(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field8MessageType<'this> = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
            <U as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::options(t).map(|t| ::puroro::Either::Left(t)),
                |u| {
                    <U as FieldDescriptorProtoTrait>::options(u).map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn proto3_optional<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::proto3_optional(t),
                |u| <U as FieldDescriptorProtoTrait>::proto3_optional(u),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField3 {
        number: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        fn number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField4 {
        label: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        >,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        fn label<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            Clone::clone(&self.label)
        }
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField5 {
        type_: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        >,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        fn type_<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            Clone::clone(&self.type_)
        }
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField6 {
        type_name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField6 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField6 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'this str;
        fn type_name<'this>(&'this self) -> Option<Self::Field6StringType<'this>> {
            self.type_name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField2 {
        extendee: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'this str;
        fn extendee<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.extendee.as_ref().map(|v| v.as_ref())
        }
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField7 {
        default_value: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField7 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField7 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'this str;
        fn default_value<'this>(&'this self) -> Option<Self::Field7StringType<'this>> {
            self.default_value.as_ref().map(|v| v.as_ref())
        }
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField9 {
        oneof_index: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField9 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField9 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        fn oneof_index<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.oneof_index)
        }
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField10 {
        json_name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField10 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField10 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'this str;
        fn json_name<'this>(&'this self) -> Option<Self::Field10StringType<'this>> {
            self.json_name.as_ref().map(|v| v.as_ref())
        }
        type Field8MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField8 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::FieldOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField8 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField8 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FieldOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldDescriptorProtoSimpleField17 {
        proto3_optional: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FieldDescriptorProtoSimpleField17 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField17 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> = ();
        fn proto3_optional<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.proto3_optional)
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct OneofDescriptorProtoSimple {
        pub name: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::OneofOptionsSimple,
            >,
        >,
    }
    impl ::puroro::Message for OneofDescriptorProtoSimple {}

    impl OneofDescriptorProtoTrait for OneofDescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::OneofOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for OneofDescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for OneofDescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::_puroro_root::google::protobuf::_puroro_impls::OneofOptionsSimple,
                    >,
                >::deser_field(&mut self.options, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for OneofDescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::OneofOptionsSimple,
                >,
            >::ser_field(&self.options, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl OneofDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
    }
    impl<T, U> OneofDescriptorProtoTrait for (T, U)
    where
        T: OneofDescriptorProtoTrait,
        U: OneofDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as OneofDescriptorProtoTrait>::Field1StringType<'this>,
            <U as OneofDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as OneofDescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as OneofDescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field2MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
                <U as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
            >,
            (
                <T as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
                <U as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            match (
                <T as OneofDescriptorProtoTrait>::options(&self.0),
                <U as OneofDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
    }
    impl<T, U> OneofDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: OneofDescriptorProtoTrait,
        U: OneofDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as OneofDescriptorProtoTrait>::Field1StringType<'this>,
            <U as OneofDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as OneofDescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as OneofDescriptorProtoTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field2MessageType<'this> = ::puroro::Either<
            <T as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            self.as_ref().either(
                |t| <T as OneofDescriptorProtoTrait>::options(t).map(|t| ::puroro::Either::Left(t)),
                |u| {
                    <U as OneofDescriptorProtoTrait>::options(u).map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct OneofDescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for OneofDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::OneofDescriptorProtoTrait for OneofDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct OneofDescriptorProtoSimpleField2 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::OneofOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for OneofDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::OneofDescriptorProtoTrait for OneofDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::OneofOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumDescriptorProtoSimple {
    pub name: ::std::option::Option<::std::string::String>,
    pub value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple>,
    pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_impls::EnumOptionsSimple>>,
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
    impl ::puroro::Message for EnumDescriptorProtoSimple {}

    impl EnumDescriptorProtoTrait for EnumDescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple,
            >,
        >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.value.iter())
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple>>;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field5StringType<'this> = &'this str;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumDescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumDescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.name, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple>
            >::deser_field(&mut self.value, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::EnumOptionsSimple>
            >::deser_field(&mut self.options, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple>
            >::deser_field(&mut self.reserved_range, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.reserved_name, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for EnumDescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple>
        >::ser_field(&self.value, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::EnumOptionsSimple,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple>
        >::ser_field(&self.reserved_range, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.reserved_name, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl EnumDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> EnumDescriptorProtoTrait for (T, U)
    where
        T: EnumDescriptorProtoTrait,
        U: EnumDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field1StringType<'this>,
            <U as EnumDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as EnumDescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as EnumDescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field2MessageType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumDescriptorProtoTrait>::value(&self.0),
                <U as EnumDescriptorProtoTrait>::value(&self.1),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
                <U as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
            >,
            (
                <T as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
                <U as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as EnumDescriptorProtoTrait>::options(&self.0),
                <U as EnumDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumDescriptorProtoTrait>::reserved_range(&self.0),
                <U as EnumDescriptorProtoTrait>::reserved_range(&self.1),
            )
        }
        type Field5StringType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field5StringType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5StringType<'this>,
        >;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedLDField<
            <T as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedLDField::new(
                <T as EnumDescriptorProtoTrait>::reserved_name(&self.0),
                <U as EnumDescriptorProtoTrait>::reserved_name(&self.1),
            )
        }
    }
    impl<T, U> EnumDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: EnumDescriptorProtoTrait,
        U: EnumDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field1StringType<'this>,
            <U as EnumDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as EnumDescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as EnumDescriptorProtoTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field2MessageType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumDescriptorProtoTrait>::value(t))
                    .map_right(|u| <U as EnumDescriptorProtoTrait>::value(u)),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| <T as EnumDescriptorProtoTrait>::options(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as EnumDescriptorProtoTrait>::options(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumDescriptorProtoTrait>::reserved_range(t))
                    .map_right(|u| <U as EnumDescriptorProtoTrait>::reserved_range(u)),
            )
        }
        type Field5StringType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field5StringType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5StringType<'this>,
        >;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedLDField<
            <T as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumDescriptorProtoTrait>::reserved_name(t))
                    .map_right(|u| <U as EnumDescriptorProtoTrait>::reserved_name(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumDescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for EnumDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumDescriptorProtoSimpleField2 {
        value: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for EnumDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::EnumValueDescriptorProtoSimple,
            >,
        >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.value.iter())
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumDescriptorProtoSimpleField3 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::EnumOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for EnumDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumDescriptorProtoSimpleField4 {
    reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple>,
}

    impl ::puroro::Message for EnumDescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple;
        type Field4RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_impls::EnumReservedRangeSimple>>;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumDescriptorProtoSimpleField5 {
        reserved_name: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro::Message for EnumDescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'this str;
        type Field5RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumValueDescriptorProtoSimple {
        pub name: ::std::option::Option<::std::string::String>,
        pub number: ::std::option::Option<i32>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::EnumValueOptionsSimple,
            >,
        >,
    }
    impl ::puroro::Message for EnumValueDescriptorProtoSimple {}

    impl EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        fn number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumValueOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumValueDescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumValueDescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.number, data),
                3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::_puroro_root::google::protobuf::_puroro_impls::EnumValueOptionsSimple,
                    >,
                >::deser_field(&mut self.options, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for EnumValueDescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.number, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::EnumValueOptionsSimple,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl EnumValueDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field3MessageType<'this> = ();
    }
    impl<T, U> EnumValueDescriptorProtoTrait for (T, U)
    where
        T: EnumValueDescriptorProtoTrait,
        U: EnumValueDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as EnumValueDescriptorProtoTrait>::Field1StringType<'this>,
            <U as EnumValueDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as EnumValueDescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as EnumValueDescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        fn number<'this>(&'this self) -> Option<i32> {
            <U as EnumValueDescriptorProtoTrait>::number(&self.1)
                .or_else(|| <T as EnumValueDescriptorProtoTrait>::number(&self.0))
        }
        type Field3MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
                <U as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
            >,
            (
                <T as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
                <U as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as EnumValueDescriptorProtoTrait>::options(&self.0),
                <U as EnumValueDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
    }
    impl<T, U> EnumValueDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: EnumValueDescriptorProtoTrait,
        U: EnumValueDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as EnumValueDescriptorProtoTrait>::Field1StringType<'this>,
            <U as EnumValueDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as EnumValueDescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as EnumValueDescriptorProtoTrait>::name(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn number<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as EnumValueDescriptorProtoTrait>::number(t),
                |u| <U as EnumValueDescriptorProtoTrait>::number(u),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
            <U as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as EnumValueDescriptorProtoTrait>::options(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as EnumValueDescriptorProtoTrait>::options(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumValueDescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for EnumValueDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field3MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumValueDescriptorProtoSimpleField2 {
        number: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for EnumValueDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        fn number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        type Field3MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumValueDescriptorProtoSimpleField3 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::EnumValueOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for EnumValueDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::EnumValueOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ServiceDescriptorProtoSimple {
        pub name: ::std::option::Option<::std::string::String>,
        pub method: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple,
        >,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::ServiceOptionsSimple,
            >,
        >,
    }
    impl ::puroro::Message for ServiceDescriptorProtoSimple {}

    impl ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple,
            >,
        >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.method.iter())
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::ServiceOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for ServiceDescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for ServiceDescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.name, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple>
            >::deser_field(&mut self.method, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::ServiceOptionsSimple>
            >::deser_field(&mut self.options, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for ServiceDescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple>
        >::ser_field(&self.method, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::ServiceOptionsSimple,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl ServiceDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }
    impl<T, U> ServiceDescriptorProtoTrait for (T, U)
    where
        T: ServiceDescriptorProtoTrait,
        U: ServiceDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field1StringType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as ServiceDescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as ServiceDescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field2MessageType<'this> = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as ServiceDescriptorProtoTrait>::method(&self.0),
                <U as ServiceDescriptorProtoTrait>::method(&self.1),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
                <U as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
            >,
            (
                <T as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
                <U as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as ServiceDescriptorProtoTrait>::options(&self.0),
                <U as ServiceDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
    }
    impl<T, U> ServiceDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: ServiceDescriptorProtoTrait,
        U: ServiceDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field1StringType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as ServiceDescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as ServiceDescriptorProtoTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field2MessageType<'this> = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as ServiceDescriptorProtoTrait>::method(t))
                    .map_right(|u| <U as ServiceDescriptorProtoTrait>::method(u)),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as ServiceDescriptorProtoTrait>::options(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as ServiceDescriptorProtoTrait>::options(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct ServiceDescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for ServiceDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct ServiceDescriptorProtoSimpleField2 {
        method: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for ServiceDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::MethodDescriptorProtoSimple,
            >,
        >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.method.iter())
        }
        type Field3MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct ServiceDescriptorProtoSimpleField3 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::ServiceOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for ServiceDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::ServiceOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MethodDescriptorProtoSimple {
        pub name: ::std::option::Option<::std::string::String>,
        pub input_type: ::std::option::Option<::std::string::String>,
        pub output_type: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::MethodOptionsSimple,
            >,
        >,
        pub client_streaming: ::std::option::Option<bool>,
        pub server_streaming: ::std::option::Option<bool>,
    }
    impl ::puroro::Message for MethodDescriptorProtoSimple {}

    impl MethodDescriptorProtoTrait for MethodDescriptorProtoSimple {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'this str;
        fn input_type<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.input_type.as_ref().map(|v| v.as_ref())
        }
        type Field3StringType<'this> = &'this str;
        fn output_type<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.output_type.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::MethodOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        fn client_streaming<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.client_streaming)
        }
        fn server_streaming<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.server_streaming)
        }
    }

    impl ::puroro::DeserFromBytesIter for MethodDescriptorProtoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MethodDescriptorProtoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
                1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.name, data),
                2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.input_type, data),
                3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.output_type, data),
                4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Message<
                        self::_puroro_root::google::protobuf::_puroro_impls::MethodOptionsSimple,
                    >,
                >::deser_field(&mut self.options, data),
                5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.client_streaming, data),
                6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >::deser_field(&mut self.server_streaming, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for MethodDescriptorProtoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.name, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.input_type, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.output_type, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::MethodOptionsSimple,
                >,
            >::ser_field(&self.options, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.client_streaming, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.server_streaming, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl MethodDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
    }
    impl<T, U> MethodDescriptorProtoTrait for (T, U)
    where
        T: MethodDescriptorProtoTrait,
        U: MethodDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field1StringType<'this>,
            <U as MethodDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as MethodDescriptorProtoTrait>::name(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MethodDescriptorProtoTrait>::name(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field2StringType<'this>,
            <U as MethodDescriptorProtoTrait>::Field2StringType<'this>,
        >;
        fn input_type<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            if let Some(right) = <U as MethodDescriptorProtoTrait>::input_type(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MethodDescriptorProtoTrait>::input_type(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field3StringType<'this> = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field3StringType<'this>,
            <U as MethodDescriptorProtoTrait>::Field3StringType<'this>,
        >;
        fn output_type<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            if let Some(right) = <U as MethodDescriptorProtoTrait>::output_type(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as MethodDescriptorProtoTrait>::output_type(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field4MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
                <U as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
            >,
            (
                <T as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
                <U as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
            ),
        >;
        fn options<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            match (
                <T as MethodDescriptorProtoTrait>::options(&self.0),
                <U as MethodDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
        fn client_streaming<'this>(&'this self) -> Option<bool> {
            <U as MethodDescriptorProtoTrait>::client_streaming(&self.1)
                .or_else(|| <T as MethodDescriptorProtoTrait>::client_streaming(&self.0))
        }
        fn server_streaming<'this>(&'this self) -> Option<bool> {
            <U as MethodDescriptorProtoTrait>::server_streaming(&self.1)
                .or_else(|| <T as MethodDescriptorProtoTrait>::server_streaming(&self.0))
        }
    }
    impl<T, U> MethodDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: MethodDescriptorProtoTrait,
        U: MethodDescriptorProtoTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field1StringType<'this>,
            <U as MethodDescriptorProtoTrait>::Field1StringType<'this>,
        >;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as MethodDescriptorProtoTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field2StringType<'this>,
            <U as MethodDescriptorProtoTrait>::Field2StringType<'this>,
        >;
        fn input_type<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as MethodDescriptorProtoTrait>::input_type(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as MethodDescriptorProtoTrait>::input_type(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field3StringType<'this> = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field3StringType<'this>,
            <U as MethodDescriptorProtoTrait>::Field3StringType<'this>,
        >;
        fn output_type<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as MethodDescriptorProtoTrait>::output_type(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as MethodDescriptorProtoTrait>::output_type(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as MethodDescriptorProtoTrait>::options(t).map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as MethodDescriptorProtoTrait>::options(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn client_streaming<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::client_streaming(t),
                |u| <U as MethodDescriptorProtoTrait>::client_streaming(u),
            )
        }
        fn server_streaming<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::server_streaming(t),
                |u| <U as MethodDescriptorProtoTrait>::server_streaming(u),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodDescriptorProtoSimpleField1 {
        name: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for MethodDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodDescriptorProtoSimpleField2 {
        input_type: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for MethodDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'this str;
        fn input_type<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.input_type.as_ref().map(|v| v.as_ref())
        }
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodDescriptorProtoSimpleField3 {
        output_type: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for MethodDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'this str;
        fn output_type<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.output_type.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodDescriptorProtoSimpleField4 {
        options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_impls::MethodOptionsSimple,
            >,
        >,
    }

    impl ::puroro::Message for MethodDescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::MethodOptionsSimple;
        fn options<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodDescriptorProtoSimpleField5 {
        client_streaming: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for MethodDescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
        fn client_streaming<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.client_streaming)
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodDescriptorProtoSimpleField6 {
        server_streaming: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for MethodDescriptorProtoSimpleField6 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField6 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
        fn server_streaming<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.server_streaming)
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileOptionsSimple {
        pub java_package: ::std::option::Option<::std::string::String>,
        pub java_outer_classname: ::std::option::Option<::std::string::String>,
        pub java_multiple_files: ::std::option::Option<bool>,
        pub java_generate_equals_and_hash: ::std::option::Option<bool>,
        pub java_string_check_utf8: ::std::option::Option<bool>,
        pub optimize_for: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
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
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for FileOptionsSimple {}

    impl FileOptionsTrait for FileOptionsSimple {
        type Field1StringType<'this> = &'this str;
        fn java_package<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.java_package.as_ref().map(|v| v.as_ref())
        }
        type Field8StringType<'this> = &'this str;
        fn java_outer_classname<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            self.java_outer_classname.as_ref().map(|v| v.as_ref())
        }
        fn java_multiple_files<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_multiple_files)
        }
        fn java_generate_equals_and_hash<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_generate_equals_and_hash)
        }
        fn java_string_check_utf8<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_string_check_utf8)
        }
        fn optimize_for<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        {
            Clone::clone(&self.optimize_for)
        }
        type Field11StringType<'this> = &'this str;
        fn go_package<'this>(&'this self) -> Option<Self::Field11StringType<'this>> {
            self.go_package.as_ref().map(|v| v.as_ref())
        }
        fn cc_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.cc_generic_services)
        }
        fn java_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_generic_services)
        }
        fn py_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.py_generic_services)
        }
        fn php_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.php_generic_services)
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn cc_enable_arenas<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.cc_enable_arenas)
        }
        type Field36StringType<'this> = &'this str;
        fn objc_class_prefix<'this>(&'this self) -> Option<Self::Field36StringType<'this>> {
            self.objc_class_prefix.as_ref().map(|v| v.as_ref())
        }
        type Field37StringType<'this> = &'this str;
        fn csharp_namespace<'this>(&'this self) -> Option<Self::Field37StringType<'this>> {
            self.csharp_namespace.as_ref().map(|v| v.as_ref())
        }
        type Field39StringType<'this> = &'this str;
        fn swift_prefix<'this>(&'this self) -> Option<Self::Field39StringType<'this>> {
            self.swift_prefix.as_ref().map(|v| v.as_ref())
        }
        type Field40StringType<'this> = &'this str;
        fn php_class_prefix<'this>(&'this self) -> Option<Self::Field40StringType<'this>> {
            self.php_class_prefix.as_ref().map(|v| v.as_ref())
        }
        type Field41StringType<'this> = &'this str;
        fn php_namespace<'this>(&'this self) -> Option<Self::Field41StringType<'this>> {
            self.php_namespace.as_ref().map(|v| v.as_ref())
        }
        type Field44StringType<'this> = &'this str;
        fn php_metadata_namespace<'this>(&'this self) -> Option<Self::Field44StringType<'this>> {
            self.php_metadata_namespace.as_ref().map(|v| v.as_ref())
        }
        type Field45StringType<'this> = &'this str;
        fn ruby_package<'this>(&'this self) -> Option<Self::Field45StringType<'this>> {
            self.ruby_package.as_ref().map(|v| v.as_ref())
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for FileOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FileOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.java_package, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.java_outer_classname, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_multiple_files, data),
            20 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_generate_equals_and_hash, data),
            27 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_string_check_utf8, data),
            9 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
            >::deser_field(&mut self.optimize_for, data),
            11 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.go_package, data),
            16 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.cc_generic_services, data),
            17 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_generic_services, data),
            18 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.py_generic_services, data),
            42 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.php_generic_services, data),
            23 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            31 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.cc_enable_arenas, data),
            36 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.objc_class_prefix, data),
            37 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.csharp_namespace, data),
            39 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.swift_prefix, data),
            40 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.php_class_prefix, data),
            41 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.php_namespace, data),
            44 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.php_metadata_namespace, data),
            45 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.ruby_package, data),
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for FileOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.java_package, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.java_outer_classname, 8, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_multiple_files, 10, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_generate_equals_and_hash, 20, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_string_check_utf8, 27, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        >::ser_field(&self.optimize_for, 9, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.go_package, 11, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.cc_generic_services, 16, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.java_generic_services, 17, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.py_generic_services, 18, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.php_generic_services, 42, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 23, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.cc_enable_arenas, 31, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.objc_class_prefix, 36, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.csharp_namespace, 37, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.swift_prefix, 39, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.php_class_prefix, 40, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.php_namespace, 41, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.php_metadata_namespace, 44, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.ruby_package, 45, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl FileOptionsTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> FileOptionsTrait for (T, U)
    where
        T: FileOptionsTrait,
        U: FileOptionsTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field1StringType<'this>,
            <U as FileOptionsTrait>::Field1StringType<'this>,
        >;
        fn java_package<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::java_package(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::java_package(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field8StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field8StringType<'this>,
            <U as FileOptionsTrait>::Field8StringType<'this>,
        >;
        fn java_outer_classname<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::java_outer_classname(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::java_outer_classname(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        fn java_multiple_files<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_multiple_files(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_multiple_files(&self.0))
        }
        fn java_generate_equals_and_hash<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_generate_equals_and_hash(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_generate_equals_and_hash(&self.0))
        }
        fn java_string_check_utf8<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_string_check_utf8(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_string_check_utf8(&self.0))
        }
        fn optimize_for<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        {
            <U as FileOptionsTrait>::optimize_for(&self.1)
                .or_else(|| <T as FileOptionsTrait>::optimize_for(&self.0))
        }
        type Field11StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field11StringType<'this>,
            <U as FileOptionsTrait>::Field11StringType<'this>,
        >;
        fn go_package<'this>(&'this self) -> Option<Self::Field11StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::go_package(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::go_package(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        fn cc_generic_services<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::cc_generic_services(&self.1)
                .or_else(|| <T as FileOptionsTrait>::cc_generic_services(&self.0))
        }
        fn java_generic_services<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_generic_services(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_generic_services(&self.0))
        }
        fn py_generic_services<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::py_generic_services(&self.1)
                .or_else(|| <T as FileOptionsTrait>::py_generic_services(&self.0))
        }
        fn php_generic_services<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::php_generic_services(&self.1)
                .or_else(|| <T as FileOptionsTrait>::php_generic_services(&self.0))
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::deprecated(&self.1)
                .or_else(|| <T as FileOptionsTrait>::deprecated(&self.0))
        }
        fn cc_enable_arenas<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::cc_enable_arenas(&self.1)
                .or_else(|| <T as FileOptionsTrait>::cc_enable_arenas(&self.0))
        }
        type Field36StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field36StringType<'this>,
            <U as FileOptionsTrait>::Field36StringType<'this>,
        >;
        fn objc_class_prefix<'this>(&'this self) -> Option<Self::Field36StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::objc_class_prefix(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::objc_class_prefix(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field37StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field37StringType<'this>,
            <U as FileOptionsTrait>::Field37StringType<'this>,
        >;
        fn csharp_namespace<'this>(&'this self) -> Option<Self::Field37StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::csharp_namespace(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::csharp_namespace(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field39StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field39StringType<'this>,
            <U as FileOptionsTrait>::Field39StringType<'this>,
        >;
        fn swift_prefix<'this>(&'this self) -> Option<Self::Field39StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::swift_prefix(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::swift_prefix(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field40StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field40StringType<'this>,
            <U as FileOptionsTrait>::Field40StringType<'this>,
        >;
        fn php_class_prefix<'this>(&'this self) -> Option<Self::Field40StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::php_class_prefix(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::php_class_prefix(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field41StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field41StringType<'this>,
            <U as FileOptionsTrait>::Field41StringType<'this>,
        >;
        fn php_namespace<'this>(&'this self) -> Option<Self::Field41StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::php_namespace(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::php_namespace(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field44StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field44StringType<'this>,
            <U as FileOptionsTrait>::Field44StringType<'this>,
        >;
        fn php_metadata_namespace<'this>(&'this self) -> Option<Self::Field44StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::php_metadata_namespace(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::php_metadata_namespace(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field45StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field45StringType<'this>,
            <U as FileOptionsTrait>::Field45StringType<'this>,
        >;
        fn ruby_package<'this>(&'this self) -> Option<Self::Field45StringType<'this>> {
            if let Some(right) = <U as FileOptionsTrait>::ruby_package(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as FileOptionsTrait>::ruby_package(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field999MessageType<'this>,
            <U as FileOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as FileOptionsTrait>::Field999RepeatedType<'this>,
                <U as FileOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileOptionsTrait>::uninterpreted_option(&self.0),
                <U as FileOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> FileOptionsTrait for ::puroro::Either<T, U>
    where
        T: FileOptionsTrait,
        U: FileOptionsTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field1StringType<'this>,
            <U as FileOptionsTrait>::Field1StringType<'this>,
        >;
        fn java_package<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_package(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileOptionsTrait>::java_package(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field8StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field8StringType<'this>,
            <U as FileOptionsTrait>::Field8StringType<'this>,
        >;
        fn java_outer_classname<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FileOptionsTrait>::java_outer_classname(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FileOptionsTrait>::java_outer_classname(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn java_multiple_files<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_multiple_files(t),
                |u| <U as FileOptionsTrait>::java_multiple_files(u),
            )
        }
        fn java_generate_equals_and_hash<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_generate_equals_and_hash(t),
                |u| <U as FileOptionsTrait>::java_generate_equals_and_hash(u),
            )
        }
        fn java_string_check_utf8<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_string_check_utf8(t),
                |u| <U as FileOptionsTrait>::java_string_check_utf8(u),
            )
        }
        fn optimize_for<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::optimize_for(t),
                |u| <U as FileOptionsTrait>::optimize_for(u),
            )
        }
        type Field11StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field11StringType<'this>,
            <U as FileOptionsTrait>::Field11StringType<'this>,
        >;
        fn go_package<'this>(&'this self) -> Option<Self::Field11StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::go_package(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileOptionsTrait>::go_package(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        fn cc_generic_services<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::cc_generic_services(t),
                |u| <U as FileOptionsTrait>::cc_generic_services(u),
            )
        }
        fn java_generic_services<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_generic_services(t),
                |u| <U as FileOptionsTrait>::java_generic_services(u),
            )
        }
        fn py_generic_services<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::py_generic_services(t),
                |u| <U as FileOptionsTrait>::py_generic_services(u),
            )
        }
        fn php_generic_services<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::php_generic_services(t),
                |u| <U as FileOptionsTrait>::php_generic_services(u),
            )
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::deprecated(t),
                |u| <U as FileOptionsTrait>::deprecated(u),
            )
        }
        fn cc_enable_arenas<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::cc_enable_arenas(t),
                |u| <U as FileOptionsTrait>::cc_enable_arenas(u),
            )
        }
        type Field36StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field36StringType<'this>,
            <U as FileOptionsTrait>::Field36StringType<'this>,
        >;
        fn objc_class_prefix<'this>(&'this self) -> Option<Self::Field36StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FileOptionsTrait>::objc_class_prefix(t).map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FileOptionsTrait>::objc_class_prefix(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field37StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field37StringType<'this>,
            <U as FileOptionsTrait>::Field37StringType<'this>,
        >;
        fn csharp_namespace<'this>(&'this self) -> Option<Self::Field37StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::csharp_namespace(t).map(|t| ::puroro::Either::Left(t)),
                |u| {
                    <U as FileOptionsTrait>::csharp_namespace(u).map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field39StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field39StringType<'this>,
            <U as FileOptionsTrait>::Field39StringType<'this>,
        >;
        fn swift_prefix<'this>(&'this self) -> Option<Self::Field39StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::swift_prefix(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileOptionsTrait>::swift_prefix(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field40StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field40StringType<'this>,
            <U as FileOptionsTrait>::Field40StringType<'this>,
        >;
        fn php_class_prefix<'this>(&'this self) -> Option<Self::Field40StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::php_class_prefix(t).map(|t| ::puroro::Either::Left(t)),
                |u| {
                    <U as FileOptionsTrait>::php_class_prefix(u).map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field41StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field41StringType<'this>,
            <U as FileOptionsTrait>::Field41StringType<'this>,
        >;
        fn php_namespace<'this>(&'this self) -> Option<Self::Field41StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::php_namespace(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileOptionsTrait>::php_namespace(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field44StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field44StringType<'this>,
            <U as FileOptionsTrait>::Field44StringType<'this>,
        >;
        fn php_metadata_namespace<'this>(&'this self) -> Option<Self::Field44StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FileOptionsTrait>::php_metadata_namespace(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FileOptionsTrait>::php_metadata_namespace(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field45StringType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field45StringType<'this>,
            <U as FileOptionsTrait>::Field45StringType<'this>,
        >;
        fn ruby_package<'this>(&'this self) -> Option<Self::Field45StringType<'this>> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::ruby_package(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as FileOptionsTrait>::ruby_package(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as FileOptionsTrait>::Field999MessageType<'this>,
            <U as FileOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as FileOptionsTrait>::Field999RepeatedType<'this>,
                <U as FileOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as FileOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField1 {
        java_package: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField1 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn java_package<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.java_package.as_ref().map(|v| v.as_ref())
        }
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField8 {
        java_outer_classname: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField8 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField8 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'this str;
        fn java_outer_classname<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            self.java_outer_classname.as_ref().map(|v| v.as_ref())
        }
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField10 {
        java_multiple_files: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField10 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField10 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        fn java_multiple_files<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_multiple_files)
        }
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField20 {
        java_generate_equals_and_hash: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField20 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField20 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        fn java_generate_equals_and_hash<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_generate_equals_and_hash)
        }
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField27 {
        java_string_check_utf8: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField27 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField27 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        fn java_string_check_utf8<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_string_check_utf8)
        }
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField9 {
        optimize_for: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        >,
    }

    impl ::puroro::Message for FileOptionsSimpleField9 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField9 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        fn optimize_for<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        {
            Clone::clone(&self.optimize_for)
        }
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField11 {
        go_package: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField11 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField11 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'this str;
        fn go_package<'this>(&'this self) -> Option<Self::Field11StringType<'this>> {
            self.go_package.as_ref().map(|v| v.as_ref())
        }
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField16 {
        cc_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField16 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField16 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        fn cc_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.cc_generic_services)
        }
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField17 {
        java_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField17 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField17 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        fn java_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_generic_services)
        }
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField18 {
        py_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField18 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField18 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        fn py_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.py_generic_services)
        }
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField42 {
        php_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField42 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField42 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        fn php_generic_services<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.php_generic_services)
        }
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField23 {
        deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField23 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField23 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField31 {
        cc_enable_arenas: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FileOptionsSimpleField31 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField31 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        fn cc_enable_arenas<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.cc_enable_arenas)
        }
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField36 {
        objc_class_prefix: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField36 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField36 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'this str;
        fn objc_class_prefix<'this>(&'this self) -> Option<Self::Field36StringType<'this>> {
            self.objc_class_prefix.as_ref().map(|v| v.as_ref())
        }
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField37 {
        csharp_namespace: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField37 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField37 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'this str;
        fn csharp_namespace<'this>(&'this self) -> Option<Self::Field37StringType<'this>> {
            self.csharp_namespace.as_ref().map(|v| v.as_ref())
        }
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField39 {
        swift_prefix: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField39 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField39 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'this str;
        fn swift_prefix<'this>(&'this self) -> Option<Self::Field39StringType<'this>> {
            self.swift_prefix.as_ref().map(|v| v.as_ref())
        }
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField40 {
        php_class_prefix: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField40 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField40 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'this str;
        fn php_class_prefix<'this>(&'this self) -> Option<Self::Field40StringType<'this>> {
            self.php_class_prefix.as_ref().map(|v| v.as_ref())
        }
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField41 {
        php_namespace: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField41 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField41 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'this str;
        fn php_namespace<'this>(&'this self) -> Option<Self::Field41StringType<'this>> {
            self.php_namespace.as_ref().map(|v| v.as_ref())
        }
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField44 {
        php_metadata_namespace: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField44 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField44 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'this str;
        fn php_metadata_namespace<'this>(&'this self) -> Option<Self::Field44StringType<'this>> {
            self.php_metadata_namespace.as_ref().map(|v| v.as_ref())
        }
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField45 {
        ruby_package: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for FileOptionsSimpleField45 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField45 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'this str;
        fn ruby_package<'this>(&'this self) -> Option<Self::Field45StringType<'this>> {
            self.ruby_package.as_ref().map(|v| v.as_ref())
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FileOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for FileOptionsSimpleField999 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSimpleField999 {
        type Field1StringType<'this> = &'static str;
        type Field8StringType<'this> = &'static str;
        type Field11StringType<'this> = &'static str;
        type Field36StringType<'this> = &'static str;
        type Field37StringType<'this> = &'static str;
        type Field39StringType<'this> = &'static str;
        type Field40StringType<'this> = &'static str;
        type Field41StringType<'this> = &'static str;
        type Field44StringType<'this> = &'static str;
        type Field45StringType<'this> = &'static str;
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MessageOptionsSimple {
        pub message_set_wire_format: ::std::option::Option<bool>,
        pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub map_entry: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for MessageOptionsSimple {}

    impl MessageOptionsTrait for MessageOptionsSimple {
        fn message_set_wire_format<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.message_set_wire_format)
        }
        fn no_standard_descriptor_accessor<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.no_standard_descriptor_accessor)
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn map_entry<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.map_entry)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for MessageOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MessageOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.message_set_wire_format, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.no_standard_descriptor_accessor, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.map_entry, data),
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for MessageOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.message_set_wire_format, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.no_standard_descriptor_accessor, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.map_entry, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl MessageOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MessageOptionsTrait for (T, U)
    where
        T: MessageOptionsTrait,
        U: MessageOptionsTrait,
    {
        fn message_set_wire_format<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::message_set_wire_format(&self.1)
                .or_else(|| <T as MessageOptionsTrait>::message_set_wire_format(&self.0))
        }
        fn no_standard_descriptor_accessor<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::no_standard_descriptor_accessor(&self.1)
                .or_else(|| <T as MessageOptionsTrait>::no_standard_descriptor_accessor(&self.0))
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::deprecated(&self.1)
                .or_else(|| <T as MessageOptionsTrait>::deprecated(&self.0))
        }
        fn map_entry<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::map_entry(&self.1)
                .or_else(|| <T as MessageOptionsTrait>::map_entry(&self.0))
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as MessageOptionsTrait>::Field999MessageType<'this>,
            <U as MessageOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as MessageOptionsTrait>::Field999RepeatedType<'this>,
                <U as MessageOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MessageOptionsTrait>::uninterpreted_option(&self.0),
                <U as MessageOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> MessageOptionsTrait for ::puroro::Either<T, U>
    where
        T: MessageOptionsTrait,
        U: MessageOptionsTrait,
    {
        fn message_set_wire_format<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::message_set_wire_format(t),
                |u| <U as MessageOptionsTrait>::message_set_wire_format(u),
            )
        }
        fn no_standard_descriptor_accessor<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::no_standard_descriptor_accessor(t),
                |u| <U as MessageOptionsTrait>::no_standard_descriptor_accessor(u),
            )
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::deprecated(t),
                |u| <U as MessageOptionsTrait>::deprecated(u),
            )
        }
        fn map_entry<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::map_entry(t),
                |u| <U as MessageOptionsTrait>::map_entry(u),
            )
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as MessageOptionsTrait>::Field999MessageType<'this>,
            <U as MessageOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as MessageOptionsTrait>::Field999RepeatedType<'this>,
                <U as MessageOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MessageOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as MessageOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MessageOptionsSimpleField1 {
        message_set_wire_format: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for MessageOptionsSimpleField1 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField1 {
        fn message_set_wire_format<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.message_set_wire_format)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MessageOptionsSimpleField2 {
        no_standard_descriptor_accessor: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for MessageOptionsSimpleField2 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField2 {
        fn no_standard_descriptor_accessor<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.no_standard_descriptor_accessor)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MessageOptionsSimpleField3 {
        deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for MessageOptionsSimpleField3 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField3 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MessageOptionsSimpleField7 {
        map_entry: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for MessageOptionsSimpleField7 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField7 {
        fn map_entry<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.map_entry)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MessageOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for MessageOptionsSimpleField999 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FieldOptionsSimple {
        pub ctype: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        >,
        pub packed: ::std::option::Option<bool>,
        pub jstype: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        >,
        pub lazy: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub weak: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for FieldOptionsSimple {}

    impl FieldOptionsTrait for FieldOptionsSimple {
        fn ctype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            Clone::clone(&self.ctype)
        }
        fn packed<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.packed)
        }
        fn jstype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            Clone::clone(&self.jstype)
        }
        fn lazy<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.lazy)
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn weak<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.weak)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for FieldOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for FieldOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
            >::deser_field(&mut self.ctype, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.packed, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
            >::deser_field(&mut self.jstype, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.lazy, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            10 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.weak, data),
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for FieldOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
                >,
            >::ser_field(&self.ctype, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.packed, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
                >,
            >::ser_field(&self.jstype, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.lazy, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.weak, 10, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl FieldOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> FieldOptionsTrait for (T, U)
    where
        T: FieldOptionsTrait,
        U: FieldOptionsTrait,
    {
        fn ctype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            <U as FieldOptionsTrait>::ctype(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::ctype(&self.0))
        }
        fn packed<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::packed(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::packed(&self.0))
        }
        fn jstype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            <U as FieldOptionsTrait>::jstype(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::jstype(&self.0))
        }
        fn lazy<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::lazy(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::lazy(&self.0))
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::deprecated(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::deprecated(&self.0))
        }
        fn weak<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::weak(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::weak(&self.0))
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as FieldOptionsTrait>::Field999MessageType<'this>,
            <U as FieldOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as FieldOptionsTrait>::Field999RepeatedType<'this>,
                <U as FieldOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FieldOptionsTrait>::uninterpreted_option(&self.0),
                <U as FieldOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> FieldOptionsTrait for ::puroro::Either<T, U>
    where
        T: FieldOptionsTrait,
        U: FieldOptionsTrait,
    {
        fn ctype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::ctype(t),
                |u| <U as FieldOptionsTrait>::ctype(u),
            )
        }
        fn packed<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::packed(t),
                |u| <U as FieldOptionsTrait>::packed(u),
            )
        }
        fn jstype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::jstype(t),
                |u| <U as FieldOptionsTrait>::jstype(u),
            )
        }
        fn lazy<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::lazy(t),
                |u| <U as FieldOptionsTrait>::lazy(u),
            )
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::deprecated(t),
                |u| <U as FieldOptionsTrait>::deprecated(u),
            )
        }
        fn weak<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::weak(t),
                |u| <U as FieldOptionsTrait>::weak(u),
            )
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as FieldOptionsTrait>::Field999MessageType<'this>,
            <U as FieldOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as FieldOptionsTrait>::Field999RepeatedType<'this>,
                <U as FieldOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FieldOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as FieldOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldOptionsSimpleField1 {
        ctype: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        >,
    }

    impl ::puroro::Message for FieldOptionsSimpleField1 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField1 {
        fn ctype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            Clone::clone(&self.ctype)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldOptionsSimpleField2 {
        packed: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FieldOptionsSimpleField2 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField2 {
        fn packed<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.packed)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldOptionsSimpleField6 {
        jstype: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        >,
    }

    impl ::puroro::Message for FieldOptionsSimpleField6 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField6 {
        fn jstype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            Clone::clone(&self.jstype)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldOptionsSimpleField5 {
        lazy: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FieldOptionsSimpleField5 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField5 {
        fn lazy<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.lazy)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldOptionsSimpleField3 {
        deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FieldOptionsSimpleField3 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField3 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldOptionsSimpleField10 {
        weak: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for FieldOptionsSimpleField10 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField10 {
        fn weak<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.weak)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct FieldOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for FieldOptionsSimpleField999 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct OneofOptionsSimple {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for OneofOptionsSimple {}

    impl OneofOptionsTrait for OneofOptionsSimple {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for OneofOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for OneofOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for OneofOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl OneofOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> OneofOptionsTrait for (T, U)
    where
        T: OneofOptionsTrait,
        U: OneofOptionsTrait,
    {
        type Field999MessageType<'this> = ::puroro::Either<
            <T as OneofOptionsTrait>::Field999MessageType<'this>,
            <U as OneofOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as OneofOptionsTrait>::Field999RepeatedType<'this>,
                <U as OneofOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as OneofOptionsTrait>::uninterpreted_option(&self.0),
                <U as OneofOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> OneofOptionsTrait for ::puroro::Either<T, U>
    where
        T: OneofOptionsTrait,
        U: OneofOptionsTrait,
    {
        type Field999MessageType<'this> = ::puroro::Either<
            <T as OneofOptionsTrait>::Field999MessageType<'this>,
            <U as OneofOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as OneofOptionsTrait>::Field999RepeatedType<'this>,
                <U as OneofOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as OneofOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as OneofOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct OneofOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for OneofOptionsSimpleField999 {}

    impl super::_puroro_traits::OneofOptionsTrait for OneofOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumOptionsSimple {
        pub allow_alias: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for EnumOptionsSimple {}

    impl EnumOptionsTrait for EnumOptionsSimple {
        fn allow_alias<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.allow_alias)
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.allow_alias, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for EnumOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.allow_alias, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl EnumOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> EnumOptionsTrait for (T, U)
    where
        T: EnumOptionsTrait,
        U: EnumOptionsTrait,
    {
        fn allow_alias<'this>(&'this self) -> Option<bool> {
            <U as EnumOptionsTrait>::allow_alias(&self.1)
                .or_else(|| <T as EnumOptionsTrait>::allow_alias(&self.0))
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            <U as EnumOptionsTrait>::deprecated(&self.1)
                .or_else(|| <T as EnumOptionsTrait>::deprecated(&self.0))
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as EnumOptionsTrait>::Field999MessageType<'this>,
            <U as EnumOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumOptionsTrait>::uninterpreted_option(&self.0),
                <U as EnumOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> EnumOptionsTrait for ::puroro::Either<T, U>
    where
        T: EnumOptionsTrait,
        U: EnumOptionsTrait,
    {
        fn allow_alias<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as EnumOptionsTrait>::allow_alias(t),
                |u| <U as EnumOptionsTrait>::allow_alias(u),
            )
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as EnumOptionsTrait>::deprecated(t),
                |u| <U as EnumOptionsTrait>::deprecated(u),
            )
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as EnumOptionsTrait>::Field999MessageType<'this>,
            <U as EnumOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as EnumOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as EnumOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumOptionsSimpleField2 {
        allow_alias: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for EnumOptionsSimpleField2 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSimpleField2 {
        fn allow_alias<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.allow_alias)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumOptionsSimpleField3 {
        deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for EnumOptionsSimpleField3 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSimpleField3 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for EnumOptionsSimpleField999 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumValueOptionsSimple {
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for EnumValueOptionsSimple {}

    impl EnumValueOptionsTrait for EnumValueOptionsSimple {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for EnumValueOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumValueOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for EnumValueOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl EnumValueOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> EnumValueOptionsTrait for (T, U)
    where
        T: EnumValueOptionsTrait,
        U: EnumValueOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            <U as EnumValueOptionsTrait>::deprecated(&self.1)
                .or_else(|| <T as EnumValueOptionsTrait>::deprecated(&self.0))
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as EnumValueOptionsTrait>::Field999MessageType<'this>,
            <U as EnumValueOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumValueOptionsTrait>::uninterpreted_option(&self.0),
                <U as EnumValueOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> EnumValueOptionsTrait for ::puroro::Either<T, U>
    where
        T: EnumValueOptionsTrait,
        U: EnumValueOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as EnumValueOptionsTrait>::deprecated(t),
                |u| <U as EnumValueOptionsTrait>::deprecated(u),
            )
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as EnumValueOptionsTrait>::Field999MessageType<'this>,
            <U as EnumValueOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumValueOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as EnumValueOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumValueOptionsSimpleField1 {
        deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for EnumValueOptionsSimpleField1 {}

    impl super::_puroro_traits::EnumValueOptionsTrait for EnumValueOptionsSimpleField1 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct EnumValueOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for EnumValueOptionsSimpleField999 {}

    impl super::_puroro_traits::EnumValueOptionsTrait for EnumValueOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ServiceOptionsSimple {
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for ServiceOptionsSimple {}

    impl ServiceOptionsTrait for ServiceOptionsSimple {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for ServiceOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for ServiceOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            33 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for ServiceOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 33, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl ServiceOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> ServiceOptionsTrait for (T, U)
    where
        T: ServiceOptionsTrait,
        U: ServiceOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            <U as ServiceOptionsTrait>::deprecated(&self.1)
                .or_else(|| <T as ServiceOptionsTrait>::deprecated(&self.0))
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as ServiceOptionsTrait>::Field999MessageType<'this>,
            <U as ServiceOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as ServiceOptionsTrait>::Field999RepeatedType<'this>,
                <U as ServiceOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as ServiceOptionsTrait>::uninterpreted_option(&self.0),
                <U as ServiceOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> ServiceOptionsTrait for ::puroro::Either<T, U>
    where
        T: ServiceOptionsTrait,
        U: ServiceOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as ServiceOptionsTrait>::deprecated(t),
                |u| <U as ServiceOptionsTrait>::deprecated(u),
            )
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as ServiceOptionsTrait>::Field999MessageType<'this>,
            <U as ServiceOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as ServiceOptionsTrait>::Field999RepeatedType<'this>,
                <U as ServiceOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as ServiceOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as ServiceOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct ServiceOptionsSimpleField33 {
        deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for ServiceOptionsSimpleField33 {}

    impl super::_puroro_traits::ServiceOptionsTrait for ServiceOptionsSimpleField33 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct ServiceOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for ServiceOptionsSimpleField999 {}

    impl super::_puroro_traits::ServiceOptionsTrait for ServiceOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MethodOptionsSimple {
        pub deprecated: ::std::option::Option<bool>,
        pub idempotency_level: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        >,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }
    impl ::puroro::Message for MethodOptionsSimple {}

    impl MethodOptionsTrait for MethodOptionsSimple {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn idempotency_level<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            Clone::clone(&self.idempotency_level)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for MethodOptionsSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for MethodOptionsSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            33 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            34 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>
            >::deser_field(&mut self.idempotency_level, data),
            999 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for MethodOptionsSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >::ser_field(&self.deprecated, 33, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>
        >::ser_field(&self.idempotency_level, 34, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl MethodOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MethodOptionsTrait for (T, U)
    where
        T: MethodOptionsTrait,
        U: MethodOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            <U as MethodOptionsTrait>::deprecated(&self.1)
                .or_else(|| <T as MethodOptionsTrait>::deprecated(&self.0))
        }
        fn idempotency_level<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            <U as MethodOptionsTrait>::idempotency_level(&self.1)
                .or_else(|| <T as MethodOptionsTrait>::idempotency_level(&self.0))
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as MethodOptionsTrait>::Field999MessageType<'this>,
            <U as MethodOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as MethodOptionsTrait>::Field999RepeatedType<'this>,
                <U as MethodOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as MethodOptionsTrait>::uninterpreted_option(&self.0),
                <U as MethodOptionsTrait>::uninterpreted_option(&self.1),
            )
        }
    }
    impl<T, U> MethodOptionsTrait for ::puroro::Either<T, U>
    where
        T: MethodOptionsTrait,
        U: MethodOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            self.as_ref().either(
                |t| <T as MethodOptionsTrait>::deprecated(t),
                |u| <U as MethodOptionsTrait>::deprecated(u),
            )
        }
        fn idempotency_level<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            self.as_ref().either(
                |t| <T as MethodOptionsTrait>::idempotency_level(t),
                |u| <U as MethodOptionsTrait>::idempotency_level(u),
            )
        }
        type Field999MessageType<'this> = ::puroro::Either<
            <T as MethodOptionsTrait>::Field999MessageType<'this>,
            <U as MethodOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as MethodOptionsTrait>::Field999RepeatedType<'this>,
                <U as MethodOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MethodOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as MethodOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodOptionsSimpleField33 {
        deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message for MethodOptionsSimpleField33 {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptionsSimpleField33 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodOptionsSimpleField34 {
        idempotency_level: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        >,
    }

    impl ::puroro::Message for MethodOptionsSimpleField34 {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptionsSimpleField34 {
        fn idempotency_level<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            Clone::clone(&self.idempotency_level)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct MethodOptionsSimpleField999 {
        uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
        >,
    }

    impl ::puroro::Message for MethodOptionsSimpleField999 {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple;
        type Field999RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::UninterpretedOptionSimple,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct UninterpretedOptionSimple {
    pub name: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple>,
    pub identifier_value: ::std::option::Option<::std::string::String>,
    pub positive_int_value: ::std::option::Option<u64>,
    pub negative_int_value: ::std::option::Option<i64>,
    pub double_value: ::std::option::Option<f64>,
    pub string_value: ::std::option::Option<::std::vec::Vec<u8>>,
    pub aggregate_value: ::std::option::Option<::std::string::String>,
}
    impl ::puroro::Message for UninterpretedOptionSimple {}

    impl UninterpretedOptionTrait for UninterpretedOptionSimple {
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple>>;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.name.iter())
        }
        type Field3StringType<'this> = &'this str;
        fn identifier_value<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.identifier_value.as_ref().map(|v| v.as_ref())
        }
        fn positive_int_value<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.positive_int_value)
        }
        fn negative_int_value<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.negative_int_value)
        }
        fn double_value<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.double_value)
        }
        type Field7BytesType<'this> = &'this [u8];
        fn string_value<'this>(&'this self) -> Option<Self::Field7BytesType<'this>> {
            self.string_value.as_ref().map(|v| v.as_ref())
        }
        type Field8StringType<'this> = &'this str;
        fn aggregate_value<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            self.aggregate_value.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for UninterpretedOptionSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for UninterpretedOptionSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple>
            >::deser_field(&mut self.name, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.identifier_value, data),
            4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.positive_int_value, data),
            5 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.negative_int_value, data),
            6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Double
            >::deser_field(&mut self.double_value, data),
            7 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.string_value, data),
            8 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.aggregate_value, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for UninterpretedOptionSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple>
        >::ser_field(&self.name, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.identifier_value, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(&self.positive_int_value, 4, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >::ser_field(&self.negative_int_value, 5, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >::ser_field(&self.double_value, 6, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >::ser_field(&self.string_value, 7, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.aggregate_value, 8, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl UninterpretedOptionTrait for () {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }
    impl<T, U> UninterpretedOptionTrait for (T, U)
    where
        T: UninterpretedOptionTrait,
        U: UninterpretedOptionTrait,
    {
        type Field2MessageType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field2MessageType<'this>,
            <U as UninterpretedOptionTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
                <U as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
            >;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as UninterpretedOptionTrait>::name(&self.0),
                <U as UninterpretedOptionTrait>::name(&self.1),
            )
        }
        type Field3StringType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field3StringType<'this>,
            <U as UninterpretedOptionTrait>::Field3StringType<'this>,
        >;
        fn identifier_value<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            if let Some(right) = <U as UninterpretedOptionTrait>::identifier_value(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as UninterpretedOptionTrait>::identifier_value(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        fn positive_int_value<'this>(&'this self) -> Option<u64> {
            <U as UninterpretedOptionTrait>::positive_int_value(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::positive_int_value(&self.0))
        }
        fn negative_int_value<'this>(&'this self) -> Option<i64> {
            <U as UninterpretedOptionTrait>::negative_int_value(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::negative_int_value(&self.0))
        }
        fn double_value<'this>(&'this self) -> Option<f64> {
            <U as UninterpretedOptionTrait>::double_value(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::double_value(&self.0))
        }
        type Field7BytesType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field7BytesType<'this>,
            <U as UninterpretedOptionTrait>::Field7BytesType<'this>,
        >;
        fn string_value<'this>(&'this self) -> Option<Self::Field7BytesType<'this>> {
            if let Some(right) = <U as UninterpretedOptionTrait>::string_value(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as UninterpretedOptionTrait>::string_value(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field8StringType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field8StringType<'this>,
            <U as UninterpretedOptionTrait>::Field8StringType<'this>,
        >;
        fn aggregate_value<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            if let Some(right) = <U as UninterpretedOptionTrait>::aggregate_value(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as UninterpretedOptionTrait>::aggregate_value(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
    }
    impl<T, U> UninterpretedOptionTrait for ::puroro::Either<T, U>
    where
        T: UninterpretedOptionTrait,
        U: UninterpretedOptionTrait,
    {
        type Field2MessageType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field2MessageType<'this>,
            <U as UninterpretedOptionTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
                <U as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
            >;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as UninterpretedOptionTrait>::name(t))
                    .map_right(|u| <U as UninterpretedOptionTrait>::name(u)),
            )
        }
        type Field3StringType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field3StringType<'this>,
            <U as UninterpretedOptionTrait>::Field3StringType<'this>,
        >;
        fn identifier_value<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as UninterpretedOptionTrait>::identifier_value(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as UninterpretedOptionTrait>::identifier_value(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn positive_int_value<'this>(&'this self) -> Option<u64> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::positive_int_value(t),
                |u| <U as UninterpretedOptionTrait>::positive_int_value(u),
            )
        }
        fn negative_int_value<'this>(&'this self) -> Option<i64> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::negative_int_value(t),
                |u| <U as UninterpretedOptionTrait>::negative_int_value(u),
            )
        }
        fn double_value<'this>(&'this self) -> Option<f64> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::double_value(t),
                |u| <U as UninterpretedOptionTrait>::double_value(u),
            )
        }
        type Field7BytesType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field7BytesType<'this>,
            <U as UninterpretedOptionTrait>::Field7BytesType<'this>,
        >;
        fn string_value<'this>(&'this self) -> Option<Self::Field7BytesType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as UninterpretedOptionTrait>::string_value(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as UninterpretedOptionTrait>::string_value(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field8StringType<'this> = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field8StringType<'this>,
            <U as UninterpretedOptionTrait>::Field8StringType<'this>,
        >;
        fn aggregate_value<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as UninterpretedOptionTrait>::aggregate_value(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as UninterpretedOptionTrait>::aggregate_value(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct UninterpretedOptionSimpleField2 {
    name: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple>,
}

    impl ::puroro::Message for UninterpretedOptionSimpleField2 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField2 {
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple;
        type Field2RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_impls::NamePartSimple>>;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.name.iter())
        }
        type Field3StringType<'this> = &'static str;
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct UninterpretedOptionSimpleField3 {
        identifier_value: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for UninterpretedOptionSimpleField3 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField3 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'this str;
        fn identifier_value<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.identifier_value.as_ref().map(|v| v.as_ref())
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct UninterpretedOptionSimpleField4 {
        positive_int_value: ::std::option::Option<u64>,
    }

    impl ::puroro::Message for UninterpretedOptionSimpleField4 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField4 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        fn positive_int_value<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.positive_int_value)
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct UninterpretedOptionSimpleField5 {
        negative_int_value: ::std::option::Option<i64>,
    }

    impl ::puroro::Message for UninterpretedOptionSimpleField5 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField5 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        fn negative_int_value<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.negative_int_value)
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct UninterpretedOptionSimpleField6 {
        double_value: ::std::option::Option<f64>,
    }

    impl ::puroro::Message for UninterpretedOptionSimpleField6 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField6 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        fn double_value<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.double_value)
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct UninterpretedOptionSimpleField7 {
        string_value: ::std::option::Option<::std::vec::Vec<u8>>,
    }

    impl ::puroro::Message for UninterpretedOptionSimpleField7 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField7 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        type Field7BytesType<'this> = &'this [u8];
        fn string_value<'this>(&'this self) -> Option<Self::Field7BytesType<'this>> {
            self.string_value.as_ref().map(|v| v.as_ref())
        }
        type Field8StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct UninterpretedOptionSimpleField8 {
        aggregate_value: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for UninterpretedOptionSimpleField8 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField8 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'this str;
        fn aggregate_value<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            self.aggregate_value.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SourceCodeInfoSimple {
    pub location: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple>,
}
    impl ::puroro::Message for SourceCodeInfoSimple {}

    impl SourceCodeInfoTrait for SourceCodeInfoSimple {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple>>;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.location.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for SourceCodeInfoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for SourceCodeInfoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple>
            >::deser_field(&mut self.location, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for SourceCodeInfoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple>
        >::ser_field(&self.location, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl SourceCodeInfoTrait for () {
        type Field1MessageType<'this> = ();
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> SourceCodeInfoTrait for (T, U)
    where
        T: SourceCodeInfoTrait,
        U: SourceCodeInfoTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T as SourceCodeInfoTrait>::Field1MessageType<'this>,
            <U as SourceCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as SourceCodeInfoTrait>::location(&self.0),
                <U as SourceCodeInfoTrait>::location(&self.1),
            )
        }
    }
    impl<T, U> SourceCodeInfoTrait for ::puroro::Either<T, U>
    where
        T: SourceCodeInfoTrait,
        U: SourceCodeInfoTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T as SourceCodeInfoTrait>::Field1MessageType<'this>,
            <U as SourceCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as SourceCodeInfoTrait>::location(t))
                    .map_right(|u| <U as SourceCodeInfoTrait>::location(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct SourceCodeInfoSimpleField1 {
    location: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple>,
}

    impl ::puroro::Message for SourceCodeInfoSimpleField1 {}

    impl super::_puroro_traits::SourceCodeInfoTrait for SourceCodeInfoSimpleField1 {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_impls::LocationSimple>>;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.location.iter())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct GeneratedCodeInfoSimple {
    pub annotation: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple>,
}
    impl ::puroro::Message for GeneratedCodeInfoSimple {}

    impl GeneratedCodeInfoTrait for GeneratedCodeInfoSimple {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple>>;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.annotation.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for GeneratedCodeInfoSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for GeneratedCodeInfoSimple {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro_internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            match field_number {
            1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple>
            >::deser_field(&mut self.annotation, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for GeneratedCodeInfoSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple>
        >::ser_field(&self.annotation, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl GeneratedCodeInfoTrait for () {
        type Field1MessageType<'this> = ();
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> GeneratedCodeInfoTrait for (T, U)
    where
        T: GeneratedCodeInfoTrait,
        U: GeneratedCodeInfoTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
            <U as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as GeneratedCodeInfoTrait>::annotation(&self.0),
                <U as GeneratedCodeInfoTrait>::annotation(&self.1),
            )
        }
    }
    impl<T, U> GeneratedCodeInfoTrait for ::puroro::Either<T, U>
    where
        T: GeneratedCodeInfoTrait,
        U: GeneratedCodeInfoTrait,
    {
        type Field1MessageType<'this> = ::puroro::Either<
            <T as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
            <U as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as GeneratedCodeInfoTrait>::annotation(t))
                    .map_right(|u| <U as GeneratedCodeInfoTrait>::annotation(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct GeneratedCodeInfoSimpleField1 {
    annotation: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple>,
}

    impl ::puroro::Message for GeneratedCodeInfoSimpleField1 {}

    impl super::_puroro_traits::GeneratedCodeInfoTrait for GeneratedCodeInfoSimpleField1 {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_impls::AnnotationSimple>>;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.annotation.iter())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait FileDescriptorSetTrait {
        type Field1MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait;
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field1MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    macro_rules! file_descriptor_set_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this> = <$ty>::Field1MessageType<'this>;
            type Field1RepeatedType<'this> = <$ty>::Field1RepeatedType<'this>;
            fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                (**self).file()
            }
        };
    }

    impl<T> FileDescriptorSetTrait for &'_ T
    where
        T: FileDescriptorSetTrait,
    {
        file_descriptor_set_delegate!(T);
    }

    impl<T> FileDescriptorSetTrait for ::std::boxed::Box<T>
    where
        T: FileDescriptorSetTrait,
    {
        file_descriptor_set_delegate!(T);
    }
    pub trait FileDescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>;
        fn package<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field3StringType<'this>: ::std::ops::Deref<Target = str>;
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        type Field11RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this>;
        type Field4MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field6MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait;
        type Field7RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this>;
        type Field8MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field9MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait;
        fn source_code_info<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field12StringType<'this>: ::std::ops::Deref<Target = str>;
        fn syntax<'this>(&'this self) -> ::std::option::Option<Self::Field12StringType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! file_descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            fn package<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
                (**self).package()
            }
            type Field3StringType<'this> = <$ty>::Field3StringType<'this>;
            type Field3RepeatedType<'this> = <$ty>::Field3RepeatedType<'this>;
            fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).dependency()
            }
            type Field10RepeatedType<'this> = <$ty>::Field10RepeatedType<'this>;
            fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
                (**self).public_dependency()
            }
            type Field11RepeatedType<'this> = <$ty>::Field11RepeatedType<'this>;
            fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
                (**self).weak_dependency()
            }
            type Field4MessageType<'this> = <$ty>::Field4MessageType<'this>;
            type Field4RepeatedType<'this> = <$ty>::Field4RepeatedType<'this>;
            fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).message_type()
            }
            type Field5MessageType<'this> = <$ty>::Field5MessageType<'this>;
            type Field5RepeatedType<'this> = <$ty>::Field5RepeatedType<'this>;
            fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
                (**self).enum_type()
            }
            type Field6MessageType<'this> = <$ty>::Field6MessageType<'this>;
            type Field6RepeatedType<'this> = <$ty>::Field6RepeatedType<'this>;
            fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
                (**self).service()
            }
            type Field7MessageType<'this> = <$ty>::Field7MessageType<'this>;
            type Field7RepeatedType<'this> = <$ty>::Field7RepeatedType<'this>;
            fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
                (**self).extension()
            }
            type Field8MessageType<'this> = <$ty>::Field8MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
                (**self).options()
            }
            type Field9MessageType<'this> = <$ty>::Field9MessageType<'this>;
            fn source_code_info<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
                (**self).source_code_info()
            }
            type Field12StringType<'this> = <$ty>::Field12StringType<'this>;
            fn syntax<'this>(&'this self) -> ::std::option::Option<Self::Field12StringType<'this>> {
                (**self).syntax()
            }
        };
    }

    impl<T> FileDescriptorProtoTrait for &'_ T
    where
        T: FileDescriptorProtoTrait,
    {
        file_descriptor_proto_delegate!(T);
    }

    impl<T> FileDescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: FileDescriptorProtoTrait,
    {
        file_descriptor_proto_delegate!(T);
    }
    pub trait DescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field6MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field3MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait;
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        type Field4MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field8MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        type Field7MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field9MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait;
        type Field9RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this>;
        type Field10StringType<'this>: ::std::ops::Deref<Target = str>;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
    }

    macro_rules! descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            type Field2MessageType<'this> = <$ty>::Field2MessageType<'this>;
            type Field2RepeatedType<'this> = <$ty>::Field2RepeatedType<'this>;
            fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).field()
            }
            type Field6MessageType<'this> = <$ty>::Field6MessageType<'this>;
            type Field6RepeatedType<'this> = <$ty>::Field6RepeatedType<'this>;
            fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
                (**self).extension()
            }
            type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
            type Field3RepeatedType<'this> = <$ty>::Field3RepeatedType<'this>;
            fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).nested_type()
            }
            type Field4MessageType<'this> = <$ty>::Field4MessageType<'this>;
            type Field4RepeatedType<'this> = <$ty>::Field4RepeatedType<'this>;
            fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).enum_type()
            }
            type Field5MessageType<'this> = <$ty>::Field5MessageType<'this>;
            type Field5RepeatedType<'this> = <$ty>::Field5RepeatedType<'this>;
            fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
                (**self).extension_range()
            }
            type Field8MessageType<'this> = <$ty>::Field8MessageType<'this>;
            type Field8RepeatedType<'this> = <$ty>::Field8RepeatedType<'this>;
            fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
                (**self).oneof_decl()
            }
            type Field7MessageType<'this> = <$ty>::Field7MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
                (**self).options()
            }
            type Field9MessageType<'this> = <$ty>::Field9MessageType<'this>;
            type Field9RepeatedType<'this> = <$ty>::Field9RepeatedType<'this>;
            fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
                (**self).reserved_range()
            }
            type Field10StringType<'this> = <$ty>::Field10StringType<'this>;
            type Field10RepeatedType<'this> = <$ty>::Field10RepeatedType<'this>;
            fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
                (**self).reserved_name()
            }
        };
    }

    impl<T> DescriptorProtoTrait for &'_ T
    where
        T: DescriptorProtoTrait,
    {
        descriptor_proto_delegate!(T);
    }

    impl<T> DescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: DescriptorProtoTrait,
    {
        descriptor_proto_delegate!(T);
    }
    pub trait ExtensionRangeOptionsTrait {
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! extension_range_options_delegate {
        ($ty:ty) => {
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> ExtensionRangeOptionsTrait for &'_ T
    where
        T: ExtensionRangeOptionsTrait,
    {
        extension_range_options_delegate!(T);
    }

    impl<T> ExtensionRangeOptionsTrait for ::std::boxed::Box<T>
    where
        T: ExtensionRangeOptionsTrait,
    {
        extension_range_options_delegate!(T);
    }
    pub trait FieldDescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn label<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            ::std::default::Default::default()
        }
        fn type_<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            ::std::default::Default::default()
        }
        type Field6StringType<'this>: ::std::ops::Deref<Target = str>;
        fn type_name<'this>(&'this self) -> ::std::option::Option<Self::Field6StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>;
        fn extendee<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field7StringType<'this>: ::std::ops::Deref<Target = str>;
        fn default_value<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7StringType<'this>> {
            ::std::default::Default::default()
        }
        fn oneof_index<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field10StringType<'this>: ::std::ops::Deref<Target = str>;
        fn json_name<'this>(&'this self) -> ::std::option::Option<Self::Field10StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field8MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            ::std::default::Default::default()
        }
        fn proto3_optional<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
    }

    macro_rules! field_descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            fn number<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).number()
            }
            fn label<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
            > {
                (**self).label()
            }
            fn type_<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
            > {
                (**self).type_()
            }
            type Field6StringType<'this> = <$ty>::Field6StringType<'this>;
            fn type_name<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field6StringType<'this>> {
                (**self).type_name()
            }
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            fn extendee<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                (**self).extendee()
            }
            type Field7StringType<'this> = <$ty>::Field7StringType<'this>;
            fn default_value<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field7StringType<'this>> {
                (**self).default_value()
            }
            fn oneof_index<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).oneof_index()
            }
            type Field10StringType<'this> = <$ty>::Field10StringType<'this>;
            fn json_name<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field10StringType<'this>> {
                (**self).json_name()
            }
            type Field8MessageType<'this> = <$ty>::Field8MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
                (**self).options()
            }
            fn proto3_optional<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).proto3_optional()
            }
        };
    }

    impl<T> FieldDescriptorProtoTrait for &'_ T
    where
        T: FieldDescriptorProtoTrait,
    {
        field_descriptor_proto_delegate!(T);
    }

    impl<T> FieldDescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: FieldDescriptorProtoTrait,
    {
        field_descriptor_proto_delegate!(T);
    }
    pub trait OneofDescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field2MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! oneof_descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            type Field2MessageType<'this> = <$ty>::Field2MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field2MessageType<'this>> {
                (**self).options()
            }
        };
    }

    impl<T> OneofDescriptorProtoTrait for &'_ T
    where
        T: OneofDescriptorProtoTrait,
    {
        oneof_descriptor_proto_delegate!(T);
    }

    impl<T> OneofDescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: OneofDescriptorProtoTrait,
    {
        oneof_descriptor_proto_delegate!(T);
    }
    pub trait EnumDescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field4MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5StringType<'this>: ::std::ops::Deref<Target = str>;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
    }

    macro_rules! enum_descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            type Field2MessageType<'this> = <$ty>::Field2MessageType<'this>;
            type Field2RepeatedType<'this> = <$ty>::Field2RepeatedType<'this>;
            fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).value()
            }
            type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).options()
            }
            type Field4MessageType<'this> = <$ty>::Field4MessageType<'this>;
            type Field4RepeatedType<'this> = <$ty>::Field4RepeatedType<'this>;
            fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).reserved_range()
            }
            type Field5StringType<'this> = <$ty>::Field5StringType<'this>;
            type Field5RepeatedType<'this> = <$ty>::Field5RepeatedType<'this>;
            fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
                (**self).reserved_name()
            }
        };
    }

    impl<T> EnumDescriptorProtoTrait for &'_ T
    where
        T: EnumDescriptorProtoTrait,
    {
        enum_descriptor_proto_delegate!(T);
    }

    impl<T> EnumDescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: EnumDescriptorProtoTrait,
    {
        enum_descriptor_proto_delegate!(T);
    }
    pub trait EnumValueDescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field3MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! enum_value_descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            fn number<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).number()
            }
            type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).options()
            }
        };
    }

    impl<T> EnumValueDescriptorProtoTrait for &'_ T
    where
        T: EnumValueDescriptorProtoTrait,
    {
        enum_value_descriptor_proto_delegate!(T);
    }

    impl<T> EnumValueDescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: EnumValueDescriptorProtoTrait,
    {
        enum_value_descriptor_proto_delegate!(T);
    }
    pub trait ServiceDescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! service_descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            type Field2MessageType<'this> = <$ty>::Field2MessageType<'this>;
            type Field2RepeatedType<'this> = <$ty>::Field2RepeatedType<'this>;
            fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).method()
            }
            type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).options()
            }
        };
    }

    impl<T> ServiceDescriptorProtoTrait for &'_ T
    where
        T: ServiceDescriptorProtoTrait,
    {
        service_descriptor_proto_delegate!(T);
    }

    impl<T> ServiceDescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: ServiceDescriptorProtoTrait,
    {
        service_descriptor_proto_delegate!(T);
    }
    pub trait MethodDescriptorProtoTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>;
        fn input_type<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field3StringType<'this>: ::std::ops::Deref<Target = str>;
        fn output_type<'this>(&'this self) -> ::std::option::Option<Self::Field3StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field4MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field4MessageType<'this>> {
            ::std::default::Default::default()
        }
        fn client_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn server_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
    }

    macro_rules! method_descriptor_proto_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).name()
            }
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            fn input_type<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                (**self).input_type()
            }
            type Field3StringType<'this> = <$ty>::Field3StringType<'this>;
            fn output_type<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3StringType<'this>> {
                (**self).output_type()
            }
            type Field4MessageType<'this> = <$ty>::Field4MessageType<'this>;
            fn options<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field4MessageType<'this>> {
                (**self).options()
            }
            fn client_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).client_streaming()
            }
            fn server_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).server_streaming()
            }
        };
    }

    impl<T> MethodDescriptorProtoTrait for &'_ T
    where
        T: MethodDescriptorProtoTrait,
    {
        method_descriptor_proto_delegate!(T);
    }

    impl<T> MethodDescriptorProtoTrait for ::std::boxed::Box<T>
    where
        T: MethodDescriptorProtoTrait,
    {
        method_descriptor_proto_delegate!(T);
    }
    pub trait FileOptionsTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn java_package<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field8StringType<'this>: ::std::ops::Deref<Target = str>;
        fn java_outer_classname<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8StringType<'this>> {
            ::std::default::Default::default()
        }
        fn java_multiple_files<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn optimize_for<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        > {
            ::std::default::Default::default()
        }
        type Field11StringType<'this>: ::std::ops::Deref<Target = str>;
        fn go_package<'this>(&'this self) -> ::std::option::Option<Self::Field11StringType<'this>> {
            ::std::default::Default::default()
        }
        fn cc_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn java_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn py_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn php_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        type Field36StringType<'this>: ::std::ops::Deref<Target = str>;
        fn objc_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field36StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field37StringType<'this>: ::std::ops::Deref<Target = str>;
        fn csharp_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field37StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field39StringType<'this>: ::std::ops::Deref<Target = str>;
        fn swift_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field39StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field40StringType<'this>: ::std::ops::Deref<Target = str>;
        fn php_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field40StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field41StringType<'this>: ::std::ops::Deref<Target = str>;
        fn php_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field44StringType<'this>: ::std::ops::Deref<Target = str>;
        fn php_metadata_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field44StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field45StringType<'this>: ::std::ops::Deref<Target = str>;
        fn ruby_package<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field45StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! file_options_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn java_package<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).java_package()
            }
            type Field8StringType<'this> = <$ty>::Field8StringType<'this>;
            fn java_outer_classname<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field8StringType<'this>> {
                (**self).java_outer_classname()
            }
            fn java_multiple_files<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).java_multiple_files()
            }
            fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).java_generate_equals_and_hash()
            }
            fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).java_string_check_utf8()
            }
            fn optimize_for<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
            > {
                (**self).optimize_for()
            }
            type Field11StringType<'this> = <$ty>::Field11StringType<'this>;
            fn go_package<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field11StringType<'this>> {
                (**self).go_package()
            }
            fn cc_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).cc_generic_services()
            }
            fn java_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).java_generic_services()
            }
            fn py_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).py_generic_services()
            }
            fn php_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).php_generic_services()
            }
            fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated()
            }
            fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).cc_enable_arenas()
            }
            type Field36StringType<'this> = <$ty>::Field36StringType<'this>;
            fn objc_class_prefix<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field36StringType<'this>> {
                (**self).objc_class_prefix()
            }
            type Field37StringType<'this> = <$ty>::Field37StringType<'this>;
            fn csharp_namespace<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field37StringType<'this>> {
                (**self).csharp_namespace()
            }
            type Field39StringType<'this> = <$ty>::Field39StringType<'this>;
            fn swift_prefix<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field39StringType<'this>> {
                (**self).swift_prefix()
            }
            type Field40StringType<'this> = <$ty>::Field40StringType<'this>;
            fn php_class_prefix<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field40StringType<'this>> {
                (**self).php_class_prefix()
            }
            type Field41StringType<'this> = <$ty>::Field41StringType<'this>;
            fn php_namespace<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field41StringType<'this>> {
                (**self).php_namespace()
            }
            type Field44StringType<'this> = <$ty>::Field44StringType<'this>;
            fn php_metadata_namespace<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field44StringType<'this>> {
                (**self).php_metadata_namespace()
            }
            type Field45StringType<'this> = <$ty>::Field45StringType<'this>;
            fn ruby_package<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field45StringType<'this>> {
                (**self).ruby_package()
            }
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> FileOptionsTrait for &'_ T
    where
        T: FileOptionsTrait,
    {
        file_options_delegate!(T);
    }

    impl<T> FileOptionsTrait for ::std::boxed::Box<T>
    where
        T: FileOptionsTrait,
    {
        file_options_delegate!(T);
    }
    pub trait MessageOptionsTrait {
        fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn map_entry<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! message_options_delegate {
        ($ty:ty) => {
            fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).message_set_wire_format()
            }
            fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).no_standard_descriptor_accessor()
            }
            fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated()
            }
            fn map_entry<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).map_entry()
            }
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> MessageOptionsTrait for &'_ T
    where
        T: MessageOptionsTrait,
    {
        message_options_delegate!(T);
    }

    impl<T> MessageOptionsTrait for ::std::boxed::Box<T>
    where
        T: MessageOptionsTrait,
    {
        message_options_delegate!(T);
    }
    pub trait FieldOptionsTrait {
        fn ctype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        > {
            ::std::default::Default::default()
        }
        fn packed<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn jstype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        > {
            ::std::default::Default::default()
        }
        fn lazy<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn weak<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! field_options_delegate {
        ($ty:ty) => {
            fn ctype<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
            > {
                (**self).ctype()
            }
            fn packed<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).packed()
            }
            fn jstype<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
            > {
                (**self).jstype()
            }
            fn lazy<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).lazy()
            }
            fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated()
            }
            fn weak<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).weak()
            }
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> FieldOptionsTrait for &'_ T
    where
        T: FieldOptionsTrait,
    {
        field_options_delegate!(T);
    }

    impl<T> FieldOptionsTrait for ::std::boxed::Box<T>
    where
        T: FieldOptionsTrait,
    {
        field_options_delegate!(T);
    }
    pub trait OneofOptionsTrait {
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! oneof_options_delegate {
        ($ty:ty) => {
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> OneofOptionsTrait for &'_ T
    where
        T: OneofOptionsTrait,
    {
        oneof_options_delegate!(T);
    }

    impl<T> OneofOptionsTrait for ::std::boxed::Box<T>
    where
        T: OneofOptionsTrait,
    {
        oneof_options_delegate!(T);
    }
    pub trait EnumOptionsTrait {
        fn allow_alias<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! enum_options_delegate {
        ($ty:ty) => {
            fn allow_alias<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).allow_alias()
            }
            fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated()
            }
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> EnumOptionsTrait for &'_ T
    where
        T: EnumOptionsTrait,
    {
        enum_options_delegate!(T);
    }

    impl<T> EnumOptionsTrait for ::std::boxed::Box<T>
    where
        T: EnumOptionsTrait,
    {
        enum_options_delegate!(T);
    }
    pub trait EnumValueOptionsTrait {
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! enum_value_options_delegate {
        ($ty:ty) => {
            fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated()
            }
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> EnumValueOptionsTrait for &'_ T
    where
        T: EnumValueOptionsTrait,
    {
        enum_value_options_delegate!(T);
    }

    impl<T> EnumValueOptionsTrait for ::std::boxed::Box<T>
    where
        T: EnumValueOptionsTrait,
    {
        enum_value_options_delegate!(T);
    }
    pub trait ServiceOptionsTrait {
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! service_options_delegate {
        ($ty:ty) => {
            fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated()
            }
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> ServiceOptionsTrait for &'_ T
    where
        T: ServiceOptionsTrait,
    {
        service_options_delegate!(T);
    }

    impl<T> ServiceOptionsTrait for ::std::boxed::Box<T>
    where
        T: ServiceOptionsTrait,
    {
        service_options_delegate!(T);
    }
    pub trait MethodOptionsTrait {
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::default::Default::default()
        }
        fn idempotency_level<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait;
        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! method_options_delegate {
        ($ty:ty) => {
            fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated()
            }
            fn idempotency_level<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel> {
                (**self).idempotency_level()
            }
            type Field999MessageType<'this> = <$ty>::Field999MessageType<'this>;
            type Field999RepeatedType<'this> = <$ty>::Field999RepeatedType<'this>;
            fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
                (**self).uninterpreted_option()
            }
        };
    }

    impl<T> MethodOptionsTrait for &'_ T
    where
        T: MethodOptionsTrait,
    {
        method_options_delegate!(T);
    }

    impl<T> MethodOptionsTrait for ::std::boxed::Box<T>
    where
        T: MethodOptionsTrait,
    {
        method_options_delegate!(T);
    }
    pub trait UninterpretedOptionTrait {
        type Field2MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3StringType<'this>: ::std::ops::Deref<Target = str>;
        fn identifier_value<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3StringType<'this>> {
            ::std::default::Default::default()
        }
        fn positive_int_value<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::default::Default::default()
        }
        fn negative_int_value<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::default::Default::default()
        }
        fn double_value<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::default::Default::default()
        }
        type Field7BytesType<'this>: ::std::ops::Deref<Target = [u8]>;
        fn string_value<'this>(&'this self) -> ::std::option::Option<Self::Field7BytesType<'this>> {
            ::std::default::Default::default()
        }
        type Field8StringType<'this>: ::std::ops::Deref<Target = str>;
        fn aggregate_value<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8StringType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! uninterpreted_option_delegate {
        ($ty:ty) => {
            type Field2MessageType<'this> = <$ty>::Field2MessageType<'this>;
            type Field2RepeatedType<'this> = <$ty>::Field2RepeatedType<'this>;
            fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).name()
            }
            type Field3StringType<'this> = <$ty>::Field3StringType<'this>;
            fn identifier_value<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3StringType<'this>> {
                (**self).identifier_value()
            }
            fn positive_int_value<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).positive_int_value()
            }
            fn negative_int_value<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).negative_int_value()
            }
            fn double_value<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).double_value()
            }
            type Field7BytesType<'this> = <$ty>::Field7BytesType<'this>;
            fn string_value<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field7BytesType<'this>> {
                (**self).string_value()
            }
            type Field8StringType<'this> = <$ty>::Field8StringType<'this>;
            fn aggregate_value<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field8StringType<'this>> {
                (**self).aggregate_value()
            }
        };
    }

    impl<T> UninterpretedOptionTrait for &'_ T
    where
        T: UninterpretedOptionTrait,
    {
        uninterpreted_option_delegate!(T);
    }

    impl<T> UninterpretedOptionTrait for ::std::boxed::Box<T>
    where
        T: UninterpretedOptionTrait,
    {
        uninterpreted_option_delegate!(T);
    }
    pub trait SourceCodeInfoTrait {
        type Field1MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait;
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field1MessageType<'this>>;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    macro_rules! source_code_info_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this> = <$ty>::Field1MessageType<'this>;
            type Field1RepeatedType<'this> = <$ty>::Field1RepeatedType<'this>;
            fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                (**self).location()
            }
        };
    }

    impl<T> SourceCodeInfoTrait for &'_ T
    where
        T: SourceCodeInfoTrait,
    {
        source_code_info_delegate!(T);
    }

    impl<T> SourceCodeInfoTrait for ::std::boxed::Box<T>
    where
        T: SourceCodeInfoTrait,
    {
        source_code_info_delegate!(T);
    }
    pub trait GeneratedCodeInfoTrait {
        type Field1MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait;
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field1MessageType<'this>>;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    macro_rules! generated_code_info_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this> = <$ty>::Field1MessageType<'this>;
            type Field1RepeatedType<'this> = <$ty>::Field1RepeatedType<'this>;
            fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                (**self).annotation()
            }
        };
    }

    impl<T> GeneratedCodeInfoTrait for &'_ T
    where
        T: GeneratedCodeInfoTrait,
    {
        generated_code_info_delegate!(T);
    }

    impl<T> GeneratedCodeInfoTrait for ::std::boxed::Box<T>
    where
        T: GeneratedCodeInfoTrait,
    {
        generated_code_info_delegate!(T);
    }
}
pub use _puroro_nested::*;
pub mod _puroro_nested {
    pub mod file_descriptor_set {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod file_descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_impls::ExtensionRangeSimple as ExtensionRange;
        pub use _puroro_impls::ReservedRangeSimple as ReservedRange;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct ExtensionRangeSimple {
            pub start: ::std::option::Option<i32>,
            pub end: ::std::option::Option<i32>,
            pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_impls::ExtensionRangeOptionsSimple>>,
        }
            impl ::puroro::Message for ExtensionRangeSimple {}

            impl ExtensionRangeTrait for ExtensionRangeSimple {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
                type Field3MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::ExtensionRangeOptionsSimple;
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    self.options.as_ref().map(|v| v.as_ref())
                }
            }

            impl ::puroro::DeserFromBytesIter for ExtensionRangeSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for ExtensionRangeSimple {
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
                    1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Int32
                    >::deser_field(&mut self.start, data),
                    2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Int32
                    >::deser_field(&mut self.end, data),
                    3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::ExtensionRangeOptionsSimple>
                    >::deser_field(&mut self.options, data),
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
                }
            }

            impl ::puroro::SerToIoWrite for ExtensionRangeSimple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.start, 1, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 2, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::ExtensionRangeOptionsSimple>
                >::ser_field(&self.options, 3, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl ExtensionRangeTrait for () {
                type Field3MessageType<'this> = ();
            }
            impl<T, U> ExtensionRangeTrait for (T, U)
            where
                T: ExtensionRangeTrait,
                U: ExtensionRangeTrait,
            {
                fn start<'this>(&'this self) -> Option<i32> {
                    <U as ExtensionRangeTrait>::start(&self.1)
                        .or_else(|| <T as ExtensionRangeTrait>::start(&self.0))
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    <U as ExtensionRangeTrait>::end(&self.1)
                        .or_else(|| <T as ExtensionRangeTrait>::end(&self.0))
                }
                type Field3MessageType<'this> = ::puroro::Either<
                    ::puroro::Either<
                        <T as ExtensionRangeTrait>::Field3MessageType<'this>,
                        <U as ExtensionRangeTrait>::Field3MessageType<'this>,
                    >,
                    (
                        <T as ExtensionRangeTrait>::Field3MessageType<'this>,
                        <U as ExtensionRangeTrait>::Field3MessageType<'this>,
                    ),
                >;
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    match (
                        <T as ExtensionRangeTrait>::options(&self.0),
                        <U as ExtensionRangeTrait>::options(&self.1),
                    ) {
                        (None, None) => None,
                        (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                        (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                        (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
                    }
                }
            }
            impl<T, U> ExtensionRangeTrait for ::puroro::Either<T, U>
            where
                T: ExtensionRangeTrait,
                U: ExtensionRangeTrait,
            {
                fn start<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as ExtensionRangeTrait>::start(t),
                        |u| <U as ExtensionRangeTrait>::start(u),
                    )
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as ExtensionRangeTrait>::end(t),
                        |u| <U as ExtensionRangeTrait>::end(u),
                    )
                }
                type Field3MessageType<'this> = ::puroro::Either<
                    <T as ExtensionRangeTrait>::Field3MessageType<'this>,
                    <U as ExtensionRangeTrait>::Field3MessageType<'this>,
                >;
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    self.as_ref().either(
                        |t| {
                            <T as ExtensionRangeTrait>::options(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as ExtensionRangeTrait>::options(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct ExtensionRangeSimpleField1 {
                start: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for ExtensionRangeSimpleField1 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSimpleField1 {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                type Field3MessageType<'this> = ();
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct ExtensionRangeSimpleField2 {
                end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for ExtensionRangeSimpleField2 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSimpleField2 {
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
                type Field3MessageType<'this> = ();
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct ExtensionRangeSimpleField3 {
            options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_impls::ExtensionRangeOptionsSimple>>,
        }

            impl ::puroro::Message for ExtensionRangeSimpleField3 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSimpleField3 {
                type Field3MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::ExtensionRangeOptionsSimple;
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    self.options.as_ref().map(|v| v.as_ref())
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct ReservedRangeSimple {
                pub start: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }
            impl ::puroro::Message for ReservedRangeSimple {}

            impl ReservedRangeTrait for ReservedRangeSimple {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::DeserFromBytesIter for ReservedRangeSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for ReservedRangeSimple {
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
                        1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.start, data),
                        2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for ReservedRangeSimple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.start, 1, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl ReservedRangeTrait for () {}
            impl<T, U> ReservedRangeTrait for (T, U)
            where
                T: ReservedRangeTrait,
                U: ReservedRangeTrait,
            {
                fn start<'this>(&'this self) -> Option<i32> {
                    <U as ReservedRangeTrait>::start(&self.1)
                        .or_else(|| <T as ReservedRangeTrait>::start(&self.0))
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    <U as ReservedRangeTrait>::end(&self.1)
                        .or_else(|| <T as ReservedRangeTrait>::end(&self.0))
                }
            }
            impl<T, U> ReservedRangeTrait for ::puroro::Either<T, U>
            where
                T: ReservedRangeTrait,
                U: ReservedRangeTrait,
            {
                fn start<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as ReservedRangeTrait>::start(t),
                        |u| <U as ReservedRangeTrait>::start(u),
                    )
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as ReservedRangeTrait>::end(t),
                        |u| <U as ReservedRangeTrait>::end(u),
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct ReservedRangeSimpleField1 {
                start: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for ReservedRangeSimpleField1 {}

            impl super::_puroro_traits::ReservedRangeTrait for ReservedRangeSimpleField1 {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct ReservedRangeSimpleField2 {
                end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for ReservedRangeSimpleField2 {}

            impl super::_puroro_traits::ReservedRangeTrait for ReservedRangeSimpleField2 {
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait ExtensionRangeTrait {
                fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                type Field3MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait;
                fn options<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! extension_range_delegate {
                ($ty:ty) => {
                    fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).start()
                    }
                    fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end()
                    }
                    type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
                    fn options<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                        (**self).options()
                    }
                };
            }

            impl<T> ExtensionRangeTrait for &'_ T
            where
                T: ExtensionRangeTrait,
            {
                extension_range_delegate!(T);
            }

            impl<T> ExtensionRangeTrait for ::std::boxed::Box<T>
            where
                T: ExtensionRangeTrait,
            {
                extension_range_delegate!(T);
            }
            pub trait ReservedRangeTrait {
                fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! reserved_range_delegate {
                ($ty:ty) => {
                    fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).start()
                    }
                    fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end()
                    }
                };
            }

            impl<T> ReservedRangeTrait for &'_ T
            where
                T: ReservedRangeTrait,
            {
                reserved_range_delegate!(T);
            }

            impl<T> ReservedRangeTrait for ::std::boxed::Box<T>
            where
                T: ReservedRangeTrait,
            {
                reserved_range_delegate!(T);
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod extension_range {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
            pub mod reserved_range {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
    pub mod extension_range_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod field_descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
    }
    pub mod oneof_descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod enum_descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_impls::EnumReservedRangeSimple as EnumReservedRange;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct EnumReservedRangeSimple {
                pub start: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }
            impl ::puroro::Message for EnumReservedRangeSimple {}

            impl EnumReservedRangeTrait for EnumReservedRangeSimple {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::DeserFromBytesIter for EnumReservedRangeSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for EnumReservedRangeSimple {
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
                        1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.start, data),
                        2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for EnumReservedRangeSimple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.start, 1, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl EnumReservedRangeTrait for () {}
            impl<T, U> EnumReservedRangeTrait for (T, U)
            where
                T: EnumReservedRangeTrait,
                U: EnumReservedRangeTrait,
            {
                fn start<'this>(&'this self) -> Option<i32> {
                    <U as EnumReservedRangeTrait>::start(&self.1)
                        .or_else(|| <T as EnumReservedRangeTrait>::start(&self.0))
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    <U as EnumReservedRangeTrait>::end(&self.1)
                        .or_else(|| <T as EnumReservedRangeTrait>::end(&self.0))
                }
            }
            impl<T, U> EnumReservedRangeTrait for ::puroro::Either<T, U>
            where
                T: EnumReservedRangeTrait,
                U: EnumReservedRangeTrait,
            {
                fn start<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as EnumReservedRangeTrait>::start(t),
                        |u| <U as EnumReservedRangeTrait>::start(u),
                    )
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as EnumReservedRangeTrait>::end(t),
                        |u| <U as EnumReservedRangeTrait>::end(u),
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct EnumReservedRangeSimpleField1 {
                start: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for EnumReservedRangeSimpleField1 {}

            impl super::_puroro_traits::EnumReservedRangeTrait for EnumReservedRangeSimpleField1 {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct EnumReservedRangeSimpleField2 {
                end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for EnumReservedRangeSimpleField2 {}

            impl super::_puroro_traits::EnumReservedRangeTrait for EnumReservedRangeSimpleField2 {
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait EnumReservedRangeTrait {
                fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! enum_reserved_range_delegate {
                ($ty:ty) => {
                    fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).start()
                    }
                    fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end()
                    }
                };
            }

            impl<T> EnumReservedRangeTrait for &'_ T
            where
                T: EnumReservedRangeTrait,
            {
                enum_reserved_range_delegate!(T);
            }

            impl<T> EnumReservedRangeTrait for ::std::boxed::Box<T>
            where
                T: EnumReservedRangeTrait,
            {
                enum_reserved_range_delegate!(T);
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod enum_reserved_range {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
    pub mod enum_value_descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod service_descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod method_descriptor_proto {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod file_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
    }
    pub mod message_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod field_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
    }
    pub mod oneof_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod enum_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod enum_value_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod service_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }
    }
    pub mod method_options {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::cmp::PartialEq)]
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
    }
    pub mod uninterpreted_option {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_impls::NamePartSimple as NamePart;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct NamePartSimple {
                pub name_part: ::std::option::Option<::std::string::String>,
                pub is_extension: ::std::option::Option<bool>,
            }
            impl ::puroro::Message for NamePartSimple {}

            impl NamePartTrait for NamePartSimple {
                type Field1StringType<'this> = &'this str;
                fn name_part<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.name_part.as_ref().map(|v| v.as_ref())
                }
                fn is_extension<'this>(&'this self) -> Option<bool> {
                    Clone::clone(&self.is_extension)
                }
            }

            impl ::puroro::DeserFromBytesIter for NamePartSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for NamePartSimple {
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
                        1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::String,
                        >::deser_field(&mut self.name_part, data),
                        2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::Bool,
                        >::deser_field(&mut self.is_extension, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for NamePartSimple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Required,
                        ::puroro::tags::String,
                    >::ser_field(&self.name_part, 1, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Required,
                        ::puroro::tags::Bool,
                    >::ser_field(&self.is_extension, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl NamePartTrait for () {
                type Field1StringType<'this> = &'static str;
            }
            impl<T, U> NamePartTrait for (T, U)
            where
                T: NamePartTrait,
                U: NamePartTrait,
            {
                type Field1StringType<'this> = ::puroro::Either<
                    <T as NamePartTrait>::Field1StringType<'this>,
                    <U as NamePartTrait>::Field1StringType<'this>,
                >;
                fn name_part<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    if let Some(right) = <U as NamePartTrait>::name_part(&self.1) {
                        Some(::puroro::Either::Right(right))
                    } else if let Some(left) = <T as NamePartTrait>::name_part(&self.0) {
                        Some(::puroro::Either::Left(left))
                    } else {
                        None
                    }
                }
                fn is_extension<'this>(&'this self) -> Option<bool> {
                    <U as NamePartTrait>::is_extension(&self.1)
                        .or_else(|| <T as NamePartTrait>::is_extension(&self.0))
                }
            }
            impl<T, U> NamePartTrait for ::puroro::Either<T, U>
            where
                T: NamePartTrait,
                U: NamePartTrait,
            {
                type Field1StringType<'this> = ::puroro::Either<
                    <T as NamePartTrait>::Field1StringType<'this>,
                    <U as NamePartTrait>::Field1StringType<'this>,
                >;
                fn name_part<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.as_ref().either(
                        |t| <T as NamePartTrait>::name_part(t).map(|t| ::puroro::Either::Left(t)),
                        |u| <U as NamePartTrait>::name_part(u).map(|u| ::puroro::Either::Right(u)),
                    )
                }
                fn is_extension<'this>(&'this self) -> Option<bool> {
                    self.as_ref().either(
                        |t| <T as NamePartTrait>::is_extension(t),
                        |u| <U as NamePartTrait>::is_extension(u),
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct NamePartSimpleField1 {
                name_part: ::std::option::Option<::std::string::String>,
            }

            impl ::puroro::Message for NamePartSimpleField1 {}

            impl super::_puroro_traits::NamePartTrait for NamePartSimpleField1 {
                type Field1StringType<'this> = &'this str;
                fn name_part<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.name_part.as_ref().map(|v| v.as_ref())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct NamePartSimpleField2 {
                is_extension: ::std::option::Option<bool>,
            }

            impl ::puroro::Message for NamePartSimpleField2 {}

            impl super::_puroro_traits::NamePartTrait for NamePartSimpleField2 {
                type Field1StringType<'this> = &'static str;
                fn is_extension<'this>(&'this self) -> Option<bool> {
                    Clone::clone(&self.is_extension)
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait NamePartTrait {
                type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
                fn name_part<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field1StringType<'this>> {
                    ::std::default::Default::default()
                }
                fn is_extension<'this>(&'this self) -> ::std::option::Option<bool> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! name_part_delegate {
                ($ty:ty) => {
                    type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
                    fn name_part<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field1StringType<'this>> {
                        (**self).name_part()
                    }
                    fn is_extension<'this>(&'this self) -> ::std::option::Option<bool> {
                        (**self).is_extension()
                    }
                };
            }

            impl<T> NamePartTrait for &'_ T
            where
                T: NamePartTrait,
            {
                name_part_delegate!(T);
            }

            impl<T> NamePartTrait for ::std::boxed::Box<T>
            where
                T: NamePartTrait,
            {
                name_part_delegate!(T);
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod name_part {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
    pub mod source_code_info {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_impls::LocationSimple as Location;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct LocationSimple {
                pub path: ::std::vec::Vec<i32>,
                pub span: ::std::vec::Vec<i32>,
                pub leading_comments: ::std::option::Option<::std::string::String>,
                pub trailing_comments: ::std::option::Option<::std::string::String>,
                pub leading_detached_comments: ::std::vec::Vec<::std::string::String>,
            }
            impl ::puroro::Message for LocationSimple {}

            impl LocationTrait for LocationSimple {
                type Field1RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.path.iter().cloned()
                }
                type Field2RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    self.span.iter().cloned()
                }
                type Field3StringType<'this> = &'this str;
                fn leading_comments<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
                    self.leading_comments.as_ref().map(|v| v.as_ref())
                }
                type Field4StringType<'this> = &'this str;
                fn trailing_comments<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
                    self.trailing_comments.as_ref().map(|v| v.as_ref())
                }
                type Field6StringType<'this> = &'this str;
                type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
                    str,
                    ::std::slice::Iter<'this, ::std::string::String>,
                >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::simple::BorrowedIter::new(
                        self.leading_detached_comments.iter(),
                    )
                }
            }

            impl ::puroro::DeserFromBytesIter for LocationSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for LocationSimple {
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
                        1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.path, data),
                        2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.span, data),
                        3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.leading_comments, data
                        ),
                        4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.trailing_comments, data
                        ),
                        6 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.leading_detached_comments, data
                        ),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for LocationSimple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.path, 1, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.span, 2, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(&self.leading_comments, 3, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(&self.trailing_comments, 4, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >::ser_field(&self.leading_detached_comments, 6, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl LocationTrait for () {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
            }
            impl<T, U> LocationTrait for (T, U)
            where
                T: LocationTrait,
                U: LocationTrait,
            {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::merged::MergedRepeatedField<
                        <T as LocationTrait>::Field1RepeatedType<'this>,
                        <U as LocationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::merged::MergedRepeatedField::new(
                        <T as LocationTrait>::path(&self.0),
                        <U as LocationTrait>::path(&self.1),
                    )
                }
                type Field2RepeatedType<'this> =
                    ::puroro_internal::impls::merged::MergedRepeatedField<
                        <T as LocationTrait>::Field2RepeatedType<'this>,
                        <U as LocationTrait>::Field2RepeatedType<'this>,
                    >;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro_internal::impls::merged::MergedRepeatedField::new(
                        <T as LocationTrait>::span(&self.0),
                        <U as LocationTrait>::span(&self.1),
                    )
                }
                type Field3StringType<'this> = ::puroro::Either<
                    <T as LocationTrait>::Field3StringType<'this>,
                    <U as LocationTrait>::Field3StringType<'this>,
                >;
                fn leading_comments<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
                    if let Some(right) = <U as LocationTrait>::leading_comments(&self.1) {
                        Some(::puroro::Either::Right(right))
                    } else if let Some(left) = <T as LocationTrait>::leading_comments(&self.0) {
                        Some(::puroro::Either::Left(left))
                    } else {
                        None
                    }
                }
                type Field4StringType<'this> = ::puroro::Either<
                    <T as LocationTrait>::Field4StringType<'this>,
                    <U as LocationTrait>::Field4StringType<'this>,
                >;
                fn trailing_comments<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
                    if let Some(right) = <U as LocationTrait>::trailing_comments(&self.1) {
                        Some(::puroro::Either::Right(right))
                    } else if let Some(left) = <T as LocationTrait>::trailing_comments(&self.0) {
                        Some(::puroro::Either::Left(left))
                    } else {
                        None
                    }
                }
                type Field6StringType<'this> = ::puroro::Either<
                    <T as LocationTrait>::Field6StringType<'this>,
                    <U as LocationTrait>::Field6StringType<'this>,
                >;
                type Field6RepeatedType<'this> =
                    ::puroro_internal::impls::merged::MergedRepeatedLDField<
                        <T as LocationTrait>::Field6RepeatedType<'this>,
                        <U as LocationTrait>::Field6RepeatedType<'this>,
                    >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::merged::MergedRepeatedLDField::new(
                        <T as LocationTrait>::leading_detached_comments(&self.0),
                        <U as LocationTrait>::leading_detached_comments(&self.1),
                    )
                }
            }
            impl<T, U> LocationTrait for ::puroro::Either<T, U>
            where
                T: LocationTrait,
                U: LocationTrait,
            {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::either::EitherRepeatedField<
                        <T as LocationTrait>::Field1RepeatedType<'this>,
                        <U as LocationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::either::EitherRepeatedField::new(
                        self.as_ref()
                            .map_left(|t| <T as LocationTrait>::path(t))
                            .map_right(|u| <U as LocationTrait>::path(u)),
                    )
                }
                type Field2RepeatedType<'this> =
                    ::puroro_internal::impls::either::EitherRepeatedField<
                        <T as LocationTrait>::Field2RepeatedType<'this>,
                        <U as LocationTrait>::Field2RepeatedType<'this>,
                    >;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro_internal::impls::either::EitherRepeatedField::new(
                        self.as_ref()
                            .map_left(|t| <T as LocationTrait>::span(t))
                            .map_right(|u| <U as LocationTrait>::span(u)),
                    )
                }
                type Field3StringType<'this> = ::puroro::Either<
                    <T as LocationTrait>::Field3StringType<'this>,
                    <U as LocationTrait>::Field3StringType<'this>,
                >;
                fn leading_comments<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
                    self.as_ref().either(
                        |t| {
                            <T as LocationTrait>::leading_comments(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as LocationTrait>::leading_comments(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
                type Field4StringType<'this> = ::puroro::Either<
                    <T as LocationTrait>::Field4StringType<'this>,
                    <U as LocationTrait>::Field4StringType<'this>,
                >;
                fn trailing_comments<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
                    self.as_ref().either(
                        |t| {
                            <T as LocationTrait>::trailing_comments(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as LocationTrait>::trailing_comments(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
                type Field6StringType<'this> = ::puroro::Either<
                    <T as LocationTrait>::Field6StringType<'this>,
                    <U as LocationTrait>::Field6StringType<'this>,
                >;
                type Field6RepeatedType<'this> =
                    ::puroro_internal::impls::either::EitherRepeatedLDField<
                        <T as LocationTrait>::Field6RepeatedType<'this>,
                        <U as LocationTrait>::Field6RepeatedType<'this>,
                    >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::either::EitherRepeatedLDField::new(
                        self.as_ref()
                            .map_left(|t| <T as LocationTrait>::leading_detached_comments(t))
                            .map_right(|u| <U as LocationTrait>::leading_detached_comments(u)),
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct LocationSimpleField1 {
                path: ::std::vec::Vec<i32>,
            }

            impl ::puroro::Message for LocationSimpleField1 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField1 {
                type Field1RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.path.iter().cloned()
                }
                type Field2RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct LocationSimpleField2 {
                span: ::std::vec::Vec<i32>,
            }

            impl ::puroro::Message for LocationSimpleField2 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField2 {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    self.span.iter().cloned()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct LocationSimpleField3 {
                leading_comments: ::std::option::Option<::std::string::String>,
            }

            impl ::puroro::Message for LocationSimpleField3 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField3 {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'this str;
                fn leading_comments<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
                    self.leading_comments.as_ref().map(|v| v.as_ref())
                }
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct LocationSimpleField4 {
                trailing_comments: ::std::option::Option<::std::string::String>,
            }

            impl ::puroro::Message for LocationSimpleField4 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField4 {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'this str;
                fn trailing_comments<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
                    self.trailing_comments.as_ref().map(|v| v.as_ref())
                }
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct LocationSimpleField6 {
                leading_detached_comments: ::std::vec::Vec<::std::string::String>,
            }

            impl ::puroro::Message for LocationSimpleField6 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField6 {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'this str;
                type Field6RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
                    str,
                    ::std::slice::Iter<'this, ::std::string::String>,
                >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro_internal::impls::simple::BorrowedIter::new(
                        self.leading_detached_comments.iter(),
                    )
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait LocationTrait {
                type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
                type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
                type Field3StringType<'this>: ::std::ops::Deref<Target = str>;
                fn leading_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3StringType<'this>> {
                    ::std::default::Default::default()
                }
                type Field4StringType<'this>: ::std::ops::Deref<Target = str>;
                fn trailing_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field4StringType<'this>> {
                    ::std::default::Default::default()
                }
                type Field6StringType<'this>: ::std::ops::Deref<Target = str>;
                type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = Self::Field6StringType<'this>>;
                fn leading_detached_comments<'this>(&'this self)
                -> Self::Field6RepeatedType<'this>;
            }

            macro_rules! location_delegate {
                ($ty:ty) => {
                    type Field1RepeatedType<'this> = <$ty>::Field1RepeatedType<'this>;
                    fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                        (**self).path()
                    }
                    type Field2RepeatedType<'this> = <$ty>::Field2RepeatedType<'this>;
                    fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                        (**self).span()
                    }
                    type Field3StringType<'this> = <$ty>::Field3StringType<'this>;
                    fn leading_comments<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field3StringType<'this>> {
                        (**self).leading_comments()
                    }
                    type Field4StringType<'this> = <$ty>::Field4StringType<'this>;
                    fn trailing_comments<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field4StringType<'this>> {
                        (**self).trailing_comments()
                    }
                    type Field6StringType<'this> = <$ty>::Field6StringType<'this>;
                    type Field6RepeatedType<'this> = <$ty>::Field6RepeatedType<'this>;
                    fn leading_detached_comments<'this>(
                        &'this self,
                    ) -> Self::Field6RepeatedType<'this> {
                        (**self).leading_detached_comments()
                    }
                };
            }

            impl<T> LocationTrait for &'_ T
            where
                T: LocationTrait,
            {
                location_delegate!(T);
            }

            impl<T> LocationTrait for ::std::boxed::Box<T>
            where
                T: LocationTrait,
            {
                location_delegate!(T);
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod location {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
    pub mod generated_code_info {
        mod _puroro_root {
            pub use super::super::super::_puroro_root::*;
        }

        pub use _puroro_impls::AnnotationSimple as Annotation;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct AnnotationSimple {
                pub path: ::std::vec::Vec<i32>,
                pub source_file: ::std::option::Option<::std::string::String>,
                pub begin: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }
            impl ::puroro::Message for AnnotationSimple {}

            impl AnnotationTrait for AnnotationSimple {
                type Field1RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.path.iter().cloned()
                }
                type Field2StringType<'this> = &'this str;
                fn source_file<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    self.source_file.as_ref().map(|v| v.as_ref())
                }
                fn begin<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.begin)
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::DeserFromBytesIter for AnnotationSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for AnnotationSimple {
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
                        1 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.path, data),
                        2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(&mut self.source_file, data),
                        3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.begin, data),
                        4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerToIoWrite for AnnotationSimple {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.path, 1, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(&self.source_file, 2, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.begin, 3, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >::ser_field(&self.end, 4, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl AnnotationTrait for () {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'static str;
            }
            impl<T, U> AnnotationTrait for (T, U)
            where
                T: AnnotationTrait,
                U: AnnotationTrait,
            {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::merged::MergedRepeatedField<
                        <T as AnnotationTrait>::Field1RepeatedType<'this>,
                        <U as AnnotationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::merged::MergedRepeatedField::new(
                        <T as AnnotationTrait>::path(&self.0),
                        <U as AnnotationTrait>::path(&self.1),
                    )
                }
                type Field2StringType<'this> = ::puroro::Either<
                    <T as AnnotationTrait>::Field2StringType<'this>,
                    <U as AnnotationTrait>::Field2StringType<'this>,
                >;
                fn source_file<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    if let Some(right) = <U as AnnotationTrait>::source_file(&self.1) {
                        Some(::puroro::Either::Right(right))
                    } else if let Some(left) = <T as AnnotationTrait>::source_file(&self.0) {
                        Some(::puroro::Either::Left(left))
                    } else {
                        None
                    }
                }
                fn begin<'this>(&'this self) -> Option<i32> {
                    <U as AnnotationTrait>::begin(&self.1)
                        .or_else(|| <T as AnnotationTrait>::begin(&self.0))
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    <U as AnnotationTrait>::end(&self.1)
                        .or_else(|| <T as AnnotationTrait>::end(&self.0))
                }
            }
            impl<T, U> AnnotationTrait for ::puroro::Either<T, U>
            where
                T: AnnotationTrait,
                U: AnnotationTrait,
            {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::either::EitherRepeatedField<
                        <T as AnnotationTrait>::Field1RepeatedType<'this>,
                        <U as AnnotationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::either::EitherRepeatedField::new(
                        self.as_ref()
                            .map_left(|t| <T as AnnotationTrait>::path(t))
                            .map_right(|u| <U as AnnotationTrait>::path(u)),
                    )
                }
                type Field2StringType<'this> = ::puroro::Either<
                    <T as AnnotationTrait>::Field2StringType<'this>,
                    <U as AnnotationTrait>::Field2StringType<'this>,
                >;
                fn source_file<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    self.as_ref().either(
                        |t| {
                            <T as AnnotationTrait>::source_file(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as AnnotationTrait>::source_file(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
                fn begin<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as AnnotationTrait>::begin(t),
                        |u| <U as AnnotationTrait>::begin(u),
                    )
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    self.as_ref().either(
                        |t| <T as AnnotationTrait>::end(t),
                        |u| <U as AnnotationTrait>::end(u),
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct AnnotationSimpleField1 {
                path: ::std::vec::Vec<i32>,
            }

            impl ::puroro::Message for AnnotationSimpleField1 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField1 {
                type Field1RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.path.iter().cloned()
                }
                type Field2StringType<'this> = &'static str;
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct AnnotationSimpleField2 {
                source_file: ::std::option::Option<::std::string::String>,
            }

            impl ::puroro::Message for AnnotationSimpleField2 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField2 {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'this str;
                fn source_file<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    self.source_file.as_ref().map(|v| v.as_ref())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct AnnotationSimpleField3 {
                begin: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for AnnotationSimpleField3 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField3 {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'static str;
                fn begin<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.begin)
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct AnnotationSimpleField4 {
                end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message for AnnotationSimpleField4 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField4 {
                type Field1RepeatedType<'this> =
                    ::puroro_internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro_internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'static str;
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait AnnotationTrait {
                type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
                type Field2StringType<'this>: ::std::ops::Deref<Target = str>;
                fn source_file<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                    ::std::default::Default::default()
                }
                fn begin<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! annotation_delegate {
                ($ty:ty) => {
                    type Field1RepeatedType<'this> = <$ty>::Field1RepeatedType<'this>;
                    fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                        (**self).path()
                    }
                    type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
                    fn source_file<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                        (**self).source_file()
                    }
                    fn begin<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).begin()
                    }
                    fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end()
                    }
                };
            }

            impl<T> AnnotationTrait for &'_ T
            where
                T: AnnotationTrait,
            {
                annotation_delegate!(T);
            }

            impl<T> AnnotationTrait for ::std::boxed::Box<T>
            where
                T: AnnotationTrait,
            {
                annotation_delegate!(T);
            }
        }
        pub use _puroro_nested::*;
        pub mod _puroro_nested {
            pub mod annotation {
                mod _puroro_root {
                    pub use super::super::super::_puroro_root::*;
                }
            }
        }
    }
}
