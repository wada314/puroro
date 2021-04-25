#[derive(Debug, Clone)]
pub struct CodeGeneratorResponse {
    pub error: ::std::string::String,
    pub supported_features: u64,
    pub file: ::std::vec::Vec<
        super::super::super::google::protobuf::compiler::code_generator_response::File,
    >,
}
impl ::std::default::Default for CodeGeneratorResponse {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            error: ::std::default::Default::default(),
            supported_features: ::std::default::Default::default(),
            file: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut CodeGeneratorResponse
{
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        use puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.supported_features) =
                        variant.to_native::<::puroro_internal::tags::UInt64>()?;
                }
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.error) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| {
                                variant.to_native::<::puroro_internal::tags::UInt64>()
                            })
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.supported_features, first, iter);
                }
                15 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.file);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for CodeGeneratorResponse {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <CodeGeneratorResponse as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for CodeGeneratorResponse {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.error) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::UInt64, _>(
            2,
            MaybeRepeatedField::iter(&self.supported_features)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.file) {
            serializer.serialize_message_twice(15, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for CodeGeneratorResponse {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
pub mod code_generator_response {
    #[derive(Debug, Clone)]
    pub enum Feature {
        FeatureNone = 0,
        FeatureProto3Optional = 1,
    }
    impl std::convert::TryFrom<i32> for Feature {
        type Error = i32;
        fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::FeatureNone),
                1 => Ok(Self::FeatureProto3Optional),
                x => Err(x),
            }
        }
    }
    impl std::convert::Into<i32> for Feature {
        fn into(self) -> i32 {
            self as i32
        }
    }
    #[derive(Debug, Clone)]
    pub struct File {
        pub name: ::std::string::String,
        pub insertion_point: ::std::string::String,
        pub content: ::std::string::String,
        pub generated_code_info: ::std::option::Optional<
            ::std::boxed::Box<super::super::super::super::google::protobuf::GeneratedCodeInfo>,
        >,
    }
    impl ::std::default::Default for File {
        fn default() -> Self {
            #[allow(unused)]
            use std::convert::TryInto;
            Self {
                name: ::std::default::Default::default(),
                insertion_point: ::std::default::Default::default(),
                content: ::std::default::Default::default(),
                generated_code_info: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut File {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
        }
        fn met_field<T: ::puroro::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            use puroro::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                },
                ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        *MaybeRepeatedField::last_mut(&mut self.name) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *MaybeRepeatedField::last_mut(&mut self.insertion_point) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    15 => {
                        *MaybeRepeatedField::last_mut(&mut self.content) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let msg = MaybeRepeatedField::last_mut(&mut self.generated_code_info);
                        ldd.deserialize_as_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for File {
        fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            iter: I,
        ) -> ::puroro::Result<Self> {
            use puroro::deserializer::stream::Deserializer;
            let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            let mut msg = <File as ::std::default::Default>::default();
            deserializer.deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro::serializer::Serializable for File {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            for string in MaybeRepeatedField::iter(&self.name) {
                serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
            }
            for string in MaybeRepeatedField::iter(&self.insertion_point) {
                serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
            }
            for string in MaybeRepeatedField::iter(&self.content) {
                serializer.serialize_bytes_twice(15, string.bytes().map(|b| Ok(b)))?;
            }
            for msg in MaybeRepeatedField::iter(&self.generated_code_info) {
                serializer.serialize_message_twice(16, msg)?;
            }
            Ok(())
        }
    }
    impl ::puroro::Serializable for File {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro::serializer::default_serializer(write);
            <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
} // mod code_generator_response
#[derive(Debug, Clone)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: ::std::vec::Vec<::std::string::String>,
    pub parameter: ::std::string::String,
    pub proto_file: ::std::vec::Vec<super::super::super::google::protobuf::FileDescriptorProto>,
    pub compiler_version: ::std::option::Optional<
        ::std::boxed::Box<super::super::super::google::protobuf::compiler::Version>,
    >,
}
impl ::std::default::Default for CodeGeneratorRequest {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            file_to_generate: ::std::default::Default::default(),
            parameter: ::std::default::Default::default(),
            proto_file: ::std::default::Default::default(),
            compiler_version: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut CodeGeneratorRequest
{
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        use puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.file_to_generate) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.parameter) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                15 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.proto_file);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.compiler_version);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for CodeGeneratorRequest {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <CodeGeneratorRequest as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for CodeGeneratorRequest {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.file_to_generate) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.parameter) {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.proto_file) {
            serializer.serialize_message_twice(15, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.compiler_version) {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for CodeGeneratorRequest {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub suffix: ::std::string::String,
}
impl ::std::default::Default for Version {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            major: ::std::default::Default::default(),
            minor: ::std::default::Default::default(),
            patch: ::std::default::Default::default(),
            suffix: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut Version {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        use puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.major) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.minor) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.patch) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| {
                                variant.to_native::<::puroro_internal::tags::Int32>()
                            })
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.major, first, iter);
                }
                2 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| {
                                variant.to_native::<::puroro_internal::tags::Int32>()
                            })
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.minor, first, iter);
                }
                3 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| {
                                variant.to_native::<::puroro_internal::tags::Int32>()
                            })
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.patch, first, iter);
                }
                4 => {
                    *MaybeRepeatedField::last_mut(&mut self.suffix) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for Version {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <Version as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for Version {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            1,
            MaybeRepeatedField::iter(&self.major)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            2,
            MaybeRepeatedField::iter(&self.minor)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            3,
            MaybeRepeatedField::iter(&self.patch)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for string in MaybeRepeatedField::iter(&self.suffix) {
            serializer.serialize_bytes_twice(4, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for Version {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
