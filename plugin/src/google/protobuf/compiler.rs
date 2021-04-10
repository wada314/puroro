pub struct CodeGeneratorResponse {
    error: String,
    supported_features: u64,
    file: ::std::vec::Vec<super::super::super::google::protobuf::compiler::code_generator_response::File>,
}
impl ::std::default::Default for CodeGeneratorResponse {
    fn default() -> Self {
        Self {
            error: ::std::default::Default::default(),
            supported_features: ::std::default::Default::default(),
            file: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for CodeGeneratorResponse {
    type Target = Self;
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(self)
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    self.supported_features = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.error = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.supported_features = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())?;
                }
                15 => {
                    self.file.push(ldd.deserialize_as_message(
                        <super::super::super::google::protobuf::compiler::code_generator_response::File as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for CodeGeneratorResponse {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use ::puroro_serializer::deserializer::stream::Deserializer;
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}
impl ::puroro_serializer::serializer::Serializable for CodeGeneratorResponse {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.error.bytes().map(|b| Ok(b)))?;
        unimplemented!("Serializer for something else");for msg in &self.file {
            serializer.serialize_message_twice::<super::super::super::google::protobuf::compiler::code_generator_response::File>(15, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for CodeGeneratorResponse {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)?;
    }
}
    mod code_generator_response {
    pub enum Feature {
        FeatureNone = 0,
        FeatureProto3Optional = 1,
    }
    impl std::convert::TryFrom<i32> for Feature {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::FeatureNone),
                1 => Ok(Self::FeatureProto3Optional),
                x => Err(x),
            }
        }
    }
    pub struct File {
        name: String,
        insertion_point: String,
        content: String,
        generated_code_info: ::std::option::Option<::std::boxed::Box<super::super::super::super::google::protobuf::GeneratedCodeInfo>>,
    }
    impl ::std::default::Default for File {
        fn default() -> Self {
            Self {
                name: ::std::default::Default::default(),
                insertion_point: ::std::default::Default::default(),
                content: ::std::default::Default::default(),
                generated_code_info: ::std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for File {
        type Target = Self;
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(self)
        }
        fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro_serializer::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            match field {
                ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        self.insertion_point = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    15 => {
                        self.content = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let msg = self.generated_code_info.get_or_insert_with(<super::super::super::super::google::protobuf::GeneratedCodeInfo as ::std::default::Default>::default);
                        self.generated_code_info = Some(ldd.deserialize_as_message(msg)?);
                    }
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for File {
        fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
            use ::puroro_serializer::deserializer::stream::Deserializer;
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
                <Self as ::std::default::Default>::default())
        }
    }
    impl ::puroro_serializer::serializer::Serializable for File {
        fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
            &self, serializer: &mut T) -> ::puroro::Result<()>
        {
            serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
            serializer.serialize_bytes_twice(2, self.insertion_point.bytes().map(|b| Ok(b)))?;
            serializer.serialize_bytes_twice(15, self.content.bytes().map(|b| Ok(b)))?;
            if let Some(msg) = &self.generated_code_info {
                serializer.serialize_message_twice::<super::super::super::super::google::protobuf::GeneratedCodeInfo>(16, msg)?;
            }
            Ok(())
        }
    }
    impl ::puroro::Serializable for File {
        fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
            <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)?;
        }
    }
        }
pub struct CodeGeneratorRequest {
    file_to_generate: ::std::vec::Vec<String>,
    parameter: String,
    proto_file: ::std::vec::Vec<super::super::super::google::protobuf::FileDescriptorProto>,
    compiler_version: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::compiler::Version>>,
}
impl ::std::default::Default for CodeGeneratorRequest {
    fn default() -> Self {
        Self {
            file_to_generate: ::std::default::Default::default(),
            parameter: ::std::default::Default::default(),
            proto_file: ::std::default::Default::default(),
            compiler_version: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for CodeGeneratorRequest {
    type Target = Self;
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(self)
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.file_to_generate.push(ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?);
                }
                2 => {
                    self.parameter = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                15 => {
                    self.proto_file.push(ldd.deserialize_as_message(
                        <super::super::super::google::protobuf::FileDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                3 => {
                    let msg = self.compiler_version.get_or_insert_with(<super::super::super::google::protobuf::compiler::Version as ::std::default::Default>::default);
                    self.compiler_version = Some(ldd.deserialize_as_message(msg)?);
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for CodeGeneratorRequest {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use ::puroro_serializer::deserializer::stream::Deserializer;
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}
impl ::puroro_serializer::serializer::Serializable for CodeGeneratorRequest {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for string in &self.file_to_generate {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_bytes_twice(2, self.parameter.bytes().map(|b| Ok(b)))?;
        for msg in &self.proto_file {
            serializer.serialize_message_twice::<super::super::super::google::protobuf::FileDescriptorProto>(15, msg)?;
        }
        if let Some(msg) = &self.compiler_version {
            serializer.serialize_message_twice::<super::super::super::google::protobuf::compiler::Version>(3, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for CodeGeneratorRequest {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)?;
    }
}
    pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
    suffix: String,
}
impl ::std::default::Default for Version {
    fn default() -> Self {
        Self {
            major: ::std::default::Default::default(),
            minor: ::std::default::Default::default(),
            patch: ::std::default::Default::default(),
            suffix: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for Version {
    type Target = Self;
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(self)
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => {
                    self.major = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    self.minor = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => {
                    self.patch = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.major = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                2 => {
                    self.minor = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                3 => {
                    self.patch = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                4 => {
                    self.suffix = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for Version {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use ::puroro_serializer::deserializer::stream::Deserializer;
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}
impl ::puroro_serializer::serializer::Serializable for Version {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        unimplemented!("Serializer for something else");unimplemented!("Serializer for something else");unimplemented!("Serializer for something else");serializer.serialize_bytes_twice(4, self.suffix.bytes().map(|b| Ok(b)))?;
        Ok(())
    }
}
impl ::puroro::Serializable for Version {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)?;
    }
}
    