// A generated source code by puroro library
// package google.protobuf.compiler

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_impls::CodeGeneratorRequestSimple as CodeGeneratorRequest;
pub use _puroro_impls::CodeGeneratorResponseSimple as CodeGeneratorResponse;
pub use _puroro_impls::VersionSimple as Version;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct VersionSimple {
        pub major: ::std::option::Option<i32>,
        pub minor: ::std::option::Option<i32>,
        pub patch: ::std::option::Option<i32>,
        pub suffix: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message for VersionSimple {}

    impl VersionTrait for VersionSimple {
        fn major<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.major)
        }
        fn minor<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.minor)
        }
        fn patch<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.patch)
        }
        type Field4StringType<'this> = &'this str;
        fn suffix<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
            self.suffix.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for VersionSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for VersionSimple {
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
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.major, data),
                2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.minor, data),
                3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >::deser_field(&mut self.patch, data),
                4 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >::deser_field(&mut self.suffix, data),

                _ => unimplemented!("TODO: This case should be handled properly..."),
            }
        }
    }

    impl ::puroro::SerToIoWrite for VersionSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.major, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.minor, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >::ser_field(&self.patch, 3, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.suffix, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl VersionTrait for () {
        type Field4StringType<'this> = &'static str;
    }
    impl<T, U> VersionTrait for (T, U)
    where
        T: VersionTrait,
        U: VersionTrait,
    {
        fn major<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::major(&self.1).or_else(|| <T as VersionTrait>::major(&self.0))
        }
        fn minor<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::minor(&self.1).or_else(|| <T as VersionTrait>::minor(&self.0))
        }
        fn patch<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::patch(&self.1).or_else(|| <T as VersionTrait>::patch(&self.0))
        }
        type Field4StringType<'this> = ::puroro::Either<
            <T as VersionTrait>::Field4StringType<'this>,
            <U as VersionTrait>::Field4StringType<'this>,
        >;
        fn suffix<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
            if let Some(right) = <U as VersionTrait>::suffix(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as VersionTrait>::suffix(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
    }
    impl<T, U> VersionTrait for ::puroro::Either<T, U>
    where
        T: VersionTrait,
        U: VersionTrait,
    {
        fn major<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::major(t),
                |u| <U as VersionTrait>::major(u),
            )
        }
        fn minor<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::minor(t),
                |u| <U as VersionTrait>::minor(u),
            )
        }
        fn patch<'this>(&'this self) -> Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::patch(t),
                |u| <U as VersionTrait>::patch(u),
            )
        }
        type Field4StringType<'this> = ::puroro::Either<
            <T as VersionTrait>::Field4StringType<'this>,
            <U as VersionTrait>::Field4StringType<'this>,
        >;
        fn suffix<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
            self.as_ref().either(
                |t| <T as VersionTrait>::suffix(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as VersionTrait>::suffix(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct VersionSimpleField1 {
        major: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for VersionSimpleField1 {}

    impl super::_puroro_traits::VersionTrait for VersionSimpleField1 {
        fn major<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.major)
        }
        type Field4StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct VersionSimpleField2 {
        minor: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for VersionSimpleField2 {}

    impl super::_puroro_traits::VersionTrait for VersionSimpleField2 {
        fn minor<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.minor)
        }
        type Field4StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct VersionSimpleField3 {
        patch: ::std::option::Option<i32>,
    }

    impl ::puroro::Message for VersionSimpleField3 {}

    impl super::_puroro_traits::VersionTrait for VersionSimpleField3 {
        fn patch<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.patch)
        }
        type Field4StringType<'this> = &'static str;
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct VersionSimpleField4 {
        suffix: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for VersionSimpleField4 {}

    impl super::_puroro_traits::VersionTrait for VersionSimpleField4 {
        type Field4StringType<'this> = &'this str;
        fn suffix<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
            self.suffix.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct CodeGeneratorRequestSimple {
        pub file_to_generate: ::std::vec::Vec<::std::string::String>,
        pub parameter: ::std::option::Option<::std::string::String>,
        pub proto_file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
        >,
        pub compiler_version: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionSimple,
            >,
        >,
    }
    impl ::puroro::Message for CodeGeneratorRequestSimple {}

    impl CodeGeneratorRequestTrait for CodeGeneratorRequestSimple {
        type Field1StringType<'this> = &'this str;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.file_to_generate.iter())
        }
        type Field2StringType<'this> = &'this str;
        fn parameter<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.parameter.as_ref().map(|v| v.as_ref())
        }
        type Field15MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple;
        type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            >,
        >;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.proto_file.iter())
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionSimple;
        fn compiler_version<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.compiler_version.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::DeserFromBytesIter for CodeGeneratorRequestSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorRequestSimple {
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
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.file_to_generate, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.parameter, data),
            15 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple>
            >::deser_field(&mut self.proto_file, data),
            3 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionSimple>
            >::deser_field(&mut self.compiler_version, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for CodeGeneratorRequestSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >::ser_field(&self.file_to_generate, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.parameter, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
                >,
            >::ser_field(&self.proto_file, 15, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionSimple,
                >,
            >::ser_field(&self.compiler_version, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl CodeGeneratorRequestTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field1StringType<'this>>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field2StringType<'this> = &'static str;
        type Field15MessageType<'this> = ();
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }
    impl<T, U> CodeGeneratorRequestTrait for (T, U)
    where
        T: CodeGeneratorRequestTrait,
        U: CodeGeneratorRequestTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field1StringType<'this>,
            <U as CodeGeneratorRequestTrait>::Field1StringType<'this>,
        >;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::merged::MergedRepeatedLDField<
            <T as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedLDField::new(
                <T as CodeGeneratorRequestTrait>::file_to_generate(&self.0),
                <U as CodeGeneratorRequestTrait>::file_to_generate(&self.1),
            )
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field2StringType<'this>,
            <U as CodeGeneratorRequestTrait>::Field2StringType<'this>,
        >;
        fn parameter<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            if let Some(right) = <U as CodeGeneratorRequestTrait>::parameter(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as CodeGeneratorRequestTrait>::parameter(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        type Field15MessageType<'this> = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
                <U as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
            >;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as CodeGeneratorRequestTrait>::proto_file(&self.0),
                <U as CodeGeneratorRequestTrait>::proto_file(&self.1),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            ::puroro::Either<
                <T as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
                <U as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
            >,
            (
                <T as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
                <U as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
            ),
        >;
        fn compiler_version<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as CodeGeneratorRequestTrait>::compiler_version(&self.0),
                <U as CodeGeneratorRequestTrait>::compiler_version(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
            }
        }
    }
    impl<T, U> CodeGeneratorRequestTrait for ::puroro::Either<T, U>
    where
        T: CodeGeneratorRequestTrait,
        U: CodeGeneratorRequestTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field1StringType<'this>,
            <U as CodeGeneratorRequestTrait>::Field1StringType<'this>,
        >;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::either::EitherRepeatedLDField<
            <T as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorRequestTrait>::file_to_generate(t))
                    .map_right(|u| <U as CodeGeneratorRequestTrait>::file_to_generate(u)),
            )
        }
        type Field2StringType<'this> = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field2StringType<'this>,
            <U as CodeGeneratorRequestTrait>::Field2StringType<'this>,
        >;
        fn parameter<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as CodeGeneratorRequestTrait>::parameter(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as CodeGeneratorRequestTrait>::parameter(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field15MessageType<'this> = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
                <U as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
            >;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorRequestTrait>::proto_file(t))
                    .map_right(|u| <U as CodeGeneratorRequestTrait>::proto_file(u)),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
            <U as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
        >;
        fn compiler_version<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as CodeGeneratorRequestTrait>::compiler_version(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as CodeGeneratorRequestTrait>::compiler_version(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct CodeGeneratorRequestSimpleField1 {
        file_to_generate: ::std::vec::Vec<::std::string::String>,
    }

    impl ::puroro::Message for CodeGeneratorRequestSimpleField1 {}

    impl super::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequestSimpleField1 {
        type Field1StringType<'this> = &'this str;
        type Field1RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.file_to_generate.iter())
        }
        type Field2StringType<'this> = &'static str;
        type Field15MessageType<'this> = ();
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct CodeGeneratorRequestSimpleField2 {
        parameter: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for CodeGeneratorRequestSimpleField2 {}

    impl super::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequestSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field1StringType<'this>>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field2StringType<'this> = &'this str;
        fn parameter<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.parameter.as_ref().map(|v| v.as_ref())
        }
        type Field15MessageType<'this> = ();
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct CodeGeneratorRequestSimpleField15 {
        proto_file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
        >,
    }

    impl ::puroro::Message for CodeGeneratorRequestSimpleField15 {}

    impl super::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequestSimpleField15 {
        type Field1StringType<'this> = &'static str;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field1StringType<'this>>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field2StringType<'this> = &'static str;
        type Field15MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple;
        type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_impls::FileDescriptorProtoSimple,
            >,
        >;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.proto_file.iter())
        }
        type Field3MessageType<'this> = ();
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct CodeGeneratorRequestSimpleField3 {
        compiler_version: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionSimple,
            >,
        >,
    }

    impl ::puroro::Message for CodeGeneratorRequestSimpleField3 {}

    impl super::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequestSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field1RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field1StringType<'this>>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field2StringType<'this> = &'static str;
        type Field15MessageType<'this> = ();
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::compiler::_puroro_impls::VersionSimple;
        fn compiler_version<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.compiler_version.as_ref().map(|v| v.as_ref())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct CodeGeneratorResponseSimple {
    pub error: ::std::option::Option<::std::string::String>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::std::vec::Vec<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple>,
}
    impl ::puroro::Message for CodeGeneratorResponseSimple {}

    impl CodeGeneratorResponseTrait for CodeGeneratorResponseSimple {
        type Field1StringType<'this> = &'this str;
        fn error<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.error.as_ref().map(|v| v.as_ref())
        }
        fn supported_features<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.supported_features)
        }
        type Field15MessageType<'this> = &'this self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple;
        type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple>>;

        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.file.iter())
        }
    }

    impl ::puroro::DeserFromBytesIter for CodeGeneratorResponseSimple {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro_internal::de::DeserFieldsFromBytesIter for CodeGeneratorResponseSimple {
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
            >::deser_field(&mut self.error, data),
            2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.supported_features, data),
            15 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple>
            >::deser_field(&mut self.file, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerToIoWrite for CodeGeneratorResponseSimple {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >::ser_field(&self.error, 1, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >::ser_field(&self.supported_features, 2, out)?;
            ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple>
        >::ser_field(&self.file, 15, out)?;
            ::std::result::Result::Ok(())
        }
    }
    impl CodeGeneratorResponseTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field15MessageType<'this> = ();
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> CodeGeneratorResponseTrait for (T, U)
    where
        T: CodeGeneratorResponseTrait,
        U: CodeGeneratorResponseTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::Field1StringType<'this>,
            <U as CodeGeneratorResponseTrait>::Field1StringType<'this>,
        >;
        fn error<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            if let Some(right) = <U as CodeGeneratorResponseTrait>::error(&self.1) {
                Some(::puroro::Either::Right(right))
            } else if let Some(left) = <T as CodeGeneratorResponseTrait>::error(&self.0) {
                Some(::puroro::Either::Left(left))
            } else {
                None
            }
        }
        fn supported_features<'this>(&'this self) -> Option<u64> {
            <U as CodeGeneratorResponseTrait>::supported_features(&self.1)
                .or_else(|| <T as CodeGeneratorResponseTrait>::supported_features(&self.0))
        }
        type Field15MessageType<'this> = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::merged::MergedRepeatedMessageField<
                <T as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
                <U as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
            >;

        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::merged::MergedRepeatedMessageField::new(
                <T as CodeGeneratorResponseTrait>::file(&self.0),
                <U as CodeGeneratorResponseTrait>::file(&self.1),
            )
        }
    }
    impl<T, U> CodeGeneratorResponseTrait for ::puroro::Either<T, U>
    where
        T: CodeGeneratorResponseTrait,
        U: CodeGeneratorResponseTrait,
    {
        type Field1StringType<'this> = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::Field1StringType<'this>,
            <U as CodeGeneratorResponseTrait>::Field1StringType<'this>,
        >;
        fn error<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.as_ref().either(
                |t| <T as CodeGeneratorResponseTrait>::error(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as CodeGeneratorResponseTrait>::error(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        fn supported_features<'this>(&'this self) -> Option<u64> {
            self.as_ref().either(
                |t| <T as CodeGeneratorResponseTrait>::supported_features(t),
                |u| <U as CodeGeneratorResponseTrait>::supported_features(u),
            )
        }
        type Field15MessageType<'this> = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::either::EitherRepeatedMessageField<
                <T as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
                <U as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
            >;

        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorResponseTrait>::file(t))
                    .map_right(|u| <U as CodeGeneratorResponseTrait>::file(u)),
            )
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct CodeGeneratorResponseSimpleField1 {
        error: ::std::option::Option<::std::string::String>,
    }

    impl ::puroro::Message for CodeGeneratorResponseSimpleField1 {}

    impl super::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponseSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn error<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.error.as_ref().map(|v| v.as_ref())
        }
        type Field15MessageType<'this> = ();
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct CodeGeneratorResponseSimpleField2 {
        supported_features: ::std::option::Option<u64>,
    }

    impl ::puroro::Message for CodeGeneratorResponseSimpleField2 {}

    impl super::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponseSimpleField2 {
        type Field1StringType<'this> = &'static str;
        fn supported_features<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.supported_features)
        }
        type Field15MessageType<'this> = ();
        type Field15RepeatedType<'this> =
            ::puroro_internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    struct CodeGeneratorResponseSimpleField15 {
    file: ::std::vec::Vec<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple>,
}

    impl ::puroro::Message for CodeGeneratorResponseSimpleField15 {}

    impl super::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponseSimpleField15 {
        type Field1StringType<'this> = &'static str;
        type Field15MessageType<'this> = &'this self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple;
        type Field15RepeatedType<'this> = ::puroro_internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_impls::FileSimple>>;

        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro_internal::impls::simple::BorrowedIter::new(self.file.iter())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait VersionTrait {
        fn major<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn minor<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        fn patch<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field4StringType<'this>: ::std::ops::Deref<Target = str>;
        fn suffix<'this>(&'this self) -> ::std::option::Option<Self::Field4StringType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! version_delegate {
        ($ty:ty) => {
            fn major<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).major()
            }
            fn minor<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).minor()
            }
            fn patch<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).patch()
            }
            type Field4StringType<'this> = <$ty>::Field4StringType<'this>;
            fn suffix<'this>(&'this self) -> ::std::option::Option<Self::Field4StringType<'this>> {
                (**self).suffix()
            }
        };
    }

    impl<T> VersionTrait for &'_ T
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }

    impl<T> VersionTrait for ::std::boxed::Box<T>
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }
    pub trait CodeGeneratorRequestTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field1StringType<'this>>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>;
        fn parameter<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field15MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait;
        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
        type Field3MessageType<'this>: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait;
        fn compiler_version<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::default::Default::default()
        }
    }

    macro_rules! code_generator_request_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            type Field1RepeatedType<'this> = <$ty>::Field1RepeatedType<'this>;
            fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                (**self).file_to_generate()
            }
            type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
            fn parameter<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                (**self).parameter()
            }
            type Field15MessageType<'this> = <$ty>::Field15MessageType<'this>;
            type Field15RepeatedType<'this> = <$ty>::Field15RepeatedType<'this>;
            fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
                (**self).proto_file()
            }
            type Field3MessageType<'this> = <$ty>::Field3MessageType<'this>;
            fn compiler_version<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).compiler_version()
            }
        };
    }

    impl<T> CodeGeneratorRequestTrait for &'_ T
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }

    impl<T> CodeGeneratorRequestTrait for ::std::boxed::Box<T>
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }
    pub trait CodeGeneratorResponseTrait {
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
        fn error<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        fn supported_features<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::default::Default::default()
        }
        type Field15MessageType<'this>: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait;
        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field15MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
    }

    macro_rules! code_generator_response_delegate {
        ($ty:ty) => {
            type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
            fn error<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
                (**self).error()
            }
            fn supported_features<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).supported_features()
            }
            type Field15MessageType<'this> = <$ty>::Field15MessageType<'this>;
            type Field15RepeatedType<'this> = <$ty>::Field15RepeatedType<'this>;
            fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
                (**self).file()
            }
        };
    }

    impl<T> CodeGeneratorResponseTrait for &'_ T
    where
        T: CodeGeneratorResponseTrait,
    {
        code_generator_response_delegate!(T);
    }

    impl<T> CodeGeneratorResponseTrait for ::std::boxed::Box<T>
    where
        T: CodeGeneratorResponseTrait,
    {
        code_generator_response_delegate!(T);
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

        pub use _puroro_impls::FileSimple as File;
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
            pub struct FileSimple {
            pub name: ::std::option::Option<::std::string::String>,
            pub insertion_point: ::std::option::Option<::std::string::String>,
            pub content: ::std::option::Option<::std::string::String>,
            pub generated_code_info: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoSimple>>,
        }
            impl ::puroro::Message for FileSimple {}

            impl FileTrait for FileSimple {
                type Field1StringType<'this> = &'this str;
                fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.name.as_ref().map(|v| v.as_ref())
                }
                type Field2StringType<'this> = &'this str;
                fn insertion_point<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    self.insertion_point.as_ref().map(|v| v.as_ref())
                }
                type Field15StringType<'this> = &'this str;
                fn content<'this>(&'this self) -> Option<Self::Field15StringType<'this>> {
                    self.content.as_ref().map(|v| v.as_ref())
                }
                type Field16MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoSimple;
                fn generated_code_info<'this>(
                    &'this self,
                ) -> Option<Self::Field16MessageType<'this>> {
                    self.generated_code_info.as_ref().map(|v| v.as_ref())
                }
            }

            impl ::puroro::DeserFromBytesIter for FileSimple {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro_internal::de::DeserFieldsFromBytesIter for FileSimple {
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
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.name, data),
                    2 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.insertion_point, data),
                    15 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.content, data),
                    16 => ::puroro_internal::impls::simple::de::DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoSimple>
                    >::deser_field(&mut self.generated_code_info, data),
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
                }
            }

            impl ::puroro::SerToIoWrite for FileSimple {
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
                    >::ser_field(&self.insertion_point, 2, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >::ser_field(&self.content, 15, out)?;
                    ::puroro_internal::impls::simple::se::SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoSimple>
                >::ser_field(&self.generated_code_info, 16, out)?;
                    ::std::result::Result::Ok(())
                }
            }
            impl FileTrait for () {
                type Field1StringType<'this> = &'static str;
                type Field2StringType<'this> = &'static str;
                type Field15StringType<'this> = &'static str;
                type Field16MessageType<'this> = ();
            }
            impl<T, U> FileTrait for (T, U)
            where
                T: FileTrait,
                U: FileTrait,
            {
                type Field1StringType<'this> = ::puroro::Either<
                    <T as FileTrait>::Field1StringType<'this>,
                    <U as FileTrait>::Field1StringType<'this>,
                >;
                fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    if let Some(right) = <U as FileTrait>::name(&self.1) {
                        Some(::puroro::Either::Right(right))
                    } else if let Some(left) = <T as FileTrait>::name(&self.0) {
                        Some(::puroro::Either::Left(left))
                    } else {
                        None
                    }
                }
                type Field2StringType<'this> = ::puroro::Either<
                    <T as FileTrait>::Field2StringType<'this>,
                    <U as FileTrait>::Field2StringType<'this>,
                >;
                fn insertion_point<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    if let Some(right) = <U as FileTrait>::insertion_point(&self.1) {
                        Some(::puroro::Either::Right(right))
                    } else if let Some(left) = <T as FileTrait>::insertion_point(&self.0) {
                        Some(::puroro::Either::Left(left))
                    } else {
                        None
                    }
                }
                type Field15StringType<'this> = ::puroro::Either<
                    <T as FileTrait>::Field15StringType<'this>,
                    <U as FileTrait>::Field15StringType<'this>,
                >;
                fn content<'this>(&'this self) -> Option<Self::Field15StringType<'this>> {
                    if let Some(right) = <U as FileTrait>::content(&self.1) {
                        Some(::puroro::Either::Right(right))
                    } else if let Some(left) = <T as FileTrait>::content(&self.0) {
                        Some(::puroro::Either::Left(left))
                    } else {
                        None
                    }
                }
                type Field16MessageType<'this> = ::puroro::Either<
                    ::puroro::Either<
                        <T as FileTrait>::Field16MessageType<'this>,
                        <U as FileTrait>::Field16MessageType<'this>,
                    >,
                    (
                        <T as FileTrait>::Field16MessageType<'this>,
                        <U as FileTrait>::Field16MessageType<'this>,
                    ),
                >;
                fn generated_code_info<'this>(
                    &'this self,
                ) -> Option<Self::Field16MessageType<'this>> {
                    match (
                        <T as FileTrait>::generated_code_info(&self.0),
                        <U as FileTrait>::generated_code_info(&self.1),
                    ) {
                        (None, None) => None,
                        (Some(t), None) => Some(::puroro::Either::Left(::puroro::Either::Left(t))),
                        (None, Some(u)) => Some(::puroro::Either::Left(::puroro::Either::Right(u))),
                        (Some(t), Some(u)) => Some(::puroro::Either::Right((t, u))),
                    }
                }
            }
            impl<T, U> FileTrait for ::puroro::Either<T, U>
            where
                T: FileTrait,
                U: FileTrait,
            {
                type Field1StringType<'this> = ::puroro::Either<
                    <T as FileTrait>::Field1StringType<'this>,
                    <U as FileTrait>::Field1StringType<'this>,
                >;
                fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::name(t).map(|t| ::puroro::Either::Left(t)),
                        |u| <U as FileTrait>::name(u).map(|u| ::puroro::Either::Right(u)),
                    )
                }
                type Field2StringType<'this> = ::puroro::Either<
                    <T as FileTrait>::Field2StringType<'this>,
                    <U as FileTrait>::Field2StringType<'this>,
                >;
                fn insertion_point<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::insertion_point(t).map(|t| ::puroro::Either::Left(t)),
                        |u| {
                            <U as FileTrait>::insertion_point(u).map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
                type Field15StringType<'this> = ::puroro::Either<
                    <T as FileTrait>::Field15StringType<'this>,
                    <U as FileTrait>::Field15StringType<'this>,
                >;
                fn content<'this>(&'this self) -> Option<Self::Field15StringType<'this>> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::content(t).map(|t| ::puroro::Either::Left(t)),
                        |u| <U as FileTrait>::content(u).map(|u| ::puroro::Either::Right(u)),
                    )
                }
                type Field16MessageType<'this> = ::puroro::Either<
                    <T as FileTrait>::Field16MessageType<'this>,
                    <U as FileTrait>::Field16MessageType<'this>,
                >;
                fn generated_code_info<'this>(
                    &'this self,
                ) -> Option<Self::Field16MessageType<'this>> {
                    self.as_ref().either(
                        |t| {
                            <T as FileTrait>::generated_code_info(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as FileTrait>::generated_code_info(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct FileSimpleField1 {
                name: ::std::option::Option<::std::string::String>,
            }

            impl ::puroro::Message for FileSimpleField1 {}

            impl super::_puroro_traits::FileTrait for FileSimpleField1 {
                type Field1StringType<'this> = &'this str;
                fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.name.as_ref().map(|v| v.as_ref())
                }
                type Field2StringType<'this> = &'static str;
                type Field15StringType<'this> = &'static str;
                type Field16MessageType<'this> = ();
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct FileSimpleField2 {
                insertion_point: ::std::option::Option<::std::string::String>,
            }

            impl ::puroro::Message for FileSimpleField2 {}

            impl super::_puroro_traits::FileTrait for FileSimpleField2 {
                type Field1StringType<'this> = &'static str;
                type Field2StringType<'this> = &'this str;
                fn insertion_point<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    self.insertion_point.as_ref().map(|v| v.as_ref())
                }
                type Field15StringType<'this> = &'static str;
                type Field16MessageType<'this> = ();
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct FileSimpleField15 {
                content: ::std::option::Option<::std::string::String>,
            }

            impl ::puroro::Message for FileSimpleField15 {}

            impl super::_puroro_traits::FileTrait for FileSimpleField15 {
                type Field1StringType<'this> = &'static str;
                type Field2StringType<'this> = &'static str;
                type Field15StringType<'this> = &'this str;
                fn content<'this>(&'this self) -> Option<Self::Field15StringType<'this>> {
                    self.content.as_ref().map(|v| v.as_ref())
                }
                type Field16MessageType<'this> = ();
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            struct FileSimpleField16 {
            generated_code_info: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoSimple>>,
        }

            impl ::puroro::Message for FileSimpleField16 {}

            impl super::_puroro_traits::FileTrait for FileSimpleField16 {
                type Field1StringType<'this> = &'static str;
                type Field2StringType<'this> = &'static str;
                type Field15StringType<'this> = &'static str;
                type Field16MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_impls::GeneratedCodeInfoSimple;
                fn generated_code_info<'this>(
                    &'this self,
                ) -> Option<Self::Field16MessageType<'this>> {
                    self.generated_code_info.as_ref().map(|v| v.as_ref())
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait FileTrait {
                type Field1StringType<'this>: ::std::ops::Deref<Target = str>;
                fn name<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field1StringType<'this>> {
                    ::std::default::Default::default()
                }
                type Field2StringType<'this>: ::std::ops::Deref<Target = str>;
                fn insertion_point<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                    ::std::default::Default::default()
                }
                type Field15StringType<'this>: ::std::ops::Deref<Target = str>;
                fn content<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field15StringType<'this>> {
                    ::std::default::Default::default()
                }
                type Field16MessageType<'this>: self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait;
                fn generated_code_info<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                    ::std::default::Default::default()
                }
            }

            macro_rules! file_delegate {
                ($ty:ty) => {
                    type Field1StringType<'this> = <$ty>::Field1StringType<'this>;
                    fn name<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field1StringType<'this>> {
                        (**self).name()
                    }
                    type Field2StringType<'this> = <$ty>::Field2StringType<'this>;
                    fn insertion_point<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                        (**self).insertion_point()
                    }
                    type Field15StringType<'this> = <$ty>::Field15StringType<'this>;
                    fn content<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field15StringType<'this>> {
                        (**self).content()
                    }
                    type Field16MessageType<'this> = <$ty>::Field16MessageType<'this>;
                    fn generated_code_info<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                        (**self).generated_code_info()
                    }
                };
            }

            impl<T> FileTrait for &'_ T
            where
                T: FileTrait,
            {
                file_delegate!(T);
            }

            impl<T> FileTrait for ::std::boxed::Box<T>
            where
                T: FileTrait,
            {
                file_delegate!(T);
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
