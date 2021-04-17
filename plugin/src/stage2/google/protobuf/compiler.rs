#![allow(unused_variables)]
#![allow(unused_imports)]

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
        use ::std::convert::TryInto;
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
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.supported_features.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::UInt64>()?;
                }
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.error.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.supported_features, first, iter);
                }
                15 => {
                    let msg = self.file.push_and_get_mut();
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for CodeGeneratorResponse {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> ::puroro::Result<()> {
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for CodeGeneratorResponse {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.error.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::UInt64, _>(
            2,
            self.supported_features
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in self.file.iter_for_ser() {
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

impl CodeGeneratorResponseTrait for CodeGeneratorResponse {
    fn error(&self) -> &str {
        self.error.as_ref()
    }
    fn supported_features(&self) -> u64 {
        self.supported_features.clone()
    }
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::compiler::code_generator_response::File),
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::super::google::protobuf::compiler::code_generator_response::File>>{
        ::std::boxed::Box::new(self.file.iter())
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct CodeGeneratorResponseBumpalo {
    pub error: ::bumpalo::collections::String,
    pub supported_features: u64,
    pub file: ::bumpalo::collections::Vec<
        super::super::super::google::protobuf::compiler::code_generator_response::File,
    >,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut CodeGeneratorResponseBumpalo
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
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.supported_features.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::UInt64>()?;
                }
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.error.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.supported_features, first, iter);
                }
                15 => {
                    let msg = self.file.push_and_get_mut();
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::Deserializable for CodeGeneratorResponseBumpalo {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> ::puroro::Result<()> {
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::serializer::Serializable for CodeGeneratorResponseBumpalo {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.error.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::UInt64, _>(
            2,
            self.supported_features
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in self.file.iter_for_ser() {
            serializer.serialize_message_twice(15, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::Serializable for CodeGeneratorResponseBumpalo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl CodeGeneratorResponseTrait for CodeGeneratorResponseBumpalo {
    fn error(&self) -> &str {
        self.error.as_ref()
    }
    fn supported_features(&self) -> u64 {
        self.supported_features.clone()
    }
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::compiler::code_generator_response::File),
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
    -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::super::google::protobuf::compiler::code_generator_response::File>>{
        ::std::boxed::Box::new(self.file.iter())
    }
}
pub trait CodeGeneratorResponseTrait {
    fn error(&self) -> &str;
    fn supported_features(&self) -> u64;
    fn for_each_file<F>(&self, f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::compiler::code_generator_response::File);
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::super::google::protobuf::compiler::code_generator_response::File>>;
}
pub trait CodeGeneratorResponseMutTrait {
    fn error_mut(&self) -> &str;
    fn supported_features_mut(&self) -> u64;
    fn for_each_file_mut<F>(&self, f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::compiler::code_generator_response::File);
    fn file_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::super::google::protobuf::compiler::code_generator_response::File>>;
    // We need more! Maybe just expose &mut Vec<T> ?
}
pub mod code_generator_response {
    #[derive(Debug, Clone)]
    pub enum Feature {
        FeatureNone = 0,
        FeatureProto3Optional = 1,
    }
    impl ::std::convert::TryFrom<i32> for Feature {
        type Error = i32;
        fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::FeatureNone),
                1 => Ok(Self::FeatureProto3Optional),
                x => Err(x),
            }
        }
    }
    impl ::std::convert::Into<i32> for Feature {
        fn into(self) -> i32 {
            self as i32
        }
    }

    #[derive(Debug, Clone)]
    pub struct File {
        pub name: ::std::string::String,
        pub insertion_point: ::std::string::String,
        pub content: ::std::string::String,
        pub generated_code_info: ::std::option::Option<
            ::std::boxed::Box<super::super::super::super::google::protobuf::GeneratedCodeInfo>,
        >,
    }

    impl ::std::default::Default for File {
        fn default() -> Self {
            use ::std::convert::TryInto;
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
            use ::puroro::helpers::MaybeRepeatedField;
            use ::puroro::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        *self.name.push_and_get_mut() = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.insertion_point.push_and_get_mut() =
                            ldd.deserialize_as_chars()
                                .collect::<::puroro::Result<_>>()?;
                    }
                    15 => {
                        *self.content.push_and_get_mut() = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let msg = self.generated_code_info.push_and_get_mut();
                        ldd.deserialize_as_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro::Deserializable for File {
        fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self,
            iter: I,
        ) -> ::puroro::Result<()> {
            use ::puroro::deserializer::stream::Deserializer;
            let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            deserializer.deserialize(self)?;
            Ok(())
        }
    }

    impl ::puroro::serializer::Serializable for File {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro::helpers::MaybeRepeatedField;
            for string in self.name.iter_for_ser() {
                serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
            }
            for string in self.insertion_point.iter_for_ser() {
                serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
            }
            for string in self.content.iter_for_ser() {
                serializer.serialize_bytes_twice(15, string.bytes().map(|b| Ok(b)))?;
            }
            for msg in self.generated_code_info.iter_for_ser() {
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

    impl FileTrait for File {
        fn name(&self) -> &str {
            self.name.as_ref()
        }
        fn insertion_point(&self) -> &str {
            self.insertion_point.as_ref()
        }
        fn content(&self) -> &str {
            self.content.as_ref()
        }
        fn generated_code_info(
            &self,
        ) -> ::std::option::Option<&super::super::super::super::google::protobuf::GeneratedCodeInfo>
        {
            self.generated_code_info.as_deref()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct FileBumpalo {
        pub name: ::bumpalo::collections::String,
        pub insertion_point: ::bumpalo::collections::String,
        pub content: ::bumpalo::collections::String,
        pub generated_code_info: ::std::option::Option<
            ::bumpalo::boxed::Box<super::super::super::super::google::protobuf::GeneratedCodeInfo>,
        >,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut FileBumpalo {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
        }
        fn met_field<T: ::puroro::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            use ::puroro::helpers::MaybeRepeatedField;
            use ::puroro::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        *self.name.push_and_get_mut() = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.insertion_point.push_and_get_mut() =
                            ldd.deserialize_as_chars()
                                .collect::<::puroro::Result<_>>()?;
                    }
                    15 => {
                        *self.content.push_and_get_mut() = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let msg = self.generated_code_info.push_and_get_mut();
                        ldd.deserialize_as_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl ::puroro::Deserializable for FileBumpalo {
        fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self,
            iter: I,
        ) -> ::puroro::Result<()> {
            use ::puroro::deserializer::stream::Deserializer;
            let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            deserializer.deserialize(self)?;
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl ::puroro::serializer::Serializable for FileBumpalo {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro::helpers::MaybeRepeatedField;
            for string in self.name.iter_for_ser() {
                serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
            }
            for string in self.insertion_point.iter_for_ser() {
                serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
            }
            for string in self.content.iter_for_ser() {
                serializer.serialize_bytes_twice(15, string.bytes().map(|b| Ok(b)))?;
            }
            for msg in self.generated_code_info.iter_for_ser() {
                serializer.serialize_message_twice(16, msg)?;
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl ::puroro::Serializable for FileBumpalo {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro::serializer::default_serializer(write);
            <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl FileTrait for FileBumpalo {
        fn name(&self) -> &str {
            self.name.as_ref()
        }
        fn insertion_point(&self) -> &str {
            self.insertion_point.as_ref()
        }
        fn content(&self) -> &str {
            self.content.as_ref()
        }
        fn generated_code_info(
            &self,
        ) -> ::std::option::Option<&super::super::super::super::google::protobuf::GeneratedCodeInfo>
        {
            self.generated_code_info.as_deref()
        }
    }
    pub trait FileTrait {
        fn name(&self) -> &str;
        fn insertion_point(&self) -> &str;
        fn content(&self) -> &str;
        fn generated_code_info(
            &self,
        ) -> ::std::option::Option<&super::super::super::super::google::protobuf::GeneratedCodeInfo>;
    }
    pub trait FileMutTrait {
        fn name_mut(&self) -> &str;
        fn insertion_point_mut(&self) -> &str;
        fn content_mut(&self) -> &str;
        fn generated_code_info_mut(
            &self,
        ) -> ::std::option::Option<&super::super::super::super::google::protobuf::GeneratedCodeInfo>;
    }
} // mod code_generator_response

#[derive(Debug, Clone)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: ::std::vec::Vec<::std::string::String>,
    pub parameter: ::std::string::String,
    pub proto_file: ::std::vec::Vec<super::super::super::google::protobuf::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option<
        ::std::boxed::Box<super::super::super::google::protobuf::compiler::Version>,
    >,
}

impl ::std::default::Default for CodeGeneratorRequest {
    fn default() -> Self {
        use ::std::convert::TryInto;
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
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.file_to_generate.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.parameter.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                15 => {
                    let msg = self.proto_file.push_and_get_mut();
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.compiler_version.push_and_get_mut();
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for CodeGeneratorRequest {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> ::puroro::Result<()> {
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for CodeGeneratorRequest {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.file_to_generate.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.parameter.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.proto_file.iter_for_ser() {
            serializer.serialize_message_twice(15, msg)?;
        }
        for msg in self.compiler_version.iter_for_ser() {
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

impl CodeGeneratorRequestTrait for CodeGeneratorRequest {
    fn for_each_file_to_generate<F>(&self, mut f: F)
    where
        F: FnMut(&str),
    {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &str>> {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    fn parameter(&self) -> &str {
        self.parameter.as_ref()
    }
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::FileDescriptorProto),
    {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &super::super::super::google::protobuf::FileDescriptorProto>,
    > {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    fn compiler_version(
        &self,
    ) -> ::std::option::Option<&super::super::super::google::protobuf::compiler::Version> {
        self.compiler_version.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct CodeGeneratorRequestBumpalo {
    pub file_to_generate: ::bumpalo::collections::Vec<::bumpalo::collections::String>,
    pub parameter: ::bumpalo::collections::String,
    pub proto_file:
        ::bumpalo::collections::Vec<super::super::super::google::protobuf::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option<
        ::bumpalo::boxed::Box<super::super::super::google::protobuf::compiler::Version>,
    >,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut CodeGeneratorRequestBumpalo
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
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.file_to_generate.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.parameter.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                15 => {
                    let msg = self.proto_file.push_and_get_mut();
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.compiler_version.push_and_get_mut();
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::Deserializable for CodeGeneratorRequestBumpalo {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> ::puroro::Result<()> {
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::serializer::Serializable for CodeGeneratorRequestBumpalo {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.file_to_generate.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.parameter.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.proto_file.iter_for_ser() {
            serializer.serialize_message_twice(15, msg)?;
        }
        for msg in self.compiler_version.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::Serializable for CodeGeneratorRequestBumpalo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl CodeGeneratorRequestTrait for CodeGeneratorRequestBumpalo {
    fn for_each_file_to_generate<F>(&self, mut f: F)
    where
        F: FnMut(&str),
    {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &str>> {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    fn parameter(&self) -> &str {
        self.parameter.as_ref()
    }
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::FileDescriptorProto),
    {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &super::super::super::google::protobuf::FileDescriptorProto>,
    > {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    fn compiler_version(
        &self,
    ) -> ::std::option::Option<&super::super::super::google::protobuf::compiler::Version> {
        self.compiler_version.as_deref()
    }
}
pub trait CodeGeneratorRequestTrait {
    fn for_each_file_to_generate<F>(&self, f: F)
    where
        F: FnMut(&str);
    fn file_to_generate_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &str>>;
    fn parameter(&self) -> &str;
    fn for_each_proto_file<F>(&self, f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::FileDescriptorProto);
    fn proto_file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &super::super::super::google::protobuf::FileDescriptorProto>,
    >;
    fn compiler_version(
        &self,
    ) -> ::std::option::Option<&super::super::super::google::protobuf::compiler::Version>;
}
pub trait CodeGeneratorRequestMutTrait {
    fn for_each_file_to_generate_mut<F>(&self, f: F)
    where
        F: FnMut(&str);
    fn file_to_generate_boxed_iter_mut(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &str>>;
    // We need more! Maybe just expose &mut Vec<T> ?
    fn parameter_mut(&self) -> &str;
    fn for_each_proto_file_mut<F>(&self, f: F)
    where
        F: FnMut(&super::super::super::google::protobuf::FileDescriptorProto);
    fn proto_file_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &super::super::super::google::protobuf::FileDescriptorProto>,
    >;
    // We need more! Maybe just expose &mut Vec<T> ?
    fn compiler_version_mut(
        &self,
    ) -> ::std::option::Option<&super::super::super::google::protobuf::compiler::Version>;
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
        use ::std::convert::TryInto;
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
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => {
                    *self.major.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.minor.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => {
                    *self.patch.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
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
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
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
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.patch, first, iter);
                }
                4 => {
                    *self.suffix.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for Version {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> ::puroro::Result<()> {
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for Version {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1,
            self.major.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2,
            self.minor.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            3,
            self.patch.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for string in self.suffix.iter_for_ser() {
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

impl VersionTrait for Version {
    fn major(&self) -> i32 {
        self.major.clone()
    }
    fn minor(&self) -> i32 {
        self.minor.clone()
    }
    fn patch(&self) -> i32 {
        self.patch.clone()
    }
    fn suffix(&self) -> &str {
        self.suffix.as_ref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct VersionBumpalo {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub suffix: ::bumpalo::collections::String,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut VersionBumpalo {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => {
                    *self.major.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.minor.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => {
                    *self.patch.push_and_get_mut() =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
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
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
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
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                        })
                        .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.patch, first, iter);
                }
                4 => {
                    *self.suffix.push_and_get_mut() = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::Deserializable for VersionBumpalo {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> ::puroro::Result<()> {
        use ::puroro::deserializer::stream::Deserializer;
        let deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::serializer::Serializable for VersionBumpalo {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1,
            self.major.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2,
            self.minor.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            3,
            self.patch.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for string in self.suffix.iter_for_ser() {
            serializer.serialize_bytes_twice(4, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ::puroro::Serializable for VersionBumpalo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl VersionTrait for VersionBumpalo {
    fn major(&self) -> i32 {
        self.major.clone()
    }
    fn minor(&self) -> i32 {
        self.minor.clone()
    }
    fn patch(&self) -> i32 {
        self.patch.clone()
    }
    fn suffix(&self) -> &str {
        self.suffix.as_ref()
    }
}
pub trait VersionTrait {
    fn major(&self) -> i32;
    fn minor(&self) -> i32;
    fn patch(&self) -> i32;
    fn suffix(&self) -> &str;
}
pub trait VersionMutTrait {
    fn major_mut(&self) -> i32;
    fn minor_mut(&self) -> i32;
    fn patch_mut(&self) -> i32;
    fn suffix_mut(&self) -> &str;
}
