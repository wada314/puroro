// A generated source code by puroro library
pub mod compiler;
// package google.protobuf

pub mod puroro_root {
    pub use super::super::super::*;
}

pub mod puroro_structs {

    pub mod public {

        pub mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        use ::puroro_internal::de::DeserEnumFromBytesIter as _;
        use ::puroro_internal::de::DeserMsgFromBytesIter as _;
        use ::puroro_internal::se::SerEnumToIoWrite as _;
        use ::puroro_internal::se::SerMsgToIoWrite as _;

        #[allow(unused)]
        pub struct FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// repeated FileDescriptorProto file = 1;
            pub file: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for FileDescriptorSet<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    file: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.file, data, &self.puroro_internal
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.file, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FileDescriptorSet<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FileDescriptorSet<NewImplTag>,
                    super::private::FileDescriptorSet<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FileDescriptorSet<NewImplTag>,
                super::private::FileDescriptorSet<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    file: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.file, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("FileDescriptorSet")
                    .field("file", &self.file)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string package = 2;
            pub package: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// repeated string dependency = 3;
            pub dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >>::Type,
            /// repeated int32 public_dependency = 10;
            pub public_dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >>::Type,
            /// repeated int32 weak_dependency = 11;
            pub weak_dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >>::Type,
            /// repeated DescriptorProto message_type = 4;
            pub message_type:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::DescriptorProto<ImplTag>>,
            /// repeated EnumDescriptorProto enum_type = 5;
            pub enum_type:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>>,
            /// repeated ServiceDescriptorProto service = 6;
            pub service:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::ServiceDescriptorProto<ImplTag>>,
            /// repeated FieldDescriptorProto extension = 7;
            pub extension:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>>,
            /// optional FileOptions options = 8;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::FileOptions<ImplTag>>,
            /// optional SourceCodeInfo source_code_info = 9;
            pub source_code_info:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                >>::MsgType<self::puroro_root::google::protobuf::SourceCodeInfo<ImplTag>>,
            /// optional string syntax = 12;
            pub syntax: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for FileDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    public_dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::default(&internal_data),
                    weak_dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::default(&internal_data),
                    message_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    enum_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    service: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    extension: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    source_code_info: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    syntax: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.package, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.dependency, data, &self.puroro_internal),
                    10 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.public_dependency, data, &self.puroro_internal),
                    11 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.weak_dependency, data, &self.puroro_internal),
                    4 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::DescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.message_type, data, &self.puroro_internal),
                    5 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.enum_type, data, &self.puroro_internal),
                    6 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::ServiceDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.service, data, &self.puroro_internal),
                    7 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.extension, data, &self.puroro_internal),
                    8 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::FileOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
                    9 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::SourceCodeInfo<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.source_code_info, data, &self.puroro_internal),
                    12 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.syntax, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.package, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.dependency, 3, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >>::ser_to_io_write(
                    &self.public_dependency, 10, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                    ::puroro::tags::Int32,
                >>::ser_to_io_write(
                    &self.weak_dependency, 11, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::DescriptorProto<ImplTag>
                >::ser_to_io_write(&self.message_type, 4, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.enum_type, 5, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::ServiceDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.service, 6, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.extension, 7, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::FileOptions<ImplTag>
                >::ser_to_io_write(&self.options, 8, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::SourceCodeInfo<ImplTag>
                >::ser_to_io_write(&self.source_code_info, 9, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.syntax, 12, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FileDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FileDescriptorProto<NewImplTag>,
                    super::private::FileDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FileDescriptorProto<NewImplTag>,
                super::private::FileDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.package, &internal_data),
                    dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::clone(&self.dependency, &internal_data),
                    public_dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::clone(
                        &self.public_dependency, &internal_data
                    ),
                    weak_dependency: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::clone(
                        &self.weak_dependency, &internal_data
                    ),
                    message_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.message_type, &internal_data),
                    enum_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.enum_type, &internal_data),
                    service: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.service, &internal_data),
                    extension: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.extension, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    source_code_info: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(
                        &self.source_code_info, &internal_data
                    ),
                    syntax: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.syntax, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::DescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::ServiceDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::FileOptions<ImplTag>>: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::SourceCodeInfo<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("FileDescriptorProto")
                    .field("name", &self.name)
                    .field("package", &self.package)
                    .field("dependency", &self.dependency)
                    .field("public_dependency", &self.public_dependency)
                    .field("weak_dependency", &self.weak_dependency)
                    .field("message_type", &self.message_type)
                    .field("enum_type", &self.enum_type)
                    .field("service", &self.service)
                    .field("extension", &self.extension)
                    .field("options", &self.options)
                    .field("source_code_info", &self.source_code_info)
                    .field("syntax", &self.syntax)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct DescriptorProto<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            /// repeated FieldDescriptorProto field = 2;
            pub field: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
            >,
            /// repeated FieldDescriptorProto extension = 6;
            pub extension: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
            >,
            /// repeated DescriptorProto nested_type = 3;
            pub nested_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::DescriptorProto<ImplTag>
            >,
            /// repeated EnumDescriptorProto enum_type = 4;
            pub enum_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>
            >,
            /// repeated ExtensionRange extension_range = 5;
            pub extension_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<ImplTag>
            >,
            /// repeated OneofDescriptorProto oneof_decl = 8;
            pub oneof_decl: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::OneofDescriptorProto<ImplTag>
            >,
            /// optional MessageOptions options = 7;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::MessageOptions<ImplTag>
            >,
            /// repeated ReservedRange reserved_range = 9;
            pub reserved_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<ImplTag>
            >,
            /// repeated string reserved_name = 10;
            pub reserved_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for DescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    field: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    extension: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    nested_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    enum_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    extension_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    oneof_decl: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    reserved_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    reserved_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.field, data, &self.puroro_internal),
                    6 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.extension, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::DescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.nested_type, data, &self.puroro_internal),
                    4 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.enum_type, data, &self.puroro_internal),
                    5 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.extension_range, data, &self.puroro_internal),
                    8 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::OneofDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.oneof_decl, data, &self.puroro_internal),
                    7 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::MessageOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
                    9 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.reserved_range, data, &self.puroro_internal),
                    10 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.reserved_name, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.field, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.extension, 6, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::DescriptorProto<ImplTag>
                >::ser_to_io_write(&self.nested_type, 3, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.enum_type, 4, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<ImplTag>
                >::ser_to_io_write(&self.extension_range, 5, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::OneofDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.oneof_decl, 8, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::MessageOptions<ImplTag>
                >::ser_to_io_write(&self.options, 7, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<ImplTag>
                >::ser_to_io_write(&self.reserved_range, 9, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.reserved_name, 10, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for DescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::DescriptorProto<NewImplTag>,
                    super::private::DescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::DescriptorProto<NewImplTag>,
                super::private::DescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    field: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.field, &internal_data),
                    extension: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.extension, &internal_data),
                    nested_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.nested_type, &internal_data),
                    enum_type: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.enum_type, &internal_data),
                    extension_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.extension_range, &internal_data
                    ),
                    oneof_decl: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.oneof_decl, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    reserved_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.reserved_range, &internal_data
                    ),
                    reserved_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.reserved_name, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::DescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::OneofDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::MessageOptions<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("DescriptorProto")
                    .field("name", &self.name)
                    .field("field", &self.field)
                    .field("extension", &self.extension)
                    .field("nested_type", &self.nested_type)
                    .field("enum_type", &self.enum_type)
                    .field("extension_range", &self.extension_range)
                    .field("oneof_decl", &self.oneof_decl)
                    .field("options", &self.options)
                    .field("reserved_range", &self.reserved_range)
                    .field("reserved_name", &self.reserved_name)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for ExtensionRangeOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.uninterpreted_option,
                        data,
                        &self.puroro_internal,
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ExtensionRangeOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::ExtensionRangeOptions<NewImplTag>,
                    super::private::ExtensionRangeOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::ExtensionRangeOptions<NewImplTag>,
                super::private::ExtensionRangeOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("ExtensionRangeOptions")
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional int32 number = 3;
            pub number: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type,
            /// optional Label label = 4;
            pub label: <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label,
            >,
            /// optional Type type = 5;
            pub type_: <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type,
            >,
            /// optional string type_name = 6;
            pub type_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string extendee = 2;
            pub extendee: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string default_value = 7;
            pub default_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional int32 oneof_index = 9;
            pub oneof_index: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type,
            /// optional string json_name = 10;
            pub json_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional FieldOptions options = 8;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::FieldOptions<ImplTag>>,
            /// optional bool proto3_optional = 17;
            pub proto3_optional: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for FieldDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    number: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::default(&internal_data),
                    label: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    type_: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    type_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    extendee: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    default_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    oneof_index: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::default(&internal_data),
                    json_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    proto3_optional: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.number, data, &self.puroro_internal),
                    4 => <ImplTag as ::puroro_internal::de::DeserEnumFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserEnum::<
                        self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label
                    >::deser_from_scoped_bytes_iter(&mut self.label, data, &self.puroro_internal),
                    5 => <ImplTag as ::puroro_internal::de::DeserEnumFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserEnum::<
                        self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type
                    >::deser_from_scoped_bytes_iter(&mut self.type_, data, &self.puroro_internal),
                    6 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.type_name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.extendee, data, &self.puroro_internal),
                    7 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.default_value, data, &self.puroro_internal),
                    9 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.oneof_index, data, &self.puroro_internal),
                    10 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.json_name, data, &self.puroro_internal),
                    8 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::FieldOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
                    17 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.proto3_optional, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >>::ser_to_io_write(&self.number, 3, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerEnumToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerEnum::<
                    self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label
                >::ser_to_io_write(&self.label, 4, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerEnumToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerEnum::<
                    self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type
                >::ser_to_io_write(&self.type_, 5, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.type_name, 6, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.extendee, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.default_value, 7, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >>::ser_to_io_write(
                    &self.oneof_index, 9, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.json_name, 10, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::FieldOptions<ImplTag>
                >::ser_to_io_write(&self.options, 8, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.proto3_optional, 17, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FieldDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FieldDescriptorProto<NewImplTag>,
                    super::private::FieldDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FieldDescriptorProto<NewImplTag>,
                super::private::FieldDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    number: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::clone(&self.number, &internal_data),
                    label: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.label, &internal_data),
                    type_: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.type_, &internal_data),
                    type_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.type_name, &internal_data),
                    extendee: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.extendee, &internal_data),
                    default_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.default_value, &internal_data
                    ),
                    oneof_index: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::clone(&self.oneof_index, &internal_data),
                    json_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.json_name, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    proto3_optional: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.proto3_optional, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::FieldOptions<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("FieldDescriptorProto")
                    .field("name", &self.name)
                    .field("number", &self.number)
                    .field("label", &self.label)
                    .field("type_", &self.type_)
                    .field("type_name", &self.type_name)
                    .field("extendee", &self.extendee)
                    .field("default_value", &self.default_value)
                    .field("oneof_index", &self.oneof_index)
                    .field("json_name", &self.json_name)
                    .field("options", &self.options)
                    .field("proto3_optional", &self.proto3_optional)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional OneofOptions options = 2;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::OneofOptions<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for OneofDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::OneofOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::OneofOptions<ImplTag>
                >::ser_to_io_write(&self.options, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for OneofDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::OneofDescriptorProto<NewImplTag>,
                    super::private::OneofDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::OneofDescriptorProto<NewImplTag>,
                super::private::OneofDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::OneofOptions<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("OneofDescriptorProto")
                    .field("name", &self.name)
                    .field("options", &self.options)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumDescriptorProto<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            /// repeated EnumValueDescriptorProto value = 2;
            pub value: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumValueDescriptorProto<ImplTag>
            >,
            /// optional EnumOptions options = 3;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumOptions<ImplTag>
            >,
            /// repeated EnumReservedRange reserved_range = 4;
            pub reserved_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<ImplTag>
            >,
            /// repeated string reserved_name = 5;
            pub reserved_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for EnumDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    value: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    reserved_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    reserved_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::EnumValueDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.value, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::EnumOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
                    4 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.reserved_range, data, &self.puroro_internal),
                    5 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.reserved_name, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::EnumValueDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.value, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::EnumOptions<ImplTag>
                >::ser_to_io_write(&self.options, 3, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<ImplTag>
                >::ser_to_io_write(&self.reserved_range, 4, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.reserved_name, 5, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumDescriptorProto<NewImplTag>,
                    super::private::EnumDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumDescriptorProto<NewImplTag>,
                super::private::EnumDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    value: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.value, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    reserved_range: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.reserved_range, &internal_data
                    ),
                    reserved_name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.reserved_name, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumValueDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumOptions<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("EnumDescriptorProto")
                    .field("name", &self.name)
                    .field("value", &self.value)
                    .field("options", &self.options)
                    .field("reserved_range", &self.reserved_range)
                    .field("reserved_name", &self.reserved_name)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional int32 number = 2;
            pub number: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type,
            /// optional EnumValueOptions options = 3;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::EnumValueOptions<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for EnumValueDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    number: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.number, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::EnumValueOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int32,
                >>::ser_to_io_write(&self.number, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::EnumValueOptions<ImplTag>
                >::ser_to_io_write(&self.options, 3, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumValueDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumValueDescriptorProto<NewImplTag>,
                    super::private::EnumValueDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumValueDescriptorProto<NewImplTag>,
                super::private::EnumValueDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    number: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::clone(&self.number, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::EnumValueOptions<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("EnumValueDescriptorProto")
                    .field("name", &self.name)
                    .field("number", &self.number)
                    .field("options", &self.options)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// repeated MethodDescriptorProto method = 2;
            pub method:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::MethodDescriptorProto<ImplTag>>,
            /// optional ServiceOptions options = 3;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::ServiceOptions<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for ServiceDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    method: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::MethodDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.method, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::ServiceOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::MethodDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.method, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::ServiceOptions<ImplTag>
                >::ser_to_io_write(&self.options, 3, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ServiceDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::ServiceDescriptorProto<NewImplTag>,
                    super::private::ServiceDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::ServiceDescriptorProto<NewImplTag>,
                super::private::ServiceDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    method: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.method, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::MethodDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::ServiceOptions<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("ServiceDescriptorProto")
                    .field("name", &self.name)
                    .field("method", &self.method)
                    .field("options", &self.options)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string input_type = 2;
            pub input_type: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string output_type = 3;
            pub output_type: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional MethodOptions options = 4;
            pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::MethodOptions<ImplTag>>,
            /// optional bool client_streaming = 5;
            pub client_streaming: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool server_streaming = 6;
            pub server_streaming: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for MethodDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    input_type: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    output_type: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    client_streaming: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    server_streaming: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.input_type, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.output_type, data, &self.puroro_internal),
                    4 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::MethodOptions<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.options, data, &self.puroro_internal),
                    5 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.client_streaming, data, &self.puroro_internal),
                    6 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.server_streaming, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.input_type, 2, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.output_type, 3, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::MethodOptions<ImplTag>
                >::ser_to_io_write(&self.options, 4, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.client_streaming, 5, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.server_streaming, 6, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for MethodDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::MethodDescriptorProto<NewImplTag>,
                    super::private::MethodDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::MethodDescriptorProto<NewImplTag>,
                super::private::MethodDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.name, &internal_data),
                    input_type: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.input_type, &internal_data),
                    output_type: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.output_type, &internal_data),
                    options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.options, &internal_data),
                    client_streaming: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.client_streaming, &internal_data
                    ),
                    server_streaming: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.server_streaming, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::MethodOptions<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("MethodDescriptorProto")
                    .field("name", &self.name)
                    .field("input_type", &self.input_type)
                    .field("output_type", &self.output_type)
                    .field("options", &self.options)
                    .field("client_streaming", &self.client_streaming)
                    .field("server_streaming", &self.server_streaming)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string java_package = 1;
            pub java_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string java_outer_classname = 8;
            pub java_outer_classname: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional bool java_multiple_files = 10;
            pub java_multiple_files: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool java_generate_equals_and_hash = 20;
            pub java_generate_equals_and_hash: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool java_string_check_utf8 = 27;
            pub java_string_check_utf8: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional OptimizeMode optimize_for = 9;
            pub optimize_for: <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
            >,
            /// optional string go_package = 11;
            pub go_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional bool cc_generic_services = 16;
            pub cc_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool java_generic_services = 17;
            pub java_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool py_generic_services = 18;
            pub py_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool php_generic_services = 42;
            pub php_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool deprecated = 23;
            pub deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool cc_enable_arenas = 31;
            pub cc_enable_arenas: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional string objc_class_prefix = 36;
            pub objc_class_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string csharp_namespace = 37;
            pub csharp_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string swift_prefix = 39;
            pub swift_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string php_class_prefix = 40;
            pub php_class_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string php_namespace = 41;
            pub php_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string php_metadata_namespace = 44;
            pub php_metadata_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional string ruby_package = 45;
            pub ruby_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for FileOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    java_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    java_outer_classname: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    java_multiple_files: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    java_generate_equals_and_hash: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    java_string_check_utf8: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    optimize_for: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    go_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    cc_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    java_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    py_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    php_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    cc_enable_arenas: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    objc_class_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    csharp_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    swift_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    php_class_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    php_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    php_metadata_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    ruby_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.java_package, data, &self.puroro_internal),
                    8 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.java_outer_classname, data, &self.puroro_internal),
                    10 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.java_multiple_files, data, &self.puroro_internal),
                    20 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.java_generate_equals_and_hash, data, &self.puroro_internal),
                    27 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.java_string_check_utf8, data, &self.puroro_internal),
                    9 => <ImplTag as ::puroro_internal::de::DeserEnumFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserEnum::<
                        self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode
                    >::deser_from_scoped_bytes_iter(&mut self.optimize_for, data, &self.puroro_internal),
                    11 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.go_package, data, &self.puroro_internal),
                    16 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.cc_generic_services, data, &self.puroro_internal),
                    17 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.java_generic_services, data, &self.puroro_internal),
                    18 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.py_generic_services, data, &self.puroro_internal),
                    42 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.php_generic_services, data, &self.puroro_internal),
                    23 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.deprecated, data, &self.puroro_internal),
                    31 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.cc_enable_arenas, data, &self.puroro_internal),
                    36 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.objc_class_prefix, data, &self.puroro_internal),
                    37 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.csharp_namespace, data, &self.puroro_internal),
                    39 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.swift_prefix, data, &self.puroro_internal),
                    40 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.php_class_prefix, data, &self.puroro_internal),
                    41 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.php_namespace, data, &self.puroro_internal),
                    44 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.php_metadata_namespace, data, &self.puroro_internal),
                    45 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.ruby_package, data, &self.puroro_internal),
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.uninterpreted_option, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.java_package, 1, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.java_outer_classname, 8, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.java_multiple_files, 10, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.java_generate_equals_and_hash,
                    20,
                    out,
                    &self.puroro_internal,
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.java_string_check_utf8, 27, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerEnumToIoWriteProxy<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                >>::SerEnum::<
                    self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
                >::ser_to_io_write(
                    &self.optimize_for, 9, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.go_package, 11, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.cc_generic_services, 16, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.java_generic_services, 17, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.py_generic_services, 18, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.php_generic_services, 42, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.deprecated, 23, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.cc_enable_arenas, 31, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.objc_class_prefix, 36, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.csharp_namespace, 37, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.swift_prefix, 39, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.php_class_prefix, 40, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.php_namespace, 41, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.php_metadata_namespace, 44, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.ruby_package, 45, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FileOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FileOptions<NewImplTag>,
                    super::private::FileOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FileOptions<NewImplTag>,
                super::private::FileOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    java_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.java_package, &internal_data),
                    java_outer_classname: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.java_outer_classname, &internal_data
                    ),
                    java_multiple_files: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.java_multiple_files, &internal_data
                    ),
                    java_generate_equals_and_hash: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.java_generate_equals_and_hash,
                        &internal_data,
                    ),
                    java_string_check_utf8: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.java_string_check_utf8, &internal_data
                    ),
                    optimize_for: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.optimize_for, &internal_data),
                    go_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.go_package, &internal_data),
                    cc_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.cc_generic_services, &internal_data
                    ),
                    java_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.java_generic_services, &internal_data
                    ),
                    py_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.py_generic_services, &internal_data
                    ),
                    php_generic_services: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.php_generic_services, &internal_data
                    ),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.deprecated, &internal_data),
                    cc_enable_arenas: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.cc_enable_arenas, &internal_data
                    ),
                    objc_class_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.objc_class_prefix, &internal_data
                    ),
                    csharp_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.csharp_namespace, &internal_data
                    ),
                    swift_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.swift_prefix, &internal_data),
                    php_class_prefix: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.php_class_prefix, &internal_data
                    ),
                    php_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.php_namespace, &internal_data
                    ),
                    php_metadata_namespace: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.php_metadata_namespace, &internal_data
                    ),
                    ruby_package: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(&self.ruby_package, &internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("FileOptions")
                    .field("java_package", &self.java_package)
                    .field("java_outer_classname", &self.java_outer_classname)
                    .field("java_multiple_files", &self.java_multiple_files)
                    .field(
                        "java_generate_equals_and_hash",
                        &self.java_generate_equals_and_hash,
                    )
                    .field("java_string_check_utf8", &self.java_string_check_utf8)
                    .field("optimize_for", &self.optimize_for)
                    .field("go_package", &self.go_package)
                    .field("cc_generic_services", &self.cc_generic_services)
                    .field("java_generic_services", &self.java_generic_services)
                    .field("py_generic_services", &self.py_generic_services)
                    .field("php_generic_services", &self.php_generic_services)
                    .field("deprecated", &self.deprecated)
                    .field("cc_enable_arenas", &self.cc_enable_arenas)
                    .field("objc_class_prefix", &self.objc_class_prefix)
                    .field("csharp_namespace", &self.csharp_namespace)
                    .field("swift_prefix", &self.swift_prefix)
                    .field("php_class_prefix", &self.php_class_prefix)
                    .field("php_namespace", &self.php_namespace)
                    .field("php_metadata_namespace", &self.php_metadata_namespace)
                    .field("ruby_package", &self.ruby_package)
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional bool message_set_wire_format = 1;
            pub message_set_wire_format: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool no_standard_descriptor_accessor = 2;
            pub no_standard_descriptor_accessor: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool deprecated = 3;
            pub deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool map_entry = 7;
            pub map_entry: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for MessageOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    message_set_wire_format: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    no_standard_descriptor_accessor: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    map_entry: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.message_set_wire_format,
                        data,
                        &self.puroro_internal,
                    ),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.no_standard_descriptor_accessor,
                        data,
                        &self.puroro_internal,
                    ),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.deprecated,
                        data,
                        &self.puroro_internal,
                    ),
                    7 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.map_entry,
                        data,
                        &self.puroro_internal,
                    ),
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.uninterpreted_option,
                        data,
                        &self.puroro_internal,
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.message_set_wire_format, 1, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.no_standard_descriptor_accessor,
                    2,
                    out,
                    &self.puroro_internal,
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.deprecated, 3, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(&self.map_entry, 7, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for MessageOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::MessageOptions<NewImplTag>,
                    super::private::MessageOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::MessageOptions<NewImplTag>,
                super::private::MessageOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    message_set_wire_format: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.message_set_wire_format, &internal_data
                    ),
                    no_standard_descriptor_accessor: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(
                        &self.no_standard_descriptor_accessor,
                        &internal_data,
                    ),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.deprecated, &internal_data),
                    map_entry: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.map_entry, &internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("MessageOptions")
                    .field("message_set_wire_format", &self.message_set_wire_format)
                    .field(
                        "no_standard_descriptor_accessor",
                        &self.no_standard_descriptor_accessor,
                    )
                    .field("deprecated", &self.deprecated)
                    .field("map_entry", &self.map_entry)
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional CType ctype = 1;
            pub ctype: <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
            >,
            /// optional bool packed = 2;
            pub packed: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional JSType jstype = 6;
            pub jstype: <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
            >,
            /// optional bool lazy = 5;
            pub lazy: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool deprecated = 3;
            pub deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool weak = 10;
            pub weak: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for FieldOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    ctype: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    packed: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    jstype: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    lazy: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    weak: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserEnumFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::DeserEnum::<
                        self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.ctype, data, &self.puroro_internal
                    ),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.packed, data, &self.puroro_internal
                    ),
                    6 => <ImplTag as ::puroro_internal::de::DeserEnumFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::DeserEnum::<
                        self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.jstype, data, &self.puroro_internal
                    ),
                    5 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.lazy, data, &self.puroro_internal
                    ),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.deprecated,
                        data,
                        &self.puroro_internal,
                    ),
                    10 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.weak, data, &self.puroro_internal
                    ),
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.uninterpreted_option,
                        data,
                        &self.puroro_internal,
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerEnumToIoWriteProxy<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                >>::SerEnum::<
                    self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype,
                >::ser_to_io_write(&self.ctype, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(&self.packed, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerEnumToIoWriteProxy<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                >>::SerEnum::<
                    self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype,
                >::ser_to_io_write(&self.jstype, 6, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(&self.lazy, 5, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.deprecated, 3, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(&self.weak, 10, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FieldOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FieldOptions<NewImplTag>,
                    super::private::FieldOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FieldOptions<NewImplTag>,
                super::private::FieldOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    ctype: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.ctype, &internal_data),
                    packed: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.packed, &internal_data),
                    jstype: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(&self.jstype, &internal_data),
                    lazy: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.lazy, &internal_data),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.deprecated, &internal_data),
                    weak: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.weak, &internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("FieldOptions")
                    .field("ctype", &self.ctype)
                    .field("packed", &self.packed)
                    .field("jstype", &self.jstype)
                    .field("lazy", &self.lazy)
                    .field("deprecated", &self.deprecated)
                    .field("weak", &self.weak)
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for OneofOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.uninterpreted_option,
                        data,
                        &self.puroro_internal,
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for OneofOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::OneofOptions<NewImplTag>,
                    super::private::OneofOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::OneofOptions<NewImplTag>,
                super::private::OneofOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("OneofOptions")
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional bool allow_alias = 2;
            pub allow_alias: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// optional bool deprecated = 3;
            pub deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for EnumOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    allow_alias: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.allow_alias,
                        data,
                        &self.puroro_internal,
                    ),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.deprecated,
                        data,
                        &self.puroro_internal,
                    ),
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.uninterpreted_option,
                        data,
                        &self.puroro_internal,
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.allow_alias, 2, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.deprecated, 3, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumOptions<NewImplTag>,
                    super::private::EnumOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumOptions<NewImplTag>,
                super::private::EnumOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    allow_alias: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.allow_alias, &internal_data),
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.deprecated, &internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("EnumOptions")
                    .field("allow_alias", &self.allow_alias)
                    .field("deprecated", &self.deprecated)
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional bool deprecated = 1;
            pub deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for EnumValueOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.deprecated,
                        data,
                        &self.puroro_internal,
                    ),
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.uninterpreted_option,
                        data,
                        &self.puroro_internal,
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.deprecated, 1, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumValueOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumValueOptions<NewImplTag>,
                    super::private::EnumValueOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumValueOptions<NewImplTag>,
                super::private::EnumValueOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.deprecated, &internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("EnumValueOptions")
                    .field("deprecated", &self.deprecated)
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional bool deprecated = 33;
            pub deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type,
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option:
                <ImplTag as ::puroro_internal::MsgTypeGen<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for ServiceOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    33 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::deser_from_scoped_bytes_iter(
                        &mut self.deprecated,
                        data,
                        &self.puroro_internal,
                    ),
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>,
                    >::deser_from_scoped_bytes_iter(
                        &mut self.uninterpreted_option,
                        data,
                        &self.puroro_internal,
                    ),

                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.deprecated, 33, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ServiceOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::ServiceOptions<NewImplTag>,
                    super::private::ServiceOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::ServiceOptions<NewImplTag>,
                super::private::ServiceOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.deprecated, &internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("ServiceOptions")
                    .field("deprecated", &self.deprecated)
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct MethodOptions<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional bool deprecated = 33;
            pub deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
            >>::Type,
            /// optional IdempotencyLevel idempotency_level = 34;
            pub idempotency_level: <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::EnumType::<
                self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel
            >,
            /// repeated UninterpretedOption uninterpreted_option = 999;
            pub uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
            >,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for MethodOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::default(&internal_data),
                    idempotency_level: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::default(&internal_data),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    33 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
                    >>::deser_from_scoped_bytes_iter(&mut self.deprecated, data, &self.puroro_internal),
                    34 => <ImplTag as ::puroro_internal::de::DeserEnumFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserEnum::<
                        self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel
                    >::deser_from_scoped_bytes_iter(&mut self.idempotency_level, data, &self.puroro_internal),
                    999 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.uninterpreted_option, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bool,
                >>::ser_to_io_write(
                    &self.deprecated, 33, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerEnumToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerEnum::<
                    self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel
                >::ser_to_io_write(&self.idempotency_level, 34, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
                >::ser_to_io_write(&self.uninterpreted_option, 999, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for MethodOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::MethodOptions<NewImplTag>,
                    super::private::MethodOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::MethodOptions<NewImplTag>,
                super::private::MethodOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    deprecated: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bool,
                    >>::clone(&self.deprecated, &internal_data),
                    idempotency_level: <ImplTag as ::puroro_internal::EnumTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::clone(
                        &self.idempotency_level, &internal_data
                    ),
                    uninterpreted_option: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(
                        &self.uninterpreted_option, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::EnumType::<
                self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("MethodOptions")
                    .field("deprecated", &self.deprecated)
                    .field("idempotency_level", &self.idempotency_level)
                    .field("uninterpreted_option", &self.uninterpreted_option)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// repeated NamePart name = 2;
            pub name: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<
                    ImplTag,
                >,
            >,
            /// optional string identifier_value = 3;
            pub identifier_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            /// optional uint64 positive_int_value = 4;
            pub positive_int_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >>::Type,
            /// optional int64 negative_int_value = 5;
            pub negative_int_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >>::Type,
            /// optional double double_value = 6;
            pub double_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >>::Type,
            /// optional bytes string_value = 7;
            pub string_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >>::Type,
            /// optional string aggregate_value = 8;
            pub aggregate_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for UninterpretedOption<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    identifier_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    positive_int_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::UInt64,
                    >>::default(&internal_data),
                    negative_int_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int64,
                    >>::default(&internal_data),
                    double_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Double,
                    >>::default(&internal_data),
                    string_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bytes,
                    >>::default(&internal_data),
                    aggregate_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    2 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.identifier_value, data, &self.puroro_internal),
                    4 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64
                    >>::deser_from_scoped_bytes_iter(&mut self.positive_int_value, data, &self.puroro_internal),
                    5 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int64
                    >>::deser_from_scoped_bytes_iter(&mut self.negative_int_value, data, &self.puroro_internal),
                    6 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Double
                    >>::deser_from_scoped_bytes_iter(&mut self.double_value, data, &self.puroro_internal),
                    7 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bytes
                    >>::deser_from_scoped_bytes_iter(&mut self.string_value, data, &self.puroro_internal),
                    8 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.aggregate_value, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<ImplTag>
                >::ser_to_io_write(&self.name, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.identifier_value, 3, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::UInt64,
                >>::ser_to_io_write(
                    &self.positive_int_value, 4, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Int64,
                >>::ser_to_io_write(
                    &self.negative_int_value, 5, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Double,
                >>::ser_to_io_write(
                    &self.double_value, 6, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::Bytes,
                >>::ser_to_io_write(
                    &self.string_value, 7, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Optional,
                    ::puroro::tags::String,
                >>::ser_to_io_write(
                    &self.aggregate_value, 8, out, &self.puroro_internal
                )?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for UninterpretedOption<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::UninterpretedOption<NewImplTag>,
                    super::private::UninterpretedOption<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::UninterpretedOption<NewImplTag>,
                super::private::UninterpretedOption<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.name, &internal_data),
                    identifier_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.identifier_value, &internal_data
                    ),
                    positive_int_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::UInt64,
                    >>::clone(
                        &self.positive_int_value, &internal_data
                    ),
                    negative_int_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int64,
                    >>::clone(
                        &self.negative_int_value, &internal_data
                    ),
                    double_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Double,
                    >>::clone(&self.double_value, &internal_data),
                    string_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Bytes,
                    >>::clone(&self.string_value, &internal_data),
                    aggregate_value: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::clone(
                        &self.aggregate_value, &internal_data
                    ),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<
                    ImplTag,
                >,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("UninterpretedOption")
                    .field("name", &self.name)
                    .field("identifier_value", &self.identifier_value)
                    .field("positive_int_value", &self.positive_int_value)
                    .field("negative_int_value", &self.negative_int_value)
                    .field("double_value", &self.double_value)
                    .field("string_value", &self.string_value)
                    .field("aggregate_value", &self.aggregate_value)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// repeated Location location = 1;
            pub location: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                    ImplTag,
                >,
            >,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for SourceCodeInfo<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    location: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.location, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2,
                    ::puroro::tags::Repeated,
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                        ImplTag,
                    >,
                >::ser_to_io_write(&self.location, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for SourceCodeInfo<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::SourceCodeInfo<NewImplTag>,
                    super::private::SourceCodeInfo<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::SourceCodeInfo<NewImplTag>,
                super::private::SourceCodeInfo<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    location: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.location, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                    ImplTag,
                >,
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("SourceCodeInfo")
                    .field("location", &self.location)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// repeated Annotation annotation = 1;
            pub annotation: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<
                    ImplTag,
                >,
            >,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }

        impl<ImplTag> ::puroro::Message for GeneratedCodeInfo<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    annotation: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.annotation, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<ImplTag>
                >::ser_to_io_write(&self.annotation, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(
                    out,
                    &self.puroro_internal,
                )?;
                Ok(())
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for GeneratedCodeInfo<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::GeneratedCodeInfo<NewImplTag>,
                    super::private::GeneratedCodeInfo<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::GeneratedCodeInfo<NewImplTag>,
                super::private::GeneratedCodeInfo<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    annotation: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                    >>::clone(&self.annotation, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<
                    ImplTag,
                >,
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("GeneratedCodeInfo")
                    .field("annotation", &self.annotation)
                    .finish()
            }
        }

        impl<ImplTag> ::std::fmt::Display for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
    }

    pub mod private {

        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        #[allow(unused)]
        pub struct FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::FileDescriptorSet<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for FileDescriptorSet<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FileDescriptorSet<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FileDescriptorSet<NewImplTag>,
                    super::private::FileDescriptorSet<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FileDescriptorSet<NewImplTag>,
                super::private::FileDescriptorSet<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::FileDescriptorSet<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for FileDescriptorSet<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::FileDescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for FileDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FileDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FileDescriptorProto<NewImplTag>,
                    super::private::FileDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FileDescriptorProto<NewImplTag>,
                super::private::FileDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::DescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::ServiceDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::FileOptions<ImplTag>>: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::SourceCodeInfo<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::FileDescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for FileDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::DescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for DescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for DescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::DescriptorProto<NewImplTag>,
                    super::private::DescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::DescriptorProto<NewImplTag>,
                super::private::DescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FieldDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::DescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ExtensionRange<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::OneofDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::MessageOptions<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::descriptor_proto::ReservedRange<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::DescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for DescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::ExtensionRangeOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for ExtensionRangeOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ExtensionRangeOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::ExtensionRangeOptions<NewImplTag>,
                    super::private::ExtensionRangeOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::ExtensionRangeOptions<NewImplTag>,
                super::private::ExtensionRangeOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::ExtensionRangeOptions<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for ExtensionRangeOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::FieldDescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for FieldDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FieldDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FieldDescriptorProto<NewImplTag>,
                    super::private::FieldDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FieldDescriptorProto<NewImplTag>,
                super::private::FieldDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Label,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::field_descriptor_proto::Type,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::FieldOptions<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::FieldDescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for FieldDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::OneofDescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for OneofDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for OneofDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::OneofDescriptorProto<NewImplTag>,
                    super::private::OneofDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::OneofDescriptorProto<NewImplTag>,
                super::private::OneofDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::OneofOptions<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::OneofDescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for OneofDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::EnumDescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for EnumDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumDescriptorProto<NewImplTag>,
                    super::private::EnumDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumDescriptorProto<NewImplTag>,
                super::private::EnumDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumValueDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::EnumOptions<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::puroro_nested::enum_descriptor_proto::EnumReservedRange<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::EnumDescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::EnumValueDescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for EnumValueDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumValueDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumValueDescriptorProto<NewImplTag>,
                    super::private::EnumValueDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumValueDescriptorProto<NewImplTag>,
                super::private::EnumValueDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int32,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::EnumValueOptions<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::EnumValueDescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumValueDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::ServiceDescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for ServiceDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ServiceDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::ServiceDescriptorProto<NewImplTag>,
                    super::private::ServiceDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::ServiceDescriptorProto<NewImplTag>,
                super::private::ServiceDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::MethodDescriptorProto<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::ServiceOptions<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::ServiceDescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for ServiceDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::MethodDescriptorProto<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for MethodDescriptorProto<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for MethodDescriptorProto<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::MethodDescriptorProto<NewImplTag>,
                    super::private::MethodDescriptorProto<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::MethodDescriptorProto<NewImplTag>,
                super::private::MethodDescriptorProto<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::MsgType<self::puroro_root::google::protobuf::MethodOptions<ImplTag>>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::MethodDescriptorProto<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for MethodDescriptorProto<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::FileOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for FileOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FileOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FileOptions<NewImplTag>,
                    super::private::FileOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FileOptions<NewImplTag>,
                super::private::FileOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<
                self::puroro_root::google::protobuf::puroro_nested::file_options::OptimizeMode,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::FileOptions<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for FileOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::MessageOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for MessageOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for MessageOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::MessageOptions<NewImplTag>,
                    super::private::MessageOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::MessageOptions<NewImplTag>,
                super::private::MessageOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::MessageOptions<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for MessageOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::FieldOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for FieldOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for FieldOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::FieldOptions<NewImplTag>,
                    super::private::FieldOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::FieldOptions<NewImplTag>,
                super::private::FieldOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<self::puroro_root::google::protobuf::puroro_nested::field_options::Ctype>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
            >>::EnumType<self::puroro_root::google::protobuf::puroro_nested::field_options::Jstype>:
                std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::FieldOptions<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for FieldOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::OneofOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for OneofOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for OneofOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::OneofOptions<NewImplTag>,
                    super::private::OneofOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::OneofOptions<NewImplTag>,
                super::private::OneofOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::OneofOptions<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for OneofOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::EnumOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for EnumOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumOptions<NewImplTag>,
                    super::private::EnumOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumOptions<NewImplTag>,
                super::private::EnumOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::EnumOptions<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::EnumValueOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for EnumValueOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumValueOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::EnumValueOptions<NewImplTag>,
                    super::private::EnumValueOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::EnumValueOptions<NewImplTag>,
                super::private::EnumValueOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::EnumValueOptions<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for EnumValueOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::ServiceOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for ServiceOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ServiceOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::ServiceOptions<NewImplTag>,
                    super::private::ServiceOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::ServiceOptions<NewImplTag>,
                super::private::ServiceOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bool,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>>:
                std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::ServiceOptions<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for ServiceOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::MethodOptions<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for MethodOptions<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for MethodOptions<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::MethodOptions<NewImplTag>,
                    super::private::MethodOptions<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::MethodOptions<NewImplTag>,
                super::private::MethodOptions<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Bool
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::EnumTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::EnumType::<
                self::puroro_root::google::protobuf::puroro_nested::method_options::IdempotencyLevel
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::UninterpretedOption<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::MethodOptions<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for MethodOptions<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::UninterpretedOption<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for UninterpretedOption<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for UninterpretedOption<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::UninterpretedOption<NewImplTag>,
                    super::private::UninterpretedOption<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::UninterpretedOption<NewImplTag>,
                super::private::UninterpretedOption<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::uninterpreted_option::NamePart<
                    ImplTag,
                >,
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::UInt64,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Int64,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Double,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::Bytes,
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Optional,
                ::puroro::tags::String,
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::UninterpretedOption<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f,
                )
            }
        }

        impl<ImplTag> ::std::fmt::Display for UninterpretedOption<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::SourceCodeInfo<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for SourceCodeInfo<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for SourceCodeInfo<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::SourceCodeInfo<NewImplTag>,
                    super::private::SourceCodeInfo<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::SourceCodeInfo<NewImplTag>,
                super::private::SourceCodeInfo<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::source_code_info::Location<
                    ImplTag,
                >,
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::SourceCodeInfo<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for SourceCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }

        #[allow(unused)]
        pub struct GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::GeneratedCodeInfo<ImplTag>,
        }

        impl<ImplTag> ::puroro::Message for GeneratedCodeInfo<ImplTag> where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen
        {
        }

        impl<ImplTag> ::puroro_internal::MessageInternal for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;

            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }

        impl<ImplTag> ::puroro::DeserFromBytesIter for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }

        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::DeserAnyFieldFromBytesIter,
        {
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(
                    &mut self.body,
                    field_number,
                    data,
                )
            }
        }

        impl<ImplTag> ::puroro::SerToIoWrite for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::SerAnyFieldToIoWrite
                + ::puroro_internal::StructInternalTypeGen,
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write,
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }

        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for GeneratedCodeInfo<ImplTag>
        where
            NewImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::GeneratedCodeInfo<NewImplTag>,
                    super::private::GeneratedCodeInfo<NewImplTag>,
                >,
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::GeneratedCodeInfo<NewImplTag>,
                super::private::GeneratedCodeInfo<NewImplTag>,
            >>::Type;
        }

        impl<ImplTag> ::std::default::Default for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                    ::std::default::Default::default(),
                )
            }
        }

        impl<ImplTag> ::std::clone::Clone for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }

        impl<ImplTag> ::std::fmt::Debug for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2,
                ::puroro::tags::Repeated,
            >>::MsgType<
                self::puroro_root::google::protobuf::puroro_nested::generated_code_info::Annotation<
                    ImplTag,
                >,
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::GeneratedCodeInfo<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
            }
        }

        impl<ImplTag> ::std::fmt::Display for GeneratedCodeInfo<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
    }
} // mod puroro_structs
pub type FileDescriptorSet<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::FileDescriptorSet<ImplTag>;
pub type FileDescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::FileDescriptorProto<ImplTag>;
pub type DescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::DescriptorProto<ImplTag>;
pub type ExtensionRangeOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::ExtensionRangeOptions<ImplTag>;
pub type FieldDescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::FieldDescriptorProto<ImplTag>;
pub type OneofDescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::OneofDescriptorProto<ImplTag>;
pub type EnumDescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::EnumDescriptorProto<ImplTag>;
pub type EnumValueDescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::EnumValueDescriptorProto<ImplTag>;
pub type ServiceDescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::ServiceDescriptorProto<ImplTag>;
pub type MethodDescriptorProto<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::MethodDescriptorProto<ImplTag>;
pub type FileOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::FileOptions<ImplTag>;
pub type MessageOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::MessageOptions<ImplTag>;
pub type FieldOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::FieldOptions<ImplTag>;
pub type OneofOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::OneofOptions<ImplTag>;
pub type EnumOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::EnumOptions<ImplTag>;
pub type EnumValueOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::EnumValueOptions<ImplTag>;
pub type ServiceOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::ServiceOptions<ImplTag>;
pub type MethodOptions<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::MethodOptions<ImplTag>;
pub type UninterpretedOption<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::UninterpretedOption<ImplTag>;
pub type SourceCodeInfo<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::SourceCodeInfo<ImplTag>;
pub type GeneratedCodeInfo<ImplTag = ::puroro_internal::SimpleImpl> =
    self::puroro_structs::public::GeneratedCodeInfo<ImplTag>;

