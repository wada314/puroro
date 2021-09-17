// A generated source code by puroro library
// package google.protobuf
pub mod compiler;

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub use _puroro_simple_impl::DescriptorProto;
pub use _puroro_simple_impl::EnumDescriptorProto;
pub use _puroro_simple_impl::EnumOptions;
pub use _puroro_simple_impl::EnumValueDescriptorProto;
pub use _puroro_simple_impl::EnumValueOptions;
pub use _puroro_simple_impl::ExtensionRangeOptions;
pub use _puroro_simple_impl::FieldDescriptorProto;
pub use _puroro_simple_impl::FieldOptions;
pub use _puroro_simple_impl::FileDescriptorProto;
pub use _puroro_simple_impl::FileDescriptorSet;
pub use _puroro_simple_impl::FileOptions;
pub use _puroro_simple_impl::GeneratedCodeInfo;
pub use _puroro_simple_impl::MessageOptions;
pub use _puroro_simple_impl::MethodDescriptorProto;
pub use _puroro_simple_impl::MethodOptions;
pub use _puroro_simple_impl::OneofDescriptorProto;
pub use _puroro_simple_impl::OneofOptions;
pub use _puroro_simple_impl::ServiceDescriptorProto;
pub use _puroro_simple_impl::ServiceOptions;
pub use _puroro_simple_impl::SourceCodeInfo;
pub use _puroro_simple_impl::UninterpretedOption;
pub mod _puroro_simple_impl {
    mod _puroro_root {
        pub use super::super::_puroro_root::*;
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileDescriptorSet {
        pub file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
        >,
    }
    impl ::puroro::Message<FileDescriptorSet> for FileDescriptorSet {}

