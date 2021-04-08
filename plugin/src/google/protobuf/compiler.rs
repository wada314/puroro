pub struct CodeGeneratorResponse {
    error: String,
    supported_features: u64,
    file: ::std::vec::Vec<super::super::super::google::protobuf::compiler::code_generator_response::File>,
}
impl ::std::default::Default for CodeGeneratorResponse {
    fn default() -> Self {
        Self {
            error: std::default::Default::default(),
            supported_features: std::default::Default::default(),
            file: std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for CodeGeneratorResponse {
    type Target = Self;
    fn finish(self) -> ::puroro::Result<Self::Target> { Ok(self) }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                2 => {
                    self.supported_features = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedWireType)?
            }
        }
        Ok(())
    }
}
mod code_generator_response {
    pub enum Feature {
        FeatureNone = 0,
        FeatureProto3Optional = 1,
    }
    impl std::convert::TryFrom<i32> for Feature {
        type Error = i32; fn try_from(val: i32) -> std::result::Result<Self, i32> {
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
        generated_code_info: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::GeneratedCodeInfo>>,
    }
    impl ::std::default::Default for File {
        fn default() -> Self {
            Self {
                name: std::default::Default::default(),
                insertion_point: std::default::Default::default(),
                content: std::default::Default::default(),
                generated_code_info: std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for File {
        type Target = Self;
        fn finish(self) -> ::puroro::Result<Self::Target> { Ok(self) }
        fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro_serializer::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            match field {
                ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                    _ => Err(::puroro::PuroroError::UnexpectedWireType)?
                }
            }
            Ok(())
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
            file_to_generate: std::default::Default::default(),
            parameter: std::default::Default::default(),
            proto_file: std::default::Default::default(),
            compiler_version: std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for CodeGeneratorRequest {
    type Target = Self;
    fn finish(self) -> ::puroro::Result<Self::Target> { Ok(self) }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                _ => Err(::puroro::PuroroError::UnexpectedWireType)?
            }
        }
        Ok(())
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
            major: std::default::Default::default(),
            minor: std::default::Default::default(),
            patch: std::default::Default::default(),
            suffix: std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for Version {
    type Target = Self;
    fn finish(self) -> ::puroro::Result<Self::Target> { Ok(self) }
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
                _ => Err(::puroro::PuroroError::UnexpectedWireType)?
            }
        }
        Ok(())
    }
}
