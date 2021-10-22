// A generated source code by puroro library
// package google.protobuf.compiler

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::CodeGeneratorRequest;
pub use _puroro_simple_impl::CodeGeneratorResponse;
pub use _puroro_simple_impl::Version;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct Version {
        pub major: ::std::option::Option<i32>,
        pub minor: ::std::option::Option<i32>,
        pub patch: ::std::option::Option<i32>,
        pub suffix: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message<Version> for Version {}

    impl super::_puroro_traits::VersionTrait for Version {
        fn major_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.major)
        }
        fn minor_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.minor)
        }
        fn patch_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.patch)
        }
        fn suffix_opt<'this>(&'this self) -> Option<&'this str> {
            self.suffix.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for Version {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 4]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "major",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <Version as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "minor",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <Version as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "patch",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <Version as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "suffix",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <Version as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "Version",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for Version {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for Version {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.major, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.minor, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.patch, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.suffix, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for Version {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.major,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.minor,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.patch,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.suffix,
                4,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct CodeGeneratorRequest {
        pub file_to_generate: ::std::vec::Vec<::std::string::String>,
        pub parameter: ::std::option::Option<::std::string::String>,
        pub proto_file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
        >,
        pub compiler_version: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version,
            >,
        >,
    }
    impl ::puroro::Message<CodeGeneratorRequest> for CodeGeneratorRequest {}

    impl super::_puroro_traits::CodeGeneratorRequestTrait for CodeGeneratorRequest {
        type Field1RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.file_to_generate.iter())
        }
        fn parameter_opt<'this>(&'this self) -> Option<&'this str> {
            self.parameter.as_ref().map(|v| v.as_ref())
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto;
        type Field15RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            >,
        >;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.proto_file.iter())
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version;
        fn compiler_version_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.compiler_version.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for CodeGeneratorRequest {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 4]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "file_to_generate",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <CodeGeneratorRequest as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "parameter",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <CodeGeneratorRequest as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "proto_file",
                                number: 15,
                                lazy_containing_type: Lazy::new(|| {
                                    <CodeGeneratorRequest as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "compiler_version",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <CodeGeneratorRequest as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "CodeGeneratorRequest",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for CodeGeneratorRequest {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for CodeGeneratorRequest {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.file_to_generate, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.parameter, data),
            15 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto>
            >::deser_field(&mut self.proto_file, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version>>
            >::deser_field(&mut self.compiler_version, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for CodeGeneratorRequest {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.file_to_generate,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.parameter,
                2,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
                >,
            >::ser_field(&self.proto_file, 15, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::compiler::_puroro_simple_impl::Version>>
        >::ser_field(&self.compiler_version, 3, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct CodeGeneratorResponse {
    pub error: ::std::option::Option<::std::string::String>,
    pub supported_features: ::std::option::Option<u64>,
    pub file: ::std::vec::Vec<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File>,
}
    impl ::puroro::Message<CodeGeneratorResponse> for CodeGeneratorResponse {}

    impl super::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponse {
        fn error_opt<'this>(&'this self) -> Option<&'this str> {
            self.error.as_ref().map(|v| v.as_ref())
        }
        fn supported_features_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.supported_features)
        }
        type Field15MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File;
        type Field15RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File>>;

        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.file.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for CodeGeneratorResponse {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 3]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "error",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <CodeGeneratorResponse as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "supported_features",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <CodeGeneratorResponse as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "file",
                                number: 15,
                                lazy_containing_type: Lazy::new(|| {
                                    <CodeGeneratorResponse as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "CodeGeneratorResponse",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for CodeGeneratorResponse {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for CodeGeneratorResponse {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.error, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.supported_features, data),
            15 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File>
            >::deser_field(&mut self.file, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for CodeGeneratorResponse {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.error,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt64>::ser_field(
                &self.supported_features,
                2,
                out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_simple_impl::File>
        >::ser_field(&self.file, 15, out)?;

            ::std::result::Result::Ok(())
        }
    }
}

pub use _puroro_impls::*;
pub mod _puroro_impls {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    use super::_puroro_traits::*;
    impl VersionTrait for () {}
    impl<T, U> VersionTrait for (T, U)
    where
        T: VersionTrait,
        U: VersionTrait,
    {
        fn major_opt<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::major_opt(&self.1)
                .or_else(|| <T as VersionTrait>::major_opt(&self.0))
        }
        fn minor_opt<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::minor_opt(&self.1)
                .or_else(|| <T as VersionTrait>::minor_opt(&self.0))
        }
        fn patch_opt<'this>(&'this self) -> Option<i32> {
            <U as VersionTrait>::patch_opt(&self.1)
                .or_else(|| <T as VersionTrait>::patch_opt(&self.0))
        }
        fn suffix_opt<'this>(&'this self) -> Option<&'this str> {
            <U as VersionTrait>::suffix_opt(&self.1)
                .or_else(|| <T as VersionTrait>::suffix_opt(&self.0))
        }
    }
    impl<T, U> VersionTrait for ::puroro::Either<T, U>
    where
        T: VersionTrait,
        U: VersionTrait,
    {
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::major_opt(t),
                |u| <U as VersionTrait>::major_opt(u),
            )
        }
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::minor_opt(t),
                |u| <U as VersionTrait>::minor_opt(u),
            )
        }
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as VersionTrait>::patch_opt(t),
                |u| <U as VersionTrait>::patch_opt(u),
            )
        }
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as VersionTrait>::suffix_opt(t),
                |u| <U as VersionTrait>::suffix_opt(u),
            )
        }
    }
    impl<T> VersionTrait for ::std::option::Option<T>
    where
        T: VersionTrait,
    {
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.major_opt())
        }
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.minor_opt())
        }
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.patch_opt())
        }
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.suffix_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct VersionSingleField1 {
        pub major: i32,
    }

    impl ::puroro::Message<super::Version> for VersionSingleField1 {}

    impl super::_puroro_traits::VersionTrait for VersionSingleField1 {
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.major))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for VersionSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.major), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for VersionSingleField1 {
        fn from(value: i32) -> Self {
            Self { major: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct VersionSingleField2 {
        pub minor: i32,
    }

    impl ::puroro::Message<super::Version> for VersionSingleField2 {}

    impl super::_puroro_traits::VersionTrait for VersionSingleField2 {
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.minor))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for VersionSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.minor), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for VersionSingleField2 {
        fn from(value: i32) -> Self {
            Self { minor: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct VersionSingleField3 {
        pub patch: i32,
    }

    impl ::puroro::Message<super::Version> for VersionSingleField3 {}

    impl super::_puroro_traits::VersionTrait for VersionSingleField3 {
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.patch))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for VersionSingleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.patch), 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for VersionSingleField3 {
        fn from(value: i32) -> Self {
            Self { patch: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub suffix: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::Version> for VersionSingleField4<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::VersionTrait for VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.suffix.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(::std::iter::once(&self.suffix), 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for VersionSingleField4<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { suffix: value }
        }
    }
    pub struct VersionBuilder<T>(T);

    impl<T> VersionBuilder<T>
    where
        T: VersionTrait,
    {
        pub fn append_major(self, value: i32) -> VersionBuilder<(T, VersionSingleField1)> {
            VersionBuilder((self.0, VersionSingleField1 { major: value }))
        }

        pub fn append_minor(self, value: i32) -> VersionBuilder<(T, VersionSingleField2)> {
            VersionBuilder((self.0, VersionSingleField2 { minor: value }))
        }

        pub fn append_patch(self, value: i32) -> VersionBuilder<(T, VersionSingleField3)> {
            VersionBuilder((self.0, VersionSingleField3 { patch: value }))
        }

        pub fn append_suffix<ScalarType>(
            self,
            value: ScalarType,
        ) -> VersionBuilder<(T, VersionSingleField4<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            VersionBuilder((self.0, VersionSingleField4 { suffix: value }))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl VersionBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl CodeGeneratorRequestTrait for () {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> CodeGeneratorRequestTrait for (T, U)
    where
        T: CodeGeneratorRequestTrait,
        U: CodeGeneratorRequestTrait,
    {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as CodeGeneratorRequestTrait>::file_to_generate(&self.0),
                <U as CodeGeneratorRequestTrait>::file_to_generate(&self.1),
            )
        }
        fn parameter_opt<'this>(&'this self) -> Option<&'this str> {
            <U as CodeGeneratorRequestTrait>::parameter_opt(&self.1)
                .or_else(|| <T as CodeGeneratorRequestTrait>::parameter_opt(&self.0))
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
        >;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as CodeGeneratorRequestTrait>::proto_file(&self.0),
                <U as CodeGeneratorRequestTrait>::proto_file(&self.1),
            )
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as CodeGeneratorRequestTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as CodeGeneratorRequestTrait>::Field3MessageType<'this>>,
        );
        fn compiler_version_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as CodeGeneratorRequestTrait>::compiler_version_opt(&self.0),
                <U as CodeGeneratorRequestTrait>::compiler_version_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
    }
    impl<T, U> CodeGeneratorRequestTrait for ::puroro::Either<T, U>
    where
        T: CodeGeneratorRequestTrait,
        U: CodeGeneratorRequestTrait,
    {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::Field1RepeatedType<'this>,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorRequestTrait>::file_to_generate(t))
                    .map_right(|u| <U as CodeGeneratorRequestTrait>::file_to_generate(u)),
            )
        }
        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as CodeGeneratorRequestTrait>::parameter_opt(t),
                |u| <U as CodeGeneratorRequestTrait>::parameter_opt(u),
            )
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorRequestTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
            <T as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
            <U as CodeGeneratorRequestTrait>::Field15RepeatedType<'this>,
        >;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorRequestTrait>::proto_file(t))
                    .map_right(|u| <U as CodeGeneratorRequestTrait>::proto_file(u)),
            )
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
            <U as CodeGeneratorRequestTrait>::Field3MessageType<'this>,
        >;
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as CodeGeneratorRequestTrait>::compiler_version_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as CodeGeneratorRequestTrait>::compiler_version_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }
    impl<T> CodeGeneratorRequestTrait for ::std::option::Option<T>
    where
        T: CodeGeneratorRequestTrait,
    {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field1RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.file_to_generate().into_iter())
                .into_iter()
                .flatten()
        }
        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.parameter_opt())
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = T::Field15MessageType<'this>;

        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field15RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.proto_file().into_iter())
                .into_iter()
                .flatten()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = T::Field3MessageType<'this>;
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.compiler_version_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub file_to_generate: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::AsRefIter<
            <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
            str,
        >;

        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::single_field::AsRefIter::new(
                ::std::iter::IntoIterator::into_iter(&self.file_to_generate),
            )
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(&self.file_to_generate, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                file_to_generate: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub parameter: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.parameter.as_ref())
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(::std::iter::once(&self.parameter), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for CodeGeneratorRequestSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { parameter: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub proto_file: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.proto_file)
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<ScalarType>
        >::ser_field::
        <ScalarType, _, _>
        (
            &self.proto_file,
            15,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { proto_file: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub compiler_version: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::CodeGeneratorRequest>
        for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::CodeGeneratorRequestTrait
        for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::Some(&self.compiler_version)
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Message<ScalarType>
        >::ser_field::
        <ScalarType, _, _>
        (
            ::std::iter::once(&self.compiler_version),
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for CodeGeneratorRequestSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                compiler_version: value,
            }
        }
    }
    pub struct CodeGeneratorRequestBuilder<T>(T);

    impl<T> CodeGeneratorRequestBuilder<T>
    where
        T: CodeGeneratorRequestTrait,
    {
        pub fn append_file_to_generate<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> CodeGeneratorRequestBuilder<(
            T,
            CodeGeneratorRequestSingleField1<ScalarType, RepeatedType>,
        )>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField1 {
                    file_to_generate: value,
                },
            ))
        }

        pub fn append_parameter<ScalarType>(
            self,
            value: ScalarType,
        ) -> CodeGeneratorRequestBuilder<(T, CodeGeneratorRequestSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField2 { parameter: value },
            ))
        }

        pub fn append_proto_file<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> CodeGeneratorRequestBuilder<(
            T,
            CodeGeneratorRequestSingleField15<ScalarType, RepeatedType>,
        )>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField15 { proto_file: value },
            ))
        }

        pub fn append_compiler_version<ScalarType>(
            self,
            value: ScalarType,
        ) -> CodeGeneratorRequestBuilder<(T, CodeGeneratorRequestSingleField3<ScalarType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
        {
            CodeGeneratorRequestBuilder((
                self.0,
                CodeGeneratorRequestSingleField3 {
                    compiler_version: value,
                },
            ))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl CodeGeneratorRequestBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl CodeGeneratorResponseTrait for () {
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> CodeGeneratorResponseTrait for (T, U)
    where
        T: CodeGeneratorResponseTrait,
        U: CodeGeneratorResponseTrait,
    {
        fn error_opt<'this>(&'this self) -> Option<&'this str> {
            <U as CodeGeneratorResponseTrait>::error_opt(&self.1)
                .or_else(|| <T as CodeGeneratorResponseTrait>::error_opt(&self.0))
        }
        fn supported_features_opt<'this>(&'this self) -> Option<u64> {
            <U as CodeGeneratorResponseTrait>::supported_features_opt(&self.1)
                .or_else(|| <T as CodeGeneratorResponseTrait>::supported_features_opt(&self.0))
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
            <U as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
        >;

        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as CodeGeneratorResponseTrait>::error_opt(t),
                |u| <U as CodeGeneratorResponseTrait>::error_opt(u),
            )
        }
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().either(
                |t| <T as CodeGeneratorResponseTrait>::supported_features_opt(t),
                |u| <U as CodeGeneratorResponseTrait>::supported_features_opt(u),
            )
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
            <U as CodeGeneratorResponseTrait>::Field15MessageType<'this>,
        >;
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
            <T as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
            <U as CodeGeneratorResponseTrait>::Field15RepeatedType<'this>,
        >;

        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as CodeGeneratorResponseTrait>::file(t))
                    .map_right(|u| <U as CodeGeneratorResponseTrait>::file(u)),
            )
        }
    }
    impl<T> CodeGeneratorResponseTrait for ::std::option::Option<T>
    where
        T: CodeGeneratorResponseTrait,
    {
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.error_opt())
        }
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.supported_features_opt())
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = T::Field15MessageType<'this>;

        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field15RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.file().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub error: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::CodeGeneratorResponse>
        for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::CodeGeneratorResponseTrait
        for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.error.as_ref())
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field::<
                ScalarType,
                _,
                _,
            >(::std::iter::once(&self.error), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for CodeGeneratorResponseSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { error: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct CodeGeneratorResponseSingleField2 {
        pub supported_features: u64,
    }

    impl ::puroro::Message<super::CodeGeneratorResponse> for CodeGeneratorResponseSingleField2 {}

    impl super::_puroro_traits::CodeGeneratorResponseTrait for CodeGeneratorResponseSingleField2 {
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.supported_features))
        }
        type Field15MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field15RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field15MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for CodeGeneratorResponseSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt64>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.supported_features), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<u64> for CodeGeneratorResponseSingleField2 {
        fn from(value: u64) -> Self {
            Self {
                supported_features: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    pub file: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::CodeGeneratorResponse>
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::CodeGeneratorResponseTrait
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
type Field15MessageType<'this> where Self: 'this = &'this ScalarType;
type Field15RepeatedType<'this> where Self: 'this
    = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
    ::std::iter::IntoIterator::into_iter(&self.file)
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
{
    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
    where
        W: ::std::io::Write
    {
        use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
        SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<ScalarType>
        >::ser_field::
        <ScalarType, _, _>
        (
            &self.file,
            15,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            file: value,
        }
    }
}
    pub struct CodeGeneratorResponseBuilder<T>(T);

    impl<T> CodeGeneratorResponseBuilder<T>
    where
        T: CodeGeneratorResponseTrait,
    {
        pub fn append_error<ScalarType>(
            self,
            value: ScalarType,
        ) -> CodeGeneratorResponseBuilder<(T, CodeGeneratorResponseSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            CodeGeneratorResponseBuilder((
                self.0,
                CodeGeneratorResponseSingleField1 { error: value },
            ))
        }

        pub fn append_supported_features(
            self,
            value: u64,
        ) -> CodeGeneratorResponseBuilder<(T, CodeGeneratorResponseSingleField2)> {
            CodeGeneratorResponseBuilder((
                self.0,
                CodeGeneratorResponseSingleField2 {
                    supported_features: value,
                },
            ))
        }

    pub fn append_file<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> CodeGeneratorResponseBuilder<(T, CodeGeneratorResponseSingleField15<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            CodeGeneratorResponseBuilder((
                self.0,
                CodeGeneratorResponseSingleField15 { file: value },
            ))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl CodeGeneratorResponseBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
}
pub use _puroro_traits::*;
pub mod _puroro_traits {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }

    pub trait VersionTrait {
        fn major<'this>(&'this self) -> i32 {
            self.major_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_major<'this>(&'this self) -> bool {
            self.major_opt().is_some()
        }
        fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn minor<'this>(&'this self) -> i32 {
            self.minor_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_minor<'this>(&'this self) -> bool {
            self.minor_opt().is_some()
        }
        fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn patch<'this>(&'this self) -> i32 {
            self.patch_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_patch<'this>(&'this self) -> bool {
            self.patch_opt().is_some()
        }
        fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn suffix<'this>(&'this self) -> &'this str {
            self.suffix_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_suffix<'this>(&'this self) -> bool {
            self.suffix_opt().is_some()
        }
        fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
    }

    macro_rules! version_delegate {
        ($ty:ty) => {
            fn major_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).major_opt()
            }
            fn minor_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).minor_opt()
            }
            fn patch_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).patch_opt()
            }
            fn suffix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).suffix_opt()
            }
        };
    }

    impl<T> VersionTrait for &'_ T
    where
        T: VersionTrait,
    {
        version_delegate!(T);
    }

    impl<T> VersionTrait for &'_ mut T
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
        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
        fn parameter<'this>(&'this self) -> &'this str {
            self.parameter_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_parameter<'this>(&'this self) -> bool {
            self.parameter_opt().is_some()
        }
        fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        type Field15MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field15MessageType<'this>>
        where
            Self: 'this;
        fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::compiler::_puroro_traits::VersionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn compiler_version<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.compiler_version_opt()
        }
        fn has_compiler_version<'this>(&'this self) -> bool {
            self.compiler_version_opt().is_some()
        }
        fn compiler_version_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! code_generator_request_delegate {
        ($ty:ty) => {
            type Field1RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field1RepeatedType<'this>;
            fn file_to_generate<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                (**self).file_to_generate()
            }
            fn parameter_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).parameter_opt()
            }
            type Field15MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field15MessageType<'this>;

            type Field15RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field15RepeatedType<'this>;
            fn proto_file<'this>(&'this self) -> Self::Field15RepeatedType<'this> {
                (**self).proto_file()
            }
            type Field3MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field3MessageType<'this>;
            fn compiler_version_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).compiler_version_opt()
            }
        };
    }

    impl<T> CodeGeneratorRequestTrait for &'_ T
    where
        T: CodeGeneratorRequestTrait,
    {
        code_generator_request_delegate!(T);
    }

    impl<T> CodeGeneratorRequestTrait for &'_ mut T
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
        fn error<'this>(&'this self) -> &'this str {
            self.error_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_error<'this>(&'this self) -> bool {
            self.error_opt().is_some()
        }
        fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn supported_features<'this>(&'this self) -> u64 {
            self.supported_features_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_supported_features<'this>(&'this self) -> bool {
            self.supported_features_opt().is_some()
        }
        fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }
        type Field15MessageType<'this>:
            self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response::_puroro_traits::FileTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field15RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field15MessageType<'this>>
        where
            Self: 'this;
        fn file<'this>(&'this self) -> Self::Field15RepeatedType<'this>;
    }

    macro_rules! code_generator_response_delegate {
        ($ty:ty) => {
            fn error_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).error_opt()
            }
            fn supported_features_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).supported_features_opt()
            }
            type Field15MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field15MessageType<'this>;

            type Field15RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field15RepeatedType<'this>;
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

    impl<T> CodeGeneratorResponseTrait for &'_ mut T
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

        pub use _puroro_simple_impl::File;
        pub mod _puroro_simple_impl {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct File {
            pub name: ::std::option::Option<::std::string::String>,
            pub insertion_point: ::std::option::Option<::std::string::String>,
            pub content: ::std::option::Option<::std::string::String>,
            pub generated_code_info: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo>>,
        }
            impl ::puroro::Message<File> for File {}

            impl super::_puroro_traits::FileTrait for File {
                fn name_opt<'this>(&'this self) -> Option<&'this str> {
                    self.name.as_ref().map(|v| v.as_ref())
                }
                fn insertion_point_opt<'this>(&'this self) -> Option<&'this str> {
                    self.insertion_point.as_ref().map(|v| v.as_ref())
                }
                fn content_opt<'this>(&'this self) -> Option<&'this str> {
                    self.content.as_ref().map(|v| v.as_ref())
                }
                type Field16MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo;
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> Option<Self::Field16MessageType<'this>> {
                    self.generated_code_info.as_ref().map(|v| v.as_ref())
                }
            }

            impl ::puroro::MessageRepresentativeImpl for File {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 4]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "name",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <File as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "insertion_point",
                                        number: 2,
                                        lazy_containing_type: Lazy::new(|| {
                                            <File as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "content",
                                        number: 15,
                                        lazy_containing_type: Lazy::new(|| {
                                            <File as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "generated_code_info",
                                        number: 16,
                                        lazy_containing_type: Lazy::new(|| {
                                            <File as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "File",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for File {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for File {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::internal::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                    1 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.name, data),
                    2 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.insertion_point, data),
                    15 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::String
                    >::deser_field(&mut self.content, data),
                    16 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo>>
                    >::deser_field(&mut self.generated_code_info, data),
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for File {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.name, 1, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.insertion_point, 2, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.content, 15, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::GeneratedCodeInfo>>
                >::ser_field(&self.generated_code_info, 16, out)?;

                    ::std::result::Result::Ok(())
                }
            }
        }

        pub use _puroro_impls::*;
        pub mod _puroro_impls {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }
            use super::_puroro_traits::*;
            impl FileTrait for () {
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = ();
            }
            impl<T, U> FileTrait for (T, U)
            where
                T: FileTrait,
                U: FileTrait,
            {
                fn name_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as FileTrait>::name_opt(&self.1)
                        .or_else(|| <T as FileTrait>::name_opt(&self.0))
                }
                fn insertion_point_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as FileTrait>::insertion_point_opt(&self.1)
                        .or_else(|| <T as FileTrait>::insertion_point_opt(&self.0))
                }
                fn content_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as FileTrait>::content_opt(&self.1)
                        .or_else(|| <T as FileTrait>::content_opt(&self.0))
                }
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = (
                    ::std::option::Option<<T as FileTrait>::Field16MessageType<'this>>,
                    ::std::option::Option<<U as FileTrait>::Field16MessageType<'this>>,
                );
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> Option<Self::Field16MessageType<'this>> {
                    match (
                        <T as FileTrait>::generated_code_info_opt(&self.0),
                        <U as FileTrait>::generated_code_info_opt(&self.1),
                    ) {
                        (None, None) => None,
                        (Some(t), None) => Some((Some(t), None)),
                        (None, Some(u)) => Some((None, Some(u))),
                        (Some(t), Some(u)) => Some((Some(t), Some(u))),
                    }
                }
            }
            impl<T, U> FileTrait for ::puroro::Either<T, U>
            where
                T: FileTrait,
                U: FileTrait,
            {
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::name_opt(t),
                        |u| <U as FileTrait>::name_opt(u),
                    )
                }
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::insertion_point_opt(t),
                        |u| <U as FileTrait>::insertion_point_opt(u),
                    )
                }
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as FileTrait>::content_opt(t),
                        |u| <U as FileTrait>::content_opt(u),
                    )
                }
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = ::puroro::Either<
                    <T as FileTrait>::Field16MessageType<'this>,
                    <U as FileTrait>::Field16MessageType<'this>,
                >;
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                    self.as_ref().either(
                        |t| {
                            <T as FileTrait>::generated_code_info_opt(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as FileTrait>::generated_code_info_opt(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
            }
            impl<T> FileTrait for ::std::option::Option<T>
            where
                T: FileTrait,
            {
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.name_opt())
                }
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.insertion_point_opt())
                }
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.content_opt())
                }
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = T::Field16MessageType<'this>;
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                    self.as_ref().and_then(|msg| msg.generated_code_info_opt())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub name: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField1<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.name.as_ref())
                }
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field::
                <ScalarType, _, _>
                (
                    ::std::iter::once(&self.name),
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self { name: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub insertion_point: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField2<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.insertion_point.as_ref())
                }
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field::
                <ScalarType, _, _>
                (
                    ::std::iter::once(&self.insertion_point),
                    2,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        insertion_point: value,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub content: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField15<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.content.as_ref())
                }
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field::
                <ScalarType, _, _>
                (
                    ::std::iter::once(&self.content),
                    15,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField15<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self { content: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
            {
                pub generated_code_info: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::File> for FileSingleField16<ScalarType> where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::FileTrait for FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
            {
                type Field16MessageType<'this>
                where
                    Self: 'this,
                = &'this ScalarType;

                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                    ::std::option::Option::Some(&self.generated_code_info)
                }
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite for FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                        ::puroro::tags::Optional,
                        ::puroro::tags::Message<ScalarType>,
                    >::ser_field::<ScalarType, _, _>(
                        ::std::iter::once(&self.generated_code_info),
                        16,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for FileSingleField16<ScalarType>
            where
                ScalarType:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        generated_code_info: value,
                    }
                }
            }
            pub struct FileBuilder<T>(T);

            impl<T> FileBuilder<T>
            where
                T: FileTrait,
            {
                pub fn append_name<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> FileBuilder<(T, FileSingleField1<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    FileBuilder((self.0, FileSingleField1 { name: value }))
                }

                pub fn append_insertion_point<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> FileBuilder<(T, FileSingleField2<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    FileBuilder((
                        self.0,
                        FileSingleField2 {
                            insertion_point: value,
                        },
                    ))
                }

                pub fn append_content<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> FileBuilder<(T, FileSingleField15<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    FileBuilder((self.0, FileSingleField15 { content: value }))
                }
        
            pub fn append_generated_code_info<ScalarType>(self, value: ScalarType)
                -> FileBuilder<(T, FileSingleField16<ScalarType>)>
        where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
            {
                    FileBuilder((
                        self.0,
                        FileSingleField16 {
                            generated_code_info: value,
                        },
                    ))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl FileBuilder<()> {
                pub fn new() -> Self {
                    Self(())
                }
            }
        }
        pub use _puroro_traits::*;
        pub mod _puroro_traits {
            mod _puroro_root {
                pub use super::super::_puroro_root::*;
            }

            pub trait FileTrait {
                fn name<'this>(&'this self) -> &'this str {
                    self.name_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_name<'this>(&'this self) -> bool {
                    self.name_opt().is_some()
                }
                fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                fn insertion_point<'this>(&'this self) -> &'this str {
                    self.insertion_point_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_insertion_point<'this>(&'this self) -> bool {
                    self.insertion_point_opt().is_some()
                }
                fn insertion_point_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                fn content<'this>(&'this self) -> &'this str {
                    self.content_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_content<'this>(&'this self) -> bool {
                    self.content_opt().is_some()
                }
                fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                type Field16MessageType<'this>:
                    self::_puroro_root::google::protobuf::_puroro_traits::GeneratedCodeInfoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
                    where Self: 'this;
                fn generated_code_info<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                    self.generated_code_info_opt()
                }
                fn has_generated_code_info<'this>(&'this self) -> bool {
                    self.generated_code_info_opt().is_some()
                }
                fn generated_code_info_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                    ::std::option::Option::None
                }
            }

            macro_rules! file_delegate {
                ($ty:ty) => {
                    fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                        (**self).name_opt()
                    }
                    fn insertion_point_opt<'this>(
                        &'this self,
                    ) -> ::std::option::Option<&'this str> {
                        (**self).insertion_point_opt()
                    }
                    fn content_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                        (**self).content_opt()
                    }
                    type Field16MessageType<'this>
                    where
                        Self: 'this,
                    = <$ty>::Field16MessageType<'this>;
                    fn generated_code_info_opt<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field16MessageType<'this>> {
                        (**self).generated_code_info_opt()
                    }
                };
            }

            impl<T> FileTrait for &'_ T
            where
                T: FileTrait,
            {
                file_delegate!(T);
            }

            impl<T> FileTrait for &'_ mut T
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
