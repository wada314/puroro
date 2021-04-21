#![allow(unused_variables)]
#![allow(unused_imports)]


#[derive(Debug, Clone)]
pub struct GeneratedCodeInfo {
    pub annotation: ::std::vec::Vec<super::super::google::protobuf::generated_code_info::Annotation>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl GeneratedCodeInfo {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            annotation: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut GeneratedCodeInfo {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = self.annotation.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for GeneratedCodeInfo {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let msg = self.annotation.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for GeneratedCodeInfo {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for GeneratedCodeInfo {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.annotation.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for GeneratedCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl GeneratedCodeInfoTrait for GeneratedCodeInfo {
    fn for_each_annotation<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::generated_code_info::Annotation) {
        for item in (self.annotation).iter() {
            (f)(item);
        }
    }
    fn annotation_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::generated_code_info::Annotation>> {
        ::std::boxed::Box::new(self.annotation.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::generated_code_info::Annotation>;
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_> {
        self.annotation.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct GeneratedCodeInfoBumpalo<'b> {
    pub annotation: ::bumpalo::collections::Vec<super::super::google::protobuf::generated_code_info::Annotation>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> GeneratedCodeInfoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            annotation: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut GeneratedCodeInfoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = self.annotation.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for GeneratedCodeInfoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let msg = self.annotation.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for GeneratedCodeInfoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for GeneratedCodeInfoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.annotation.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for GeneratedCodeInfoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl GeneratedCodeInfoTrait for GeneratedCodeInfoBumpalo {
    fn for_each_annotation<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::generated_code_info::Annotation) {
        for item in (self.annotation).iter() {
            (f)(item);
        }
    }
    fn annotation_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::generated_code_info::Annotation>> {
        ::std::boxed::Box::new(self.annotation.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::generated_code_info::Annotation>;
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_> {
        self.annotation.iter()
    }
}
pub trait GeneratedCodeInfoTrait {
    fn for_each_annotation<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::generated_code_info::Annotation);
    fn annotation_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::generated_code_info::Annotation>>;
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a>: Iterator<Item=&'a super::super::google::protobuf::generated_code_info::Annotation>;
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_>;
}
pub trait GeneratedCodeInfoMutTrait {
    fn for_each_annotation_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::generated_code_info::Annotation);
    fn annotation_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::generated_code_info::Annotation>>;
    // We need more! 
}
pub mod generated_code_info {

#[derive(Debug, Clone)]
pub struct Annotation {
    pub path: ::std::vec::Vec<i32>,
    pub source_file: ::std::string::String,
    pub begin: i32,
    pub end: i32,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl Annotation {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            path: ::std::default::Default::default(),
            source_file: ::std::default::Default::default(),
            begin: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for Annotation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut Annotation {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.begin.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    *self.source_file.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.begin, first, iter);
                }
                4 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for Annotation {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.begin.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    *self.source_file.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.begin, first, iter);
                }
                4 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for Annotation {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for Annotation {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.path.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.source_file.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            3, 
            self.begin.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            4, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}

impl ::puroro::Serializable for Annotation {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl AnnotationTrait for Annotation {
    fn for_each_path<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.path).iter().cloned() {
            (f)(item);
        }
    }
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.path.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_> {
        self.path.iter().cloned()
    }
    fn source_file(&self) -> &str {
        self.source_file.as_ref()
    }
    fn begin(&self) -> i32 {
        self.begin.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct AnnotationBumpalo<'b> {
    pub path: ::bumpalo::collections::Vec<i32>,
    pub source_file: ::bumpalo::collections::String,
    pub begin: i32,
    pub end: i32,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> AnnotationBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            path: ::std::default::Default::default(),
            source_file: ::std::default::Default::default(),
            begin: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut AnnotationBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.begin.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    *self.source_file.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.begin, first, iter);
                }
                4 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for AnnotationBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.begin.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    *self.source_file.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.begin, first, iter);
                }
                4 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for AnnotationBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for AnnotationBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.path.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.source_file.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            3, 
            self.begin.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            4, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for AnnotationBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl AnnotationTrait for AnnotationBumpalo {
    fn for_each_path<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.path).iter().cloned() {
            (f)(item);
        }
    }
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.path.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_> {
        self.path.iter().cloned()
    }
    fn source_file(&self) -> &str {
        self.source_file.as_ref()
    }
    fn begin(&self) -> i32 {
        self.begin.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
}
pub trait AnnotationTrait {
    fn for_each_path<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a>: Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_>;
    fn source_file(&self) -> &str;
    fn begin(&self) -> i32;
    fn end(&self) -> i32;
}
pub trait AnnotationMutTrait {
    fn for_each_path_mut<F>(&self, f: F)
    where
        F: FnMut(&mut i32);
    fn path_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut i32>>;
    // We need more! 
    fn source_file_mut(&self) -> &mut String;
    fn begin_mut(&self) -> &mut i32;
    fn end_mut(&self) -> &mut i32;
}
} // mod generated_code_info

#[derive(Debug, Clone)]
pub struct SourceCodeInfo {
    pub location: ::std::vec::Vec<super::super::google::protobuf::source_code_info::Location>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl SourceCodeInfo {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            location: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut SourceCodeInfo {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = self.location.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for SourceCodeInfo {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let msg = self.location.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for SourceCodeInfo {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for SourceCodeInfo {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.location.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for SourceCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl SourceCodeInfoTrait for SourceCodeInfo {
    fn for_each_location<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::source_code_info::Location) {
        for item in (self.location).iter() {
            (f)(item);
        }
    }
    fn location_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::source_code_info::Location>> {
        ::std::boxed::Box::new(self.location.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::source_code_info::Location>;
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_> {
        self.location.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct SourceCodeInfoBumpalo<'b> {
    pub location: ::bumpalo::collections::Vec<super::super::google::protobuf::source_code_info::Location>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> SourceCodeInfoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            location: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut SourceCodeInfoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = self.location.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for SourceCodeInfoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let msg = self.location.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for SourceCodeInfoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for SourceCodeInfoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.location.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for SourceCodeInfoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl SourceCodeInfoTrait for SourceCodeInfoBumpalo {
    fn for_each_location<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::source_code_info::Location) {
        for item in (self.location).iter() {
            (f)(item);
        }
    }
    fn location_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::source_code_info::Location>> {
        ::std::boxed::Box::new(self.location.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::source_code_info::Location>;
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_> {
        self.location.iter()
    }
}
pub trait SourceCodeInfoTrait {
    fn for_each_location<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::source_code_info::Location);
    fn location_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::source_code_info::Location>>;
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a>: Iterator<Item=&'a super::super::google::protobuf::source_code_info::Location>;
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_>;
}
pub trait SourceCodeInfoMutTrait {
    fn for_each_location_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::source_code_info::Location);
    fn location_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::source_code_info::Location>>;
    // We need more! 
}
pub mod source_code_info {

#[derive(Debug, Clone)]
pub struct Location {
    pub path: ::std::vec::Vec<i32>,
    pub span: ::std::vec::Vec<i32>,
    pub leading_comments: ::std::string::String,
    pub trailing_comments: ::std::string::String,
    pub leading_detached_comments: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl Location {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            path: ::std::default::Default::default(),
            span: ::std::default::Default::default(),
            leading_comments: ::std::default::Default::default(),
            trailing_comments: ::std::default::Default::default(),
            leading_detached_comments: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut Location {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.span.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.span, first, iter);
                }
                3 => {
                    *self.leading_comments.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    *self.trailing_comments.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                6 => {
                    *self.leading_detached_comments.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for Location {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.span.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.span, first, iter);
                }
                3 => {
                    *self.leading_comments.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    *self.trailing_comments.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                6 => {
                    *self.leading_detached_comments.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for Location {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for Location {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.path.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.span.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.leading_comments.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.trailing_comments.iter_for_ser() {
            serializer.serialize_bytes_twice(4, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.leading_detached_comments.iter_for_ser() {
            serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for Location {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl LocationTrait for Location {
    fn for_each_path<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.path).iter().cloned() {
            (f)(item);
        }
    }
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.path.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_> {
        self.path.iter().cloned()
    }
    fn for_each_span<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.span).iter().cloned() {
            (f)(item);
        }
    }
    fn span_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.span.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type SpanIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn span_iter(&self) -> Self::SpanIter<'_> {
        self.span.iter().cloned()
    }
    fn leading_comments(&self) -> &str {
        self.leading_comments.as_ref()
    }
    fn trailing_comments(&self) -> &str {
        self.trailing_comments.as_ref()
    }
    fn for_each_leading_detached_comments<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.leading_detached_comments).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn leading_detached_comments_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.leading_detached_comments.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type LeadingDetachedCommentsIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_> {
        self.leading_detached_comments.iter().map(|v| v.as_ref())
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct LocationBumpalo<'b> {
    pub path: ::bumpalo::collections::Vec<i32>,
    pub span: ::bumpalo::collections::Vec<i32>,
    pub leading_comments: ::bumpalo::collections::String,
    pub trailing_comments: ::bumpalo::collections::String,
    pub leading_detached_comments: ::bumpalo::collections::Vec<::bumpalo::collections::String>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> LocationBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            path: ::std::default::Default::default(),
            span: ::std::default::Default::default(),
            leading_comments: ::std::default::Default::default(),
            trailing_comments: ::std::default::Default::default(),
            leading_detached_comments: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut LocationBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.span.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.span, first, iter);
                }
                3 => {
                    *self.leading_comments.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    *self.trailing_comments.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                6 => {
                    *self.leading_detached_comments.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for LocationBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.path.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.span.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.span, first, iter);
                }
                3 => {
                    *self.leading_comments.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    *self.trailing_comments.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                6 => {
                    *self.leading_detached_comments.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for LocationBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for LocationBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.path.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.span.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.leading_comments.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.trailing_comments.iter_for_ser() {
            serializer.serialize_bytes_twice(4, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.leading_detached_comments.iter_for_ser() {
            serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for LocationBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl LocationTrait for LocationBumpalo {
    fn for_each_path<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.path).iter().cloned() {
            (f)(item);
        }
    }
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.path.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_> {
        self.path.iter().cloned()
    }
    fn for_each_span<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.span).iter().cloned() {
            (f)(item);
        }
    }
    fn span_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.span.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type SpanIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn span_iter(&self) -> Self::SpanIter<'_> {
        self.span.iter().cloned()
    }
    fn leading_comments(&self) -> &str {
        self.leading_comments.as_ref()
    }
    fn trailing_comments(&self) -> &str {
        self.trailing_comments.as_ref()
    }
    fn for_each_leading_detached_comments<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.leading_detached_comments).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn leading_detached_comments_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.leading_detached_comments.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type LeadingDetachedCommentsIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_> {
        self.leading_detached_comments.iter().map(|v| v.as_ref())
    }
}
pub trait LocationTrait {
    fn for_each_path<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a>: Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_>;
    fn for_each_span<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn span_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    type SpanIter<'a>: Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn span_iter(&self) -> Self::SpanIter<'_>;
    fn leading_comments(&self) -> &str;
    fn trailing_comments(&self) -> &str;
    fn for_each_leading_detached_comments<F>(&self, f: F)
    where
        F: FnMut(&str);
    fn leading_detached_comments_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>>;
    #[cfg(feature = "puroro-nightly")]
    type LeadingDetachedCommentsIter<'a>: Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_>;
}
pub trait LocationMutTrait {
    fn for_each_path_mut<F>(&self, f: F)
    where
        F: FnMut(&mut i32);
    fn path_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut i32>>;
    // We need more! 
    fn for_each_span_mut<F>(&self, f: F)
    where
        F: FnMut(&mut i32);
    fn span_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut i32>>;
    // We need more! 
    fn leading_comments_mut(&self) -> &mut String;
    fn trailing_comments_mut(&self) -> &mut String;
    fn for_each_leading_detached_comments_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn leading_detached_comments_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut String>>;
    // We need more! 
}
} // mod source_code_info

#[derive(Debug, Clone)]
pub struct UninterpretedOption {
    pub name: ::std::vec::Vec<super::super::google::protobuf::uninterpreted_option::NamePart>,
    pub identifier_value: ::std::string::String,
    pub positive_int_value: u64,
    pub negative_int_value: i64,
    pub double_value: f64,
    pub string_value: ::std::vec::Vec<u8>,
    pub aggregate_value: ::std::string::String,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl UninterpretedOption {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            identifier_value: ::std::default::Default::default(),
            positive_int_value: ::std::default::Default::default(),
            negative_int_value: ::std::default::Default::default(),
            double_value: ::std::default::Default::default(),
            string_value: ::std::default::Default::default(),
            aggregate_value: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut UninterpretedOption {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    *self.positive_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                5 => {
                    *self.negative_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                2 => {
                    let msg = self.name.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    *self.identifier_value.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.positive_int_value, first, iter);
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.negative_int_value, first, iter);
                }
                6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                7 => {
                    *self.string_value.push_and_get_mut()
                        = ldd.deserialize_as_bytes().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.aggregate_value.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                6 => {
                    *self.double_value.push_and_get_mut() = f64::from_le_bytes(bytes);
                }
                2 | 3 | 4 | 5 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for UninterpretedOption {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    *self.positive_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                5 => {
                    *self.negative_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                2 => {
                    let msg = self.name.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    *self.identifier_value.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.positive_int_value, first, iter);
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.negative_int_value, first, iter);
                }
                6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                7 => {
                    *self.string_value.push_and_get_mut()
                        = bytes_iter.bytes().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.aggregate_value.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                6 => {
                    *self.double_value.push_and_get_mut() = f64::from_le_bytes(bytes);
                }
                2 | 3 | 4 | 5 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for UninterpretedOption {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for UninterpretedOption {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.name.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for string in self.identifier_value.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::UInt64, _>(
            4, 
            self.positive_int_value.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int64, _>(
            5, 
            self.negative_int_value.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for item in self.double_value.iter_for_ser() {
            serializer.serialize_fixed_bits(6, item.to_le_bytes())?;
        }
        for bytes in self.string_value.iter_for_ser() {
            serializer.serialize_bytes_twice(7, bytes.iter().map(|b| Ok(*b)))?;
        }
        for string in self.aggregate_value.iter_for_ser() {
            serializer.serialize_bytes_twice(8, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for UninterpretedOption {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl UninterpretedOptionTrait for UninterpretedOption {
    fn for_each_name<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::uninterpreted_option::NamePart) {
        for item in (self.name).iter() {
            (f)(item);
        }
    }
    fn name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::uninterpreted_option::NamePart>> {
        ::std::boxed::Box::new(self.name.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::uninterpreted_option::NamePart>;
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_> {
        self.name.iter()
    }
    fn identifier_value(&self) -> &str {
        self.identifier_value.as_ref()
    }
    fn positive_int_value(&self) -> u64 {
        self.positive_int_value.clone()
    }
    fn negative_int_value(&self) -> i64 {
        self.negative_int_value.clone()
    }
    fn double_value(&self) -> f64 {
        self.double_value.clone()
    }
    fn string_value(&self) -> &[u8] {
        self.string_value.as_ref()
    }
    fn aggregate_value(&self) -> &str {
        self.aggregate_value.as_ref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct UninterpretedOptionBumpalo<'b> {
    pub name: ::bumpalo::collections::Vec<super::super::google::protobuf::uninterpreted_option::NamePart>,
    pub identifier_value: ::bumpalo::collections::String,
    pub positive_int_value: u64,
    pub negative_int_value: i64,
    pub double_value: f64,
    pub string_value: ::bumpalo::collections::Vec<u8>,
    pub aggregate_value: ::bumpalo::collections::String,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> UninterpretedOptionBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            identifier_value: ::std::default::Default::default(),
            positive_int_value: ::std::default::Default::default(),
            negative_int_value: ::std::default::Default::default(),
            double_value: ::std::default::Default::default(),
            string_value: ::std::default::Default::default(),
            aggregate_value: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut UninterpretedOptionBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    *self.positive_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                5 => {
                    *self.negative_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                2 => {
                    let msg = self.name.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    *self.identifier_value.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.positive_int_value, first, iter);
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.negative_int_value, first, iter);
                }
                6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                7 => {
                    *self.string_value.push_and_get_mut()
                        = ldd.deserialize_as_bytes().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.aggregate_value.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                6 => {
                    *self.double_value.push_and_get_mut() = f64::from_le_bytes(bytes);
                }
                2 | 3 | 4 | 5 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for UninterpretedOptionBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    *self.positive_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                5 => {
                    *self.negative_int_value.push_and_get_mut() = variant.to_native::<::puroro::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                2 => {
                    let msg = self.name.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    *self.identifier_value.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.positive_int_value, first, iter);
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int64>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.negative_int_value, first, iter);
                }
                6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                7 => {
                    *self.string_value.push_and_get_mut()
                        = bytes_iter.bytes().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.aggregate_value.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                6 => {
                    *self.double_value.push_and_get_mut() = f64::from_le_bytes(bytes);
                }
                2 | 3 | 4 | 5 | 7 | 8 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for UninterpretedOptionBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for UninterpretedOptionBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.name.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for string in self.identifier_value.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::UInt64, _>(
            4, 
            self.positive_int_value.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int64, _>(
            5, 
            self.negative_int_value.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for item in self.double_value.iter_for_ser() {
            serializer.serialize_fixed_bits(6, item.to_le_bytes())?;
        }
        for bytes in self.string_value.iter_for_ser() {
            serializer.serialize_bytes_twice(7, bytes.iter().map(|b| Ok(*b)))?;
        }
        for string in self.aggregate_value.iter_for_ser() {
            serializer.serialize_bytes_twice(8, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for UninterpretedOptionBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl UninterpretedOptionTrait for UninterpretedOptionBumpalo {
    fn for_each_name<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::uninterpreted_option::NamePart) {
        for item in (self.name).iter() {
            (f)(item);
        }
    }
    fn name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::uninterpreted_option::NamePart>> {
        ::std::boxed::Box::new(self.name.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::uninterpreted_option::NamePart>;
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_> {
        self.name.iter()
    }
    fn identifier_value(&self) -> &str {
        self.identifier_value.as_ref()
    }
    fn positive_int_value(&self) -> u64 {
        self.positive_int_value.clone()
    }
    fn negative_int_value(&self) -> i64 {
        self.negative_int_value.clone()
    }
    fn double_value(&self) -> f64 {
        self.double_value.clone()
    }
    fn string_value(&self) -> &[u8] {
        self.string_value.as_ref()
    }
    fn aggregate_value(&self) -> &str {
        self.aggregate_value.as_ref()
    }
}
pub trait UninterpretedOptionTrait {
    fn for_each_name<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::uninterpreted_option::NamePart);
    fn name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::uninterpreted_option::NamePart>>;
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a>: Iterator<Item=&'a super::super::google::protobuf::uninterpreted_option::NamePart>;
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_>;
    fn identifier_value(&self) -> &str;
    fn positive_int_value(&self) -> u64;
    fn negative_int_value(&self) -> i64;
    fn double_value(&self) -> f64;
    fn string_value(&self) -> &[u8];
    fn aggregate_value(&self) -> &str;
}
pub trait UninterpretedOptionMutTrait {
    fn for_each_name_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::uninterpreted_option::NamePart);
    fn name_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::uninterpreted_option::NamePart>>;
    // We need more! 
    fn identifier_value_mut(&self) -> &mut String;
    fn positive_int_value_mut(&self) -> &mut u64;
    fn negative_int_value_mut(&self) -> &mut i64;
    fn double_value_mut(&self) -> &mut f64;
    fn string_value_mut(&self) -> &mut Vec<u8>;
    fn aggregate_value_mut(&self) -> &mut String;
}
pub mod uninterpreted_option {

#[derive(Debug, Clone)]
pub struct NamePart {
    pub name_part: ::std::string::String,
    pub is_extension: bool,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl NamePart {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name_part: ::std::default::Default::default(),
            is_extension: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for NamePart {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut NamePart {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.is_extension.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name_part.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.is_extension, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for NamePart {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.is_extension.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name_part.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.is_extension, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for NamePart {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for NamePart {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name_part.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.is_extension.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}

impl ::puroro::Serializable for NamePart {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl NamePartTrait for NamePart {
    fn name_part(&self) -> &str {
        self.name_part.as_ref()
    }
    fn is_extension(&self) -> bool {
        self.is_extension.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct NamePartBumpalo<'b> {
    pub name_part: ::bumpalo::collections::String,
    pub is_extension: bool,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> NamePartBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name_part: ::std::default::Default::default(),
            is_extension: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut NamePartBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.is_extension.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name_part.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.is_extension, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for NamePartBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.is_extension.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name_part.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.is_extension, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for NamePartBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for NamePartBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name_part.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.is_extension.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for NamePartBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl NamePartTrait for NamePartBumpalo {
    fn name_part(&self) -> &str {
        self.name_part.as_ref()
    }
    fn is_extension(&self) -> bool {
        self.is_extension.clone()
    }
}
pub trait NamePartTrait {
    fn name_part(&self) -> &str;
    fn is_extension(&self) -> bool;
}
pub trait NamePartMutTrait {
    fn name_part_mut(&self) -> &mut String;
    fn is_extension_mut(&self) -> &mut bool;
}
} // mod uninterpreted_option

#[derive(Debug, Clone)]
pub struct MethodOptions {
    pub deprecated: bool,
    pub idempotency_level: ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl MethodOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            idempotency_level: 0i32.try_into(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut MethodOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                34 => {
                    *self.idempotency_level.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                33 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                34 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.idempotency_level, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for MethodOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                34 => {
                    *self.idempotency_level.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                33 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                34 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.idempotency_level, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for MethodOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for MethodOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            33, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>, _>(
            34, 
            self.idempotency_level.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for MethodOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl MethodOptionsTrait for MethodOptions {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn idempotency_level(&self) -> ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32> {
        self.idempotency_level.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct MethodOptionsBumpalo<'b> {
    pub deprecated: bool,
    pub idempotency_level: ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> MethodOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            idempotency_level: 0i32.try_into(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut MethodOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                34 => {
                    *self.idempotency_level.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                33 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                34 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.idempotency_level, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for MethodOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                34 => {
                    *self.idempotency_level.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                33 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                34 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.idempotency_level, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 34 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for MethodOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for MethodOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            33, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>, _>(
            34, 
            self.idempotency_level.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for MethodOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl MethodOptionsTrait for MethodOptionsBumpalo {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn idempotency_level(&self) -> ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32> {
        self.idempotency_level.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait MethodOptionsTrait {
    fn deprecated(&self) -> bool;
    fn idempotency_level(&self) -> ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait MethodOptionsMutTrait {
    fn deprecated_mut(&self) -> &mut bool;
    fn idempotency_level_mut(&self) -> &mut ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}
pub mod method_options {
#[derive(Debug, Clone)]
pub enum IdempotencyLevel {
    IdempotencyUnknown = 0,
    NoSideEffects = 1,
    Idempotent = 2,
}
impl ::std::convert::TryFrom<i32> for IdempotencyLevel {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            0 => Ok(Self::IdempotencyUnknown),
            1 => Ok(Self::NoSideEffects),
            2 => Ok(Self::Idempotent),
            x => Err(x),
        }
    }
}
impl ::std::convert::Into<i32> for IdempotencyLevel {
    fn into(self) -> i32 {
        self as i32
    }
}
} // mod method_options

#[derive(Debug, Clone)]
pub struct ServiceOptions {
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl ServiceOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ServiceOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                33 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ServiceOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                33 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for ServiceOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for ServiceOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            33, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for ServiceOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ServiceOptionsTrait for ServiceOptions {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ServiceOptionsBumpalo<'b> {
    pub deprecated: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ServiceOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ServiceOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                33 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ServiceOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                33 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                33 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for ServiceOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for ServiceOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            33, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for ServiceOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ServiceOptionsTrait for ServiceOptionsBumpalo {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait ServiceOptionsTrait {
    fn deprecated(&self) -> bool;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait ServiceOptionsMutTrait {
    fn deprecated_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}

#[derive(Debug, Clone)]
pub struct EnumValueOptions {
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl EnumValueOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumValueOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumValueOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumValueOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for EnumValueOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for EnumValueOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            1, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumValueOptionsTrait for EnumValueOptions {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumValueOptionsBumpalo<'b> {
    pub deprecated: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> EnumValueOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumValueOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumValueOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for EnumValueOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for EnumValueOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            1, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for EnumValueOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl EnumValueOptionsTrait for EnumValueOptionsBumpalo {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait EnumValueOptionsTrait {
    fn deprecated(&self) -> bool;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait EnumValueOptionsMutTrait {
    fn deprecated_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}

#[derive(Debug, Clone)]
pub struct EnumOptions {
    pub allow_alias: bool,
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl EnumOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            allow_alias: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => {
                    *self.allow_alias.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.allow_alias, first, iter);
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => {
                    *self.allow_alias.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.allow_alias, first, iter);
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for EnumOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for EnumOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.allow_alias.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            3, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for EnumOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumOptionsTrait for EnumOptions {
    fn allow_alias(&self) -> bool {
        self.allow_alias.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumOptionsBumpalo<'b> {
    pub allow_alias: bool,
    pub deprecated: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> EnumOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            allow_alias: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => {
                    *self.allow_alias.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.allow_alias, first, iter);
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                2 => {
                    *self.allow_alias.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.allow_alias, first, iter);
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                2 | 3 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for EnumOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for EnumOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.allow_alias.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            3, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for EnumOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl EnumOptionsTrait for EnumOptionsBumpalo {
    fn allow_alias(&self) -> bool {
        self.allow_alias.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait EnumOptionsTrait {
    fn allow_alias(&self) -> bool;
    fn deprecated(&self) -> bool;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait EnumOptionsMutTrait {
    fn allow_alias_mut(&self) -> &mut bool;
    fn deprecated_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}

#[derive(Debug, Clone)]
pub struct OneofOptions {
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl OneofOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut OneofOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for OneofOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for OneofOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for OneofOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for OneofOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl OneofOptionsTrait for OneofOptions {
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct OneofOptionsBumpalo<'b> {
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> OneofOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut OneofOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for OneofOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for OneofOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for OneofOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for OneofOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl OneofOptionsTrait for OneofOptionsBumpalo {
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait OneofOptionsTrait {
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait OneofOptionsMutTrait {
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}

#[derive(Debug, Clone)]
pub struct FieldOptions {
    pub ctype: ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>,
    pub packed: bool,
    pub jstype: ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>,
    pub lazy: bool,
    pub deprecated: bool,
    pub weak: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl FieldOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            ctype: 0i32.try_into(),
            packed: ::std::default::Default::default(),
            jstype: 0i32.try_into(),
            lazy: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            weak: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FieldOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.ctype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>()?;
                }
                2 => {
                    *self.packed.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.jstype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>()?;
                }
                5 => {
                    *self.lazy.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                10 => {
                    *self.weak.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.ctype, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.packed, first, iter);
                }
                6 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.jstype, first, iter);
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.lazy, first, iter);
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                10 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FieldOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.ctype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>()?;
                }
                2 => {
                    *self.packed.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.jstype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>()?;
                }
                5 => {
                    *self.lazy.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                10 => {
                    *self.weak.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.ctype, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.packed, first, iter);
                }
                6 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.jstype, first, iter);
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.lazy, first, iter);
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                10 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for FieldOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for FieldOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>, _>(
            1, 
            self.ctype.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.packed.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>, _>(
            6, 
            self.jstype.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            5, 
            self.lazy.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            3, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            10, 
            self.weak.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for FieldOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FieldOptionsTrait for FieldOptions {
    fn ctype(&self) -> ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32> {
        self.ctype.clone()
    }
    fn packed(&self) -> bool {
        self.packed.clone()
    }
    fn jstype(&self) -> ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32> {
        self.jstype.clone()
    }
    fn lazy(&self) -> bool {
        self.lazy.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn weak(&self) -> bool {
        self.weak.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FieldOptionsBumpalo<'b> {
    pub ctype: ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>,
    pub packed: bool,
    pub jstype: ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>,
    pub lazy: bool,
    pub deprecated: bool,
    pub weak: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> FieldOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            ctype: 0i32.try_into(),
            packed: ::std::default::Default::default(),
            jstype: 0i32.try_into(),
            lazy: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            weak: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FieldOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.ctype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>()?;
                }
                2 => {
                    *self.packed.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.jstype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>()?;
                }
                5 => {
                    *self.lazy.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                10 => {
                    *self.weak.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.ctype, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.packed, first, iter);
                }
                6 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.jstype, first, iter);
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.lazy, first, iter);
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                10 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FieldOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.ctype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>()?;
                }
                2 => {
                    *self.packed.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.jstype.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>()?;
                }
                5 => {
                    *self.lazy.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                10 => {
                    *self.weak.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.ctype, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.packed, first, iter);
                }
                6 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.jstype, first, iter);
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.lazy, first, iter);
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                10 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for FieldOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for FieldOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Ctype>, _>(
            1, 
            self.ctype.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.packed.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_options::Jstype>, _>(
            6, 
            self.jstype.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            5, 
            self.lazy.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            3, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            10, 
            self.weak.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for FieldOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl FieldOptionsTrait for FieldOptionsBumpalo {
    fn ctype(&self) -> ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32> {
        self.ctype.clone()
    }
    fn packed(&self) -> bool {
        self.packed.clone()
    }
    fn jstype(&self) -> ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32> {
        self.jstype.clone()
    }
    fn lazy(&self) -> bool {
        self.lazy.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn weak(&self) -> bool {
        self.weak.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait FieldOptionsTrait {
    fn ctype(&self) -> ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>;
    fn packed(&self) -> bool;
    fn jstype(&self) -> ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>;
    fn lazy(&self) -> bool;
    fn deprecated(&self) -> bool;
    fn weak(&self) -> bool;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait FieldOptionsMutTrait {
    fn ctype_mut(&self) -> &mut ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>;
    fn packed_mut(&self) -> &mut bool;
    fn jstype_mut(&self) -> &mut ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>;
    fn lazy_mut(&self) -> &mut bool;
    fn deprecated_mut(&self) -> &mut bool;
    fn weak_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}
pub mod field_options {
#[derive(Debug, Clone)]
pub enum Jstype {
    JsNormal = 0,
    JsString = 1,
    JsNumber = 2,
}
impl ::std::convert::TryFrom<i32> for Jstype {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            0 => Ok(Self::JsNormal),
            1 => Ok(Self::JsString),
            2 => Ok(Self::JsNumber),
            x => Err(x),
        }
    }
}
impl ::std::convert::Into<i32> for Jstype {
    fn into(self) -> i32 {
        self as i32
    }
}
#[derive(Debug, Clone)]
pub enum Ctype {
    String = 0,
    Cord = 1,
    StringPiece = 2,
}
impl ::std::convert::TryFrom<i32> for Ctype {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            0 => Ok(Self::String),
            1 => Ok(Self::Cord),
            2 => Ok(Self::StringPiece),
            x => Err(x),
        }
    }
}
impl ::std::convert::Into<i32> for Ctype {
    fn into(self) -> i32 {
        self as i32
    }
}
} // mod field_options

#[derive(Debug, Clone)]
pub struct MessageOptions {
    pub message_set_wire_format: bool,
    pub no_standard_descriptor_accessor: bool,
    pub deprecated: bool,
    pub map_entry: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl MessageOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            message_set_wire_format: ::std::default::Default::default(),
            no_standard_descriptor_accessor: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            map_entry: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MessageOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut MessageOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.message_set_wire_format.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                2 => {
                    *self.no_standard_descriptor_accessor.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                7 => {
                    *self.map_entry.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.message_set_wire_format, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.no_standard_descriptor_accessor, first, iter);
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                7 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.map_entry, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for MessageOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.message_set_wire_format.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                2 => {
                    *self.no_standard_descriptor_accessor.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                7 => {
                    *self.map_entry.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.message_set_wire_format, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.no_standard_descriptor_accessor, first, iter);
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                7 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.map_entry, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for MessageOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for MessageOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            1, 
            self.message_set_wire_format.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.no_standard_descriptor_accessor.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            3, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            7, 
            self.map_entry.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for MessageOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl MessageOptionsTrait for MessageOptions {
    fn message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.clone()
    }
    fn no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn map_entry(&self) -> bool {
        self.map_entry.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct MessageOptionsBumpalo<'b> {
    pub message_set_wire_format: bool,
    pub no_standard_descriptor_accessor: bool,
    pub deprecated: bool,
    pub map_entry: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> MessageOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            message_set_wire_format: ::std::default::Default::default(),
            no_standard_descriptor_accessor: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            map_entry: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut MessageOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.message_set_wire_format.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                2 => {
                    *self.no_standard_descriptor_accessor.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                7 => {
                    *self.map_entry.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.message_set_wire_format, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.no_standard_descriptor_accessor, first, iter);
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                7 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.map_entry, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for MessageOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.message_set_wire_format.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                2 => {
                    *self.no_standard_descriptor_accessor.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                7 => {
                    *self.map_entry.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.message_set_wire_format, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.no_standard_descriptor_accessor, first, iter);
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                7 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.map_entry, first, iter);
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for MessageOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for MessageOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            1, 
            self.message_set_wire_format.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            2, 
            self.no_standard_descriptor_accessor.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            3, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            7, 
            self.map_entry.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for MessageOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl MessageOptionsTrait for MessageOptionsBumpalo {
    fn message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.clone()
    }
    fn no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn map_entry(&self) -> bool {
        self.map_entry.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait MessageOptionsTrait {
    fn message_set_wire_format(&self) -> bool;
    fn no_standard_descriptor_accessor(&self) -> bool;
    fn deprecated(&self) -> bool;
    fn map_entry(&self) -> bool;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait MessageOptionsMutTrait {
    fn message_set_wire_format_mut(&self) -> &mut bool;
    fn no_standard_descriptor_accessor_mut(&self) -> &mut bool;
    fn deprecated_mut(&self) -> &mut bool;
    fn map_entry_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}

#[derive(Debug, Clone)]
pub struct FileOptions {
    pub java_package: ::std::string::String,
    pub java_outer_classname: ::std::string::String,
    pub java_multiple_files: bool,
    pub java_generate_equals_and_hash: bool,
    pub java_string_check_utf8: bool,
    pub optimize_for: ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
    pub go_package: ::std::string::String,
    pub cc_generic_services: bool,
    pub java_generic_services: bool,
    pub py_generic_services: bool,
    pub php_generic_services: bool,
    pub deprecated: bool,
    pub cc_enable_arenas: bool,
    pub objc_class_prefix: ::std::string::String,
    pub csharp_namespace: ::std::string::String,
    pub swift_prefix: ::std::string::String,
    pub php_class_prefix: ::std::string::String,
    pub php_namespace: ::std::string::String,
    pub php_metadata_namespace: ::std::string::String,
    pub ruby_package: ::std::string::String,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl FileOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            java_package: ::std::default::Default::default(),
            java_outer_classname: ::std::default::Default::default(),
            java_multiple_files: ::std::default::Default::default(),
            java_generate_equals_and_hash: ::std::default::Default::default(),
            java_string_check_utf8: ::std::default::Default::default(),
            optimize_for: 0i32.try_into(),
            go_package: ::std::default::Default::default(),
            cc_generic_services: ::std::default::Default::default(),
            java_generic_services: ::std::default::Default::default(),
            py_generic_services: ::std::default::Default::default(),
            php_generic_services: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            cc_enable_arenas: ::std::default::Default::default(),
            objc_class_prefix: ::std::default::Default::default(),
            csharp_namespace: ::std::default::Default::default(),
            swift_prefix: ::std::default::Default::default(),
            php_class_prefix: ::std::default::Default::default(),
            php_namespace: ::std::default::Default::default(),
            php_metadata_namespace: ::std::default::Default::default(),
            ruby_package: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FileOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.java_multiple_files.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                20 => {
                    *self.java_generate_equals_and_hash.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                27 => {
                    *self.java_string_check_utf8.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                9 => {
                    *self.optimize_for.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>()?;
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    *self.cc_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                17 => {
                    *self.java_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                18 => {
                    *self.py_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                42 => {
                    *self.php_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                23 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                31 => {
                    *self.cc_enable_arenas.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                36 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                37 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                39 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                40 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                41 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                44 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                45 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.java_package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.java_outer_classname.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_multiple_files, first, iter);
                }
                20 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generate_equals_and_hash, first, iter);
                }
                27 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_string_check_utf8, first, iter);
                }
                9 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.optimize_for, first, iter);
                }
                11 => {
                    *self.go_package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                16 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_generic_services, first, iter);
                }
                17 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generic_services, first, iter);
                }
                18 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.py_generic_services, first, iter);
                }
                42 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.php_generic_services, first, iter);
                }
                23 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                31 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_enable_arenas, first, iter);
                }
                36 => {
                    *self.objc_class_prefix.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                37 => {
                    *self.csharp_namespace.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                39 => {
                    *self.swift_prefix.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                40 => {
                    *self.php_class_prefix.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                41 => {
                    *self.php_namespace.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                44 => {
                    *self.php_metadata_namespace.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                45 => {
                    *self.ruby_package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FileOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.java_multiple_files.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                20 => {
                    *self.java_generate_equals_and_hash.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                27 => {
                    *self.java_string_check_utf8.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                9 => {
                    *self.optimize_for.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>()?;
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    *self.cc_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                17 => {
                    *self.java_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                18 => {
                    *self.py_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                42 => {
                    *self.php_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                23 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                31 => {
                    *self.cc_enable_arenas.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                36 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                37 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                39 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                40 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                41 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                44 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                45 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.java_package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.java_outer_classname.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_multiple_files, first, iter);
                }
                20 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generate_equals_and_hash, first, iter);
                }
                27 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_string_check_utf8, first, iter);
                }
                9 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.optimize_for, first, iter);
                }
                11 => {
                    *self.go_package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                16 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_generic_services, first, iter);
                }
                17 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generic_services, first, iter);
                }
                18 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.py_generic_services, first, iter);
                }
                42 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.php_generic_services, first, iter);
                }
                23 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                31 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_enable_arenas, first, iter);
                }
                36 => {
                    *self.objc_class_prefix.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                37 => {
                    *self.csharp_namespace.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                39 => {
                    *self.swift_prefix.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                40 => {
                    *self.php_class_prefix.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                41 => {
                    *self.php_namespace.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                44 => {
                    *self.php_metadata_namespace.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                45 => {
                    *self.ruby_package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for FileOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for FileOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.java_package.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.java_outer_classname.iter_for_ser() {
            serializer.serialize_bytes_twice(8, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            10, 
            self.java_multiple_files.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            20, 
            self.java_generate_equals_and_hash.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            27, 
            self.java_string_check_utf8.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>, _>(
            9, 
            self.optimize_for.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.go_package.iter_for_ser() {
            serializer.serialize_bytes_twice(11, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            16, 
            self.cc_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            17, 
            self.java_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            18, 
            self.py_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            42, 
            self.php_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            23, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            31, 
            self.cc_enable_arenas.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.objc_class_prefix.iter_for_ser() {
            serializer.serialize_bytes_twice(36, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.csharp_namespace.iter_for_ser() {
            serializer.serialize_bytes_twice(37, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.swift_prefix.iter_for_ser() {
            serializer.serialize_bytes_twice(39, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.php_class_prefix.iter_for_ser() {
            serializer.serialize_bytes_twice(40, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.php_namespace.iter_for_ser() {
            serializer.serialize_bytes_twice(41, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.php_metadata_namespace.iter_for_ser() {
            serializer.serialize_bytes_twice(44, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.ruby_package.iter_for_ser() {
            serializer.serialize_bytes_twice(45, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for FileOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileOptionsTrait for FileOptions {
    fn java_package(&self) -> &str {
        self.java_package.as_ref()
    }
    fn java_outer_classname(&self) -> &str {
        self.java_outer_classname.as_ref()
    }
    fn java_multiple_files(&self) -> bool {
        self.java_multiple_files.clone()
    }
    fn java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.clone()
    }
    fn java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8.clone()
    }
    fn optimize_for(&self) -> ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32> {
        self.optimize_for.clone()
    }
    fn go_package(&self) -> &str {
        self.go_package.as_ref()
    }
    fn cc_generic_services(&self) -> bool {
        self.cc_generic_services.clone()
    }
    fn java_generic_services(&self) -> bool {
        self.java_generic_services.clone()
    }
    fn py_generic_services(&self) -> bool {
        self.py_generic_services.clone()
    }
    fn php_generic_services(&self) -> bool {
        self.php_generic_services.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas.clone()
    }
    fn objc_class_prefix(&self) -> &str {
        self.objc_class_prefix.as_ref()
    }
    fn csharp_namespace(&self) -> &str {
        self.csharp_namespace.as_ref()
    }
    fn swift_prefix(&self) -> &str {
        self.swift_prefix.as_ref()
    }
    fn php_class_prefix(&self) -> &str {
        self.php_class_prefix.as_ref()
    }
    fn php_namespace(&self) -> &str {
        self.php_namespace.as_ref()
    }
    fn php_metadata_namespace(&self) -> &str {
        self.php_metadata_namespace.as_ref()
    }
    fn ruby_package(&self) -> &str {
        self.ruby_package.as_ref()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FileOptionsBumpalo<'b> {
    pub java_package: ::bumpalo::collections::String,
    pub java_outer_classname: ::bumpalo::collections::String,
    pub java_multiple_files: bool,
    pub java_generate_equals_and_hash: bool,
    pub java_string_check_utf8: bool,
    pub optimize_for: ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
    pub go_package: ::bumpalo::collections::String,
    pub cc_generic_services: bool,
    pub java_generic_services: bool,
    pub py_generic_services: bool,
    pub php_generic_services: bool,
    pub deprecated: bool,
    pub cc_enable_arenas: bool,
    pub objc_class_prefix: ::bumpalo::collections::String,
    pub csharp_namespace: ::bumpalo::collections::String,
    pub swift_prefix: ::bumpalo::collections::String,
    pub php_class_prefix: ::bumpalo::collections::String,
    pub php_namespace: ::bumpalo::collections::String,
    pub php_metadata_namespace: ::bumpalo::collections::String,
    pub ruby_package: ::bumpalo::collections::String,
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> FileOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            java_package: ::std::default::Default::default(),
            java_outer_classname: ::std::default::Default::default(),
            java_multiple_files: ::std::default::Default::default(),
            java_generate_equals_and_hash: ::std::default::Default::default(),
            java_string_check_utf8: ::std::default::Default::default(),
            optimize_for: 0i32.try_into(),
            go_package: ::std::default::Default::default(),
            cc_generic_services: ::std::default::Default::default(),
            java_generic_services: ::std::default::Default::default(),
            py_generic_services: ::std::default::Default::default(),
            php_generic_services: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            cc_enable_arenas: ::std::default::Default::default(),
            objc_class_prefix: ::std::default::Default::default(),
            csharp_namespace: ::std::default::Default::default(),
            swift_prefix: ::std::default::Default::default(),
            php_class_prefix: ::std::default::Default::default(),
            php_namespace: ::std::default::Default::default(),
            php_metadata_namespace: ::std::default::Default::default(),
            ruby_package: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FileOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.java_multiple_files.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                20 => {
                    *self.java_generate_equals_and_hash.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                27 => {
                    *self.java_string_check_utf8.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                9 => {
                    *self.optimize_for.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>()?;
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    *self.cc_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                17 => {
                    *self.java_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                18 => {
                    *self.py_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                42 => {
                    *self.php_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                23 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                31 => {
                    *self.cc_enable_arenas.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                36 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                37 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                39 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                40 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                41 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                44 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                45 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.java_package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.java_outer_classname.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_multiple_files, first, iter);
                }
                20 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generate_equals_and_hash, first, iter);
                }
                27 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_string_check_utf8, first, iter);
                }
                9 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.optimize_for, first, iter);
                }
                11 => {
                    *self.go_package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                16 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_generic_services, first, iter);
                }
                17 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generic_services, first, iter);
                }
                18 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.py_generic_services, first, iter);
                }
                42 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.php_generic_services, first, iter);
                }
                23 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                31 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_enable_arenas, first, iter);
                }
                36 => {
                    *self.objc_class_prefix.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                37 => {
                    *self.csharp_namespace.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                39 => {
                    *self.swift_prefix.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                40 => {
                    *self.php_class_prefix.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                41 => {
                    *self.php_namespace.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                44 => {
                    *self.php_metadata_namespace.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                45 => {
                    *self.ruby_package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FileOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.java_multiple_files.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                20 => {
                    *self.java_generate_equals_and_hash.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                27 => {
                    *self.java_string_check_utf8.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                9 => {
                    *self.optimize_for.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>()?;
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    *self.cc_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                17 => {
                    *self.java_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                18 => {
                    *self.py_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                42 => {
                    *self.php_generic_services.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                23 => {
                    *self.deprecated.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                31 => {
                    *self.cc_enable_arenas.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                36 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                37 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                39 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                40 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                41 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                44 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                45 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.java_package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *self.java_outer_classname.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_multiple_files, first, iter);
                }
                20 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generate_equals_and_hash, first, iter);
                }
                27 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_string_check_utf8, first, iter);
                }
                9 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.optimize_for, first, iter);
                }
                11 => {
                    *self.go_package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                16 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_generic_services, first, iter);
                }
                17 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.java_generic_services, first, iter);
                }
                18 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.py_generic_services, first, iter);
                }
                42 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.php_generic_services, first, iter);
                }
                23 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                }
                31 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.cc_enable_arenas, first, iter);
                }
                36 => {
                    *self.objc_class_prefix.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                37 => {
                    *self.csharp_namespace.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                39 => {
                    *self.swift_prefix.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                40 => {
                    *self.php_class_prefix.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                41 => {
                    *self.php_namespace.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                44 => {
                    *self.php_metadata_namespace.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                45 => {
                    *self.ruby_package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40 | 41 | 44 | 45 | 999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for FileOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for FileOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.java_package.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.java_outer_classname.iter_for_ser() {
            serializer.serialize_bytes_twice(8, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            10, 
            self.java_multiple_files.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            20, 
            self.java_generate_equals_and_hash.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            27, 
            self.java_string_check_utf8.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::file_options::OptimizeMode>, _>(
            9, 
            self.optimize_for.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.go_package.iter_for_ser() {
            serializer.serialize_bytes_twice(11, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            16, 
            self.cc_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            17, 
            self.java_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            18, 
            self.py_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            42, 
            self.php_generic_services.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            23, 
            self.deprecated.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            31, 
            self.cc_enable_arenas.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.objc_class_prefix.iter_for_ser() {
            serializer.serialize_bytes_twice(36, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.csharp_namespace.iter_for_ser() {
            serializer.serialize_bytes_twice(37, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.swift_prefix.iter_for_ser() {
            serializer.serialize_bytes_twice(39, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.php_class_prefix.iter_for_ser() {
            serializer.serialize_bytes_twice(40, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.php_namespace.iter_for_ser() {
            serializer.serialize_bytes_twice(41, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.php_metadata_namespace.iter_for_ser() {
            serializer.serialize_bytes_twice(44, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.ruby_package.iter_for_ser() {
            serializer.serialize_bytes_twice(45, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for FileOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl FileOptionsTrait for FileOptionsBumpalo {
    fn java_package(&self) -> &str {
        self.java_package.as_ref()
    }
    fn java_outer_classname(&self) -> &str {
        self.java_outer_classname.as_ref()
    }
    fn java_multiple_files(&self) -> bool {
        self.java_multiple_files.clone()
    }
    fn java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.clone()
    }
    fn java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8.clone()
    }
    fn optimize_for(&self) -> ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32> {
        self.optimize_for.clone()
    }
    fn go_package(&self) -> &str {
        self.go_package.as_ref()
    }
    fn cc_generic_services(&self) -> bool {
        self.cc_generic_services.clone()
    }
    fn java_generic_services(&self) -> bool {
        self.java_generic_services.clone()
    }
    fn py_generic_services(&self) -> bool {
        self.py_generic_services.clone()
    }
    fn php_generic_services(&self) -> bool {
        self.php_generic_services.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas.clone()
    }
    fn objc_class_prefix(&self) -> &str {
        self.objc_class_prefix.as_ref()
    }
    fn csharp_namespace(&self) -> &str {
        self.csharp_namespace.as_ref()
    }
    fn swift_prefix(&self) -> &str {
        self.swift_prefix.as_ref()
    }
    fn php_class_prefix(&self) -> &str {
        self.php_class_prefix.as_ref()
    }
    fn php_namespace(&self) -> &str {
        self.php_namespace.as_ref()
    }
    fn php_metadata_namespace(&self) -> &str {
        self.php_metadata_namespace.as_ref()
    }
    fn ruby_package(&self) -> &str {
        self.ruby_package.as_ref()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait FileOptionsTrait {
    fn java_package(&self) -> &str;
    fn java_outer_classname(&self) -> &str;
    fn java_multiple_files(&self) -> bool;
    fn java_generate_equals_and_hash(&self) -> bool;
    fn java_string_check_utf8(&self) -> bool;
    fn optimize_for(&self) -> ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>;
    fn go_package(&self) -> &str;
    fn cc_generic_services(&self) -> bool;
    fn java_generic_services(&self) -> bool;
    fn py_generic_services(&self) -> bool;
    fn php_generic_services(&self) -> bool;
    fn deprecated(&self) -> bool;
    fn cc_enable_arenas(&self) -> bool;
    fn objc_class_prefix(&self) -> &str;
    fn csharp_namespace(&self) -> &str;
    fn swift_prefix(&self) -> &str;
    fn php_class_prefix(&self) -> &str;
    fn php_namespace(&self) -> &str;
    fn php_metadata_namespace(&self) -> &str;
    fn ruby_package(&self) -> &str;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait FileOptionsMutTrait {
    fn java_package_mut(&self) -> &mut String;
    fn java_outer_classname_mut(&self) -> &mut String;
    fn java_multiple_files_mut(&self) -> &mut bool;
    fn java_generate_equals_and_hash_mut(&self) -> &mut bool;
    fn java_string_check_utf8_mut(&self) -> &mut bool;
    fn optimize_for_mut(&self) -> &mut ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>;
    fn go_package_mut(&self) -> &mut String;
    fn cc_generic_services_mut(&self) -> &mut bool;
    fn java_generic_services_mut(&self) -> &mut bool;
    fn py_generic_services_mut(&self) -> &mut bool;
    fn php_generic_services_mut(&self) -> &mut bool;
    fn deprecated_mut(&self) -> &mut bool;
    fn cc_enable_arenas_mut(&self) -> &mut bool;
    fn objc_class_prefix_mut(&self) -> &mut String;
    fn csharp_namespace_mut(&self) -> &mut String;
    fn swift_prefix_mut(&self) -> &mut String;
    fn php_class_prefix_mut(&self) -> &mut String;
    fn php_namespace_mut(&self) -> &mut String;
    fn php_metadata_namespace_mut(&self) -> &mut String;
    fn ruby_package_mut(&self) -> &mut String;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}
pub mod file_options {
#[derive(Debug, Clone)]
pub enum OptimizeMode {
    Speed = 1,
    CodeSize = 2,
    LiteRuntime = 3,
}
impl ::std::convert::TryFrom<i32> for OptimizeMode {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            1 => Ok(Self::Speed),
            2 => Ok(Self::CodeSize),
            3 => Ok(Self::LiteRuntime),
            x => Err(x),
        }
    }
}
impl ::std::convert::Into<i32> for OptimizeMode {
    fn into(self) -> i32 {
        self as i32
    }
}
} // mod file_options

#[derive(Debug, Clone)]
pub struct MethodDescriptorProto {
    pub name: ::std::string::String,
    pub input_type: ::std::string::String,
    pub output_type: ::std::string::String,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MethodOptions>>,
    pub client_streaming: bool,
    pub server_streaming: bool,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl MethodDescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            input_type: ::std::default::Default::default(),
            output_type: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            client_streaming: ::std::default::Default::default(),
            server_streaming: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut MethodDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    *self.client_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.server_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.input_type.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.output_type.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.client_streaming, first, iter);
                }
                6 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.server_streaming, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for MethodDescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    *self.client_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.server_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.input_type.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.output_type.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.client_streaming, first, iter);
                }
                6 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.server_streaming, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for MethodDescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for MethodDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.input_type.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.output_type.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            5, 
            self.client_streaming.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            6, 
            self.server_streaming.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}

impl ::puroro::Serializable for MethodDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl MethodDescriptorProtoTrait for MethodDescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn input_type(&self) -> &str {
        self.input_type.as_ref()
    }
    fn output_type(&self) -> &str {
        self.output_type.as_ref()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::MethodOptions> {
        self.options.as_deref()
    }
    fn client_streaming(&self) -> bool {
        self.client_streaming.clone()
    }
    fn server_streaming(&self) -> bool {
        self.server_streaming.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct MethodDescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub input_type: ::bumpalo::collections::String,
    pub output_type: ::bumpalo::collections::String,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::MethodOptions>>,
    pub client_streaming: bool,
    pub server_streaming: bool,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> MethodDescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            input_type: ::std::default::Default::default(),
            output_type: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            client_streaming: ::std::default::Default::default(),
            server_streaming: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut MethodDescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    *self.client_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.server_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.input_type.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.output_type.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.client_streaming, first, iter);
                }
                6 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.server_streaming, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for MethodDescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    *self.client_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    *self.server_streaming.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.input_type.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.output_type.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.client_streaming, first, iter);
                }
                6 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.server_streaming, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for MethodDescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for MethodDescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.input_type.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.output_type.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            5, 
            self.client_streaming.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            6, 
            self.server_streaming.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for MethodDescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl MethodDescriptorProtoTrait for MethodDescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn input_type(&self) -> &str {
        self.input_type.as_ref()
    }
    fn output_type(&self) -> &str {
        self.output_type.as_ref()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::MethodOptions> {
        self.options.as_deref()
    }
    fn client_streaming(&self) -> bool {
        self.client_streaming.clone()
    }
    fn server_streaming(&self) -> bool {
        self.server_streaming.clone()
    }
}
pub trait MethodDescriptorProtoTrait {
    fn name(&self) -> &str;
    fn input_type(&self) -> &str;
    fn output_type(&self) -> &str;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::MethodOptions>;
    fn client_streaming(&self) -> bool;
    fn server_streaming(&self) -> bool;
}
pub trait MethodDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn input_type_mut(&self) -> &mut String;
    fn output_type_mut(&self) -> &mut String;
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::MethodOptions>;
    fn client_streaming_mut(&self) -> &mut bool;
    fn server_streaming_mut(&self) -> &mut bool;
}

#[derive(Debug, Clone)]
pub struct ServiceDescriptorProto {
    pub name: ::std::string::String,
    pub method: ::std::vec::Vec<super::super::google::protobuf::MethodDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::ServiceOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl ServiceDescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            method: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ServiceDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.method.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ServiceDescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.method.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for ServiceDescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for ServiceDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.method.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for ServiceDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ServiceDescriptorProtoTrait for ServiceDescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn for_each_method<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::MethodDescriptorProto) {
        for item in (self.method).iter() {
            (f)(item);
        }
    }
    fn method_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::MethodDescriptorProto>> {
        ::std::boxed::Box::new(self.method.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::MethodDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_> {
        self.method.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::ServiceOptions> {
        self.options.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ServiceDescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub method: ::bumpalo::collections::Vec<super::super::google::protobuf::MethodDescriptorProto>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::ServiceOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ServiceDescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            method: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ServiceDescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.method.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ServiceDescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.method.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for ServiceDescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for ServiceDescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.method.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for ServiceDescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ServiceDescriptorProtoTrait for ServiceDescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn for_each_method<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::MethodDescriptorProto) {
        for item in (self.method).iter() {
            (f)(item);
        }
    }
    fn method_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::MethodDescriptorProto>> {
        ::std::boxed::Box::new(self.method.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::MethodDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_> {
        self.method.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::ServiceOptions> {
        self.options.as_deref()
    }
}
pub trait ServiceDescriptorProtoTrait {
    fn name(&self) -> &str;
    fn for_each_method<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::MethodDescriptorProto);
    fn method_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::MethodDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a>: Iterator<Item=&'a super::super::google::protobuf::MethodDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_>;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::ServiceOptions>;
}
pub trait ServiceDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn for_each_method_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::MethodDescriptorProto);
    fn method_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::MethodDescriptorProto>>;
    // We need more! 
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::ServiceOptions>;
}

#[derive(Debug, Clone)]
pub struct EnumValueDescriptorProto {
    pub name: ::std::string::String,
    pub number: i32,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumValueOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl EnumValueDescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumValueDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumValueDescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for EnumValueDescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for EnumValueDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.number.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumValueDescriptorProtoTrait for EnumValueDescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::EnumValueOptions> {
        self.options.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumValueDescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub number: i32,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::EnumValueOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> EnumValueDescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumValueDescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumValueDescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for EnumValueDescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for EnumValueDescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.number.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for EnumValueDescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::EnumValueOptions> {
        self.options.as_deref()
    }
}
pub trait EnumValueDescriptorProtoTrait {
    fn name(&self) -> &str;
    fn number(&self) -> i32;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::EnumValueOptions>;
}
pub trait EnumValueDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn number_mut(&self) -> &mut i32;
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::EnumValueOptions>;
}

#[derive(Debug, Clone)]
pub struct EnumDescriptorProto {
    pub name: ::std::string::String,
    pub value: ::std::vec::Vec<super::super::google::protobuf::EnumValueDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumOptions>>,
    pub reserved_range: ::std::vec::Vec<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl EnumDescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            value: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            reserved_range: ::std::default::Default::default(),
            reserved_name: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.value.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                4 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    *self.reserved_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumDescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.value.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                4 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    *self.reserved_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for EnumDescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for EnumDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.value.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        for msg in self.reserved_range.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        for string in self.reserved_name.iter_for_ser() {
            serializer.serialize_bytes_twice(5, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for EnumDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumDescriptorProtoTrait for EnumDescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn for_each_value<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumValueDescriptorProto) {
        for item in (self.value).iter() {
            (f)(item);
        }
    }
    fn value_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumValueDescriptorProto>> {
        ::std::boxed::Box::new(self.value.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::EnumValueDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_> {
        self.value.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::EnumOptions> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumDescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub value: ::bumpalo::collections::Vec<super::super::google::protobuf::EnumValueDescriptorProto>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::EnumOptions>>,
    pub reserved_range: ::bumpalo::collections::Vec<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>,
    pub reserved_name: ::bumpalo::collections::Vec<::bumpalo::collections::String>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> EnumDescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            value: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            reserved_range: ::std::default::Default::default(),
            reserved_name: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumDescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.value.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                4 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    *self.reserved_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumDescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.value.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                4 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    *self.reserved_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for EnumDescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for EnumDescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.value.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        for msg in self.reserved_range.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        for string in self.reserved_name.iter_for_ser() {
            serializer.serialize_bytes_twice(5, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for EnumDescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl EnumDescriptorProtoTrait for EnumDescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn for_each_value<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumValueDescriptorProto) {
        for item in (self.value).iter() {
            (f)(item);
        }
    }
    fn value_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumValueDescriptorProto>> {
        ::std::boxed::Box::new(self.value.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::EnumValueDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_> {
        self.value.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::EnumOptions> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
pub trait EnumDescriptorProtoTrait {
    fn name(&self) -> &str;
    fn for_each_value<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumValueDescriptorProto);
    fn value_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumValueDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a>: Iterator<Item=&'a super::super::google::protobuf::EnumValueDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_>;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::EnumOptions>;
    fn for_each_reserved_range<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange);
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a>: Iterator<Item=&'a super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_>;
    fn for_each_reserved_name<F>(&self, f: F)
    where
        F: FnMut(&str);
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a>: Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_>;
}
pub trait EnumDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn for_each_value_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::EnumValueDescriptorProto);
    fn value_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::EnumValueDescriptorProto>>;
    // We need more! 
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::EnumOptions>;
    fn for_each_reserved_range_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange);
    fn reserved_range_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>>;
    // We need more! 
    fn for_each_reserved_name_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn reserved_name_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut String>>;
    // We need more! 
}
pub mod enum_descriptor_proto {

#[derive(Debug, Clone)]
pub struct EnumReservedRange {
    pub start: i32,
    pub end: i32,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl EnumReservedRange {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            start: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumReservedRange {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumReservedRange {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumReservedRange {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for EnumReservedRange {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for EnumReservedRange {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.start.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumReservedRange {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumReservedRangeTrait for EnumReservedRange {
    fn start(&self) -> i32 {
        self.start.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumReservedRangeBumpalo<'b> {
    pub start: i32,
    pub end: i32,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> EnumReservedRangeBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            start: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut EnumReservedRangeBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for EnumReservedRangeBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for EnumReservedRangeBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for EnumReservedRangeBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.start.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for EnumReservedRangeBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl EnumReservedRangeTrait for EnumReservedRangeBumpalo {
    fn start(&self) -> i32 {
        self.start.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
}
pub trait EnumReservedRangeTrait {
    fn start(&self) -> i32;
    fn end(&self) -> i32;
}
pub trait EnumReservedRangeMutTrait {
    fn start_mut(&self) -> &mut i32;
    fn end_mut(&self) -> &mut i32;
}
} // mod enum_descriptor_proto

#[derive(Debug, Clone)]
pub struct OneofDescriptorProto {
    pub name: ::std::string::String,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::OneofOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl OneofDescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut OneofDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for OneofDescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for OneofDescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for OneofDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for OneofDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl OneofDescriptorProtoTrait for OneofDescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::OneofOptions> {
        self.options.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct OneofDescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::OneofOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> OneofDescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut OneofDescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for OneofDescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for OneofDescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for OneofDescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for OneofDescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl OneofDescriptorProtoTrait for OneofDescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::OneofOptions> {
        self.options.as_deref()
    }
}
pub trait OneofDescriptorProtoTrait {
    fn name(&self) -> &str;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::OneofOptions>;
}
pub trait OneofDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::OneofOptions>;
}

#[derive(Debug, Clone)]
pub struct FieldDescriptorProto {
    pub name: ::std::string::String,
    pub number: i32,
    pub label: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>,
    pub type_: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>,
    pub type_name: ::std::string::String,
    pub extendee: ::std::string::String,
    pub default_value: ::std::string::String,
    pub oneof_index: i32,
    pub json_name: ::std::string::String,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FieldOptions>>,
    pub proto3_optional: bool,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl FieldDescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            label: 0i32.try_into(),
            type_: 0i32.try_into(),
            type_name: ::std::default::Default::default(),
            extendee: ::std::default::Default::default(),
            default_value: ::std::default::Default::default(),
            oneof_index: ::std::default::Default::default(),
            json_name: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            proto3_optional: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FieldDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.label.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>()?;
                }
                5 => {
                    *self.type_.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    *self.oneof_index.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    *self.proto3_optional.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                4 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.label, first, iter);
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.type_, first, iter);
                }
                6 => {
                    *self.type_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.extendee.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                7 => {
                    *self.default_value.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                9 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.oneof_index, first, iter);
                }
                10 => {
                    *self.json_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                17 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.proto3_optional, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FieldDescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.label.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>()?;
                }
                5 => {
                    *self.type_.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    *self.oneof_index.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    *self.proto3_optional.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                4 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.label, first, iter);
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.type_, first, iter);
                }
                6 => {
                    *self.type_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.extendee.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                7 => {
                    *self.default_value.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                9 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.oneof_index, first, iter);
                }
                10 => {
                    *self.json_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                17 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.proto3_optional, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for FieldDescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for FieldDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            3, 
            self.number.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>, _>(
            4, 
            self.label.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>, _>(
            5, 
            self.type_.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.type_name.iter_for_ser() {
            serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.extendee.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.default_value.iter_for_ser() {
            serializer.serialize_bytes_twice(7, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            9, 
            self.oneof_index.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.json_name.iter_for_ser() {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            17, 
            self.proto3_optional.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}

impl ::puroro::Serializable for FieldDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FieldDescriptorProtoTrait for FieldDescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    fn label(&self) -> ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32> {
        self.label.clone()
    }
    fn type_(&self) -> ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32> {
        self.type_.clone()
    }
    fn type_name(&self) -> &str {
        self.type_name.as_ref()
    }
    fn extendee(&self) -> &str {
        self.extendee.as_ref()
    }
    fn default_value(&self) -> &str {
        self.default_value.as_ref()
    }
    fn oneof_index(&self) -> i32 {
        self.oneof_index.clone()
    }
    fn json_name(&self) -> &str {
        self.json_name.as_ref()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::FieldOptions> {
        self.options.as_deref()
    }
    fn proto3_optional(&self) -> bool {
        self.proto3_optional.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FieldDescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub number: i32,
    pub label: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>,
    pub type_: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>,
    pub type_name: ::bumpalo::collections::String,
    pub extendee: ::bumpalo::collections::String,
    pub default_value: ::bumpalo::collections::String,
    pub oneof_index: i32,
    pub json_name: ::bumpalo::collections::String,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::FieldOptions>>,
    pub proto3_optional: bool,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> FieldDescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            label: 0i32.try_into(),
            type_: 0i32.try_into(),
            type_name: ::std::default::Default::default(),
            extendee: ::std::default::Default::default(),
            default_value: ::std::default::Default::default(),
            oneof_index: ::std::default::Default::default(),
            json_name: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            proto3_optional: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FieldDescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.label.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>()?;
                }
                5 => {
                    *self.type_.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    *self.oneof_index.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    *self.proto3_optional.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                4 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.label, first, iter);
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.type_, first, iter);
                }
                6 => {
                    *self.type_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.extendee.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                7 => {
                    *self.default_value.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                9 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.oneof_index, first, iter);
                }
                10 => {
                    *self.json_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                17 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.proto3_optional, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FieldDescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.number.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    *self.label.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>()?;
                }
                5 => {
                    *self.type_.push_and_get_mut() = variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    *self.oneof_index.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    *self.proto3_optional.push_and_get_mut() = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                4 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.label, first, iter);
                }
                5 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.type_, first, iter);
                }
                6 => {
                    *self.type_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.extendee.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                7 => {
                    *self.default_value.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                9 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.oneof_index, first, iter);
                }
                10 => {
                    *self.json_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                17 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Bool>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.proto3_optional, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for FieldDescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for FieldDescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            3, 
            self.number.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>, _>(
            4, 
            self.label.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>, _>(
            5, 
            self.type_.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.type_name.iter_for_ser() {
            serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.extendee.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.default_value.iter_for_ser() {
            serializer.serialize_bytes_twice(7, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            9, 
            self.oneof_index.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for string in self.json_name.iter_for_ser() {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Bool, _>(
            17, 
            self.proto3_optional.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for FieldDescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl FieldDescriptorProtoTrait for FieldDescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    fn label(&self) -> ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32> {
        self.label.clone()
    }
    fn type_(&self) -> ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32> {
        self.type_.clone()
    }
    fn type_name(&self) -> &str {
        self.type_name.as_ref()
    }
    fn extendee(&self) -> &str {
        self.extendee.as_ref()
    }
    fn default_value(&self) -> &str {
        self.default_value.as_ref()
    }
    fn oneof_index(&self) -> i32 {
        self.oneof_index.clone()
    }
    fn json_name(&self) -> &str {
        self.json_name.as_ref()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::FieldOptions> {
        self.options.as_deref()
    }
    fn proto3_optional(&self) -> bool {
        self.proto3_optional.clone()
    }
}
pub trait FieldDescriptorProtoTrait {
    fn name(&self) -> &str;
    fn number(&self) -> i32;
    fn label(&self) -> ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>;
    fn type_(&self) -> ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>;
    fn type_name(&self) -> &str;
    fn extendee(&self) -> &str;
    fn default_value(&self) -> &str;
    fn oneof_index(&self) -> i32;
    fn json_name(&self) -> &str;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::FieldOptions>;
    fn proto3_optional(&self) -> bool;
}
pub trait FieldDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn number_mut(&self) -> &mut i32;
    fn label_mut(&self) -> &mut ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>;
    fn type__mut(&self) -> &mut ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>;
    fn type_name_mut(&self) -> &mut String;
    fn extendee_mut(&self) -> &mut String;
    fn default_value_mut(&self) -> &mut String;
    fn oneof_index_mut(&self) -> &mut i32;
    fn json_name_mut(&self) -> &mut String;
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::FieldOptions>;
    fn proto3_optional_mut(&self) -> &mut bool;
}
pub mod field_descriptor_proto {
#[derive(Debug, Clone)]
pub enum Label {
    LabelOptional = 1,
    LabelRequired = 2,
    LabelRepeated = 3,
}
impl ::std::convert::TryFrom<i32> for Label {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            1 => Ok(Self::LabelOptional),
            2 => Ok(Self::LabelRequired),
            3 => Ok(Self::LabelRepeated),
            x => Err(x),
        }
    }
}
impl ::std::convert::Into<i32> for Label {
    fn into(self) -> i32 {
        self as i32
    }
}
#[derive(Debug, Clone)]
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
impl ::std::convert::TryFrom<i32> for Type {
    type Error = i32;
    fn try_from(val: i32) -> ::std::result::Result<Self, i32> {
        match val {
            1 => Ok(Self::TypeDouble),
            2 => Ok(Self::TypeFloat),
            3 => Ok(Self::TypeInt64),
            4 => Ok(Self::TypeUint64),
            5 => Ok(Self::TypeInt32),
            6 => Ok(Self::TypeFixed64),
            7 => Ok(Self::TypeFixed32),
            8 => Ok(Self::TypeBool),
            9 => Ok(Self::TypeString),
            10 => Ok(Self::TypeGroup),
            11 => Ok(Self::TypeMessage),
            12 => Ok(Self::TypeBytes),
            13 => Ok(Self::TypeUint32),
            14 => Ok(Self::TypeEnum),
            15 => Ok(Self::TypeSfixed32),
            16 => Ok(Self::TypeSfixed64),
            17 => Ok(Self::TypeSint32),
            18 => Ok(Self::TypeSint64),
            x => Err(x),
        }
    }
}
impl ::std::convert::Into<i32> for Type {
    fn into(self) -> i32 {
        self as i32
    }
}
} // mod field_descriptor_proto

#[derive(Debug, Clone)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl ExtensionRangeOptions {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ExtensionRangeOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ExtensionRangeOptions {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for ExtensionRangeOptions {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for ExtensionRangeOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for ExtensionRangeOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ExtensionRangeOptionsTrait for ExtensionRangeOptions {
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ExtensionRangeOptionsBumpalo<'b> {
    pub uninterpreted_option: ::bumpalo::collections::Vec<super::super::google::protobuf::UninterpretedOption>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ExtensionRangeOptionsBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ExtensionRangeOptionsBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ExtensionRangeOptionsBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                999 => {
                    let msg = self.uninterpreted_option.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                999 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for ExtensionRangeOptionsBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for ExtensionRangeOptionsBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for ExtensionRangeOptionsBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ExtensionRangeOptionsTrait for ExtensionRangeOptionsBumpalo {
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
pub trait ExtensionRangeOptionsTrait {
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait ExtensionRangeOptionsMutTrait {
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::UninterpretedOption>>;
    // We need more! 
}

#[derive(Debug, Clone)]
pub struct DescriptorProto {
    pub name: ::std::string::String,
    pub field: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    pub extension_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec<super::super::google::protobuf::OneofDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MessageOptions>>,
    pub reserved_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl DescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            field: ::std::default::Default::default(),
            extension: ::std::default::Default::default(),
            nested_type: ::std::default::Default::default(),
            enum_type: ::std::default::Default::default(),
            extension_range: ::std::default::Default::default(),
            oneof_decl: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            reserved_range: ::std::default::Default::default(),
            reserved_name: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut DescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.field.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                6 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.nested_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                4 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let msg = self.extension_range.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                8 => {
                    let msg = self.oneof_decl.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                7 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                9 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                10 => {
                    *self.reserved_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for DescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.field.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                6 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self.nested_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                4 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    let msg = self.extension_range.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                8 => {
                    let msg = self.oneof_decl.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                7 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                9 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                10 => {
                    *self.reserved_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for DescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for DescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.field.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in self.extension.iter_for_ser() {
            serializer.serialize_message_twice(6, msg)?;
        }
        for msg in self.nested_type.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        for msg in self.enum_type.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        for msg in self.extension_range.iter_for_ser() {
            serializer.serialize_message_twice(5, msg)?;
        }
        for msg in self.oneof_decl.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(7, msg)?;
        }
        for msg in self.reserved_range.iter_for_ser() {
            serializer.serialize_message_twice(9, msg)?;
        }
        for string in self.reserved_name.iter_for_ser() {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for DescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl DescriptorProtoTrait for DescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn for_each_field<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto) {
        for item in (self.field).iter() {
            (f)(item);
        }
    }
    fn field_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.field.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_> {
        self.field.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn for_each_nested_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::DescriptorProto) {
        for item in (self.nested_type).iter() {
            (f)(item);
        }
    }
    fn nested_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::DescriptorProto>> {
        ::std::boxed::Box::new(self.nested_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_> {
        self.nested_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumDescriptorProto) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_extension_range<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::descriptor_proto::ExtensionRange) {
        for item in (self.extension_range).iter() {
            (f)(item);
        }
    }
    fn extension_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::descriptor_proto::ExtensionRange>> {
        ::std::boxed::Box::new(self.extension_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::descriptor_proto::ExtensionRange>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_> {
        self.extension_range.iter()
    }
    fn for_each_oneof_decl<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::OneofDescriptorProto) {
        for item in (self.oneof_decl).iter() {
            (f)(item);
        }
    }
    fn oneof_decl_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::OneofDescriptorProto>> {
        ::std::boxed::Box::new(self.oneof_decl.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::OneofDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_> {
        self.oneof_decl.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::MessageOptions> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::descriptor_proto::ReservedRange) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::descriptor_proto::ReservedRange>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::descriptor_proto::ReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct DescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub field: ::bumpalo::collections::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub extension: ::bumpalo::collections::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub nested_type: ::bumpalo::collections::Vec<super::super::google::protobuf::DescriptorProto>,
    pub enum_type: ::bumpalo::collections::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    pub extension_range: ::bumpalo::collections::Vec<super::super::google::protobuf::descriptor_proto::ExtensionRange>,
    pub oneof_decl: ::bumpalo::collections::Vec<super::super::google::protobuf::OneofDescriptorProto>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::MessageOptions>>,
    pub reserved_range: ::bumpalo::collections::Vec<super::super::google::protobuf::descriptor_proto::ReservedRange>,
    pub reserved_name: ::bumpalo::collections::Vec<::bumpalo::collections::String>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> DescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            field: ::std::default::Default::default(),
            extension: ::std::default::Default::default(),
            nested_type: ::std::default::Default::default(),
            enum_type: ::std::default::Default::default(),
            extension_range: ::std::default::Default::default(),
            oneof_decl: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            reserved_range: ::std::default::Default::default(),
            reserved_name: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut DescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.field.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                6 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = self.nested_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                4 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let msg = self.extension_range.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                8 => {
                    let msg = self.oneof_decl.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                7 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                9 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                10 => {
                    *self.reserved_name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for DescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.field.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                6 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                3 => {
                    let msg = self.nested_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                4 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    let msg = self.extension_range.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                8 => {
                    let msg = self.oneof_decl.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                7 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                9 => {
                    let msg = self.reserved_range.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                10 => {
                    *self.reserved_name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for DescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for DescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.field.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in self.extension.iter_for_ser() {
            serializer.serialize_message_twice(6, msg)?;
        }
        for msg in self.nested_type.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        for msg in self.enum_type.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        for msg in self.extension_range.iter_for_ser() {
            serializer.serialize_message_twice(5, msg)?;
        }
        for msg in self.oneof_decl.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(7, msg)?;
        }
        for msg in self.reserved_range.iter_for_ser() {
            serializer.serialize_message_twice(9, msg)?;
        }
        for string in self.reserved_name.iter_for_ser() {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for DescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl DescriptorProtoTrait for DescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn for_each_field<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto) {
        for item in (self.field).iter() {
            (f)(item);
        }
    }
    fn field_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.field.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_> {
        self.field.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn for_each_nested_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::DescriptorProto) {
        for item in (self.nested_type).iter() {
            (f)(item);
        }
    }
    fn nested_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::DescriptorProto>> {
        ::std::boxed::Box::new(self.nested_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_> {
        self.nested_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumDescriptorProto) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_extension_range<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::descriptor_proto::ExtensionRange) {
        for item in (self.extension_range).iter() {
            (f)(item);
        }
    }
    fn extension_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::descriptor_proto::ExtensionRange>> {
        ::std::boxed::Box::new(self.extension_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::descriptor_proto::ExtensionRange>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_> {
        self.extension_range.iter()
    }
    fn for_each_oneof_decl<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::OneofDescriptorProto) {
        for item in (self.oneof_decl).iter() {
            (f)(item);
        }
    }
    fn oneof_decl_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::OneofDescriptorProto>> {
        ::std::boxed::Box::new(self.oneof_decl.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::OneofDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_> {
        self.oneof_decl.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::MessageOptions> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::descriptor_proto::ReservedRange) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::descriptor_proto::ReservedRange>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::descriptor_proto::ReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
pub trait DescriptorProtoTrait {
    fn name(&self) -> &str;
    fn for_each_field<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto);
    fn field_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a>: Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_>;
    fn for_each_extension<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto);
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_>;
    fn for_each_nested_type<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::DescriptorProto);
    fn nested_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::DescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a>: Iterator<Item=&'a super::super::google::protobuf::DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_>;
    fn for_each_enum_type<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumDescriptorProto);
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a>: Iterator<Item=&'a super::super::google::protobuf::EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_>;
    fn for_each_extension_range<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::descriptor_proto::ExtensionRange);
    fn extension_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::descriptor_proto::ExtensionRange>>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a>: Iterator<Item=&'a super::super::google::protobuf::descriptor_proto::ExtensionRange>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_>;
    fn for_each_oneof_decl<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::OneofDescriptorProto);
    fn oneof_decl_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::OneofDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a>: Iterator<Item=&'a super::super::google::protobuf::OneofDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_>;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::MessageOptions>;
    fn for_each_reserved_range<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::descriptor_proto::ReservedRange);
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::descriptor_proto::ReservedRange>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a>: Iterator<Item=&'a super::super::google::protobuf::descriptor_proto::ReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_>;
    fn for_each_reserved_name<F>(&self, f: F)
    where
        F: FnMut(&str);
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a>: Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_>;
}
pub trait DescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn for_each_field_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FieldDescriptorProto);
    fn field_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::FieldDescriptorProto>>;
    // We need more! 
    fn for_each_extension_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FieldDescriptorProto);
    fn extension_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::FieldDescriptorProto>>;
    // We need more! 
    fn for_each_nested_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::DescriptorProto);
    fn nested_type_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::DescriptorProto>>;
    // We need more! 
    fn for_each_enum_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::EnumDescriptorProto);
    fn enum_type_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::EnumDescriptorProto>>;
    // We need more! 
    fn for_each_extension_range_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::descriptor_proto::ExtensionRange);
    fn extension_range_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::descriptor_proto::ExtensionRange>>;
    // We need more! 
    fn for_each_oneof_decl_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::OneofDescriptorProto);
    fn oneof_decl_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::OneofDescriptorProto>>;
    // We need more! 
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::MessageOptions>;
    fn for_each_reserved_range_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::descriptor_proto::ReservedRange);
    fn reserved_range_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::descriptor_proto::ReservedRange>>;
    // We need more! 
    fn for_each_reserved_name_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn reserved_name_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut String>>;
    // We need more! 
}
pub mod descriptor_proto {

#[derive(Debug, Clone)]
pub struct ReservedRange {
    pub start: i32,
    pub end: i32,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl ReservedRange {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            start: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ReservedRange {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ReservedRange {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ReservedRange {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for ReservedRange {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for ReservedRange {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.start.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}

impl ::puroro::Serializable for ReservedRange {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ReservedRangeTrait for ReservedRange {
    fn start(&self) -> i32 {
        self.start.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ReservedRangeBumpalo<'b> {
    pub start: i32,
    pub end: i32,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ReservedRangeBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            start: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ReservedRangeBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ReservedRangeBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for ReservedRangeBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for ReservedRangeBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.start.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for ReservedRangeBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ReservedRangeTrait for ReservedRangeBumpalo {
    fn start(&self) -> i32 {
        self.start.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
}
pub trait ReservedRangeTrait {
    fn start(&self) -> i32;
    fn end(&self) -> i32;
}
pub trait ReservedRangeMutTrait {
    fn start_mut(&self) -> &mut i32;
    fn end_mut(&self) -> &mut i32;
}

#[derive(Debug, Clone)]
pub struct ExtensionRange {
    pub start: i32,
    pub end: i32,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::ExtensionRangeOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl ExtensionRange {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            start: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ExtensionRange {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ExtensionRange {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ExtensionRange {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for ExtensionRange {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for ExtensionRange {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.start.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for ExtensionRange {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ExtensionRangeTrait for ExtensionRange {
    fn start(&self) -> i32 {
        self.start.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
    fn options(&self) -> ::std::option::Option<&super::super::super::google::protobuf::ExtensionRangeOptions> {
        self.options.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ExtensionRangeBumpalo<'b> {
    pub start: i32,
    pub end: i32,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::super::google::protobuf::ExtensionRangeOptions>>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ExtensionRangeBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            start: ::std::default::Default::default(),
            end: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut ExtensionRangeBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for ExtensionRangeBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.start.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                2 => {
                    *self.end.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                }
                2 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                }
                3 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for ExtensionRangeBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for ExtensionRangeBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            1, 
            self.start.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            2, 
            self.end.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for ExtensionRangeBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl ExtensionRangeTrait for ExtensionRangeBumpalo {
    fn start(&self) -> i32 {
        self.start.clone()
    }
    fn end(&self) -> i32 {
        self.end.clone()
    }
    fn options(&self) -> ::std::option::Option<&super::super::super::google::protobuf::ExtensionRangeOptions> {
        self.options.as_deref()
    }
}
pub trait ExtensionRangeTrait {
    fn start(&self) -> i32;
    fn end(&self) -> i32;
    fn options(&self) -> ::std::option::Option<&super::super::super::google::protobuf::ExtensionRangeOptions>;
}
pub trait ExtensionRangeMutTrait {
    fn start_mut(&self) -> &mut i32;
    fn end_mut(&self) -> &mut i32;
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::super::google::protobuf::ExtensionRangeOptions>;
}
} // mod descriptor_proto

#[derive(Debug, Clone)]
pub struct FileDescriptorProto {
    pub name: ::std::string::String,
    pub package: ::std::string::String,
    pub dependency: ::std::vec::Vec<::std::string::String>,
    pub public_dependency: ::std::vec::Vec<i32>,
    pub weak_dependency: ::std::vec::Vec<i32>,
    pub message_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    pub service: ::std::vec::Vec<super::super::google::protobuf::ServiceDescriptorProto>,
    pub extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FileOptions>>,
    pub source_code_info: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::SourceCodeInfo>>,
    pub syntax: ::std::string::String,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl FileDescriptorProto {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            package: ::std::default::Default::default(),
            dependency: ::std::default::Default::default(),
            public_dependency: ::std::default::Default::default(),
            weak_dependency: ::std::default::Default::default(),
            message_type: ::std::default::Default::default(),
            enum_type: ::std::default::Default::default(),
            service: ::std::default::Default::default(),
            extension: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            source_code_info: ::std::default::Default::default(),
            syntax: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FileDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.public_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                11 => {
                    *self.weak_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.dependency.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.public_dependency, first, iter);
                }
                11 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak_dependency, first, iter);
                }
                4 => {
                    let msg = self.message_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                6 => {
                    let msg = self.service.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                7 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                9 => {
                    let msg = self.source_code_info.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                12 => {
                    *self.syntax.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FileDescriptorProto {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.public_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                11 => {
                    *self.weak_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.dependency.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.public_dependency, first, iter);
                }
                11 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak_dependency, first, iter);
                }
                4 => {
                    let msg = self.message_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                6 => {
                    let msg = self.service.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                7 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                9 => {
                    let msg = self.source_code_info.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                12 => {
                    *self.syntax.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for FileDescriptorProto {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for FileDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.package.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.dependency.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            10, 
            self.public_dependency.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            11, 
            self.weak_dependency.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.message_type.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        for msg in self.enum_type.iter_for_ser() {
            serializer.serialize_message_twice(5, msg)?;
        }
        for msg in self.service.iter_for_ser() {
            serializer.serialize_message_twice(6, msg)?;
        }
        for msg in self.extension.iter_for_ser() {
            serializer.serialize_message_twice(7, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        for msg in self.source_code_info.iter_for_ser() {
            serializer.serialize_message_twice(9, msg)?;
        }
        for string in self.syntax.iter_for_ser() {
            serializer.serialize_bytes_twice(12, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for FileDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileDescriptorProtoTrait for FileDescriptorProto {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn package(&self) -> &str {
        self.package.as_ref()
    }
    fn for_each_dependency<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.dependency).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.dependency.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn dependency_iter(&self) -> Self::DependencyIter<'_> {
        self.dependency.iter().map(|v| v.as_ref())
    }
    fn for_each_public_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.public_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn public_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.public_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn public_dependency_iter(&self) -> Self::PublicDependencyIter<'_> {
        self.public_dependency.iter().cloned()
    }
    fn for_each_weak_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.weak_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn weak_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.weak_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_> {
        self.weak_dependency.iter().cloned()
    }
    fn for_each_message_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::DescriptorProto) {
        for item in (self.message_type).iter() {
            (f)(item);
        }
    }
    fn message_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::DescriptorProto>> {
        ::std::boxed::Box::new(self.message_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_> {
        self.message_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumDescriptorProto) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_service<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::ServiceDescriptorProto) {
        for item in (self.service).iter() {
            (f)(item);
        }
    }
    fn service_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::ServiceDescriptorProto>> {
        ::std::boxed::Box::new(self.service.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::ServiceDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_> {
        self.service.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::FileOptions> {
        self.options.as_deref()
    }
    fn source_code_info(&self) -> ::std::option::Option<&super::super::google::protobuf::SourceCodeInfo> {
        self.source_code_info.as_deref()
    }
    fn syntax(&self) -> &str {
        self.syntax.as_ref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FileDescriptorProtoBumpalo<'b> {
    pub name: ::bumpalo::collections::String,
    pub package: ::bumpalo::collections::String,
    pub dependency: ::bumpalo::collections::Vec<::bumpalo::collections::String>,
    pub public_dependency: ::bumpalo::collections::Vec<i32>,
    pub weak_dependency: ::bumpalo::collections::Vec<i32>,
    pub message_type: ::bumpalo::collections::Vec<super::super::google::protobuf::DescriptorProto>,
    pub enum_type: ::bumpalo::collections::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    pub service: ::bumpalo::collections::Vec<super::super::google::protobuf::ServiceDescriptorProto>,
    pub extension: ::bumpalo::collections::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::FileOptions>>,
    pub source_code_info: ::std::option::Option<::bumpalo::boxed::Box<super::super::google::protobuf::SourceCodeInfo>>,
    pub syntax: ::bumpalo::collections::String,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> FileDescriptorProtoBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            package: ::std::default::Default::default(),
            dependency: ::std::default::Default::default(),
            public_dependency: ::std::default::Default::default(),
            weak_dependency: ::std::default::Default::default(),
            message_type: ::std::default::Default::default(),
            enum_type: ::std::default::Default::default(),
            service: ::std::default::Default::default(),
            extension: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            source_code_info: ::std::default::Default::default(),
            syntax: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FileDescriptorProtoBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.public_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                11 => {
                    *self.weak_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.package.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.dependency.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.public_dependency, first, iter);
                }
                11 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak_dependency, first, iter);
                }
                4 => {
                    let msg = self.message_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                6 => {
                    let msg = self.service.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                7 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                9 => {
                    let msg = self.source_code_info.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                12 => {
                    *self.syntax.push_and_get_mut()
                        = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FileDescriptorProtoBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self.public_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                11 => {
                    *self.weak_dependency.push_and_get_mut() = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    *self.name.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *self.package.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *self.dependency.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.public_dependency, first, iter);
                }
                11 => {
                    let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter.next().ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.weak_dependency, first, iter);
                }
                4 => {
                    let msg = self.message_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                5 => {
                    let msg = self.enum_type.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                6 => {
                    let msg = self.service.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                7 => {
                    let msg = self.extension.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                8 => {
                    let msg = self.options.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                9 => {
                    let msg = self.source_code_info.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                12 => {
                    *self.syntax.push_and_get_mut()
                        = bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for FileDescriptorProtoBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for FileDescriptorProtoBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.package.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.dependency.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            10, 
            self.public_dependency.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro::tags::Int32, _>(
            11, 
            self.weak_dependency.iter_for_ser()
                .cloned()
                .map(|v| Ok(v)))?;
        for msg in self.message_type.iter_for_ser() {
            serializer.serialize_message_twice(4, msg)?;
        }
        for msg in self.enum_type.iter_for_ser() {
            serializer.serialize_message_twice(5, msg)?;
        }
        for msg in self.service.iter_for_ser() {
            serializer.serialize_message_twice(6, msg)?;
        }
        for msg in self.extension.iter_for_ser() {
            serializer.serialize_message_twice(7, msg)?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        for msg in self.source_code_info.iter_for_ser() {
            serializer.serialize_message_twice(9, msg)?;
        }
        for string in self.syntax.iter_for_ser() {
            serializer.serialize_bytes_twice(12, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for FileDescriptorProtoBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl FileDescriptorProtoTrait for FileDescriptorProtoBumpalo {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn package(&self) -> &str {
        self.package.as_ref()
    }
    fn for_each_dependency<F>(&self, mut f: F)
    where
        F: FnMut(&str) {
        for item in (self.dependency).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>> {
        ::std::boxed::Box::new(self.dependency.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a> = impl Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn dependency_iter(&self) -> Self::DependencyIter<'_> {
        self.dependency.iter().map(|v| v.as_ref())
    }
    fn for_each_public_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.public_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn public_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.public_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn public_dependency_iter(&self) -> Self::PublicDependencyIter<'_> {
        self.public_dependency.iter().cloned()
    }
    fn for_each_weak_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32) {
        for item in (self.weak_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn weak_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>> {
        ::std::boxed::Box::new(self.weak_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a> = impl Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_> {
        self.weak_dependency.iter().cloned()
    }
    fn for_each_message_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::DescriptorProto) {
        for item in (self.message_type).iter() {
            (f)(item);
        }
    }
    fn message_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::DescriptorProto>> {
        ::std::boxed::Box::new(self.message_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_> {
        self.message_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumDescriptorProto) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_service<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::ServiceDescriptorProto) {
        for item in (self.service).iter() {
            (f)(item);
        }
    }
    fn service_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::ServiceDescriptorProto>> {
        ::std::boxed::Box::new(self.service.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::ServiceDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_> {
        self.service.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::FileOptions> {
        self.options.as_deref()
    }
    fn source_code_info(&self) -> ::std::option::Option<&super::super::google::protobuf::SourceCodeInfo> {
        self.source_code_info.as_deref()
    }
    fn syntax(&self) -> &str {
        self.syntax.as_ref()
    }
}
pub trait FileDescriptorProtoTrait {
    fn name(&self) -> &str;
    fn package(&self) -> &str;
    fn for_each_dependency<F>(&self, f: F)
    where
        F: FnMut(&str);
    fn dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&str>>;
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a>: Iterator<Item=&'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn dependency_iter(&self) -> Self::DependencyIter<'_>;
    fn for_each_public_dependency<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn public_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a>: Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn public_dependency_iter(&self) -> Self::PublicDependencyIter<'_>;
    fn for_each_weak_dependency<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn weak_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a>: Iterator<Item=i32>;
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_>;
    fn for_each_message_type<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::DescriptorProto);
    fn message_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::DescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a>: Iterator<Item=&'a super::super::google::protobuf::DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_>;
    fn for_each_enum_type<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::EnumDescriptorProto);
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::EnumDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a>: Iterator<Item=&'a super::super::google::protobuf::EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_>;
    fn for_each_service<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::ServiceDescriptorProto);
    fn service_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::ServiceDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a>: Iterator<Item=&'a super::super::google::protobuf::ServiceDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_>;
    fn for_each_extension<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::FieldDescriptorProto);
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FieldDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a>: Iterator<Item=&'a super::super::google::protobuf::FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_>;
    fn options(&self) -> ::std::option::Option<&super::super::google::protobuf::FileOptions>;
    fn source_code_info(&self) -> ::std::option::Option<&super::super::google::protobuf::SourceCodeInfo>;
    fn syntax(&self) -> &str;
}
pub trait FileDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn package_mut(&self) -> &mut String;
    fn for_each_dependency_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn dependency_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut String>>;
    // We need more! 
    fn for_each_public_dependency_mut<F>(&self, f: F)
    where
        F: FnMut(&mut i32);
    fn public_dependency_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut i32>>;
    // We need more! 
    fn for_each_weak_dependency_mut<F>(&self, f: F)
    where
        F: FnMut(&mut i32);
    fn weak_dependency_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut i32>>;
    // We need more! 
    fn for_each_message_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::DescriptorProto);
    fn message_type_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::DescriptorProto>>;
    // We need more! 
    fn for_each_enum_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::EnumDescriptorProto);
    fn enum_type_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::EnumDescriptorProto>>;
    // We need more! 
    fn for_each_service_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::ServiceDescriptorProto);
    fn service_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::ServiceDescriptorProto>>;
    // We need more! 
    fn for_each_extension_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FieldDescriptorProto);
    fn extension_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::FieldDescriptorProto>>;
    // We need more! 
    fn options_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::FileOptions>;
    fn source_code_info_mut(&self) -> ::std::option::Option<&mut super::super::google::protobuf::SourceCodeInfo>;
    fn syntax_mut(&self) -> &mut String;
}

#[derive(Debug, Clone)]
pub struct FileDescriptorSet {
    pub file: ::std::vec::Vec<super::super::google::protobuf::FileDescriptorProto>,
    puroro_internal: ::puroro::helpers::InternalDataForNormalStruct,
}

impl FileDescriptorSet {
    pub fn new() -> Self {
        use ::std::convert::TryInto;
        Self {
            file: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FileDescriptorSet {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = self.file.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl<'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FileDescriptorSet {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let msg = self.file.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}

impl ::puroro::Deserializable for FileDescriptorSet {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}

impl ::puroro::serializer::Serializable for FileDescriptorSet {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.file.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for FileDescriptorSet {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileDescriptorSetTrait for FileDescriptorSet {
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FileDescriptorProto) {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FileDescriptorProto>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FileDescriptorSetBumpalo<'b> {
    pub file: ::bumpalo::collections::Vec<super::super::google::protobuf::FileDescriptorProto>,
    puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct<'b>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> FileDescriptorSetBumpalo<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump
) -> Self {
        use ::std::convert::TryInto;
        Self {
            file: ::std::default::Default::default(),
            puroro_internal: ::puroro::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deserializer::MessageDeserializeEventHandler for &'a mut FileDescriptorSetBumpalo<'b> {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro::deserializer::bytes::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro::types::FieldData<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = self.file.push_and_get_mut2(&self.puroro_internal);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b, 'a> ::puroro::deser::DeserializeMessageFromBytesEventHandler for FileDescriptorSetBumpalo<'b> {
    fn met_field<B: ::puroro::deser::BytesIter>(
        &mut self,
        field: ::puroro::types::FieldData<B>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        use ::puroro::helpers::MaybeRepeatedField;
        use ::puroro::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::LengthDelimited(mut bytes_iter) => match field_number {
                1 => {
                    let msg = self.file.push_and_get_mut2(&self.puroro_internal);
                    bytes_iter.deser_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits32(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
            ::puroro::types::FieldData::Bits64(bytes) => match field_number {
                1 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            }
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Deserializable for FileDescriptorSetBumpalo<'b> {
    fn deser_from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            &mut self, iter: I) -> ::puroro::Result<()> {
        use ::puroro::deserializer::Deserializer;
        let deserializer = ::puroro::deserializer::bytes::deserializer_from_bytes(iter);
        deserializer.deserialize(self)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::serializer::Serializable for FileDescriptorSetBumpalo<'b> {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro::helpers::MaybeRepeatedField;
        for msg in self.file.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'b> ::puroro::Serializable for FileDescriptorSetBumpalo<'b> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl FileDescriptorSetTrait for FileDescriptorSetBumpalo {
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&super::super::google::protobuf::FileDescriptorProto) {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FileDescriptorProto>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item=&'a super::super::google::protobuf::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
pub trait FileDescriptorSetTrait {
    fn for_each_file<F>(&self, f: F)
    where
        F: FnMut(&super::super::google::protobuf::FileDescriptorProto);
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&super::super::google::protobuf::FileDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a>: Iterator<Item=&'a super::super::google::protobuf::FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_>;
}
pub trait FileDescriptorSetMutTrait {
    fn for_each_file_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FileDescriptorProto);
    fn file_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::FileDescriptorProto>>;
    // We need more! 
}

pub mod compiler;
