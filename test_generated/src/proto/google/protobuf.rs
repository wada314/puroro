#![allow(unused_variables)]
#![allow(unused_imports)]

#[derive(Debug, Clone)]
pub struct GeneratedCodeInfo {
    pub annotation: ::std::vec::Vec<generated_code_info::Annotation>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl GeneratedCodeInfo {
    pub fn new() -> Self {
        Self {
            annotation: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for GeneratedCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let msg = self.annotation.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for GeneratedCodeInfo {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for GeneratedCodeInfo {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.annotation.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for GeneratedCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl GeneratedCodeInfoTrait for GeneratedCodeInfo {
    type AnnotationType = generated_code_info::Annotation;
    fn for_each_annotation<F>(&self, mut f: F)
    where
        F: FnMut(&'_ generated_code_info::Annotation),
    {
        for item in (self.annotation).iter() {
            (f)(item);
        }
    }
    fn annotation_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ generated_code_info::Annotation>> {
        ::std::boxed::Box::new(self.annotation.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a> = impl Iterator<Item = &'a generated_code_info::Annotation>;
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_> {
        self.annotation.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for GeneratedCodeInfo {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct GeneratedCodeInfoBumpalo<'bump> {
    pub annotation:
        ::bumpalo::collections::Vec<'bump, generated_code_info::AnnotationBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> GeneratedCodeInfoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            annotation: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for GeneratedCodeInfoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let msg = self.annotation.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for GeneratedCodeInfoBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for GeneratedCodeInfoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.annotation.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for GeneratedCodeInfoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> GeneratedCodeInfoTrait for GeneratedCodeInfoBumpalo<'bump> {
    type AnnotationType = generated_code_info::AnnotationBumpalo<'bump>;
    fn for_each_annotation<F>(&self, mut f: F)
    where
        F: FnMut(&'_ generated_code_info::Annotation),
    {
        for item in (self.annotation).iter() {
            (f)(item);
        }
    }
    fn annotation_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ generated_code_info::Annotation>> {
        ::std::boxed::Box::new(self.annotation.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a> = impl Iterator<Item = &'a generated_code_info::Annotation>;
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_> {
        self.annotation.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for GeneratedCodeInfoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait GeneratedCodeInfoTrait {
    type AnnotationType: generated_code_info::AnnotationTrait;
    fn for_each_annotation<F>(&self, f: F)
    where
        F: FnMut(&'_ generated_code_info::Annotation);
    fn annotation_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ generated_code_info::Annotation>>;
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a>: Iterator<Item = &'a generated_code_info::Annotation>;
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_>;
}
pub trait GeneratedCodeInfoMutTrait {
    fn for_each_annotation_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::generated_code_info::Annotation);
    fn annotation_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_
            + Iterator<Item = &mut super::super::google::protobuf::generated_code_info::Annotation>,
    >;
    // We need more!
}
pub mod generated_code_info {

    #[derive(Debug, Clone)]
    pub struct Annotation {
        pub path: ::std::vec::Vec<i32>,
        pub source_file: ::std::string::String,
        pub begin: i32,
        pub end: i32,
        puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
    }

    impl Annotation {
        pub fn new() -> Self {
            Self {
                path: ::puroro_internal::helpers::FieldNew::new(),
                source_file: ::puroro_internal::helpers::FieldNew::new(),
                begin: ::puroro_internal::helpers::FieldNew::new(),
                end: ::puroro_internal::helpers::FieldNew::new(),
                puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
            }
        }
    }

    impl ::std::default::Default for Annotation {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::puroro_internal::deser::DeserializableMessageFromIter for Annotation {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.path.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => {
                        *self.begin.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    4 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                        }
                        2 => {
                            *self.source_file.push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        3 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.begin, first, iter);
                        }
                        4 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro_internal::deser::DeserializableFromIter for Annotation {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }

    impl ::puroro_internal::serializer::Serializable for Annotation {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.path.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            for string in self.source_file.iter_for_ser() {
                serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
            }
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                3,
                self.begin.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                4,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }

    impl ::puroro::Serializable for Annotation {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }

    impl AnnotationTrait for Annotation {
        fn for_each_path<F>(&self, mut f: F)
        where
            F: FnMut(i32),
        {
            for item in (self.path).iter().cloned() {
                (f)(item);
            }
        }
        fn path_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
            ::std::boxed::Box::new(self.path.iter().cloned())
        }
        #[cfg(feature = "puroro-nightly")]
        type PathIter<'a> = impl Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn path_iter(&self) -> Self::PathIter<'_> {
            self.path.iter().cloned()
        }
        fn source_file(&self) -> &'_ str {
            self.source_file.as_ref()
        }
        fn begin(&self) -> i32 {
            self.begin.clone()
        }
        fn end(&self) -> i32 {
            self.end.clone()
        }
    }
    impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Annotation {
        fn new() -> Self {
            Default::default()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct AnnotationBumpalo<'bump> {
        pub path: ::bumpalo::collections::Vec<'bump, i32>,
        pub source_file: ::bumpalo::collections::String<'bump>,
        pub begin: i32,
        pub end: i32,
        puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> AnnotationBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
            Self {
                path: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                source_file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                begin: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(
                    bump,
                ),
            }
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for AnnotationBumpalo<'bump> {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.path.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => {
                        *self.begin.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    4 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                        }
                        2 => {
                            *self.source_file.push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        3 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.begin, first, iter);
                        }
                        4 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 3 | 4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableFromIter for AnnotationBumpalo<'bump> {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::serializer::Serializable for AnnotationBumpalo<'bump> {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.path.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            for string in self.source_file.iter_for_ser() {
                serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
            }
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                3,
                self.begin.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                4,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::Serializable for AnnotationBumpalo<'bump> {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> AnnotationTrait for AnnotationBumpalo<'bump> {
        fn for_each_path<F>(&self, mut f: F)
        where
            F: FnMut(i32),
        {
            for item in (self.path).iter().cloned() {
                (f)(item);
            }
        }
        fn path_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
            ::std::boxed::Box::new(self.path.iter().cloned())
        }
        #[cfg(feature = "puroro-nightly")]
        type PathIter<'a> = impl Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn path_iter(&self) -> Self::PathIter<'_> {
            self.path.iter().cloned()
        }
        fn source_file(&self) -> &'_ str {
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
    impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for AnnotationBumpalo<'bump> {
        fn new() -> Self {
            unimplemented!()
        }
        fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }
    pub trait AnnotationTrait {
        fn for_each_path<F>(&self, f: F)
        where
            F: FnMut(i32);
        fn path_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>>;
        #[cfg(feature = "puroro-nightly")]
        type PathIter<'a>: Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn path_iter(&self) -> Self::PathIter<'_>;
        fn source_file(&'_ self) -> &'_ str;
        fn begin(&'_ self) -> i32;
        fn end(&'_ self) -> i32;
    }
    pub trait AnnotationMutTrait {
        fn for_each_path_mut<F>(&self, f: F)
        where
            F: FnMut(&mut i32);
        fn path_boxed_iter_mut(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut i32>>;
        // We need more!
        fn source_file_mut(&self) -> &mut String;
        fn begin_mut(&self) -> &mut i32;
        fn end_mut(&self) -> &mut i32;
    }
} // mod generated_code_info

#[derive(Debug, Clone)]
pub struct SourceCodeInfo {
    pub location: ::std::vec::Vec<source_code_info::Location>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl SourceCodeInfo {
    pub fn new() -> Self {
        Self {
            location: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for SourceCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let msg = self.location.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for SourceCodeInfo {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for SourceCodeInfo {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.location.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for SourceCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl SourceCodeInfoTrait for SourceCodeInfo {
    type LocationType = source_code_info::Location;
    fn for_each_location<F>(&self, mut f: F)
    where
        F: FnMut(&'_ source_code_info::Location),
    {
        for item in (self.location).iter() {
            (f)(item);
        }
    }
    fn location_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ source_code_info::Location>> {
        ::std::boxed::Box::new(self.location.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a> = impl Iterator<Item = &'a source_code_info::Location>;
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_> {
        self.location.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for SourceCodeInfo {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct SourceCodeInfoBumpalo<'bump> {
    pub location: ::bumpalo::collections::Vec<'bump, source_code_info::LocationBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SourceCodeInfoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            location: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for SourceCodeInfoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let msg = self.location.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for SourceCodeInfoBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for SourceCodeInfoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.location.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for SourceCodeInfoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SourceCodeInfoTrait for SourceCodeInfoBumpalo<'bump> {
    type LocationType = source_code_info::LocationBumpalo<'bump>;
    fn for_each_location<F>(&self, mut f: F)
    where
        F: FnMut(&'_ source_code_info::Location),
    {
        for item in (self.location).iter() {
            (f)(item);
        }
    }
    fn location_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ source_code_info::Location>> {
        ::std::boxed::Box::new(self.location.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a> = impl Iterator<Item = &'a source_code_info::Location>;
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_> {
        self.location.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for SourceCodeInfoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait SourceCodeInfoTrait {
    type LocationType: source_code_info::LocationTrait;
    fn for_each_location<F>(&self, f: F)
    where
        F: FnMut(&'_ source_code_info::Location);
    fn location_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ source_code_info::Location>>;
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a>: Iterator<Item = &'a source_code_info::Location>;
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_>;
}
pub trait SourceCodeInfoMutTrait {
    fn for_each_location_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::source_code_info::Location);
    fn location_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::source_code_info::Location>,
    >;
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
        puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
    }

    impl Location {
        pub fn new() -> Self {
            Self {
                path: ::puroro_internal::helpers::FieldNew::new(),
                span: ::puroro_internal::helpers::FieldNew::new(),
                leading_comments: ::puroro_internal::helpers::FieldNew::new(),
                trailing_comments: ::puroro_internal::helpers::FieldNew::new(),
                leading_detached_comments: ::puroro_internal::helpers::FieldNew::new(),
                puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
            }
        }
    }

    impl ::std::default::Default for Location {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::puroro_internal::deser::DeserializableMessageFromIter for Location {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.path.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.span.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.span, first, iter);
                        }
                        3 => {
                            *self
                                .leading_comments
                                .push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        4 => {
                            *self
                                .trailing_comments
                                .push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        6 => {
                            *self
                                .leading_detached_comments
                                .push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 3 | 4 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 3 | 4 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro_internal::deser::DeserializableFromIter for Location {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }

    impl ::puroro_internal::serializer::Serializable for Location {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.path.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.span.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
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
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }

    impl LocationTrait for Location {
        fn for_each_path<F>(&self, mut f: F)
        where
            F: FnMut(i32),
        {
            for item in (self.path).iter().cloned() {
                (f)(item);
            }
        }
        fn path_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
            ::std::boxed::Box::new(self.path.iter().cloned())
        }
        #[cfg(feature = "puroro-nightly")]
        type PathIter<'a> = impl Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn path_iter(&self) -> Self::PathIter<'_> {
            self.path.iter().cloned()
        }
        fn for_each_span<F>(&self, mut f: F)
        where
            F: FnMut(i32),
        {
            for item in (self.span).iter().cloned() {
                (f)(item);
            }
        }
        fn span_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
            ::std::boxed::Box::new(self.span.iter().cloned())
        }
        #[cfg(feature = "puroro-nightly")]
        type SpanIter<'a> = impl Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn span_iter(&self) -> Self::SpanIter<'_> {
            self.span.iter().cloned()
        }
        fn leading_comments(&self) -> &'_ str {
            self.leading_comments.as_ref()
        }
        fn trailing_comments(&self) -> &'_ str {
            self.trailing_comments.as_ref()
        }
        fn for_each_leading_detached_comments<F>(&self, mut f: F)
        where
            F: FnMut(&'_ str),
        {
            for item in (self.leading_detached_comments).iter().map(|v| v.as_ref()) {
                (f)(item);
            }
        }
        fn leading_detached_comments_boxed_iter(
            &self,
        ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
            ::std::boxed::Box::new(self.leading_detached_comments.iter().map(|v| v.as_ref()))
        }
        #[cfg(feature = "puroro-nightly")]
        type LeadingDetachedCommentsIter<'a> = impl Iterator<Item = &'a str>;
        #[cfg(feature = "puroro-nightly")]
        fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_> {
            self.leading_detached_comments.iter().map(|v| v.as_ref())
        }
    }
    impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Location {
        fn new() -> Self {
            Default::default()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct LocationBumpalo<'bump> {
        pub path: ::bumpalo::collections::Vec<'bump, i32>,
        pub span: ::bumpalo::collections::Vec<'bump, i32>,
        pub leading_comments: ::bumpalo::collections::String<'bump>,
        pub trailing_comments: ::bumpalo::collections::String<'bump>,
        pub leading_detached_comments:
            ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
        puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> LocationBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
            Self {
                path: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                span: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                leading_comments: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                trailing_comments: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                leading_detached_comments: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(
                    bump,
                ),
                puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(
                    bump,
                ),
            }
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for LocationBumpalo<'bump> {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.path.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.span.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.span, first, iter);
                        }
                        3 => {
                            *self
                                .leading_comments
                                .push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        4 => {
                            *self
                                .trailing_comments
                                .push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        6 => {
                            *self
                                .leading_detached_comments
                                .push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 3 | 4 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 3 | 4 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableFromIter for LocationBumpalo<'bump> {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::serializer::Serializable for LocationBumpalo<'bump> {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.path.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.span.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
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
    impl<'bump> ::puroro::Serializable for LocationBumpalo<'bump> {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> LocationTrait for LocationBumpalo<'bump> {
        fn for_each_path<F>(&self, mut f: F)
        where
            F: FnMut(i32),
        {
            for item in (self.path).iter().cloned() {
                (f)(item);
            }
        }
        fn path_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
            ::std::boxed::Box::new(self.path.iter().cloned())
        }
        #[cfg(feature = "puroro-nightly")]
        type PathIter<'a> = impl Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn path_iter(&self) -> Self::PathIter<'_> {
            self.path.iter().cloned()
        }
        fn for_each_span<F>(&self, mut f: F)
        where
            F: FnMut(i32),
        {
            for item in (self.span).iter().cloned() {
                (f)(item);
            }
        }
        fn span_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
            ::std::boxed::Box::new(self.span.iter().cloned())
        }
        #[cfg(feature = "puroro-nightly")]
        type SpanIter<'a> = impl Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn span_iter(&self) -> Self::SpanIter<'_> {
            self.span.iter().cloned()
        }
        fn leading_comments(&self) -> &'_ str {
            self.leading_comments.as_ref()
        }
        fn trailing_comments(&self) -> &'_ str {
            self.trailing_comments.as_ref()
        }
        fn for_each_leading_detached_comments<F>(&self, mut f: F)
        where
            F: FnMut(&'_ str),
        {
            for item in (self.leading_detached_comments).iter().map(|v| v.as_ref()) {
                (f)(item);
            }
        }
        fn leading_detached_comments_boxed_iter(
            &self,
        ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
            ::std::boxed::Box::new(self.leading_detached_comments.iter().map(|v| v.as_ref()))
        }
        #[cfg(feature = "puroro-nightly")]
        type LeadingDetachedCommentsIter<'a> = impl Iterator<Item = &'a str>;
        #[cfg(feature = "puroro-nightly")]
        fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_> {
            self.leading_detached_comments.iter().map(|v| v.as_ref())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for LocationBumpalo<'bump> {
        fn new() -> Self {
            unimplemented!()
        }
        fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }
    pub trait LocationTrait {
        fn for_each_path<F>(&self, f: F)
        where
            F: FnMut(i32);
        fn path_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>>;
        #[cfg(feature = "puroro-nightly")]
        type PathIter<'a>: Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn path_iter(&self) -> Self::PathIter<'_>;
        fn for_each_span<F>(&self, f: F)
        where
            F: FnMut(i32);
        fn span_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>>;
        #[cfg(feature = "puroro-nightly")]
        type SpanIter<'a>: Iterator<Item = i32>;
        #[cfg(feature = "puroro-nightly")]
        fn span_iter(&self) -> Self::SpanIter<'_>;
        fn leading_comments(&'_ self) -> &'_ str;
        fn trailing_comments(&'_ self) -> &'_ str;
        fn for_each_leading_detached_comments<F>(&self, f: F)
        where
            F: FnMut(&'_ str);
        fn leading_detached_comments_boxed_iter(
            &self,
        ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>>;
        #[cfg(feature = "puroro-nightly")]
        type LeadingDetachedCommentsIter<'a>: Iterator<Item = &'a str>;
        #[cfg(feature = "puroro-nightly")]
        fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_>;
    }
    pub trait LocationMutTrait {
        fn for_each_path_mut<F>(&self, f: F)
        where
            F: FnMut(&mut i32);
        fn path_boxed_iter_mut(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut i32>>;
        // We need more!
        fn for_each_span_mut<F>(&self, f: F)
        where
            F: FnMut(&mut i32);
        fn span_boxed_iter_mut(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut i32>>;
        // We need more!
        fn leading_comments_mut(&self) -> &mut String;
        fn trailing_comments_mut(&self) -> &mut String;
        fn for_each_leading_detached_comments_mut<F>(&self, f: F)
        where
            F: FnMut(&mut String);
        fn leading_detached_comments_boxed_iter_mut(
            &self,
        ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut String>>;
        // We need more!
    }
} // mod source_code_info

#[derive(Debug, Clone)]
pub struct UninterpretedOption {
    pub name: ::std::vec::Vec<uninterpreted_option::NamePart>,
    pub identifier_value: ::std::string::String,
    pub positive_int_value: u64,
    pub negative_int_value: i64,
    pub double_value: f64,
    pub string_value: ::std::vec::Vec<u8>,
    pub aggregate_value: ::std::string::String,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl UninterpretedOption {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            identifier_value: ::puroro_internal::helpers::FieldNew::new(),
            positive_int_value: ::puroro_internal::helpers::FieldNew::new(),
            negative_int_value: ::puroro_internal::helpers::FieldNew::new(),
            double_value: ::puroro_internal::helpers::FieldNew::new(),
            string_value: ::puroro_internal::helpers::FieldNew::new(),
            aggregate_value: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for UninterpretedOption {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    *self
                        .positive_int_value
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::UInt64>()?;
                }
                5 => {
                    *self
                        .negative_int_value
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    2 => {
                        let msg = self.name.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        *self
                            .identifier_value
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    4 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(
                            &mut self.positive_int_value,
                            first,
                            iter,
                        );
                    }
                    5 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Int64>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.negative_int_value,
                            first,
                            iter,
                        );
                    }
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    7 => {
                        *self.string_value.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.bytes().collect::<::puroro::Result<_>>()?;
                    }
                    8 => {
                        *self.aggregate_value.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 4 | 5 | 6 | 7 | 8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                6 => {
                    *self.double_value.push_and_get_mut(&self.puroro_internal) =
                        f64::from_le_bytes(bytes);
                }
                2 | 3 | 4 | 5 | 7 | 8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for UninterpretedOption {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for UninterpretedOption {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.name.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for string in self.identifier_value.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::UInt64, _>(
            4,
            self.positive_int_value
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int64, _>(
            5,
            self.negative_int_value
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
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
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl UninterpretedOptionTrait for UninterpretedOption {
    type NameType = uninterpreted_option::NamePart;
    fn for_each_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ uninterpreted_option::NamePart),
    {
        for item in (self.name).iter() {
            (f)(item);
        }
    }
    fn name_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ uninterpreted_option::NamePart>> {
        ::std::boxed::Box::new(self.name.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a> = impl Iterator<Item = &'a uninterpreted_option::NamePart>;
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_> {
        self.name.iter()
    }
    fn identifier_value(&self) -> &'_ str {
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
    fn string_value(&self) -> &'_ [u8] {
        self.string_value.as_ref()
    }
    fn aggregate_value(&self) -> &'_ str {
        self.aggregate_value.as_ref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for UninterpretedOption {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct UninterpretedOptionBumpalo<'bump> {
    pub name: ::bumpalo::collections::Vec<'bump, uninterpreted_option::NamePartBumpalo<'bump>>,
    pub identifier_value: ::bumpalo::collections::String<'bump>,
    pub positive_int_value: u64,
    pub negative_int_value: i64,
    pub double_value: f64,
    pub string_value: ::bumpalo::collections::Vec<'bump, u8>,
    pub aggregate_value: ::bumpalo::collections::String<'bump>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> UninterpretedOptionBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            identifier_value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            positive_int_value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            negative_int_value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            double_value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            string_value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            aggregate_value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for UninterpretedOptionBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    *self
                        .positive_int_value
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::UInt64>()?;
                }
                5 => {
                    *self
                        .negative_int_value
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    2 => {
                        let msg = self.name.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        *self
                            .identifier_value
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    4 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(
                            &mut self.positive_int_value,
                            first,
                            iter,
                        );
                    }
                    5 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Int64>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.negative_int_value,
                            first,
                            iter,
                        );
                    }
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    7 => {
                        *self.string_value.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.bytes().collect::<::puroro::Result<_>>()?;
                    }
                    8 => {
                        *self.aggregate_value.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 4 | 5 | 6 | 7 | 8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                6 => {
                    *self.double_value.push_and_get_mut(&self.puroro_internal) =
                        f64::from_le_bytes(bytes);
                }
                2 | 3 | 4 | 5 | 7 | 8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for UninterpretedOptionBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for UninterpretedOptionBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.name.iter_for_ser() {
            serializer.serialize_message_twice(2, msg)?;
        }
        for string in self.identifier_value.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::UInt64, _>(
            4,
            self.positive_int_value
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int64, _>(
            5,
            self.negative_int_value
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
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
impl<'bump> ::puroro::Serializable for UninterpretedOptionBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> UninterpretedOptionTrait for UninterpretedOptionBumpalo<'bump> {
    type NameType = uninterpreted_option::NamePartBumpalo<'bump>;
    fn for_each_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ uninterpreted_option::NamePart),
    {
        for item in (self.name).iter() {
            (f)(item);
        }
    }
    fn name_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ uninterpreted_option::NamePart>> {
        ::std::boxed::Box::new(self.name.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a> = impl Iterator<Item = &'a uninterpreted_option::NamePart>;
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_> {
        self.name.iter()
    }
    fn identifier_value(&self) -> &'_ str {
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
    fn string_value(&self) -> &'_ [u8] {
        self.string_value.as_ref()
    }
    fn aggregate_value(&self) -> &'_ str {
        self.aggregate_value.as_ref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for UninterpretedOptionBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait UninterpretedOptionTrait {
    type NameType: uninterpreted_option::NamePartTrait;
    fn for_each_name<F>(&self, f: F)
    where
        F: FnMut(&'_ uninterpreted_option::NamePart);
    fn name_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ uninterpreted_option::NamePart>>;
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a>: Iterator<Item = &'a uninterpreted_option::NamePart>;
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_>;
    fn identifier_value(&'_ self) -> &'_ str;
    fn positive_int_value(&'_ self) -> u64;
    fn negative_int_value(&'_ self) -> i64;
    fn double_value(&'_ self) -> f64;
    fn string_value(&'_ self) -> &'_ [u8];
    fn aggregate_value(&'_ self) -> &'_ str;
}
pub trait UninterpretedOptionMutTrait {
    fn for_each_name_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::uninterpreted_option::NamePart);
    fn name_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_
            + Iterator<Item = &mut super::super::google::protobuf::uninterpreted_option::NamePart>,
    >;
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
        puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
    }

    impl NamePart {
        pub fn new() -> Self {
            Self {
                name_part: ::puroro_internal::helpers::FieldNew::new(),
                is_extension: ::puroro_internal::helpers::FieldNew::new(),
                puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
            }
        }
    }

    impl ::std::default::Default for NamePart {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::puroro_internal::deser::DeserializableMessageFromIter for NamePart {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => {
                        *self.is_extension.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Bool>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            *self.name_part.push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
                                .map(|rv| {
                                    rv.and_then(|variant| {
                                        variant.to_native::<::puroro_internal::tags::Bool>()
                                    })
                                })
                                .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                            let mut iter = values.into_iter();
                            let first = iter
                                .next()
                                .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                            MaybeRepeatedVariantField::extend(&mut self.is_extension, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro_internal::deser::DeserializableFromIter for NamePart {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }

    impl ::puroro_internal::serializer::Serializable for NamePart {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            for string in self.name_part.iter_for_ser() {
                serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
            }
            serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
                2,
                self.is_extension.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }

    impl ::puroro::Serializable for NamePart {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }

    impl NamePartTrait for NamePart {
        fn name_part(&self) -> &'_ str {
            self.name_part.as_ref()
        }
        fn is_extension(&self) -> bool {
            self.is_extension.clone()
        }
    }
    impl<'a> ::puroro_internal::helpers::FieldNew<'a> for NamePart {
        fn new() -> Self {
            Default::default()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct NamePartBumpalo<'bump> {
        pub name_part: ::bumpalo::collections::String<'bump>,
        pub is_extension: bool,
        puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> NamePartBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
            Self {
                name_part: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                is_extension: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(
                    bump,
                ),
            }
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for NamePartBumpalo<'bump> {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => {
                        *self.is_extension.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Bool>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            *self.name_part.push_and_get_mut(&self.puroro_internal) =
                                bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
                                .map(|rv| {
                                    rv.and_then(|variant| {
                                        variant.to_native::<::puroro_internal::tags::Bool>()
                                    })
                                })
                                .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                            let mut iter = values.into_iter();
                            let first = iter
                                .next()
                                .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                            MaybeRepeatedVariantField::extend(&mut self.is_extension, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableFromIter for NamePartBumpalo<'bump> {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::serializer::Serializable for NamePartBumpalo<'bump> {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            for string in self.name_part.iter_for_ser() {
                serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
            }
            serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
                2,
                self.is_extension.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::Serializable for NamePartBumpalo<'bump> {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> NamePartTrait for NamePartBumpalo<'bump> {
        fn name_part(&self) -> &'_ str {
            self.name_part.as_ref()
        }
        fn is_extension(&self) -> bool {
            self.is_extension.clone()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for NamePartBumpalo<'bump> {
        fn new() -> Self {
            unimplemented!()
        }
        fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }
    pub trait NamePartTrait {
        fn name_part(&'_ self) -> &'_ str;
        fn is_extension(&'_ self) -> bool;
    }
    pub trait NamePartMutTrait {
        fn name_part_mut(&self) -> &mut String;
        fn is_extension_mut(&self) -> &mut bool;
    }
} // mod uninterpreted_option

#[derive(Debug, Clone)]
pub struct MethodOptions {
    pub deprecated: bool,
    pub idempotency_level: ::std::result::Result<
        super::super::google::protobuf::method_options::IdempotencyLevel,
        i32,
    >,
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl MethodOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            idempotency_level: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MethodOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                34 => {
                    *self
                        .idempotency_level
                        .push_and_get_mut(&self.puroro_internal) = variant
                        .to_native::<::puroro_internal::tags::Enum<
                        super::super::google::protobuf::method_options::IdempotencyLevel,
                    >>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    33 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    34 => {
                        let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.idempotency_level, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                33 | 34 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                33 | 34 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for MethodOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for MethodOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            33,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::method_options::IdempotencyLevel,
        >, _>(
            34,
            self.idempotency_level
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for MethodOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl MethodOptionsTrait for MethodOptions {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn idempotency_level(&self) -> ::std::result::Result<method_options::IdempotencyLevel, i32> {
        self.idempotency_level.clone()
    }
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MethodOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct MethodOptionsBumpalo<'bump> {
    pub deprecated: bool,
    pub idempotency_level: ::std::result::Result<
        super::super::google::protobuf::method_options::IdempotencyLevel,
        i32,
    >,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            idempotency_level: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for MethodOptionsBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                34 => {
                    *self
                        .idempotency_level
                        .push_and_get_mut(&self.puroro_internal) = variant
                        .to_native::<::puroro_internal::tags::Enum<
                        super::super::google::protobuf::method_options::IdempotencyLevel,
                    >>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    33 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    34 => {
                        let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<super::super::google::protobuf::method_options::IdempotencyLevel>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.idempotency_level, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                33 | 34 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                33 | 34 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for MethodOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for MethodOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            33,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::method_options::IdempotencyLevel,
        >, _>(
            34,
            self.idempotency_level
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MethodOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodOptionsTrait for MethodOptionsBumpalo<'bump> {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    fn idempotency_level(&self) -> ::std::result::Result<method_options::IdempotencyLevel, i32> {
        self.idempotency_level.clone()
    }
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for MethodOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait MethodOptionsTrait {
    fn deprecated(&'_ self) -> bool;
    fn idempotency_level(&'_ self) -> ::std::result::Result<method_options::IdempotencyLevel, i32>;
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait MethodOptionsMutTrait {
    fn deprecated_mut(&self) -> &mut bool;
    fn idempotency_level_mut(
        &self,
    ) -> &mut ::std::result::Result<
        super::super::google::protobuf::method_options::IdempotencyLevel,
        i32,
    >;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
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
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl ServiceOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ServiceOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    33 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                33 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                33 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for ServiceOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for ServiceOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            33,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for ServiceOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ServiceOptionsTrait for ServiceOptions {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ServiceOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ServiceOptionsBumpalo<'bump> {
    pub deprecated: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for ServiceOptionsBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                33 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    33 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                33 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                33 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for ServiceOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for ServiceOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            33,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ServiceOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceOptionsTrait for ServiceOptionsBumpalo<'bump> {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for ServiceOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait ServiceOptionsTrait {
    fn deprecated(&'_ self) -> bool;
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait ServiceOptionsMutTrait {
    fn deprecated_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
    // We need more!
}

#[derive(Debug, Clone)]
pub struct EnumValueOptions {
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl EnumValueOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumValueOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for EnumValueOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for EnumValueOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            1,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumValueOptionsTrait for EnumValueOptions {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumValueOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumValueOptionsBumpalo<'bump> {
    pub deprecated: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for EnumValueOptionsBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for EnumValueOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for EnumValueOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            1,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumValueOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueOptionsTrait for EnumValueOptionsBumpalo<'bump> {
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for EnumValueOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait EnumValueOptionsTrait {
    fn deprecated(&'_ self) -> bool;
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait EnumValueOptionsMutTrait {
    fn deprecated_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
    // We need more!
}

#[derive(Debug, Clone)]
pub struct EnumOptions {
    pub allow_alias: bool,
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl EnumOptions {
    pub fn new() -> Self {
        Self {
            allow_alias: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                2 => {
                    *self.allow_alias.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    2 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.allow_alias, first, iter);
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                2 | 3 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for EnumOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for EnumOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            self.allow_alias.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for EnumOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumOptionsTrait for EnumOptions {
    fn allow_alias(&self) -> bool {
        self.allow_alias.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumOptionsBumpalo<'bump> {
    pub allow_alias: bool,
    pub deprecated: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            allow_alias: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                2 => {
                    *self.allow_alias.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    2 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.allow_alias, first, iter);
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                2 | 3 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                2 | 3 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for EnumOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for EnumOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            self.allow_alias.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumOptionsTrait for EnumOptionsBumpalo<'bump> {
    fn allow_alias(&self) -> bool {
        self.allow_alias.clone()
    }
    fn deprecated(&self) -> bool {
        self.deprecated.clone()
    }
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for EnumOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait EnumOptionsTrait {
    fn allow_alias(&'_ self) -> bool;
    fn deprecated(&'_ self) -> bool;
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait EnumOptionsMutTrait {
    fn allow_alias_mut(&self) -> &mut bool;
    fn deprecated_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
    // We need more!
}

#[derive(Debug, Clone)]
pub struct OneofOptions {
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl OneofOptions {
    pub fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for OneofOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for OneofOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for OneofOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for OneofOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl OneofOptionsTrait for OneofOptions {
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for OneofOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct OneofOptionsBumpalo<'bump> {
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for OneofOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for OneofOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for OneofOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for OneofOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofOptionsTrait for OneofOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for OneofOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait OneofOptionsTrait {
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait OneofOptionsMutTrait {
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
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
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl FieldOptions {
    pub fn new() -> Self {
        Self {
            ctype: ::puroro_internal::helpers::FieldNew::new(),
            packed: ::puroro_internal::helpers::FieldNew::new(),
            jstype: ::puroro_internal::helpers::FieldNew::new(),
            lazy: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            weak: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FieldOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.ctype.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_options::Ctype,
                        >>()?;
                }
                2 => {
                    *self.packed.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                6 => {
                    *self.jstype.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_options::Jstype,
                        >>()?;
                }
                5 => {
                    *self.lazy.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                10 => {
                    *self.weak.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Enum<
                                        super::super::google::protobuf::field_options::Ctype,
                                    >>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.ctype, first, iter);
                    }
                    2 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.packed, first, iter);
                    }
                    6 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Enum<
                                        super::super::google::protobuf::field_options::Jstype,
                                    >>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.jstype, first, iter);
                    }
                    5 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.lazy, first, iter);
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    10 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.weak, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for FieldOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for FieldOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_options::Ctype,
        >, _>(1, self.ctype.iter_for_ser().cloned().map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            self.packed.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_options::Jstype,
        >, _>(6, self.jstype.iter_for_ser().cloned().map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            5,
            self.lazy.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            10,
            self.weak.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for FieldOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FieldOptionsTrait for FieldOptions {
    fn ctype(&self) -> ::std::result::Result<field_options::Ctype, i32> {
        self.ctype.clone()
    }
    fn packed(&self) -> bool {
        self.packed.clone()
    }
    fn jstype(&self) -> ::std::result::Result<field_options::Jstype, i32> {
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
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FieldOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FieldOptionsBumpalo<'bump> {
    pub ctype: ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>,
    pub packed: bool,
    pub jstype: ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>,
    pub lazy: bool,
    pub deprecated: bool,
    pub weak: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            ctype: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            packed: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            jstype: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            lazy: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            weak: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FieldOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self.ctype.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_options::Ctype,
                        >>()?;
                }
                2 => {
                    *self.packed.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                6 => {
                    *self.jstype.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_options::Jstype,
                        >>()?;
                }
                5 => {
                    *self.lazy.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                10 => {
                    *self.weak.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Enum<
                                        super::super::google::protobuf::field_options::Ctype,
                                    >>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.ctype, first, iter);
                    }
                    2 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.packed, first, iter);
                    }
                    6 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Enum<
                                        super::super::google::protobuf::field_options::Jstype,
                                    >>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.jstype, first, iter);
                    }
                    5 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.lazy, first, iter);
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    10 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.weak, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 5 | 3 | 10 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for FieldOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for FieldOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_options::Ctype,
        >, _>(1, self.ctype.iter_for_ser().cloned().map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            self.packed.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_options::Jstype,
        >, _>(6, self.jstype.iter_for_ser().cloned().map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            5,
            self.lazy.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            10,
            self.weak.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FieldOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldOptionsTrait for FieldOptionsBumpalo<'bump> {
    fn ctype(&self) -> ::std::result::Result<field_options::Ctype, i32> {
        self.ctype.clone()
    }
    fn packed(&self) -> bool {
        self.packed.clone()
    }
    fn jstype(&self) -> ::std::result::Result<field_options::Jstype, i32> {
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
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for FieldOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait FieldOptionsTrait {
    fn ctype(&'_ self) -> ::std::result::Result<field_options::Ctype, i32>;
    fn packed(&'_ self) -> bool;
    fn jstype(&'_ self) -> ::std::result::Result<field_options::Jstype, i32>;
    fn lazy(&'_ self) -> bool;
    fn deprecated(&'_ self) -> bool;
    fn weak(&'_ self) -> bool;
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait FieldOptionsMutTrait {
    fn ctype_mut(
        &self,
    ) -> &mut ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>;
    fn packed_mut(&self) -> &mut bool;
    fn jstype_mut(
        &self,
    ) -> &mut ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>;
    fn lazy_mut(&self) -> &mut bool;
    fn deprecated_mut(&self) -> &mut bool;
    fn weak_mut(&self) -> &mut bool;
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
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
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl MessageOptions {
    pub fn new() -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::helpers::FieldNew::new(),
            no_standard_descriptor_accessor: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            map_entry: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MessageOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MessageOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self
                        .message_set_wire_format
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                2 => {
                    *self
                        .no_standard_descriptor_accessor
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                7 => {
                    *self.map_entry.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.message_set_wire_format,
                            first,
                            iter,
                        );
                    }
                    2 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.no_standard_descriptor_accessor,
                            first,
                            iter,
                        );
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    7 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.map_entry, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for MessageOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for MessageOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            1,
            self.message_set_wire_format
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            self.no_standard_descriptor_accessor
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            7,
            self.map_entry.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for MessageOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
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
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MessageOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct MessageOptionsBumpalo<'bump> {
    pub message_set_wire_format: bool,
    pub no_standard_descriptor_accessor: bool,
    pub deprecated: bool,
    pub map_entry: bool,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MessageOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            no_standard_descriptor_accessor: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(
                bump,
            ),
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            map_entry: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for MessageOptionsBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => {
                    *self
                        .message_set_wire_format
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                2 => {
                    *self
                        .no_standard_descriptor_accessor
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                7 => {
                    *self.map_entry.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.message_set_wire_format,
                            first,
                            iter,
                        );
                    }
                    2 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.no_standard_descriptor_accessor,
                            first,
                            iter,
                        );
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    7 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.map_entry, first, iter);
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 7 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for MessageOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for MessageOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            1,
            self.message_set_wire_format
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            self.no_standard_descriptor_accessor
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            7,
            self.map_entry.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MessageOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MessageOptionsTrait for MessageOptionsBumpalo<'bump> {
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
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for MessageOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait MessageOptionsTrait {
    fn message_set_wire_format(&'_ self) -> bool;
    fn no_standard_descriptor_accessor(&'_ self) -> bool;
    fn deprecated(&'_ self) -> bool;
    fn map_entry(&'_ self) -> bool;
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
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
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
    // We need more!
}

#[derive(Debug, Clone)]
pub struct FileOptions {
    pub java_package: ::std::string::String,
    pub java_outer_classname: ::std::string::String,
    pub java_multiple_files: bool,
    pub java_generate_equals_and_hash: bool,
    pub java_string_check_utf8: bool,
    pub optimize_for:
        ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
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
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl FileOptions {
    pub fn new() -> Self {
        Self {
            java_package: ::puroro_internal::helpers::FieldNew::new(),
            java_outer_classname: ::puroro_internal::helpers::FieldNew::new(),
            java_multiple_files: ::puroro_internal::helpers::FieldNew::new(),
            java_generate_equals_and_hash: ::puroro_internal::helpers::FieldNew::new(),
            java_string_check_utf8: ::puroro_internal::helpers::FieldNew::new(),
            optimize_for: ::puroro_internal::helpers::FieldNew::new(),
            go_package: ::puroro_internal::helpers::FieldNew::new(),
            cc_generic_services: ::puroro_internal::helpers::FieldNew::new(),
            java_generic_services: ::puroro_internal::helpers::FieldNew::new(),
            py_generic_services: ::puroro_internal::helpers::FieldNew::new(),
            php_generic_services: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            cc_enable_arenas: ::puroro_internal::helpers::FieldNew::new(),
            objc_class_prefix: ::puroro_internal::helpers::FieldNew::new(),
            csharp_namespace: ::puroro_internal::helpers::FieldNew::new(),
            swift_prefix: ::puroro_internal::helpers::FieldNew::new(),
            php_class_prefix: ::puroro_internal::helpers::FieldNew::new(),
            php_namespace: ::puroro_internal::helpers::FieldNew::new(),
            php_metadata_namespace: ::puroro_internal::helpers::FieldNew::new(),
            ruby_package: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self
                        .java_multiple_files
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                20 => {
                    *self
                        .java_generate_equals_and_hash
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                27 => {
                    *self
                        .java_string_check_utf8
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                9 => {
                    *self.optimize_for.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::file_options::OptimizeMode,
                        >>()?;
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    *self
                        .cc_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                17 => {
                    *self
                        .java_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                18 => {
                    *self
                        .py_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                42 => {
                    *self
                        .php_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                23 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                31 => {
                    *self
                        .cc_enable_arenas
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
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
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.java_package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    8 => {
                        *self
                            .java_outer_classname
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    10 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_multiple_files,
                            first,
                            iter,
                        );
                    }
                    20 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_generate_equals_and_hash,
                            first,
                            iter,
                        );
                    }
                    27 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_string_check_utf8,
                            first,
                            iter,
                        );
                    }
                    9 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Enum<
                                        super::super::google::protobuf::file_options::OptimizeMode,
                                    >>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.optimize_for, first, iter);
                    }
                    11 => {
                        *self.go_package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.cc_generic_services,
                            first,
                            iter,
                        );
                    }
                    17 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_generic_services,
                            first,
                            iter,
                        );
                    }
                    18 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.py_generic_services,
                            first,
                            iter,
                        );
                    }
                    42 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.php_generic_services,
                            first,
                            iter,
                        );
                    }
                    23 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    31 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.cc_enable_arenas, first, iter);
                    }
                    36 => {
                        *self
                            .objc_class_prefix
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    37 => {
                        *self
                            .csharp_namespace
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    39 => {
                        *self.swift_prefix.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    40 => {
                        *self
                            .php_class_prefix
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    41 => {
                        *self.php_namespace.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    44 => {
                        *self
                            .php_metadata_namespace
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    45 => {
                        *self.ruby_package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40
                | 41 | 44 | 45 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40
                | 41 | 44 | 45 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for FileOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for FileOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.java_package.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.java_outer_classname.iter_for_ser() {
            serializer.serialize_bytes_twice(8, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            10,
            self.java_multiple_files
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            20,
            self.java_generate_equals_and_hash
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            27,
            self.java_string_check_utf8
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::file_options::OptimizeMode,
        >, _>(9, self.optimize_for.iter_for_ser().cloned().map(|v| Ok(v)))?;
        for string in self.go_package.iter_for_ser() {
            serializer.serialize_bytes_twice(11, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            16,
            self.cc_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            17,
            self.java_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            18,
            self.py_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            42,
            self.php_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            23,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            31,
            self.cc_enable_arenas.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
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
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileOptionsTrait for FileOptions {
    fn java_package(&self) -> &'_ str {
        self.java_package.as_ref()
    }
    fn java_outer_classname(&self) -> &'_ str {
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
    fn optimize_for(&self) -> ::std::result::Result<file_options::OptimizeMode, i32> {
        self.optimize_for.clone()
    }
    fn go_package(&self) -> &'_ str {
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
    fn objc_class_prefix(&self) -> &'_ str {
        self.objc_class_prefix.as_ref()
    }
    fn csharp_namespace(&self) -> &'_ str {
        self.csharp_namespace.as_ref()
    }
    fn swift_prefix(&self) -> &'_ str {
        self.swift_prefix.as_ref()
    }
    fn php_class_prefix(&self) -> &'_ str {
        self.php_class_prefix.as_ref()
    }
    fn php_namespace(&self) -> &'_ str {
        self.php_namespace.as_ref()
    }
    fn php_metadata_namespace(&self) -> &'_ str {
        self.php_metadata_namespace.as_ref()
    }
    fn ruby_package(&self) -> &'_ str {
        self.ruby_package.as_ref()
    }
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FileOptionsBumpalo<'bump> {
    pub java_package: ::bumpalo::collections::String<'bump>,
    pub java_outer_classname: ::bumpalo::collections::String<'bump>,
    pub java_multiple_files: bool,
    pub java_generate_equals_and_hash: bool,
    pub java_string_check_utf8: bool,
    pub optimize_for:
        ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
    pub go_package: ::bumpalo::collections::String<'bump>,
    pub cc_generic_services: bool,
    pub java_generic_services: bool,
    pub py_generic_services: bool,
    pub php_generic_services: bool,
    pub deprecated: bool,
    pub cc_enable_arenas: bool,
    pub objc_class_prefix: ::bumpalo::collections::String<'bump>,
    pub csharp_namespace: ::bumpalo::collections::String<'bump>,
    pub swift_prefix: ::bumpalo::collections::String<'bump>,
    pub php_class_prefix: ::bumpalo::collections::String<'bump>,
    pub php_namespace: ::bumpalo::collections::String<'bump>,
    pub php_metadata_namespace: ::bumpalo::collections::String<'bump>,
    pub ruby_package: ::bumpalo::collections::String<'bump>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            java_package: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            java_outer_classname: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            java_multiple_files: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            java_generate_equals_and_hash: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(
                bump,
            ),
            java_string_check_utf8: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            optimize_for: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            go_package: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            cc_generic_services: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            java_generic_services: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            py_generic_services: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            php_generic_services: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            cc_enable_arenas: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            objc_class_prefix: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            csharp_namespace: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            swift_prefix: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            php_class_prefix: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            php_namespace: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            php_metadata_namespace: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            ruby_package: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self
                        .java_multiple_files
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                20 => {
                    *self
                        .java_generate_equals_and_hash
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                27 => {
                    *self
                        .java_string_check_utf8
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                9 => {
                    *self.optimize_for.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::file_options::OptimizeMode,
                        >>()?;
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    *self
                        .cc_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                17 => {
                    *self
                        .java_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                18 => {
                    *self
                        .py_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                42 => {
                    *self
                        .php_generic_services
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                23 => {
                    *self.deprecated.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                31 => {
                    *self
                        .cc_enable_arenas
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
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
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.java_package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    8 => {
                        *self
                            .java_outer_classname
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    10 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_multiple_files,
                            first,
                            iter,
                        );
                    }
                    20 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_generate_equals_and_hash,
                            first,
                            iter,
                        );
                    }
                    27 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_string_check_utf8,
                            first,
                            iter,
                        );
                    }
                    9 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Enum<
                                        super::super::google::protobuf::file_options::OptimizeMode,
                                    >>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.optimize_for, first, iter);
                    }
                    11 => {
                        *self.go_package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    16 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.cc_generic_services,
                            first,
                            iter,
                        );
                    }
                    17 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.java_generic_services,
                            first,
                            iter,
                        );
                    }
                    18 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.py_generic_services,
                            first,
                            iter,
                        );
                    }
                    42 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(
                            &mut self.php_generic_services,
                            first,
                            iter,
                        );
                    }
                    23 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.deprecated, first, iter);
                    }
                    31 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.cc_enable_arenas, first, iter);
                    }
                    36 => {
                        *self
                            .objc_class_prefix
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    37 => {
                        *self
                            .csharp_namespace
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    39 => {
                        *self.swift_prefix.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    40 => {
                        *self
                            .php_class_prefix
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    41 => {
                        *self.php_namespace.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    44 => {
                        *self
                            .php_metadata_namespace
                            .push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    45 => {
                        *self.ruby_package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40
                | 41 | 44 | 45 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 8 | 10 | 20 | 27 | 9 | 11 | 16 | 17 | 18 | 42 | 23 | 31 | 36 | 37 | 39 | 40
                | 41 | 44 | 45 | 999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for FileOptionsBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for FileOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.java_package.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.java_outer_classname.iter_for_ser() {
            serializer.serialize_bytes_twice(8, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            10,
            self.java_multiple_files
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            20,
            self.java_generate_equals_and_hash
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            27,
            self.java_string_check_utf8
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::file_options::OptimizeMode,
        >, _>(9, self.optimize_for.iter_for_ser().cloned().map(|v| Ok(v)))?;
        for string in self.go_package.iter_for_ser() {
            serializer.serialize_bytes_twice(11, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            16,
            self.cc_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            17,
            self.java_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            18,
            self.py_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            42,
            self.php_generic_services
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            23,
            self.deprecated.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            31,
            self.cc_enable_arenas.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
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
impl<'bump> ::puroro::Serializable for FileOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileOptionsTrait for FileOptionsBumpalo<'bump> {
    fn java_package(&self) -> &'_ str {
        self.java_package.as_ref()
    }
    fn java_outer_classname(&self) -> &'_ str {
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
    fn optimize_for(&self) -> ::std::result::Result<file_options::OptimizeMode, i32> {
        self.optimize_for.clone()
    }
    fn go_package(&self) -> &'_ str {
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
    fn objc_class_prefix(&self) -> &'_ str {
        self.objc_class_prefix.as_ref()
    }
    fn csharp_namespace(&self) -> &'_ str {
        self.csharp_namespace.as_ref()
    }
    fn swift_prefix(&self) -> &'_ str {
        self.swift_prefix.as_ref()
    }
    fn php_class_prefix(&self) -> &'_ str {
        self.php_class_prefix.as_ref()
    }
    fn php_namespace(&self) -> &'_ str {
        self.php_namespace.as_ref()
    }
    fn php_metadata_namespace(&self) -> &'_ str {
        self.php_metadata_namespace.as_ref()
    }
    fn ruby_package(&self) -> &'_ str {
        self.ruby_package.as_ref()
    }
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for FileOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait FileOptionsTrait {
    fn java_package(&'_ self) -> &'_ str;
    fn java_outer_classname(&'_ self) -> &'_ str;
    fn java_multiple_files(&'_ self) -> bool;
    fn java_generate_equals_and_hash(&'_ self) -> bool;
    fn java_string_check_utf8(&'_ self) -> bool;
    fn optimize_for(&'_ self) -> ::std::result::Result<file_options::OptimizeMode, i32>;
    fn go_package(&'_ self) -> &'_ str;
    fn cc_generic_services(&'_ self) -> bool;
    fn java_generic_services(&'_ self) -> bool;
    fn py_generic_services(&'_ self) -> bool;
    fn php_generic_services(&'_ self) -> bool;
    fn deprecated(&'_ self) -> bool;
    fn cc_enable_arenas(&'_ self) -> bool;
    fn objc_class_prefix(&'_ self) -> &'_ str;
    fn csharp_namespace(&'_ self) -> &'_ str;
    fn swift_prefix(&'_ self) -> &'_ str;
    fn php_class_prefix(&'_ self) -> &'_ str;
    fn php_namespace(&'_ self) -> &'_ str;
    fn php_metadata_namespace(&'_ self) -> &'_ str;
    fn ruby_package(&'_ self) -> &'_ str;
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait FileOptionsMutTrait {
    fn java_package_mut(&self) -> &mut String;
    fn java_outer_classname_mut(&self) -> &mut String;
    fn java_multiple_files_mut(&self) -> &mut bool;
    fn java_generate_equals_and_hash_mut(&self) -> &mut bool;
    fn java_string_check_utf8_mut(&self) -> &mut bool;
    fn optimize_for_mut(
        &self,
    ) -> &mut ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>;
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
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
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
    pub options: ::std::option::Option<::std::boxed::Box<MethodOptions>>,
    pub client_streaming: bool,
    pub server_streaming: bool,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl MethodDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            input_type: ::puroro_internal::helpers::FieldNew::new(),
            output_type: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            client_streaming: ::puroro_internal::helpers::FieldNew::new(),
            server_streaming: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MethodDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    *self
                        .client_streaming
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                6 => {
                    *self
                        .server_streaming
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.input_type.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        *self.output_type.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    4 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.client_streaming, first, iter);
                    }
                    6 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.server_streaming, first, iter);
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for MethodDescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for MethodDescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            5,
            self.client_streaming.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            6,
            self.server_streaming.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        Ok(())
    }
}

impl ::puroro::Serializable for MethodDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl MethodDescriptorProtoTrait for MethodDescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn input_type(&self) -> &'_ str {
        self.input_type.as_ref()
    }
    fn output_type(&self) -> &'_ str {
        self.output_type.as_ref()
    }
    type OptionsType = MethodOptions;
    fn options(&self) -> ::std::option::Option<&'_ MethodOptions> {
        self.options.as_deref()
    }
    fn client_streaming(&self) -> bool {
        self.client_streaming.clone()
    }
    fn server_streaming(&self) -> bool {
        self.server_streaming.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MethodDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct MethodDescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub input_type: ::bumpalo::collections::String<'bump>,
    pub output_type: ::bumpalo::collections::String<'bump>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, MethodOptionsBumpalo<'bump>>>,
    pub client_streaming: bool,
    pub server_streaming: bool,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            input_type: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            output_type: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            client_streaming: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            server_streaming: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for MethodDescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    *self
                        .client_streaming
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                6 => {
                    *self
                        .server_streaming
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.input_type.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        *self.output_type.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    4 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.client_streaming, first, iter);
                    }
                    6 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.server_streaming, first, iter);
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 | 6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter
    for MethodDescriptorProtoBumpalo<'bump>
{
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for MethodDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            5,
            self.client_streaming.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            6,
            self.server_streaming.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MethodDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodDescriptorProtoTrait for MethodDescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn input_type(&self) -> &'_ str {
        self.input_type.as_ref()
    }
    fn output_type(&self) -> &'_ str {
        self.output_type.as_ref()
    }
    type OptionsType = MethodOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ MethodOptions> {
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
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for MethodDescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait MethodDescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    fn input_type(&'_ self) -> &'_ str;
    fn output_type(&'_ self) -> &'_ str;
    type OptionsType: MethodOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ MethodOptions>;
    fn client_streaming(&'_ self) -> bool;
    fn server_streaming(&'_ self) -> bool;
}
pub trait MethodDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn input_type_mut(&self) -> &mut String;
    fn output_type_mut(&self) -> &mut String;
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::MethodOptions>;
    fn client_streaming_mut(&self) -> &mut bool;
    fn server_streaming_mut(&self) -> &mut bool;
}

#[derive(Debug, Clone)]
pub struct ServiceDescriptorProto {
    pub name: ::std::string::String,
    pub method: ::std::vec::Vec<MethodDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<ServiceOptions>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl ServiceDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            method: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ServiceDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.method.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for ServiceDescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for ServiceDescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ServiceDescriptorProtoTrait for ServiceDescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type MethodType = MethodDescriptorProto;
    fn for_each_method<F>(&self, mut f: F)
    where
        F: FnMut(&'_ MethodDescriptorProto),
    {
        for item in (self.method).iter() {
            (f)(item);
        }
    }
    fn method_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ MethodDescriptorProto>> {
        ::std::boxed::Box::new(self.method.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a> = impl Iterator<Item = &'a MethodDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_> {
        self.method.iter()
    }
    type OptionsType = ServiceOptions;
    fn options(&self) -> ::std::option::Option<&'_ ServiceOptions> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ServiceDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ServiceDescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub method: ::bumpalo::collections::Vec<'bump, MethodDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, ServiceOptionsBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            method: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for ServiceDescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.method.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter
    for ServiceDescriptorProtoBumpalo<'bump>
{
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for ServiceDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
impl<'bump> ::puroro::Serializable for ServiceDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceDescriptorProtoTrait for ServiceDescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type MethodType = MethodDescriptorProtoBumpalo<'bump>;
    fn for_each_method<F>(&self, mut f: F)
    where
        F: FnMut(&'_ MethodDescriptorProto),
    {
        for item in (self.method).iter() {
            (f)(item);
        }
    }
    fn method_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ MethodDescriptorProto>> {
        ::std::boxed::Box::new(self.method.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a> = impl Iterator<Item = &'a MethodDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_> {
        self.method.iter()
    }
    type OptionsType = ServiceOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ ServiceOptions> {
        self.options.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for ServiceDescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait ServiceDescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    type MethodType: MethodDescriptorProtoTrait;
    fn for_each_method<F>(&self, f: F)
    where
        F: FnMut(&'_ MethodDescriptorProto);
    fn method_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ MethodDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a>: Iterator<Item = &'a MethodDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_>;
    type OptionsType: ServiceOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ ServiceOptions>;
}
pub trait ServiceDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn for_each_method_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::MethodDescriptorProto);
    fn method_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::MethodDescriptorProto>,
    >;
    // We need more!
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::ServiceOptions>;
}

#[derive(Debug, Clone)]
pub struct EnumValueDescriptorProto {
    pub name: ::std::string::String,
    pub number: i32,
    pub options: ::std::option::Option<::std::boxed::Box<EnumValueOptions>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl EnumValueDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            number: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.number.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                    }
                    3 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for EnumValueDescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for EnumValueDescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            2,
            self.number.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumValueDescriptorProtoTrait for EnumValueDescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    type OptionsType = EnumValueOptions;
    fn options(&self) -> ::std::option::Option<&'_ EnumValueOptions> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumValueDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumValueDescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub number: i32,
    pub options:
        ::std::option::Option<::bumpalo::boxed::Box<'bump, EnumValueOptionsBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            number: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for EnumValueDescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => {
                    *self.number.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                    }
                    3 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter
    for EnumValueDescriptorProtoBumpalo<'bump>
{
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for EnumValueDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            2,
            self.number.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(3, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumValueDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    type OptionsType = EnumValueOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ EnumValueOptions> {
        self.options.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for EnumValueDescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait EnumValueDescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    fn number(&'_ self) -> i32;
    type OptionsType: EnumValueOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ EnumValueOptions>;
}
pub trait EnumValueDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn number_mut(&self) -> &mut i32;
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::EnumValueOptions>;
}

#[derive(Debug, Clone)]
pub struct EnumDescriptorProto {
    pub name: ::std::string::String,
    pub value: ::std::vec::Vec<EnumValueDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<EnumOptions>>,
    pub reserved_range: ::std::vec::Vec<enum_descriptor_proto::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl EnumDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            value: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            reserved_range: ::puroro_internal::helpers::FieldNew::new(),
            reserved_name: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.value.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    4 => {
                        let msg = self.reserved_range.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        *self.reserved_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for EnumDescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for EnumDescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumDescriptorProtoTrait for EnumDescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type ValueType = EnumValueDescriptorProto;
    fn for_each_value<F>(&self, mut f: F)
    where
        F: FnMut(&'_ EnumValueDescriptorProto),
    {
        for item in (self.value).iter() {
            (f)(item);
        }
    }
    fn value_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumValueDescriptorProto>> {
        ::std::boxed::Box::new(self.value.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a> = impl Iterator<Item = &'a EnumValueDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_> {
        self.value.iter()
    }
    type OptionsType = EnumOptions;
    fn options(&self) -> ::std::option::Option<&'_ EnumOptions> {
        self.options.as_deref()
    }
    type ReservedRangeType = enum_descriptor_proto::EnumReservedRange;
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ enum_descriptor_proto::EnumReservedRange),
    {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ enum_descriptor_proto::EnumReservedRange>>
    {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item = &'a enum_descriptor_proto::EnumReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str),
    {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct EnumDescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub value: ::bumpalo::collections::Vec<'bump, EnumValueDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, EnumOptionsBumpalo<'bump>>>,
    pub reserved_range:
        ::bumpalo::collections::Vec<'bump, enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>,
    pub reserved_name: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            reserved_range: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            reserved_name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for EnumDescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.value.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    4 => {
                        let msg = self.reserved_range.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        *self.reserved_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 4 | 5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for EnumDescriptorProtoBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for EnumDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
impl<'bump> ::puroro::Serializable for EnumDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumDescriptorProtoTrait for EnumDescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type ValueType = EnumValueDescriptorProtoBumpalo<'bump>;
    fn for_each_value<F>(&self, mut f: F)
    where
        F: FnMut(&'_ EnumValueDescriptorProto),
    {
        for item in (self.value).iter() {
            (f)(item);
        }
    }
    fn value_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumValueDescriptorProto>> {
        ::std::boxed::Box::new(self.value.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a> = impl Iterator<Item = &'a EnumValueDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_> {
        self.value.iter()
    }
    type OptionsType = EnumOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ EnumOptions> {
        self.options.as_deref()
    }
    type ReservedRangeType = enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>;
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ enum_descriptor_proto::EnumReservedRange),
    {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ enum_descriptor_proto::EnumReservedRange>>
    {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item = &'a enum_descriptor_proto::EnumReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str),
    {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for EnumDescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait EnumDescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    type ValueType: EnumValueDescriptorProtoTrait;
    fn for_each_value<F>(&self, f: F)
    where
        F: FnMut(&'_ EnumValueDescriptorProto);
    fn value_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumValueDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a>: Iterator<Item = &'a EnumValueDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_>;
    type OptionsType: EnumOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ EnumOptions>;
    type ReservedRangeType: enum_descriptor_proto::EnumReservedRangeTrait;
    fn for_each_reserved_range<F>(&self, f: F)
    where
        F: FnMut(&'_ enum_descriptor_proto::EnumReservedRange);
    fn reserved_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ enum_descriptor_proto::EnumReservedRange>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a>: Iterator<Item = &'a enum_descriptor_proto::EnumReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_>;
    fn for_each_reserved_name<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn reserved_name_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_>;
}
pub trait EnumDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn for_each_value_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::EnumValueDescriptorProto);
    fn value_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::EnumValueDescriptorProto>,
    >;
    // We need more!
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::EnumOptions>;
    fn for_each_reserved_range_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange);
    fn reserved_range_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&mut super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>>;
    // We need more!
    fn for_each_reserved_name_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn reserved_name_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut String>>;
    // We need more!
}
pub mod enum_descriptor_proto {

    #[derive(Debug, Clone)]
    pub struct EnumReservedRange {
        pub start: i32,
        pub end: i32,
        puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
    }

    impl EnumReservedRange {
        pub fn new() -> Self {
            Self {
                start: ::puroro_internal::helpers::FieldNew::new(),
                end: ::puroro_internal::helpers::FieldNew::new(),
                puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
            }
        }
    }

    impl ::std::default::Default for EnumReservedRange {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumReservedRange {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.start.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro_internal::deser::DeserializableFromIter for EnumReservedRange {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }

    impl ::puroro_internal::serializer::Serializable for EnumReservedRange {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.start.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }

    impl ::puroro::Serializable for EnumReservedRange {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
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
    impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumReservedRange {
        fn new() -> Self {
            Default::default()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct EnumReservedRangeBumpalo<'bump> {
        pub start: i32,
        pub end: i32,
        puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> EnumReservedRangeBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
            Self {
                start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(
                    bump,
                ),
            }
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
        for EnumReservedRangeBumpalo<'bump>
    {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.start.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableFromIter for EnumReservedRangeBumpalo<'bump> {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::serializer::Serializable for EnumReservedRangeBumpalo<'bump> {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.start.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::Serializable for EnumReservedRangeBumpalo<'bump> {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> EnumReservedRangeTrait for EnumReservedRangeBumpalo<'bump> {
        fn start(&self) -> i32 {
            self.start.clone()
        }
        fn end(&self) -> i32 {
            self.end.clone()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for EnumReservedRangeBumpalo<'bump> {
        fn new() -> Self {
            unimplemented!()
        }
        fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }
    pub trait EnumReservedRangeTrait {
        fn start(&'_ self) -> i32;
        fn end(&'_ self) -> i32;
    }
    pub trait EnumReservedRangeMutTrait {
        fn start_mut(&self) -> &mut i32;
        fn end_mut(&self) -> &mut i32;
    }
} // mod enum_descriptor_proto

#[derive(Debug, Clone)]
pub struct OneofDescriptorProto {
    pub name: ::std::string::String,
    pub options: ::std::option::Option<::std::boxed::Box<OneofOptions>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl OneofDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for OneofDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for OneofDescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for OneofDescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl OneofDescriptorProtoTrait for OneofDescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type OptionsType = OneofOptions;
    fn options(&self) -> ::std::option::Option<&'_ OneofOptions> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for OneofDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct OneofDescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, OneofOptionsBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for OneofDescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter
    for OneofDescriptorProtoBumpalo<'bump>
{
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for OneofDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
impl<'bump> ::puroro::Serializable for OneofDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofDescriptorProtoTrait for OneofDescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type OptionsType = OneofOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ OneofOptions> {
        self.options.as_deref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for OneofDescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait OneofDescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    type OptionsType: OneofOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ OneofOptions>;
}
pub trait OneofDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::OneofOptions>;
}

#[derive(Debug, Clone)]
pub struct FieldDescriptorProto {
    pub name: ::std::string::String,
    pub number: i32,
    pub label:
        ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>,
    pub type_:
        ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>,
    pub type_name: ::std::string::String,
    pub extendee: ::std::string::String,
    pub default_value: ::std::string::String,
    pub oneof_index: i32,
    pub json_name: ::std::string::String,
    pub options: ::std::option::Option<::std::boxed::Box<FieldOptions>>,
    pub proto3_optional: bool,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl FieldDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            number: ::puroro_internal::helpers::FieldNew::new(),
            label: ::puroro_internal::helpers::FieldNew::new(),
            type_: ::puroro_internal::helpers::FieldNew::new(),
            type_name: ::puroro_internal::helpers::FieldNew::new(),
            extendee: ::puroro_internal::helpers::FieldNew::new(),
            default_value: ::puroro_internal::helpers::FieldNew::new(),
            oneof_index: ::puroro_internal::helpers::FieldNew::new(),
            json_name: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            proto3_optional: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FieldDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.number.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                4 => {
                    *self.label.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_descriptor_proto::Label,
                        >>()?;
                }
                5 => {
                    *self.type_.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_descriptor_proto::Type,
                        >>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    *self.oneof_index.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    *self.proto3_optional.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                    }
                    4 => {
                        let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.label, first, iter);
                    }
                    5 => {
                        let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.type_, first, iter);
                    }
                    6 => {
                        *self.type_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.extendee.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    7 => {
                        *self.default_value.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    9 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.oneof_index, first, iter);
                    }
                    10 => {
                        *self.json_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    8 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    17 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.proto3_optional, first, iter);
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for FieldDescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for FieldDescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            3,
            self.number.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_descriptor_proto::Label,
        >, _>(4, self.label.iter_for_ser().cloned().map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_descriptor_proto::Type,
        >, _>(5, self.type_.iter_for_ser().cloned().map(|v| Ok(v)))?;
        for string in self.type_name.iter_for_ser() {
            serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.extendee.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.default_value.iter_for_ser() {
            serializer.serialize_bytes_twice(7, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            9,
            self.oneof_index.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for string in self.json_name.iter_for_ser() {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            17,
            self.proto3_optional.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        Ok(())
    }
}

impl ::puroro::Serializable for FieldDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FieldDescriptorProtoTrait for FieldDescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    fn label(&self) -> ::std::result::Result<field_descriptor_proto::Label, i32> {
        self.label.clone()
    }
    fn type_(&self) -> ::std::result::Result<field_descriptor_proto::Type, i32> {
        self.type_.clone()
    }
    fn type_name(&self) -> &'_ str {
        self.type_name.as_ref()
    }
    fn extendee(&self) -> &'_ str {
        self.extendee.as_ref()
    }
    fn default_value(&self) -> &'_ str {
        self.default_value.as_ref()
    }
    fn oneof_index(&self) -> i32 {
        self.oneof_index.clone()
    }
    fn json_name(&self) -> &'_ str {
        self.json_name.as_ref()
    }
    type OptionsType = FieldOptions;
    fn options(&self) -> ::std::option::Option<&'_ FieldOptions> {
        self.options.as_deref()
    }
    fn proto3_optional(&self) -> bool {
        self.proto3_optional.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FieldDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FieldDescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub number: i32,
    pub label:
        ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>,
    pub type_:
        ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>,
    pub type_name: ::bumpalo::collections::String<'bump>,
    pub extendee: ::bumpalo::collections::String<'bump>,
    pub default_value: ::bumpalo::collections::String<'bump>,
    pub oneof_index: i32,
    pub json_name: ::bumpalo::collections::String<'bump>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, FieldOptionsBumpalo<'bump>>>,
    pub proto3_optional: bool,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            number: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            label: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            type_: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            type_name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            extendee: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            default_value: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            oneof_index: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            json_name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            proto3_optional: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for FieldDescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => {
                    *self.number.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                4 => {
                    *self.label.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_descriptor_proto::Label,
                        >>()?;
                }
                5 => {
                    *self.type_.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            super::super::google::protobuf::field_descriptor_proto::Type,
                        >>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    *self.oneof_index.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    *self.proto3_optional.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                    }
                    4 => {
                        let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Label>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.label, first, iter);
                    }
                    5 => {
                        let values = bytes_iter.variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<super::super::google::protobuf::field_descriptor_proto::Type>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.type_, first, iter);
                    }
                    6 => {
                        *self.type_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.extendee.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    7 => {
                        *self.default_value.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    9 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.oneof_index, first, iter);
                    }
                    10 => {
                        *self.json_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    8 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    17 => {
                        let values = bytes_iter
                            .variants()
                            .map(|rv| {
                                rv.and_then(|variant| {
                                    variant.to_native::<::puroro_internal::tags::Bool>()
                                })
                            })
                            .collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                        let mut iter = values.into_iter();
                        let first = iter
                            .next()
                            .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                        MaybeRepeatedVariantField::extend(&mut self.proto3_optional, first, iter);
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 3 | 4 | 5 | 6 | 2 | 7 | 9 | 10 | 8 | 17 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter
    for FieldDescriptorProtoBumpalo<'bump>
{
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for FieldDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            3,
            self.number.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_descriptor_proto::Label,
        >, _>(4, self.label.iter_for_ser().cloned().map(|v| Ok(v)))?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            super::super::google::protobuf::field_descriptor_proto::Type,
        >, _>(5, self.type_.iter_for_ser().cloned().map(|v| Ok(v)))?;
        for string in self.type_name.iter_for_ser() {
            serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.extendee.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.default_value.iter_for_ser() {
            serializer.serialize_bytes_twice(7, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            9,
            self.oneof_index.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        for string in self.json_name.iter_for_ser() {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in self.options.iter_for_ser() {
            serializer.serialize_message_twice(8, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            17,
            self.proto3_optional.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FieldDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldDescriptorProtoTrait for FieldDescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn number(&self) -> i32 {
        self.number.clone()
    }
    fn label(&self) -> ::std::result::Result<field_descriptor_proto::Label, i32> {
        self.label.clone()
    }
    fn type_(&self) -> ::std::result::Result<field_descriptor_proto::Type, i32> {
        self.type_.clone()
    }
    fn type_name(&self) -> &'_ str {
        self.type_name.as_ref()
    }
    fn extendee(&self) -> &'_ str {
        self.extendee.as_ref()
    }
    fn default_value(&self) -> &'_ str {
        self.default_value.as_ref()
    }
    fn oneof_index(&self) -> i32 {
        self.oneof_index.clone()
    }
    fn json_name(&self) -> &'_ str {
        self.json_name.as_ref()
    }
    type OptionsType = FieldOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ FieldOptions> {
        self.options.as_deref()
    }
    fn proto3_optional(&self) -> bool {
        self.proto3_optional.clone()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for FieldDescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait FieldDescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    fn number(&'_ self) -> i32;
    fn label(&'_ self) -> ::std::result::Result<field_descriptor_proto::Label, i32>;
    fn type_(&'_ self) -> ::std::result::Result<field_descriptor_proto::Type, i32>;
    fn type_name(&'_ self) -> &'_ str;
    fn extendee(&'_ self) -> &'_ str;
    fn default_value(&'_ self) -> &'_ str;
    fn oneof_index(&'_ self) -> i32;
    fn json_name(&'_ self) -> &'_ str;
    type OptionsType: FieldOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ FieldOptions>;
    fn proto3_optional(&'_ self) -> bool;
}
pub trait FieldDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn number_mut(&self) -> &mut i32;
    fn label_mut(
        &self,
    ) -> &mut ::std::result::Result<
        super::super::google::protobuf::field_descriptor_proto::Label,
        i32,
    >;
    fn type__mut(
        &self,
    ) -> &mut ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>;
    fn type_name_mut(&self) -> &mut String;
    fn extendee_mut(&self) -> &mut String;
    fn default_value_mut(&self) -> &mut String;
    fn oneof_index_mut(&self) -> &mut i32;
    fn json_name_mut(&self) -> &mut String;
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::FieldOptions>;
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
    pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl ExtensionRangeOptions {
    pub fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRangeOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for ExtensionRangeOptions {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for ExtensionRangeOptions {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for ExtensionRangeOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl ExtensionRangeOptionsTrait for ExtensionRangeOptions {
    type UninterpretedOptionType = UninterpretedOption;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ExtensionRangeOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct ExtensionRangeOptionsBumpalo<'bump> {
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for ExtensionRangeOptionsBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    999 => {
                        let msg = self
                            .uninterpreted_option
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter
    for ExtensionRangeOptionsBumpalo<'bump>
{
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for ExtensionRangeOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.uninterpreted_option.iter_for_ser() {
            serializer.serialize_message_twice(999, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ExtensionRangeOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeOptionsTrait for ExtensionRangeOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ UninterpretedOption),
    {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for ExtensionRangeOptionsBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait ExtensionRangeOptionsTrait {
    type UninterpretedOptionType: UninterpretedOptionTrait;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ UninterpretedOption);
    fn uninterpreted_option_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ UninterpretedOption>>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: Iterator<Item = &'a UninterpretedOption>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}
pub trait ExtensionRangeOptionsMutTrait {
    fn for_each_uninterpreted_option_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::UninterpretedOption);
    fn uninterpreted_option_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::UninterpretedOption>,
    >;
    // We need more!
}

#[derive(Debug, Clone)]
pub struct DescriptorProto {
    pub name: ::std::string::String,
    pub field: ::std::vec::Vec<FieldDescriptorProto>,
    pub extension: ::std::vec::Vec<FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec<DescriptorProto>,
    pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,
    pub extension_range: ::std::vec::Vec<descriptor_proto::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec<OneofDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<MessageOptions>>,
    pub reserved_range: ::std::vec::Vec<descriptor_proto::ReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl DescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            field: ::puroro_internal::helpers::FieldNew::new(),
            extension: ::puroro_internal::helpers::FieldNew::new(),
            nested_type: ::puroro_internal::helpers::FieldNew::new(),
            enum_type: ::puroro_internal::helpers::FieldNew::new(),
            extension_range: ::puroro_internal::helpers::FieldNew::new(),
            oneof_decl: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            reserved_range: ::puroro_internal::helpers::FieldNew::new(),
            reserved_name: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for DescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
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
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.field.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    6 => {
                        let msg = self.extension.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        let msg = self.nested_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    4 => {
                        let msg = self.enum_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        let msg = self.extension_range.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    8 => {
                        let msg = self.oneof_decl.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    7 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    9 => {
                        let msg = self.reserved_range.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    10 => {
                        *self.reserved_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for DescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for DescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl DescriptorProtoTrait for DescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type FieldType = FieldDescriptorProto;
    fn for_each_field<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto),
    {
        for item in (self.field).iter() {
            (f)(item);
        }
    }
    fn field_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.field.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a> = impl Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_> {
        self.field.iter()
    }
    type ExtensionType = FieldDescriptorProto;
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto),
    {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    type NestedTypeType = DescriptorProto;
    fn for_each_nested_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ DescriptorProto),
    {
        for item in (self.nested_type).iter() {
            (f)(item);
        }
    }
    fn nested_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ DescriptorProto>> {
        ::std::boxed::Box::new(self.nested_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a> = impl Iterator<Item = &'a DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_> {
        self.nested_type.iter()
    }
    type EnumTypeType = EnumDescriptorProto;
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ EnumDescriptorProto),
    {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item = &'a EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    type ExtensionRangeType = descriptor_proto::ExtensionRange;
    fn for_each_extension_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ descriptor_proto::ExtensionRange),
    {
        for item in (self.extension_range).iter() {
            (f)(item);
        }
    }
    fn extension_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ descriptor_proto::ExtensionRange>> {
        ::std::boxed::Box::new(self.extension_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a> = impl Iterator<Item = &'a descriptor_proto::ExtensionRange>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_> {
        self.extension_range.iter()
    }
    type OneofDeclType = OneofDescriptorProto;
    fn for_each_oneof_decl<F>(&self, mut f: F)
    where
        F: FnMut(&'_ OneofDescriptorProto),
    {
        for item in (self.oneof_decl).iter() {
            (f)(item);
        }
    }
    fn oneof_decl_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ OneofDescriptorProto>> {
        ::std::boxed::Box::new(self.oneof_decl.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a> = impl Iterator<Item = &'a OneofDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_> {
        self.oneof_decl.iter()
    }
    type OptionsType = MessageOptions;
    fn options(&self) -> ::std::option::Option<&'_ MessageOptions> {
        self.options.as_deref()
    }
    type ReservedRangeType = descriptor_proto::ReservedRange;
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ descriptor_proto::ReservedRange),
    {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ descriptor_proto::ReservedRange>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item = &'a descriptor_proto::ReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str),
    {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for DescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct DescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub field: ::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>>,
    pub extension: ::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>>,
    pub nested_type: ::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>>,
    pub enum_type: ::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>>,
    pub extension_range:
        ::bumpalo::collections::Vec<'bump, descriptor_proto::ExtensionRangeBumpalo<'bump>>,
    pub oneof_decl: ::bumpalo::collections::Vec<'bump, OneofDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, MessageOptionsBumpalo<'bump>>>,
    pub reserved_range:
        ::bumpalo::collections::Vec<'bump, descriptor_proto::ReservedRangeBumpalo<'bump>>,
    pub reserved_name: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> DescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            field: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            extension: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            nested_type: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            enum_type: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            extension_range: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            oneof_decl: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            reserved_range: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            reserved_name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for DescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
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
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let msg = self.field.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    6 => {
                        let msg = self.extension.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    3 => {
                        let msg = self.nested_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    4 => {
                        let msg = self.enum_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        let msg = self.extension_range.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    8 => {
                        let msg = self.oneof_decl.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    7 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    9 => {
                        let msg = self.reserved_range.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    10 => {
                        *self.reserved_name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 6 | 3 | 4 | 5 | 8 | 7 | 9 | 10 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for DescriptorProtoBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for DescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
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
impl<'bump> ::puroro::Serializable for DescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> DescriptorProtoTrait for DescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    type FieldType = FieldDescriptorProtoBumpalo<'bump>;
    fn for_each_field<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto),
    {
        for item in (self.field).iter() {
            (f)(item);
        }
    }
    fn field_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.field.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a> = impl Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_> {
        self.field.iter()
    }
    type ExtensionType = FieldDescriptorProtoBumpalo<'bump>;
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto),
    {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    type NestedTypeType = DescriptorProtoBumpalo<'bump>;
    fn for_each_nested_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ DescriptorProto),
    {
        for item in (self.nested_type).iter() {
            (f)(item);
        }
    }
    fn nested_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ DescriptorProto>> {
        ::std::boxed::Box::new(self.nested_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a> = impl Iterator<Item = &'a DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_> {
        self.nested_type.iter()
    }
    type EnumTypeType = EnumDescriptorProtoBumpalo<'bump>;
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ EnumDescriptorProto),
    {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item = &'a EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    type ExtensionRangeType = descriptor_proto::ExtensionRangeBumpalo<'bump>;
    fn for_each_extension_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ descriptor_proto::ExtensionRange),
    {
        for item in (self.extension_range).iter() {
            (f)(item);
        }
    }
    fn extension_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ descriptor_proto::ExtensionRange>> {
        ::std::boxed::Box::new(self.extension_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a> = impl Iterator<Item = &'a descriptor_proto::ExtensionRange>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_> {
        self.extension_range.iter()
    }
    type OneofDeclType = OneofDescriptorProtoBumpalo<'bump>;
    fn for_each_oneof_decl<F>(&self, mut f: F)
    where
        F: FnMut(&'_ OneofDescriptorProto),
    {
        for item in (self.oneof_decl).iter() {
            (f)(item);
        }
    }
    fn oneof_decl_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ OneofDescriptorProto>> {
        ::std::boxed::Box::new(self.oneof_decl.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a> = impl Iterator<Item = &'a OneofDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_> {
        self.oneof_decl.iter()
    }
    type OptionsType = MessageOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ MessageOptions> {
        self.options.as_deref()
    }
    type ReservedRangeType = descriptor_proto::ReservedRangeBumpalo<'bump>;
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ descriptor_proto::ReservedRange),
    {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ descriptor_proto::ReservedRange>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl Iterator<Item = &'a descriptor_proto::ReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str),
    {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for DescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait DescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    type FieldType: FieldDescriptorProtoTrait;
    fn for_each_field<F>(&self, f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto);
    fn field_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a>: Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_>;
    type ExtensionType: FieldDescriptorProtoTrait;
    fn for_each_extension<F>(&self, f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto);
    fn extension_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a>: Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_>;
    type NestedTypeType: DescriptorProtoTrait;
    fn for_each_nested_type<F>(&self, f: F)
    where
        F: FnMut(&'_ DescriptorProto);
    fn nested_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ DescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a>: Iterator<Item = &'a DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_>;
    type EnumTypeType: EnumDescriptorProtoTrait;
    fn for_each_enum_type<F>(&self, f: F)
    where
        F: FnMut(&'_ EnumDescriptorProto);
    fn enum_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a>: Iterator<Item = &'a EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_>;
    type ExtensionRangeType: descriptor_proto::ExtensionRangeTrait;
    fn for_each_extension_range<F>(&self, f: F)
    where
        F: FnMut(&'_ descriptor_proto::ExtensionRange);
    fn extension_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ descriptor_proto::ExtensionRange>>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a>: Iterator<Item = &'a descriptor_proto::ExtensionRange>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_>;
    type OneofDeclType: OneofDescriptorProtoTrait;
    fn for_each_oneof_decl<F>(&self, f: F)
    where
        F: FnMut(&'_ OneofDescriptorProto);
    fn oneof_decl_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ OneofDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a>: Iterator<Item = &'a OneofDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_>;
    type OptionsType: MessageOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ MessageOptions>;
    type ReservedRangeType: descriptor_proto::ReservedRangeTrait;
    fn for_each_reserved_range<F>(&self, f: F)
    where
        F: FnMut(&'_ descriptor_proto::ReservedRange);
    fn reserved_range_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ descriptor_proto::ReservedRange>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a>: Iterator<Item = &'a descriptor_proto::ReservedRange>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_>;
    fn for_each_reserved_name<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn reserved_name_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_>;
}
pub trait DescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn for_each_field_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FieldDescriptorProto);
    fn field_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::FieldDescriptorProto>,
    >;
    // We need more!
    fn for_each_extension_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FieldDescriptorProto);
    fn extension_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::FieldDescriptorProto>,
    >;
    // We need more!
    fn for_each_nested_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::DescriptorProto);
    fn nested_type_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::DescriptorProto>,
    >;
    // We need more!
    fn for_each_enum_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::EnumDescriptorProto);
    fn enum_type_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::EnumDescriptorProto>,
    >;
    // We need more!
    fn for_each_extension_range_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::descriptor_proto::ExtensionRange);
    fn extension_range_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_
            + Iterator<Item = &mut super::super::google::protobuf::descriptor_proto::ExtensionRange>,
    >;
    // We need more!
    fn for_each_oneof_decl_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::OneofDescriptorProto);
    fn oneof_decl_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::OneofDescriptorProto>,
    >;
    // We need more!
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::MessageOptions>;
    fn for_each_reserved_range_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::descriptor_proto::ReservedRange);
    fn reserved_range_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_
            + Iterator<Item = &mut super::super::google::protobuf::descriptor_proto::ReservedRange>,
    >;
    // We need more!
    fn for_each_reserved_name_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn reserved_name_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut String>>;
    // We need more!
}
pub mod descriptor_proto {

    #[derive(Debug, Clone)]
    pub struct ReservedRange {
        pub start: i32,
        pub end: i32,
        puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
    }

    impl ReservedRange {
        pub fn new() -> Self {
            Self {
                start: ::puroro_internal::helpers::FieldNew::new(),
                end: ::puroro_internal::helpers::FieldNew::new(),
                puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
            }
        }
    }

    impl ::std::default::Default for ReservedRange {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::puroro_internal::deser::DeserializableMessageFromIter for ReservedRange {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.start.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro_internal::deser::DeserializableFromIter for ReservedRange {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }

    impl ::puroro_internal::serializer::Serializable for ReservedRange {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.start.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }

    impl ::puroro::Serializable for ReservedRange {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
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
    impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ReservedRange {
        fn new() -> Self {
            Default::default()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct ReservedRangeBumpalo<'bump> {
        pub start: i32,
        pub end: i32,
        puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ReservedRangeBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
            Self {
                start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(
                    bump,
                ),
            }
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
        for ReservedRangeBumpalo<'bump>
    {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.start.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableFromIter for ReservedRangeBumpalo<'bump> {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::serializer::Serializable for ReservedRangeBumpalo<'bump> {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.start.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::Serializable for ReservedRangeBumpalo<'bump> {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ReservedRangeTrait for ReservedRangeBumpalo<'bump> {
        fn start(&self) -> i32 {
            self.start.clone()
        }
        fn end(&self) -> i32 {
            self.end.clone()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for ReservedRangeBumpalo<'bump> {
        fn new() -> Self {
            unimplemented!()
        }
        fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }
    pub trait ReservedRangeTrait {
        fn start(&'_ self) -> i32;
        fn end(&'_ self) -> i32;
    }
    pub trait ReservedRangeMutTrait {
        fn start_mut(&self) -> &mut i32;
        fn end_mut(&self) -> &mut i32;
    }

    #[derive(Debug, Clone)]
    pub struct ExtensionRange {
        pub start: i32,
        pub end: i32,
        pub options: ::std::option::Option<::std::boxed::Box<super::ExtensionRangeOptions>>,
        puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
    }

    impl ExtensionRange {
        pub fn new() -> Self {
            Self {
                start: ::puroro_internal::helpers::FieldNew::new(),
                end: ::puroro_internal::helpers::FieldNew::new(),
                options: ::puroro_internal::helpers::FieldNew::new(),
                puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
            }
        }
    }

    impl ::std::default::Default for ExtensionRange {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRange {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.start.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        3 => {
                            let msg = self.options.push_and_get_mut(&self.puroro_internal);
                            bytes_iter.deser_message(msg)?;
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }

    impl ::puroro_internal::deser::DeserializableFromIter for ExtensionRange {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }

    impl ::puroro_internal::serializer::Serializable for ExtensionRange {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.start.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            for msg in self.options.iter_for_ser() {
                serializer.serialize_message_twice(3, msg)?;
            }
            Ok(())
        }
    }

    impl ::puroro::Serializable for ExtensionRange {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }

    impl ExtensionRangeTrait for ExtensionRange {
        fn start(&self) -> i32 {
            self.start.clone()
        }
        fn end(&self) -> i32 {
            self.end.clone()
        }
        type OptionsType = super::ExtensionRangeOptions;
        fn options(&self) -> ::std::option::Option<&'_ super::ExtensionRangeOptions> {
            self.options.as_deref()
        }
    }
    impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ExtensionRange {
        fn new() -> Self {
            Default::default()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    #[derive(Debug, Clone)]
    pub struct ExtensionRangeBumpalo<'bump> {
        pub start: i32,
        pub end: i32,
        pub options: ::std::option::Option<
            ::bumpalo::boxed::Box<'bump, super::ExtensionRangeOptionsBumpalo<'bump>>,
        >,
        puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ExtensionRangeBumpalo<'bump> {
        pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
            Self {
                start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
                puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(
                    bump,
                ),
            }
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
        for ExtensionRangeBumpalo<'bump>
    {
        fn met_field<'a, 'b, I>(
            &mut self,
            field: ::puroro_internal::types::FieldData<
                &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
            >,
            field_number: usize,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            use ::puroro_internal::helpers::MaybeRepeatedVariantField;
            match field {
                ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                    1 => {
                        *self.start.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *self.end.push_and_get_mut(&self.puroro_internal) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                    match field_number {
                        1 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
                        }
                        2 => {
                            let values = bytes_iter
                                .variants()
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
                            MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                        }
                        3 => {
                            let msg = self.options.push_and_get_mut(&self.puroro_internal);
                            bytes_iter.deser_message(msg)?;
                        }
                        _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                    }
                }
                ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                    1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                    1 | 2 | 3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::deser::DeserializableFromIter for ExtensionRangeBumpalo<'bump> {
        fn deserialize_from_bytes_iter<'a, I>(
            &mut self,
            mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
        ) -> ::puroro::Result<()>
        where
            I: Iterator<Item = ::std::io::Result<u8>>,
        {
            bytes_iter.deser_message(self)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::serializer::Serializable for ExtensionRangeBumpalo<'bump> {
        fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use ::puroro_internal::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                self.start.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                self.end.iter_for_ser().cloned().map(|v| Ok(v)),
            )?;
            for msg in self.options.iter_for_ser() {
                serializer.serialize_message_twice(3, msg)?;
            }
            Ok(())
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro::Serializable for ExtensionRangeBumpalo<'bump> {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_internal::serializer::default_serializer(write);
            <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ExtensionRangeTrait for ExtensionRangeBumpalo<'bump> {
        fn start(&self) -> i32 {
            self.start.clone()
        }
        fn end(&self) -> i32 {
            self.end.clone()
        }
        type OptionsType = super::ExtensionRangeOptionsBumpalo<'bump>;
        fn options(&self) -> ::std::option::Option<&'_ super::ExtensionRangeOptions> {
            self.options.as_deref()
        }
    }
    #[cfg(feature = "puroro-bumpalo")]
    impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for ExtensionRangeBumpalo<'bump> {
        fn new() -> Self {
            unimplemented!()
        }
        fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
            Self::new_in(bump)
        }
    }
    pub trait ExtensionRangeTrait {
        fn start(&'_ self) -> i32;
        fn end(&'_ self) -> i32;
        type OptionsType: super::ExtensionRangeOptionsTrait;
        fn options(&'_ self) -> ::std::option::Option<&'_ super::ExtensionRangeOptions>;
    }
    pub trait ExtensionRangeMutTrait {
        fn start_mut(&self) -> &mut i32;
        fn end_mut(&self) -> &mut i32;
        fn options_mut(
            &self,
        ) -> ::std::option::Option<&mut super::super::super::google::protobuf::ExtensionRangeOptions>;
    }
} // mod descriptor_proto

#[derive(Debug, Clone)]
pub struct FileDescriptorProto {
    pub name: ::std::string::String,
    pub package: ::std::string::String,
    pub dependency: ::std::vec::Vec<::std::string::String>,
    pub public_dependency: ::std::vec::Vec<i32>,
    pub weak_dependency: ::std::vec::Vec<i32>,
    pub message_type: ::std::vec::Vec<DescriptorProto>,
    pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,
    pub service: ::std::vec::Vec<ServiceDescriptorProto>,
    pub extension: ::std::vec::Vec<FieldDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<FileOptions>>,
    pub source_code_info: ::std::option::Option<::std::boxed::Box<SourceCodeInfo>>,
    pub syntax: ::std::string::String,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl FileDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            package: ::puroro_internal::helpers::FieldNew::new(),
            dependency: ::puroro_internal::helpers::FieldNew::new(),
            public_dependency: ::puroro_internal::helpers::FieldNew::new(),
            weak_dependency: ::puroro_internal::helpers::FieldNew::new(),
            message_type: ::puroro_internal::helpers::FieldNew::new(),
            enum_type: ::puroro_internal::helpers::FieldNew::new(),
            service: ::puroro_internal::helpers::FieldNew::new(),
            extension: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            source_code_info: ::puroro_internal::helpers::FieldNew::new(),
            syntax: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self
                        .public_dependency
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                11 => {
                    *self.weak_dependency.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        *self.dependency.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    10 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.public_dependency, first, iter);
                    }
                    11 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.weak_dependency, first, iter);
                    }
                    4 => {
                        let msg = self.message_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        let msg = self.enum_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    6 => {
                        let msg = self.service.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    7 => {
                        let msg = self.extension.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    8 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    9 => {
                        let msg = self
                            .source_code_info
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    12 => {
                        *self.syntax.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for FileDescriptorProto {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for FileDescriptorProto {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.package.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.dependency.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            10,
            self.public_dependency
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            11,
            self.weak_dependency.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
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
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileDescriptorProtoTrait for FileDescriptorProto {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn package(&self) -> &'_ str {
        self.package.as_ref()
    }
    fn for_each_dependency<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str),
    {
        for item in (self.dependency).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.dependency.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn dependency_iter(&self) -> Self::DependencyIter<'_> {
        self.dependency.iter().map(|v| v.as_ref())
    }
    fn for_each_public_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32),
    {
        for item in (self.public_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn public_dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
        ::std::boxed::Box::new(self.public_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a> = impl Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    fn public_dependency_iter(&self) -> Self::PublicDependencyIter<'_> {
        self.public_dependency.iter().cloned()
    }
    fn for_each_weak_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32),
    {
        for item in (self.weak_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn weak_dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
        ::std::boxed::Box::new(self.weak_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a> = impl Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_> {
        self.weak_dependency.iter().cloned()
    }
    type MessageTypeType = DescriptorProto;
    fn for_each_message_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ DescriptorProto),
    {
        for item in (self.message_type).iter() {
            (f)(item);
        }
    }
    fn message_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ DescriptorProto>> {
        ::std::boxed::Box::new(self.message_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a> = impl Iterator<Item = &'a DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_> {
        self.message_type.iter()
    }
    type EnumTypeType = EnumDescriptorProto;
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ EnumDescriptorProto),
    {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item = &'a EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    type ServiceType = ServiceDescriptorProto;
    fn for_each_service<F>(&self, mut f: F)
    where
        F: FnMut(&'_ ServiceDescriptorProto),
    {
        for item in (self.service).iter() {
            (f)(item);
        }
    }
    fn service_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ ServiceDescriptorProto>> {
        ::std::boxed::Box::new(self.service.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a> = impl Iterator<Item = &'a ServiceDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_> {
        self.service.iter()
    }
    type ExtensionType = FieldDescriptorProto;
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto),
    {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    type OptionsType = FileOptions;
    fn options(&self) -> ::std::option::Option<&'_ FileOptions> {
        self.options.as_deref()
    }
    type SourceCodeInfoType = SourceCodeInfo;
    fn source_code_info(&self) -> ::std::option::Option<&'_ SourceCodeInfo> {
        self.source_code_info.as_deref()
    }
    fn syntax(&self) -> &'_ str {
        self.syntax.as_ref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FileDescriptorProtoBumpalo<'bump> {
    pub name: ::bumpalo::collections::String<'bump>,
    pub package: ::bumpalo::collections::String<'bump>,
    pub dependency: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    pub public_dependency: ::bumpalo::collections::Vec<'bump, i32>,
    pub weak_dependency: ::bumpalo::collections::Vec<'bump, i32>,
    pub message_type: ::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>>,
    pub enum_type: ::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>>,
    pub service: ::bumpalo::collections::Vec<'bump, ServiceDescriptorProtoBumpalo<'bump>>,
    pub extension: ::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, FileOptionsBumpalo<'bump>>>,
    pub source_code_info:
        ::std::option::Option<::bumpalo::boxed::Box<'bump, SourceCodeInfoBumpalo<'bump>>>,
    pub syntax: ::bumpalo::collections::String<'bump>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            package: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            dependency: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            public_dependency: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            weak_dependency: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            message_type: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            enum_type: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            service: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            extension: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            source_code_info: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            syntax: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for FileDescriptorProtoBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *self
                        .public_dependency
                        .push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                11 => {
                    *self.weak_dependency.push_and_get_mut(&self.puroro_internal) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        *self.name.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        *self.package.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        *self.dependency.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    10 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.public_dependency, first, iter);
                    }
                    11 => {
                        let values = bytes_iter
                            .variants()
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
                        MaybeRepeatedVariantField::extend(&mut self.weak_dependency, first, iter);
                    }
                    4 => {
                        let msg = self.message_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    5 => {
                        let msg = self.enum_type.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    6 => {
                        let msg = self.service.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    7 => {
                        let msg = self.extension.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    8 => {
                        let msg = self.options.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    9 => {
                        let msg = self
                            .source_code_info
                            .push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    12 => {
                        *self.syntax.push_and_get_mut(&self.puroro_internal) =
                            bytes_iter.chars().collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 | 2 | 3 | 10 | 11 | 4 | 5 | 6 | 7 | 8 | 9 | 12 => {
                    Err(::puroro::PuroroError::UnexpectedWireType)?
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for FileDescriptorProtoBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for FileDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for string in self.name.iter_for_ser() {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.package.iter_for_ser() {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in self.dependency.iter_for_ser() {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            10,
            self.public_dependency
                .iter_for_ser()
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            11,
            self.weak_dependency.iter_for_ser().cloned().map(|v| Ok(v)),
        )?;
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
impl<'bump> ::puroro::Serializable for FileDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorProtoTrait for FileDescriptorProtoBumpalo<'bump> {
    fn name(&self) -> &'_ str {
        self.name.as_ref()
    }
    fn package(&self) -> &'_ str {
        self.package.as_ref()
    }
    fn for_each_dependency<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str),
    {
        for item in (self.dependency).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>> {
        ::std::boxed::Box::new(self.dependency.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a> = impl Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn dependency_iter(&self) -> Self::DependencyIter<'_> {
        self.dependency.iter().map(|v| v.as_ref())
    }
    fn for_each_public_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32),
    {
        for item in (self.public_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn public_dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
        ::std::boxed::Box::new(self.public_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a> = impl Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    fn public_dependency_iter(&self) -> Self::PublicDependencyIter<'_> {
        self.public_dependency.iter().cloned()
    }
    fn for_each_weak_dependency<F>(&self, mut f: F)
    where
        F: FnMut(i32),
    {
        for item in (self.weak_dependency).iter().cloned() {
            (f)(item);
        }
    }
    fn weak_dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>> {
        ::std::boxed::Box::new(self.weak_dependency.iter().cloned())
    }
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a> = impl Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_> {
        self.weak_dependency.iter().cloned()
    }
    type MessageTypeType = DescriptorProtoBumpalo<'bump>;
    fn for_each_message_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ DescriptorProto),
    {
        for item in (self.message_type).iter() {
            (f)(item);
        }
    }
    fn message_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ DescriptorProto>> {
        ::std::boxed::Box::new(self.message_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a> = impl Iterator<Item = &'a DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_> {
        self.message_type.iter()
    }
    type EnumTypeType = EnumDescriptorProtoBumpalo<'bump>;
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ EnumDescriptorProto),
    {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumDescriptorProto>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl Iterator<Item = &'a EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    type ServiceType = ServiceDescriptorProtoBumpalo<'bump>;
    fn for_each_service<F>(&self, mut f: F)
    where
        F: FnMut(&'_ ServiceDescriptorProto),
    {
        for item in (self.service).iter() {
            (f)(item);
        }
    }
    fn service_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ ServiceDescriptorProto>> {
        ::std::boxed::Box::new(self.service.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a> = impl Iterator<Item = &'a ServiceDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_> {
        self.service.iter()
    }
    type ExtensionType = FieldDescriptorProtoBumpalo<'bump>;
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto),
    {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    type OptionsType = FileOptionsBumpalo<'bump>;
    fn options(&self) -> ::std::option::Option<&'_ FileOptions> {
        self.options.as_deref()
    }
    type SourceCodeInfoType = SourceCodeInfoBumpalo<'bump>;
    fn source_code_info(&self) -> ::std::option::Option<&'_ SourceCodeInfo> {
        self.source_code_info.as_deref()
    }
    fn syntax(&self) -> &'_ str {
        self.syntax.as_ref()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for FileDescriptorProtoBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait FileDescriptorProtoTrait {
    fn name(&'_ self) -> &'_ str;
    fn package(&'_ self) -> &'_ str;
    fn for_each_dependency<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    fn dependency_iter(&self) -> Self::DependencyIter<'_>;
    fn for_each_public_dependency<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn public_dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>>;
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a>: Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    fn public_dependency_iter(&self) -> Self::PublicDependencyIter<'_>;
    fn for_each_weak_dependency<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn weak_dependency_boxed_iter(&self) -> ::std::boxed::Box<dyn '_ + Iterator<Item = i32>>;
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a>: Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_>;
    type MessageTypeType: DescriptorProtoTrait;
    fn for_each_message_type<F>(&self, f: F)
    where
        F: FnMut(&'_ DescriptorProto);
    fn message_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ DescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a>: Iterator<Item = &'a DescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_>;
    type EnumTypeType: EnumDescriptorProtoTrait;
    fn for_each_enum_type<F>(&self, f: F)
    where
        F: FnMut(&'_ EnumDescriptorProto);
    fn enum_type_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ EnumDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a>: Iterator<Item = &'a EnumDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_>;
    type ServiceType: ServiceDescriptorProtoTrait;
    fn for_each_service<F>(&self, f: F)
    where
        F: FnMut(&'_ ServiceDescriptorProto);
    fn service_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ ServiceDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a>: Iterator<Item = &'a ServiceDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_>;
    type ExtensionType: FieldDescriptorProtoTrait;
    fn for_each_extension<F>(&self, f: F)
    where
        F: FnMut(&'_ FieldDescriptorProto);
    fn extension_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FieldDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a>: Iterator<Item = &'a FieldDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_>;
    type OptionsType: FileOptionsTrait;
    fn options(&'_ self) -> ::std::option::Option<&'_ FileOptions>;
    type SourceCodeInfoType: SourceCodeInfoTrait;
    fn source_code_info(&'_ self) -> ::std::option::Option<&'_ SourceCodeInfo>;
    fn syntax(&'_ self) -> &'_ str;
}
pub trait FileDescriptorProtoMutTrait {
    fn name_mut(&self) -> &mut String;
    fn package_mut(&self) -> &mut String;
    fn for_each_dependency_mut<F>(&self, f: F)
    where
        F: FnMut(&mut String);
    fn dependency_boxed_iter_mut(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut String>>;
    // We need more!
    fn for_each_public_dependency_mut<F>(&self, f: F)
    where
        F: FnMut(&mut i32);
    fn public_dependency_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut i32>>;
    // We need more!
    fn for_each_weak_dependency_mut<F>(&self, f: F)
    where
        F: FnMut(&mut i32);
    fn weak_dependency_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &mut i32>>;
    // We need more!
    fn for_each_message_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::DescriptorProto);
    fn message_type_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::DescriptorProto>,
    >;
    // We need more!
    fn for_each_enum_type_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::EnumDescriptorProto);
    fn enum_type_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::EnumDescriptorProto>,
    >;
    // We need more!
    fn for_each_service_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::ServiceDescriptorProto);
    fn service_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::ServiceDescriptorProto>,
    >;
    // We need more!
    fn for_each_extension_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FieldDescriptorProto);
    fn extension_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::FieldDescriptorProto>,
    >;
    // We need more!
    fn options_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::FileOptions>;
    fn source_code_info_mut(
        &self,
    ) -> ::std::option::Option<&mut super::super::google::protobuf::SourceCodeInfo>;
    fn syntax_mut(&self) -> &mut String;
}

#[derive(Debug, Clone)]
pub struct FileDescriptorSet {
    pub file: ::std::vec::Vec<FileDescriptorProto>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct,
}

impl FileDescriptorSet {
    pub fn new() -> Self {
        Self {
            file: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::helpers::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorSet {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let msg = self.file.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}

impl ::puroro_internal::deser::DeserializableFromIter for FileDescriptorSet {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}

impl ::puroro_internal::serializer::Serializable for FileDescriptorSet {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.file.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}

impl ::puroro::Serializable for FileDescriptorSet {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileDescriptorSetTrait for FileDescriptorSet {
    type FileType = FileDescriptorProto;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FileDescriptorProto),
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FileDescriptorProto>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item = &'a FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileDescriptorSet {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct FileDescriptorSetBumpalo<'bump> {
    pub file: ::bumpalo::collections::Vec<'bump, FileDescriptorProtoBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorSetBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter
    for FileDescriptorSetBumpalo<'bump>
{
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>,
        >,
        field_number: usize,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        use ::puroro_internal::helpers::MaybeRepeatedVariantField;
        match field {
            ::puroro_internal::types::FieldData::Variant(variant) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::LengthDelimited(bytes_iter) => {
                match field_number {
                    1 => {
                        let msg = self.file.push_and_get_mut(&self.puroro_internal);
                        bytes_iter.deser_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                }
            }
            ::puroro_internal::types::FieldData::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro_internal::types::FieldData::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableFromIter for FileDescriptorSetBumpalo<'bump> {
    fn deserialize_from_bytes_iter<'a, I>(
        &mut self,
        mut bytes_iter: ::puroro_internal::deser::BytesIter<'a, I>,
    ) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        bytes_iter.deser_message(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::serializer::Serializable for FileDescriptorSetBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use ::puroro_internal::helpers::MaybeRepeatedField;
        for msg in self.file.iter_for_ser() {
            serializer.serialize_message_twice(1, msg)?;
        }
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileDescriptorSetBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::serializer::default_serializer(write);
        <Self as ::puroro_internal::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorSetTrait for FileDescriptorSetBumpalo<'bump> {
    type FileType = FileDescriptorProtoBumpalo<'bump>;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ FileDescriptorProto),
    {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FileDescriptorProto>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl Iterator<Item = &'a FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::helpers::FieldNew<'bump> for FileDescriptorSetBumpalo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}
pub trait FileDescriptorSetTrait {
    type FileType: FileDescriptorProtoTrait;
    fn for_each_file<F>(&self, f: F)
    where
        F: FnMut(&'_ FileDescriptorProto);
    fn file_boxed_iter(
        &self,
    ) -> ::std::boxed::Box<dyn '_ + Iterator<Item = &'_ FileDescriptorProto>>;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a>: Iterator<Item = &'a FileDescriptorProto>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_>;
}
pub trait FileDescriptorSetMutTrait {
    fn for_each_file_mut<F>(&self, f: F)
    where
        F: FnMut(&mut super::super::google::protobuf::FileDescriptorProto);
    fn file_boxed_iter_mut(
        &self,
    ) -> ::std::boxed::Box<
        dyn '_ + Iterator<Item = &mut super::super::google::protobuf::FileDescriptorProto>,
    >;
    // We need more!
}

pub mod compiler;