pub mod puroro_traits {
    mod puroro_root {
        pub use super::super::puroro_root::*;
    }
} // mod puroro_traits

pub use self::puroro_traits::*;

pub mod puroro_nested {

    pub mod descriptor_proto {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;

                #[allow(unused)]
                pub struct ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    /// optional int32 start = 1;
                    pub start: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// optional int32 end = 2;
                    pub end: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// optional ExtensionRangeOptions options = 3;
                    pub options: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::MsgType<
                        self::puroro_root::google::protobuf::ExtensionRangeOptions<ImplTag>,
                    >,
                    puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
                }

                impl<ImplTag> ::puroro::Message for ExtensionRange<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            start: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            options: <ImplTag as ::puroro_internal::MsgTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                            >>::default(&internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        match field_number {
                            1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.start,
                                data,
                                &self.puroro_internal,
                            ),
                            2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.end,
                                data,
                                &self.puroro_internal,
                            ),
                            3 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                            >>::DeserMsg::<
                                self::puroro_root::google::protobuf::ExtensionRangeOptions<ImplTag>,
                            >::deser_from_scoped_bytes_iter(
                                &mut self.options,
                                data,
                                &self.puroro_internal,
                            ),

                            _ => {
                                ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?
                            }
                        }
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.start, 1, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.end, 2, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                        >>::SerMsg::<
                            self::puroro_root::google::protobuf::ExtensionRangeOptions<ImplTag>,
                        >::ser_to_io_write(
                            &self.options, 3, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                        Ok(())
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ExtensionRange<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::ExtensionRange<NewImplTag>,
                            super::private::ExtensionRange<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::ExtensionRange<NewImplTag>,
                        super::private::ExtensionRange<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                        Self {
                            start: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(
                                &self.start, &internal_data
                            ),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(&self.end, &internal_data),
                            options: <ImplTag as ::puroro_internal::MsgTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                            >>::clone(
                                &self.options, &internal_data
                            ),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::MsgType<
                        self::puroro_root::google::protobuf::ExtensionRangeOptions<ImplTag>,
                    >: std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct("ExtensionRange")
                            .field("start", &self.start)
                            .field("end", &self.end)
                            .field("options", &self.options)
                            .finish()
                    }
                }

                impl<ImplTag> ::std::fmt::Display for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }

                #[allow(unused)]
                pub struct ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    /// optional int32 start = 1;
                    pub start: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// optional int32 end = 2;
                    pub end: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
                }

                impl<ImplTag> ::puroro::Message for ReservedRange<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            start: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        match field_number {
                            1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.start,
                                data,
                                &self.puroro_internal,
                            ),
                            2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.end,
                                data,
                                &self.puroro_internal,
                            ),

                            _ => {
                                ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?
                            }
                        }
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.start, 1, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.end, 2, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                        Ok(())
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ReservedRange<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::ReservedRange<NewImplTag>,
                            super::private::ReservedRange<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::ReservedRange<NewImplTag>,
                        super::private::ReservedRange<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                        Self {
                            start: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(
                                &self.start, &internal_data
                            ),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(&self.end, &internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct("ReservedRange")
                            .field("start", &self.start)
                            .field("end", &self.end)
                            .finish()
                    }
                }

                impl<ImplTag> ::std::fmt::Display for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                #[allow(unused)]
                pub struct ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    body: super::public::ExtensionRange<ImplTag>,
                }

                impl<ImplTag> ::puroro::Message for ExtensionRange<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            body: ::puroro_internal::MessageInternal::new_with_internal_data(
                                internal_data,
                            ),
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::MessageFromBytesIter::deser_field(
                            &mut self.body,
                            field_number,
                            data,
                        )
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        ::puroro::SerToIoWrite::ser(&self.body, out)
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ExtensionRange<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::ExtensionRange<NewImplTag>,
                            super::private::ExtensionRange<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::ExtensionRange<NewImplTag>,
                        super::private::ExtensionRange<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        Self {
                            body: ::std::clone::Clone::clone(&self.body),
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                    >>::MsgType<
                        self::puroro_root::google::protobuf::ExtensionRangeOptions<ImplTag>,
                    >: std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <super::public::ExtensionRange<ImplTag> as ::std::fmt::Debug>::fmt(
                            &self.body, f,
                        )
                    }
                }

                impl<ImplTag> ::std::fmt::Display for ExtensionRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }

                #[allow(unused)]
                pub struct ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    body: super::public::ReservedRange<ImplTag>,
                }

                impl<ImplTag> ::puroro::Message for ReservedRange<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            body: ::puroro_internal::MessageInternal::new_with_internal_data(
                                internal_data,
                            ),
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::MessageFromBytesIter::deser_field(
                            &mut self.body,
                            field_number,
                            data,
                        )
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        ::puroro::SerToIoWrite::ser(&self.body, out)
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for ReservedRange<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::ReservedRange<NewImplTag>,
                            super::private::ReservedRange<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::ReservedRange<NewImplTag>,
                        super::private::ReservedRange<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        Self {
                            body: ::std::clone::Clone::clone(&self.body),
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <super::public::ReservedRange<ImplTag> as ::std::fmt::Debug>::fmt(
                            &self.body, f,
                        )
                    }
                }

                impl<ImplTag> ::std::fmt::Display for ReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }
        } // mod puroro_structs
        pub type ExtensionRange<ImplTag = ::puroro_internal::SimpleImpl> =
            self::puroro_structs::public::ExtensionRange<ImplTag>;
        pub type ReservedRange<ImplTag = ::puroro_internal::SimpleImpl> =
            self::puroro_structs::public::ReservedRange<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod descriptor_proto

    pub mod field_descriptor_proto {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }
            }
        } // mod puroro_structs

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;

        #[derive(Debug, Clone, PartialEq)]
        pub enum Type {
            TypeDouble = 1,
            TypeFloat = 2,
            TypeInt64 = 3,
            TypeUint64 = 4,
            TypeInt32 = 5,
            TypeFixed64 = 6,
            TypeFixed32 = 7,
            TypeBool = 8,
            TypeString = 9,
            TypeGroup = 10,
            TypeMessage = 11,
            TypeBytes = 12,
            TypeUint32 = 13,
            TypeEnum = 14,
            TypeSfixed32 = 15,
            TypeSfixed64 = 16,
            TypeSint32 = 17,
            TypeSint64 = 18,
        }

        impl ::puroro::Enum for Type {}

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
                value as i32
            }
        }

        impl ::std::default::Default for Type {
            fn default() -> Self {
                Type::TypeDouble
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum Label {
            LabelOptional = 1,
            LabelRequired = 2,
            LabelRepeated = 3,
        }

        impl ::puroro::Enum for Label {}

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
                value as i32
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

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;

                #[allow(unused)]
                pub struct EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    /// optional int32 start = 1;
                    pub start: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// optional int32 end = 2;
                    pub end: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
                }

                impl<ImplTag> ::puroro::Message for EnumReservedRange<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            start: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        match field_number {
                            1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.start,
                                data,
                                &self.puroro_internal,
                            ),
                            2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.end,
                                data,
                                &self.puroro_internal,
                            ),

                            _ => {
                                ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?
                            }
                        }
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.start, 1, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.end, 2, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                        Ok(())
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumReservedRange<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::EnumReservedRange<NewImplTag>,
                            super::private::EnumReservedRange<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::EnumReservedRange<NewImplTag>,
                        super::private::EnumReservedRange<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                        Self {
                            start: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(
                                &self.start, &internal_data
                            ),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(&self.end, &internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct("EnumReservedRange")
                            .field("start", &self.start)
                            .field("end", &self.end)
                            .finish()
                    }
                }

                impl<ImplTag> ::std::fmt::Display for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                #[allow(unused)]
                pub struct EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    body: super::public::EnumReservedRange<ImplTag>,
                }

                impl<ImplTag> ::puroro::Message for EnumReservedRange<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            body: ::puroro_internal::MessageInternal::new_with_internal_data(
                                internal_data,
                            ),
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::MessageFromBytesIter::deser_field(
                            &mut self.body,
                            field_number,
                            data,
                        )
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        ::puroro::SerToIoWrite::ser(&self.body, out)
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for EnumReservedRange<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::EnumReservedRange<NewImplTag>,
                            super::private::EnumReservedRange<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::EnumReservedRange<NewImplTag>,
                        super::private::EnumReservedRange<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        Self {
                            body: ::std::clone::Clone::clone(&self.body),
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <super::public::EnumReservedRange<ImplTag> as ::std::fmt::Debug>::fmt(
                            &self.body, f,
                        )
                    }
                }

                impl<ImplTag> ::std::fmt::Display for EnumReservedRange<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }
        } // mod puroro_structs
        pub type EnumReservedRange<ImplTag = ::puroro_internal::SimpleImpl> =
            self::puroro_structs::public::EnumReservedRange<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod enum_descriptor_proto

    pub mod file_options {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }
            }
        } // mod puroro_structs

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;

        #[derive(Debug, Clone, PartialEq)]
        pub enum OptimizeMode {
            Speed = 1,
            CodeSize = 2,
            LiteRuntime = 3,
        }

        impl ::puroro::Enum for OptimizeMode {}

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
                value as i32
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

        pub mod puroro_structs {

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }
            }
        } // mod puroro_structs

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;

        #[derive(Debug, Clone, PartialEq)]
        pub enum Ctype {
            String = 0,
            Cord = 1,
            StringPiece = 2,
        }

        impl ::puroro::Enum for Ctype {}

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
                value as i32
            }
        }

        impl ::std::default::Default for Ctype {
            fn default() -> Self {
                Ctype::String
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum Jstype {
            JsNormal = 0,
            JsString = 1,
            JsNumber = 2,
        }

        impl ::puroro::Enum for Jstype {}

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
                value as i32
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

        pub mod puroro_structs {

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }
            }
        } // mod puroro_structs

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;

        #[derive(Debug, Clone, PartialEq)]
        pub enum IdempotencyLevel {
            IdempotencyUnknown = 0,
            NoSideEffects = 1,
            Idempotent = 2,
        }

        impl ::puroro::Enum for IdempotencyLevel {}

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
                value as i32
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

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;

                #[allow(unused)]
                pub struct NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    /// required string name_part = 1;
                    pub name_part: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Required,
                        ::puroro::tags::String,
                    >>::Type,
                    /// required bool is_extension = 2;
                    pub is_extension: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Required,
                        ::puroro::tags::Bool,
                    >>::Type,
                    puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
                }

                impl<ImplTag> ::puroro::Message for NamePart<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            name_part: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Required,
                                ::puroro::tags::String,
                            >>::default(&internal_data),
                            is_extension: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Required,
                                ::puroro::tags::Bool,
                            >>::default(&internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        match field_number {
                            1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Required,
                                ::puroro::tags::String,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.name_part,
                                data,
                                &self.puroro_internal,
                            ),
                            2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Required,
                                ::puroro::tags::Bool,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.is_extension,
                                data,
                                &self.puroro_internal,
                            ),

                            _ => {
                                ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?
                            }
                        }
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Required,
                            ::puroro::tags::String,
                        >>::ser_to_io_write(
                            &self.name_part, 1, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Required,
                            ::puroro::tags::Bool,
                        >>::ser_to_io_write(
                            &self.is_extension, 2, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                        Ok(())
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for NamePart<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::NamePart<NewImplTag>,
                            super::private::NamePart<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::NamePart<NewImplTag>,
                        super::private::NamePart<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                        Self {
                            name_part: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Required,
                                ::puroro::tags::String,
                            >>::clone(
                                &self.name_part, &internal_data
                            ),
                            is_extension: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Required,
                                ::puroro::tags::Bool,
                            >>::clone(
                                &self.is_extension, &internal_data
                            ),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Required,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Required,
                        ::puroro::tags::Bool,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct("NamePart")
                            .field("name_part", &self.name_part)
                            .field("is_extension", &self.is_extension)
                            .finish()
                    }
                }

                impl<ImplTag> ::std::fmt::Display for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                #[allow(unused)]
                pub struct NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    body: super::public::NamePart<ImplTag>,
                }

                impl<ImplTag> ::puroro::Message for NamePart<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            body: ::puroro_internal::MessageInternal::new_with_internal_data(
                                internal_data,
                            ),
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::MessageFromBytesIter::deser_field(
                            &mut self.body,
                            field_number,
                            data,
                        )
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        ::puroro::SerToIoWrite::ser(&self.body, out)
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for NamePart<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::NamePart<NewImplTag>,
                            super::private::NamePart<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::NamePart<NewImplTag>,
                        super::private::NamePart<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        Self {
                            body: ::std::clone::Clone::clone(&self.body),
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Required,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Required,
                        ::puroro::tags::Bool,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <super::public::NamePart<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
                    }
                }

                impl<ImplTag> ::std::fmt::Display for NamePart<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }
        } // mod puroro_structs
        pub type NamePart<ImplTag = ::puroro_internal::SimpleImpl> =
            self::puroro_structs::public::NamePart<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod uninterpreted_option

    pub mod source_code_info {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;

                #[allow(unused)]
                pub struct Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    /// repeated int32 path = 1;
                    pub path: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// repeated int32 span = 2;
                    pub span: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// optional string leading_comments = 3;
                    pub leading_comments: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type,
                    /// optional string trailing_comments = 4;
                    pub trailing_comments: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type,
                    /// repeated string leading_detached_comments = 6;
                    pub leading_detached_comments: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::Type,
                    puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
                }

                impl<ImplTag> ::puroro::Message for Location<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            path: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            span: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            leading_comments: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::default(
                                &internal_data
                            ),
                            trailing_comments: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::default(
                                &internal_data
                            ),
                            leading_detached_comments:
                                <ImplTag as ::puroro_internal::FieldTypeGen<
                                    ::puroro::tags::Proto2,
                                    ::puroro::tags::Repeated,
                                    ::puroro::tags::String,
                                >>::default(&internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        match field_number {
                            1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.path,
                                data,
                                &self.puroro_internal,
                            ),
                            2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.span,
                                data,
                                &self.puroro_internal,
                            ),
                            3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.leading_comments,
                                data,
                                &self.puroro_internal,
                            ),
                            4 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.trailing_comments,
                                data,
                                &self.puroro_internal,
                            ),
                            6 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::String,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.leading_detached_comments,
                                data,
                                &self.puroro_internal,
                            ),

                            _ => {
                                ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?
                            }
                        }
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.path, 1, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.span, 2, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >>::ser_to_io_write(
                            &self.leading_comments, 3, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >>::ser_to_io_write(
                            &self.trailing_comments,
                            4,
                            out,
                            &self.puroro_internal,
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Repeated,
                            ::puroro::tags::String,
                        >>::ser_to_io_write(
                            &self.leading_detached_comments,
                            6,
                            out,
                            &self.puroro_internal,
                        )?;
                        <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                        Ok(())
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for Location<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::Location<NewImplTag>,
                            super::private::Location<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::Location<NewImplTag>,
                        super::private::Location<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                        Self {
                            path: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::clone(&self.path, &internal_data),
                            span: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::clone(&self.span, &internal_data),
                            leading_comments: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::clone(
                                &self.leading_comments, &internal_data
                            ),
                            trailing_comments: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::clone(
                                &self.trailing_comments, &internal_data
                            ),
                            leading_detached_comments:
                                <ImplTag as ::puroro_internal::FieldTypeGen<
                                    ::puroro::tags::Proto2,
                                    ::puroro::tags::Repeated,
                                    ::puroro::tags::String,
                                >>::clone(
                                    &self.leading_detached_comments, &internal_data
                                ),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct("Location")
                            .field("path", &self.path)
                            .field("span", &self.span)
                            .field("leading_comments", &self.leading_comments)
                            .field("trailing_comments", &self.trailing_comments)
                            .field("leading_detached_comments", &self.leading_detached_comments)
                            .finish()
                    }
                }

                impl<ImplTag> ::std::fmt::Display for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                #[allow(unused)]
                pub struct Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    body: super::public::Location<ImplTag>,
                }

                impl<ImplTag> ::puroro::Message for Location<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            body: ::puroro_internal::MessageInternal::new_with_internal_data(
                                internal_data,
                            ),
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::MessageFromBytesIter::deser_field(
                            &mut self.body,
                            field_number,
                            data,
                        )
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        ::puroro::SerToIoWrite::ser(&self.body, out)
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for Location<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::Location<NewImplTag>,
                            super::private::Location<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::Location<NewImplTag>,
                        super::private::Location<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        Self {
                            body: ::std::clone::Clone::clone(&self.body),
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <super::public::Location<ImplTag> as ::std::fmt::Debug>::fmt(&self.body, f)
                    }
                }

                impl<ImplTag> ::std::fmt::Display for Location<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }
        } // mod puroro_structs
        pub type Location<ImplTag = ::puroro_internal::SimpleImpl> =
            self::puroro_structs::public::Location<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod source_code_info

    pub mod generated_code_info {
        mod puroro_root {
            pub use super::super::super::puroro_root::*;
        }

        pub mod puroro_structs {

            pub mod public {

                pub mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                use ::puroro_internal::de::DeserEnumFromBytesIter as _;
                use ::puroro_internal::de::DeserMsgFromBytesIter as _;
                use ::puroro_internal::se::SerEnumToIoWrite as _;
                use ::puroro_internal::se::SerMsgToIoWrite as _;

                #[allow(unused)]
                pub struct Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    /// repeated int32 path = 1;
                    pub path: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// optional string source_file = 2;
                    pub source_file: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type,
                    /// optional int32 begin = 3;
                    pub begin: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    /// optional int32 end = 4;
                    pub end: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type,
                    puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
                }

                impl<ImplTag> ::puroro::Message for Annotation<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            path: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            source_file: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::default(&internal_data),
                            begin: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::default(&internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        match field_number {
                            1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.path,
                                data,
                                &self.puroro_internal,
                            ),
                            2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.source_file,
                                data,
                                &self.puroro_internal,
                            ),
                            3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.begin,
                                data,
                                &self.puroro_internal,
                            ),
                            4 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::deser_from_scoped_bytes_iter(
                                &mut self.end,
                                data,
                                &self.puroro_internal,
                            ),

                            _ => {
                                ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?
                            }
                        }
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Repeated,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.path, 1, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::String,
                        >>::ser_to_io_write(
                            &self.source_file, 2, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.begin, 3, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                            ::puroro::tags::Proto2,
                            ::puroro::tags::Optional,
                            ::puroro::tags::Int32,
                        >>::ser_to_io_write(
                            &self.end, 4, out, &self.puroro_internal
                        )?;
                        <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                        Ok(())
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for Annotation<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::Annotation<NewImplTag>,
                            super::private::Annotation<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::Annotation<NewImplTag>,
                        super::private::Annotation<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                        Self {
                            path: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Repeated,
                                ::puroro::tags::Int32,
                            >>::clone(&self.path, &internal_data),
                            source_file: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::String,
                            >>::clone(
                                &self.source_file, &internal_data
                            ),
                            begin: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(
                                &self.begin, &internal_data
                            ),
                            end: <ImplTag as ::puroro_internal::FieldTypeGen<
                                ::puroro::tags::Proto2,
                                ::puroro::tags::Optional,
                                ::puroro::tags::Int32,
                            >>::clone(&self.end, &internal_data),
                            puroro_internal: internal_data,
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.debug_struct("Annotation")
                            .field("path", &self.path)
                            .field("source_file", &self.source_file)
                            .field("begin", &self.begin)
                            .field("end", &self.end)
                            .finish()
                    }
                }

                impl<ImplTag> ::std::fmt::Display for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }

            pub mod private {

                mod puroro_root {
                    pub use super::super::super::puroro_root::*;
                }

                #[allow(unused)]
                pub struct Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    body: super::public::Annotation<ImplTag>,
                }

                impl<ImplTag> ::puroro::Message for Annotation<ImplTag> where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                {
                }

                impl<ImplTag> ::puroro_internal::MessageInternal for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type ImplTypeTag = ImplTag;

                    fn new_with_internal_data(
                        internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
                    ) -> Self {
                        Self {
                            body: ::puroro_internal::MessageInternal::new_with_internal_data(
                                internal_data,
                            ),
                        }
                    }
                }

                impl<ImplTag> ::puroro::DeserFromBytesIter for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
                    }
                }

                impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::DeserAnyFieldFromBytesIter,
                {
                    fn deser_field<I>(
                        &mut self,
                        field_number: i32,
                        data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
                    ) -> ::puroro::Result<()>
                    where
                        I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
                    {
                        ::puroro_internal::de::MessageFromBytesIter::deser_field(
                            &mut self.body,
                            field_number,
                            data,
                        )
                    }
                }

                impl<ImplTag> ::puroro::SerToIoWrite for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::SerAnyFieldToIoWrite
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
                    where
                        W: ::std::io::Write,
                    {
                        ::puroro::SerToIoWrite::ser(&self.body, out)
                    }
                }

                impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for Annotation<ImplTag>
                where
                    NewImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen
                        + ::puroro_internal::ChooseStructVisibility<
                            super::public::Annotation<NewImplTag>,
                            super::private::Annotation<NewImplTag>,
                        >,
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                {
                    type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                        super::public::Annotation<NewImplTag>,
                        super::private::Annotation<NewImplTag>,
                    >>::Type;
                }

                impl<ImplTag> ::std::default::Default for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::default::Default,
                {
                    fn default() -> Self {
                        <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(
                            ::std::default::Default::default(),
                        )
                    }
                }

                impl<ImplTag> ::std::clone::Clone for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type:
                        ::std::clone::Clone,
                {
                    fn clone(&self) -> Self {
                        Self {
                            body: ::std::clone::Clone::clone(&self.body),
                        }
                    }
                }

                impl<ImplTag> ::std::fmt::Debug for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Repeated,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::String,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                    <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2,
                        ::puroro::tags::Optional,
                        ::puroro::tags::Int32,
                    >>::Type: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <super::public::Annotation<ImplTag> as ::std::fmt::Debug>::fmt(
                            &self.body, f,
                        )
                    }
                }

                impl<ImplTag> ::std::fmt::Display for Annotation<ImplTag>
                where
                    ImplTag: ::puroro_internal::AnyFieldTypeGen
                        + ::puroro_internal::StructInternalTypeGen,
                    Self: ::std::fmt::Debug,
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        <Self as ::std::fmt::Debug>::fmt(self, f)
                    }
                }
            }
        } // mod puroro_structs
        pub type Annotation<ImplTag = ::puroro_internal::SimpleImpl> =
            self::puroro_structs::public::Annotation<ImplTag>;

        pub mod puroro_traits {
            mod puroro_root {
                pub use super::super::puroro_root::*;
            }
        } // mod puroro_traits

        pub use self::puroro_traits::*;
    } // mod generated_code_info
} // mod puroro_nested

pub use self::puroro_nested::*;
