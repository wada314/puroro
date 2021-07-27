// A generated source code by puroro library
// package google.protobuf.compiler

pub mod puroro_root {
    pub use super::super::super::super::*;
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
        pub struct Version<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional int32 major = 1;
            pub major: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type,
            /// optional int32 minor = 2;
            pub minor: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type,
            /// optional int32 patch = 3;
            pub patch: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type,
            /// optional string suffix = 4;
            pub suffix: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }
        
        impl<ImplTag> ::puroro::Message for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    major: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32>>::default(&internal_data),
                    minor: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32>>::default(&internal_data),
                    patch: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32>>::default(&internal_data),
                    suffix: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String>>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.major, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.minor, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::deser_from_scoped_bytes_iter(&mut self.patch, data, &self.puroro_internal),
                    4 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.suffix, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                >>::ser_to_io_write(&self.major, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                >>::ser_to_io_write(&self.minor, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                >>::ser_to_io_write(&self.patch, 3, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                >>::ser_to_io_write(&self.suffix, 4, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                Ok(())
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for Version<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::Version<NewImplTag>,
                    super::private::Version<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::Version<NewImplTag>,
                super::private::Version<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for Version<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for Version<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    major: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::clone(&self.major, &internal_data),
                    minor: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::clone(&self.minor, &internal_data),
                    patch: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
                    >>::clone(&self.patch, &internal_data),
                    suffix: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::clone(&self.suffix, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for Version<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("Version")
                    .field("major", &self.major)
                    .field("minor", &self.minor)
                    .field("patch", &self.patch)
                    .field("suffix", &self.suffix)
                    .finish()
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for Version<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
        
        #[allow(unused)]
        pub struct CodeGeneratorRequest<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// repeated string file_to_generate = 1;
            pub file_to_generate: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type,
            /// optional string parameter = 2;
            pub parameter: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            /// repeated FileDescriptorProto proto_file = 15;
            pub proto_file: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>
            >,
            /// optional Version compiler_version = 3;
            pub compiler_version: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::compiler::Version<ImplTag>
            >,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }
        
        impl<ImplTag> ::puroro::Message for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    file_to_generate: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String>>::default(&internal_data),
                    parameter: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String>>::default(&internal_data),
                    proto_file: <ImplTag as ::puroro_internal::MsgTypeGen<::puroro::tags::Proto2, ::puroro::tags::Repeated>>::default(&internal_data),
                    compiler_version: <ImplTag as ::puroro_internal::MsgTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional>>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.file_to_generate, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.parameter, data, &self.puroro_internal),
                    15 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.proto_file, data, &self.puroro_internal),
                    3 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::compiler::Version<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.compiler_version, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
                >>::ser_to_io_write(&self.file_to_generate, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                >>::ser_to_io_write(&self.parameter, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>
                >::ser_to_io_write(&self.proto_file, 15, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::compiler::Version<ImplTag>
                >::ser_to_io_write(&self.compiler_version, 3, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                Ok(())
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for CodeGeneratorRequest<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::CodeGeneratorRequest<NewImplTag>,
                    super::private::CodeGeneratorRequest<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::CodeGeneratorRequest<NewImplTag>,
                super::private::CodeGeneratorRequest<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for CodeGeneratorRequest<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for CodeGeneratorRequest<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    file_to_generate: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
                    >>::clone(&self.file_to_generate, &internal_data),
                    parameter: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::clone(&self.parameter, &internal_data),
                    proto_file: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::clone(&self.proto_file, &internal_data),
                    compiler_version: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::clone(&self.compiler_version, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for CodeGeneratorRequest<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::compiler::Version<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("CodeGeneratorRequest")
                    .field("file_to_generate", &self.file_to_generate)
                    .field("parameter", &self.parameter)
                    .field("proto_file", &self.proto_file)
                    .field("compiler_version", &self.compiler_version)
                    .finish()
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for CodeGeneratorRequest<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
        
        #[allow(unused)]
        pub struct CodeGeneratorResponse<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string error = 1;
            pub error: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            /// optional uint64 supported_features = 2;
            pub supported_features: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64
            >>::Type,
            /// repeated File file = 15;
            pub file: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<ImplTag>
            >,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }
        
        impl<ImplTag> ::puroro::Message for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    error: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String>>::default(&internal_data),
                    supported_features: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64>>::default(&internal_data),
                    file: <ImplTag as ::puroro_internal::MsgTypeGen<::puroro::tags::Proto2, ::puroro::tags::Repeated>>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.error, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64
                    >>::deser_from_scoped_bytes_iter(&mut self.supported_features, data, &self.puroro_internal),
                    15 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.file, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                >>::ser_to_io_write(&self.error, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64
                >>::ser_to_io_write(&self.supported_features, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Repeated
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<ImplTag>
                >::ser_to_io_write(&self.file, 15, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                Ok(())
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for CodeGeneratorResponse<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::CodeGeneratorResponse<NewImplTag>,
                    super::private::CodeGeneratorResponse<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::CodeGeneratorResponse<NewImplTag>,
                super::private::CodeGeneratorResponse<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for CodeGeneratorResponse<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for CodeGeneratorResponse<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    error: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::clone(&self.error, &internal_data),
                    supported_features: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64
                    >>::clone(&self.supported_features, &internal_data),
                    file: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Repeated
                    >>::clone(&self.file, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for CodeGeneratorResponse<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("CodeGeneratorResponse")
                    .field("error", &self.error)
                    .field("supported_features", &self.supported_features)
                    .field("file", &self.file)
                    .finish()
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for CodeGeneratorResponse<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
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
        pub struct Version<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::Version<ImplTag>,
        }
        
        impl<ImplTag> ::puroro::Message for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(&mut self.body, field_number, data)
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for Version<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for Version<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::Version<NewImplTag>,
                    super::private::Version<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::Version<NewImplTag>,
                super::private::Version<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for Version<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for Version<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for Version<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::Int32
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::Version<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f
                )
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for Version<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
        
        #[allow(unused)]
        pub struct CodeGeneratorRequest<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::CodeGeneratorRequest<ImplTag>,
        }
        
        impl<ImplTag> ::puroro::Message for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(&mut self.body, field_number, data)
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for CodeGeneratorRequest<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for CodeGeneratorRequest<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::CodeGeneratorRequest<NewImplTag>,
                    super::private::CodeGeneratorRequest<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::CodeGeneratorRequest<NewImplTag>,
                super::private::CodeGeneratorRequest<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for CodeGeneratorRequest<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for CodeGeneratorRequest<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for CodeGeneratorRequest<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::FileDescriptorProto<ImplTag>
            >: std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::compiler::Version<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::CodeGeneratorRequest<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f
                )
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for CodeGeneratorRequest<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
        
        #[allow(unused)]
        pub struct CodeGeneratorResponse<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::CodeGeneratorResponse<ImplTag>,
        }
        
        impl<ImplTag> ::puroro::Message for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(&mut self.body, field_number, data)
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for CodeGeneratorResponse<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for CodeGeneratorResponse<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::CodeGeneratorResponse<NewImplTag>,
                    super::private::CodeGeneratorResponse<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::CodeGeneratorResponse<NewImplTag>,
                super::private::CodeGeneratorResponse<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for CodeGeneratorResponse<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for CodeGeneratorResponse<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for CodeGeneratorResponse<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::UInt64
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Repeated
            >>::MsgType::<
                self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response::File<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::CodeGeneratorResponse<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f
                )
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for CodeGeneratorResponse<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
    }

} // mod puroro_structs
pub type Version<ImplTag = ::puroro_internal::SimpleImpl> = self::puroro_structs::public::Version<ImplTag>;
pub type CodeGeneratorRequest<ImplTag = ::puroro_internal::SimpleImpl> = self::puroro_structs::public::CodeGeneratorRequest<ImplTag>;
pub type CodeGeneratorResponse<ImplTag = ::puroro_internal::SimpleImpl> = self::puroro_structs::public::CodeGeneratorResponse<ImplTag>;

pub mod puroro_traits {
    mod puroro_root {
        pub use super::super::puroro_root::*;
    }
    
    
    

} // mod puroro_traits

pub use self::puroro_traits::*;

pub mod puroro_nested {

pub mod code_generator_response {
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
        pub struct File<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            /// optional string name = 1;
            pub name: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            /// optional string insertion_point = 2;
            pub insertion_point: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            /// optional string content = 15;
            pub content: <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type,
            /// optional GeneratedCodeInfo generated_code_info = 16;
            pub generated_code_info: <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::GeneratedCodeInfo<ImplTag>
            >,
            puroro_internal: <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type,
        }
        
        impl<ImplTag> ::puroro::Message for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String>>::default(&internal_data),
                    insertion_point: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String>>::default(&internal_data),
                    content: <ImplTag as ::puroro_internal::FieldTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String>>::default(&internal_data),
                    generated_code_info: <ImplTag as ::puroro_internal::MsgTypeGen<::puroro::tags::Proto2, ::puroro::tags::Optional>>::default(&internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro_internal::de::from_iter::deser_from_iter(self, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                match field_number {
                    1 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.name, data, &self.puroro_internal),
                    2 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.insertion_point, data, &self.puroro_internal),
                    15 => <ImplTag as ::puroro_internal::de::DeserFieldFromBytesIter<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::deser_from_scoped_bytes_iter(&mut self.content, data, &self.puroro_internal),
                    16 => <ImplTag as ::puroro_internal::de::DeserMsgFromBytesIterProxy<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::DeserMsg::<
                        self::puroro_root::google::protobuf::GeneratedCodeInfo<ImplTag>
                    >::deser_from_scoped_bytes_iter(&mut self.generated_code_info, data, &self.puroro_internal),
        
                    _ => ::std::result::Result::Err(::puroro::ErrorKind::UnknownFieldNumber)?,
                }
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                >>::ser_to_io_write(&self.name, 1, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                >>::ser_to_io_write(&self.insertion_point, 2, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerFieldToIoWrite<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                >>::ser_to_io_write(&self.content, 15, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerMsgToIoWriteProxy<
                    ::puroro::tags::Proto2, ::puroro::tags::Optional
                >>::SerMsg::<
                    self::puroro_root::google::protobuf::GeneratedCodeInfo<ImplTag>
                >::ser_to_io_write(&self.generated_code_info, 16, out, &self.puroro_internal)?;
                <ImplTag as ::puroro_internal::se::SerInternalDataToIoWrite>::ser_to_io_write(out, &self.puroro_internal)?;
                Ok(())
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for File<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::File<NewImplTag>,
                    super::private::File<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::File<NewImplTag>,
                super::private::File<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for File<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for File<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {let internal_data = ::std::clone::Clone::clone(&self.puroro_internal);
                Self {
                    name: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::clone(&self.name, &internal_data),
                    insertion_point: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::clone(&self.insertion_point, &internal_data),
                    content: <ImplTag as ::puroro_internal::FieldTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
                    >>::clone(&self.content, &internal_data),
                    generated_code_info: <ImplTag as ::puroro_internal::MsgTypeGen<
                        ::puroro::tags::Proto2, ::puroro::tags::Optional
                    >>::clone(&self.generated_code_info, &internal_data),
                    puroro_internal: internal_data,
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for File<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::GeneratedCodeInfo<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("File")
                    .field("name", &self.name)
                    .field("insertion_point", &self.insertion_point)
                    .field("content", &self.content)
                    .field("generated_code_info", &self.generated_code_info)
                    .finish()
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for File<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
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
        pub struct File<ImplTag> 
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            body: super::public::File<ImplTag>,
        }
        
        impl<ImplTag> ::puroro::Message for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
        }
        
        impl<ImplTag> ::puroro_internal::MessageInternal for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
        {
            type ImplTypeTag = ImplTag;
        
            fn new_with_internal_data(
                internal_data: <Self::ImplTypeTag as ::puroro_internal::StructInternalTypeGen>::Type,
            ) -> Self {
                Self {body: ::puroro_internal::MessageInternal::new_with_internal_data(internal_data),
                }
            }
        }
        
        impl<ImplTag> ::puroro::DeserFromBytesIter for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {
            fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>
            {
                ::puroro::DeserFromBytesIter::deser(&mut self.body, iter)
            }
        }
        
        impl<ImplTag> ::puroro_internal::de::MessageFromBytesIter for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::StructInternalTypeGen
            + ::puroro_internal::DeserAnyFieldFromBytesIter
        {    
            fn deser_field<I>(
                &mut self,
                field_number: i32,
                data: ::puroro::types::FieldData<&mut ::puroro_internal::ScopedIter<I>>,
            ) -> ::puroro::Result<()>
            where
                I: ::std::iter::Iterator<Item = ::std::io::Result<u8>> 
            {
                ::puroro_internal::de::MessageFromBytesIter::deser_field(&mut self.body, field_number, data)
            }
        }
        
        
        impl<ImplTag> ::puroro::SerToIoWrite for File<ImplTag>
        where ImplTag:
            ::puroro_internal::AnyFieldTypeGen
            + ::puroro_internal::SerAnyFieldToIoWrite
            + ::puroro_internal::StructInternalTypeGen
        {
            fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()>
            where
                W: ::std::io::Write
            {
                ::puroro::SerToIoWrite::ser(&self.body, out)
            }
        }
        
        impl<ImplTag, NewImplTag> ::puroro::GetImpl<NewImplTag> for File<ImplTag> 
        where
            NewImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen
                + ::puroro_internal::ChooseStructVisibility<
                    super::public::File<NewImplTag>,
                    super::private::File<NewImplTag>,
                >,
            ImplTag:
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
        {
            type Type = <NewImplTag as ::puroro_internal::ChooseStructVisibility<
                super::public::File<NewImplTag>,
                super::private::File<NewImplTag>,
            >>::Type;
        }
        
        impl<ImplTag> ::std::default::Default for File<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::default::Default,
        {
            fn default() -> Self {
                <Self as ::puroro_internal::MessageInternal>::new_with_internal_data(::std::default::Default::default())
            }
        }
        
        impl<ImplTag> ::std::clone::Clone for File<ImplTag>
        where 
            ImplTag: 
                ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::StructInternalTypeGen>::Type: ::std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    body: ::std::clone::Clone::clone(&self.body),
                }
            }
        }
        
        impl<ImplTag> ::std::fmt::Debug for File<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen
                + ::puroro_internal::StructInternalTypeGen,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::FieldTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional, ::puroro::tags::String
            >>::Type: ::std::fmt::Debug,
            <ImplTag as ::puroro_internal::MsgTypeGen<
                ::puroro::tags::Proto2, ::puroro::tags::Optional
            >>::MsgType::<
                self::puroro_root::google::protobuf::GeneratedCodeInfo<ImplTag>
            >: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <super::public::File<ImplTag> as ::std::fmt::Debug>::fmt(
                    &self.body, f
                )
            }
        }
        
        impl<ImplTag> ::std::fmt::Display for File<ImplTag>
        where
            ImplTag: ::puroro_internal::AnyFieldTypeGen + ::puroro_internal::StructInternalTypeGen,
            Self: ::std::fmt::Debug
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <Self as ::std::fmt::Debug>::fmt(self, f)
            }
        }
    }

} // mod puroro_structs
pub type File<ImplTag = ::puroro_internal::SimpleImpl> = self::puroro_structs::public::File<ImplTag>;

pub mod puroro_traits {
    mod puroro_root {
        pub use super::super::puroro_root::*;
    }
    

} // mod puroro_traits

pub use self::puroro_traits::*;


#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    FeatureNone = 0,
    FeatureProto3Optional = 1,
}

impl ::puroro::Enum for Feature {
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
        value as i32
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