    impl super::_puroro_traits::FileDescriptorSetTrait for FileDescriptorSet {
        type Field1MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto;
        type Field1RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            >,
        >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.file.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for FileDescriptorSet {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "file",
                            number: 1,
                            lazy_containing_type: Lazy::new(|| {
                                <FileDescriptorSet as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "FileDescriptorSet",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for FileDescriptorSet {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for FileDescriptorSet {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto>
            >::deser_field(&mut self.file, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorSet {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
                >,
            >::ser_field(&self.file, 1, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileDescriptorProto {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub dependency: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        pub public_dependency: ::std::vec::Vec<i32>,
        pub weak_dependency: ::std::vec::Vec<i32>,
        pub message_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
        >,
        pub enum_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
        >,
        pub service: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
        >,
        pub extension: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
        >,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
            >,
        >,
        pub source_code_info: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
            >,
        >,
        pub syntax: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }
    impl ::puroro::Message<FileDescriptorProto> for FileDescriptorProto {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProto {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'this str;
        fn package<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.package.as_ref().map(|v| v.as_ref())
        }
        type Field3StringType<'this> = &'this str;
        type Field3RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.dependency.iter())
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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto;
        type Field4RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.message_type.iter())
        }
        type Field5MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field6MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
            >,
        >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.service.iter())
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
        type Field7RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo;
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            self.source_code_info.as_ref().map(|v| v.as_ref())
        }
        type Field12StringType<'this> = &'this str;
        fn syntax<'this>(&'this self) -> Option<Self::Field12StringType<'this>> {
            self.syntax.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for FileDescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 12]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "package",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "dependency",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "public_dependency",
                                number: 10,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "weak_dependency",
                                number: 11,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "message_type",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_type",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "service",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "extension",
                                number: 7,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 8,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "source_code_info",
                                number: 9,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "syntax",
                                number: 12,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "FileDescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for FileDescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for FileDescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
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
            >::deser_field(&mut self.package, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.dependency, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.public_dependency, data),
            11 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Int32
            >::deser_field(&mut self.weak_dependency, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto>
            >::deser_field(&mut self.message_type, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto>
            >::deser_field(&mut self.enum_type, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto>
            >::deser_field(&mut self.service, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto>
            >::deser_field(&mut self.extension, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions>
            >::deser_field(&mut self.options, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo>
            >::deser_field(&mut self.source_code_info, data),
            12 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.syntax, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.package,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.dependency,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.public_dependency,
                10,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.weak_dependency,
                11,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
                >,
            >::ser_field(&self.message_type, 4, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
                >,
            >::ser_field(&self.enum_type, 5, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto>
        >::ser_field(&self.service, 6, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
                >,
            >::ser_field(&self.extension, 7, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
                >,
            >::ser_field(&self.options, 8, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
                >,
            >::ser_field(&self.source_code_info, 9, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.syntax,
                12,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct DescriptorProto {
    pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    pub field: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto>,
    pub extension: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto>,
    pub extension_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions>>,
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
}
    impl ::puroro::Message<DescriptorProto> for DescriptorProto {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProto {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.field.iter())
        }
        type Field6MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto;
        type Field3RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.nested_type.iter())
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto;
        type Field4RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field5MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>>;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.extension_range.iter())
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto;
        type Field8RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
            >,
        >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.oneof_decl.iter())
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions;
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange;
        type Field9RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>>;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field10StringType<'this> = &'this str;
        type Field10RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for DescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 10]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "field",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "extension",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "nested_type",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "enum_type",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "extension_range",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "oneof_decl",
                                number: 8,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 7,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "reserved_range",
                                number: 9,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "reserved_name",
                                number: 10,
                                lazy_containing_type: Lazy::new(|| {
                                    <DescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "DescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for DescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for DescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto>
            >::deser_field(&mut self.field, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto>
            >::deser_field(&mut self.extension, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto>
            >::deser_field(&mut self.nested_type, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto>
            >::deser_field(&mut self.enum_type, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>
            >::deser_field(&mut self.extension_range, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto>
            >::deser_field(&mut self.oneof_decl, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions>
            >::deser_field(&mut self.options, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>
            >::deser_field(&mut self.reserved_range, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.reserved_name, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
                >,
            >::ser_field(&self.field, 2, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
                >,
            >::ser_field(&self.extension, 6, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
                >,
            >::ser_field(&self.nested_type, 3, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
                >,
            >::ser_field(&self.enum_type, 4, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>
        >::ser_field(&self.extension_range, 5, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
                >,
            >::ser_field(&self.oneof_decl, 8, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
                >,
            >::ser_field(&self.options, 7, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>
        >::ser_field(&self.reserved_range, 9, out)?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.reserved_name,
                10,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ExtensionRangeOptions {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<ExtensionRangeOptions> for ExtensionRangeOptions {}

    impl super::_puroro_traits::ExtensionRangeOptionsTrait for ExtensionRangeOptions {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for ExtensionRangeOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "uninterpreted_option",
                            number: 999,
                            lazy_containing_type: Lazy::new(|| {
                                <ExtensionRangeOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "ExtensionRangeOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for ExtensionRangeOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for ExtensionRangeOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for ExtensionRangeOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FieldDescriptorProto {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub number: ::std::option::Option<i32>,
        pub label: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        >,
        pub type_: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        >,
        pub type_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub extendee: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub default_value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub oneof_index: ::std::option::Option<i32>,
        pub json_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
            >,
        >,
        pub proto3_optional: ::std::option::Option<bool>,
    }
    impl ::puroro::Message<FieldDescriptorProto> for FieldDescriptorProto {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProto {
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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        fn proto3_optional<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.proto3_optional)
        }
    }

    impl ::puroro::MessageRepresentativeImpl for FieldDescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 11]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "number",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "label",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "type",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "type_name",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "extendee",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "default_value",
                                number: 7,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "oneof_index",
                                number: 9,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "json_name",
                                number: 10,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 8,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "proto3_optional",
                                number: 17,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "FieldDescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for FieldDescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for FieldDescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.name, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.number, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label>
            >::deser_field(&mut self.label, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type>
            >::deser_field(&mut self.type_, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.type_name, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.extendee, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.default_value, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.oneof_index, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.json_name, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions>
            >::deser_field(&mut self.options, data),
            17 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.proto3_optional, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.number,
                3,
                out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label>
        >::ser_field(&self.label, 4, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type>
        >::ser_field(&self.type_, 5, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.type_name,
                6,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.extendee,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.default_value,
                7,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.oneof_index,
                9,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.json_name,
                10,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
                >,
            >::ser_field(&self.options, 8, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.proto3_optional,
                17,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct OneofDescriptorProto {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
            >,
        >,
    }
    impl ::puroro::Message<OneofDescriptorProto> for OneofDescriptorProto {}

    impl super::_puroro_traits::OneofDescriptorProtoTrait for OneofDescriptorProto {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions;
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for OneofDescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 2]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <OneofDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <OneofDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "OneofDescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for OneofDescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for OneofDescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
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
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions>
            >::deser_field(&mut self.options, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for OneofDescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
                >,
            >::ser_field(&self.options, 2, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumDescriptorProto {
    pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    pub value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions>>,
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
}
    impl ::puroro::Message<EnumDescriptorProto> for EnumDescriptorProto {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProto {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
            >,
        >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.value.iter())
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange;
        type Field4RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>>;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field5StringType<'this> = &'this str;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for EnumDescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 5]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "value",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "reserved_range",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "reserved_name",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "EnumDescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for EnumDescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for EnumDescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto>
            >::deser_field(&mut self.value, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions>
            >::deser_field(&mut self.options, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>
            >::deser_field(&mut self.reserved_range, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::String
            >::deser_field(&mut self.reserved_name, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumDescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto>
        >::ser_field(&self.value, 2, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
                >,
            >::ser_field(&self.options, 3, out)?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>
        >::ser_field(&self.reserved_range, 4, out)?;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.reserved_name,
                5,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumValueDescriptorProto {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub number: ::std::option::Option<i32>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
            >,
        >,
    }
    impl ::puroro::Message<EnumValueDescriptorProto> for EnumValueDescriptorProto {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProto {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        fn number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for EnumValueDescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 3]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumValueDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "number",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumValueDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumValueDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "EnumValueDescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for EnumValueDescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for EnumValueDescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.number, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions>
            >::deser_field(&mut self.options, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumValueDescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.number,
                2,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
                >,
            >::ser_field(&self.options, 3, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ServiceDescriptorProto {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub method: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
        >,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
            >,
        >,
    }
    impl ::puroro::Message<ServiceDescriptorProto> for ServiceDescriptorProto {}

    impl super::_puroro_traits::ServiceDescriptorProtoTrait for ServiceDescriptorProto {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
            >,
        >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.method.iter())
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for ServiceDescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 3]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <ServiceDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "method",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <ServiceDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <ServiceDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "ServiceDescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for ServiceDescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for ServiceDescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto>
            >::deser_field(&mut self.method, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions>
            >::deser_field(&mut self.options, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for ServiceDescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto>
        >::ser_field(&self.method, 2, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
                >,
            >::ser_field(&self.options, 3, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MethodDescriptorProto {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub input_type: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub output_type: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
            >,
        >,
        pub client_streaming: ::std::option::Option<bool>,
        pub server_streaming: ::std::option::Option<bool>,
    }
    impl ::puroro::Message<MethodDescriptorProto> for MethodDescriptorProto {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProto {
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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions;
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

    impl ::puroro::MessageRepresentativeImpl for MethodDescriptorProto {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 6]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "input_type",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "output_type",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "options",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "client_streaming",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "server_streaming",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodDescriptorProto as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "MethodDescriptorProto",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for MethodDescriptorProto {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for MethodDescriptorProto {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
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
            >::deser_field(&mut self.input_type, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.output_type, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions>
            >::deser_field(&mut self.options, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.client_streaming, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.server_streaming, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodDescriptorProto {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.input_type,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.output_type,
                3,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
                >,
            >::ser_field(&self.options, 4, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.client_streaming,
                5,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.server_streaming,
                6,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileOptions {
        pub java_package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub java_outer_classname: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub java_multiple_files: ::std::option::Option<bool>,
        pub java_generate_equals_and_hash: ::std::option::Option<bool>,
        pub java_string_check_utf8: ::std::option::Option<bool>,
        pub optimize_for: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        >,
        pub go_package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub cc_generic_services: ::std::option::Option<bool>,
        pub java_generic_services: ::std::option::Option<bool>,
        pub py_generic_services: ::std::option::Option<bool>,
        pub php_generic_services: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub cc_enable_arenas: ::std::option::Option<bool>,
        pub objc_class_prefix: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub csharp_namespace: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub swift_prefix: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub php_class_prefix: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub php_namespace: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub php_metadata_namespace: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub ruby_package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<FileOptions> for FileOptions {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptions {
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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for FileOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 21]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "java_package",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "java_outer_classname",
                                number: 8,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "java_multiple_files",
                                number: 10,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "java_generate_equals_and_hash",
                                number: 20,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "java_string_check_utf8",
                                number: 27,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "optimize_for",
                                number: 9,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "go_package",
                                number: 11,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "cc_generic_services",
                                number: 16,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "java_generic_services",
                                number: 17,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "py_generic_services",
                                number: 18,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "php_generic_services",
                                number: 42,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "deprecated",
                                number: 23,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "cc_enable_arenas",
                                number: 31,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "objc_class_prefix",
                                number: 36,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "csharp_namespace",
                                number: 37,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "swift_prefix",
                                number: 39,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "php_class_prefix",
                                number: 40,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "php_namespace",
                                number: 41,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "php_metadata_namespace",
                                number: 44,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "ruby_package",
                                number: 45,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "uninterpreted_option",
                                number: 999,
                                lazy_containing_type: Lazy::new(|| {
                                    <FileOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "FileOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for FileOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for FileOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.java_package, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.java_outer_classname, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_multiple_files, data),
            20 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_generate_equals_and_hash, data),
            27 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_string_check_utf8, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
            >::deser_field(&mut self.optimize_for, data),
            11 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.go_package, data),
            16 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.cc_generic_services, data),
            17 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.java_generic_services, data),
            18 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.py_generic_services, data),
            42 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.php_generic_services, data),
            23 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            31 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.cc_enable_arenas, data),
            36 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.objc_class_prefix, data),
            37 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.csharp_namespace, data),
            39 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.swift_prefix, data),
            40 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.php_class_prefix, data),
            41 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.php_namespace, data),
            44 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.php_metadata_namespace, data),
            45 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.ruby_package, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.java_package,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.java_outer_classname,
                8,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_multiple_files,
                10,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_generate_equals_and_hash,
                20,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_string_check_utf8,
                27,
                out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        >::ser_field(&self.optimize_for, 9, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.go_package,
                11,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.cc_generic_services,
                16,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_generic_services,
                17,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.py_generic_services,
                18,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.php_generic_services,
                42,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                23,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.cc_enable_arenas,
                31,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.objc_class_prefix,
                36,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.csharp_namespace,
                37,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.swift_prefix,
                39,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.php_class_prefix,
                40,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.php_namespace,
                41,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.php_metadata_namespace,
                44,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.ruby_package,
                45,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MessageOptions {
        pub message_set_wire_format: ::std::option::Option<bool>,
        pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub map_entry: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<MessageOptions> for MessageOptions {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptions {
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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for MessageOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 5]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "message_set_wire_format",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <MessageOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "no_standard_descriptor_accessor",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <MessageOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "deprecated",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <MessageOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "map_entry",
                                number: 7,
                                lazy_containing_type: Lazy::new(|| {
                                    <MessageOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "uninterpreted_option",
                                number: 999,
                                lazy_containing_type: Lazy::new(|| {
                                    <MessageOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "MessageOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for MessageOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for MessageOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.message_set_wire_format, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.no_standard_descriptor_accessor, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.map_entry, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MessageOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.message_set_wire_format,
                1,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.no_standard_descriptor_accessor,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.map_entry,
                7,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FieldOptions {
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
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<FieldOptions> for FieldOptions {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptions {
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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for FieldOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 7]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "ctype",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "packed",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "jstype",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "lazy",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "deprecated",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "weak",
                                number: 10,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "uninterpreted_option",
                                number: 999,
                                lazy_containing_type: Lazy::new(|| {
                                    <FieldOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "FieldOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for FieldOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for FieldOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
            >::deser_field(&mut self.ctype, data),
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.packed, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
            >::deser_field(&mut self.jstype, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.lazy, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            10 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.weak, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
                >,
            >::ser_field(&self.ctype, 1, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.packed,
                2,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
                >,
            >::ser_field(&self.jstype, 6, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.lazy, 5, out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.weak, 10, out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct OneofOptions {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<OneofOptions> for OneofOptions {}

    impl super::_puroro_traits::OneofOptionsTrait for OneofOptions {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for OneofOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "uninterpreted_option",
                            number: 999,
                            lazy_containing_type: Lazy::new(|| {
                                <OneofOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "OneofOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for OneofOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for OneofOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for OneofOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumOptions {
        pub allow_alias: ::std::option::Option<bool>,
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<EnumOptions> for EnumOptions {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptions {
        fn allow_alias<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.allow_alias)
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for EnumOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 3]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "allow_alias",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "deprecated",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "uninterpreted_option",
                                number: 999,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumOptions as ::puroro::MessageRepresentativeImpl>::descriptor(
                                    )
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "EnumOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for EnumOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for EnumOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.allow_alias, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.allow_alias,
                2,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                3,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumValueOptions {
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<EnumValueOptions> for EnumValueOptions {}

    impl super::_puroro_traits::EnumValueOptionsTrait for EnumValueOptions {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for EnumValueOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 2]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "deprecated",
                                number: 1,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumValueOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "uninterpreted_option",
                                number: 999,
                                lazy_containing_type: Lazy::new(|| {
                                    <EnumValueOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "EnumValueOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for EnumValueOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for EnumValueOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumValueOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                1,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ServiceOptions {
        pub deprecated: ::std::option::Option<bool>,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<ServiceOptions> for ServiceOptions {}

    impl super::_puroro_traits::ServiceOptionsTrait for ServiceOptions {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for ServiceOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 2]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "deprecated",
                                number: 33,
                                lazy_containing_type: Lazy::new(|| {
                                    <ServiceOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "uninterpreted_option",
                                number: 999,
                                lazy_containing_type: Lazy::new(|| {
                                    <ServiceOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "ServiceOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for ServiceOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for ServiceOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            33 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for ServiceOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                33,
                out,
            )?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MethodOptions {
        pub deprecated: ::std::option::Option<bool>,
        pub idempotency_level: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        >,
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<MethodOptions> for MethodOptions {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptions {
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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for MethodOptions {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 3]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "deprecated",
                                number: 33,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "idempotency_level",
                                number: 34,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "uninterpreted_option",
                                number: 999,
                                lazy_containing_type: Lazy::new(|| {
                                    <MethodOptions as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "MethodOptions",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for MethodOptions {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for MethodOptions {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            33 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            34 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>
            >::deser_field(&mut self.idempotency_level, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodOptions {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                33,
                out,
            )?;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>
        >::ser_field(&self.idempotency_level, 34, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct UninterpretedOption {
    pub name: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>,
    pub identifier_value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    pub positive_int_value: ::std::option::Option<u64>,
    pub negative_int_value: ::std::option::Option<i64>,
    pub double_value: ::std::option::Option<f64>,
    pub string_value: ::std::option::Option<::std::borrow::Cow<'static, [u8]>>,
    pub aggregate_value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
}
    impl ::puroro::Message<UninterpretedOption> for UninterpretedOption {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOption {
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>>;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.name.iter())
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

    impl ::puroro::MessageRepresentativeImpl for UninterpretedOption {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 7]> =
                Lazy::new(|| {
                    [
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "name",
                                number: 2,
                                lazy_containing_type: Lazy::new(|| {
                                    <UninterpretedOption as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "identifier_value",
                                number: 3,
                                lazy_containing_type: Lazy::new(|| {
                                    <UninterpretedOption as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "positive_int_value",
                                number: 4,
                                lazy_containing_type: Lazy::new(|| {
                                    <UninterpretedOption as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "negative_int_value",
                                number: 5,
                                lazy_containing_type: Lazy::new(|| {
                                    <UninterpretedOption as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "double_value",
                                number: 6,
                                lazy_containing_type: Lazy::new(|| {
                                    <UninterpretedOption as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "string_value",
                                number: 7,
                                lazy_containing_type: Lazy::new(|| {
                                    <UninterpretedOption as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                        {
                            let init = ::puroro::internal::FieldDescriptorInitializer {
                                name: "aggregate_value",
                                number: 8,
                                lazy_containing_type: Lazy::new(|| {
                                    <UninterpretedOption as ::puroro::MessageRepresentativeImpl>::descriptor()
                                }),
                            };
                            ::puroro::internal::init_field_descriptor(init)
                        },
                    ]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "UninterpretedOption",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for UninterpretedOption {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for UninterpretedOption {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            2 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>
            >::deser_field(&mut self.name, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.identifier_value, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::UInt64
            >::deser_field(&mut self.positive_int_value, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int64
            >::deser_field(&mut self.negative_int_value, data),
            6 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Double
            >::deser_field(&mut self.double_value, data),
            7 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bytes
            >::deser_field(&mut self.string_value, data),
            8 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.aggregate_value, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOption {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>
        >::ser_field(&self.name, 2, out)?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.identifier_value,
                3,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt64>::ser_field(
                &self.positive_int_value,
                4,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int64>::ser_field(
                &self.negative_int_value,
                5,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Double>::ser_field(
                &self.double_value,
                6,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bytes>::ser_field(
                &self.string_value,
                7,
                out,
            )?;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.aggregate_value,
                8,
                out,
            )?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SourceCodeInfo {
    pub location: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>,
}
    impl ::puroro::Message<SourceCodeInfo> for SourceCodeInfo {}

    impl super::_puroro_traits::SourceCodeInfoTrait for SourceCodeInfo {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location;
        type Field1RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>>;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.location.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for SourceCodeInfo {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "location",
                            number: 1,
                            lazy_containing_type: Lazy::new(|| {
                                <SourceCodeInfo as ::puroro::MessageRepresentativeImpl>::descriptor(
                                )
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "SourceCodeInfo",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for SourceCodeInfo {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for SourceCodeInfo {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>
            >::deser_field(&mut self.location, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for SourceCodeInfo {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>
        >::ser_field(&self.location, 1, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct GeneratedCodeInfo {
    pub annotation: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>,
}
    impl ::puroro::Message<GeneratedCodeInfo> for GeneratedCodeInfo {}

    impl super::_puroro_traits::GeneratedCodeInfoTrait for GeneratedCodeInfo {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation;
        type Field1RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>>;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.annotation.iter())
        }
    }

    impl ::puroro::MessageRepresentativeImpl for GeneratedCodeInfo {
        fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
            use ::puroro::once_cell::sync::Lazy;
            static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 1]> =
                Lazy::new(|| {
                    [{
                        let init = ::puroro::internal::FieldDescriptorInitializer {
                            name: "annotation",
                            number: 1,
                            lazy_containing_type: Lazy::new(|| {
                                <GeneratedCodeInfo as ::puroro::MessageRepresentativeImpl>::descriptor()
                            }),
                        };
                        ::puroro::internal::init_field_descriptor(init)
                    }]
                });
            static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> = Lazy::new(|| {
                let init = ::puroro::internal::MessageDescriptorInitializer {
                    name: "GeneratedCodeInfo",
                    lazy_fields: Lazy::new(|| Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()),
                };
                ::puroro::internal::init_message_descriptor(init)
            });
            Lazy::force(&LAZY_DESCRIPTOR)
        }
    }

    impl ::puroro::DeserializableMessageFromBytesIterator for GeneratedCodeInfo {
        fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
        }
    }

    impl ::puroro::internal::de::DeserFieldsFromBytesIter for GeneratedCodeInfo {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
        ) -> ::puroro::Result<()>
        where
            I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
            match field_number {
            1 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>
            >::deser_field(&mut self.annotation, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for GeneratedCodeInfo {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>
        >::ser_field(&self.annotation, 1, out)?;

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
    impl FileDescriptorSetTrait for () {
        type Field1MessageType<'this> = ();
        type Field1RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
                <U as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
            >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
                <U as FileDescriptorSetTrait>::Field1RepeatedType<'this>,
            >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorSetTrait>::file(t))
                    .map_right(|u| <U as FileDescriptorSetTrait>::file(u)),
            )
        }
    }
    impl<T> FileDescriptorSetTrait for ::std::option::Option<T>
    where
        T: FileDescriptorSetTrait,
    {
        type Field1MessageType<'this> = T::Field1MessageType<'this>;
        type Field1RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field1RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.file().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorSetSimpleField1 {
        pub file: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::FileDescriptorSet> for FileDescriptorSetSimpleField1 {}

    impl super::_puroro_traits::FileDescriptorSetTrait for FileDescriptorSetSimpleField1 {
        type Field1MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto;
        type Field1RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            >,
        >;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.file.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorSetSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
                >,
            >::ser_field(&self.file, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            >,
        > for FileDescriptorSetSimpleField1
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            >,
        ) -> Self {
            Self { file: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileDescriptorSetSimpleByValue {}
    impl ::puroro::Message<super::FileDescriptorSet> for FileDescriptorSetSimpleByValue {}

    impl FileDescriptorSetTrait for FileDescriptorSetSimpleByValue {
        type Field1MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
        >;
        type Field1RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct FileDescriptorSetBuilder<T>(T);

    impl<T> FileDescriptorSetBuilder<T>
    where
        T: FileDescriptorSetTrait,
    {
        pub fn append_file(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto,
            >,
        ) -> FileDescriptorSetBuilder<(T, FileDescriptorSetSimpleField1)> {
            FileDescriptorSetBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl FileDescriptorSetBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl FileDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
        type Field3RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
                <T as FileDescriptorProtoTrait>::dependency(&self.0),
                <U as FileDescriptorProtoTrait>::dependency(&self.1),
            )
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as FileDescriptorProtoTrait>::public_dependency(&self.0),
                <U as FileDescriptorProtoTrait>::public_dependency(&self.1),
            )
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
        >;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as FileDescriptorProtoTrait>::weak_dependency(&self.0),
                <U as FileDescriptorProtoTrait>::weak_dependency(&self.1),
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::message_type(&self.0),
                <U as FileDescriptorProtoTrait>::message_type(&self.1),
            )
        }
        type Field5MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field5MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::enum_type(&self.0),
                <U as FileDescriptorProtoTrait>::enum_type(&self.1),
            )
        }
        type Field6MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field6MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::service(&self.0),
                <U as FileDescriptorProtoTrait>::service(&self.1),
            )
        }
        type Field7MessageType<'this> = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field7MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field7MessageType<'this>,
        >;
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::extension(&self.0),
                <U as FileDescriptorProtoTrait>::extension(&self.1),
            )
        }
        type Field8MessageType<'this> = (
            ::std::option::Option<<T as FileDescriptorProtoTrait>::Field8MessageType<'this>>,
            ::std::option::Option<<U as FileDescriptorProtoTrait>::Field8MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            match (
                <T as FileDescriptorProtoTrait>::options(&self.0),
                <U as FileDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field9MessageType<'this> = (
            ::std::option::Option<<T as FileDescriptorProtoTrait>::Field9MessageType<'this>>,
            ::std::option::Option<<U as FileDescriptorProtoTrait>::Field9MessageType<'this>>,
        );
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            match (
                <T as FileDescriptorProtoTrait>::source_code_info(&self.0),
                <U as FileDescriptorProtoTrait>::source_code_info(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
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
        type Field3RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedLDField<
            <T as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::dependency(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::dependency(u)),
            )
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::public_dependency(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::public_dependency(u)),
            )
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
        >;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
                <U as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
    impl<T> FileDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: FileDescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        type Field2StringType<'this> = T::Field2StringType<'this>;
        fn package<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            self.as_ref().and_then(|msg| msg.package())
        }
        type Field3StringType<'this> = T::Field3StringType<'this>;
        type Field3RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field3RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.dependency().into_iter())
                .into_iter()
                .flatten()
        }
        type Field10RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field10RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.public_dependency().into_iter())
                .into_iter()
                .flatten()
        }
        type Field11RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field11RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.weak_dependency().into_iter())
                .into_iter()
                .flatten()
        }
        type Field4MessageType<'this> = T::Field4MessageType<'this>;
        type Field4RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field4RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.message_type().into_iter())
                .into_iter()
                .flatten()
        }
        type Field5MessageType<'this> = T::Field5MessageType<'this>;
        type Field5RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field5RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.enum_type().into_iter())
                .into_iter()
                .flatten()
        }
        type Field6MessageType<'this> = T::Field6MessageType<'this>;
        type Field6RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field6RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.service().into_iter())
                .into_iter()
                .flatten()
        }
        type Field7MessageType<'this> = T::Field7MessageType<'this>;
        type Field7RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field7RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.extension().into_iter())
                .into_iter()
                .flatten()
        }
        type Field8MessageType<'this> = T::Field8MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
        type Field9MessageType<'this> = T::Field9MessageType<'this>;
        fn source_code_info<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.source_code_info())
        }
        type Field12StringType<'this> = T::Field12StringType<'this>;
        fn syntax<'this>(&'this self) -> ::std::option::Option<Self::Field12StringType<'this>> {
            self.as_ref().and_then(|msg| msg.syntax())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileDescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField2 {
        pub package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'this str;
        fn package<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.package.as_ref().map(|v| v.as_ref())
        }
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.package,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileDescriptorProtoSimpleField2
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { package: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField3 {
        pub dependency: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'this str;
        type Field3RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.dependency.iter())
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.dependency,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::borrow::Cow<'static, str>>>
        for FileDescriptorProtoSimpleField3
    {
        fn from(value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>) -> Self {
            Self { dependency: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField10 {
        pub public_dependency: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField10 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField10 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.public_dependency.iter().cloned()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.public_dependency,
                10,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<i32>> for FileDescriptorProtoSimpleField10 {
        fn from(value: ::std::vec::Vec<i32>) -> Self {
            Self {
                public_dependency: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField11 {
        pub weak_dependency: ::std::vec::Vec<i32>,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField11 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField11 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            self.weak_dependency.iter().cloned()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField11 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field(
                &self.weak_dependency,
                11,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<i32>> for FileDescriptorProtoSimpleField11 {
        fn from(value: ::std::vec::Vec<i32>) -> Self {
            Self {
                weak_dependency: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField4 {
        pub message_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
        >,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto;
        type Field4RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.message_type.iter())
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
                >,
            >::ser_field(&self.message_type, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        > for FileDescriptorProtoSimpleField4
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        ) -> Self {
            Self {
                message_type: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField5 {
        pub enum_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
                >,
            >::ser_field(&self.enum_type, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        > for FileDescriptorProtoSimpleField5
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        ) -> Self {
            Self { enum_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField6 {
        pub service: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField6 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField6 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
            >,
        >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.service.iter())
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto>
        >::ser_field(&self.service, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
            >,
        > for FileDescriptorProtoSimpleField6
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
            >,
        ) -> Self {
            Self { service: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField7 {
        pub extension: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField7 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField7 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
        type Field7RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField7 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
                >,
            >::ser_field(&self.extension, 7, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        > for FileDescriptorProtoSimpleField7
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        ) -> Self {
            Self { extension: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField8 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField8 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField8 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField8 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
                >,
            >::ser_field(&self.options, 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
                >,
            >,
        > for FileDescriptorProtoSimpleField8
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField9 {
        pub source_code_info: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
            >,
        >,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField9 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField9 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo;
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            self.source_code_info.as_ref().map(|v| v.as_ref())
        }
        type Field12StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
                >,
            >::ser_field(&self.source_code_info, 9, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
                >,
            >,
        > for FileDescriptorProtoSimpleField9
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
                >,
            >,
        ) -> Self {
            Self {
                source_code_info: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileDescriptorProtoSimpleField12 {
        pub syntax: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleField12 {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProtoSimpleField12 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field12StringType<'this> = &'this str;
        fn syntax<'this>(&'this self) -> Option<Self::Field12StringType<'this>> {
            self.syntax.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileDescriptorProtoSimpleField12 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.syntax,
                12,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileDescriptorProtoSimpleField12
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { syntax: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileDescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::FileDescriptorProto> for FileDescriptorProtoSimpleByValue {}

    impl FileDescriptorProtoTrait for FileDescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn package<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field11RepeatedType<'this> = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field4MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
        >;
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field5MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
        >;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field6MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
        >;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field7MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
        >;
        type Field7RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field8MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field9MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
        >;
        fn source_code_info<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field12StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn syntax<'this>(&'this self) -> Option<Self::Field12StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct FileDescriptorProtoBuilder<T>(T);

    impl<T> FileDescriptorProtoBuilder<T>
    where
        T: FileDescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField1)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_package(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField2)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_dependency(
            self,
            value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField3)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_public_dependency(
            self,
            value: ::std::vec::Vec<i32>,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField10)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_weak_dependency(
            self,
            value: ::std::vec::Vec<i32>,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField11)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_message_type(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField4)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_enum_type(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField5)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_service(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto,
            >,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField6)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_extension(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField7)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
                >,
            >,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField8)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_source_code_info(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
                >,
            >,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField9)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_syntax(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSimpleField12)> {
            FileDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl FileDescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl DescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::field(&self.0),
                <U as DescriptorProtoTrait>::field(&self.1),
            )
        }
        type Field6MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field6MessageType<'this>,
            <U as DescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::extension(&self.0),
                <U as DescriptorProtoTrait>::extension(&self.1),
            )
        }
        type Field3MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field3MessageType<'this>,
            <U as DescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field3RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field3RepeatedType<'this>,
            >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::nested_type(&self.0),
                <U as DescriptorProtoTrait>::nested_type(&self.1),
            )
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field4MessageType<'this>,
            <U as DescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::enum_type(&self.0),
                <U as DescriptorProtoTrait>::enum_type(&self.1),
            )
        }
        type Field5MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field5MessageType<'this>,
            <U as DescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::extension_range(&self.0),
                <U as DescriptorProtoTrait>::extension_range(&self.1),
            )
        }
        type Field8MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field8MessageType<'this>,
            <U as DescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field8RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field8RepeatedType<'this>,
            >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::oneof_decl(&self.0),
                <U as DescriptorProtoTrait>::oneof_decl(&self.1),
            )
        }
        type Field7MessageType<'this> = (
            ::std::option::Option<<T as DescriptorProtoTrait>::Field7MessageType<'this>>,
            ::std::option::Option<<U as DescriptorProtoTrait>::Field7MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            match (
                <T as DescriptorProtoTrait>::options(&self.0),
                <U as DescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field9MessageType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field9MessageType<'this>,
            <U as DescriptorProtoTrait>::Field9MessageType<'this>,
        >;
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field9RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field9RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::reserved_range(&self.0),
                <U as DescriptorProtoTrait>::reserved_range(&self.1),
            )
        }
        type Field10StringType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field10StringType<'this>,
            <U as DescriptorProtoTrait>::Field10StringType<'this>,
        >;
        type Field10RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as DescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field6RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field6RepeatedType<'this>,
            >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field3RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field3RepeatedType<'this>,
            >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field5RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field5RepeatedType<'this>,
            >;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field8RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field8RepeatedType<'this>,
            >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as DescriptorProtoTrait>::Field9RepeatedType<'this>,
                <U as DescriptorProtoTrait>::Field9RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::reserved_range(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::reserved_range(u)),
            )
        }
        type Field10StringType<'this> = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field10StringType<'this>,
            <U as DescriptorProtoTrait>::Field10StringType<'this>,
        >;
        type Field10RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedLDField<
            <T as DescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as DescriptorProtoTrait>::reserved_name(t))
                    .map_right(|u| <U as DescriptorProtoTrait>::reserved_name(u)),
            )
        }
    }
    impl<T> DescriptorProtoTrait for ::std::option::Option<T>
    where
        T: DescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        type Field2MessageType<'this> = T::Field2MessageType<'this>;
        type Field2RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field2RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.field().into_iter())
                .into_iter()
                .flatten()
        }
        type Field6MessageType<'this> = T::Field6MessageType<'this>;
        type Field6RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field6RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.extension().into_iter())
                .into_iter()
                .flatten()
        }
        type Field3MessageType<'this> = T::Field3MessageType<'this>;
        type Field3RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field3RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.nested_type().into_iter())
                .into_iter()
                .flatten()
        }
        type Field4MessageType<'this> = T::Field4MessageType<'this>;
        type Field4RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field4RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.enum_type().into_iter())
                .into_iter()
                .flatten()
        }
        type Field5MessageType<'this> = T::Field5MessageType<'this>;
        type Field5RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field5RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.extension_range().into_iter())
                .into_iter()
                .flatten()
        }
        type Field8MessageType<'this> = T::Field8MessageType<'this>;
        type Field8RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field8RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.oneof_decl().into_iter())
                .into_iter()
                .flatten()
        }
        type Field7MessageType<'this> = T::Field7MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
        type Field9MessageType<'this> = T::Field9MessageType<'this>;
        type Field9RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field9RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.reserved_range().into_iter())
                .into_iter()
                .flatten()
        }
        type Field10StringType<'this> = T::Field10StringType<'this>;
        type Field10RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field10RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.reserved_name().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for DescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField2 {
        pub field: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.field.iter())
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
                >,
            >::ser_field(&self.field, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        > for DescriptorProtoSimpleField2
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        ) -> Self {
            Self { field: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField6 {
        pub extension: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField6 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField6 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
        type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.extension.iter())
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
                >,
            >::ser_field(&self.extension, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        > for DescriptorProtoSimpleField6
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        ) -> Self {
            Self { extension: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField3 {
        pub nested_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
        >,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto;
        type Field3RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.nested_type.iter())
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
                >,
            >::ser_field(&self.nested_type, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        > for DescriptorProtoSimpleField3
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        ) -> Self {
            Self { nested_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField4 {
        pub enum_type: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto;
        type Field4RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.enum_type.iter())
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
                >,
            >::ser_field(&self.enum_type, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        > for DescriptorProtoSimpleField4
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        ) -> Self {
            Self { enum_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField5 {
    pub extension_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>,
}

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>>;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.extension_range.iter())
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>
        >::ser_field(&self.extension_range, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>> for DescriptorProtoSimpleField5 {
    fn from(value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>) -> Self {
        Self {
            extension_range: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField8 {
        pub oneof_decl: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField8 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField8 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto;
        type Field8RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
            >,
        >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.oneof_decl.iter())
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField8 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
                >,
            >::ser_field(&self.oneof_decl, 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
            >,
        > for DescriptorProtoSimpleField8
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
            >,
        ) -> Self {
            Self { oneof_decl: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField7 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField7 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField7 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions;
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField7 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
                >,
            >::ser_field(&self.options, 7, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
                >,
            >,
        > for DescriptorProtoSimpleField7
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField9 {
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>,
}

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField9 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField9 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange;
        type Field9RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>>;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field10StringType<'this> = &'static str;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>
        >::ser_field(&self.reserved_range, 9, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>> for DescriptorProtoSimpleField9 {
    fn from(value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>) -> Self {
        Self {
            reserved_range: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct DescriptorProtoSimpleField10 {
        pub reserved_name: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleField10 {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProtoSimpleField10 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this> = ();
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this> = ();
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this> = ();
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this> = ();
        type Field9MessageType<'this> = ();
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10StringType<'this> = &'this str;
        type Field10RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for DescriptorProtoSimpleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.reserved_name,
                10,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::borrow::Cow<'static, str>>>
        for DescriptorProtoSimpleField10
    {
        fn from(value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                reserved_name: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct DescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::DescriptorProto> for DescriptorProtoSimpleByValue {}

    impl DescriptorProtoTrait for DescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
        >;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field6MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
        >;
        type Field6RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
        >;
        type Field3RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field4MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
        >;
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field5MessageType<'this> = ::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field8MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
        >;
        type Field8RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field7MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field9MessageType<'this> = ::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>;
        type Field9RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field10StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field10RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field10StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct DescriptorProtoBuilder<T>(T);

    impl<T> DescriptorProtoBuilder<T>
    where
        T: DescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField1)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_field(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField2)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_extension(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto,
            >,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField6)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_nested_type(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto,
            >,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField3)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_enum_type(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto,
            >,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField4)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_extension_range(
            self,
            value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField5)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_oneof_decl(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto,
            >,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField8)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
                >,
            >,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField7)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_reserved_range(
            self,
            value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField9)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_reserved_name(
            self,
            value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSimpleField10)> {
            DescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl DescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl ExtensionRangeOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
                <U as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
                <U as ExtensionRangeOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as ExtensionRangeOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as ExtensionRangeOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> ExtensionRangeOptionsTrait for ::std::option::Option<T>
    where
        T: ExtensionRangeOptionsTrait,
    {
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct ExtensionRangeOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::ExtensionRangeOptions> for ExtensionRangeOptionsSimpleField999 {}

    impl super::_puroro_traits::ExtensionRangeOptionsTrait for ExtensionRangeOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for ExtensionRangeOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for ExtensionRangeOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ExtensionRangeOptionsSimpleByValue {}
    impl ::puroro::Message<super::ExtensionRangeOptions> for ExtensionRangeOptionsSimpleByValue {}

    impl ExtensionRangeOptionsTrait for ExtensionRangeOptionsSimpleByValue {
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct ExtensionRangeOptionsBuilder<T>(T);

    impl<T> ExtensionRangeOptionsBuilder<T>
    where
        T: ExtensionRangeOptionsTrait,
    {
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> ExtensionRangeOptionsBuilder<(T, ExtensionRangeOptionsSimpleField999)> {
            ExtensionRangeOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl ExtensionRangeOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
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
        type Field8MessageType<'this> = (
            ::std::option::Option<<T as FieldDescriptorProtoTrait>::Field8MessageType<'this>>,
            ::std::option::Option<<U as FieldDescriptorProtoTrait>::Field8MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            match (
                <T as FieldDescriptorProtoTrait>::options(&self.0),
                <U as FieldDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
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
    impl<T> FieldDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: FieldDescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.number())
        }
        fn label<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            self.as_ref().and_then(|msg| msg.label())
        }
        fn type_<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            self.as_ref().and_then(|msg| msg.type_())
        }
        type Field6StringType<'this> = T::Field6StringType<'this>;
        fn type_name<'this>(&'this self) -> ::std::option::Option<Self::Field6StringType<'this>> {
            self.as_ref().and_then(|msg| msg.type_name())
        }
        type Field2StringType<'this> = T::Field2StringType<'this>;
        fn extendee<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            self.as_ref().and_then(|msg| msg.extendee())
        }
        type Field7StringType<'this> = T::Field7StringType<'this>;
        fn default_value<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7StringType<'this>> {
            self.as_ref().and_then(|msg| msg.default_value())
        }
        fn oneof_index<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.oneof_index())
        }
        type Field10StringType<'this> = T::Field10StringType<'this>;
        fn json_name<'this>(&'this self) -> ::std::option::Option<Self::Field10StringType<'this>> {
            self.as_ref().and_then(|msg| msg.json_name())
        }
        type Field8MessageType<'this> = T::Field8MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
        fn proto3_optional<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.proto3_optional())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField1 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FieldDescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField3 {
        pub number: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField3 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.number,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for FieldDescriptorProtoSimpleField3 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self { number: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField4 {
        pub label: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        >,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField4 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label>
        >::ser_field(&self.label, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
            >,
        > for FieldDescriptorProtoSimpleField4
    {
        fn from(
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
            >,
        ) -> Self {
            Self { label: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField5 {
        pub type_: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        >,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField5 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type>
        >::ser_field(&self.type_, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
            >,
        > for FieldDescriptorProtoSimpleField5
    {
        fn from(
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
            >,
        ) -> Self {
            Self { type_: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField6 {
        pub type_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField6 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.type_name,
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FieldDescriptorProtoSimpleField6
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { type_name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField2 {
        pub extendee: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField2 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.extendee,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FieldDescriptorProtoSimpleField2
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { extendee: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField7 {
        pub default_value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField7 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField7 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.default_value,
                7,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FieldDescriptorProtoSimpleField7
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                default_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField9 {
        pub oneof_index: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField9 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.oneof_index,
                9,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for FieldDescriptorProtoSimpleField9 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self { oneof_index: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField10 {
        pub json_name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField10 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.json_name,
                10,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FieldDescriptorProtoSimpleField10
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { json_name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField8 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField8 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleField8 {
        type Field1StringType<'this> = &'static str;
        type Field6StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field7StringType<'this> = &'static str;
        type Field10StringType<'this> = &'static str;
        type Field8MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField8 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
                >,
            >::ser_field(&self.options, 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
                >,
            >,
        > for FieldDescriptorProtoSimpleField8
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldDescriptorProtoSimpleField17 {
        pub proto3_optional: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleField17 {}

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

    impl ::puroro::SerializableMessageToIoWrite for FieldDescriptorProtoSimpleField17 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.proto3_optional,
                17,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FieldDescriptorProtoSimpleField17 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                proto3_optional: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FieldDescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSimpleByValue {}

    impl FieldDescriptorProtoTrait for FieldDescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn number<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn label<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn type_<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field6StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn type_name<'this>(&'this self) -> Option<Self::Field6StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn extendee<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field7StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn default_value<'this>(&'this self) -> Option<Self::Field7StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn oneof_index<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field10StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn json_name<'this>(&'this self) -> Option<Self::Field10StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field8MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn proto3_optional<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct FieldDescriptorProtoBuilder<T>(T);

    impl<T> FieldDescriptorProtoBuilder<T>
    where
        T: FieldDescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField1)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_number(
            self,
            value: ::std::option::Option<i32>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField3)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_label(
            self,
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
            >,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField4)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_type_(
            self,
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
            >,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField5)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_type_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField6)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_extendee(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField2)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_default_value(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField7)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_oneof_index(
            self,
            value: ::std::option::Option<i32>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField9)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_json_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField10)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
                >,
            >,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField8)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_proto3_optional(
            self,
            value: ::std::option::Option<bool>,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSimpleField17)> {
            FieldDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl FieldDescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
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
        type Field2MessageType<'this> = (
            ::std::option::Option<<T as OneofDescriptorProtoTrait>::Field2MessageType<'this>>,
            ::std::option::Option<<U as OneofDescriptorProtoTrait>::Field2MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            match (
                <T as OneofDescriptorProtoTrait>::options(&self.0),
                <U as OneofDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
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
    impl<T> OneofDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: OneofDescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        type Field2MessageType<'this> = T::Field2MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field2MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct OneofDescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::OneofDescriptorProto> for OneofDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::OneofDescriptorProtoTrait for OneofDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for OneofDescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for OneofDescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct OneofDescriptorProtoSimpleField2 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::OneofDescriptorProto> for OneofDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::OneofDescriptorProtoTrait for OneofDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions;
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for OneofDescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
                >,
            >::ser_field(&self.options, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
                >,
            >,
        > for OneofDescriptorProtoSimpleField2
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct OneofDescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::OneofDescriptorProto> for OneofDescriptorProtoSimpleByValue {}

    impl OneofDescriptorProtoTrait for OneofDescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct OneofDescriptorProtoBuilder<T>(T);

    impl<T> OneofDescriptorProtoBuilder<T>
    where
        T: OneofDescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> OneofDescriptorProtoBuilder<(T, OneofDescriptorProtoSimpleField1)> {
            OneofDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
                >,
            >,
        ) -> OneofDescriptorProtoBuilder<(T, OneofDescriptorProtoSimpleField2)> {
            OneofDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl OneofDescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl EnumDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumDescriptorProtoTrait>::value(&self.0),
                <U as EnumDescriptorProtoTrait>::value(&self.1),
            )
        }
        type Field3MessageType<'this> = (
            ::std::option::Option<<T as EnumDescriptorProtoTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as EnumDescriptorProtoTrait>::Field3MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as EnumDescriptorProtoTrait>::options(&self.0),
                <U as EnumDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field4MessageType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumDescriptorProtoTrait>::reserved_range(&self.0),
                <U as EnumDescriptorProtoTrait>::reserved_range(&self.1),
            )
        }
        type Field5StringType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field5StringType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5StringType<'this>,
        >;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::merged::MergedRepeatedLDField<
            <T as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
                <U as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
            >;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumDescriptorProtoTrait>::reserved_range(t))
                    .map_right(|u| <U as EnumDescriptorProtoTrait>::reserved_range(u)),
            )
        }
        type Field5StringType<'this> = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field5StringType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5StringType<'this>,
        >;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::either::EitherRepeatedLDField<
            <T as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumDescriptorProtoTrait>::reserved_name(t))
                    .map_right(|u| <U as EnumDescriptorProtoTrait>::reserved_name(u)),
            )
        }
    }
    impl<T> EnumDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: EnumDescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        type Field2MessageType<'this> = T::Field2MessageType<'this>;
        type Field2RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field2RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.value().into_iter())
                .into_iter()
                .flatten()
        }
        type Field3MessageType<'this> = T::Field3MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
        type Field4MessageType<'this> = T::Field4MessageType<'this>;
        type Field4RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field4RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.reserved_range().into_iter())
                .into_iter()
                .flatten()
        }
        type Field5StringType<'this> = T::Field5StringType<'this>;
        type Field5RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field5RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.reserved_name().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumDescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::EnumDescriptorProto> for EnumDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumDescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for EnumDescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumDescriptorProtoSimpleField2 {
        pub value: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::EnumDescriptorProto> for EnumDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
            >,
        >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.value.iter())
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumDescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto>
        >::ser_field(&self.value, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
            >,
        > for EnumDescriptorProtoSimpleField2
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
            >,
        ) -> Self {
            Self { value: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumDescriptorProtoSimpleField3 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::EnumDescriptorProto> for EnumDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumDescriptorProtoSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
                >,
            >,
        > for EnumDescriptorProtoSimpleField3
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumDescriptorProtoSimpleField4 {
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>,
}

    impl ::puroro::Message<super::EnumDescriptorProto> for EnumDescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange;
        type Field4RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>>;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field5StringType<'this> = &'static str;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumDescriptorProtoSimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>
        >::ser_field(&self.reserved_range, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>> for EnumDescriptorProtoSimpleField4 {
    fn from(value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>) -> Self {
        Self {
            reserved_range: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumDescriptorProtoSimpleField5 {
        pub reserved_name: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::EnumDescriptorProto> for EnumDescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
        type Field4MessageType<'this> = ();
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5StringType<'this> = &'this str;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_name.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumDescriptorProtoSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::String>::ser_field(
                &self.reserved_name,
                5,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::borrow::Cow<'static, str>>>
        for EnumDescriptorProtoSimpleField5
    {
        fn from(value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                reserved_name: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumDescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::EnumDescriptorProto> for EnumDescriptorProtoSimpleByValue {}

    impl EnumDescriptorProtoTrait for EnumDescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
        >;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field4MessageType<'this> = ::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>;
        type Field4RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field5StringType<'this> = ::std::borrow::Cow<'this, str>;
        type Field5RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5StringType<'this>>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct EnumDescriptorProtoBuilder<T>(T);

    impl<T> EnumDescriptorProtoBuilder<T>
    where
        T: EnumDescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSimpleField1)> {
            EnumDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_value(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto,
            >,
        ) -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSimpleField2)> {
            EnumDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
                >,
            >,
        ) -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSimpleField3)> {
            EnumDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_reserved_range(
            self,
            value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>,
        ) -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSimpleField4)> {
            EnumDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_reserved_name(
            self,
            value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
        ) -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSimpleField5)> {
            EnumDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl EnumDescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
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
        type Field3MessageType<'this> = (
            ::std::option::Option<<T as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as EnumValueDescriptorProtoTrait>::options(&self.0),
                <U as EnumValueDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
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
    impl<T> EnumValueDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: EnumValueDescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.number())
        }
        type Field3MessageType<'this> = T::Field3MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumValueDescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::EnumValueDescriptorProto> for EnumValueDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field3MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumValueDescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for EnumValueDescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumValueDescriptorProtoSimpleField2 {
        pub number: ::std::option::Option<i32>,
    }

    impl ::puroro::Message<super::EnumValueDescriptorProto> for EnumValueDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        fn number<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        type Field3MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumValueDescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field(
                &self.number,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i32>> for EnumValueDescriptorProtoSimpleField2 {
        fn from(value: ::std::option::Option<i32>) -> Self {
            Self { number: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumValueDescriptorProtoSimpleField3 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::EnumValueDescriptorProto> for EnumValueDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumValueDescriptorProtoSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
                >,
            >,
        > for EnumValueDescriptorProtoSimpleField3
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumValueDescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::EnumValueDescriptorProto> for EnumValueDescriptorProtoSimpleByValue {}

    impl EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn number<'this>(&'this self) -> Option<i32> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct EnumValueDescriptorProtoBuilder<T>(T);

    impl<T> EnumValueDescriptorProtoBuilder<T>
    where
        T: EnumValueDescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> EnumValueDescriptorProtoBuilder<(T, EnumValueDescriptorProtoSimpleField1)> {
            EnumValueDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_number(
            self,
            value: ::std::option::Option<i32>,
        ) -> EnumValueDescriptorProtoBuilder<(T, EnumValueDescriptorProtoSimpleField2)> {
            EnumValueDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
                >,
            >,
        ) -> EnumValueDescriptorProtoBuilder<(T, EnumValueDescriptorProtoSimpleField3)> {
            EnumValueDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl EnumValueDescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl ServiceDescriptorProtoTrait for () {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as ServiceDescriptorProtoTrait>::method(&self.0),
                <U as ServiceDescriptorProtoTrait>::method(&self.1),
            )
        }
        type Field3MessageType<'this> = (
            ::std::option::Option<<T as ServiceDescriptorProtoTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as ServiceDescriptorProtoTrait>::Field3MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as ServiceDescriptorProtoTrait>::options(&self.0),
                <U as ServiceDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
                <U as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
            >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
    impl<T> ServiceDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: ServiceDescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        type Field2MessageType<'this> = T::Field2MessageType<'this>;
        type Field2RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field2RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.method().into_iter())
                .into_iter()
                .flatten()
        }
        type Field3MessageType<'this> = T::Field3MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct ServiceDescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::ServiceDescriptorProto> for ServiceDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for ServiceDescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for ServiceDescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct ServiceDescriptorProtoSimpleField2 {
        pub method: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
        >,
    }

    impl ::puroro::Message<super::ServiceDescriptorProto> for ServiceDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
            >,
        >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.method.iter())
        }
        type Field3MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for ServiceDescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto>
        >::ser_field(&self.method, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
            >,
        > for ServiceDescriptorProtoSimpleField2
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
            >,
        ) -> Self {
            Self { method: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct ServiceDescriptorProtoSimpleField3 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::ServiceDescriptorProto> for ServiceDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for ServiceDescriptorProtoSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
                >,
            >::ser_field(&self.options, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
                >,
            >,
        > for ServiceDescriptorProtoSimpleField3
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ServiceDescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::ServiceDescriptorProto> for ServiceDescriptorProtoSimpleByValue {}

    impl ServiceDescriptorProtoTrait for ServiceDescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
        >;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct ServiceDescriptorProtoBuilder<T>(T);

    impl<T> ServiceDescriptorProtoBuilder<T>
    where
        T: ServiceDescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> ServiceDescriptorProtoBuilder<(T, ServiceDescriptorProtoSimpleField1)> {
            ServiceDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_method(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto,
            >,
        ) -> ServiceDescriptorProtoBuilder<(T, ServiceDescriptorProtoSimpleField2)> {
            ServiceDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
                >,
            >,
        ) -> ServiceDescriptorProtoBuilder<(T, ServiceDescriptorProtoSimpleField3)> {
            ServiceDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl ServiceDescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
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
        type Field4MessageType<'this> = (
            ::std::option::Option<<T as MethodDescriptorProtoTrait>::Field4MessageType<'this>>,
            ::std::option::Option<<U as MethodDescriptorProtoTrait>::Field4MessageType<'this>>,
        );
        fn options<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            match (
                <T as MethodDescriptorProtoTrait>::options(&self.0),
                <U as MethodDescriptorProtoTrait>::options(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
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
    impl<T> MethodDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: MethodDescriptorProtoTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.name())
        }
        type Field2StringType<'this> = T::Field2StringType<'this>;
        fn input_type<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            self.as_ref().and_then(|msg| msg.input_type())
        }
        type Field3StringType<'this> = T::Field3StringType<'this>;
        fn output_type<'this>(&'this self) -> ::std::option::Option<Self::Field3StringType<'this>> {
            self.as_ref().and_then(|msg| msg.output_type())
        }
        type Field4MessageType<'this> = T::Field4MessageType<'this>;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field4MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options())
        }
        fn client_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.client_streaming())
        }
        fn server_streaming<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.server_streaming())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodDescriptorProtoSimpleField1 {
        pub name: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSimpleField1 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField1 {
        type Field1StringType<'this> = &'this str;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodDescriptorProtoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.name, 1, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for MethodDescriptorProtoSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodDescriptorProtoSimpleField2 {
        pub input_type: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSimpleField2 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField2 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'this str;
        fn input_type<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            self.input_type.as_ref().map(|v| v.as_ref())
        }
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodDescriptorProtoSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.input_type,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for MethodDescriptorProtoSimpleField2
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { input_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodDescriptorProtoSimpleField3 {
        pub output_type: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSimpleField3 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField3 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'this str;
        fn output_type<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.output_type.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> = ();
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodDescriptorProtoSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.output_type,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for MethodDescriptorProtoSimpleField3
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { output_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodDescriptorProtoSimpleField4 {
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
            >,
        >,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSimpleField4 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField4 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions;
        fn options<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodDescriptorProtoSimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
                >,
            >::ser_field(&self.options, 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
                >,
            >,
        > for MethodDescriptorProtoSimpleField4
    {
        fn from(
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
                >,
            >,
        ) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodDescriptorProtoSimpleField5 {
        pub client_streaming: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSimpleField5 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField5 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
        fn client_streaming<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.client_streaming)
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodDescriptorProtoSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.client_streaming,
                5,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for MethodDescriptorProtoSimpleField5 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                client_streaming: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodDescriptorProtoSimpleField6 {
        pub server_streaming: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSimpleField6 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleField6 {
        type Field1StringType<'this> = &'static str;
        type Field2StringType<'this> = &'static str;
        type Field3StringType<'this> = &'static str;
        type Field4MessageType<'this> = ();
        fn server_streaming<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.server_streaming)
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodDescriptorProtoSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.server_streaming,
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for MethodDescriptorProtoSimpleField6 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                server_streaming: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MethodDescriptorProtoSimpleByValue {}
    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSimpleByValue {}

    impl MethodDescriptorProtoTrait for MethodDescriptorProtoSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn name<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn input_type<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn output_type<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field4MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
        >;
        fn options<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn client_streaming<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn server_streaming<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MethodDescriptorProtoBuilder<T>(T);

    impl<T> MethodDescriptorProtoBuilder<T>
    where
        T: MethodDescriptorProtoTrait,
    {
        pub fn append_name(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSimpleField1)> {
            MethodDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_input_type(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSimpleField2)> {
            MethodDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_output_type(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSimpleField3)> {
            MethodDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_options(
            self,
            value: ::std::option::Option<
                ::std::boxed::Box<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
                >,
            >,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSimpleField4)> {
            MethodDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_client_streaming(
            self,
            value: ::std::option::Option<bool>,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSimpleField5)> {
            MethodDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_server_streaming(
            self,
            value: ::std::option::Option<bool>,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSimpleField6)> {
            MethodDescriptorProtoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MethodDescriptorProtoBuilder<()> {
        pub fn new() -> Self {
            Self(())
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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as FileOptionsTrait>::Field999RepeatedType<'this>,
                <U as FileOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as FileOptionsTrait>::Field999RepeatedType<'this>,
                <U as FileOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FileOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as FileOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> FileOptionsTrait for ::std::option::Option<T>
    where
        T: FileOptionsTrait,
    {
        type Field1StringType<'this> = T::Field1StringType<'this>;
        fn java_package<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1StringType<'this>> {
            self.as_ref().and_then(|msg| msg.java_package())
        }
        type Field8StringType<'this> = T::Field8StringType<'this>;
        fn java_outer_classname<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8StringType<'this>> {
            self.as_ref().and_then(|msg| msg.java_outer_classname())
        }
        fn java_multiple_files<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.java_multiple_files())
        }
        fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref()
                .and_then(|msg| msg.java_generate_equals_and_hash())
        }
        fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.java_string_check_utf8())
        }
        fn optimize_for<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        > {
            self.as_ref().and_then(|msg| msg.optimize_for())
        }
        type Field11StringType<'this> = T::Field11StringType<'this>;
        fn go_package<'this>(&'this self) -> ::std::option::Option<Self::Field11StringType<'this>> {
            self.as_ref().and_then(|msg| msg.go_package())
        }
        fn cc_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.cc_generic_services())
        }
        fn java_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.java_generic_services())
        }
        fn py_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.py_generic_services())
        }
        fn php_generic_services<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.php_generic_services())
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated())
        }
        fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.cc_enable_arenas())
        }
        type Field36StringType<'this> = T::Field36StringType<'this>;
        fn objc_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field36StringType<'this>> {
            self.as_ref().and_then(|msg| msg.objc_class_prefix())
        }
        type Field37StringType<'this> = T::Field37StringType<'this>;
        fn csharp_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field37StringType<'this>> {
            self.as_ref().and_then(|msg| msg.csharp_namespace())
        }
        type Field39StringType<'this> = T::Field39StringType<'this>;
        fn swift_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field39StringType<'this>> {
            self.as_ref().and_then(|msg| msg.swift_prefix())
        }
        type Field40StringType<'this> = T::Field40StringType<'this>;
        fn php_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field40StringType<'this>> {
            self.as_ref().and_then(|msg| msg.php_class_prefix())
        }
        type Field41StringType<'this> = T::Field41StringType<'this>;
        fn php_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41StringType<'this>> {
            self.as_ref().and_then(|msg| msg.php_namespace())
        }
        type Field44StringType<'this> = T::Field44StringType<'this>;
        fn php_metadata_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field44StringType<'this>> {
            self.as_ref().and_then(|msg| msg.php_metadata_namespace())
        }
        type Field45StringType<'this> = T::Field45StringType<'this>;
        fn ruby_package<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field45StringType<'this>> {
            self.as_ref().and_then(|msg| msg.ruby_package())
        }
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField1 {
        pub java_package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField1 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.java_package,
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField1
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                java_package: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField8 {
        pub java_outer_classname: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField8 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField8 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.java_outer_classname,
                8,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField8
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                java_outer_classname: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField10 {
        pub java_multiple_files: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField10 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_multiple_files,
                10,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField10 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                java_multiple_files: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField20 {
        pub java_generate_equals_and_hash: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField20 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField20 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_generate_equals_and_hash,
                20,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField20 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                java_generate_equals_and_hash: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField27 {
        pub java_string_check_utf8: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField27 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField27 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_string_check_utf8,
                27,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField27 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                java_string_check_utf8: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField9 {
        pub optimize_for: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        >,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField9 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        >::ser_field(&self.optimize_for, 9, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
            >,
        > for FileOptionsSimpleField9
    {
        fn from(
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
            >,
        ) -> Self {
            Self {
                optimize_for: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField11 {
        pub go_package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField11 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField11 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.go_package,
                11,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField11
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self { go_package: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField16 {
        pub cc_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField16 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField16 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.cc_generic_services,
                16,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField16 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                cc_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField17 {
        pub java_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField17 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField17 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.java_generic_services,
                17,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField17 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                java_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField18 {
        pub py_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField18 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField18 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.py_generic_services,
                18,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField18 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                py_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField42 {
        pub php_generic_services: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField42 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField42 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.php_generic_services,
                42,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField42 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                php_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField23 {
        pub deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField23 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField23 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                23,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField23 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField31 {
        pub cc_enable_arenas: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField31 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField31 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.cc_enable_arenas,
                31,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FileOptionsSimpleField31 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                cc_enable_arenas: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField36 {
        pub objc_class_prefix: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField36 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField36 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.objc_class_prefix,
                36,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField36
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                objc_class_prefix: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField37 {
        pub csharp_namespace: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField37 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField37 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.csharp_namespace,
                37,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField37
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                csharp_namespace: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField39 {
        pub swift_prefix: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField39 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField39 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.swift_prefix,
                39,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField39
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                swift_prefix: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField40 {
        pub php_class_prefix: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField40 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField40 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.php_class_prefix,
                40,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField40
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                php_class_prefix: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField41 {
        pub php_namespace: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField41 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField41 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.php_namespace,
                41,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField41
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                php_namespace: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField44 {
        pub php_metadata_namespace: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField44 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField44 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.php_metadata_namespace,
                44,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField44
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                php_metadata_namespace: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField45 {
        pub ruby_package: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField45 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField45 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.ruby_package,
                45,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for FileOptionsSimpleField45
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                ruby_package: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FileOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleField999 {}

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
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FileOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for FileOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FileOptionsSimpleByValue {}
    impl ::puroro::Message<super::FileOptions> for FileOptionsSimpleByValue {}

    impl FileOptionsTrait for FileOptionsSimpleByValue {
        type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn java_package<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field8StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn java_outer_classname<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn java_multiple_files<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn java_generate_equals_and_hash<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn java_string_check_utf8<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn optimize_for<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field11StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn go_package<'this>(&'this self) -> Option<Self::Field11StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn cc_generic_services<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn java_generic_services<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn py_generic_services<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn php_generic_services<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn cc_enable_arenas<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field36StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn objc_class_prefix<'this>(&'this self) -> Option<Self::Field36StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field37StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn csharp_namespace<'this>(&'this self) -> Option<Self::Field37StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field39StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn swift_prefix<'this>(&'this self) -> Option<Self::Field39StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field40StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn php_class_prefix<'this>(&'this self) -> Option<Self::Field40StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field41StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn php_namespace<'this>(&'this self) -> Option<Self::Field41StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field44StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn php_metadata_namespace<'this>(&'this self) -> Option<Self::Field44StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field45StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn ruby_package<'this>(&'this self) -> Option<Self::Field45StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct FileOptionsBuilder<T>(T);

    impl<T> FileOptionsBuilder<T>
    where
        T: FileOptionsTrait,
    {
        pub fn append_java_package(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField1)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_java_outer_classname(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField8)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_java_multiple_files(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField10)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_java_generate_equals_and_hash(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField20)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_java_string_check_utf8(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField27)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_optimize_for(
            self,
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
            >,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField9)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_go_package(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField11)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_cc_generic_services(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField16)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_java_generic_services(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField17)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_py_generic_services(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField18)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_php_generic_services(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField42)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_deprecated(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField23)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_cc_enable_arenas(
            self,
            value: ::std::option::Option<bool>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField31)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_objc_class_prefix(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField36)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_csharp_namespace(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField37)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_swift_prefix(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField39)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_php_class_prefix(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField40)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_php_namespace(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField41)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_php_metadata_namespace(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField44)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_ruby_package(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField45)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> FileOptionsBuilder<(T, FileOptionsSimpleField999)> {
            FileOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl FileOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl MessageOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as MessageOptionsTrait>::Field999RepeatedType<'this>,
                <U as MessageOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as MessageOptionsTrait>::Field999RepeatedType<'this>,
                <U as MessageOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MessageOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as MessageOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> MessageOptionsTrait for ::std::option::Option<T>
    where
        T: MessageOptionsTrait,
    {
        fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.message_set_wire_format())
        }
        fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref()
                .and_then(|msg| msg.no_standard_descriptor_accessor())
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated())
        }
        fn map_entry<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.map_entry())
        }
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MessageOptionsSimpleField1 {
        pub message_set_wire_format: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSimpleField1 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField1 {
        fn message_set_wire_format<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.message_set_wire_format)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MessageOptionsSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.message_set_wire_format,
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for MessageOptionsSimpleField1 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                message_set_wire_format: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MessageOptionsSimpleField2 {
        pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSimpleField2 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField2 {
        fn no_standard_descriptor_accessor<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.no_standard_descriptor_accessor)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MessageOptionsSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.no_standard_descriptor_accessor,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for MessageOptionsSimpleField2 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self {
                no_standard_descriptor_accessor: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MessageOptionsSimpleField3 {
        pub deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSimpleField3 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField3 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MessageOptionsSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for MessageOptionsSimpleField3 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MessageOptionsSimpleField7 {
        pub map_entry: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSimpleField7 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField7 {
        fn map_entry<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.map_entry)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MessageOptionsSimpleField7 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.map_entry,
                7,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for MessageOptionsSimpleField7 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { map_entry: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MessageOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSimpleField999 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MessageOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for MessageOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MessageOptionsSimpleByValue {}
    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSimpleByValue {}

    impl MessageOptionsTrait for MessageOptionsSimpleByValue {
        fn message_set_wire_format<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn no_standard_descriptor_accessor<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn map_entry<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MessageOptionsBuilder<T>(T);

    impl<T> MessageOptionsBuilder<T>
    where
        T: MessageOptionsTrait,
    {
        pub fn append_message_set_wire_format(
            self,
            value: ::std::option::Option<bool>,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSimpleField1)> {
            MessageOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_no_standard_descriptor_accessor(
            self,
            value: ::std::option::Option<bool>,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSimpleField2)> {
            MessageOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_deprecated(
            self,
            value: ::std::option::Option<bool>,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSimpleField3)> {
            MessageOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_map_entry(
            self,
            value: ::std::option::Option<bool>,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSimpleField7)> {
            MessageOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSimpleField999)> {
            MessageOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MessageOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl FieldOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as FieldOptionsTrait>::Field999RepeatedType<'this>,
                <U as FieldOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as FieldOptionsTrait>::Field999RepeatedType<'this>,
                <U as FieldOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as FieldOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as FieldOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> FieldOptionsTrait for ::std::option::Option<T>
    where
        T: FieldOptionsTrait,
    {
        fn ctype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        > {
            self.as_ref().and_then(|msg| msg.ctype())
        }
        fn packed<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.packed())
        }
        fn jstype<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        > {
            self.as_ref().and_then(|msg| msg.jstype())
        }
        fn lazy<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.lazy())
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated())
        }
        fn weak<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.weak())
        }
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldOptionsSimpleField1 {
        pub ctype: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        >,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleField1 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField1 {
        fn ctype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            Clone::clone(&self.ctype)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptionsSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
                >,
            >::ser_field(&self.ctype, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
            >,
        > for FieldOptionsSimpleField1
    {
        fn from(
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
            >,
        ) -> Self {
            Self { ctype: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldOptionsSimpleField2 {
        pub packed: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleField2 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField2 {
        fn packed<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.packed)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptionsSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.packed,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FieldOptionsSimpleField2 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { packed: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldOptionsSimpleField6 {
        pub jstype: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        >,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleField6 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField6 {
        fn jstype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            Clone::clone(&self.jstype)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptionsSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
                >,
            >::ser_field(&self.jstype, 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
            >,
        > for FieldOptionsSimpleField6
    {
        fn from(
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
            >,
        ) -> Self {
            Self { jstype: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldOptionsSimpleField5 {
        pub lazy: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleField5 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField5 {
        fn lazy<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.lazy)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptionsSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.lazy, 5, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FieldOptionsSimpleField5 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { lazy: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldOptionsSimpleField3 {
        pub deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleField3 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField3 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptionsSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FieldOptionsSimpleField3 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldOptionsSimpleField10 {
        pub weak: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleField10 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField10 {
        fn weak<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.weak)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptionsSimpleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.weak, 10, out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for FieldOptionsSimpleField10 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { weak: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct FieldOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleField999 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for FieldOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for FieldOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct FieldOptionsSimpleByValue {}
    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSimpleByValue {}

    impl FieldOptionsTrait for FieldOptionsSimpleByValue {
        fn ctype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn packed<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn jstype<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn lazy<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn weak<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct FieldOptionsBuilder<T>(T);

    impl<T> FieldOptionsBuilder<T>
    where
        T: FieldOptionsTrait,
    {
        pub fn append_ctype(
            self,
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
            >,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSimpleField1)> {
            FieldOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_packed(
            self,
            value: ::std::option::Option<bool>,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSimpleField2)> {
            FieldOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_jstype(
            self,
            value: ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
            >,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSimpleField6)> {
            FieldOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_lazy(
            self,
            value: ::std::option::Option<bool>,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSimpleField5)> {
            FieldOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_deprecated(
            self,
            value: ::std::option::Option<bool>,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSimpleField3)> {
            FieldOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_weak(
            self,
            value: ::std::option::Option<bool>,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSimpleField10)> {
            FieldOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSimpleField999)> {
            FieldOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl FieldOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl OneofOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as OneofOptionsTrait>::Field999RepeatedType<'this>,
                <U as OneofOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as OneofOptionsTrait>::Field999RepeatedType<'this>,
                <U as OneofOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as OneofOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as OneofOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> OneofOptionsTrait for ::std::option::Option<T>
    where
        T: OneofOptionsTrait,
    {
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct OneofOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::OneofOptions> for OneofOptionsSimpleField999 {}

    impl super::_puroro_traits::OneofOptionsTrait for OneofOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for OneofOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for OneofOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct OneofOptionsSimpleByValue {}
    impl ::puroro::Message<super::OneofOptions> for OneofOptionsSimpleByValue {}

    impl OneofOptionsTrait for OneofOptionsSimpleByValue {
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct OneofOptionsBuilder<T>(T);

    impl<T> OneofOptionsBuilder<T>
    where
        T: OneofOptionsTrait,
    {
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> OneofOptionsBuilder<(T, OneofOptionsSimpleField999)> {
            OneofOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl OneofOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl EnumOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as EnumOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as EnumOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> EnumOptionsTrait for ::std::option::Option<T>
    where
        T: EnumOptionsTrait,
    {
        fn allow_alias<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.allow_alias())
        }
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated())
        }
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumOptionsSimpleField2 {
        pub allow_alias: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::EnumOptions> for EnumOptionsSimpleField2 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSimpleField2 {
        fn allow_alias<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.allow_alias)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumOptionsSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.allow_alias,
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for EnumOptionsSimpleField2 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { allow_alias: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumOptionsSimpleField3 {
        pub deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::EnumOptions> for EnumOptionsSimpleField3 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSimpleField3 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumOptionsSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for EnumOptionsSimpleField3 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::EnumOptions> for EnumOptionsSimpleField999 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for EnumOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumOptionsSimpleByValue {}
    impl ::puroro::Message<super::EnumOptions> for EnumOptionsSimpleByValue {}

    impl EnumOptionsTrait for EnumOptionsSimpleByValue {
        fn allow_alias<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn deprecated<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct EnumOptionsBuilder<T>(T);

    impl<T> EnumOptionsBuilder<T>
    where
        T: EnumOptionsTrait,
    {
        pub fn append_allow_alias(
            self,
            value: ::std::option::Option<bool>,
        ) -> EnumOptionsBuilder<(T, EnumOptionsSimpleField2)> {
            EnumOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_deprecated(
            self,
            value: ::std::option::Option<bool>,
        ) -> EnumOptionsBuilder<(T, EnumOptionsSimpleField3)> {
            EnumOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> EnumOptionsBuilder<(T, EnumOptionsSimpleField999)> {
            EnumOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl EnumOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl EnumValueOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
                <U as EnumValueOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as EnumValueOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as EnumValueOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> EnumValueOptionsTrait for ::std::option::Option<T>
    where
        T: EnumValueOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated())
        }
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumValueOptionsSimpleField1 {
        pub deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::EnumValueOptions> for EnumValueOptionsSimpleField1 {}

    impl super::_puroro_traits::EnumValueOptionsTrait for EnumValueOptionsSimpleField1 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumValueOptionsSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                1,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for EnumValueOptionsSimpleField1 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct EnumValueOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::EnumValueOptions> for EnumValueOptionsSimpleField999 {}

    impl super::_puroro_traits::EnumValueOptionsTrait for EnumValueOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for EnumValueOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for EnumValueOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumValueOptionsSimpleByValue {}
    impl ::puroro::Message<super::EnumValueOptions> for EnumValueOptionsSimpleByValue {}

    impl EnumValueOptionsTrait for EnumValueOptionsSimpleByValue {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct EnumValueOptionsBuilder<T>(T);

    impl<T> EnumValueOptionsBuilder<T>
    where
        T: EnumValueOptionsTrait,
    {
        pub fn append_deprecated(
            self,
            value: ::std::option::Option<bool>,
        ) -> EnumValueOptionsBuilder<(T, EnumValueOptionsSimpleField1)> {
            EnumValueOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> EnumValueOptionsBuilder<(T, EnumValueOptionsSimpleField999)> {
            EnumValueOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl EnumValueOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl ServiceOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as ServiceOptionsTrait>::Field999RepeatedType<'this>,
                <U as ServiceOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as ServiceOptionsTrait>::Field999RepeatedType<'this>,
                <U as ServiceOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as ServiceOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as ServiceOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> ServiceOptionsTrait for ::std::option::Option<T>
    where
        T: ServiceOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated())
        }
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct ServiceOptionsSimpleField33 {
        pub deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::ServiceOptions> for ServiceOptionsSimpleField33 {}

    impl super::_puroro_traits::ServiceOptionsTrait for ServiceOptionsSimpleField33 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for ServiceOptionsSimpleField33 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                33,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for ServiceOptionsSimpleField33 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct ServiceOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::ServiceOptions> for ServiceOptionsSimpleField999 {}

    impl super::_puroro_traits::ServiceOptionsTrait for ServiceOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for ServiceOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for ServiceOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ServiceOptionsSimpleByValue {}
    impl ::puroro::Message<super::ServiceOptions> for ServiceOptionsSimpleByValue {}

    impl ServiceOptionsTrait for ServiceOptionsSimpleByValue {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct ServiceOptionsBuilder<T>(T);

    impl<T> ServiceOptionsBuilder<T>
    where
        T: ServiceOptionsTrait,
    {
        pub fn append_deprecated(
            self,
            value: ::std::option::Option<bool>,
        ) -> ServiceOptionsBuilder<(T, ServiceOptionsSimpleField33)> {
            ServiceOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> ServiceOptionsBuilder<(T, ServiceOptionsSimpleField999)> {
            ServiceOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl ServiceOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl MethodOptionsTrait for () {
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as MethodOptionsTrait>::Field999RepeatedType<'this>,
                <U as MethodOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as MethodOptionsTrait>::Field999RepeatedType<'this>,
                <U as MethodOptionsTrait>::Field999RepeatedType<'this>,
            >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as MethodOptionsTrait>::uninterpreted_option(t))
                    .map_right(|u| <U as MethodOptionsTrait>::uninterpreted_option(u)),
            )
        }
    }
    impl<T> MethodOptionsTrait for ::std::option::Option<T>
    where
        T: MethodOptionsTrait,
    {
        fn deprecated<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated())
        }
        fn idempotency_level<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            self.as_ref().and_then(|msg| msg.idempotency_level())
        }
        type Field999MessageType<'this> = T::Field999MessageType<'this>;
        type Field999RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field999RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.uninterpreted_option().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodOptionsSimpleField33 {
        pub deprecated: ::std::option::Option<bool>,
    }

    impl ::puroro::Message<super::MethodOptions> for MethodOptionsSimpleField33 {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptionsSimpleField33 {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this> = ();
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodOptionsSimpleField33 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field(
                &self.deprecated,
                33,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<bool>> for MethodOptionsSimpleField33 {
        fn from(value: ::std::option::Option<bool>) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodOptionsSimpleField34 {
        pub idempotency_level: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        >,
    }

    impl ::puroro::Message<super::MethodOptions> for MethodOptionsSimpleField34 {}

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
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodOptionsSimpleField34 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>
        >::ser_field(&self.idempotency_level, 34, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>> for MethodOptionsSimpleField34 {
    fn from(value: ::std::option::Option<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>) -> Self {
        Self {
            idempotency_level: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct MethodOptionsSimpleField999 {
        pub uninterpreted_option: ::std::vec::Vec<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }

    impl ::puroro::Message<super::MethodOptions> for MethodOptionsSimpleField999 {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptionsSimpleField999 {
        type Field999MessageType<'this> =
            &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
        type Field999RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            ::std::slice::Iter<
                'this,
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        >;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.uninterpreted_option.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for MethodOptionsSimpleField999 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Repeated,
                ::puroro::tags::Message<
                    self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
                >,
            >::ser_field(&self.uninterpreted_option, 999, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        > for MethodOptionsSimpleField999
    {
        fn from(
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MethodOptionsSimpleByValue {}
    impl ::puroro::Message<super::MethodOptions> for MethodOptionsSimpleByValue {}

    impl MethodOptionsTrait for MethodOptionsSimpleByValue {
        fn deprecated<'this>(&'this self) -> Option<bool> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn idempotency_level<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field999MessageType<'this> = ::std::boxed::Box<
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >;
        type Field999RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct MethodOptionsBuilder<T>(T);

    impl<T> MethodOptionsBuilder<T>
    where
        T: MethodOptionsTrait,
    {
        pub fn append_deprecated(
            self,
            value: ::std::option::Option<bool>,
        ) -> MethodOptionsBuilder<(T, MethodOptionsSimpleField33)> {
            MethodOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_idempotency_level(
            self,
            value: ::std::option::Option<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>,
        ) -> MethodOptionsBuilder<(T, MethodOptionsSimpleField34)> {
            MethodOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_uninterpreted_option(
            self,
            value: ::std::vec::Vec<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
            >,
        ) -> MethodOptionsBuilder<(T, MethodOptionsSimpleField999)> {
            MethodOptionsBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl MethodOptionsBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl UninterpretedOptionTrait for () {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
                <U as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
            >;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
                <U as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
            >;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
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
    impl<T> UninterpretedOptionTrait for ::std::option::Option<T>
    where
        T: UninterpretedOptionTrait,
    {
        type Field2MessageType<'this> = T::Field2MessageType<'this>;
        type Field2RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field2RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.name().into_iter())
                .into_iter()
                .flatten()
        }
        type Field3StringType<'this> = T::Field3StringType<'this>;
        fn identifier_value<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3StringType<'this>> {
            self.as_ref().and_then(|msg| msg.identifier_value())
        }
        fn positive_int_value<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.positive_int_value())
        }
        fn negative_int_value<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.negative_int_value())
        }
        fn double_value<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().and_then(|msg| msg.double_value())
        }
        type Field7BytesType<'this> = T::Field7BytesType<'this>;
        fn string_value<'this>(&'this self) -> ::std::option::Option<Self::Field7BytesType<'this>> {
            self.as_ref().and_then(|msg| msg.string_value())
        }
        type Field8StringType<'this> = T::Field8StringType<'this>;
        fn aggregate_value<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8StringType<'this>> {
            self.as_ref().and_then(|msg| msg.aggregate_value())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct UninterpretedOptionSimpleField2 {
    pub name: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>,
}

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleField2 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField2 {
        type Field2MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>>;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.name.iter())
        }
        type Field3StringType<'this> = &'static str;
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOptionSimpleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>
        >::ser_field(&self.name, 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>> for UninterpretedOptionSimpleField2 {
    fn from(value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>) -> Self {
        Self {
            name: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct UninterpretedOptionSimpleField3 {
        pub identifier_value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleField3 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField3 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'this str;
        fn identifier_value<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            self.identifier_value.as_ref().map(|v| v.as_ref())
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOptionSimpleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.identifier_value,
                3,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for UninterpretedOptionSimpleField3
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                identifier_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct UninterpretedOptionSimpleField4 {
        pub positive_int_value: ::std::option::Option<u64>,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleField4 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField4 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        fn positive_int_value<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.positive_int_value)
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOptionSimpleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt64>::ser_field(
                &self.positive_int_value,
                4,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<u64>> for UninterpretedOptionSimpleField4 {
        fn from(value: ::std::option::Option<u64>) -> Self {
            Self {
                positive_int_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct UninterpretedOptionSimpleField5 {
        pub negative_int_value: ::std::option::Option<i64>,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleField5 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField5 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        fn negative_int_value<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.negative_int_value)
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOptionSimpleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int64>::ser_field(
                &self.negative_int_value,
                5,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<i64>> for UninterpretedOptionSimpleField5 {
        fn from(value: ::std::option::Option<i64>) -> Self {
            Self {
                negative_int_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct UninterpretedOptionSimpleField6 {
        pub double_value: ::std::option::Option<f64>,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleField6 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField6 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        fn double_value<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.double_value)
        }
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOptionSimpleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Double>::ser_field(
                &self.double_value,
                6,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<f64>> for UninterpretedOptionSimpleField6 {
        fn from(value: ::std::option::Option<f64>) -> Self {
            Self {
                double_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct UninterpretedOptionSimpleField7 {
        pub string_value: ::std::option::Option<::std::borrow::Cow<'static, [u8]>>,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleField7 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField7 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        type Field7BytesType<'this> = &'this [u8];
        fn string_value<'this>(&'this self) -> Option<Self::Field7BytesType<'this>> {
            self.string_value.as_ref().map(|v| v.as_ref())
        }
        type Field8StringType<'this> = &'static str;
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOptionSimpleField7 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bytes>::ser_field(
                &self.string_value,
                7,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, [u8]>>>
        for UninterpretedOptionSimpleField7
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, [u8]>>) -> Self {
            Self {
                string_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct UninterpretedOptionSimpleField8 {
        pub aggregate_value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleField8 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSimpleField8 {
        type Field2MessageType<'this> = ();
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3StringType<'this> = &'static str;
        type Field7BytesType<'this> = &'static [u8];
        type Field8StringType<'this> = &'this str;
        fn aggregate_value<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            self.aggregate_value.as_ref().map(|v| v.as_ref())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for UninterpretedOptionSimpleField8 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::String>::ser_field(
                &self.aggregate_value,
                8,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
        for UninterpretedOptionSimpleField8
    {
        fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
            Self {
                aggregate_value: value,
            }
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct UninterpretedOptionSimpleByValue {}
    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSimpleByValue {}

    impl UninterpretedOptionTrait for UninterpretedOptionSimpleByValue {
        type Field2MessageType<'this> = ::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>;
        type Field2RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field3StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn identifier_value<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn positive_int_value<'this>(&'this self) -> Option<u64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn negative_int_value<'this>(&'this self) -> Option<i64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        fn double_value<'this>(&'this self) -> Option<f64> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field7BytesType<'this> = ::std::borrow::Cow<'this, [u8]>;
        fn string_value<'this>(&'this self) -> Option<Self::Field7BytesType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
        type Field8StringType<'this> = ::std::borrow::Cow<'this, str>;
        fn aggregate_value<'this>(&'this self) -> Option<Self::Field8StringType<'this>> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct UninterpretedOptionBuilder<T>(T);

    impl<T> UninterpretedOptionBuilder<T>
    where
        T: UninterpretedOptionTrait,
    {
        pub fn append_name(
            self,
            value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSimpleField2)> {
            UninterpretedOptionBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_identifier_value(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSimpleField3)> {
            UninterpretedOptionBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_positive_int_value(
            self,
            value: ::std::option::Option<u64>,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSimpleField4)> {
            UninterpretedOptionBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_negative_int_value(
            self,
            value: ::std::option::Option<i64>,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSimpleField5)> {
            UninterpretedOptionBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_double_value(
            self,
            value: ::std::option::Option<f64>,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSimpleField6)> {
            UninterpretedOptionBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_string_value(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, [u8]>>,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSimpleField7)> {
            UninterpretedOptionBuilder((self.0, ::std::convert::From::from(value)))
        }
        pub fn append_aggregate_value(
            self,
            value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSimpleField8)> {
            UninterpretedOptionBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl UninterpretedOptionBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl SourceCodeInfoTrait for () {
        type Field1MessageType<'this> = ();
        type Field1RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as SourceCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as SourceCodeInfoTrait>::location(t))
                    .map_right(|u| <U as SourceCodeInfoTrait>::location(u)),
            )
        }
    }
    impl<T> SourceCodeInfoTrait for ::std::option::Option<T>
    where
        T: SourceCodeInfoTrait,
    {
        type Field1MessageType<'this> = T::Field1MessageType<'this>;
        type Field1RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field1RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.location().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct SourceCodeInfoSimpleField1 {
    pub location: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>,
}

    impl ::puroro::Message<super::SourceCodeInfo> for SourceCodeInfoSimpleField1 {}

    impl super::_puroro_traits::SourceCodeInfoTrait for SourceCodeInfoSimpleField1 {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location;
        type Field1RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>>;

        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.location.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for SourceCodeInfoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>
        >::ser_field(&self.location, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>> for SourceCodeInfoSimpleField1 {
    fn from(value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>) -> Self {
        Self {
            location: value,
        }
    }
}
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct SourceCodeInfoSimpleByValue {}
    impl ::puroro::Message<super::SourceCodeInfo> for SourceCodeInfoSimpleByValue {}

    impl SourceCodeInfoTrait for SourceCodeInfoSimpleByValue {
        type Field1MessageType<'this> = ::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>;
        type Field1RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct SourceCodeInfoBuilder<T>(T);

    impl<T> SourceCodeInfoBuilder<T>
    where
        T: SourceCodeInfoTrait,
    {
        pub fn append_location(
            self,
            value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>,
        ) -> SourceCodeInfoBuilder<(T, SourceCodeInfoSimpleField1)> {
            SourceCodeInfoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl SourceCodeInfoBuilder<()> {
        pub fn new() -> Self {
            Self(())
        }
    }
    impl GeneratedCodeInfoTrait for () {
        type Field1MessageType<'this> = ();
        type Field1RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
            ::puroro::internal::impls::merged::MergedRepeatedMessageField<
                <T as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
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
            ::puroro::internal::impls::either::EitherRepeatedMessageField<
                <T as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
                <U as GeneratedCodeInfoTrait>::Field1RepeatedType<'this>,
            >;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedMessageField::new(
                self.as_ref()
                    .map_left(|t| <T as GeneratedCodeInfoTrait>::annotation(t))
                    .map_right(|u| <U as GeneratedCodeInfoTrait>::annotation(u)),
            )
        }
    }
    impl<T> GeneratedCodeInfoTrait for ::std::option::Option<T>
    where
        T: GeneratedCodeInfoTrait,
    {
        type Field1MessageType<'this> = T::Field1MessageType<'this>;
        type Field1RepeatedType<'this> = ::std::iter::Flatten<
            ::std::option::IntoIter<
                <T::Field1RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
            >,
        >;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            self.as_ref()
                .map(|msg| msg.annotation().into_iter())
                .into_iter()
                .flatten()
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
    pub struct GeneratedCodeInfoSimpleField1 {
    pub annotation: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>,
}

    impl ::puroro::Message<super::GeneratedCodeInfo> for GeneratedCodeInfoSimpleField1 {}

    impl super::_puroro_traits::GeneratedCodeInfoTrait for GeneratedCodeInfoSimpleField1 {
        type Field1MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation;
        type Field1RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>>;

        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.annotation.iter())
        }
    }

    impl ::puroro::SerializableMessageToIoWrite for GeneratedCodeInfoSimpleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>
        >::ser_field(&self.annotation, 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>> for GeneratedCodeInfoSimpleField1 {
    fn from(value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>) -> Self {
        Self {
            annotation: value,
        }
    }
}
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct GeneratedCodeInfoSimpleByValue {}
    impl ::puroro::Message<super::GeneratedCodeInfo> for GeneratedCodeInfoSimpleByValue {}

    impl GeneratedCodeInfoTrait for GeneratedCodeInfoSimpleByValue {
        type Field1MessageType<'this> = ::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>;
        type Field1RepeatedType<'this> =
            ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            unimplemented!("Please don't use / instantiate this struct!!")
        }
    }
    pub struct GeneratedCodeInfoBuilder<T>(T);

    impl<T> GeneratedCodeInfoBuilder<T>
    where
        T: GeneratedCodeInfoTrait,
    {
        pub fn append_annotation(
            self,
            value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>,
        ) -> GeneratedCodeInfoBuilder<(T, GeneratedCodeInfoSimpleField1)> {
            GeneratedCodeInfoBuilder((self.0, ::std::convert::From::from(value)))
        }

        pub fn build(self) -> T {
            self.0
        }
    }

    impl GeneratedCodeInfoBuilder<()> {
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

    pub trait FileDescriptorSetTrait {
        type Field1MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn package<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field3StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field3StringType<'this>>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
        type Field11RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this>;
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field6MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field7RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this>;
        type Field8MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field9MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn source_code_info<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field12StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field6MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field8MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        type Field7MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field9MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field9RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this>;
        type Field10StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field6StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn type_name<'this>(&'this self) -> ::std::option::Option<Self::Field6StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn extendee<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field7StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn default_value<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7StringType<'this>> {
            ::std::default::Default::default()
        }
        fn oneof_index<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field10StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn json_name<'this>(&'this self) -> ::std::option::Option<Self::Field10StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field8MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::default::Default::default()
        }
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        fn number<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::default::Default::default()
        }
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn name<'this>(&'this self) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field2StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn input_type<'this>(&'this self) -> ::std::option::Option<Self::Field2StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field3StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn output_type<'this>(&'this self) -> ::std::option::Option<Self::Field3StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn java_package<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field1StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field8StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field11StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field36StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn objc_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field36StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field37StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn csharp_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field37StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field39StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn swift_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field39StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field40StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn php_class_prefix<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field40StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field41StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn php_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field41StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field44StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn php_metadata_namespace<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field44StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field45StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn ruby_package<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field45StringType<'this>> {
            ::std::default::Default::default()
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field7BytesType<'this>: ::std::ops::Deref<Target = [u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
        fn string_value<'this>(&'this self) -> ::std::option::Option<Self::Field7BytesType<'this>> {
            ::std::default::Default::default()
        }
        type Field8StringType<'this>: ::std::ops::Deref<Target = str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug;
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
        type Field1MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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
        type Field1MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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

        pub use _puroro_simple_impl::ExtensionRange;
        pub use _puroro_simple_impl::ReservedRange;
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
            pub struct ExtensionRange {
            pub start: ::std::option::Option<i32>,
            pub end: ::std::option::Option<i32>,
            pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>>,
        }
            impl ::puroro::Message<ExtensionRange> for ExtensionRange {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRange {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
                type Field3MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions;
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    self.options.as_ref().map(|v| v.as_ref())
                }
            }

            impl ::puroro::MessageRepresentativeImpl for ExtensionRange {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 3]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "start",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <ExtensionRange as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "end",
                                        number: 2,
                                        lazy_containing_type: Lazy::new(|| {
                                            <ExtensionRange as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "options",
                                        number: 3,
                                        lazy_containing_type: Lazy::new(|| {
                                            <ExtensionRange as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "ExtensionRange",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::DeserializableMessageFromBytesIterator for ExtensionRange {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for ExtensionRange {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
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
                    >::deser_field(&mut self.start, data),
                    2 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Int32
                    >::deser_field(&mut self.end, data),
                    3 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>
                    >::deser_field(&mut self.options, data),
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for ExtensionRange {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.start, 1, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 2, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>
                >::ser_field(&self.options, 3, out)?;

                    ::std::result::Result::Ok(())
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct ReservedRange {
                pub start: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }
            impl ::puroro::Message<ReservedRange> for ReservedRange {}

            impl super::_puroro_traits::ReservedRangeTrait for ReservedRange {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::MessageRepresentativeImpl for ReservedRange {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 2]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "start",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <ReservedRange as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "end",
                                        number: 2,
                                        lazy_containing_type: Lazy::new(|| {
                                            <ReservedRange as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "ReservedRange",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::DeserializableMessageFromBytesIterator for ReservedRange {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for ReservedRange {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.start, data),
                        2 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for ReservedRange {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.start, 1, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 2, out)?;

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
                type Field3MessageType<'this> = (
                    ::std::option::Option<<T as ExtensionRangeTrait>::Field3MessageType<'this>>,
                    ::std::option::Option<<U as ExtensionRangeTrait>::Field3MessageType<'this>>,
                );
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    match (
                        <T as ExtensionRangeTrait>::options(&self.0),
                        <U as ExtensionRangeTrait>::options(&self.1),
                    ) {
                        (None, None) => None,
                        (Some(t), None) => Some((Some(t), None)),
                        (None, Some(u)) => Some((None, Some(u))),
                        (Some(t), Some(u)) => Some((Some(t), Some(u))),
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
            impl<T> ExtensionRangeTrait for ::std::option::Option<T>
            where
                T: ExtensionRangeTrait,
            {
                fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.start())
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end())
                }
                type Field3MessageType<'this> = T::Field3MessageType<'this>;
                fn options<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                    self.as_ref().and_then(|msg| msg.options())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct ExtensionRangeSimpleField1 {
                pub start: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::ExtensionRange> for ExtensionRangeSimpleField1 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSimpleField1 {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                type Field3MessageType<'this> = ();
            }

            impl ::puroro::SerializableMessageToIoWrite for ExtensionRangeSimpleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.start, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for ExtensionRangeSimpleField1 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { start: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct ExtensionRangeSimpleField2 {
                pub end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::ExtensionRange> for ExtensionRangeSimpleField2 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSimpleField2 {
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
                type Field3MessageType<'this> = ();
            }

            impl ::puroro::SerializableMessageToIoWrite for ExtensionRangeSimpleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for ExtensionRangeSimpleField2 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { end: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct ExtensionRangeSimpleField3 {
            pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>>,
        }

            impl ::puroro::Message<super::ExtensionRange> for ExtensionRangeSimpleField3 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSimpleField3 {
                type Field3MessageType<'this> = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions;
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    self.options.as_ref().map(|v| v.as_ref())
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for ExtensionRangeSimpleField3 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>
                >::ser_field(&self.options, 3, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>>> for ExtensionRangeSimpleField3 {
            fn from(value: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>>) -> Self {
                Self {
                    options: value,
                }
            }
        }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct ExtensionRangeSimpleByValue {}
            impl ::puroro::Message<super::ExtensionRange> for ExtensionRangeSimpleByValue {}

            impl ExtensionRangeTrait for ExtensionRangeSimpleByValue {
                fn start<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                type Field3MessageType<'this> = ::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>;
                fn options<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct ExtensionRangeBuilder<T>(T);

            impl<T> ExtensionRangeBuilder<T>
            where
                T: ExtensionRangeTrait,
            {
                pub fn append_start(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> ExtensionRangeBuilder<(T, ExtensionRangeSimpleField1)> {
                    ExtensionRangeBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_end(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> ExtensionRangeBuilder<(T, ExtensionRangeSimpleField2)> {
                    ExtensionRangeBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_options(
                    self,
                    value: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>>,
                ) -> ExtensionRangeBuilder<(T, ExtensionRangeSimpleField3)> {
                    ExtensionRangeBuilder((self.0, ::std::convert::From::from(value)))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl ExtensionRangeBuilder<()> {
                pub fn new() -> Self {
                    Self(())
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
            impl<T> ReservedRangeTrait for ::std::option::Option<T>
            where
                T: ReservedRangeTrait,
            {
                fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.start())
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct ReservedRangeSimpleField1 {
                pub start: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::ReservedRange> for ReservedRangeSimpleField1 {}

            impl super::_puroro_traits::ReservedRangeTrait for ReservedRangeSimpleField1 {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for ReservedRangeSimpleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.start, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for ReservedRangeSimpleField1 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { start: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct ReservedRangeSimpleField2 {
                pub end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::ReservedRange> for ReservedRangeSimpleField2 {}

            impl super::_puroro_traits::ReservedRangeTrait for ReservedRangeSimpleField2 {
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for ReservedRangeSimpleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for ReservedRangeSimpleField2 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { end: value }
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct ReservedRangeSimpleByValue {}
            impl ::puroro::Message<super::ReservedRange> for ReservedRangeSimpleByValue {}

            impl ReservedRangeTrait for ReservedRangeSimpleByValue {
                fn start<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct ReservedRangeBuilder<T>(T);

            impl<T> ReservedRangeBuilder<T>
            where
                T: ReservedRangeTrait,
            {
                pub fn append_start(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> ReservedRangeBuilder<(T, ReservedRangeSimpleField1)> {
                    ReservedRangeBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_end(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> ReservedRangeBuilder<(T, ReservedRangeSimpleField2)> {
                    ReservedRangeBuilder((self.0, ::std::convert::From::from(value)))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl ReservedRangeBuilder<()> {
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

            pub trait ExtensionRangeTrait {
                fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::default::Default::default()
                }
                type Field3MessageType<'this>:
                    self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug;
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

        pub use _puroro_simple_impl::EnumReservedRange;
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
            pub struct EnumReservedRange {
                pub start: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }
            impl ::puroro::Message<EnumReservedRange> for EnumReservedRange {}

            impl super::_puroro_traits::EnumReservedRangeTrait for EnumReservedRange {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::MessageRepresentativeImpl for EnumReservedRange {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 2]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "start",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <EnumReservedRange as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "end",
                                        number: 2,
                                        lazy_containing_type: Lazy::new(|| {
                                            <EnumReservedRange as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "EnumReservedRange",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::DeserializableMessageFromBytesIterator for EnumReservedRange {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for EnumReservedRange {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.start, data),
                        2 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for EnumReservedRange {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.start, 1, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 2, out)?;

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
            impl<T> EnumReservedRangeTrait for ::std::option::Option<T>
            where
                T: EnumReservedRangeTrait,
            {
                fn start<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.start())
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct EnumReservedRangeSimpleField1 {
                pub start: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::EnumReservedRange> for EnumReservedRangeSimpleField1 {}

            impl super::_puroro_traits::EnumReservedRangeTrait for EnumReservedRangeSimpleField1 {
                fn start<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for EnumReservedRangeSimpleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.start, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for EnumReservedRangeSimpleField1 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { start: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct EnumReservedRangeSimpleField2 {
                pub end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::EnumReservedRange> for EnumReservedRangeSimpleField2 {}

            impl super::_puroro_traits::EnumReservedRangeTrait for EnumReservedRangeSimpleField2 {
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for EnumReservedRangeSimpleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for EnumReservedRangeSimpleField2 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { end: value }
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct EnumReservedRangeSimpleByValue {}
            impl ::puroro::Message<super::EnumReservedRange> for EnumReservedRangeSimpleByValue {}

            impl EnumReservedRangeTrait for EnumReservedRangeSimpleByValue {
                fn start<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct EnumReservedRangeBuilder<T>(T);

            impl<T> EnumReservedRangeBuilder<T>
            where
                T: EnumReservedRangeTrait,
            {
                pub fn append_start(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> EnumReservedRangeBuilder<(T, EnumReservedRangeSimpleField1)> {
                    EnumReservedRangeBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_end(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> EnumReservedRangeBuilder<(T, EnumReservedRangeSimpleField2)> {
                    EnumReservedRangeBuilder((self.0, ::std::convert::From::from(value)))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl EnumReservedRangeBuilder<()> {
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

        pub use _puroro_simple_impl::NamePart;
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
            pub struct NamePart {
                pub name_part: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                pub is_extension: ::std::option::Option<bool>,
            }
            impl ::puroro::Message<NamePart> for NamePart {}

            impl super::_puroro_traits::NamePartTrait for NamePart {
                type Field1StringType<'this> = &'this str;
                fn name_part<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.name_part.as_ref().map(|v| v.as_ref())
                }
                fn is_extension<'this>(&'this self) -> Option<bool> {
                    Clone::clone(&self.is_extension)
                }
            }

            impl ::puroro::MessageRepresentativeImpl for NamePart {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 2]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "name_part",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <NamePart as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "is_extension",
                                        number: 2,
                                        lazy_containing_type: Lazy::new(|| {
                                            <NamePart as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "NamePart",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::DeserializableMessageFromBytesIterator for NamePart {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for NamePart {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::String,
                        >::deser_field(&mut self.name_part, data),
                        2 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Required,
                            ::puroro::tags::Bool,
                        >::deser_field(&mut self.is_extension, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for NamePart {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Required, ::puroro::tags::String
                >::ser_field(&self.name_part, 1, out)?;
                    SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Bool>::ser_field(
                        &self.is_extension,
                        2,
                        out,
                    )?;

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
            impl<T> NamePartTrait for ::std::option::Option<T>
            where
                T: NamePartTrait,
            {
                type Field1StringType<'this> = T::Field1StringType<'this>;
                fn name_part<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field1StringType<'this>> {
                    self.as_ref().and_then(|msg| msg.name_part())
                }
                fn is_extension<'this>(&'this self) -> ::std::option::Option<bool> {
                    self.as_ref().and_then(|msg| msg.is_extension())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct NamePartSimpleField1 {
                pub name_part: ::std::option::Option<::std::borrow::Cow<'static, str>>,
            }

            impl ::puroro::Message<super::NamePart> for NamePartSimpleField1 {}

            impl super::_puroro_traits::NamePartTrait for NamePartSimpleField1 {
                type Field1StringType<'this> = &'this str;
                fn name_part<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    self.name_part.as_ref().map(|v| v.as_ref())
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for NamePartSimpleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Required, ::puroro::tags::String
                >::ser_field(&self.name_part, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
                for NamePartSimpleField1
            {
                fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
                    Self { name_part: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct NamePartSimpleField2 {
                pub is_extension: ::std::option::Option<bool>,
            }

            impl ::puroro::Message<super::NamePart> for NamePartSimpleField2 {}

            impl super::_puroro_traits::NamePartTrait for NamePartSimpleField2 {
                type Field1StringType<'this> = &'static str;
                fn is_extension<'this>(&'this self) -> Option<bool> {
                    Clone::clone(&self.is_extension)
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for NamePartSimpleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Bool>::ser_field(
                        &self.is_extension,
                        2,
                        out,
                    )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<bool>> for NamePartSimpleField2 {
                fn from(value: ::std::option::Option<bool>) -> Self {
                    Self {
                        is_extension: value,
                    }
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct NamePartSimpleByValue {}
            impl ::puroro::Message<super::NamePart> for NamePartSimpleByValue {}

            impl NamePartTrait for NamePartSimpleByValue {
                type Field1StringType<'this> = ::std::borrow::Cow<'this, str>;
                fn name_part<'this>(&'this self) -> Option<Self::Field1StringType<'this>> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                fn is_extension<'this>(&'this self) -> Option<bool> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct NamePartBuilder<T>(T);

            impl<T> NamePartBuilder<T>
            where
                T: NamePartTrait,
            {
                pub fn append_name_part(
                    self,
                    value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                ) -> NamePartBuilder<(T, NamePartSimpleField1)> {
                    NamePartBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_is_extension(
                    self,
                    value: ::std::option::Option<bool>,
                ) -> NamePartBuilder<(T, NamePartSimpleField2)> {
                    NamePartBuilder((self.0, ::std::convert::From::from(value)))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl NamePartBuilder<()> {
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

            pub trait NamePartTrait {
                type Field1StringType<'this>: ::std::ops::Deref<Target = str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug;
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

        pub use _puroro_simple_impl::Location;
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
            pub struct Location {
                pub path: ::std::vec::Vec<i32>,
                pub span: ::std::vec::Vec<i32>,
                pub leading_comments: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                pub trailing_comments: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                pub leading_detached_comments: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
            }
            impl ::puroro::Message<Location> for Location {}

            impl super::_puroro_traits::LocationTrait for Location {
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
                type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
                    str,
                    ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
                >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::simple::BorrowedIter::new(
                        self.leading_detached_comments.iter(),
                    )
                }
            }

            impl ::puroro::MessageRepresentativeImpl for Location {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 5]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "path",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Location as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "span",
                                        number: 2,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Location as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "leading_comments",
                                        number: 3,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Location as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "trailing_comments",
                                        number: 4,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Location as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "leading_detached_comments",
                                        number: 6,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Location as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "Location",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::DeserializableMessageFromBytesIterator for Location {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for Location {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.path, data),
                        2 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.span, data),
                        3 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.leading_comments, data
                        ),
                        4 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.trailing_comments, data
                        ),
                        6 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::String,
                        >::deser_field(
                            &mut self.leading_detached_comments, data
                        ),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for Location {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field(&self.path, 1, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field(&self.span, 2, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.leading_comments, 3, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.trailing_comments, 4, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::ser_field(&self.leading_detached_comments, 6, out)?;

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
            impl LocationTrait for () {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }
            impl<T, U> LocationTrait for (T, U)
            where
                T: LocationTrait,
                U: LocationTrait,
            {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::merged::MergedRepeatedField<
                        <T as LocationTrait>::Field1RepeatedType<'this>,
                        <U as LocationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedField::new(
                        <T as LocationTrait>::path(&self.0),
                        <U as LocationTrait>::path(&self.1),
                    )
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::merged::MergedRepeatedField<
                        <T as LocationTrait>::Field2RepeatedType<'this>,
                        <U as LocationTrait>::Field2RepeatedType<'this>,
                    >;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedField::new(
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
                    ::puroro::internal::impls::merged::MergedRepeatedLDField<
                        <T as LocationTrait>::Field6RepeatedType<'this>,
                        <U as LocationTrait>::Field6RepeatedType<'this>,
                    >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedLDField::new(
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
                    ::puroro::internal::impls::either::EitherRepeatedField<
                        <T as LocationTrait>::Field1RepeatedType<'this>,
                        <U as LocationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::either::EitherRepeatedField::new(
                        self.as_ref()
                            .map_left(|t| <T as LocationTrait>::path(t))
                            .map_right(|u| <U as LocationTrait>::path(u)),
                    )
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::either::EitherRepeatedField<
                        <T as LocationTrait>::Field2RepeatedType<'this>,
                        <U as LocationTrait>::Field2RepeatedType<'this>,
                    >;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::either::EitherRepeatedField::new(
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
                    ::puroro::internal::impls::either::EitherRepeatedLDField<
                        <T as LocationTrait>::Field6RepeatedType<'this>,
                        <U as LocationTrait>::Field6RepeatedType<'this>,
                    >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::either::EitherRepeatedLDField::new(
                        self.as_ref()
                            .map_left(|t| <T as LocationTrait>::leading_detached_comments(t))
                            .map_right(|u| <U as LocationTrait>::leading_detached_comments(u)),
                    )
                }
            }
            impl<T> LocationTrait for ::std::option::Option<T>
            where
                T: LocationTrait,
            {
                type Field1RepeatedType<'this> = ::std::iter::Flatten<
                    ::std::option::IntoIter<
                        <T::Field1RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
                    >,
                >;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.as_ref()
                        .map(|msg| msg.path().into_iter())
                        .into_iter()
                        .flatten()
                }
                type Field2RepeatedType<'this> = ::std::iter::Flatten<
                    ::std::option::IntoIter<
                        <T::Field2RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
                    >,
                >;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    self.as_ref()
                        .map(|msg| msg.span().into_iter())
                        .into_iter()
                        .flatten()
                }
                type Field3StringType<'this> = T::Field3StringType<'this>;
                fn leading_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3StringType<'this>> {
                    self.as_ref().and_then(|msg| msg.leading_comments())
                }
                type Field4StringType<'this> = T::Field4StringType<'this>;
                fn trailing_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field4StringType<'this>> {
                    self.as_ref().and_then(|msg| msg.trailing_comments())
                }
                type Field6StringType<'this> = T::Field6StringType<'this>;
                type Field6RepeatedType<'this> = ::std::iter::Flatten<
                    ::std::option::IntoIter<
                        <T::Field6RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
                    >,
                >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    self.as_ref()
                        .map(|msg| msg.leading_detached_comments().into_iter())
                        .into_iter()
                        .flatten()
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct LocationSimpleField1 {
                pub path: ::std::vec::Vec<i32>,
            }

            impl ::puroro::Message<super::Location> for LocationSimpleField1 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField1 {
                type Field1RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.path.iter().cloned()
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for LocationSimpleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field(&self.path, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::vec::Vec<i32>> for LocationSimpleField1 {
                fn from(value: ::std::vec::Vec<i32>) -> Self {
                    Self { path: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct LocationSimpleField2 {
                pub span: ::std::vec::Vec<i32>,
            }

            impl ::puroro::Message<super::Location> for LocationSimpleField2 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField2 {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
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
                    ::puroro::internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for LocationSimpleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field(&self.span, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::vec::Vec<i32>> for LocationSimpleField2 {
                fn from(value: ::std::vec::Vec<i32>) -> Self {
                    Self { span: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct LocationSimpleField3 {
                pub leading_comments: ::std::option::Option<::std::borrow::Cow<'static, str>>,
            }

            impl ::puroro::Message<super::Location> for LocationSimpleField3 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField3 {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'this str;
                fn leading_comments<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
                    self.leading_comments.as_ref().map(|v| v.as_ref())
                }
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for LocationSimpleField3 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.leading_comments, 3, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
                for LocationSimpleField3
            {
                fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
                    Self {
                        leading_comments: value,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct LocationSimpleField4 {
                pub trailing_comments: ::std::option::Option<::std::borrow::Cow<'static, str>>,
            }

            impl ::puroro::Message<super::Location> for LocationSimpleField4 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField4 {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'this str;
                fn trailing_comments<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
                    self.trailing_comments.as_ref().map(|v| v.as_ref())
                }
                type Field6StringType<'this> = &'static str;
                type Field6RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for LocationSimpleField4 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.trailing_comments, 4, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
                for LocationSimpleField4
            {
                fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
                    Self {
                        trailing_comments: value,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct LocationSimpleField6 {
                pub leading_detached_comments: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
            }

            impl ::puroro::Message<super::Location> for LocationSimpleField6 {}

            impl super::_puroro_traits::LocationTrait for LocationSimpleField6 {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field3StringType<'this> = &'static str;
                type Field4StringType<'this> = &'static str;
                type Field6StringType<'this> = &'this str;
                type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
                    str,
                    ::std::slice::Iter<'this, ::std::borrow::Cow<'static, str>>,
                >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::simple::BorrowedIter::new(
                        self.leading_detached_comments.iter(),
                    )
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for LocationSimpleField6 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::ser_field(&self.leading_detached_comments, 6, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::vec::Vec<::std::borrow::Cow<'static, str>>>
                for LocationSimpleField6
            {
                fn from(value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>) -> Self {
                    Self {
                        leading_detached_comments: value,
                    }
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct LocationSimpleByValue {}
            impl ::puroro::Message<super::Location> for LocationSimpleByValue {}

            impl LocationTrait for LocationSimpleByValue {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                type Field2RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                type Field3StringType<'this> = ::std::borrow::Cow<'this, str>;
                fn leading_comments<'this>(&'this self) -> Option<Self::Field3StringType<'this>> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                type Field4StringType<'this> = ::std::borrow::Cow<'this, str>;
                fn trailing_comments<'this>(&'this self) -> Option<Self::Field4StringType<'this>> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                type Field6StringType<'this> = ::std::borrow::Cow<'this, str>;
                type Field6RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<
                        Self::Field6StringType<'this>,
                    >;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct LocationBuilder<T>(T);

            impl<T> LocationBuilder<T>
            where
                T: LocationTrait,
            {
                pub fn append_path(
                    self,
                    value: ::std::vec::Vec<i32>,
                ) -> LocationBuilder<(T, LocationSimpleField1)> {
                    LocationBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_span(
                    self,
                    value: ::std::vec::Vec<i32>,
                ) -> LocationBuilder<(T, LocationSimpleField2)> {
                    LocationBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_leading_comments(
                    self,
                    value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                ) -> LocationBuilder<(T, LocationSimpleField3)> {
                    LocationBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_trailing_comments(
                    self,
                    value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                ) -> LocationBuilder<(T, LocationSimpleField4)> {
                    LocationBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_leading_detached_comments(
                    self,
                    value: ::std::vec::Vec<::std::borrow::Cow<'static, str>>,
                ) -> LocationBuilder<(T, LocationSimpleField6)> {
                    LocationBuilder((self.0, ::std::convert::From::from(value)))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl LocationBuilder<()> {
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

            pub trait LocationTrait {
                type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
                type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
                type Field3StringType<'this>: ::std::ops::Deref<Target = str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug;
                fn leading_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3StringType<'this>> {
                    ::std::default::Default::default()
                }
                type Field4StringType<'this>: ::std::ops::Deref<Target = str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug;
                fn trailing_comments<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field4StringType<'this>> {
                    ::std::default::Default::default()
                }
                type Field6StringType<'this>: ::std::ops::Deref<Target = str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug;
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

        pub use _puroro_simple_impl::Annotation;
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
            pub struct Annotation {
                pub path: ::std::vec::Vec<i32>,
                pub source_file: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                pub begin: ::std::option::Option<i32>,
                pub end: ::std::option::Option<i32>,
            }
            impl ::puroro::Message<Annotation> for Annotation {}

            impl super::_puroro_traits::AnnotationTrait for Annotation {
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

            impl ::puroro::MessageRepresentativeImpl for Annotation {
                fn descriptor() -> &'static ::puroro::desc::MessageDescriptor {
                    use ::puroro::once_cell::sync::Lazy;
                    static LAZY_FIELD_DESCRIPTOR_ARRAY: Lazy<[::puroro::desc::FieldDescriptor; 4]> =
                        Lazy::new(|| {
                            [
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "path",
                                        number: 1,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Annotation as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "source_file",
                                        number: 2,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Annotation as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "begin",
                                        number: 3,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Annotation as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                                {
                                    let init = ::puroro::internal::FieldDescriptorInitializer {
                                        name: "end",
                                        number: 4,
                                        lazy_containing_type: Lazy::new(|| {
                                            <Annotation as ::puroro::MessageRepresentativeImpl>::descriptor()
                                        }),
                                    };
                                    ::puroro::internal::init_field_descriptor(init)
                                },
                            ]
                        });
                    static LAZY_DESCRIPTOR: Lazy<::puroro::desc::MessageDescriptor> =
                        Lazy::new(|| {
                            let init = ::puroro::internal::MessageDescriptorInitializer {
                                name: "Annotation",
                                lazy_fields: Lazy::new(|| {
                                    Lazy::force(&LAZY_FIELD_DESCRIPTOR_ARRAY).as_ref()
                                }),
                            };
                            ::puroro::internal::init_message_descriptor(init)
                        });
                    Lazy::force(&LAZY_DESCRIPTOR)
                }
            }

            impl ::puroro::DeserializableMessageFromBytesIterator for Annotation {
                fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
                }
            }

            impl ::puroro::internal::de::DeserFieldsFromBytesIter for Annotation {
                fn deser_field<I>(
                    &mut self,
                    field_number: i32,
                    data: ::puroro::types::FieldData<
                        &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
                    >,
                ) -> ::puroro::Result<()>
                where
                    I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                {
                    use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
                    match field_number {
                        1 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.path, data),
                        2 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >::deser_field(&mut self.source_file, data),
                        3 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.begin, data),
                        4 => DeserFieldFromBytesIter::<
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >::deser_field(&mut self.end, data),

                        _ => unimplemented!("TODO: This case should be handled properly..."),
                    }
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for Annotation {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field(&self.path, 1, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.source_file, 2, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.begin, 3, out)?;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 4, out)?;

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
            impl AnnotationTrait for () {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'static str;
            }
            impl<T, U> AnnotationTrait for (T, U)
            where
                T: AnnotationTrait,
                U: AnnotationTrait,
            {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::merged::MergedRepeatedField<
                        <T as AnnotationTrait>::Field1RepeatedType<'this>,
                        <U as AnnotationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedField::new(
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
                    ::puroro::internal::impls::either::EitherRepeatedField<
                        <T as AnnotationTrait>::Field1RepeatedType<'this>,
                        <U as AnnotationTrait>::Field1RepeatedType<'this>,
                    >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::either::EitherRepeatedField::new(
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
            impl<T> AnnotationTrait for ::std::option::Option<T>
            where
                T: AnnotationTrait,
            {
                type Field1RepeatedType<'this> = ::std::iter::Flatten<
                    ::std::option::IntoIter<
                        <T::Field1RepeatedType<'this> as ::std::iter::IntoIterator>::IntoIter,
                    >,
                >;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.as_ref()
                        .map(|msg| msg.path().into_iter())
                        .into_iter()
                        .flatten()
                }
                type Field2StringType<'this> = T::Field2StringType<'this>;
                fn source_file<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field2StringType<'this>> {
                    self.as_ref().and_then(|msg| msg.source_file())
                }
                fn begin<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.begin())
                }
                fn end<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct AnnotationSimpleField1 {
                pub path: ::std::vec::Vec<i32>,
            }

            impl ::puroro::Message<super::Annotation> for AnnotationSimpleField1 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField1 {
                type Field1RepeatedType<'this> =
                    ::std::iter::Cloned<::std::slice::Iter<'this, i32>>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    self.path.iter().cloned()
                }
                type Field2StringType<'this> = &'static str;
            }

            impl ::puroro::SerializableMessageToIoWrite for AnnotationSimpleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field(&self.path, 1, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::vec::Vec<i32>> for AnnotationSimpleField1 {
                fn from(value: ::std::vec::Vec<i32>) -> Self {
                    Self { path: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct AnnotationSimpleField2 {
                pub source_file: ::std::option::Option<::std::borrow::Cow<'static, str>>,
            }

            impl ::puroro::Message<super::Annotation> for AnnotationSimpleField2 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField2 {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'this str;
                fn source_file<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    self.source_file.as_ref().map(|v| v.as_ref())
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for AnnotationSimpleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::String
                >::ser_field(&self.source_file, 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<::std::borrow::Cow<'static, str>>>
                for AnnotationSimpleField2
            {
                fn from(value: ::std::option::Option<::std::borrow::Cow<'static, str>>) -> Self {
                    Self { source_file: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct AnnotationSimpleField3 {
                pub begin: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::Annotation> for AnnotationSimpleField3 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField3 {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'static str;
                fn begin<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.begin)
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for AnnotationSimpleField3 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.begin, 3, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for AnnotationSimpleField3 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { begin: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct AnnotationSimpleField4 {
                pub end: ::std::option::Option<i32>,
            }

            impl ::puroro::Message<super::Annotation> for AnnotationSimpleField4 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSimpleField4 {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2StringType<'this> = &'static str;
                fn end<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
            }

            impl ::puroro::SerializableMessageToIoWrite for AnnotationSimpleField4 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::simple::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field(&self.end, 4, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<::std::option::Option<i32>> for AnnotationSimpleField4 {
                fn from(value: ::std::option::Option<i32>) -> Self {
                    Self { end: value }
                }
            }
            #[derive(
                ::std::clone::Clone,
                ::std::default::Default,
                ::std::cmp::PartialEq,
                ::std::fmt::Debug,
            )]
            pub struct AnnotationSimpleByValue {}
            impl ::puroro::Message<super::Annotation> for AnnotationSimpleByValue {}

            impl AnnotationTrait for AnnotationSimpleByValue {
                type Field1RepeatedType<'this> =
                    ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                type Field2StringType<'this> = ::std::borrow::Cow<'this, str>;
                fn source_file<'this>(&'this self) -> Option<Self::Field2StringType<'this>> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                fn begin<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
                fn end<'this>(&'this self) -> Option<i32> {
                    unimplemented!("Please don't use / instantiate this struct!!")
                }
            }
            pub struct AnnotationBuilder<T>(T);

            impl<T> AnnotationBuilder<T>
            where
                T: AnnotationTrait,
            {
                pub fn append_path(
                    self,
                    value: ::std::vec::Vec<i32>,
                ) -> AnnotationBuilder<(T, AnnotationSimpleField1)> {
                    AnnotationBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_source_file(
                    self,
                    value: ::std::option::Option<::std::borrow::Cow<'static, str>>,
                ) -> AnnotationBuilder<(T, AnnotationSimpleField2)> {
                    AnnotationBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_begin(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> AnnotationBuilder<(T, AnnotationSimpleField3)> {
                    AnnotationBuilder((self.0, ::std::convert::From::from(value)))
                }
                pub fn append_end(
                    self,
                    value: ::std::option::Option<i32>,
                ) -> AnnotationBuilder<(T, AnnotationSimpleField4)> {
                    AnnotationBuilder((self.0, ::std::convert::From::from(value)))
                }

                pub fn build(self) -> T {
                    self.0
                }
            }

            impl AnnotationBuilder<()> {
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

            pub trait AnnotationTrait {
                type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
                type Field2StringType<'this>: ::std::ops::Deref<Target = str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug;
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
