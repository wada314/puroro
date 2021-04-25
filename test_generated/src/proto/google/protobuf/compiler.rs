#![allow(unused_variables)]
#![allow(unused_imports)]

#[derive(Debug, Clone)]
pub struct CodeGeneratorResponse {
    pub error: ::std::string::String,
    pub supported_features: u64,
    pub file: ::std::vec::Vec<code_generator_response::File>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl CodeGeneratorResponse {
    pub fn new() -> Self {
        Self {
            error: ::puroro::helpers::FieldNew::new(),
            supported_features: ::puroro::helpers::FieldNew::new(),
            file: ::puroro::helpers::FieldNew::new(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for CodeGeneratorResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro::deser::DeserializableMessageFromBytesIter for CodeGeneratorResponse {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self
                        .supported_features
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::UInt64>()?;
                }
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                1 => {
                    *self.error.push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = bytes_iter
                        .variants()
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
                    let msg = self.file.push_and_get_mut(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro::deser::DeserializableFromBytes for CodeGeneratorResponse {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
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
    fn error(&self) -> &'_ str {
        self.error.as_ref()
    }
    fn supported_features(&self) -> u64 {
        self.supported_features.clone()
    }
    type FileType = code_generator_response::File;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ code_generator_response::File),
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ code_generator_response::File>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item = &'a code_generator_response::File>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
impl<'a> ::puroro::helpers::FieldNew<'a> for CodeGeneratorResponse {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct CodeGeneratorResponseBumpalo<'bump> {
    pub error: ::bumpalo::collections::String<'bump>,
    pub supported_features: u64,
    pub file: ::bumpalo::collections::Vec<'bump, code_generator_response::FileBumpalo<'bump>>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorResponseBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            error: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            supported_features: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            file: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::deser::DeserializableMessageFromBytesIter
    for CodeGeneratorResponseBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self
                        .supported_features
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::UInt64>()?;
                }
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                1 => {
                    *self.error.push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = bytes_iter
                        .variants()
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
                    let msg = self.file.push_and_get_mut(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::deser::DeserializableFromBytes for CodeGeneratorResponseBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::serializer::Serializable for CodeGeneratorResponseBumpalo<'bump> {
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
impl<'bump> ::puroro::Serializable for CodeGeneratorResponseBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorResponseTrait for CodeGeneratorResponseBumpalo<'bump> {
    fn error(&self) -> &'_ str {
        self.error.as_ref()
    }
    fn supported_features(&self) -> u64 {
        self.supported_features.clone()
    }
    type FileType = code_generator_response::FileBumpalo<'bump>;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ code_generator_response::File),
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ code_generator_response::File>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item = &'a code_generator_response::File>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::helpers::FieldNew<'bump> for CodeGeneratorResponseBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait CodeGeneratorResponseTrait {
    fn error(&'_ self) -> &'_ str;
    fn supported_features(&'_ self) -> u64;
    type FileType: code_generator_response::FileTrait;
    fn for_each_file<F>(&self, f: F)
    where
        F: FnMut(&'_ code_generator_response::File);
    fn file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ code_generator_response::File>>;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a>: Iterator<Item = &'a code_generator_response::File>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_>;
}
pub trait CodeGeneratorResponseMutTrait {
    fn error_mut(&self) -> &mut String;
    fn supported_features_mut(&self) -> &mut u64;
    fn for_each_file_mut<F>(&self, f: F)
    where
        F: FnMut(
            &mut super::super::super::google::protobuf::compiler::code_generator_response::File,
        );
    fn file_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::super::google::protobuf::compiler::code_generator_response::File>>;
    // We need more!
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
        pub generated_code_info:
            ::std::option::Option<::std::boxed::Box<super::super::GeneratedCodeInfo>>,
        puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
    }

    impl File {
        pub fn new() -> Self {
            Self {
                name: ::puroro::helpers::FieldNew::new(),
                insertion_point: ::puroro::helpers::FieldNew::new(),
                content: ::puroro::helpers::FieldNew::new(),
                generated_code_info: ::puroro::helpers::FieldNew::new(),
                puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
            }
        }
    }

    impl ::std::default::Default for File {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::puroro::deser::DeserializableMessageFromBytesIter for File {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::helpers::MaybeRepeatedField;
            use ::puroro::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro::types::FieldData::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.insertion_point.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    15 => {
                        *self.content.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let msg = self
                            .generated_code_info
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro::deser::DeserializableFromBytes for File {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
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
        fn name(&self) -> &'_ str {
            self.name.as_ref()
        }
        fn insertion_point(&self) -> &'_ str {
            self.insertion_point.as_ref()
        }
        fn content(&self) -> &'_ str {
            self.content.as_ref()
        }
        type GeneratedCodeInfoType = super::super::GeneratedCodeInfo;
        fn generated_code_info(
            &self,
        ) -> ::std::option::Option<&'_ super::super::GeneratedCodeInfo> {
            self.generated_code_info.as_deref()
        }
    }
    impl<'a> ::puroro::helpers::FieldNew<'a> for File {
        fn new() -> Self {
            Default::default()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct FileBumpalo<'bump> {
        pub name: ::bumpalo::collections::String<'bump>,
        pub insertion_point: ::bumpalo::collections::String<'bump>,
        pub content: ::bumpalo::collections::String<'bump>,
        pub generated_code_info: ::std::option::Option<
            ::bumpalo::boxed::Box<'bump, super::super::GeneratedCodeInfoBumpalo<'bump>>,
        >,
        puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'bump>,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> FileBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
            Self {
                name: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
                insertion_point: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
                content: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
                generated_code_info: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
                puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
            }
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::deser::DeserializableMessageFromBytesIter for FileBumpalo<'bump> {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro::helpers::MaybeRepeatedField;
            use ::puroro::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro::types::FieldData::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.insertion_point.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    15 => {
                        *self.content.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let msg = self
                            .generated_code_info
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 15 | 16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::deser::DeserializableFromBytes for FileBumpalo<'bump> {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::serializer::Serializable for FileBumpalo<'bump> {
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
    impl<'bump> ::puroro::Serializable for FileBumpalo<'bump> {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro::serializer::default_serializer(write);
            <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> FileTrait for FileBumpalo<'bump> {
        fn name(&self) -> &'_ str {
            self.name.as_ref()
        }
        fn insertion_point(&self) -> &'_ str {
            self.insertion_point.as_ref()
        }
        fn content(&self) -> &'_ str {
            self.content.as_ref()
        }
        type GeneratedCodeInfoType = super::super::GeneratedCodeInfoBumpalo<'bump>;
        fn generated_code_info(
            &self,
        ) -> ::std::option::Option<&'_ super::super::GeneratedCodeInfo> {
            self.generated_code_info.as_deref()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::helpers::FieldNew<'bump> for FileBumpalo<'bump> {
        fn new() -> Self {
            unimplemented!()
        }
        fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }
    pub trait FileTrait {
        fn name(&'_ self) -> &'_ str;
        fn insertion_point(&'_ self) -> &'_ str;
        fn content(&'_ self) -> &'_ str;
        type GeneratedCodeInfoType: super::super::GeneratedCodeInfoTrait;
        fn generated_code_info(
            &'_ self,
        ) -> ::std::option::Option<&'_ super::super::GeneratedCodeInfo>;
    }
    pub trait FileMutTrait {
        fn name_mut(&self) -> &mut String;
        fn insertion_point_mut(&self) -> &mut String;
        fn content_mut(&self) -> &mut String;
        fn generated_code_info_mut(
            &self,
        ) -> ::std::option::Option<
            &mut super::super::super::super::google::protobuf::GeneratedCodeInfo,
        >;
    }
} // mod code_generator_response

#[derive(Debug, Clone)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: ::std::vec::Vec<::std::string::String>,
    pub parameter: ::std::string::String,
    pub proto_file: ::std::vec::Vec<super::FileDescriptorProto>,
    pub compiler_version: ::std::option::Option<::std::boxed::Box<Version>>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl CodeGeneratorRequest {
    pub fn new() -> Self {
        Self {
            file_to_generate: ::puroro::helpers::FieldNew::new(),
            parameter: ::puroro::helpers::FieldNew::new(),
            proto_file: ::puroro::helpers::FieldNew::new(),
            compiler_version: ::puroro::helpers::FieldNew::new(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for CodeGeneratorRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro::deser::DeserializableMessageFromBytesIter for CodeGeneratorRequest {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                1 => {
                    *self
                        .file_to_generate
                        .push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.parameter.push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                15 => {
                    let msg = self.proto_file.push_and_get_mut(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self
                        .compiler_version
                        .push_and_get_mut(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro::deser::DeserializableFromBytes for CodeGeneratorRequest {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
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
        F: FnMut(&'_ str),
    {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_> {
        self.file_to_generate.iter().map(|v| v.as_ref())
    }
    fn parameter(&self) -> &'_ str {
        self.parameter.as_ref()
    }
    type ProtoFileType = super::FileDescriptorProto;
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ super::FileDescriptorProto),
    {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ super::FileDescriptorProto>> {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a> = impl Iterator<Item = &'a super::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_> {
        self.proto_file.iter()
    }
    type CompilerVersionType = Version;
    fn compiler_version(&self) -> ::std::option::Option<&'_ Version> {
        self.compiler_version.as_deref()
    }
}
impl<'a> ::puroro::helpers::FieldNew<'a> for CodeGeneratorRequest {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct CodeGeneratorRequestBumpalo<'bump> {
    pub file_to_generate: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    pub parameter: ::bumpalo::collections::String<'bump>,
    pub proto_file: ::bumpalo::collections::Vec<'bump, super::FileDescriptorProtoBumpalo<'bump>>,
    pub compiler_version:
        ::std::option::Option<::bumpalo::boxed::Box<'bump, VersionBumpalo<'bump>>>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorRequestBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            file_to_generate: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            parameter: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            proto_file: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            compiler_version: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::deser::DeserializableMessageFromBytesIter
    for CodeGeneratorRequestBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                15 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                1 => {
                    *self
                        .file_to_generate
                        .push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.parameter.push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                15 => {
                    let msg = self.proto_file.push_and_get_mut(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self
                        .compiler_version
                        .push_and_get_mut(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 15 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::deser::DeserializableFromBytes for CodeGeneratorRequestBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::serializer::Serializable for CodeGeneratorRequestBumpalo<'bump> {
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
impl<'bump> ::puroro::Serializable for CodeGeneratorRequestBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> CodeGeneratorRequestTrait for CodeGeneratorRequestBumpalo<'bump> {
    fn for_each_file_to_generate<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str),
    {
        for item in (self.file_to_generate).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn file_to_generate_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.file_to_generate.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_> {
        self.file_to_generate.iter().map(|v| v.as_ref())
    }
    fn parameter(&self) -> &'_ str {
        self.parameter.as_ref()
    }
    type ProtoFileType = super::FileDescriptorProtoBumpalo<'bump>;
    fn for_each_proto_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ super::FileDescriptorProto),
    {
        for item in (self.proto_file).iter() {
            (f)(item);
        }
    }
    fn proto_file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ super::FileDescriptorProto>> {
        ::std::boxed::Box::new(self.proto_file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a> = impl Iterator<Item = &'a super::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_> {
        self.proto_file.iter()
    }
    type CompilerVersionType = VersionBumpalo<'bump>;
    fn compiler_version(&self) -> ::std::option::Option<&'_ Version> {
        self.compiler_version.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::helpers::FieldNew<'bump> for CodeGeneratorRequestBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait CodeGeneratorRequestTrait {
    fn for_each_file_to_generate<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn file_to_generate_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    type FileToGenerateIter<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn file_to_generate_iter(&self) -> Self::FileToGenerateIter<'_>;
    fn parameter(&'_ self) -> &'_ str;
    type ProtoFileType: super::FileDescriptorProtoTrait;
    fn for_each_proto_file<F>(&self, f: F)
    where
        F: FnMut(&'_ super::FileDescriptorProto);
    fn proto_file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ super::FileDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ProtoFileIter<'a>: Iterator<Item = &'a super::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn proto_file_iter(&self) -> Self::ProtoFileIter<'_>;
    type CompilerVersionType: VersionTrait;
    fn compiler_version(&'_ self) -> ::std::option::Option<&'_ Version>;
}
pub trait CodeGeneratorRequestMutTrait {
    fn for_each_file_to_generate_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn file_to_generate_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut String>>;
    // We need more!
    fn parameter_mut(&self) -> &mut String;
    fn for_each_proto_file_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::super::google::protobuf::FileDescriptorProto);
    fn proto_file_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::super::google::protobuf::FileDescriptorProto>,
    >;
    // We need more!
    fn compiler_version_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::super::google::protobuf::compiler::Version>;
}

#[derive(Debug, Clone)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub suffix: ::std::string::String,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: ::puroro::helpers::FieldNew::new(),
            minor: ::puroro::helpers::FieldNew::new(),
            patch: ::puroro::helpers::FieldNew::new(),
            suffix: ::puroro::helpers::FieldNew::new(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro::deser::DeserializableMessageFromBytesIter for Version {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.major.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.minor.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => {
                    *self.patch.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter
                        .variants()
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
                    let values = bytes_iter
                        .variants()
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
                    let values = bytes_iter
                        .variants()
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
                    *self.suffix.push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro::deser::DeserializableFromBytes for Version {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
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
    fn suffix(&self) -> &'_ str {
        self.suffix.as_ref()
    }
}
impl<'a> ::puroro::helpers::FieldNew<'a> for Version {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct VersionBumpalo<'bump> {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub suffix: ::bumpalo::collections::String<'bump>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> VersionBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            major: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            minor: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            patch: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            suffix: ::puroro::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::deser::DeserializableMessageFromBytesIter for VersionBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro::types::FieldData<&'a mut ::puroro::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.major.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.minor.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => {
                    *self.patch.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::LengthDelimited(bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter
                        .variants()
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
                    let values = bytes_iter
                        .variants()
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
                    let values = bytes_iter
                        .variants()
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
                    *self.suffix.push_and_get_mut(&self.puroro_internal) =
                        bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::deser::DeserializableFromBytes for VersionBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::serializer::Serializable for VersionBumpalo<'bump> {
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
impl<'bump> ::puroro::Serializable for VersionBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> VersionTrait for VersionBumpalo<'bump> {
    fn major(&self) -> i32 {
        self.major.clone()
    }
    fn minor(&self) -> i32 {
        self.minor.clone()
    }
    fn patch(&self) -> i32 {
        self.patch.clone()
    }
    fn suffix(&self) -> &'_ str {
        self.suffix.as_ref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::helpers::FieldNew<'bump> for VersionBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait VersionTrait {
    fn major(&'_ self) -> i32;
    fn minor(&'_ self) -> i32;
    fn patch(&'_ self) -> i32;
    fn suffix(&'_ self) -> &'_ str;
}
pub trait VersionMutTrait {
    fn major_mut(&self) -> &mut i32;
    fn minor_mut(&self) -> &mut i32;
    fn patch_mut(&self) -> &mut i32;
    fn suffix_mut(&self) -> &mut String;
}
