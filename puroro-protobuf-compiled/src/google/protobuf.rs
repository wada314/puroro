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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for FileDescriptorSet {
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::FileDescriptorProto>
            >::deser_field(&mut self.file, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileDescriptorSet {
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
        pub name: ::std::option::Option<::std::string::String>,
        pub package: ::std::option::Option<::std::string::String>,
        pub dependency: ::std::vec::Vec<::std::string::String>,
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
        pub syntax: ::std::option::Option<::std::string::String>,
    }
    impl ::puroro::Message<FileDescriptorProto> for FileDescriptorProto {}

    impl super::_puroro_traits::FileDescriptorProtoTrait for FileDescriptorProto {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        fn package_opt<'this>(&'this self) -> Option<&'this str> {
            self.package.as_ref().map(|v| v.as_ref())
        }
        type Field3RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
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
        type Field4MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto;
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto;
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
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceDescriptorProto;
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
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
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo;
        fn source_code_info_opt<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            self.source_code_info.as_ref().map(|v| v.as_ref())
        }
        fn syntax_opt<'this>(&'this self) -> Option<&'this str> {
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for FileDescriptorProto {
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
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions>>
            >::deser_field(&mut self.options, data),
            9 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo>>
            >::deser_field(&mut self.source_code_info, data),
            12 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.syntax, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileDescriptorProto {
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::FileOptions,
                    >,
                >,
            >::ser_field(&self.options, 8, out)?;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Message<
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::SourceCodeInfo,
                    >,
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
    pub name: ::std::option::Option<::std::string::String>,
    pub field: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto>,
    pub extension: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto>,
    pub extension_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions>>,
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
    impl ::puroro::Message<DescriptorProto> for DescriptorProto {}

    impl super::_puroro_traits::DescriptorProtoTrait for DescriptorProto {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
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
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldDescriptorProto;
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::DescriptorProto;
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
        type Field4MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumDescriptorProto;
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
        type Field5MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange;
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ExtensionRange>>;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.extension_range.iter())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofDescriptorProto;
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field9MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange;
        type Field9RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_simple_impl::ReservedRange>>;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field10RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for DescriptorProto {
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
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions>>
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

    impl ::puroro::internal::SerializableMessageToIoWrite for DescriptorProto {
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::MessageOptions,
                    >,
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for ExtensionRangeOptions {
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
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::SerializableMessageToIoWrite for ExtensionRangeOptions {
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
        pub name: ::std::option::Option<::std::string::String>,
        pub number: ::std::option::Option<i32>,
        pub label: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        >,
        pub r#type: ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        >,
        pub type_name: ::std::option::Option<::std::string::String>,
        pub extendee: ::std::option::Option<::std::string::String>,
        pub default_value: ::std::option::Option<::std::string::String>,
        pub oneof_index: ::std::option::Option<i32>,
        pub json_name: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
            >,
        >,
        pub proto3_optional: ::std::option::Option<bool>,
    }
    impl ::puroro::Message<FieldDescriptorProto> for FieldDescriptorProto {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProto {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        fn number_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        fn label_opt<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            Clone::clone(&self.label)
        }
        fn type_opt<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            Clone::clone(&self.r#type)
        }
        fn type_name_opt<'this>(&'this self) -> Option<&'this str> {
            self.type_name.as_ref().map(|v| v.as_ref())
        }
        fn extendee_opt<'this>(&'this self) -> Option<&'this str> {
            self.extendee.as_ref().map(|v| v.as_ref())
        }
        fn default_value_opt<'this>(&'this self) -> Option<&'this str> {
            self.default_value.as_ref().map(|v| v.as_ref())
        }
        fn oneof_index_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.oneof_index)
        }
        fn json_name_opt<'this>(&'this self) -> Option<&'this str> {
            self.json_name.as_ref().map(|v| v.as_ref())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        fn proto3_optional_opt<'this>(&'this self) -> Option<bool> {
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for FieldDescriptorProto {
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
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.number, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label>
            >::deser_field(&mut self.label, data),
            5 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type>
            >::deser_field(&mut self.r#type, data),
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
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions>>
            >::deser_field(&mut self.options, data),
            17 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.proto3_optional, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldDescriptorProto {
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
        >::ser_field(&self.r#type, 5, out)?;
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::FieldOptions,
                    >,
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
        pub name: ::std::option::Option<::std::string::String>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
            >,
        >,
    }
    impl ::puroro::Message<OneofDescriptorProto> for OneofDescriptorProto {}

    impl super::_puroro_traits::OneofDescriptorProtoTrait for OneofDescriptorProto {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for OneofDescriptorProto {
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
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions>>
            >::deser_field(&mut self.options, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for OneofDescriptorProto {
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::OneofOptions,
                    >,
                >,
            >::ser_field(&self.options, 2, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct EnumDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub value: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions>>,
    pub reserved_range: ::std::vec::Vec<self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
    impl ::puroro::Message<EnumDescriptorProto> for EnumDescriptorProto {}

    impl super::_puroro_traits::EnumDescriptorProtoTrait for EnumDescriptorProto {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto;
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange;
        type Field4RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_simple_impl::EnumReservedRange>>;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.reserved_range.iter())
        }
        type Field5RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
            str,
            ::std::slice::Iter<'this, ::std::string::String>,
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for EnumDescriptorProto {
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueDescriptorProto>
            >::deser_field(&mut self.value, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions>>
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

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumDescriptorProto {
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumOptions,
                    >,
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
        pub name: ::std::option::Option<::std::string::String>,
        pub number: ::std::option::Option<i32>,
        pub options: ::std::option::Option<
            ::std::boxed::Box<
                self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
            >,
        >,
    }
    impl ::puroro::Message<EnumValueDescriptorProto> for EnumValueDescriptorProto {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProto {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        fn number_opt<'this>(&'this self) -> Option<i32> {
            Clone::clone(&self.number)
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for EnumValueDescriptorProto {
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
                ::puroro::tags::Optional, ::puroro::tags::Int32
            >::deser_field(&mut self.number, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions>>
            >::deser_field(&mut self.options, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumValueDescriptorProto {
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::EnumValueOptions,
                    >,
                >,
            >::ser_field(&self.options, 3, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct ServiceDescriptorProto {
        pub name: ::std::option::Option<::std::string::String>,
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
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto;
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for ServiceDescriptorProto {
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodDescriptorProto>
            >::deser_field(&mut self.method, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions>>
            >::deser_field(&mut self.options, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for ServiceDescriptorProto {
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::ServiceOptions,
                    >,
                >,
            >::ser_field(&self.options, 3, out)?;

            ::std::result::Result::Ok(())
        }
    }
    #[derive(
        ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq, ::std::fmt::Debug,
    )]
    pub struct MethodDescriptorProto {
        pub name: ::std::option::Option<::std::string::String>,
        pub input_type: ::std::option::Option<::std::string::String>,
        pub output_type: ::std::option::Option<::std::string::String>,
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
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            self.name.as_ref().map(|v| v.as_ref())
        }
        fn input_type_opt<'this>(&'this self) -> Option<&'this str> {
            self.input_type.as_ref().map(|v| v.as_ref())
        }
        fn output_type_opt<'this>(&'this self) -> Option<&'this str> {
            self.output_type.as_ref().map(|v| v.as_ref())
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions;
        fn options_opt<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            self.options.as_ref().map(|v| v.as_ref())
        }
        fn client_streaming_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.client_streaming)
        }
        fn server_streaming_opt<'this>(&'this self) -> Option<bool> {
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for MethodDescriptorProto {
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
            >::deser_field(&mut self.input_type, data),
            3 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::String
            >::deser_field(&mut self.output_type, data),
            4 => DeserFieldFromBytesIter::<
                ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions>>
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

    impl ::puroro::internal::SerializableMessageToIoWrite for MethodDescriptorProto {
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
                    ::std::boxed::Box<
                        self::_puroro_root::google::protobuf::_puroro_simple_impl::MethodOptions,
                    >,
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
            self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption,
        >,
    }
    impl ::puroro::Message<FileOptions> for FileOptions {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptions {
        fn java_package_opt<'this>(&'this self) -> Option<&'this str> {
            self.java_package.as_ref().map(|v| v.as_ref())
        }
        fn java_outer_classname_opt<'this>(&'this self) -> Option<&'this str> {
            self.java_outer_classname.as_ref().map(|v| v.as_ref())
        }
        fn java_multiple_files_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_multiple_files)
        }
        fn java_generate_equals_and_hash_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_generate_equals_and_hash)
        }
        fn java_string_check_utf8_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_string_check_utf8)
        }
        fn optimize_for_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        {
            Clone::clone(&self.optimize_for)
        }
        fn go_package_opt<'this>(&'this self) -> Option<&'this str> {
            self.go_package.as_ref().map(|v| v.as_ref())
        }
        fn cc_generic_services_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.cc_generic_services)
        }
        fn java_generic_services_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.java_generic_services)
        }
        fn py_generic_services_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.py_generic_services)
        }
        fn php_generic_services_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.php_generic_services)
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn cc_enable_arenas_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.cc_enable_arenas)
        }
        fn objc_class_prefix_opt<'this>(&'this self) -> Option<&'this str> {
            self.objc_class_prefix.as_ref().map(|v| v.as_ref())
        }
        fn csharp_namespace_opt<'this>(&'this self) -> Option<&'this str> {
            self.csharp_namespace.as_ref().map(|v| v.as_ref())
        }
        fn swift_prefix_opt<'this>(&'this self) -> Option<&'this str> {
            self.swift_prefix.as_ref().map(|v| v.as_ref())
        }
        fn php_class_prefix_opt<'this>(&'this self) -> Option<&'this str> {
            self.php_class_prefix.as_ref().map(|v| v.as_ref())
        }
        fn php_namespace_opt<'this>(&'this self) -> Option<&'this str> {
            self.php_namespace.as_ref().map(|v| v.as_ref())
        }
        fn php_metadata_namespace_opt<'this>(&'this self) -> Option<&'this str> {
            self.php_metadata_namespace.as_ref().map(|v| v.as_ref())
        }
        fn ruby_package_opt<'this>(&'this self) -> Option<&'this str> {
            self.ruby_package.as_ref().map(|v| v.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for FileOptions {
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

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptions {
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
        fn message_set_wire_format_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.message_set_wire_format)
        }
        fn no_standard_descriptor_accessor_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.no_standard_descriptor_accessor)
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn map_entry_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.map_entry)
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for MessageOptions {
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

    impl ::puroro::internal::SerializableMessageToIoWrite for MessageOptions {
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
        fn ctype_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            Clone::clone(&self.ctype)
        }
        fn packed_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.packed)
        }
        fn jstype_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            Clone::clone(&self.jstype)
        }
        fn lazy_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.lazy)
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn weak_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.weak)
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for FieldOptions {
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

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldOptions {
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for OneofOptions {
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
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::SerializableMessageToIoWrite for OneofOptions {
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
        fn allow_alias_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.allow_alias)
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for EnumOptions {
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
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumOptions {
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
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for EnumValueOptions {
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
                ::puroro::tags::Optional, ::puroro::tags::Bool
            >::deser_field(&mut self.deprecated, data),
            999 => DeserFieldFromBytesIter::<
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption>
            >::deser_field(&mut self.uninterpreted_option, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumValueOptions {
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
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for ServiceOptions {
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
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::SerializableMessageToIoWrite for ServiceOptions {
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
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            Clone::clone(&self.deprecated)
        }
        fn idempotency_level_opt<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            Clone::clone(&self.idempotency_level)
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::UninterpretedOption;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for MethodOptions {
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
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::SerializableMessageToIoWrite for MethodOptions {
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
    pub identifier_value: ::std::option::Option<::std::string::String>,
    pub positive_int_value: ::std::option::Option<u64>,
    pub negative_int_value: ::std::option::Option<i64>,
    pub double_value: ::std::option::Option<f64>,
    pub string_value: ::std::option::Option<::std::vec::Vec<u8>>,
    pub aggregate_value: ::std::option::Option<::std::string::String>,
}
    impl ::puroro::Message<UninterpretedOption> for UninterpretedOption {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOption {
        type Field2MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart;
        type Field2RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
    self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart,
    ::std::slice::Iter<'this, self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_simple_impl::NamePart>>;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::simple::BorrowedIter::new(self.name.iter())
        }
        fn identifier_value_opt<'this>(&'this self) -> Option<&'this str> {
            self.identifier_value.as_ref().map(|v| v.as_ref())
        }
        fn positive_int_value_opt<'this>(&'this self) -> Option<u64> {
            Clone::clone(&self.positive_int_value)
        }
        fn negative_int_value_opt<'this>(&'this self) -> Option<i64> {
            Clone::clone(&self.negative_int_value)
        }
        fn double_value_opt<'this>(&'this self) -> Option<f64> {
            Clone::clone(&self.double_value)
        }
        fn string_value_opt<'this>(&'this self) -> Option<&'this [u8]> {
            self.string_value.as_ref().map(|v| v.as_ref())
        }
        fn aggregate_value_opt<'this>(&'this self) -> Option<&'this str> {
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for UninterpretedOption {
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
            data: ::puroro::internal::types::FieldData<
                &mut ::puroro::internal::de::from_iter::ScopedIter<I>,
            >,
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

    impl ::puroro::internal::SerializableMessageToIoWrite for UninterpretedOption {
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
        type Field1MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for SourceCodeInfo {
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_simple_impl::Location>
            >::deser_field(&mut self.location, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for SourceCodeInfo {
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
        type Field1MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation;
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

    impl ::puroro::internal::DeserializableMessageFromBytesIterator for GeneratedCodeInfo {
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
                ::puroro::tags::Repeated, ::puroro::tags::Message<self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_simple_impl::Annotation>
            >::deser_field(&mut self.annotation, data),

            _ => unimplemented!("TODO: This case should be handled properly..."),
        }
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for GeneratedCodeInfo {
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> FileDescriptorSetTrait for (T, U)
    where
        T: FileDescriptorSetTrait,
        U: FileDescriptorSetTrait,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorSetTrait>::Field1MessageType<'this>,
            <U as FileDescriptorSetTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorSetTrait>::Field1MessageType<'this>,
            <U as FileDescriptorSetTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = T::Field1MessageType<'this>;

        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct FileDescriptorSetSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub file: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FileDescriptorSet>
        for FileDescriptorSetSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FileDescriptorSetTrait
        for FileDescriptorSetSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.file)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorSetSingleField1<ScalarType, RepeatedType>
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
            &self.file,
            1,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorSetSingleField1<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { file: value }
        }
    }
    pub struct FileDescriptorSetBuilder<T>(T);

    impl<T> FileDescriptorSetBuilder<T>
    where
        T: FileDescriptorSetTrait,
    {
        pub fn append_file<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileDescriptorSetBuilder<(T, FileDescriptorSetSingleField1<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            FileDescriptorSetBuilder((self.0, FileDescriptorSetSingleField1 { file: value }))
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
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> FileDescriptorProtoTrait for (T, U)
    where
        T: FileDescriptorProtoTrait,
        U: FileDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileDescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as FileDescriptorProtoTrait>::name_opt(&self.0))
        }
        fn package_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileDescriptorProtoTrait>::package_opt(&self.1)
                .or_else(|| <T as FileDescriptorProtoTrait>::package_opt(&self.0))
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as FileDescriptorProtoTrait>::dependency(&self.0),
                <U as FileDescriptorProtoTrait>::dependency(&self.1),
            )
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as FileDescriptorProtoTrait>::public_dependency(&self.0),
                <U as FileDescriptorProtoTrait>::public_dependency(&self.1),
            )
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field11RepeatedType<'this>,
        >;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
                <T as FileDescriptorProtoTrait>::weak_dependency(&self.0),
                <U as FileDescriptorProtoTrait>::weak_dependency(&self.1),
            )
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field4RepeatedType<'this>,
        >;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::message_type(&self.0),
                <U as FileDescriptorProtoTrait>::message_type(&self.1),
            )
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field5MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::enum_type(&self.0),
                <U as FileDescriptorProtoTrait>::enum_type(&self.1),
            )
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field6MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field6RepeatedType<'this>,
        >;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::service(&self.0),
                <U as FileDescriptorProtoTrait>::service(&self.1),
            )
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field7MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field7MessageType<'this>,
        >;
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field7RepeatedType<'this>,
        >;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as FileDescriptorProtoTrait>::extension(&self.0),
                <U as FileDescriptorProtoTrait>::extension(&self.1),
            )
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as FileDescriptorProtoTrait>::Field8MessageType<'this>>,
            ::std::option::Option<<U as FileDescriptorProtoTrait>::Field8MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            match (
                <T as FileDescriptorProtoTrait>::options_opt(&self.0),
                <U as FileDescriptorProtoTrait>::options_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as FileDescriptorProtoTrait>::Field9MessageType<'this>>,
            ::std::option::Option<<U as FileDescriptorProtoTrait>::Field9MessageType<'this>>,
        );
        fn source_code_info_opt<'this>(&'this self) -> Option<Self::Field9MessageType<'this>> {
            match (
                <T as FileDescriptorProtoTrait>::source_code_info_opt(&self.0),
                <U as FileDescriptorProtoTrait>::source_code_info_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        fn syntax_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileDescriptorProtoTrait>::syntax_opt(&self.1)
                .or_else(|| <T as FileDescriptorProtoTrait>::syntax_opt(&self.0))
        }
    }
    impl<T, U> FileDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: FileDescriptorProtoTrait,
        U: FileDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileDescriptorProtoTrait>::name_opt(t),
                |u| <U as FileDescriptorProtoTrait>::name_opt(u),
            )
        }
        fn package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileDescriptorProtoTrait>::package_opt(t),
                |u| <U as FileDescriptorProtoTrait>::package_opt(u),
            )
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
            <U as FileDescriptorProtoTrait>::Field3RepeatedType<'this>,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
                self.as_ref()
                    .map_left(|t| <T as FileDescriptorProtoTrait>::dependency(t))
                    .map_right(|u| <U as FileDescriptorProtoTrait>::dependency(u)),
            )
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
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
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
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
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field5MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field6MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field7MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field7MessageType<'this>,
        >;
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field8MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FileDescriptorProtoTrait>::options_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FileDescriptorProtoTrait>::options_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileDescriptorProtoTrait>::Field9MessageType<'this>,
            <U as FileDescriptorProtoTrait>::Field9MessageType<'this>,
        >;
        fn source_code_info_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FileDescriptorProtoTrait>::source_code_info_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FileDescriptorProtoTrait>::source_code_info_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn syntax_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileDescriptorProtoTrait>::syntax_opt(t),
                |u| <U as FileDescriptorProtoTrait>::syntax_opt(u),
            )
        }
    }
    impl<T> FileDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: FileDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        fn package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.package_opt())
        }

        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field4MessageType<'this>
        where
            Self: 'this,
        = T::Field4MessageType<'this>;

        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = T::Field5MessageType<'this>;

        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field6MessageType<'this>
        where
            Self: 'this,
        = T::Field6MessageType<'this>;

        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = T::Field7MessageType<'this>;

        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field8MessageType<'this>
        where
            Self: 'this,
        = T::Field8MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = T::Field9MessageType<'this>;
        fn source_code_info_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.source_code_info_opt())
        }
        fn syntax_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.syntax_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileDescriptorProtoSingleField1<ScalarType>
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

    pub struct FileDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub package: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.package.as_ref())
        }
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField2<ScalarType>
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
            >(::std::iter::once(&self.package), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { package: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub dependency: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::AsRefIter<
            <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
            str,
        >;

        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::single_field::AsRefIter::new(
                ::std::iter::IntoIterator::into_iter(&self.dependency),
            )
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField3<ScalarType, RepeatedType>
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
            >(&self.dependency, 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { dependency: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField10<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        pub public_dependency: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField10<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
    }

    impl<RepeatedType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField10<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(
                &self.public_dependency,
            ))
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField10<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.public_dependency, 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorProtoSingleField10<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                public_dependency: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField11<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        pub weak_dependency: RepeatedType,
    }

    impl<RepeatedType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField11<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
    }

    impl<RepeatedType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField11<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(
                &self.weak_dependency,
            ))
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField11<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Repeated, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(&self.weak_dependency, 11, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorProtoSingleField11<RepeatedType>
    where
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                weak_dependency: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub message_type: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.message_type)
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
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
            &self.message_type,
            4,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                message_type: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub enum_type: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.enum_type)
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
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
            &self.enum_type,
            5,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { enum_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub service: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.service)
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait
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
            &self.service,
            6,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { service: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField7<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub extension: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField7<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField7<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.extension)
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField7<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
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
            &self.extension,
            7,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FileDescriptorProtoSingleField7<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { extension: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait
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
            ::std::iter::once(&self.options),
            8,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField9<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub source_code_info: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField9<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField9<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn source_code_info_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            ::std::option::Option::Some(&self.source_code_info)
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField9<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait
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
            ::std::iter::once(&self.source_code_info),
            9,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileDescriptorProtoSingleField9<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                source_code_info: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileDescriptorProtoSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub syntax: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileDescriptorProto>
        for FileDescriptorProtoSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FileDescriptorProtoTrait
        for FileDescriptorProtoSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field11RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field7RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field7MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();

        fn syntax_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.syntax.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileDescriptorProtoSingleField12<ScalarType>
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
            >(::std::iter::once(&self.syntax), 12, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileDescriptorProtoSingleField12<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { syntax: value }
        }
    }
    pub struct FileDescriptorProtoBuilder<T>(T);

    impl<T> FileDescriptorProtoBuilder<T>
    where
        T: FileDescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileDescriptorProtoBuilder((self.0, FileDescriptorProtoSingleField1 { name: value }))
        }

        pub fn append_package<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileDescriptorProtoBuilder((self.0, FileDescriptorProtoSingleField2 { package: value }))
        }

        pub fn append_dependency<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileDescriptorProtoBuilder<(
            T,
            FileDescriptorProtoSingleField3<ScalarType, RepeatedType>,
        )>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            FileDescriptorProtoBuilder((
                self.0,
                FileDescriptorProtoSingleField3 { dependency: value },
            ))
        }

        pub fn append_public_dependency<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField10<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
        {
            FileDescriptorProtoBuilder((
                self.0,
                FileDescriptorProtoSingleField10 {
                    public_dependency: value,
                },
            ))
        }

        pub fn append_weak_dependency<RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField11<RepeatedType>)>
        where
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
        {
            FileDescriptorProtoBuilder((
                self.0,
                FileDescriptorProtoSingleField11 {
                    weak_dependency: value,
                },
            ))
        }

        pub fn append_message_type<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileDescriptorProtoBuilder<(
            T,
            FileDescriptorProtoSingleField4<ScalarType, RepeatedType>,
        )>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            FileDescriptorProtoBuilder((
                self.0,
                FileDescriptorProtoSingleField4 {
                    message_type: value,
                },
            ))
        }

        pub fn append_enum_type<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileDescriptorProtoBuilder<(
            T,
            FileDescriptorProtoSingleField5<ScalarType, RepeatedType>,
        )>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            FileDescriptorProtoBuilder((
                self.0,
                FileDescriptorProtoSingleField5 { enum_type: value },
            ))
        }

    pub fn append_service<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField6<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            FileDescriptorProtoBuilder((self.0, FileDescriptorProtoSingleField6 { service: value }))
        }

        pub fn append_extension<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileDescriptorProtoBuilder<(
            T,
            FileDescriptorProtoSingleField7<ScalarType, RepeatedType>,
        )>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            FileDescriptorProtoBuilder((
                self.0,
                FileDescriptorProtoSingleField7 { extension: value },
            ))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField8<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileDescriptorProtoBuilder((self.0, FileDescriptorProtoSingleField8 { options: value }))
        }

        pub fn append_source_code_info<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField9<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileDescriptorProtoBuilder((
                self.0,
                FileDescriptorProtoSingleField9 {
                    source_code_info: value,
                },
            ))
        }

        pub fn append_syntax<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileDescriptorProtoBuilder<(T, FileDescriptorProtoSingleField12<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileDescriptorProtoBuilder((self.0, FileDescriptorProtoSingleField12 { syntax: value }))
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
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> DescriptorProtoTrait for (T, U)
    where
        T: DescriptorProtoTrait,
        U: DescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as DescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as DescriptorProtoTrait>::name_opt(&self.0))
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field2MessageType<'this>,
            <U as DescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as DescriptorProtoTrait>::Field2RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field2RepeatedType<'this>,
        >;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::field(&self.0),
                <U as DescriptorProtoTrait>::field(&self.1),
            )
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field6MessageType<'this>,
            <U as DescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as DescriptorProtoTrait>::Field6RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field6RepeatedType<'this>,
        >;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::extension(&self.0),
                <U as DescriptorProtoTrait>::extension(&self.1),
            )
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field3MessageType<'this>,
            <U as DescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as DescriptorProtoTrait>::Field3RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field3RepeatedType<'this>,
        >;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::nested_type(&self.0),
                <U as DescriptorProtoTrait>::nested_type(&self.1),
            )
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field4MessageType<'this>,
            <U as DescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as DescriptorProtoTrait>::Field4RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field4RepeatedType<'this>,
        >;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::enum_type(&self.0),
                <U as DescriptorProtoTrait>::enum_type(&self.1),
            )
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field5MessageType<'this>,
            <U as DescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as DescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::extension_range(&self.0),
                <U as DescriptorProtoTrait>::extension_range(&self.1),
            )
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field8MessageType<'this>,
            <U as DescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as DescriptorProtoTrait>::Field8RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field8RepeatedType<'this>,
        >;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::oneof_decl(&self.0),
                <U as DescriptorProtoTrait>::oneof_decl(&self.1),
            )
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as DescriptorProtoTrait>::Field7MessageType<'this>>,
            ::std::option::Option<<U as DescriptorProtoTrait>::Field7MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field7MessageType<'this>> {
            match (
                <T as DescriptorProtoTrait>::options_opt(&self.0),
                <U as DescriptorProtoTrait>::options_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field9MessageType<'this>,
            <U as DescriptorProtoTrait>::Field9MessageType<'this>,
        >;
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as DescriptorProtoTrait>::Field9RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field9RepeatedType<'this>,
        >;

        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as DescriptorProtoTrait>::reserved_range(&self.0),
                <U as DescriptorProtoTrait>::reserved_range(&self.1),
            )
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as DescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
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
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as DescriptorProtoTrait>::name_opt(t),
                |u| <U as DescriptorProtoTrait>::name_opt(u),
            )
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field2MessageType<'this>,
            <U as DescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field6MessageType<'this>,
            <U as DescriptorProtoTrait>::Field6MessageType<'this>,
        >;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field3MessageType<'this>,
            <U as DescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field4MessageType<'this>,
            <U as DescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field5MessageType<'this>,
            <U as DescriptorProtoTrait>::Field5MessageType<'this>,
        >;
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field8MessageType<'this>,
            <U as DescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field7MessageType<'this>,
            <U as DescriptorProtoTrait>::Field7MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            self.as_ref().either(
                |t| <T as DescriptorProtoTrait>::options_opt(t).map(|t| ::puroro::Either::Left(t)),
                |u| <U as DescriptorProtoTrait>::options_opt(u).map(|u| ::puroro::Either::Right(u)),
            )
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as DescriptorProtoTrait>::Field9MessageType<'this>,
            <U as DescriptorProtoTrait>::Field9MessageType<'this>,
        >;
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as DescriptorProtoTrait>::Field10RepeatedType<'this>,
            <U as DescriptorProtoTrait>::Field10RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
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
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = T::Field2MessageType<'this>;

        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field6MessageType<'this>
        where
            Self: 'this,
        = T::Field6MessageType<'this>;

        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = T::Field3MessageType<'this>;

        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field4MessageType<'this>
        where
            Self: 'this,
        = T::Field4MessageType<'this>;

        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field5MessageType<'this>
        where
            Self: 'this,
        = T::Field5MessageType<'this>;

        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field8MessageType<'this>
        where
            Self: 'this,
        = T::Field8MessageType<'this>;

        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field7MessageType<'this>
        where
            Self: 'this,
        = T::Field7MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = T::Field9MessageType<'this>;

        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct DescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for DescriptorProtoSingleField1<ScalarType>
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

    pub struct DescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub field: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.field)
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
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
            &self.field,
            2,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for DescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { field: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub extension: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.extension)
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
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
            &self.extension,
            6,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for DescriptorProtoSingleField6<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { extension: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub nested_type: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.nested_type)
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
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
            &self.nested_type,
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for DescriptorProtoSingleField3<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { nested_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub enum_type: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.enum_type)
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
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
            &self.enum_type,
            4,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for DescriptorProtoSingleField4<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { enum_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField5<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    pub extension_range: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
for DescriptorProtoSingleField5<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
for DescriptorProtoSingleField5<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
type Field2MessageType<'this> where Self: 'this = ();
type Field2RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field6MessageType<'this> where Self: 'this = ();
type Field6RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field3MessageType<'this> where Self: 'this = ();
type Field3RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field4MessageType<'this> where Self: 'this = ();
type Field4RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field5MessageType<'this> where Self: 'this = &'this ScalarType;
type Field5RepeatedType<'this> where Self: 'this
    = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
    ::std::iter::IntoIterator::into_iter(&self.extension_range)
}
type Field8MessageType<'this> where Self: 'this = ();
type Field8RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field7MessageType<'this> where Self: 'this = ();
type Field9MessageType<'this> where Self: 'this = ();
type Field9RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field10RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
for DescriptorProtoSingleField5<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
            &self.extension_range,
            5,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for DescriptorProtoSingleField5<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            extension_range: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField8<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub oneof_decl: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField8<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField8<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.oneof_decl)
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField8<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait
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
            &self.oneof_decl,
            8,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for DescriptorProtoSingleField8<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { oneof_decl: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait
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
            ::std::iter::once(&self.options),
            7,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for DescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField9<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    pub reserved_range: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
for DescriptorProtoSingleField9<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
for DescriptorProtoSingleField9<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
type Field2MessageType<'this> where Self: 'this = ();
type Field2RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field6MessageType<'this> where Self: 'this = ();
type Field6RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field3MessageType<'this> where Self: 'this = ();
type Field3RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field4MessageType<'this> where Self: 'this = ();
type Field4RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field5MessageType<'this> where Self: 'this = ();
type Field5RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field8MessageType<'this> where Self: 'this = ();
type Field8RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field7MessageType<'this> where Self: 'this = ();
type Field9MessageType<'this> where Self: 'this = &'this ScalarType;
type Field9RepeatedType<'this> where Self: 'this
    = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
    ::std::iter::IntoIterator::into_iter(&self.reserved_range)
}
type Field10RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
for DescriptorProtoSingleField9<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
            &self.reserved_range,
            9,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for DescriptorProtoSingleField9<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            reserved_range: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct DescriptorProtoSingleField10<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub reserved_name: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::DescriptorProto>
        for DescriptorProtoSingleField10<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::DescriptorProtoTrait
        for DescriptorProtoSingleField10<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field6MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field6RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field6MessageType<'this>>;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field3RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field3MessageType<'this>>;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field5MessageType<'this>>;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field8RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field8MessageType<'this>>;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field7MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field9RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field9MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field10RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::AsRefIter<
            <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
            str,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
            ::puroro::internal::impls::single_field::AsRefIter::new(
                ::std::iter::IntoIterator::into_iter(&self.reserved_name),
            )
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for DescriptorProtoSingleField10<ScalarType, RepeatedType>
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
            >(&self.reserved_name, 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for DescriptorProtoSingleField10<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                reserved_name: value,
            }
        }
    }
    pub struct DescriptorProtoBuilder<T>(T);

    impl<T> DescriptorProtoBuilder<T>
    where
        T: DescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            DescriptorProtoBuilder((self.0, DescriptorProtoSingleField1 { name: value }))
        }

        pub fn append_field<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField2<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            DescriptorProtoBuilder((self.0, DescriptorProtoSingleField2 { field: value }))
        }

        pub fn append_extension<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField6<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            DescriptorProtoBuilder((self.0, DescriptorProtoSingleField6 { extension: value }))
        }

        pub fn append_nested_type<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField3<ScalarType, RepeatedType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            DescriptorProtoBuilder((self.0, DescriptorProtoSingleField3 { nested_type: value }))
        }

        pub fn append_enum_type<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField4<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            DescriptorProtoBuilder((self.0, DescriptorProtoSingleField4 { enum_type: value }))
        }

    pub fn append_extension_range<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField5<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            DescriptorProtoBuilder((
                self.0,
                DescriptorProtoSingleField5 {
                    extension_range: value,
                },
            ))
        }

        pub fn append_oneof_decl<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField8<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            DescriptorProtoBuilder((self.0, DescriptorProtoSingleField8 { oneof_decl: value }))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField7<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            DescriptorProtoBuilder((self.0, DescriptorProtoSingleField7 { options: value }))
        }

    pub fn append_reserved_range<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField9<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            DescriptorProtoBuilder((
                self.0,
                DescriptorProtoSingleField9 {
                    reserved_range: value,
                },
            ))
        }

        pub fn append_reserved_name<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> DescriptorProtoBuilder<(T, DescriptorProtoSingleField10<ScalarType, RepeatedType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            DescriptorProtoBuilder((
                self.0,
                DescriptorProtoSingleField10 {
                    reserved_name: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> ExtensionRangeOptionsTrait for (T, U)
    where
        T: ExtensionRangeOptionsTrait,
        U: ExtensionRangeOptionsTrait,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
            <U as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
            <U as ExtensionRangeOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct ExtensionRangeOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::ExtensionRangeOptions>
        for ExtensionRangeOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::ExtensionRangeOptionsTrait
        for ExtensionRangeOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for ExtensionRangeOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for ExtensionRangeOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct ExtensionRangeOptionsBuilder<T>(T);

    impl<T> ExtensionRangeOptionsBuilder<T>
    where
        T: ExtensionRangeOptionsTrait,
    {
        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> ExtensionRangeOptionsBuilder<(
            T,
            ExtensionRangeOptionsSingleField999<ScalarType, RepeatedType>,
        )>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            ExtensionRangeOptionsBuilder((
                self.0,
                ExtensionRangeOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> FieldDescriptorProtoTrait for (T, U)
    where
        T: FieldDescriptorProtoTrait,
        U: FieldDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FieldDescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::name_opt(&self.0))
        }
        fn number_opt<'this>(&'this self) -> Option<i32> {
            <U as FieldDescriptorProtoTrait>::number_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::number_opt(&self.0))
        }
        fn label_opt<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            <U as FieldDescriptorProtoTrait>::label_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::label_opt(&self.0))
        }
        fn type_opt<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            <U as FieldDescriptorProtoTrait>::type_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::type_opt(&self.0))
        }
        fn type_name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FieldDescriptorProtoTrait>::type_name_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::type_name_opt(&self.0))
        }
        fn extendee_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FieldDescriptorProtoTrait>::extendee_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::extendee_opt(&self.0))
        }
        fn default_value_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FieldDescriptorProtoTrait>::default_value_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::default_value_opt(&self.0))
        }
        fn oneof_index_opt<'this>(&'this self) -> Option<i32> {
            <U as FieldDescriptorProtoTrait>::oneof_index_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::oneof_index_opt(&self.0))
        }
        fn json_name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FieldDescriptorProtoTrait>::json_name_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::json_name_opt(&self.0))
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as FieldDescriptorProtoTrait>::Field8MessageType<'this>>,
            ::std::option::Option<<U as FieldDescriptorProtoTrait>::Field8MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field8MessageType<'this>> {
            match (
                <T as FieldDescriptorProtoTrait>::options_opt(&self.0),
                <U as FieldDescriptorProtoTrait>::options_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        fn proto3_optional_opt<'this>(&'this self) -> Option<bool> {
            <U as FieldDescriptorProtoTrait>::proto3_optional_opt(&self.1)
                .or_else(|| <T as FieldDescriptorProtoTrait>::proto3_optional_opt(&self.0))
        }
    }
    impl<T, U> FieldDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: FieldDescriptorProtoTrait,
        U: FieldDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::name_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::name_opt(u),
            )
        }
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::number_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::number_opt(u),
            )
        }
        fn label_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::label_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::label_opt(u),
            )
        }
        fn type_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::type_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::type_opt(u),
            )
        }
        fn type_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::type_name_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::type_name_opt(u),
            )
        }
        fn extendee_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::extendee_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::extendee_opt(u),
            )
        }
        fn default_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::default_value_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::default_value_opt(u),
            )
        }
        fn oneof_index_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::oneof_index_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::oneof_index_opt(u),
            )
        }
        fn json_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::json_name_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::json_name_opt(u),
            )
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
            <U as FieldDescriptorProtoTrait>::Field8MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as FieldDescriptorProtoTrait>::options_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as FieldDescriptorProtoTrait>::options_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn proto3_optional_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FieldDescriptorProtoTrait>::proto3_optional_opt(t),
                |u| <U as FieldDescriptorProtoTrait>::proto3_optional_opt(u),
            )
        }
    }
    impl<T> FieldDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: FieldDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.number_opt())
        }
        fn label_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            self.as_ref().and_then(|msg| msg.label_opt())
        }
        fn type_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            self.as_ref().and_then(|msg| msg.type_opt())
        }
        fn type_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.type_name_opt())
        }
        fn extendee_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.extendee_opt())
        }
        fn default_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.default_value_opt())
        }
        fn oneof_index_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.oneof_index_opt())
        }
        fn json_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.json_name_opt())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = T::Field8MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
        fn proto3_optional_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.proto3_optional_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FieldDescriptorProto>
        for FieldDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FieldDescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FieldDescriptorProtoSingleField1<ScalarType>
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

    pub struct FieldDescriptorProtoSingleField3 {
        pub number: i32,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSingleField3 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSingleField3 {
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.number))
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldDescriptorProtoSingleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.number), 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for FieldDescriptorProtoSingleField3 {
        fn from(value: i32) -> Self {
            Self { number: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField4 {
        pub label:
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSingleField4 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSingleField4 {
        fn label_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.label))
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldDescriptorProtoSingleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label>
        >::ser_field::
        <(), _, _>
        (
            ::std::iter::once(&self.label),
            4,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > for FieldDescriptorProtoSingleField4
    {
        fn from(
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        ) -> Self {
            Self { label: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField5 {
        pub r#type:
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSingleField5 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSingleField5 {
        fn type_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.r#type))
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldDescriptorProtoSingleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type>
        >::ser_field::
        <(), _, _>
        (
            ::std::iter::once(&self.r#type),
            5,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > for FieldDescriptorProtoSingleField5
    {
        fn from(
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        ) -> Self {
            Self { r#type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub type_name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FieldDescriptorProto>
        for FieldDescriptorProtoSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProtoSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn type_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.type_name.as_ref())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FieldDescriptorProtoSingleField6<ScalarType>
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
            >(::std::iter::once(&self.type_name), 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FieldDescriptorProtoSingleField6<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { type_name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub extendee: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FieldDescriptorProto>
        for FieldDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn extendee_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.extendee.as_ref())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FieldDescriptorProtoSingleField2<ScalarType>
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
            >(::std::iter::once(&self.extendee), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FieldDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { extendee: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub default_value: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FieldDescriptorProto>
        for FieldDescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn default_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.default_value.as_ref())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FieldDescriptorProtoSingleField7<ScalarType>
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
            >(::std::iter::once(&self.default_value), 7, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FieldDescriptorProtoSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                default_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField9 {
        pub oneof_index: i32,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSingleField9 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSingleField9 {
        fn oneof_index_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.oneof_index))
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldDescriptorProtoSingleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.oneof_index), 9, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for FieldDescriptorProtoSingleField9 {
        fn from(value: i32) -> Self {
            Self { oneof_index: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField10<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub json_name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FieldDescriptorProto>
        for FieldDescriptorProtoSingleField10<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProtoSingleField10<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn json_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.json_name.as_ref())
        }
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FieldDescriptorProtoSingleField10<ScalarType>
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
            >(::std::iter::once(&self.json_name), 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FieldDescriptorProtoSingleField10<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { json_name: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FieldDescriptorProto>
        for FieldDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::FieldDescriptorProtoTrait
        for FieldDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field8MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FieldDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait
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
            ::std::iter::once(&self.options),
            8,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FieldDescriptorProtoSingleField8<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldDescriptorProtoSingleField17 {
        pub proto3_optional: bool,
    }

    impl ::puroro::Message<super::FieldDescriptorProto> for FieldDescriptorProtoSingleField17 {}

    impl super::_puroro_traits::FieldDescriptorProtoTrait for FieldDescriptorProtoSingleField17 {
        type Field8MessageType<'this>
        where
            Self: 'this,
        = ();

        fn proto3_optional_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.proto3_optional))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldDescriptorProtoSingleField17 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.proto3_optional), 17, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FieldDescriptorProtoSingleField17 {
        fn from(value: bool) -> Self {
            Self {
                proto3_optional: value,
            }
        }
    }
    pub struct FieldDescriptorProtoBuilder<T>(T);

    impl<T> FieldDescriptorProtoBuilder<T>
    where
        T: FieldDescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FieldDescriptorProtoBuilder((self.0, FieldDescriptorProtoSingleField1 { name: value }))
        }

        pub fn append_number(
            self,
            value: i32,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField3)> {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField3 { number: value },
            ))
        }

        pub fn append_label(
            self,
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField4)> {
            FieldDescriptorProtoBuilder((self.0, FieldDescriptorProtoSingleField4 { label: value }))
        }

        pub fn append_type(
            self,
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField5)> {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField5 { r#type: value },
            ))
        }

        pub fn append_type_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField6<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField6 { type_name: value },
            ))
        }

        pub fn append_extendee<ScalarType>(
            self,
            value: ScalarType,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField2 { extendee: value },
            ))
        }

        pub fn append_default_value<ScalarType>(
            self,
            value: ScalarType,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField7<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField7 {
                    default_value: value,
                },
            ))
        }

        pub fn append_oneof_index(
            self,
            value: i32,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField9)> {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField9 { oneof_index: value },
            ))
        }

        pub fn append_json_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField10<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField10 { json_name: value },
            ))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField8<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField8 { options: value },
            ))
        }

        pub fn append_proto3_optional(
            self,
            value: bool,
        ) -> FieldDescriptorProtoBuilder<(T, FieldDescriptorProtoSingleField17)> {
            FieldDescriptorProtoBuilder((
                self.0,
                FieldDescriptorProtoSingleField17 {
                    proto3_optional: value,
                },
            ))
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
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> OneofDescriptorProtoTrait for (T, U)
    where
        T: OneofDescriptorProtoTrait,
        U: OneofDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as OneofDescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as OneofDescriptorProtoTrait>::name_opt(&self.0))
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as OneofDescriptorProtoTrait>::Field2MessageType<'this>>,
            ::std::option::Option<<U as OneofDescriptorProtoTrait>::Field2MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field2MessageType<'this>> {
            match (
                <T as OneofDescriptorProtoTrait>::options_opt(&self.0),
                <U as OneofDescriptorProtoTrait>::options_opt(&self.1),
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
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as OneofDescriptorProtoTrait>::name_opt(t),
                |u| <U as OneofDescriptorProtoTrait>::name_opt(u),
            )
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as OneofDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field2MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as OneofDescriptorProtoTrait>::options_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as OneofDescriptorProtoTrait>::options_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }
    impl<T> OneofDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: OneofDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = T::Field2MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field2MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct OneofDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::OneofDescriptorProto>
        for OneofDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::OneofDescriptorProtoTrait
        for OneofDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for OneofDescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for OneofDescriptorProtoSingleField1<ScalarType>
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

    pub struct OneofDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::OneofDescriptorProto>
        for OneofDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::OneofDescriptorProtoTrait
        for OneofDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field2MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for OneofDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait
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
            ::std::iter::once(&self.options),
            2,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for OneofDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }
    pub struct OneofDescriptorProtoBuilder<T>(T);

    impl<T> OneofDescriptorProtoBuilder<T>
    where
        T: OneofDescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> OneofDescriptorProtoBuilder<(T, OneofDescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            OneofDescriptorProtoBuilder((self.0, OneofDescriptorProtoSingleField1 { name: value }))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> OneofDescriptorProtoBuilder<(T, OneofDescriptorProtoSingleField2<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            OneofDescriptorProtoBuilder((
                self.0,
                OneofDescriptorProtoSingleField2 { options: value },
            ))
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
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> EnumDescriptorProtoTrait for (T, U)
    where
        T: EnumDescriptorProtoTrait,
        U: EnumDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as EnumDescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as EnumDescriptorProtoTrait>::name_opt(&self.0))
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field2RepeatedType<'this>,
        >;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumDescriptorProtoTrait>::value(&self.0),
                <U as EnumDescriptorProtoTrait>::value(&self.1),
            )
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as EnumDescriptorProtoTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as EnumDescriptorProtoTrait>::Field3MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as EnumDescriptorProtoTrait>::options_opt(&self.0),
                <U as EnumDescriptorProtoTrait>::options_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field4RepeatedType<'this>,
        >;

        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as EnumDescriptorProtoTrait>::reserved_range(&self.0),
                <U as EnumDescriptorProtoTrait>::reserved_range(&self.1),
            )
        }
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedField<
            <T as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedField::new(
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
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as EnumDescriptorProtoTrait>::name_opt(t),
                |u| <U as EnumDescriptorProtoTrait>::name_opt(u),
            )
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as EnumDescriptorProtoTrait>::options_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as EnumDescriptorProtoTrait>::options_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as EnumDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedField<
            <T as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
            <U as EnumDescriptorProtoTrait>::Field5RepeatedType<'this>,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::either::EitherRepeatedField::new(
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
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = T::Field2MessageType<'this>;

        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = T::Field3MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = T::Field4MessageType<'this>;

        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct EnumDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::EnumDescriptorProto>
        for EnumDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::EnumDescriptorProtoTrait
        for EnumDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumDescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for EnumDescriptorProtoSingleField1<ScalarType>
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

    pub struct EnumDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub value: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::EnumDescriptorProto>
        for EnumDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::EnumDescriptorProtoTrait
        for EnumDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.value)
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait
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
            &self.value,
            2,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for EnumDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { value: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::EnumDescriptorProto>
        for EnumDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::EnumDescriptorProtoTrait
        for EnumDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait
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
            ::std::iter::once(&self.options),
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for EnumDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumDescriptorProtoSingleField4<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    pub reserved_range: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::EnumDescriptorProto>
for EnumDescriptorProtoSingleField4<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::EnumDescriptorProtoTrait
for EnumDescriptorProtoSingleField4<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
type Field2MessageType<'this> where Self: 'this = ();
type Field2RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
type Field3MessageType<'this> where Self: 'this = ();
type Field4MessageType<'this> where Self: 'this = &'this ScalarType;
type Field4RepeatedType<'this> where Self: 'this
    = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
    ::std::iter::IntoIterator::into_iter(&self.reserved_range)
}
type Field5RepeatedType<'this> where Self: 'this = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
for EnumDescriptorProtoSingleField4<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
            &self.reserved_range,
            4,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for EnumDescriptorProtoSingleField4<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            reserved_range: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub reserved_name: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::EnumDescriptorProto>
        for EnumDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::EnumDescriptorProtoTrait
        for EnumDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field4RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field4MessageType<'this>>;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field5RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::single_field::AsRefIter<
            <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
            str,
        >;

        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
            ::puroro::internal::impls::single_field::AsRefIter::new(
                ::std::iter::IntoIterator::into_iter(&self.reserved_name),
            )
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumDescriptorProtoSingleField5<ScalarType, RepeatedType>
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
            >(&self.reserved_name, 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for EnumDescriptorProtoSingleField5<ScalarType, RepeatedType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                reserved_name: value,
            }
        }
    }
    pub struct EnumDescriptorProtoBuilder<T>(T);

    impl<T> EnumDescriptorProtoBuilder<T>
    where
        T: EnumDescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            EnumDescriptorProtoBuilder((self.0, EnumDescriptorProtoSingleField1 { name: value }))
        }

    pub fn append_value<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSingleField2<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            EnumDescriptorProtoBuilder((self.0, EnumDescriptorProtoSingleField2 { value: value }))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSingleField3<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            EnumDescriptorProtoBuilder((self.0, EnumDescriptorProtoSingleField3 { options: value }))
        }

    pub fn append_reserved_range<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> EnumDescriptorProtoBuilder<(T, EnumDescriptorProtoSingleField4<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            EnumDescriptorProtoBuilder((
                self.0,
                EnumDescriptorProtoSingleField4 {
                    reserved_range: value,
                },
            ))
        }

        pub fn append_reserved_name<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> EnumDescriptorProtoBuilder<(
            T,
            EnumDescriptorProtoSingleField5<ScalarType, RepeatedType>,
        )>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            EnumDescriptorProtoBuilder((
                self.0,
                EnumDescriptorProtoSingleField5 {
                    reserved_name: value,
                },
            ))
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> EnumValueDescriptorProtoTrait for (T, U)
    where
        T: EnumValueDescriptorProtoTrait,
        U: EnumValueDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as EnumValueDescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as EnumValueDescriptorProtoTrait>::name_opt(&self.0))
        }
        fn number_opt<'this>(&'this self) -> Option<i32> {
            <U as EnumValueDescriptorProtoTrait>::number_opt(&self.1)
                .or_else(|| <T as EnumValueDescriptorProtoTrait>::number_opt(&self.0))
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as EnumValueDescriptorProtoTrait>::options_opt(&self.0),
                <U as EnumValueDescriptorProtoTrait>::options_opt(&self.1),
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
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as EnumValueDescriptorProtoTrait>::name_opt(t),
                |u| <U as EnumValueDescriptorProtoTrait>::name_opt(u),
            )
        }
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().either(
                |t| <T as EnumValueDescriptorProtoTrait>::number_opt(t),
                |u| <U as EnumValueDescriptorProtoTrait>::number_opt(u),
            )
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
            <U as EnumValueDescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as EnumValueDescriptorProtoTrait>::options_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as EnumValueDescriptorProtoTrait>::options_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }
    impl<T> EnumValueDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: EnumValueDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            self.as_ref().and_then(|msg| msg.number_opt())
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = T::Field3MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumValueDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::EnumValueDescriptorProto>
        for EnumValueDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::EnumValueDescriptorProtoTrait
        for EnumValueDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumValueDescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType>
        for EnumValueDescriptorProtoSingleField1<ScalarType>
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

    pub struct EnumValueDescriptorProtoSingleField2 {
        pub number: i32,
    }

    impl ::puroro::Message<super::EnumValueDescriptorProto> for EnumValueDescriptorProtoSingleField2 {}

    impl super::_puroro_traits::EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoSingleField2 {
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.number))
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumValueDescriptorProtoSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int32>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.number), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i32> for EnumValueDescriptorProtoSingleField2 {
        fn from(value: i32) -> Self {
            Self { number: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumValueDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::EnumValueDescriptorProto>
        for EnumValueDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::EnumValueDescriptorProtoTrait
        for EnumValueDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumValueDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait
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
            ::std::iter::once(&self.options),
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType>
        for EnumValueDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }
    pub struct EnumValueDescriptorProtoBuilder<T>(T);

    impl<T> EnumValueDescriptorProtoBuilder<T>
    where
        T: EnumValueDescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> EnumValueDescriptorProtoBuilder<(T, EnumValueDescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            EnumValueDescriptorProtoBuilder((
                self.0,
                EnumValueDescriptorProtoSingleField1 { name: value },
            ))
        }

        pub fn append_number(
            self,
            value: i32,
        ) -> EnumValueDescriptorProtoBuilder<(T, EnumValueDescriptorProtoSingleField2)> {
            EnumValueDescriptorProtoBuilder((
                self.0,
                EnumValueDescriptorProtoSingleField2 { number: value },
            ))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> EnumValueDescriptorProtoBuilder<(T, EnumValueDescriptorProtoSingleField3<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            EnumValueDescriptorProtoBuilder((
                self.0,
                EnumValueDescriptorProtoSingleField3 { options: value },
            ))
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
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> ServiceDescriptorProtoTrait for (T, U)
    where
        T: ServiceDescriptorProtoTrait,
        U: ServiceDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as ServiceDescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as ServiceDescriptorProtoTrait>::name_opt(&self.0))
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field2RepeatedType<'this>,
        >;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as ServiceDescriptorProtoTrait>::method(&self.0),
                <U as ServiceDescriptorProtoTrait>::method(&self.1),
            )
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as ServiceDescriptorProtoTrait>::Field3MessageType<'this>>,
            ::std::option::Option<<U as ServiceDescriptorProtoTrait>::Field3MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
            match (
                <T as ServiceDescriptorProtoTrait>::options_opt(&self.0),
                <U as ServiceDescriptorProtoTrait>::options_opt(&self.1),
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
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as ServiceDescriptorProtoTrait>::name_opt(t),
                |u| <U as ServiceDescriptorProtoTrait>::name_opt(u),
            )
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
            <U as ServiceDescriptorProtoTrait>::Field3MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as ServiceDescriptorProtoTrait>::options_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as ServiceDescriptorProtoTrait>::options_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
    }
    impl<T> ServiceDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: ServiceDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = T::Field2MessageType<'this>;

        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        type Field3MessageType<'this>
        where
            Self: 'this,
        = T::Field3MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct ServiceDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::ServiceDescriptorProto>
        for ServiceDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::ServiceDescriptorProtoTrait
        for ServiceDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for ServiceDescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for ServiceDescriptorProtoSingleField1<ScalarType>
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

    pub struct ServiceDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub method: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::ServiceDescriptorProto>
        for ServiceDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::ServiceDescriptorProtoTrait
        for ServiceDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.method)
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for ServiceDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait
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
            &self.method,
            2,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for ServiceDescriptorProtoSingleField2<ScalarType, RepeatedType>
    where
        ScalarType:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self { method: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct ServiceDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::ServiceDescriptorProto>
        for ServiceDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::ServiceDescriptorProtoTrait
        for ServiceDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
        type Field3MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for ServiceDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait
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
            ::std::iter::once(&self.options),
            3,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for ServiceDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }
    pub struct ServiceDescriptorProtoBuilder<T>(T);

    impl<T> ServiceDescriptorProtoBuilder<T>
    where
        T: ServiceDescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> ServiceDescriptorProtoBuilder<(T, ServiceDescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            ServiceDescriptorProtoBuilder((
                self.0,
                ServiceDescriptorProtoSingleField1 { name: value },
            ))
        }

        pub fn append_method<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> ServiceDescriptorProtoBuilder<(
            T,
            ServiceDescriptorProtoSingleField2<ScalarType, RepeatedType>,
        )>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            ServiceDescriptorProtoBuilder((
                self.0,
                ServiceDescriptorProtoSingleField2 { method: value },
            ))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> ServiceDescriptorProtoBuilder<(T, ServiceDescriptorProtoSingleField3<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            ServiceDescriptorProtoBuilder((
                self.0,
                ServiceDescriptorProtoSingleField3 { options: value },
            ))
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
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
    }
    impl<T, U> MethodDescriptorProtoTrait for (T, U)
    where
        T: MethodDescriptorProtoTrait,
        U: MethodDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> Option<&'this str> {
            <U as MethodDescriptorProtoTrait>::name_opt(&self.1)
                .or_else(|| <T as MethodDescriptorProtoTrait>::name_opt(&self.0))
        }
        fn input_type_opt<'this>(&'this self) -> Option<&'this str> {
            <U as MethodDescriptorProtoTrait>::input_type_opt(&self.1)
                .or_else(|| <T as MethodDescriptorProtoTrait>::input_type_opt(&self.0))
        }
        fn output_type_opt<'this>(&'this self) -> Option<&'this str> {
            <U as MethodDescriptorProtoTrait>::output_type_opt(&self.1)
                .or_else(|| <T as MethodDescriptorProtoTrait>::output_type_opt(&self.0))
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = (
            ::std::option::Option<<T as MethodDescriptorProtoTrait>::Field4MessageType<'this>>,
            ::std::option::Option<<U as MethodDescriptorProtoTrait>::Field4MessageType<'this>>,
        );
        fn options_opt<'this>(&'this self) -> Option<Self::Field4MessageType<'this>> {
            match (
                <T as MethodDescriptorProtoTrait>::options_opt(&self.0),
                <U as MethodDescriptorProtoTrait>::options_opt(&self.1),
            ) {
                (None, None) => None,
                (Some(t), None) => Some((Some(t), None)),
                (None, Some(u)) => Some((None, Some(u))),
                (Some(t), Some(u)) => Some((Some(t), Some(u))),
            }
        }
        fn client_streaming_opt<'this>(&'this self) -> Option<bool> {
            <U as MethodDescriptorProtoTrait>::client_streaming_opt(&self.1)
                .or_else(|| <T as MethodDescriptorProtoTrait>::client_streaming_opt(&self.0))
        }
        fn server_streaming_opt<'this>(&'this self) -> Option<bool> {
            <U as MethodDescriptorProtoTrait>::server_streaming_opt(&self.1)
                .or_else(|| <T as MethodDescriptorProtoTrait>::server_streaming_opt(&self.0))
        }
    }
    impl<T, U> MethodDescriptorProtoTrait for ::puroro::Either<T, U>
    where
        T: MethodDescriptorProtoTrait,
        U: MethodDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::name_opt(t),
                |u| <U as MethodDescriptorProtoTrait>::name_opt(u),
            )
        }
        fn input_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::input_type_opt(t),
                |u| <U as MethodDescriptorProtoTrait>::input_type_opt(u),
            )
        }
        fn output_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::output_type_opt(t),
                |u| <U as MethodDescriptorProtoTrait>::output_type_opt(u),
            )
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
            <U as MethodDescriptorProtoTrait>::Field4MessageType<'this>,
        >;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field4MessageType<'this>> {
            self.as_ref().either(
                |t| {
                    <T as MethodDescriptorProtoTrait>::options_opt(t)
                        .map(|t| ::puroro::Either::Left(t))
                },
                |u| {
                    <U as MethodDescriptorProtoTrait>::options_opt(u)
                        .map(|u| ::puroro::Either::Right(u))
                },
            )
        }
        fn client_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::client_streaming_opt(t),
                |u| <U as MethodDescriptorProtoTrait>::client_streaming_opt(u),
            )
        }
        fn server_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as MethodDescriptorProtoTrait>::server_streaming_opt(t),
                |u| <U as MethodDescriptorProtoTrait>::server_streaming_opt(u),
            )
        }
    }
    impl<T> MethodDescriptorProtoTrait for ::std::option::Option<T>
    where
        T: MethodDescriptorProtoTrait,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.name_opt())
        }
        fn input_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.input_type_opt())
        }
        fn output_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.output_type_opt())
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = T::Field4MessageType<'this>;
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field4MessageType<'this>> {
            self.as_ref().and_then(|msg| msg.options_opt())
        }
        fn client_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.client_streaming_opt())
        }
        fn server_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.server_streaming_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MethodDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub name: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::MethodDescriptorProto>
        for MethodDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::MethodDescriptorProtoTrait
        for MethodDescriptorProtoSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.name.as_ref())
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for MethodDescriptorProtoSingleField1<ScalarType>
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
            >(::std::iter::once(&self.name), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MethodDescriptorProtoSingleField1<ScalarType>
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

    pub struct MethodDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub input_type: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::MethodDescriptorProto>
        for MethodDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::MethodDescriptorProtoTrait
        for MethodDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn input_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.input_type.as_ref())
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for MethodDescriptorProtoSingleField2<ScalarType>
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
            >(::std::iter::once(&self.input_type), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MethodDescriptorProtoSingleField2<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { input_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MethodDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub output_type: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::MethodDescriptorProto>
        for MethodDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::MethodDescriptorProtoTrait
        for MethodDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn output_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.output_type.as_ref())
        }
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for MethodDescriptorProtoSingleField3<ScalarType>
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
            >(::std::iter::once(&self.output_type), 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MethodDescriptorProtoSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { output_type: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MethodDescriptorProtoSingleField4<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub options: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::MethodDescriptorProto>
        for MethodDescriptorProtoSingleField4<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::MethodDescriptorProtoTrait
        for MethodDescriptorProtoSingleField4<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field4MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;

        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field4MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for MethodDescriptorProtoSingleField4<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait
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
            ::std::iter::once(&self.options),
            4,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for MethodDescriptorProtoSingleField4<ScalarType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { options: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MethodDescriptorProtoSingleField5 {
        pub client_streaming: bool,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSingleField5 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSingleField5 {
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();

        fn client_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.client_streaming))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MethodDescriptorProtoSingleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.client_streaming), 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for MethodDescriptorProtoSingleField5 {
        fn from(value: bool) -> Self {
            Self {
                client_streaming: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MethodDescriptorProtoSingleField6 {
        pub server_streaming: bool,
    }

    impl ::puroro::Message<super::MethodDescriptorProto> for MethodDescriptorProtoSingleField6 {}

    impl super::_puroro_traits::MethodDescriptorProtoTrait for MethodDescriptorProtoSingleField6 {
        type Field4MessageType<'this>
        where
            Self: 'this,
        = ();

        fn server_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.server_streaming))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MethodDescriptorProtoSingleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.server_streaming), 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for MethodDescriptorProtoSingleField6 {
        fn from(value: bool) -> Self {
            Self {
                server_streaming: value,
            }
        }
    }
    pub struct MethodDescriptorProtoBuilder<T>(T);

    impl<T> MethodDescriptorProtoBuilder<T>
    where
        T: MethodDescriptorProtoTrait,
    {
        pub fn append_name<ScalarType>(
            self,
            value: ScalarType,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MethodDescriptorProtoBuilder((
                self.0,
                MethodDescriptorProtoSingleField1 { name: value },
            ))
        }

        pub fn append_input_type<ScalarType>(
            self,
            value: ScalarType,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSingleField2<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MethodDescriptorProtoBuilder((
                self.0,
                MethodDescriptorProtoSingleField2 { input_type: value },
            ))
        }

        pub fn append_output_type<ScalarType>(
            self,
            value: ScalarType,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSingleField3<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MethodDescriptorProtoBuilder((
                self.0,
                MethodDescriptorProtoSingleField3 { output_type: value },
            ))
        }

        pub fn append_options<ScalarType>(
            self,
            value: ScalarType,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSingleField4<ScalarType>)>
        where
            ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            MethodDescriptorProtoBuilder((
                self.0,
                MethodDescriptorProtoSingleField4 { options: value },
            ))
        }

        pub fn append_client_streaming(
            self,
            value: bool,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSingleField5)> {
            MethodDescriptorProtoBuilder((
                self.0,
                MethodDescriptorProtoSingleField5 {
                    client_streaming: value,
                },
            ))
        }

        pub fn append_server_streaming(
            self,
            value: bool,
        ) -> MethodDescriptorProtoBuilder<(T, MethodDescriptorProtoSingleField6)> {
            MethodDescriptorProtoBuilder((
                self.0,
                MethodDescriptorProtoSingleField6 {
                    server_streaming: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> FileOptionsTrait for (T, U)
    where
        T: FileOptionsTrait,
        U: FileOptionsTrait,
    {
        fn java_package_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::java_package_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_package_opt(&self.0))
        }
        fn java_outer_classname_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::java_outer_classname_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_outer_classname_opt(&self.0))
        }
        fn java_multiple_files_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_multiple_files_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_multiple_files_opt(&self.0))
        }
        fn java_generate_equals_and_hash_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_generate_equals_and_hash_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_generate_equals_and_hash_opt(&self.0))
        }
        fn java_string_check_utf8_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_string_check_utf8_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_string_check_utf8_opt(&self.0))
        }
        fn optimize_for_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        {
            <U as FileOptionsTrait>::optimize_for_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::optimize_for_opt(&self.0))
        }
        fn go_package_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::go_package_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::go_package_opt(&self.0))
        }
        fn cc_generic_services_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::cc_generic_services_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::cc_generic_services_opt(&self.0))
        }
        fn java_generic_services_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::java_generic_services_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::java_generic_services_opt(&self.0))
        }
        fn py_generic_services_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::py_generic_services_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::py_generic_services_opt(&self.0))
        }
        fn php_generic_services_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::php_generic_services_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::php_generic_services_opt(&self.0))
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::deprecated_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::deprecated_opt(&self.0))
        }
        fn cc_enable_arenas_opt<'this>(&'this self) -> Option<bool> {
            <U as FileOptionsTrait>::cc_enable_arenas_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::cc_enable_arenas_opt(&self.0))
        }
        fn objc_class_prefix_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::objc_class_prefix_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::objc_class_prefix_opt(&self.0))
        }
        fn csharp_namespace_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::csharp_namespace_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::csharp_namespace_opt(&self.0))
        }
        fn swift_prefix_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::swift_prefix_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::swift_prefix_opt(&self.0))
        }
        fn php_class_prefix_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::php_class_prefix_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::php_class_prefix_opt(&self.0))
        }
        fn php_namespace_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::php_namespace_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::php_namespace_opt(&self.0))
        }
        fn php_metadata_namespace_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::php_metadata_namespace_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::php_metadata_namespace_opt(&self.0))
        }
        fn ruby_package_opt<'this>(&'this self) -> Option<&'this str> {
            <U as FileOptionsTrait>::ruby_package_opt(&self.1)
                .or_else(|| <T as FileOptionsTrait>::ruby_package_opt(&self.0))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileOptionsTrait>::Field999MessageType<'this>,
            <U as FileOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        fn java_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_package_opt(t),
                |u| <U as FileOptionsTrait>::java_package_opt(u),
            )
        }
        fn java_outer_classname_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_outer_classname_opt(t),
                |u| <U as FileOptionsTrait>::java_outer_classname_opt(u),
            )
        }
        fn java_multiple_files_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_multiple_files_opt(t),
                |u| <U as FileOptionsTrait>::java_multiple_files_opt(u),
            )
        }
        fn java_generate_equals_and_hash_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_generate_equals_and_hash_opt(t),
                |u| <U as FileOptionsTrait>::java_generate_equals_and_hash_opt(u),
            )
        }
        fn java_string_check_utf8_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_string_check_utf8_opt(t),
                |u| <U as FileOptionsTrait>::java_string_check_utf8_opt(u),
            )
        }
        fn optimize_for_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        > {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::optimize_for_opt(t),
                |u| <U as FileOptionsTrait>::optimize_for_opt(u),
            )
        }
        fn go_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::go_package_opt(t),
                |u| <U as FileOptionsTrait>::go_package_opt(u),
            )
        }
        fn cc_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::cc_generic_services_opt(t),
                |u| <U as FileOptionsTrait>::cc_generic_services_opt(u),
            )
        }
        fn java_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::java_generic_services_opt(t),
                |u| <U as FileOptionsTrait>::java_generic_services_opt(u),
            )
        }
        fn py_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::py_generic_services_opt(t),
                |u| <U as FileOptionsTrait>::py_generic_services_opt(u),
            )
        }
        fn php_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::php_generic_services_opt(t),
                |u| <U as FileOptionsTrait>::php_generic_services_opt(u),
            )
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::deprecated_opt(t),
                |u| <U as FileOptionsTrait>::deprecated_opt(u),
            )
        }
        fn cc_enable_arenas_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::cc_enable_arenas_opt(t),
                |u| <U as FileOptionsTrait>::cc_enable_arenas_opt(u),
            )
        }
        fn objc_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::objc_class_prefix_opt(t),
                |u| <U as FileOptionsTrait>::objc_class_prefix_opt(u),
            )
        }
        fn csharp_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::csharp_namespace_opt(t),
                |u| <U as FileOptionsTrait>::csharp_namespace_opt(u),
            )
        }
        fn swift_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::swift_prefix_opt(t),
                |u| <U as FileOptionsTrait>::swift_prefix_opt(u),
            )
        }
        fn php_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::php_class_prefix_opt(t),
                |u| <U as FileOptionsTrait>::php_class_prefix_opt(u),
            )
        }
        fn php_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::php_namespace_opt(t),
                |u| <U as FileOptionsTrait>::php_namespace_opt(u),
            )
        }
        fn php_metadata_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::php_metadata_namespace_opt(t),
                |u| <U as FileOptionsTrait>::php_metadata_namespace_opt(u),
            )
        }
        fn ruby_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as FileOptionsTrait>::ruby_package_opt(t),
                |u| <U as FileOptionsTrait>::ruby_package_opt(u),
            )
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FileOptionsTrait>::Field999MessageType<'this>,
            <U as FileOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn java_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.java_package_opt())
        }
        fn java_outer_classname_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.java_outer_classname_opt())
        }
        fn java_multiple_files_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.java_multiple_files_opt())
        }
        fn java_generate_equals_and_hash_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref()
                .and_then(|msg| msg.java_generate_equals_and_hash_opt())
        }
        fn java_string_check_utf8_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref()
                .and_then(|msg| msg.java_string_check_utf8_opt())
        }
        fn optimize_for_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        > {
            self.as_ref().and_then(|msg| msg.optimize_for_opt())
        }
        fn go_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.go_package_opt())
        }
        fn cc_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.cc_generic_services_opt())
        }
        fn java_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref()
                .and_then(|msg| msg.java_generic_services_opt())
        }
        fn py_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.py_generic_services_opt())
        }
        fn php_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.php_generic_services_opt())
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated_opt())
        }
        fn cc_enable_arenas_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.cc_enable_arenas_opt())
        }
        fn objc_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.objc_class_prefix_opt())
        }
        fn csharp_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.csharp_namespace_opt())
        }
        fn swift_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.swift_prefix_opt())
        }
        fn php_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.php_class_prefix_opt())
        }
        fn php_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.php_namespace_opt())
        }
        fn php_metadata_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref()
                .and_then(|msg| msg.php_metadata_namespace_opt())
        }
        fn ruby_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.ruby_package_opt())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct FileOptionsSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub java_package: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField1<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn java_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.java_package.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField1<ScalarType>
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
            >(::std::iter::once(&self.java_package), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField1<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                java_package: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField8<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub java_outer_classname: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField8<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField8<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn java_outer_classname_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.java_outer_classname.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField8<ScalarType>
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
            >(::std::iter::once(&self.java_outer_classname), 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField8<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                java_outer_classname: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField10 {
        pub java_multiple_files: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField10 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField10 {
        fn java_multiple_files_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.java_multiple_files))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.java_multiple_files), 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField10 {
        fn from(value: bool) -> Self {
            Self {
                java_multiple_files: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField20 {
        pub java_generate_equals_and_hash: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField20 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField20 {
        fn java_generate_equals_and_hash_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(
                &self.java_generate_equals_and_hash,
            ))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField20 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(
                ::std::iter::once(&self.java_generate_equals_and_hash),
                20,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField20 {
        fn from(value: bool) -> Self {
            Self {
                java_generate_equals_and_hash: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField27 {
        pub java_string_check_utf8: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField27 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField27 {
        fn java_string_check_utf8_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.java_string_check_utf8))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField27 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.java_string_check_utf8), 27, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField27 {
        fn from(value: bool) -> Self {
            Self {
                java_string_check_utf8: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField9 {
        pub optimize_for:
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField9 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField9 {
        fn optimize_for_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        > {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.optimize_for))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField9 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode>
        >::ser_field::
        <(), _, _>
        (
            ::std::iter::once(&self.optimize_for),
            9,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        > for FileOptionsSingleField9
    {
        fn from(
            value: self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        ) -> Self {
            Self {
                optimize_for: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub go_package: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField11<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn go_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.go_package.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField11<ScalarType>
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
            >(::std::iter::once(&self.go_package), 11, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField11<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self { go_package: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField16 {
        pub cc_generic_services: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField16 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField16 {
        fn cc_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.cc_generic_services))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField16 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.cc_generic_services), 16, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField16 {
        fn from(value: bool) -> Self {
            Self {
                cc_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField17 {
        pub java_generic_services: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField17 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField17 {
        fn java_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.java_generic_services))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField17 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.java_generic_services), 17, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField17 {
        fn from(value: bool) -> Self {
            Self {
                java_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField18 {
        pub py_generic_services: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField18 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField18 {
        fn py_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.py_generic_services))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField18 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.py_generic_services), 18, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField18 {
        fn from(value: bool) -> Self {
            Self {
                py_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField42 {
        pub php_generic_services: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField42 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField42 {
        fn php_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.php_generic_services))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField42 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.php_generic_services), 42, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField42 {
        fn from(value: bool) -> Self {
            Self {
                php_generic_services: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField23 {
        pub deprecated: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField23 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField23 {
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.deprecated))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField23 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.deprecated), 23, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField23 {
        fn from(value: bool) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField31 {
        pub cc_enable_arenas: bool,
    }

    impl ::puroro::Message<super::FileOptions> for FileOptionsSingleField31 {}

    impl super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField31 {
        fn cc_enable_arenas_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.cc_enable_arenas))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FileOptionsSingleField31 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.cc_enable_arenas), 31, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FileOptionsSingleField31 {
        fn from(value: bool) -> Self {
            Self {
                cc_enable_arenas: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField36<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub objc_class_prefix: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField36<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField36<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn objc_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.objc_class_prefix.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField36<ScalarType>
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
            >(::std::iter::once(&self.objc_class_prefix), 36, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField36<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                objc_class_prefix: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField37<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub csharp_namespace: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField37<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField37<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn csharp_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.csharp_namespace.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField37<ScalarType>
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
            >(::std::iter::once(&self.csharp_namespace), 37, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField37<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                csharp_namespace: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField39<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub swift_prefix: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField39<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField39<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn swift_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.swift_prefix.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField39<ScalarType>
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
            >(::std::iter::once(&self.swift_prefix), 39, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField39<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                swift_prefix: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField40<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub php_class_prefix: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField40<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField40<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn php_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.php_class_prefix.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField40<ScalarType>
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
            >(::std::iter::once(&self.php_class_prefix), 40, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField40<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                php_class_prefix: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField41<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub php_namespace: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField41<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField41<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn php_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.php_namespace.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField41<ScalarType>
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
            >(::std::iter::once(&self.php_namespace), 41, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField41<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                php_namespace: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField44<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub php_metadata_namespace: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField44<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField44<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn php_metadata_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.php_metadata_namespace.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField44<ScalarType>
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
            >(::std::iter::once(&self.php_metadata_namespace), 44, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField44<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                php_metadata_namespace: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField45<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub ruby_package: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::FileOptions> for FileOptionsSingleField45<ScalarType> where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug
    {
    }

    impl<ScalarType> super::_puroro_traits::FileOptionsTrait for FileOptionsSingleField45<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ruby_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.ruby_package.as_ref())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField45<ScalarType>
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
            >(::std::iter::once(&self.ruby_package), 45, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for FileOptionsSingleField45<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                ruby_package: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FileOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FileOptions>
        for FileOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FileOptionsTrait
        for FileOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FileOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FileOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct FileOptionsBuilder<T>(T);

    impl<T> FileOptionsBuilder<T>
    where
        T: FileOptionsTrait,
    {
        pub fn append_java_package<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField1<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField1 {
                    java_package: value,
                },
            ))
        }

        pub fn append_java_outer_classname<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField8<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField8 {
                    java_outer_classname: value,
                },
            ))
        }

        pub fn append_java_multiple_files(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField10)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField10 {
                    java_multiple_files: value,
                },
            ))
        }

        pub fn append_java_generate_equals_and_hash(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField20)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField20 {
                    java_generate_equals_and_hash: value,
                },
            ))
        }

        pub fn append_java_string_check_utf8(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField27)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField27 {
                    java_string_check_utf8: value,
                },
            ))
        }

        pub fn append_optimize_for(
            self,
            value: self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField9)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField9 {
                    optimize_for: value,
                },
            ))
        }

        pub fn append_go_package<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField11<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((self.0, FileOptionsSingleField11 { go_package: value }))
        }

        pub fn append_cc_generic_services(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField16)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField16 {
                    cc_generic_services: value,
                },
            ))
        }

        pub fn append_java_generic_services(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField17)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField17 {
                    java_generic_services: value,
                },
            ))
        }

        pub fn append_py_generic_services(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField18)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField18 {
                    py_generic_services: value,
                },
            ))
        }

        pub fn append_php_generic_services(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField42)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField42 {
                    php_generic_services: value,
                },
            ))
        }

        pub fn append_deprecated(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField23)> {
            FileOptionsBuilder((self.0, FileOptionsSingleField23 { deprecated: value }))
        }

        pub fn append_cc_enable_arenas(
            self,
            value: bool,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField31)> {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField31 {
                    cc_enable_arenas: value,
                },
            ))
        }

        pub fn append_objc_class_prefix<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField36<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField36 {
                    objc_class_prefix: value,
                },
            ))
        }

        pub fn append_csharp_namespace<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField37<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField37 {
                    csharp_namespace: value,
                },
            ))
        }

        pub fn append_swift_prefix<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField39<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField39 {
                    swift_prefix: value,
                },
            ))
        }

        pub fn append_php_class_prefix<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField40<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField40 {
                    php_class_prefix: value,
                },
            ))
        }

        pub fn append_php_namespace<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField41<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField41 {
                    php_namespace: value,
                },
            ))
        }

        pub fn append_php_metadata_namespace<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField44<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField44 {
                    php_metadata_namespace: value,
                },
            ))
        }

        pub fn append_ruby_package<ScalarType>(
            self,
            value: ScalarType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField45<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField45 {
                    ruby_package: value,
                },
            ))
        }

        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FileOptionsBuilder<(T, FileOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            FileOptionsBuilder((
                self.0,
                FileOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MessageOptionsTrait for (T, U)
    where
        T: MessageOptionsTrait,
        U: MessageOptionsTrait,
    {
        fn message_set_wire_format_opt<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::message_set_wire_format_opt(&self.1)
                .or_else(|| <T as MessageOptionsTrait>::message_set_wire_format_opt(&self.0))
        }
        fn no_standard_descriptor_accessor_opt<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::no_standard_descriptor_accessor_opt(&self.1).or_else(|| {
                <T as MessageOptionsTrait>::no_standard_descriptor_accessor_opt(&self.0)
            })
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::deprecated_opt(&self.1)
                .or_else(|| <T as MessageOptionsTrait>::deprecated_opt(&self.0))
        }
        fn map_entry_opt<'this>(&'this self) -> Option<bool> {
            <U as MessageOptionsTrait>::map_entry_opt(&self.1)
                .or_else(|| <T as MessageOptionsTrait>::map_entry_opt(&self.0))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MessageOptionsTrait>::Field999MessageType<'this>,
            <U as MessageOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        fn message_set_wire_format_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::message_set_wire_format_opt(t),
                |u| <U as MessageOptionsTrait>::message_set_wire_format_opt(u),
            )
        }
        fn no_standard_descriptor_accessor_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::no_standard_descriptor_accessor_opt(t),
                |u| <U as MessageOptionsTrait>::no_standard_descriptor_accessor_opt(u),
            )
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::deprecated_opt(t),
                |u| <U as MessageOptionsTrait>::deprecated_opt(u),
            )
        }
        fn map_entry_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as MessageOptionsTrait>::map_entry_opt(t),
                |u| <U as MessageOptionsTrait>::map_entry_opt(u),
            )
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MessageOptionsTrait>::Field999MessageType<'this>,
            <U as MessageOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn message_set_wire_format_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref()
                .and_then(|msg| msg.message_set_wire_format_opt())
        }
        fn no_standard_descriptor_accessor_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref()
                .and_then(|msg| msg.no_standard_descriptor_accessor_opt())
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated_opt())
        }
        fn map_entry_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.map_entry_opt())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct MessageOptionsSingleField1 {
        pub message_set_wire_format: bool,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSingleField1 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSingleField1 {
        fn message_set_wire_format_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.message_set_wire_format))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MessageOptionsSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.message_set_wire_format), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for MessageOptionsSingleField1 {
        fn from(value: bool) -> Self {
            Self {
                message_set_wire_format: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MessageOptionsSingleField2 {
        pub no_standard_descriptor_accessor: bool,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSingleField2 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSingleField2 {
        fn no_standard_descriptor_accessor_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(
                &self.no_standard_descriptor_accessor,
            ))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MessageOptionsSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(
                ::std::iter::once(&self.no_standard_descriptor_accessor),
                2,
                out,
            )?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for MessageOptionsSingleField2 {
        fn from(value: bool) -> Self {
            Self {
                no_standard_descriptor_accessor: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MessageOptionsSingleField3 {
        pub deprecated: bool,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSingleField3 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSingleField3 {
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.deprecated))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MessageOptionsSingleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.deprecated), 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for MessageOptionsSingleField3 {
        fn from(value: bool) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MessageOptionsSingleField7 {
        pub map_entry: bool,
    }

    impl ::puroro::Message<super::MessageOptions> for MessageOptionsSingleField7 {}

    impl super::_puroro_traits::MessageOptionsTrait for MessageOptionsSingleField7 {
        fn map_entry_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.map_entry))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MessageOptionsSingleField7 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.map_entry), 7, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for MessageOptionsSingleField7 {
        fn from(value: bool) -> Self {
            Self { map_entry: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MessageOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::MessageOptions>
        for MessageOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MessageOptionsTrait
        for MessageOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MessageOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MessageOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct MessageOptionsBuilder<T>(T);

    impl<T> MessageOptionsBuilder<T>
    where
        T: MessageOptionsTrait,
    {
        pub fn append_message_set_wire_format(
            self,
            value: bool,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSingleField1)> {
            MessageOptionsBuilder((
                self.0,
                MessageOptionsSingleField1 {
                    message_set_wire_format: value,
                },
            ))
        }

        pub fn append_no_standard_descriptor_accessor(
            self,
            value: bool,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSingleField2)> {
            MessageOptionsBuilder((
                self.0,
                MessageOptionsSingleField2 {
                    no_standard_descriptor_accessor: value,
                },
            ))
        }

        pub fn append_deprecated(
            self,
            value: bool,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSingleField3)> {
            MessageOptionsBuilder((self.0, MessageOptionsSingleField3 { deprecated: value }))
        }

        pub fn append_map_entry(
            self,
            value: bool,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSingleField7)> {
            MessageOptionsBuilder((self.0, MessageOptionsSingleField7 { map_entry: value }))
        }

        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MessageOptionsBuilder<(T, MessageOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MessageOptionsBuilder((
                self.0,
                MessageOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> FieldOptionsTrait for (T, U)
    where
        T: FieldOptionsTrait,
        U: FieldOptionsTrait,
    {
        fn ctype_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype>
        {
            <U as FieldOptionsTrait>::ctype_opt(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::ctype_opt(&self.0))
        }
        fn packed_opt<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::packed_opt(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::packed_opt(&self.0))
        }
        fn jstype_opt<'this>(
            &'this self,
        ) -> Option<self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype>
        {
            <U as FieldOptionsTrait>::jstype_opt(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::jstype_opt(&self.0))
        }
        fn lazy_opt<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::lazy_opt(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::lazy_opt(&self.0))
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::deprecated_opt(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::deprecated_opt(&self.0))
        }
        fn weak_opt<'this>(&'this self) -> Option<bool> {
            <U as FieldOptionsTrait>::weak_opt(&self.1)
                .or_else(|| <T as FieldOptionsTrait>::weak_opt(&self.0))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FieldOptionsTrait>::Field999MessageType<'this>,
            <U as FieldOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        fn ctype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        > {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::ctype_opt(t),
                |u| <U as FieldOptionsTrait>::ctype_opt(u),
            )
        }
        fn packed_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::packed_opt(t),
                |u| <U as FieldOptionsTrait>::packed_opt(u),
            )
        }
        fn jstype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        > {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::jstype_opt(t),
                |u| <U as FieldOptionsTrait>::jstype_opt(u),
            )
        }
        fn lazy_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::lazy_opt(t),
                |u| <U as FieldOptionsTrait>::lazy_opt(u),
            )
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::deprecated_opt(t),
                |u| <U as FieldOptionsTrait>::deprecated_opt(u),
            )
        }
        fn weak_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as FieldOptionsTrait>::weak_opt(t),
                |u| <U as FieldOptionsTrait>::weak_opt(u),
            )
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as FieldOptionsTrait>::Field999MessageType<'this>,
            <U as FieldOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn ctype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        > {
            self.as_ref().and_then(|msg| msg.ctype_opt())
        }
        fn packed_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.packed_opt())
        }
        fn jstype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        > {
            self.as_ref().and_then(|msg| msg.jstype_opt())
        }
        fn lazy_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.lazy_opt())
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated_opt())
        }
        fn weak_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.weak_opt())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct FieldOptionsSingleField1 {
        pub ctype: self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSingleField1 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSingleField1 {
        fn ctype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        > {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.ctype))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldOptionsSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
                >,
            >::ser_field::<(), _, _>(::std::iter::once(&self.ctype), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        > for FieldOptionsSingleField1
    {
        fn from(
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        ) -> Self {
            Self { ctype: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldOptionsSingleField2 {
        pub packed: bool,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSingleField2 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSingleField2 {
        fn packed_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.packed))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldOptionsSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.packed), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FieldOptionsSingleField2 {
        fn from(value: bool) -> Self {
            Self { packed: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldOptionsSingleField6 {
        pub jstype: self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSingleField6 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSingleField6 {
        fn jstype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        > {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.jstype))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldOptionsSingleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
                ::puroro::tags::Optional,
                ::puroro::tags::Enum2<
                    self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
                >,
            >::ser_field::<(), _, _>(::std::iter::once(&self.jstype), 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        > for FieldOptionsSingleField6
    {
        fn from(
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        ) -> Self {
            Self { jstype: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldOptionsSingleField5 {
        pub lazy: bool,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSingleField5 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSingleField5 {
        fn lazy_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.lazy))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldOptionsSingleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.lazy), 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FieldOptionsSingleField5 {
        fn from(value: bool) -> Self {
            Self { lazy: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldOptionsSingleField3 {
        pub deprecated: bool,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSingleField3 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSingleField3 {
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.deprecated))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldOptionsSingleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.deprecated), 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FieldOptionsSingleField3 {
        fn from(value: bool) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldOptionsSingleField10 {
        pub weak: bool,
    }

    impl ::puroro::Message<super::FieldOptions> for FieldOptionsSingleField10 {}

    impl super::_puroro_traits::FieldOptionsTrait for FieldOptionsSingleField10 {
        fn weak_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.weak))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for FieldOptionsSingleField10 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.weak), 10, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for FieldOptionsSingleField10 {
        fn from(value: bool) -> Self {
            Self { weak: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct FieldOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::FieldOptions>
        for FieldOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::FieldOptionsTrait
        for FieldOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for FieldOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for FieldOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct FieldOptionsBuilder<T>(T);

    impl<T> FieldOptionsBuilder<T>
    where
        T: FieldOptionsTrait,
    {
        pub fn append_ctype(
            self,
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSingleField1)> {
            FieldOptionsBuilder((self.0, FieldOptionsSingleField1 { ctype: value }))
        }

        pub fn append_packed(
            self,
            value: bool,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSingleField2)> {
            FieldOptionsBuilder((self.0, FieldOptionsSingleField2 { packed: value }))
        }

        pub fn append_jstype(
            self,
            value: self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSingleField6)> {
            FieldOptionsBuilder((self.0, FieldOptionsSingleField6 { jstype: value }))
        }

        pub fn append_lazy(
            self,
            value: bool,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSingleField5)> {
            FieldOptionsBuilder((self.0, FieldOptionsSingleField5 { lazy: value }))
        }

        pub fn append_deprecated(
            self,
            value: bool,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSingleField3)> {
            FieldOptionsBuilder((self.0, FieldOptionsSingleField3 { deprecated: value }))
        }

        pub fn append_weak(
            self,
            value: bool,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSingleField10)> {
            FieldOptionsBuilder((self.0, FieldOptionsSingleField10 { weak: value }))
        }

        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> FieldOptionsBuilder<(T, FieldOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            FieldOptionsBuilder((
                self.0,
                FieldOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> OneofOptionsTrait for (T, U)
    where
        T: OneofOptionsTrait,
        U: OneofOptionsTrait,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as OneofOptionsTrait>::Field999MessageType<'this>,
            <U as OneofOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as OneofOptionsTrait>::Field999MessageType<'this>,
            <U as OneofOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct OneofOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::OneofOptions>
        for OneofOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::OneofOptionsTrait
        for OneofOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for OneofOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for OneofOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct OneofOptionsBuilder<T>(T);

    impl<T> OneofOptionsBuilder<T>
    where
        T: OneofOptionsTrait,
    {
        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> OneofOptionsBuilder<(T, OneofOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            OneofOptionsBuilder((
                self.0,
                OneofOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> EnumOptionsTrait for (T, U)
    where
        T: EnumOptionsTrait,
        U: EnumOptionsTrait,
    {
        fn allow_alias_opt<'this>(&'this self) -> Option<bool> {
            <U as EnumOptionsTrait>::allow_alias_opt(&self.1)
                .or_else(|| <T as EnumOptionsTrait>::allow_alias_opt(&self.0))
        }
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            <U as EnumOptionsTrait>::deprecated_opt(&self.1)
                .or_else(|| <T as EnumOptionsTrait>::deprecated_opt(&self.0))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumOptionsTrait>::Field999MessageType<'this>,
            <U as EnumOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        fn allow_alias_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as EnumOptionsTrait>::allow_alias_opt(t),
                |u| <U as EnumOptionsTrait>::allow_alias_opt(u),
            )
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as EnumOptionsTrait>::deprecated_opt(t),
                |u| <U as EnumOptionsTrait>::deprecated_opt(u),
            )
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumOptionsTrait>::Field999MessageType<'this>,
            <U as EnumOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn allow_alias_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.allow_alias_opt())
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated_opt())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct EnumOptionsSingleField2 {
        pub allow_alias: bool,
    }

    impl ::puroro::Message<super::EnumOptions> for EnumOptionsSingleField2 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSingleField2 {
        fn allow_alias_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.allow_alias))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumOptionsSingleField2 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.allow_alias), 2, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for EnumOptionsSingleField2 {
        fn from(value: bool) -> Self {
            Self { allow_alias: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumOptionsSingleField3 {
        pub deprecated: bool,
    }

    impl ::puroro::Message<super::EnumOptions> for EnumOptionsSingleField3 {}

    impl super::_puroro_traits::EnumOptionsTrait for EnumOptionsSingleField3 {
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.deprecated))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumOptionsSingleField3 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.deprecated), 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for EnumOptionsSingleField3 {
        fn from(value: bool) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::EnumOptions>
        for EnumOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::EnumOptionsTrait
        for EnumOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for EnumOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct EnumOptionsBuilder<T>(T);

    impl<T> EnumOptionsBuilder<T>
    where
        T: EnumOptionsTrait,
    {
        pub fn append_allow_alias(
            self,
            value: bool,
        ) -> EnumOptionsBuilder<(T, EnumOptionsSingleField2)> {
            EnumOptionsBuilder((self.0, EnumOptionsSingleField2 { allow_alias: value }))
        }

        pub fn append_deprecated(
            self,
            value: bool,
        ) -> EnumOptionsBuilder<(T, EnumOptionsSingleField3)> {
            EnumOptionsBuilder((self.0, EnumOptionsSingleField3 { deprecated: value }))
        }

        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> EnumOptionsBuilder<(T, EnumOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            EnumOptionsBuilder((
                self.0,
                EnumOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> EnumValueOptionsTrait for (T, U)
    where
        T: EnumValueOptionsTrait,
        U: EnumValueOptionsTrait,
    {
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            <U as EnumValueOptionsTrait>::deprecated_opt(&self.1)
                .or_else(|| <T as EnumValueOptionsTrait>::deprecated_opt(&self.0))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumValueOptionsTrait>::Field999MessageType<'this>,
            <U as EnumValueOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as EnumValueOptionsTrait>::deprecated_opt(t),
                |u| <U as EnumValueOptionsTrait>::deprecated_opt(u),
            )
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as EnumValueOptionsTrait>::Field999MessageType<'this>,
            <U as EnumValueOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated_opt())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct EnumValueOptionsSingleField1 {
        pub deprecated: bool,
    }

    impl ::puroro::Message<super::EnumValueOptions> for EnumValueOptionsSingleField1 {}

    impl super::_puroro_traits::EnumValueOptionsTrait for EnumValueOptionsSingleField1 {
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.deprecated))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for EnumValueOptionsSingleField1 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.deprecated), 1, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for EnumValueOptionsSingleField1 {
        fn from(value: bool) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct EnumValueOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::EnumValueOptions>
        for EnumValueOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::EnumValueOptionsTrait
        for EnumValueOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for EnumValueOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for EnumValueOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct EnumValueOptionsBuilder<T>(T);

    impl<T> EnumValueOptionsBuilder<T>
    where
        T: EnumValueOptionsTrait,
    {
        pub fn append_deprecated(
            self,
            value: bool,
        ) -> EnumValueOptionsBuilder<(T, EnumValueOptionsSingleField1)> {
            EnumValueOptionsBuilder((self.0, EnumValueOptionsSingleField1 { deprecated: value }))
        }

        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> EnumValueOptionsBuilder<(T, EnumValueOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            EnumValueOptionsBuilder((
                self.0,
                EnumValueOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> ServiceOptionsTrait for (T, U)
    where
        T: ServiceOptionsTrait,
        U: ServiceOptionsTrait,
    {
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            <U as ServiceOptionsTrait>::deprecated_opt(&self.1)
                .or_else(|| <T as ServiceOptionsTrait>::deprecated_opt(&self.0))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as ServiceOptionsTrait>::Field999MessageType<'this>,
            <U as ServiceOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as ServiceOptionsTrait>::deprecated_opt(t),
                |u| <U as ServiceOptionsTrait>::deprecated_opt(u),
            )
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as ServiceOptionsTrait>::Field999MessageType<'this>,
            <U as ServiceOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated_opt())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct ServiceOptionsSingleField33 {
        pub deprecated: bool,
    }

    impl ::puroro::Message<super::ServiceOptions> for ServiceOptionsSingleField33 {}

    impl super::_puroro_traits::ServiceOptionsTrait for ServiceOptionsSingleField33 {
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.deprecated))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for ServiceOptionsSingleField33 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.deprecated), 33, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for ServiceOptionsSingleField33 {
        fn from(value: bool) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct ServiceOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::ServiceOptions>
        for ServiceOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::ServiceOptionsTrait
        for ServiceOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for ServiceOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for ServiceOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct ServiceOptionsBuilder<T>(T);

    impl<T> ServiceOptionsBuilder<T>
    where
        T: ServiceOptionsTrait,
    {
        pub fn append_deprecated(
            self,
            value: bool,
        ) -> ServiceOptionsBuilder<(T, ServiceOptionsSingleField33)> {
            ServiceOptionsBuilder((self.0, ServiceOptionsSingleField33 { deprecated: value }))
        }

        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> ServiceOptionsBuilder<(T, ServiceOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            ServiceOptionsBuilder((
                self.0,
                ServiceOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> MethodOptionsTrait for (T, U)
    where
        T: MethodOptionsTrait,
        U: MethodOptionsTrait,
    {
        fn deprecated_opt<'this>(&'this self) -> Option<bool> {
            <U as MethodOptionsTrait>::deprecated_opt(&self.1)
                .or_else(|| <T as MethodOptionsTrait>::deprecated_opt(&self.0))
        }
        fn idempotency_level_opt<'this>(
            &'this self,
        ) -> Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            <U as MethodOptionsTrait>::idempotency_level_opt(&self.1)
                .or_else(|| <T as MethodOptionsTrait>::idempotency_level_opt(&self.0))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MethodOptionsTrait>::Field999MessageType<'this>,
            <U as MethodOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().either(
                |t| <T as MethodOptionsTrait>::deprecated_opt(t),
                |u| <U as MethodOptionsTrait>::deprecated_opt(u),
            )
        }
        fn idempotency_level_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            self.as_ref().either(
                |t| <T as MethodOptionsTrait>::idempotency_level_opt(t),
                |u| <U as MethodOptionsTrait>::idempotency_level_opt(u),
            )
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as MethodOptionsTrait>::Field999MessageType<'this>,
            <U as MethodOptionsTrait>::Field999MessageType<'this>,
        >;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            self.as_ref().and_then(|msg| msg.deprecated_opt())
        }
        fn idempotency_level_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            self.as_ref().and_then(|msg| msg.idempotency_level_opt())
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = T::Field999MessageType<'this>;

        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct MethodOptionsSingleField33 {
        pub deprecated: bool,
    }

    impl ::puroro::Message<super::MethodOptions> for MethodOptionsSingleField33 {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptionsSingleField33 {
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.deprecated))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MethodOptionsSingleField33 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bool>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.deprecated), 33, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<bool> for MethodOptionsSingleField33 {
        fn from(value: bool) -> Self {
            Self { deprecated: value }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MethodOptionsSingleField34 {
        pub idempotency_level:
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
    }

    impl ::puroro::Message<super::MethodOptions> for MethodOptionsSingleField34 {}

    impl super::_puroro_traits::MethodOptionsTrait for MethodOptionsSingleField34 {
        fn idempotency_level_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.idempotency_level))
        }
        type Field999MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field999MessageType<'this>>;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for MethodOptionsSingleField34 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<
            ::puroro::tags::Optional, ::puroro::tags::Enum2<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel>
        >::ser_field::
        <(), _, _>
        (
            ::std::iter::once(&self.idempotency_level),
            34,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl
        ::std::convert::From<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > for MethodOptionsSingleField34
    {
        fn from(
            value: self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        ) -> Self {
            Self {
                idempotency_level: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct MethodOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        pub uninterpreted_option: RepeatedType,
    }

    impl<ScalarType, RepeatedType> ::puroro::Message<super::MethodOptions>
        for MethodOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
    }

    impl<ScalarType, RepeatedType> super::_puroro_traits::MethodOptionsTrait
        for MethodOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        type Field999MessageType<'this>
        where
            Self: 'this,
        = &'this ScalarType;
        type Field999RepeatedType<'this>
        where
            Self: 'this,
        = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this> {
            ::std::iter::IntoIterator::into_iter(&self.uninterpreted_option)
        }
    }

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
        for MethodOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
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
            &self.uninterpreted_option,
            999,
            out
        )?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
        for MethodOptionsSingleField999<ScalarType, RepeatedType>
    where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
        for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
    {
        fn from(value: RepeatedType) -> Self {
            Self {
                uninterpreted_option: value,
            }
        }
    }
    pub struct MethodOptionsBuilder<T>(T);

    impl<T> MethodOptionsBuilder<T>
    where
        T: MethodOptionsTrait,
    {
        pub fn append_deprecated(
            self,
            value: bool,
        ) -> MethodOptionsBuilder<(T, MethodOptionsSingleField33)> {
            MethodOptionsBuilder((self.0, MethodOptionsSingleField33 { deprecated: value }))
        }

        pub fn append_idempotency_level(
            self,
            value: self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        ) -> MethodOptionsBuilder<(T, MethodOptionsSingleField34)> {
            MethodOptionsBuilder((
                self.0,
                MethodOptionsSingleField34 {
                    idempotency_level: value,
                },
            ))
        }

        pub fn append_uninterpreted_option<ScalarType, RepeatedType>(
            self,
            value: RepeatedType,
        ) -> MethodOptionsBuilder<(T, MethodOptionsSingleField999<ScalarType, RepeatedType>)>
        where
            ScalarType:
                self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
        {
            MethodOptionsBuilder((
                self.0,
                MethodOptionsSingleField999 {
                    uninterpreted_option: value,
                },
            ))
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
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> UninterpretedOptionTrait for (T, U)
    where
        T: UninterpretedOptionTrait,
        U: UninterpretedOptionTrait,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field2MessageType<'this>,
            <U as UninterpretedOptionTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
            <T as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
            <U as UninterpretedOptionTrait>::Field2RepeatedType<'this>,
        >;

        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::merged::MergedRepeatedMessageField::new(
                <T as UninterpretedOptionTrait>::name(&self.0),
                <U as UninterpretedOptionTrait>::name(&self.1),
            )
        }
        fn identifier_value_opt<'this>(&'this self) -> Option<&'this str> {
            <U as UninterpretedOptionTrait>::identifier_value_opt(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::identifier_value_opt(&self.0))
        }
        fn positive_int_value_opt<'this>(&'this self) -> Option<u64> {
            <U as UninterpretedOptionTrait>::positive_int_value_opt(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::positive_int_value_opt(&self.0))
        }
        fn negative_int_value_opt<'this>(&'this self) -> Option<i64> {
            <U as UninterpretedOptionTrait>::negative_int_value_opt(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::negative_int_value_opt(&self.0))
        }
        fn double_value_opt<'this>(&'this self) -> Option<f64> {
            <U as UninterpretedOptionTrait>::double_value_opt(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::double_value_opt(&self.0))
        }
        fn string_value_opt<'this>(&'this self) -> Option<&'this [u8]> {
            <U as UninterpretedOptionTrait>::string_value_opt(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::string_value_opt(&self.0))
        }
        fn aggregate_value_opt<'this>(&'this self) -> Option<&'this str> {
            <U as UninterpretedOptionTrait>::aggregate_value_opt(&self.1)
                .or_else(|| <T as UninterpretedOptionTrait>::aggregate_value_opt(&self.0))
        }
    }
    impl<T, U> UninterpretedOptionTrait for ::puroro::Either<T, U>
    where
        T: UninterpretedOptionTrait,
        U: UninterpretedOptionTrait,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as UninterpretedOptionTrait>::Field2MessageType<'this>,
            <U as UninterpretedOptionTrait>::Field2MessageType<'this>,
        >;
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        fn identifier_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::identifier_value_opt(t),
                |u| <U as UninterpretedOptionTrait>::identifier_value_opt(u),
            )
        }
        fn positive_int_value_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::positive_int_value_opt(t),
                |u| <U as UninterpretedOptionTrait>::positive_int_value_opt(u),
            )
        }
        fn negative_int_value_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::negative_int_value_opt(t),
                |u| <U as UninterpretedOptionTrait>::negative_int_value_opt(u),
            )
        }
        fn double_value_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::double_value_opt(t),
                |u| <U as UninterpretedOptionTrait>::double_value_opt(u),
            )
        }
        fn string_value_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::string_value_opt(t),
                |u| <U as UninterpretedOptionTrait>::string_value_opt(u),
            )
        }
        fn aggregate_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().either(
                |t| <T as UninterpretedOptionTrait>::aggregate_value_opt(t),
                |u| <U as UninterpretedOptionTrait>::aggregate_value_opt(u),
            )
        }
    }
    impl<T> UninterpretedOptionTrait for ::std::option::Option<T>
    where
        T: UninterpretedOptionTrait,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = T::Field2MessageType<'this>;

        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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
        fn identifier_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.identifier_value_opt())
        }
        fn positive_int_value_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            self.as_ref().and_then(|msg| msg.positive_int_value_opt())
        }
        fn negative_int_value_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            self.as_ref().and_then(|msg| msg.negative_int_value_opt())
        }
        fn double_value_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            self.as_ref().and_then(|msg| msg.double_value_opt())
        }
        fn string_value_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            self.as_ref().and_then(|msg| msg.string_value_opt())
        }
        fn aggregate_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            self.as_ref().and_then(|msg| msg.aggregate_value_opt())
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct UninterpretedOptionSingleField2<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    pub name: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::UninterpretedOption>
for UninterpretedOptionSingleField2<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::UninterpretedOptionTrait
for UninterpretedOptionSingleField2<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
type Field2MessageType<'this> where Self: 'this = &'this ScalarType;
type Field2RepeatedType<'this> where Self: 'this
    = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
    ::std::iter::IntoIterator::into_iter(&self.name)
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
for UninterpretedOptionSingleField2<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
            &self.name,
            2,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for UninterpretedOptionSingleField2<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            name: value,
        }
    }
}

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct UninterpretedOptionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub identifier_value: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::UninterpretedOption>
        for UninterpretedOptionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::UninterpretedOptionTrait
        for UninterpretedOptionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn identifier_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.identifier_value.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for UninterpretedOptionSingleField3<ScalarType>
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
            >(::std::iter::once(&self.identifier_value), 3, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for UninterpretedOptionSingleField3<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                identifier_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct UninterpretedOptionSingleField4 {
        pub positive_int_value: u64,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSingleField4 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSingleField4 {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn positive_int_value_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.positive_int_value))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for UninterpretedOptionSingleField4 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::UInt64>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.positive_int_value), 4, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<u64> for UninterpretedOptionSingleField4 {
        fn from(value: u64) -> Self {
            Self {
                positive_int_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct UninterpretedOptionSingleField5 {
        pub negative_int_value: i64,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSingleField5 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSingleField5 {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn negative_int_value_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.negative_int_value))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for UninterpretedOptionSingleField5 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Int64>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.negative_int_value), 5, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<i64> for UninterpretedOptionSingleField5 {
        fn from(value: i64) -> Self {
            Self {
                negative_int_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct UninterpretedOptionSingleField6 {
        pub double_value: f64,
    }

    impl ::puroro::Message<super::UninterpretedOption> for UninterpretedOptionSingleField6 {}

    impl super::_puroro_traits::UninterpretedOptionTrait for UninterpretedOptionSingleField6 {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn double_value_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::Some(::std::clone::Clone::clone(&self.double_value))
        }
    }

    impl ::puroro::internal::SerializableMessageToIoWrite for UninterpretedOptionSingleField6 {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Double>::ser_field::<
                (),
                _,
                _,
            >(::std::iter::once(&self.double_value), 6, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl ::std::convert::From<f64> for UninterpretedOptionSingleField6 {
        fn from(value: f64) -> Self {
            Self {
                double_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct UninterpretedOptionSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub string_value: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::UninterpretedOption>
        for UninterpretedOptionSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::UninterpretedOptionTrait
        for UninterpretedOptionSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn string_value_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::Some(self.string_value.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for UninterpretedOptionSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
        where
            W: ::std::io::Write,
        {
            use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
            SerFieldToIoWrite::<::puroro::tags::Optional, ::puroro::tags::Bytes>::ser_field::<
                ScalarType,
                _,
                _,
            >(::std::iter::once(&self.string_value), 7, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for UninterpretedOptionSingleField7<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<[u8]>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                string_value: value,
            }
        }
    }

    #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

    pub struct UninterpretedOptionSingleField8<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        pub aggregate_value: ScalarType,
    }

    impl<ScalarType> ::puroro::Message<super::UninterpretedOption>
        for UninterpretedOptionSingleField8<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
    }

    impl<ScalarType> super::_puroro_traits::UninterpretedOptionTrait
        for UninterpretedOptionSingleField8<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        type Field2MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field2RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field2MessageType<'this>>;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }

        fn aggregate_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::Some(self.aggregate_value.as_ref())
        }
    }

    impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for UninterpretedOptionSingleField8<ScalarType>
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
            >(::std::iter::once(&self.aggregate_value), 8, out)?;
            ::std::result::Result::Ok(())
        }
    }

    impl<ScalarType> ::std::convert::From<ScalarType> for UninterpretedOptionSingleField8<ScalarType>
    where
        ScalarType: ::std::convert::AsRef<str>
            + ::std::clone::Clone
            + ::std::cmp::PartialEq
            + ::std::fmt::Debug,
    {
        fn from(value: ScalarType) -> Self {
            Self {
                aggregate_value: value,
            }
        }
    }
    pub struct UninterpretedOptionBuilder<T>(T);

    impl<T> UninterpretedOptionBuilder<T>
    where
        T: UninterpretedOptionTrait,
    {
        pub fn append_name<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> UninterpretedOptionBuilder<(T, UninterpretedOptionSingleField2<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            UninterpretedOptionBuilder((self.0, UninterpretedOptionSingleField2 { name: value }))
        }

        pub fn append_identifier_value<ScalarType>(
            self,
            value: ScalarType,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSingleField3<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            UninterpretedOptionBuilder((
                self.0,
                UninterpretedOptionSingleField3 {
                    identifier_value: value,
                },
            ))
        }

        pub fn append_positive_int_value(
            self,
            value: u64,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSingleField4)> {
            UninterpretedOptionBuilder((
                self.0,
                UninterpretedOptionSingleField4 {
                    positive_int_value: value,
                },
            ))
        }

        pub fn append_negative_int_value(
            self,
            value: i64,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSingleField5)> {
            UninterpretedOptionBuilder((
                self.0,
                UninterpretedOptionSingleField5 {
                    negative_int_value: value,
                },
            ))
        }

        pub fn append_double_value(
            self,
            value: f64,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSingleField6)> {
            UninterpretedOptionBuilder((
                self.0,
                UninterpretedOptionSingleField6 {
                    double_value: value,
                },
            ))
        }

        pub fn append_string_value<ScalarType>(
            self,
            value: ScalarType,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSingleField7<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<[u8]>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            UninterpretedOptionBuilder((
                self.0,
                UninterpretedOptionSingleField7 {
                    string_value: value,
                },
            ))
        }

        pub fn append_aggregate_value<ScalarType>(
            self,
            value: ScalarType,
        ) -> UninterpretedOptionBuilder<(T, UninterpretedOptionSingleField8<ScalarType>)>
        where
            ScalarType: ::std::convert::AsRef<str>
                + ::std::clone::Clone
                + ::std::cmp::PartialEq
                + ::std::fmt::Debug,
        {
            UninterpretedOptionBuilder((
                self.0,
                UninterpretedOptionSingleField8 {
                    aggregate_value: value,
                },
            ))
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> SourceCodeInfoTrait for (T, U)
    where
        T: SourceCodeInfoTrait,
        U: SourceCodeInfoTrait,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as SourceCodeInfoTrait>::Field1MessageType<'this>,
            <U as SourceCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as SourceCodeInfoTrait>::Field1MessageType<'this>,
            <U as SourceCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = T::Field1MessageType<'this>;

        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct SourceCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    pub location: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::SourceCodeInfo>
for SourceCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::SourceCodeInfoTrait
for SourceCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
type Field1MessageType<'this> where Self: 'this = &'this ScalarType;
type Field1RepeatedType<'this> where Self: 'this
    = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
    ::std::iter::IntoIterator::into_iter(&self.location)
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
for SourceCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
            &self.location,
            1,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for SourceCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            location: value,
        }
    }
}
    pub struct SourceCodeInfoBuilder<T>(T);

    impl<T> SourceCodeInfoBuilder<T>
    where
        T: SourceCodeInfoTrait,
    {
        pub fn append_location<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> SourceCodeInfoBuilder<(T, SourceCodeInfoSingleField1<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            SourceCodeInfoBuilder((self.0, SourceCodeInfoSingleField1 { location: value }))
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ();
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::empty::EmptyRepeatedField<Self::Field1MessageType<'this>>;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
            ::puroro::internal::impls::empty::EmptyRepeatedField::new()
        }
    }
    impl<T, U> GeneratedCodeInfoTrait for (T, U)
    where
        T: GeneratedCodeInfoTrait,
        U: GeneratedCodeInfoTrait,
    {
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
            <U as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::merged::MergedRepeatedMessageField<
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = ::puroro::Either<
            <T as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
            <U as GeneratedCodeInfoTrait>::Field1MessageType<'this>,
        >;
        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::puroro::internal::impls::either::EitherRepeatedMessageField<
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
        type Field1MessageType<'this>
        where
            Self: 'this,
        = T::Field1MessageType<'this>;

        type Field1RepeatedType<'this>
        where
            Self: 'this,
        = ::std::iter::Flatten<
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

    pub struct GeneratedCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    pub annotation: RepeatedType,
}

    impl<ScalarType, RepeatedType> ::puroro::Message<super::GeneratedCodeInfo>
for GeneratedCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{}

    impl<ScalarType, RepeatedType> super::_puroro_traits::GeneratedCodeInfoTrait
for GeneratedCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
type Field1MessageType<'this> where Self: 'this = &'this ScalarType;
type Field1RepeatedType<'this> where Self: 'this
    = <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter;

fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
    ::std::iter::IntoIterator::into_iter(&self.annotation)
}
}

    impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
for GeneratedCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
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
            &self.annotation,
            1,
            out
        )?;
        ::std::result::Result::Ok(())
    }
}

    impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
for GeneratedCodeInfoSingleField1<ScalarType, RepeatedType>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
{
    fn from(value: RepeatedType) -> Self {
        Self {
            annotation: value,
        }
    }
}
    pub struct GeneratedCodeInfoBuilder<T>(T);

    impl<T> GeneratedCodeInfoBuilder<T>
    where
        T: GeneratedCodeInfoTrait,
    {
        pub fn append_annotation<ScalarType, RepeatedType>(self, value: RepeatedType)
        -> GeneratedCodeInfoBuilder<(T, GeneratedCodeInfoSingleField1<ScalarType, RepeatedType>)>
where
ScalarType: self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
for <'a> &'a RepeatedType: ::std::iter::IntoIterator<
    Item = &'a ScalarType
>,
    {
            GeneratedCodeInfoBuilder((self.0, GeneratedCodeInfoSingleField1 { annotation: value }))
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
            self::_puroro_root::google::protobuf::_puroro_traits::FileDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field1MessageType<'this>>
        where
            Self: 'this;
        fn file<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    macro_rules! file_descriptor_set_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field1MessageType<'this>;

            type Field1RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field1RepeatedType<'this>;
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

    impl<T> FileDescriptorSetTrait for &'_ mut T
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
        fn package<'this>(&'this self) -> &'this str {
            self.package_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_package<'this>(&'this self) -> bool {
            self.package_opt().is_some()
        }
        fn package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }

        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this>;

        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this>;

        type Field11RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = i32>
        where
            Self: 'this;
        fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this>;
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>
        where
            Self: 'this;
        fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field5MessageType<'this>>
        where
            Self: 'this;
        fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field6MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6MessageType<'this>>
        where
            Self: 'this;
        fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field7MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field7RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field7MessageType<'this>>
        where
            Self: 'this;
        fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this>;
        type Field8MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FileOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            ::std::option::Option::None
        }
        type Field9MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::SourceCodeInfoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn source_code_info<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            self.source_code_info_opt()
        }
        fn has_source_code_info<'this>(&'this self) -> bool {
            self.source_code_info_opt().is_some()
        }
        fn source_code_info_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
            ::std::option::Option::None
        }
        fn syntax<'this>(&'this self) -> &'this str {
            self.syntax_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_syntax<'this>(&'this self) -> bool {
            self.syntax_opt().is_some()
        }
        fn syntax_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
    }

    macro_rules! file_descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            fn package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).package_opt()
            }

            type Field3RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field3RepeatedType<'this>;
            fn dependency<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).dependency()
            }

            type Field10RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field10RepeatedType<'this>;
            fn public_dependency<'this>(&'this self) -> Self::Field10RepeatedType<'this> {
                (**self).public_dependency()
            }

            type Field11RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field11RepeatedType<'this>;
            fn weak_dependency<'this>(&'this self) -> Self::Field11RepeatedType<'this> {
                (**self).weak_dependency()
            }
            type Field4MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field4MessageType<'this>;

            type Field4RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field4RepeatedType<'this>;
            fn message_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).message_type()
            }
            type Field5MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field5MessageType<'this>;

            type Field5RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field5RepeatedType<'this>;
            fn enum_type<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
                (**self).enum_type()
            }
            type Field6MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field6MessageType<'this>;

            type Field6RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field6RepeatedType<'this>;
            fn service<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
                (**self).service()
            }
            type Field7MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field7MessageType<'this>;

            type Field7RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field7RepeatedType<'this>;
            fn extension<'this>(&'this self) -> Self::Field7RepeatedType<'this> {
                (**self).extension()
            }
            type Field8MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field8MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
                (**self).options_opt()
            }
            type Field9MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field9MessageType<'this>;
            fn source_code_info_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field9MessageType<'this>> {
                (**self).source_code_info_opt()
            }
            fn syntax_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).syntax_opt()
            }
        };
    }

    impl<T> FileDescriptorProtoTrait for &'_ T
    where
        T: FileDescriptorProtoTrait,
    {
        file_descriptor_proto_delegate!(T);
    }

    impl<T> FileDescriptorProtoTrait for &'_ mut T
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
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>
        where
            Self: 'this;
        fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field6MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field6MessageType<'this>>
        where
            Self: 'this;
        fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this>;
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::DescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field3RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field3MessageType<'this>>
        where
            Self: 'this;
        fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this>;
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>
        where
            Self: 'this;
        fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this>;
        type Field5MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ExtensionRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field5MessageType<'this>>
        where
            Self: 'this;
        fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
        type Field8MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::OneofDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field8RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field8MessageType<'this>>
        where
            Self: 'this;
        fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this>;
        type Field7MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::MessageOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
            ::std::option::Option::None
        }
        type Field9MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::descriptor_proto::_puroro_traits::ReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field9RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field9MessageType<'this>>
        where
            Self: 'this;
        fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this>;

        type Field10RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn reserved_name<'this>(&'this self) -> Self::Field10RepeatedType<'this>;
    }

    macro_rules! descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            type Field2MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field2MessageType<'this>;

            type Field2RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field2RepeatedType<'this>;
            fn field<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).field()
            }
            type Field6MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field6MessageType<'this>;

            type Field6RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field6RepeatedType<'this>;
            fn extension<'this>(&'this self) -> Self::Field6RepeatedType<'this> {
                (**self).extension()
            }
            type Field3MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field3MessageType<'this>;

            type Field3RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field3RepeatedType<'this>;
            fn nested_type<'this>(&'this self) -> Self::Field3RepeatedType<'this> {
                (**self).nested_type()
            }
            type Field4MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field4MessageType<'this>;

            type Field4RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field4RepeatedType<'this>;
            fn enum_type<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).enum_type()
            }
            type Field5MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field5MessageType<'this>;

            type Field5RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field5RepeatedType<'this>;
            fn extension_range<'this>(&'this self) -> Self::Field5RepeatedType<'this> {
                (**self).extension_range()
            }
            type Field8MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field8MessageType<'this>;

            type Field8RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field8RepeatedType<'this>;
            fn oneof_decl<'this>(&'this self) -> Self::Field8RepeatedType<'this> {
                (**self).oneof_decl()
            }
            type Field7MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field7MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field7MessageType<'this>> {
                (**self).options_opt()
            }
            type Field9MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field9MessageType<'this>;

            type Field9RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field9RepeatedType<'this>;
            fn reserved_range<'this>(&'this self) -> Self::Field9RepeatedType<'this> {
                (**self).reserved_range()
            }

            type Field10RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field10RepeatedType<'this>;
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

    impl<T> DescriptorProtoTrait for &'_ mut T
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
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! extension_range_options_delegate {
        ($ty:ty) => {
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> ExtensionRangeOptionsTrait for &'_ mut T
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
        fn number<'this>(&'this self) -> i32 {
            self.number_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_number<'this>(&'this self) -> bool {
            self.number_opt().is_some()
        }
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn label<'this>(
            &'this self,
        ) -> self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label
        {
            self.label_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_label<'this>(&'this self) -> bool {
            self.label_opt().is_some()
        }
        fn label_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
        > {
            ::std::option::Option::None
        }
        fn r#type<'this>(
            &'this self,
        ) -> self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type
        {
            self.type_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_type<'this>(&'this self) -> bool {
            self.type_opt().is_some()
        }
        fn type_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
        > {
            ::std::option::Option::None
        }
        fn type_name<'this>(&'this self) -> &'this str {
            self.type_name_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_type_name<'this>(&'this self) -> bool {
            self.type_name_opt().is_some()
        }
        fn type_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn extendee<'this>(&'this self) -> &'this str {
            self.extendee_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_extendee<'this>(&'this self) -> bool {
            self.extendee_opt().is_some()
        }
        fn extendee_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn default_value<'this>(&'this self) -> &'this str {
            self.default_value_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_default_value<'this>(&'this self) -> bool {
            self.default_value_opt().is_some()
        }
        fn default_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn oneof_index<'this>(&'this self) -> i32 {
            self.oneof_index_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_oneof_index<'this>(&'this self) -> bool {
            self.oneof_index_opt().is_some()
        }
        fn oneof_index_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        fn json_name<'this>(&'this self) -> &'this str {
            self.json_name_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_json_name<'this>(&'this self) -> bool {
            self.json_name_opt().is_some()
        }
        fn json_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        type Field8MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::FieldOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
            ::std::option::Option::None
        }
        fn proto3_optional<'this>(&'this self) -> bool {
            self.proto3_optional_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_proto3_optional<'this>(&'this self) -> bool {
            self.proto3_optional_opt().is_some()
        }
        fn proto3_optional_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
    }

    macro_rules! field_descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).number_opt()
            }
            fn label_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Label,
            > {
                (**self).label_opt()
            }
            fn type_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_descriptor_proto::Type,
            > {
                (**self).type_opt()
            }
            fn type_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).type_name_opt()
            }
            fn extendee_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).extendee_opt()
            }
            fn default_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).default_value_opt()
            }
            fn oneof_index_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).oneof_index_opt()
            }
            fn json_name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).json_name_opt()
            }
            type Field8MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field8MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field8MessageType<'this>> {
                (**self).options_opt()
            }
            fn proto3_optional_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).proto3_optional_opt()
            }
        };
    }

    impl<T> FieldDescriptorProtoTrait for &'_ T
    where
        T: FieldDescriptorProtoTrait,
    {
        field_descriptor_proto_delegate!(T);
    }

    impl<T> FieldDescriptorProtoTrait for &'_ mut T
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
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::OneofOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field2MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field2MessageType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! oneof_descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            type Field2MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field2MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field2MessageType<'this>> {
                (**self).options_opt()
            }
        };
    }

    impl<T> OneofDescriptorProtoTrait for &'_ T
    where
        T: OneofDescriptorProtoTrait,
    {
        oneof_descriptor_proto_delegate!(T);
    }

    impl<T> OneofDescriptorProtoTrait for &'_ mut T
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
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>
        where
            Self: 'this;
        fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::None
        }
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_nested::enum_descriptor_proto::_puroro_traits::EnumReservedRangeTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field4RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field4MessageType<'this>>
        where
            Self: 'this;
        fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this>;

        type Field5RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = &'this str>
        where
            Self: 'this;
        fn reserved_name<'this>(&'this self) -> Self::Field5RepeatedType<'this>;
    }

    macro_rules! enum_descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            type Field2MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field2MessageType<'this>;

            type Field2RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field2RepeatedType<'this>;
            fn value<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).value()
            }
            type Field3MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field3MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).options_opt()
            }
            type Field4MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field4MessageType<'this>;

            type Field4RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field4RepeatedType<'this>;
            fn reserved_range<'this>(&'this self) -> Self::Field4RepeatedType<'this> {
                (**self).reserved_range()
            }

            type Field5RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field5RepeatedType<'this>;
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

    impl<T> EnumDescriptorProtoTrait for &'_ mut T
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
        fn number<'this>(&'this self) -> i32 {
            self.number_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_number<'this>(&'this self) -> bool {
            self.number_opt().is_some()
        }
        fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
            ::std::option::Option::None
        }
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::EnumValueOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! enum_value_descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            fn number_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                (**self).number_opt()
            }
            type Field3MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field3MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).options_opt()
            }
        };
    }

    impl<T> EnumValueDescriptorProtoTrait for &'_ T
    where
        T: EnumValueDescriptorProtoTrait,
    {
        enum_value_descriptor_proto_delegate!(T);
    }

    impl<T> EnumValueDescriptorProtoTrait for &'_ mut T
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
        type Field2MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodDescriptorProtoTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>
        where
            Self: 'this;
        fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        type Field3MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::ServiceOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::None
        }
    }

    macro_rules! service_descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            type Field2MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field2MessageType<'this>;

            type Field2RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field2RepeatedType<'this>;
            fn method<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).method()
            }
            type Field3MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field3MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                (**self).options_opt()
            }
        };
    }

    impl<T> ServiceDescriptorProtoTrait for &'_ T
    where
        T: ServiceDescriptorProtoTrait,
    {
        service_descriptor_proto_delegate!(T);
    }

    impl<T> ServiceDescriptorProtoTrait for &'_ mut T
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
        fn input_type<'this>(&'this self) -> &'this str {
            self.input_type_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_input_type<'this>(&'this self) -> bool {
            self.input_type_opt().is_some()
        }
        fn input_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn output_type<'this>(&'this self) -> &'this str {
            self.output_type_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_output_type<'this>(&'this self) -> bool {
            self.output_type_opt().is_some()
        }
        fn output_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        type Field4MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::MethodOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;
        fn options<'this>(&'this self) -> ::std::option::Option<Self::Field4MessageType<'this>> {
            self.options_opt()
        }
        fn has_options<'this>(&'this self) -> bool {
            self.options_opt().is_some()
        }
        fn options_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<Self::Field4MessageType<'this>> {
            ::std::option::Option::None
        }
        fn client_streaming<'this>(&'this self) -> bool {
            self.client_streaming_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_client_streaming<'this>(&'this self) -> bool {
            self.client_streaming_opt().is_some()
        }
        fn client_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn server_streaming<'this>(&'this self) -> bool {
            self.server_streaming_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_server_streaming<'this>(&'this self) -> bool {
            self.server_streaming_opt().is_some()
        }
        fn server_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
    }

    macro_rules! method_descriptor_proto_delegate {
        ($ty:ty) => {
            fn name_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).name_opt()
            }
            fn input_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).input_type_opt()
            }
            fn output_type_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).output_type_opt()
            }
            type Field4MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field4MessageType<'this>;
            fn options_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<Self::Field4MessageType<'this>> {
                (**self).options_opt()
            }
            fn client_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).client_streaming_opt()
            }
            fn server_streaming_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).server_streaming_opt()
            }
        };
    }

    impl<T> MethodDescriptorProtoTrait for &'_ T
    where
        T: MethodDescriptorProtoTrait,
    {
        method_descriptor_proto_delegate!(T);
    }

    impl<T> MethodDescriptorProtoTrait for &'_ mut T
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
        fn java_package<'this>(&'this self) -> &'this str {
            self.java_package_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_java_package<'this>(&'this self) -> bool {
            self.java_package_opt().is_some()
        }
        fn java_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn java_outer_classname<'this>(&'this self) -> &'this str {
            self.java_outer_classname_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_java_outer_classname<'this>(&'this self) -> bool {
            self.java_outer_classname_opt().is_some()
        }
        fn java_outer_classname_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn java_multiple_files<'this>(&'this self) -> bool {
            self.java_multiple_files_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_java_multiple_files<'this>(&'this self) -> bool {
            self.java_multiple_files_opt().is_some()
        }
        fn java_multiple_files_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn java_generate_equals_and_hash<'this>(&'this self) -> bool {
            self.java_generate_equals_and_hash_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_java_generate_equals_and_hash<'this>(&'this self) -> bool {
            self.java_generate_equals_and_hash_opt().is_some()
        }
        fn java_generate_equals_and_hash_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn java_string_check_utf8<'this>(&'this self) -> bool {
            self.java_string_check_utf8_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_java_string_check_utf8<'this>(&'this self) -> bool {
            self.java_string_check_utf8_opt().is_some()
        }
        fn java_string_check_utf8_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn optimize_for<'this>(
            &'this self,
        ) -> self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode
        {
            self.optimize_for_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_optimize_for<'this>(&'this self) -> bool {
            self.optimize_for_opt().is_some()
        }
        fn optimize_for_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
        > {
            ::std::option::Option::None
        }
        fn go_package<'this>(&'this self) -> &'this str {
            self.go_package_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_go_package<'this>(&'this self) -> bool {
            self.go_package_opt().is_some()
        }
        fn go_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn cc_generic_services<'this>(&'this self) -> bool {
            self.cc_generic_services_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_cc_generic_services<'this>(&'this self) -> bool {
            self.cc_generic_services_opt().is_some()
        }
        fn cc_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn java_generic_services<'this>(&'this self) -> bool {
            self.java_generic_services_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_java_generic_services<'this>(&'this self) -> bool {
            self.java_generic_services_opt().is_some()
        }
        fn java_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn py_generic_services<'this>(&'this self) -> bool {
            self.py_generic_services_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_py_generic_services<'this>(&'this self) -> bool {
            self.py_generic_services_opt().is_some()
        }
        fn py_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn php_generic_services<'this>(&'this self) -> bool {
            self.php_generic_services_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_php_generic_services<'this>(&'this self) -> bool {
            self.php_generic_services_opt().is_some()
        }
        fn php_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt().is_some()
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn cc_enable_arenas<'this>(&'this self) -> bool {
            self.cc_enable_arenas_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_cc_enable_arenas<'this>(&'this self) -> bool {
            self.cc_enable_arenas_opt().is_some()
        }
        fn cc_enable_arenas_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn objc_class_prefix<'this>(&'this self) -> &'this str {
            self.objc_class_prefix_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_objc_class_prefix<'this>(&'this self) -> bool {
            self.objc_class_prefix_opt().is_some()
        }
        fn objc_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn csharp_namespace<'this>(&'this self) -> &'this str {
            self.csharp_namespace_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_csharp_namespace<'this>(&'this self) -> bool {
            self.csharp_namespace_opt().is_some()
        }
        fn csharp_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn swift_prefix<'this>(&'this self) -> &'this str {
            self.swift_prefix_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_swift_prefix<'this>(&'this self) -> bool {
            self.swift_prefix_opt().is_some()
        }
        fn swift_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn php_class_prefix<'this>(&'this self) -> &'this str {
            self.php_class_prefix_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_php_class_prefix<'this>(&'this self) -> bool {
            self.php_class_prefix_opt().is_some()
        }
        fn php_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn php_namespace<'this>(&'this self) -> &'this str {
            self.php_namespace_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_php_namespace<'this>(&'this self) -> bool {
            self.php_namespace_opt().is_some()
        }
        fn php_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn php_metadata_namespace<'this>(&'this self) -> &'this str {
            self.php_metadata_namespace_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_php_metadata_namespace<'this>(&'this self) -> bool {
            self.php_metadata_namespace_opt().is_some()
        }
        fn php_metadata_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn ruby_package<'this>(&'this self) -> &'this str {
            self.ruby_package_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_ruby_package<'this>(&'this self) -> bool {
            self.ruby_package_opt().is_some()
        }
        fn ruby_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! file_options_delegate {
        ($ty:ty) => {
            fn java_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).java_package_opt()
            }
            fn java_outer_classname_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).java_outer_classname_opt()
            }
            fn java_multiple_files_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).java_multiple_files_opt()
            }
            fn java_generate_equals_and_hash_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<bool> {
                (**self).java_generate_equals_and_hash_opt()
            }
            fn java_string_check_utf8_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).java_string_check_utf8_opt()
            }
            fn optimize_for_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::file_options::OptimizeMode,
            > {
                (**self).optimize_for_opt()
            }
            fn go_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).go_package_opt()
            }
            fn cc_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).cc_generic_services_opt()
            }
            fn java_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).java_generic_services_opt()
            }
            fn py_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).py_generic_services_opt()
            }
            fn php_generic_services_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).php_generic_services_opt()
            }
            fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated_opt()
            }
            fn cc_enable_arenas_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).cc_enable_arenas_opt()
            }
            fn objc_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).objc_class_prefix_opt()
            }
            fn csharp_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).csharp_namespace_opt()
            }
            fn swift_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).swift_prefix_opt()
            }
            fn php_class_prefix_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).php_class_prefix_opt()
            }
            fn php_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).php_namespace_opt()
            }
            fn php_metadata_namespace_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).php_metadata_namespace_opt()
            }
            fn ruby_package_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).ruby_package_opt()
            }
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> FileOptionsTrait for &'_ mut T
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
        fn message_set_wire_format<'this>(&'this self) -> bool {
            self.message_set_wire_format_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_message_set_wire_format<'this>(&'this self) -> bool {
            self.message_set_wire_format_opt().is_some()
        }
        fn message_set_wire_format_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn no_standard_descriptor_accessor<'this>(&'this self) -> bool {
            self.no_standard_descriptor_accessor_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_no_standard_descriptor_accessor<'this>(&'this self) -> bool {
            self.no_standard_descriptor_accessor_opt().is_some()
        }
        fn no_standard_descriptor_accessor_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt().is_some()
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn map_entry<'this>(&'this self) -> bool {
            self.map_entry_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_map_entry<'this>(&'this self) -> bool {
            self.map_entry_opt().is_some()
        }
        fn map_entry_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! message_options_delegate {
        ($ty:ty) => {
            fn message_set_wire_format_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).message_set_wire_format_opt()
            }
            fn no_standard_descriptor_accessor_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<bool> {
                (**self).no_standard_descriptor_accessor_opt()
            }
            fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated_opt()
            }
            fn map_entry_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).map_entry_opt()
            }
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> MessageOptionsTrait for &'_ mut T
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
        ) -> self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype {
            self.ctype_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_ctype<'this>(&'this self) -> bool {
            self.ctype_opt().is_some()
        }
        fn ctype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
        > {
            ::std::option::Option::None
        }
        fn packed<'this>(&'this self) -> bool {
            self.packed_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_packed<'this>(&'this self) -> bool {
            self.packed_opt().is_some()
        }
        fn packed_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn jstype<'this>(
            &'this self,
        ) -> self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype {
            self.jstype_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_jstype<'this>(&'this self) -> bool {
            self.jstype_opt().is_some()
        }
        fn jstype_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
        > {
            ::std::option::Option::None
        }
        fn lazy<'this>(&'this self) -> bool {
            self.lazy_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_lazy<'this>(&'this self) -> bool {
            self.lazy_opt().is_some()
        }
        fn lazy_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt().is_some()
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn weak<'this>(&'this self) -> bool {
            self.weak_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_weak<'this>(&'this self) -> bool {
            self.weak_opt().is_some()
        }
        fn weak_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! field_options_delegate {
        ($ty:ty) => {
            fn ctype_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Ctype,
            > {
                (**self).ctype_opt()
            }
            fn packed_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).packed_opt()
            }
            fn jstype_opt<'this>(
                &'this self,
            ) -> ::std::option::Option<
                self::_puroro_root::google::protobuf::_puroro_nested::field_options::Jstype,
            > {
                (**self).jstype_opt()
            }
            fn lazy_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).lazy_opt()
            }
            fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated_opt()
            }
            fn weak_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).weak_opt()
            }
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> FieldOptionsTrait for &'_ mut T
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
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! oneof_options_delegate {
        ($ty:ty) => {
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> OneofOptionsTrait for &'_ mut T
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
        fn allow_alias<'this>(&'this self) -> bool {
            self.allow_alias_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_allow_alias<'this>(&'this self) -> bool {
            self.allow_alias_opt().is_some()
        }
        fn allow_alias_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt().is_some()
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! enum_options_delegate {
        ($ty:ty) => {
            fn allow_alias_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).allow_alias_opt()
            }
            fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated_opt()
            }
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> EnumOptionsTrait for &'_ mut T
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
        fn deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt().is_some()
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! enum_value_options_delegate {
        ($ty:ty) => {
            fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated_opt()
            }
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> EnumValueOptionsTrait for &'_ mut T
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
        fn deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt().is_some()
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! service_options_delegate {
        ($ty:ty) => {
            fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated_opt()
            }
            type Field999MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> ServiceOptionsTrait for &'_ mut T
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
        fn deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_deprecated<'this>(&'this self) -> bool {
            self.deprecated_opt().is_some()
        }
        fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
            ::std::option::Option::None
        }
        fn idempotency_level<'this>(
            &'this self,
        ) -> self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel
        {
            self.idempotency_level_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_idempotency_level<'this>(&'this self) -> bool {
            self.idempotency_level_opt().is_some()
        }
        fn idempotency_level_opt<'this>(
            &'this self,
        ) -> ::std::option::Option<
            self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel,
        > {
            ::std::option::Option::None
        }
        type Field999MessageType<'this>:
            self::_puroro_root::google::protobuf::_puroro_traits::UninterpretedOptionTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field999RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field999MessageType<'this>>
        where
            Self: 'this;
        fn uninterpreted_option<'this>(&'this self) -> Self::Field999RepeatedType<'this>;
    }

    macro_rules! method_options_delegate {
        ($ty:ty) => {
            fn deprecated_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                (**self).deprecated_opt()
            }
            fn idempotency_level_opt<'this>(&'this self) -> ::std::option::Option<self::_puroro_root::google::protobuf::_puroro_nested::method_options::IdempotencyLevel> {
                (**self).idempotency_level_opt()
            }
            type Field999MessageType<'this> where Self: 'this = <$ty>::Field999MessageType<'this>;

            type Field999RepeatedType<'this> where Self: 'this = <$ty>::Field999RepeatedType<'this>;
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

    impl<T> MethodOptionsTrait for &'_ mut T
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
            self::_puroro_root::google::protobuf::_puroro_nested::uninterpreted_option::_puroro_traits::NamePartTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field2MessageType<'this>>
        where
            Self: 'this;
        fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
        fn identifier_value<'this>(&'this self) -> &'this str {
            self.identifier_value_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_identifier_value<'this>(&'this self) -> bool {
            self.identifier_value_opt().is_some()
        }
        fn identifier_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
        fn positive_int_value<'this>(&'this self) -> u64 {
            self.positive_int_value_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_positive_int_value<'this>(&'this self) -> bool {
            self.positive_int_value_opt().is_some()
        }
        fn positive_int_value_opt<'this>(&'this self) -> ::std::option::Option<u64> {
            ::std::option::Option::None
        }
        fn negative_int_value<'this>(&'this self) -> i64 {
            self.negative_int_value_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_negative_int_value<'this>(&'this self) -> bool {
            self.negative_int_value_opt().is_some()
        }
        fn negative_int_value_opt<'this>(&'this self) -> ::std::option::Option<i64> {
            ::std::option::Option::None
        }
        fn double_value<'this>(&'this self) -> f64 {
            self.double_value_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_double_value<'this>(&'this self) -> bool {
            self.double_value_opt().is_some()
        }
        fn double_value_opt<'this>(&'this self) -> ::std::option::Option<f64> {
            ::std::option::Option::None
        }
        fn string_value<'this>(&'this self) -> &'this [u8] {
            self.string_value_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_string_value<'this>(&'this self) -> bool {
            self.string_value_opt().is_some()
        }
        fn string_value_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
            ::std::option::Option::None
        }
        fn aggregate_value<'this>(&'this self) -> &'this str {
            self.aggregate_value_opt()
                .unwrap_or_else(::std::default::Default::default)
        }
        fn has_aggregate_value<'this>(&'this self) -> bool {
            self.aggregate_value_opt().is_some()
        }
        fn aggregate_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
            ::std::option::Option::None
        }
    }

    macro_rules! uninterpreted_option_delegate {
        ($ty:ty) => {
            type Field2MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field2MessageType<'this>;

            type Field2RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field2RepeatedType<'this>;
            fn name<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                (**self).name()
            }
            fn identifier_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).identifier_value_opt()
            }
            fn positive_int_value_opt<'this>(&'this self) -> ::std::option::Option<u64> {
                (**self).positive_int_value_opt()
            }
            fn negative_int_value_opt<'this>(&'this self) -> ::std::option::Option<i64> {
                (**self).negative_int_value_opt()
            }
            fn double_value_opt<'this>(&'this self) -> ::std::option::Option<f64> {
                (**self).double_value_opt()
            }
            fn string_value_opt<'this>(&'this self) -> ::std::option::Option<&'this [u8]> {
                (**self).string_value_opt()
            }
            fn aggregate_value_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                (**self).aggregate_value_opt()
            }
        };
    }

    impl<T> UninterpretedOptionTrait for &'_ T
    where
        T: UninterpretedOptionTrait,
    {
        uninterpreted_option_delegate!(T);
    }

    impl<T> UninterpretedOptionTrait for &'_ mut T
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
            self::_puroro_root::google::protobuf::_puroro_nested::source_code_info::_puroro_traits::LocationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field1MessageType<'this>>
        where
            Self: 'this;
        fn location<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    macro_rules! source_code_info_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field1MessageType<'this>;

            type Field1RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field1RepeatedType<'this>;
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

    impl<T> SourceCodeInfoTrait for &'_ mut T
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
            self::_puroro_root::google::protobuf::_puroro_nested::generated_code_info::_puroro_traits::AnnotationTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
            where Self: 'this;

        type Field1RepeatedType<'this>: ::puroro::RepeatedField<'this>
            + ::std::iter::IntoIterator<Item = Self::Field1MessageType<'this>>
        where
            Self: 'this;
        fn annotation<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
    }

    macro_rules! generated_code_info_delegate {
        ($ty:ty) => {
            type Field1MessageType<'this>
            where
                Self: 'this,
            = <$ty>::Field1MessageType<'this>;

            type Field1RepeatedType<'this>
            where
                Self: 'this,
            = <$ty>::Field1RepeatedType<'this>;
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

    impl<T> GeneratedCodeInfoTrait for &'_ mut T
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
                fn start_opt<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.end)
                }
                type Field3MessageType<'this> where Self: 'this = &'this self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions;
                fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
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

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for ExtensionRange {
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
                    >::deser_field(&mut self.start, data),
                    2 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Int32
                    >::deser_field(&mut self.end, data),
                    3 => DeserFieldFromBytesIter::<
                        ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>>
                    >::deser_field(&mut self.options, data),
        
                    _ => unimplemented!("TODO: This case should be handled properly..."),
                }
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for ExtensionRange {
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
                    ::puroro::tags::Optional, ::puroro::tags::Message<::std::boxed::Box<self::_puroro_root::google::protobuf::_puroro_simple_impl::ExtensionRangeOptions>>
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
                fn start_opt<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
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

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for ReservedRange {
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

            impl ::puroro::internal::SerializableMessageToIoWrite for ReservedRange {
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
                type Field3MessageType<'this>
                where
                    Self: 'this,
                = ();
            }
            impl<T, U> ExtensionRangeTrait for (T, U)
            where
                T: ExtensionRangeTrait,
                U: ExtensionRangeTrait,
            {
                fn start_opt<'this>(&'this self) -> Option<i32> {
                    <U as ExtensionRangeTrait>::start_opt(&self.1)
                        .or_else(|| <T as ExtensionRangeTrait>::start_opt(&self.0))
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
                    <U as ExtensionRangeTrait>::end_opt(&self.1)
                        .or_else(|| <T as ExtensionRangeTrait>::end_opt(&self.0))
                }
                type Field3MessageType<'this>
                where
                    Self: 'this,
                = (
                    ::std::option::Option<<T as ExtensionRangeTrait>::Field3MessageType<'this>>,
                    ::std::option::Option<<U as ExtensionRangeTrait>::Field3MessageType<'this>>,
                );
                fn options_opt<'this>(&'this self) -> Option<Self::Field3MessageType<'this>> {
                    match (
                        <T as ExtensionRangeTrait>::options_opt(&self.0),
                        <U as ExtensionRangeTrait>::options_opt(&self.1),
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
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as ExtensionRangeTrait>::start_opt(t),
                        |u| <U as ExtensionRangeTrait>::start_opt(u),
                    )
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as ExtensionRangeTrait>::end_opt(t),
                        |u| <U as ExtensionRangeTrait>::end_opt(u),
                    )
                }
                type Field3MessageType<'this>
                where
                    Self: 'this,
                = ::puroro::Either<
                    <T as ExtensionRangeTrait>::Field3MessageType<'this>,
                    <U as ExtensionRangeTrait>::Field3MessageType<'this>,
                >;
                fn options_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                    self.as_ref().either(
                        |t| {
                            <T as ExtensionRangeTrait>::options_opt(t)
                                .map(|t| ::puroro::Either::Left(t))
                        },
                        |u| {
                            <U as ExtensionRangeTrait>::options_opt(u)
                                .map(|u| ::puroro::Either::Right(u))
                        },
                    )
                }
            }
            impl<T> ExtensionRangeTrait for ::std::option::Option<T>
            where
                T: ExtensionRangeTrait,
            {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.start_opt())
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end_opt())
                }
                type Field3MessageType<'this>
                where
                    Self: 'this,
                = T::Field3MessageType<'this>;
                fn options_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                    self.as_ref().and_then(|msg| msg.options_opt())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct ExtensionRangeSingleField1 {
                pub start: i32,
            }

            impl ::puroro::Message<super::ExtensionRange> for ExtensionRangeSingleField1 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSingleField1 {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.start))
                }
                type Field3MessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for ExtensionRangeSingleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.start),
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for ExtensionRangeSingleField1 {
                fn from(value: i32) -> Self {
                    Self { start: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct ExtensionRangeSingleField2 {
                pub end: i32,
            }

            impl ::puroro::Message<super::ExtensionRange> for ExtensionRangeSingleField2 {}

            impl super::_puroro_traits::ExtensionRangeTrait for ExtensionRangeSingleField2 {
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.end))
                }
                type Field3MessageType<'this>
                where
                    Self: 'this,
                = ();
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for ExtensionRangeSingleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.end),
                    2,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for ExtensionRangeSingleField2 {
                fn from(value: i32) -> Self {
                    Self { end: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct ExtensionRangeSingleField3<ScalarType>
        where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
        {
            pub options: ScalarType,
        }

            impl<ScalarType> ::puroro::Message<super::ExtensionRange>
        for ExtensionRangeSingleField3<ScalarType>
        where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
        {}

            impl<ScalarType> super::_puroro_traits::ExtensionRangeTrait
        for ExtensionRangeSingleField3<ScalarType>
        where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
        {
        type Field3MessageType<'this> where Self: 'this = &'this ScalarType;
        
        fn options_opt<'this>(&'this self) -> ::std::option::Option<Self::Field3MessageType<'this>> {
            ::std::option::Option::Some(&self.options)
        }
        }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
        for ExtensionRangeSingleField3<ScalarType>
        where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
            ScalarType: ::puroro::internal::SerializableMessageToIoWrite,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Message<ScalarType>
                >::ser_field::
                <ScalarType, _, _>
                (
                    ::std::iter::once(&self.options),
                    3,
                    out
                )?;
                ::std::result::Result::Ok(())
            }
        }

            impl<ScalarType> ::std::convert::From<ScalarType>
        for ExtensionRangeSingleField3<ScalarType>
        where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
        {
            fn from(value: ScalarType) -> Self {
                Self {
                    options: value,
                }
            }
        }
            pub struct ExtensionRangeBuilder<T>(T);

            impl<T> ExtensionRangeBuilder<T>
            where
                T: ExtensionRangeTrait,
            {
                pub fn append_start(
                    self,
                    value: i32,
                ) -> ExtensionRangeBuilder<(T, ExtensionRangeSingleField1)> {
                    ExtensionRangeBuilder((self.0, ExtensionRangeSingleField1 { start: value }))
                }

                pub fn append_end(
                    self,
                    value: i32,
                ) -> ExtensionRangeBuilder<(T, ExtensionRangeSingleField2)> {
                    ExtensionRangeBuilder((self.0, ExtensionRangeSingleField2 { end: value }))
                }
        
            pub fn append_options<ScalarType>(self, value: ScalarType)
                -> ExtensionRangeBuilder<(T, ExtensionRangeSingleField3<ScalarType>)>
        where
        ScalarType: self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug,
            {
                    ExtensionRangeBuilder((self.0, ExtensionRangeSingleField3 { options: value }))
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
                fn start_opt<'this>(&'this self) -> Option<i32> {
                    <U as ReservedRangeTrait>::start_opt(&self.1)
                        .or_else(|| <T as ReservedRangeTrait>::start_opt(&self.0))
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
                    <U as ReservedRangeTrait>::end_opt(&self.1)
                        .or_else(|| <T as ReservedRangeTrait>::end_opt(&self.0))
                }
            }
            impl<T, U> ReservedRangeTrait for ::puroro::Either<T, U>
            where
                T: ReservedRangeTrait,
                U: ReservedRangeTrait,
            {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as ReservedRangeTrait>::start_opt(t),
                        |u| <U as ReservedRangeTrait>::start_opt(u),
                    )
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as ReservedRangeTrait>::end_opt(t),
                        |u| <U as ReservedRangeTrait>::end_opt(u),
                    )
                }
            }
            impl<T> ReservedRangeTrait for ::std::option::Option<T>
            where
                T: ReservedRangeTrait,
            {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.start_opt())
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end_opt())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct ReservedRangeSingleField1 {
                pub start: i32,
            }

            impl ::puroro::Message<super::ReservedRange> for ReservedRangeSingleField1 {}

            impl super::_puroro_traits::ReservedRangeTrait for ReservedRangeSingleField1 {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.start))
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for ReservedRangeSingleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.start),
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for ReservedRangeSingleField1 {
                fn from(value: i32) -> Self {
                    Self { start: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct ReservedRangeSingleField2 {
                pub end: i32,
            }

            impl ::puroro::Message<super::ReservedRange> for ReservedRangeSingleField2 {}

            impl super::_puroro_traits::ReservedRangeTrait for ReservedRangeSingleField2 {
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.end))
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for ReservedRangeSingleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.end),
                    2,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for ReservedRangeSingleField2 {
                fn from(value: i32) -> Self {
                    Self { end: value }
                }
            }
            pub struct ReservedRangeBuilder<T>(T);

            impl<T> ReservedRangeBuilder<T>
            where
                T: ReservedRangeTrait,
            {
                pub fn append_start(
                    self,
                    value: i32,
                ) -> ReservedRangeBuilder<(T, ReservedRangeSingleField1)> {
                    ReservedRangeBuilder((self.0, ReservedRangeSingleField1 { start: value }))
                }

                pub fn append_end(
                    self,
                    value: i32,
                ) -> ReservedRangeBuilder<(T, ReservedRangeSingleField2)> {
                    ReservedRangeBuilder((self.0, ReservedRangeSingleField2 { end: value }))
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
                fn start<'this>(&'this self) -> i32 {
                    self.start_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_start<'this>(&'this self) -> bool {
                    self.start_opt().is_some()
                }
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
                fn end<'this>(&'this self) -> i32 {
                    self.end_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_end<'this>(&'this self) -> bool {
                    self.end_opt().is_some()
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
                type Field3MessageType<'this>:
                    self::_puroro_root::google::protobuf::_puroro_traits::ExtensionRangeOptionsTrait + ::std::clone::Clone + ::std::cmp::PartialEq + ::std::fmt::Debug
                    where Self: 'this;
                fn options<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                    self.options_opt()
                }
                fn has_options<'this>(&'this self) -> bool {
                    self.options_opt().is_some()
                }
                fn options_opt<'this>(
                    &'this self,
                ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                    ::std::option::Option::None
                }
            }

            macro_rules! extension_range_delegate {
                ($ty:ty) => {
                    fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).start_opt()
                    }
                    fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end_opt()
                    }
                    type Field3MessageType<'this>
                    where
                        Self: 'this,
                    = <$ty>::Field3MessageType<'this>;
                    fn options_opt<'this>(
                        &'this self,
                    ) -> ::std::option::Option<Self::Field3MessageType<'this>> {
                        (**self).options_opt()
                    }
                };
            }

            impl<T> ExtensionRangeTrait for &'_ T
            where
                T: ExtensionRangeTrait,
            {
                extension_range_delegate!(T);
            }

            impl<T> ExtensionRangeTrait for &'_ mut T
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
                fn start<'this>(&'this self) -> i32 {
                    self.start_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_start<'this>(&'this self) -> bool {
                    self.start_opt().is_some()
                }
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
                fn end<'this>(&'this self) -> i32 {
                    self.end_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_end<'this>(&'this self) -> bool {
                    self.end_opt().is_some()
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
            }

            macro_rules! reserved_range_delegate {
                ($ty:ty) => {
                    fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).start_opt()
                    }
                    fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end_opt()
                    }
                };
            }

            impl<T> ReservedRangeTrait for &'_ T
            where
                T: ReservedRangeTrait,
            {
                reserved_range_delegate!(T);
            }

            impl<T> ReservedRangeTrait for &'_ mut T
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
                fn start_opt<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.start)
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
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

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for EnumReservedRange {
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

            impl ::puroro::internal::SerializableMessageToIoWrite for EnumReservedRange {
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
                fn start_opt<'this>(&'this self) -> Option<i32> {
                    <U as EnumReservedRangeTrait>::start_opt(&self.1)
                        .or_else(|| <T as EnumReservedRangeTrait>::start_opt(&self.0))
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
                    <U as EnumReservedRangeTrait>::end_opt(&self.1)
                        .or_else(|| <T as EnumReservedRangeTrait>::end_opt(&self.0))
                }
            }
            impl<T, U> EnumReservedRangeTrait for ::puroro::Either<T, U>
            where
                T: EnumReservedRangeTrait,
                U: EnumReservedRangeTrait,
            {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as EnumReservedRangeTrait>::start_opt(t),
                        |u| <U as EnumReservedRangeTrait>::start_opt(u),
                    )
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as EnumReservedRangeTrait>::end_opt(t),
                        |u| <U as EnumReservedRangeTrait>::end_opt(u),
                    )
                }
            }
            impl<T> EnumReservedRangeTrait for ::std::option::Option<T>
            where
                T: EnumReservedRangeTrait,
            {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.start_opt())
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end_opt())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct EnumReservedRangeSingleField1 {
                pub start: i32,
            }

            impl ::puroro::Message<super::EnumReservedRange> for EnumReservedRangeSingleField1 {}

            impl super::_puroro_traits::EnumReservedRangeTrait for EnumReservedRangeSingleField1 {
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.start))
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for EnumReservedRangeSingleField1 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.start),
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for EnumReservedRangeSingleField1 {
                fn from(value: i32) -> Self {
                    Self { start: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct EnumReservedRangeSingleField2 {
                pub end: i32,
            }

            impl ::puroro::Message<super::EnumReservedRange> for EnumReservedRangeSingleField2 {}

            impl super::_puroro_traits::EnumReservedRangeTrait for EnumReservedRangeSingleField2 {
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.end))
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for EnumReservedRangeSingleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.end),
                    2,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for EnumReservedRangeSingleField2 {
                fn from(value: i32) -> Self {
                    Self { end: value }
                }
            }
            pub struct EnumReservedRangeBuilder<T>(T);

            impl<T> EnumReservedRangeBuilder<T>
            where
                T: EnumReservedRangeTrait,
            {
                pub fn append_start(
                    self,
                    value: i32,
                ) -> EnumReservedRangeBuilder<(T, EnumReservedRangeSingleField1)> {
                    EnumReservedRangeBuilder((
                        self.0,
                        EnumReservedRangeSingleField1 { start: value },
                    ))
                }

                pub fn append_end(
                    self,
                    value: i32,
                ) -> EnumReservedRangeBuilder<(T, EnumReservedRangeSingleField2)> {
                    EnumReservedRangeBuilder((self.0, EnumReservedRangeSingleField2 { end: value }))
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
                fn start<'this>(&'this self) -> i32 {
                    self.start_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_start<'this>(&'this self) -> bool {
                    self.start_opt().is_some()
                }
                fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
                fn end<'this>(&'this self) -> i32 {
                    self.end_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_end<'this>(&'this self) -> bool {
                    self.end_opt().is_some()
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
            }

            macro_rules! enum_reserved_range_delegate {
                ($ty:ty) => {
                    fn start_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).start_opt()
                    }
                    fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end_opt()
                    }
                };
            }

            impl<T> EnumReservedRangeTrait for &'_ T
            where
                T: EnumReservedRangeTrait,
            {
                enum_reserved_range_delegate!(T);
            }

            impl<T> EnumReservedRangeTrait for &'_ mut T
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
                pub name_part: ::std::option::Option<::std::string::String>,
                pub is_extension: ::std::option::Option<bool>,
            }
            impl ::puroro::Message<NamePart> for NamePart {}

            impl super::_puroro_traits::NamePartTrait for NamePart {
                fn name_part_opt<'this>(&'this self) -> Option<&'this str> {
                    self.name_part.as_ref().map(|v| v.as_ref())
                }
                fn is_extension_opt<'this>(&'this self) -> Option<bool> {
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

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for NamePart {
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

            impl ::puroro::internal::SerializableMessageToIoWrite for NamePart {
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
            impl NamePartTrait for () {}
            impl<T, U> NamePartTrait for (T, U)
            where
                T: NamePartTrait,
                U: NamePartTrait,
            {
                fn name_part_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as NamePartTrait>::name_part_opt(&self.1)
                        .or_else(|| <T as NamePartTrait>::name_part_opt(&self.0))
                }
                fn is_extension_opt<'this>(&'this self) -> Option<bool> {
                    <U as NamePartTrait>::is_extension_opt(&self.1)
                        .or_else(|| <T as NamePartTrait>::is_extension_opt(&self.0))
                }
            }
            impl<T, U> NamePartTrait for ::puroro::Either<T, U>
            where
                T: NamePartTrait,
                U: NamePartTrait,
            {
                fn name_part_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as NamePartTrait>::name_part_opt(t),
                        |u| <U as NamePartTrait>::name_part_opt(u),
                    )
                }
                fn is_extension_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                    self.as_ref().either(
                        |t| <T as NamePartTrait>::is_extension_opt(t),
                        |u| <U as NamePartTrait>::is_extension_opt(u),
                    )
                }
            }
            impl<T> NamePartTrait for ::std::option::Option<T>
            where
                T: NamePartTrait,
            {
                fn name_part_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.name_part_opt())
                }
                fn is_extension_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                    self.as_ref().and_then(|msg| msg.is_extension_opt())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct NamePartSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub name_part: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::NamePart> for NamePartSingleField1<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::NamePartTrait for NamePartSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn name_part_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.name_part.as_ref())
                }
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
                for NamePartSingleField1<ScalarType>
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
                    ::puroro::tags::Required, ::puroro::tags::String
                >::ser_field::
                <ScalarType, _, _>
                (
                    ::std::iter::once(&self.name_part),
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for NamePartSingleField1<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self { name_part: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct NamePartSingleField2 {
                pub is_extension: bool,
            }

            impl ::puroro::Message<super::NamePart> for NamePartSingleField2 {}

            impl super::_puroro_traits::NamePartTrait for NamePartSingleField2 {
                fn is_extension_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.is_extension))
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for NamePartSingleField2 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<::puroro::tags::Required, ::puroro::tags::Bool>::ser_field::<
                        (),
                        _,
                        _,
                    >(::std::iter::once(&self.is_extension), 2, out)?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<bool> for NamePartSingleField2 {
                fn from(value: bool) -> Self {
                    Self {
                        is_extension: value,
                    }
                }
            }
            pub struct NamePartBuilder<T>(T);

            impl<T> NamePartBuilder<T>
            where
                T: NamePartTrait,
            {
                pub fn append_name_part<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> NamePartBuilder<(T, NamePartSingleField1<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    NamePartBuilder((self.0, NamePartSingleField1 { name_part: value }))
                }

                pub fn append_is_extension(
                    self,
                    value: bool,
                ) -> NamePartBuilder<(T, NamePartSingleField2)> {
                    NamePartBuilder((
                        self.0,
                        NamePartSingleField2 {
                            is_extension: value,
                        },
                    ))
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
                fn name_part<'this>(&'this self) -> &'this str {
                    self.name_part_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_name_part<'this>(&'this self) -> bool {
                    self.name_part_opt().is_some()
                }
                fn name_part_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                fn is_extension<'this>(&'this self) -> bool {
                    self.is_extension_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_is_extension<'this>(&'this self) -> bool {
                    self.is_extension_opt().is_some()
                }
                fn is_extension_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                    ::std::option::Option::None
                }
            }

            macro_rules! name_part_delegate {
                ($ty:ty) => {
                    fn name_part_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                        (**self).name_part_opt()
                    }
                    fn is_extension_opt<'this>(&'this self) -> ::std::option::Option<bool> {
                        (**self).is_extension_opt()
                    }
                };
            }

            impl<T> NamePartTrait for &'_ T
            where
                T: NamePartTrait,
            {
                name_part_delegate!(T);
            }

            impl<T> NamePartTrait for &'_ mut T
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
                pub leading_comments: ::std::option::Option<::std::string::String>,
                pub trailing_comments: ::std::option::Option<::std::string::String>,
                pub leading_detached_comments: ::std::vec::Vec<::std::string::String>,
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
                fn leading_comments_opt<'this>(&'this self) -> Option<&'this str> {
                    self.leading_comments.as_ref().map(|v| v.as_ref())
                }
                fn trailing_comments_opt<'this>(&'this self) -> Option<&'this str> {
                    self.trailing_comments.as_ref().map(|v| v.as_ref())
                }
                type Field6RepeatedType<'this> = ::puroro::internal::impls::simple::BorrowedIter<
                    str,
                    ::std::slice::Iter<'this, ::std::string::String>,
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

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for Location {
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

            impl ::puroro::internal::SerializableMessageToIoWrite for Location {
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
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
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
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::merged::MergedRepeatedField<
                    <T as LocationTrait>::Field1RepeatedType<'this>,
                    <U as LocationTrait>::Field1RepeatedType<'this>,
                >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedField::new(
                        <T as LocationTrait>::path(&self.0),
                        <U as LocationTrait>::path(&self.1),
                    )
                }
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::merged::MergedRepeatedField<
                    <T as LocationTrait>::Field2RepeatedType<'this>,
                    <U as LocationTrait>::Field2RepeatedType<'this>,
                >;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedField::new(
                        <T as LocationTrait>::span(&self.0),
                        <U as LocationTrait>::span(&self.1),
                    )
                }
                fn leading_comments_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as LocationTrait>::leading_comments_opt(&self.1)
                        .or_else(|| <T as LocationTrait>::leading_comments_opt(&self.0))
                }
                fn trailing_comments_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as LocationTrait>::trailing_comments_opt(&self.1)
                        .or_else(|| <T as LocationTrait>::trailing_comments_opt(&self.0))
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::merged::MergedRepeatedField<
                    <T as LocationTrait>::Field6RepeatedType<'this>,
                    <U as LocationTrait>::Field6RepeatedType<'this>,
                >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedField::new(
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
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::either::EitherRepeatedField<
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
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::either::EitherRepeatedField<
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
                fn leading_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as LocationTrait>::leading_comments_opt(t),
                        |u| <U as LocationTrait>::leading_comments_opt(u),
                    )
                }
                fn trailing_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as LocationTrait>::trailing_comments_opt(t),
                        |u| <U as LocationTrait>::trailing_comments_opt(u),
                    )
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::either::EitherRepeatedField<
                    <T as LocationTrait>::Field6RepeatedType<'this>,
                    <U as LocationTrait>::Field6RepeatedType<'this>,
                >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::either::EitherRepeatedField::new(
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
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::std::iter::Flatten<
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

                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::std::iter::Flatten<
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
                fn leading_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.leading_comments_opt())
                }
                fn trailing_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.trailing_comments_opt())
                }

                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::std::iter::Flatten<
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

            pub struct LocationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                pub path: RepeatedType,
            }

            impl<RepeatedType> ::puroro::Message<super::Location> for LocationSingleField1<RepeatedType> where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>
            {
            }

            impl<RepeatedType> super::_puroro_traits::LocationTrait for LocationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.path))
                }
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
                for LocationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    &self.path,
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<RepeatedType> ::std::convert::From<RepeatedType> for LocationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                fn from(value: RepeatedType) -> Self {
                    Self { path: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct LocationSingleField2<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                pub span: RepeatedType,
            }

            impl<RepeatedType> ::puroro::Message<super::Location> for LocationSingleField2<RepeatedType> where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>
            {
            }

            impl<RepeatedType> super::_puroro_traits::LocationTrait for LocationSingleField2<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.span))
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
                for LocationSingleField2<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    &self.span,
                    2,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<RepeatedType> ::std::convert::From<RepeatedType> for LocationSingleField2<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                fn from(value: RepeatedType) -> Self {
                    Self { span: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct LocationSingleField3<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub leading_comments: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Location> for LocationSingleField3<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::LocationTrait for LocationSingleField3<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }

                fn leading_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.leading_comments.as_ref())
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
                for LocationSingleField3<ScalarType>
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
                    ::std::iter::once(&self.leading_comments),
                    3,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for LocationSingleField3<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        leading_comments: value,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct LocationSingleField4<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub trailing_comments: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Location> for LocationSingleField4<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::LocationTrait for LocationSingleField4<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }

                fn trailing_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.trailing_comments.as_ref())
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<&'this str>;
                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
                for LocationSingleField4<ScalarType>
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
                    ::std::iter::once(&self.trailing_comments),
                    4,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for LocationSingleField4<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self {
                        trailing_comments: value,
                    }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct LocationSingleField6<ScalarType, RepeatedType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
            {
                pub leading_detached_comments: RepeatedType,
            }

            impl<ScalarType, RepeatedType> ::puroro::Message<super::Location>
                for LocationSingleField6<ScalarType, RepeatedType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
            {
            }

            impl<ScalarType, RepeatedType> super::_puroro_traits::LocationTrait
                for LocationSingleField6<ScalarType, RepeatedType>
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
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field2RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
                type Field6RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::single_field::AsRefIter<
                    <&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter,
                    str,
                >;

                fn leading_detached_comments<'this>(
                    &'this self,
                ) -> Self::Field6RepeatedType<'this> {
                    ::puroro::internal::impls::single_field::AsRefIter::new(
                        ::std::iter::IntoIterator::into_iter(&self.leading_detached_comments),
                    )
                }
            }

            impl<ScalarType, RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
                for LocationSingleField6<ScalarType, RepeatedType>
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
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::String
                >::ser_field::
                <ScalarType, _, _>
                (
                    &self.leading_detached_comments,
                    6,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType, RepeatedType> ::std::convert::From<RepeatedType>
                for LocationSingleField6<ScalarType, RepeatedType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
            {
                fn from(value: RepeatedType) -> Self {
                    Self {
                        leading_detached_comments: value,
                    }
                }
            }
            pub struct LocationBuilder<T>(T);

            impl<T> LocationBuilder<T>
            where
                T: LocationTrait,
            {
                pub fn append_path<RepeatedType>(
                    self,
                    value: RepeatedType,
                ) -> LocationBuilder<(T, LocationSingleField1<RepeatedType>)>
                where
                    for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
                {
                    LocationBuilder((self.0, LocationSingleField1 { path: value }))
                }

                pub fn append_span<RepeatedType>(
                    self,
                    value: RepeatedType,
                ) -> LocationBuilder<(T, LocationSingleField2<RepeatedType>)>
                where
                    for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
                {
                    LocationBuilder((self.0, LocationSingleField2 { span: value }))
                }

                pub fn append_leading_comments<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> LocationBuilder<(T, LocationSingleField3<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    LocationBuilder((
                        self.0,
                        LocationSingleField3 {
                            leading_comments: value,
                        },
                    ))
                }

                pub fn append_trailing_comments<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> LocationBuilder<(T, LocationSingleField4<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    LocationBuilder((
                        self.0,
                        LocationSingleField4 {
                            trailing_comments: value,
                        },
                    ))
                }

                pub fn append_leading_detached_comments<ScalarType, RepeatedType>(
                    self,
                    value: RepeatedType,
                ) -> LocationBuilder<(T, LocationSingleField6<ScalarType, RepeatedType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                    for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a ScalarType>,
                {
                    LocationBuilder((
                        self.0,
                        LocationSingleField6 {
                            leading_detached_comments: value,
                        },
                    ))
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
                    + ::std::iter::IntoIterator<Item = i32>
                where
                    Self: 'this;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;

                type Field2RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = i32>
                where
                    Self: 'this;
                fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this>;
                fn leading_comments<'this>(&'this self) -> &'this str {
                    self.leading_comments_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_leading_comments<'this>(&'this self) -> bool {
                    self.leading_comments_opt().is_some()
                }
                fn leading_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                fn trailing_comments<'this>(&'this self) -> &'this str {
                    self.trailing_comments_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_trailing_comments<'this>(&'this self) -> bool {
                    self.trailing_comments_opt().is_some()
                }
                fn trailing_comments_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }

                type Field6RepeatedType<'this>: ::puroro::RepeatedField<'this>
                    + ::std::iter::IntoIterator<Item = &'this str>
                where
                    Self: 'this;
                fn leading_detached_comments<'this>(&'this self)
                -> Self::Field6RepeatedType<'this>;
            }

            macro_rules! location_delegate {
                ($ty:ty) => {
                    type Field1RepeatedType<'this>
                    where
                        Self: 'this,
                    = <$ty>::Field1RepeatedType<'this>;
                    fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                        (**self).path()
                    }

                    type Field2RepeatedType<'this>
                    where
                        Self: 'this,
                    = <$ty>::Field2RepeatedType<'this>;
                    fn span<'this>(&'this self) -> Self::Field2RepeatedType<'this> {
                        (**self).span()
                    }
                    fn leading_comments_opt<'this>(
                        &'this self,
                    ) -> ::std::option::Option<&'this str> {
                        (**self).leading_comments_opt()
                    }
                    fn trailing_comments_opt<'this>(
                        &'this self,
                    ) -> ::std::option::Option<&'this str> {
                        (**self).trailing_comments_opt()
                    }

                    type Field6RepeatedType<'this>
                    where
                        Self: 'this,
                    = <$ty>::Field6RepeatedType<'this>;
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

            impl<T> LocationTrait for &'_ mut T
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
                pub source_file: ::std::option::Option<::std::string::String>,
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
                fn source_file_opt<'this>(&'this self) -> Option<&'this str> {
                    self.source_file.as_ref().map(|v| v.as_ref())
                }
                fn begin_opt<'this>(&'this self) -> Option<i32> {
                    Clone::clone(&self.begin)
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
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

            impl ::puroro::internal::DeserializableMessageFromBytesIterator for Annotation {
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

            impl ::puroro::internal::SerializableMessageToIoWrite for Annotation {
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
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }
            }
            impl<T, U> AnnotationTrait for (T, U)
            where
                T: AnnotationTrait,
                U: AnnotationTrait,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::merged::MergedRepeatedField<
                    <T as AnnotationTrait>::Field1RepeatedType<'this>,
                    <U as AnnotationTrait>::Field1RepeatedType<'this>,
                >;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::merged::MergedRepeatedField::new(
                        <T as AnnotationTrait>::path(&self.0),
                        <U as AnnotationTrait>::path(&self.1),
                    )
                }
                fn source_file_opt<'this>(&'this self) -> Option<&'this str> {
                    <U as AnnotationTrait>::source_file_opt(&self.1)
                        .or_else(|| <T as AnnotationTrait>::source_file_opt(&self.0))
                }
                fn begin_opt<'this>(&'this self) -> Option<i32> {
                    <U as AnnotationTrait>::begin_opt(&self.1)
                        .or_else(|| <T as AnnotationTrait>::begin_opt(&self.0))
                }
                fn end_opt<'this>(&'this self) -> Option<i32> {
                    <U as AnnotationTrait>::end_opt(&self.1)
                        .or_else(|| <T as AnnotationTrait>::end_opt(&self.0))
                }
            }
            impl<T, U> AnnotationTrait for ::puroro::Either<T, U>
            where
                T: AnnotationTrait,
                U: AnnotationTrait,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::either::EitherRepeatedField<
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
                fn source_file_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().either(
                        |t| <T as AnnotationTrait>::source_file_opt(t),
                        |u| <U as AnnotationTrait>::source_file_opt(u),
                    )
                }
                fn begin_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as AnnotationTrait>::begin_opt(t),
                        |u| <U as AnnotationTrait>::begin_opt(u),
                    )
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().either(
                        |t| <T as AnnotationTrait>::end_opt(t),
                        |u| <U as AnnotationTrait>::end_opt(u),
                    )
                }
            }
            impl<T> AnnotationTrait for ::std::option::Option<T>
            where
                T: AnnotationTrait,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::std::iter::Flatten<
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
                fn source_file_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    self.as_ref().and_then(|msg| msg.source_file_opt())
                }
                fn begin_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.begin_opt())
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    self.as_ref().and_then(|msg| msg.end_opt())
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct AnnotationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                pub path: RepeatedType,
            }

            impl<RepeatedType> ::puroro::Message<super::Annotation> for AnnotationSingleField1<RepeatedType> where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>
            {
            }

            impl<RepeatedType> super::_puroro_traits::AnnotationTrait for AnnotationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::std::iter::Cloned<<&'this RepeatedType as ::std::iter::IntoIterator>::IntoIter>;

                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::std::iter::Iterator::cloned(::std::iter::IntoIterator::into_iter(&self.path))
                }
            }

            impl<RepeatedType> ::puroro::internal::SerializableMessageToIoWrite
                for AnnotationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Repeated, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    &self.path,
                    1,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<RepeatedType> ::std::convert::From<RepeatedType> for AnnotationSingleField1<RepeatedType>
            where
                for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
            {
                fn from(value: RepeatedType) -> Self {
                    Self { path: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct AnnotationSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                pub source_file: ScalarType,
            }

            impl<ScalarType> ::puroro::Message<super::Annotation> for AnnotationSingleField2<ScalarType> where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug
            {
            }

            impl<ScalarType> super::_puroro_traits::AnnotationTrait for AnnotationSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }

                fn source_file_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::Some(self.source_file.as_ref())
                }
            }

            impl<ScalarType> ::puroro::internal::SerializableMessageToIoWrite
                for AnnotationSingleField2<ScalarType>
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
                    ::std::iter::once(&self.source_file),
                    2,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl<ScalarType> ::std::convert::From<ScalarType> for AnnotationSingleField2<ScalarType>
            where
                ScalarType: ::std::convert::AsRef<str>
                    + ::std::clone::Clone
                    + ::std::cmp::PartialEq
                    + ::std::fmt::Debug,
            {
                fn from(value: ScalarType) -> Self {
                    Self { source_file: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct AnnotationSingleField3 {
                pub begin: i32,
            }

            impl ::puroro::Message<super::Annotation> for AnnotationSingleField3 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSingleField3 {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }

                fn begin_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.begin))
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for AnnotationSingleField3 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.begin),
                    3,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for AnnotationSingleField3 {
                fn from(value: i32) -> Self {
                    Self { begin: value }
                }
            }

            #[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]

            pub struct AnnotationSingleField4 {
                pub end: i32,
            }

            impl ::puroro::Message<super::Annotation> for AnnotationSingleField4 {}

            impl super::_puroro_traits::AnnotationTrait for AnnotationSingleField4 {
                type Field1RepeatedType<'this>
                where
                    Self: 'this,
                = ::puroro::internal::impls::empty::EmptyRepeatedField<i32>;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                    ::puroro::internal::impls::empty::EmptyRepeatedField::new()
                }

                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::Some(::std::clone::Clone::clone(&self.end))
                }
            }

            impl ::puroro::internal::SerializableMessageToIoWrite for AnnotationSingleField4 {
                fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                where
                    W: ::std::io::Write,
                {
                    use ::puroro::internal::impls::single_field::se::SerFieldToIoWrite;
                    SerFieldToIoWrite::<
                    ::puroro::tags::Optional, ::puroro::tags::Int32
                >::ser_field::
                <(), _, _>
                (
                    ::std::iter::once(&self.end),
                    4,
                    out
                )?;
                    ::std::result::Result::Ok(())
                }
            }

            impl ::std::convert::From<i32> for AnnotationSingleField4 {
                fn from(value: i32) -> Self {
                    Self { end: value }
                }
            }
            pub struct AnnotationBuilder<T>(T);

            impl<T> AnnotationBuilder<T>
            where
                T: AnnotationTrait,
            {
                pub fn append_path<RepeatedType>(
                    self,
                    value: RepeatedType,
                ) -> AnnotationBuilder<(T, AnnotationSingleField1<RepeatedType>)>
                where
                    for<'a> &'a RepeatedType: ::std::iter::IntoIterator<Item = &'a i32>,
                {
                    AnnotationBuilder((self.0, AnnotationSingleField1 { path: value }))
                }

                pub fn append_source_file<ScalarType>(
                    self,
                    value: ScalarType,
                ) -> AnnotationBuilder<(T, AnnotationSingleField2<ScalarType>)>
                where
                    ScalarType: ::std::convert::AsRef<str>
                        + ::std::clone::Clone
                        + ::std::cmp::PartialEq
                        + ::std::fmt::Debug,
                {
                    AnnotationBuilder((self.0, AnnotationSingleField2 { source_file: value }))
                }

                pub fn append_begin(
                    self,
                    value: i32,
                ) -> AnnotationBuilder<(T, AnnotationSingleField3)> {
                    AnnotationBuilder((self.0, AnnotationSingleField3 { begin: value }))
                }

                pub fn append_end(
                    self,
                    value: i32,
                ) -> AnnotationBuilder<(T, AnnotationSingleField4)> {
                    AnnotationBuilder((self.0, AnnotationSingleField4 { end: value }))
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
                    + ::std::iter::IntoIterator<Item = i32>
                where
                    Self: 'this;
                fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this>;
                fn source_file<'this>(&'this self) -> &'this str {
                    self.source_file_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_source_file<'this>(&'this self) -> bool {
                    self.source_file_opt().is_some()
                }
                fn source_file_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                    ::std::option::Option::None
                }
                fn begin<'this>(&'this self) -> i32 {
                    self.begin_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_begin<'this>(&'this self) -> bool {
                    self.begin_opt().is_some()
                }
                fn begin_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
                fn end<'this>(&'this self) -> i32 {
                    self.end_opt()
                        .unwrap_or_else(::std::default::Default::default)
                }
                fn has_end<'this>(&'this self) -> bool {
                    self.end_opt().is_some()
                }
                fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                    ::std::option::Option::None
                }
            }

            macro_rules! annotation_delegate {
                ($ty:ty) => {
                    type Field1RepeatedType<'this>
                    where
                        Self: 'this,
                    = <$ty>::Field1RepeatedType<'this>;
                    fn path<'this>(&'this self) -> Self::Field1RepeatedType<'this> {
                        (**self).path()
                    }
                    fn source_file_opt<'this>(&'this self) -> ::std::option::Option<&'this str> {
                        (**self).source_file_opt()
                    }
                    fn begin_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).begin_opt()
                    }
                    fn end_opt<'this>(&'this self) -> ::std::option::Option<i32> {
                        (**self).end_opt()
                    }
                };
            }

            impl<T> AnnotationTrait for &'_ T
            where
                T: AnnotationTrait,
            {
                annotation_delegate!(T);
            }

            impl<T> AnnotationTrait for &'_ mut T
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
