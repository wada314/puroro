pub mod compiler;
#[derive(Debug, Clone)]
pub struct GeneratedCodeInfo {
    pub annotation:
        ::std::vec::Vec<super::super::google::protobuf::generated_code_info::Annotation>,
}
impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            annotation: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut GeneratedCodeInfo
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
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.annotation);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for GeneratedCodeInfo {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <GeneratedCodeInfo as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for GeneratedCodeInfo {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for msg in MaybeRepeatedField::iter(&self.annotation) {
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
pub mod generated_code_info {
    #[derive(Debug, Clone)]
    pub struct Annotation {
        pub path: ::std::vec::Vec<i32>,
        pub source_file: ::std::string::String,
        pub begin: i32,
        pub end: i32,
    }
    impl ::std::default::Default for Annotation {
        fn default() -> Self {
            #[allow(unused)]
            use std::convert::TryInto;
            Self {
                path: ::std::default::Default::default(),
                source_file: ::std::default::Default::default(),
                begin: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut Annotation {
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
                        *MaybeRepeatedField::last_mut(&mut self.path) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => {
                        *MaybeRepeatedField::last_mut(&mut self.begin) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    4 => {
                        *MaybeRepeatedField::last_mut(&mut self.end) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
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
                        MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
                    }
                    2 => {
                        *MaybeRepeatedField::last_mut(&mut self.source_file) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
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
                        MaybeRepeatedVariantField::extend(&mut self.begin, first, iter);
                    }
                    4 => {
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
                        MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
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
    impl ::puroro::Deserializable for Annotation {
        fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            iter: I,
        ) -> ::puroro::Result<Self> {
            use puroro::deserializer::stream::Deserializer;
            let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            let mut msg = <Annotation as ::std::default::Default>::default();
            deserializer.deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro::serializer::Serializable for Annotation {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                MaybeRepeatedField::iter(&self.path).cloned().map(|v| Ok(v)),
            )?;
            for string in MaybeRepeatedField::iter(&self.source_file) {
                serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
            }
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                3,
                MaybeRepeatedField::iter(&self.begin)
                    .cloned()
                    .map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                4,
                MaybeRepeatedField::iter(&self.end).cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    impl ::puroro::Serializable for Annotation {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro::serializer::default_serializer(write);
            <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
} // mod generated_code_info
#[derive(Debug, Clone)]
pub struct SourceCodeInfo {
    pub location: ::std::vec::Vec<super::super::google::protobuf::source_code_info::Location>,
}
impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            location: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut SourceCodeInfo {
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
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.location);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for SourceCodeInfo {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <SourceCodeInfo as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for SourceCodeInfo {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for msg in MaybeRepeatedField::iter(&self.location) {
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
pub mod source_code_info {
    #[derive(Debug, Clone)]
    pub struct Location {
        pub path: ::std::vec::Vec<i32>,
        pub span: ::std::vec::Vec<i32>,
        pub leading_comments: ::std::string::String,
        pub trailing_comments: ::std::string::String,
        pub leading_detached_comments: ::std::vec::Vec<::std::string::String>,
    }
    impl ::std::default::Default for Location {
        fn default() -> Self {
            #[allow(unused)]
            use std::convert::TryInto;
            Self {
                path: ::std::default::Default::default(),
                span: ::std::default::Default::default(),
                leading_comments: ::std::default::Default::default(),
                trailing_comments: ::std::default::Default::default(),
                leading_detached_comments: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut Location {
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
                        *MaybeRepeatedField::last_mut(&mut self.path) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *MaybeRepeatedField::last_mut(&mut self.span) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
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
                        MaybeRepeatedVariantField::extend(&mut self.path, first, iter);
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
                        MaybeRepeatedVariantField::extend(&mut self.span, first, iter);
                    }
                    3 => {
                        *MaybeRepeatedField::last_mut(&mut self.leading_comments) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    4 => {
                        *MaybeRepeatedField::last_mut(&mut self.trailing_comments) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    6 => {
                        *MaybeRepeatedField::last_mut(&mut self.leading_detached_comments) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for Location {
        fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            iter: I,
        ) -> ::puroro::Result<Self> {
            use puroro::deserializer::stream::Deserializer;
            let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            let mut msg = <Location as ::std::default::Default>::default();
            deserializer.deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro::serializer::Serializable for Location {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                MaybeRepeatedField::iter(&self.path).cloned().map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                MaybeRepeatedField::iter(&self.span).cloned().map(|v| Ok(v)),
            )?;
            for string in MaybeRepeatedField::iter(&self.leading_comments) {
                serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
            }
            for string in MaybeRepeatedField::iter(&self.trailing_comments) {
                serializer.serialize_bytes_twice(4, string.bytes().map(|b| Ok(b)))?;
            }
            for string in MaybeRepeatedField::iter(&self.leading_detached_comments) {
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
}
impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            identifier_value: ::std::default::Default::default(),
            positive_int_value: ::std::default::Default::default(),
            negative_int_value: ::std::default::Default::default(),
            double_value: ::std::default::Default::default(),
            string_value: ::std::default::Default::default(),
            aggregate_value: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut UninterpretedOption
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
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    *MaybeRepeatedField::last_mut(&mut self.positive_int_value) =
                        variant.to_native::<::puroro_internal::tags::UInt64>()?;
                }
                5 => {
                    *MaybeRepeatedField::last_mut(&mut self.negative_int_value) =
                        variant.to_native::<::puroro_internal::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                2 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.name);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.identifier_value) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                4 => {
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
                    MaybeRepeatedVariantField::extend(&mut self.positive_int_value, first, iter);
                }
                5 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    MaybeRepeatedVariantField::extend(&mut self.negative_int_value, first, iter);
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => {
                    *MaybeRepeatedField::last_mut(&mut self.string_value) = ldd
                        .deserialize_as_bytes()
                        .collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *MaybeRepeatedField::last_mut(&mut self.aggregate_value) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => {
                    self.double_value = f64::from_le_bytes(bytes);
                }
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for UninterpretedOption {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <UninterpretedOption as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for UninterpretedOption {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for msg in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_message_twice(2, msg)?;
        }
        for string in MaybeRepeatedField::iter(&self.identifier_value) {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::UInt64, _>(
            4,
            MaybeRepeatedField::iter(&self.positive_int_value)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int64, _>(
            5,
            MaybeRepeatedField::iter(&self.negative_int_value)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for item in MaybeRepeatedField::iter(&self.double_value) {
            serializer.serialize_fixed_bits(6, item.to_le_bytes())?;
        }
        for bytes in MaybeRepeatedField::iter(&self.string_value) {
            serializer.serialize_bytes_twice(7, bytes.iter().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.aggregate_value) {
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
pub mod uninterpreted_option {
    #[derive(Debug, Clone)]
    pub struct NamePart {
        pub name_part: ::std::string::String,
        pub is_extension: bool,
    }
    impl ::std::default::Default for NamePart {
        fn default() -> Self {
            #[allow(unused)]
            use std::convert::TryInto;
            Self {
                name_part: ::std::default::Default::default(),
                is_extension: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut NamePart {
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
                        *MaybeRepeatedField::last_mut(&mut self.is_extension) =
                            variant.to_native::<::puroro_internal::tags::Bool>()?;
                    }
                },
                ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        *MaybeRepeatedField::last_mut(&mut self.name_part) = ldd
                            .deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        let values = ldd
                            .deserialize_as_variants()
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
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for NamePart {
        fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            iter: I,
        ) -> ::puroro::Result<Self> {
            use puroro::deserializer::stream::Deserializer;
            let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            let mut msg = <NamePart as ::std::default::Default>::default();
            deserializer.deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro::serializer::Serializable for NamePart {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            for string in MaybeRepeatedField::iter(&self.name_part) {
                serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
            }
            serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
                2,
                MaybeRepeatedField::iter(&self.is_extension)
                    .cloned()
                    .map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    impl ::puroro::Serializable for NamePart {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro::serializer::default_serializer(write);
            <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
} // mod uninterpreted_option
#[derive(Debug, Clone)]
pub struct MethodOptions {
    pub deprecated: bool,
    pub idempotency_level: ::std::result::Result<
        super::super::google::protobuf::method_options::IdempotencyLevel,
        i32,
    >,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            idempotency_level: 0i32.try_into(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut MethodOptions {
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
            ::puroro::deserializer::stream::Field::Variant(variant) => {
                match field_number {
                    33 => {
                        *MaybeRepeatedField::last_mut(&mut self.deprecated) =
                            variant.to_native::<::puroro_internal::tags::Bool>()?;
                    }
                    34 => {
                        *MaybeRepeatedField::last_mut(&mut self.idempotency_level) = variant.to_native::<::puroro_internal::tags::Enum<::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>>>()?;
                    }
                    999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                }
            }
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                33 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.idempotency_level, first, iter);
                }
                999 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                34 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                34 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for MethodOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <MethodOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for MethodOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            33,
            MaybeRepeatedField::iter(&self.deprecated)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            ::std::result::Result<
                super::super::google::protobuf::method_options::IdempotencyLevel,
                i32,
            >,
        >, _>(
            34,
            MaybeRepeatedField::iter(&self.idempotency_level)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
pub mod method_options {
    #[derive(Debug, Clone)]
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        NoSideEffects = 1,
        Idempotent = 2,
    }
    impl std::convert::TryFrom<i32> for IdempotencyLevel {
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
    impl std::convert::Into<i32> for IdempotencyLevel {
        fn into(self) -> i32 {
            self as i32
        }
    }
} // mod method_options
#[derive(Debug, Clone)]
pub struct ServiceOptions {
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut ServiceOptions {
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
                33 => {
                    *MaybeRepeatedField::last_mut(&mut self.deprecated) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                33 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for ServiceOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <ServiceOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for ServiceOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            33,
            MaybeRepeatedField::iter(&self.deprecated)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
#[derive(Debug, Clone)]
pub struct EnumValueOptions {
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for EnumValueOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut EnumValueOptions
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
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.deprecated) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumValueOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <EnumValueOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for EnumValueOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            1,
            MaybeRepeatedField::iter(&self.deprecated)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
#[derive(Debug, Clone)]
pub struct EnumOptions {
    pub allow_alias: bool,
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for EnumOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            allow_alias: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut EnumOptions {
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
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.allow_alias) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.deprecated) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                2 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <EnumOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for EnumOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            MaybeRepeatedField::iter(&self.allow_alias)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            MaybeRepeatedField::iter(&self.deprecated)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
#[derive(Debug, Clone)]
pub struct OneofOptions {
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut OneofOptions {
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
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                999 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for OneofOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <OneofOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for OneofOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
#[derive(Debug, Clone)]
pub struct FieldOptions {
    pub ctype: ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>,
    pub packed: bool,
    pub jstype: ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>,
    pub lazy: bool,
    pub deprecated: bool,
    pub weak: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            ctype: 0i32.try_into(),
            packed: ::std::default::Default::default(),
            jstype: 0i32.try_into(),
            lazy: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            weak: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut FieldOptions {
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
                    *MaybeRepeatedField::last_mut(&mut self.ctype) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            ::std::result::Result<
                                super::super::google::protobuf::field_options::Ctype,
                                i32,
                            >,
                        >>()?;
                }
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.packed) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                6 => {
                    *MaybeRepeatedField::last_mut(&mut self.jstype) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            ::std::result::Result<
                                super::super::google::protobuf::field_options::Jstype,
                                i32,
                            >,
                        >>()?;
                }
                5 => {
                    *MaybeRepeatedField::last_mut(&mut self.lazy) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.deprecated) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                10 => {
                    *MaybeRepeatedField::last_mut(&mut self.weak) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| {
                                variant.to_native::<::puroro_internal::tags::Enum<
                                    ::std::result::Result<
                                        super::super::google::protobuf::field_options::Ctype,
                                        i32,
                                    >,
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| {
                                variant.to_native::<::puroro_internal::tags::Enum<
                                    ::std::result::Result<
                                        super::super::google::protobuf::field_options::Jstype,
                                        i32,
                                    >,
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FieldOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <FieldOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for FieldOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            ::std::result::Result<super::super::google::protobuf::field_options::Ctype, i32>,
        >, _>(
            1,
            MaybeRepeatedField::iter(&self.ctype)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            MaybeRepeatedField::iter(&self.packed)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            ::std::result::Result<super::super::google::protobuf::field_options::Jstype, i32>,
        >, _>(
            6,
            MaybeRepeatedField::iter(&self.jstype)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            5,
            MaybeRepeatedField::iter(&self.lazy).cloned().map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            MaybeRepeatedField::iter(&self.deprecated)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            10,
            MaybeRepeatedField::iter(&self.weak).cloned().map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
pub mod field_options {
    #[derive(Debug, Clone)]
    pub enum Jstype {
        JsNormal = 0,
        JsString = 1,
        JsNumber = 2,
    }
    impl std::convert::TryFrom<i32> for Jstype {
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
    impl std::convert::Into<i32> for Jstype {
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
    impl std::convert::TryFrom<i32> for Ctype {
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
    impl std::convert::Into<i32> for Ctype {
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
}
impl ::std::default::Default for MessageOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            message_set_wire_format: ::std::default::Default::default(),
            no_standard_descriptor_accessor: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            map_entry: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut MessageOptions {
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
                    *MaybeRepeatedField::last_mut(&mut self.message_set_wire_format) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.no_standard_descriptor_accessor) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.deprecated) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                7 => {
                    *MaybeRepeatedField::last_mut(&mut self.map_entry) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for MessageOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <MessageOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for MessageOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            1,
            MaybeRepeatedField::iter(&self.message_set_wire_format)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            2,
            MaybeRepeatedField::iter(&self.no_standard_descriptor_accessor)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            3,
            MaybeRepeatedField::iter(&self.deprecated)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            7,
            MaybeRepeatedField::iter(&self.map_entry)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for FileOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
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
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut FileOptions {
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
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *MaybeRepeatedField::last_mut(&mut self.java_multiple_files) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                20 => {
                    *MaybeRepeatedField::last_mut(&mut self.java_generate_equals_and_hash) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                27 => {
                    *MaybeRepeatedField::last_mut(&mut self.java_string_check_utf8) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                9 => {
                    *MaybeRepeatedField::last_mut(&mut self.optimize_for) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            ::std::result::Result<
                                super::super::google::protobuf::file_options::OptimizeMode,
                                i32,
                            >,
                        >>()?;
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    *MaybeRepeatedField::last_mut(&mut self.cc_generic_services) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                17 => {
                    *MaybeRepeatedField::last_mut(&mut self.java_generic_services) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                18 => {
                    *MaybeRepeatedField::last_mut(&mut self.py_generic_services) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                42 => {
                    *MaybeRepeatedField::last_mut(&mut self.php_generic_services) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                23 => {
                    *MaybeRepeatedField::last_mut(&mut self.deprecated) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                31 => {
                    *MaybeRepeatedField::last_mut(&mut self.cc_enable_arenas) =
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
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.java_package) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    *MaybeRepeatedField::last_mut(&mut self.java_outer_classname) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    MaybeRepeatedVariantField::extend(&mut self.java_multiple_files, first, iter);
                }
                20 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
                        .map(|rv| {
                            rv.and_then(|variant| {
                                variant.to_native::<::puroro_internal::tags::Enum<
                                    ::std::result::Result<
                                        super::super::google::protobuf::file_options::OptimizeMode,
                                        i32,
                                    >,
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
                    *MaybeRepeatedField::last_mut(&mut self.go_package) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                16 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    MaybeRepeatedVariantField::extend(&mut self.cc_generic_services, first, iter);
                }
                17 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    MaybeRepeatedVariantField::extend(&mut self.java_generic_services, first, iter);
                }
                18 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    MaybeRepeatedVariantField::extend(&mut self.py_generic_services, first, iter);
                }
                42 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    MaybeRepeatedVariantField::extend(&mut self.php_generic_services, first, iter);
                }
                23 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
                    *MaybeRepeatedField::last_mut(&mut self.objc_class_prefix) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                37 => {
                    *MaybeRepeatedField::last_mut(&mut self.csharp_namespace) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                39 => {
                    *MaybeRepeatedField::last_mut(&mut self.swift_prefix) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                40 => {
                    *MaybeRepeatedField::last_mut(&mut self.php_class_prefix) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                41 => {
                    *MaybeRepeatedField::last_mut(&mut self.php_namespace) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                44 => {
                    *MaybeRepeatedField::last_mut(&mut self.php_metadata_namespace) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                45 => {
                    *MaybeRepeatedField::last_mut(&mut self.ruby_package) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                999 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                20 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                27 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                18 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                42 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                23 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                31 => Err(::puroro::PuroroError::UnexpectedWireType)?,
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
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                20 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                27 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                18 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                42 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                23 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                31 => Err(::puroro::PuroroError::UnexpectedWireType)?,
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
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FileOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <FileOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for FileOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.java_package) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.java_outer_classname) {
            serializer.serialize_bytes_twice(8, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            10,
            MaybeRepeatedField::iter(&self.java_multiple_files)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            20,
            MaybeRepeatedField::iter(&self.java_generate_equals_and_hash)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            27,
            MaybeRepeatedField::iter(&self.java_string_check_utf8)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
        >, _>(
            9,
            MaybeRepeatedField::iter(&self.optimize_for)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for string in MaybeRepeatedField::iter(&self.go_package) {
            serializer.serialize_bytes_twice(11, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            16,
            MaybeRepeatedField::iter(&self.cc_generic_services)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            17,
            MaybeRepeatedField::iter(&self.java_generic_services)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            18,
            MaybeRepeatedField::iter(&self.py_generic_services)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            42,
            MaybeRepeatedField::iter(&self.php_generic_services)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            23,
            MaybeRepeatedField::iter(&self.deprecated)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            31,
            MaybeRepeatedField::iter(&self.cc_enable_arenas)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for string in MaybeRepeatedField::iter(&self.objc_class_prefix) {
            serializer.serialize_bytes_twice(36, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.csharp_namespace) {
            serializer.serialize_bytes_twice(37, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.swift_prefix) {
            serializer.serialize_bytes_twice(39, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.php_class_prefix) {
            serializer.serialize_bytes_twice(40, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.php_namespace) {
            serializer.serialize_bytes_twice(41, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.php_metadata_namespace) {
            serializer.serialize_bytes_twice(44, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.ruby_package) {
            serializer.serialize_bytes_twice(45, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
pub mod file_options {
    #[derive(Debug, Clone)]
    pub enum OptimizeMode {
        Speed = 1,
        CodeSize = 2,
        LiteRuntime = 3,
    }
    impl std::convert::TryFrom<i32> for OptimizeMode {
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
    impl std::convert::Into<i32> for OptimizeMode {
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
    pub options:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::MethodOptions>>,
    pub client_streaming: bool,
    pub server_streaming: bool,
}
impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            input_type: ::std::default::Default::default(),
            output_type: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            client_streaming: ::std::default::Default::default(),
            server_streaming: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut MethodDescriptorProto
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    *MaybeRepeatedField::last_mut(&mut self.client_streaming) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
                6 => {
                    *MaybeRepeatedField::last_mut(&mut self.server_streaming) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.input_type) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.output_type) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let values = ldd
                        .deserialize_as_variants()
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
                    let values = ldd
                        .deserialize_as_variants()
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
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for MethodDescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <MethodDescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for MethodDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.input_type) {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.output_type) {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.options) {
            serializer.serialize_message_twice(4, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            5,
            MaybeRepeatedField::iter(&self.client_streaming)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            6,
            MaybeRepeatedField::iter(&self.server_streaming)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        Ok(())
    }
}
impl ::puroro::Serializable for MethodDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct ServiceDescriptorProto {
    pub name: ::std::string::String,
    pub method: ::std::vec::Vec<super::super::google::protobuf::MethodDescriptorProto>,
    pub options:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::ServiceOptions>>,
}
impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            method: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut ServiceDescriptorProto
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.method);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for ServiceDescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <ServiceDescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for ServiceDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.method) {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.options) {
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
#[derive(Debug, Clone)]
pub struct EnumValueDescriptorProto {
    pub name: ::std::string::String,
    pub number: i32,
    pub options: ::std::option::Optional<
        ::std::boxed::Box<super::super::google::protobuf::EnumValueOptions>,
    >,
}
impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut EnumValueDescriptorProto
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
                    *MaybeRepeatedField::last_mut(&mut self.number) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
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
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                3 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumValueDescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <EnumValueDescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for EnumValueDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            2,
            MaybeRepeatedField::iter(&self.number)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.options) {
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
#[derive(Debug, Clone)]
pub struct EnumDescriptorProto {
    pub name: ::std::string::String,
    pub value: ::std::vec::Vec<super::super::google::protobuf::EnumValueDescriptorProto>,
    pub options:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::EnumOptions>>,
    pub reserved_range:
        ::std::vec::Vec<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            value: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            reserved_range: ::std::default::Default::default(),
            reserved_name: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut EnumDescriptorProto
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.value);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                4 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.reserved_range);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    *MaybeRepeatedField::last_mut(&mut self.reserved_name) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumDescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <EnumDescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for EnumDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.value) {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.options) {
            serializer.serialize_message_twice(3, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.reserved_range) {
            serializer.serialize_message_twice(4, msg)?;
        }
        for string in MaybeRepeatedField::iter(&self.reserved_name) {
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
pub mod enum_descriptor_proto {
    #[derive(Debug, Clone)]
    pub struct EnumReservedRange {
        pub start: i32,
        pub end: i32,
    }
    impl ::std::default::Default for EnumReservedRange {
        fn default() -> Self {
            #[allow(unused)]
            use std::convert::TryInto;
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
        for &'a mut EnumReservedRange
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
                    1 => {
                        *MaybeRepeatedField::last_mut(&mut self.start) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *MaybeRepeatedField::last_mut(&mut self.end) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
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
                        MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
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
                        MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for EnumReservedRange {
        fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            iter: I,
        ) -> ::puroro::Result<Self> {
            use puroro::deserializer::stream::Deserializer;
            let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            let mut msg = <EnumReservedRange as ::std::default::Default>::default();
            deserializer.deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro::serializer::Serializable for EnumReservedRange {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                MaybeRepeatedField::iter(&self.start)
                    .cloned()
                    .map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                MaybeRepeatedField::iter(&self.end).cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    impl ::puroro::Serializable for EnumReservedRange {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro::serializer::default_serializer(write);
            <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
} // mod enum_descriptor_proto
#[derive(Debug, Clone)]
pub struct OneofDescriptorProto {
    pub name: ::std::string::String,
    pub options:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::OneofOptions>>,
}
impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut OneofDescriptorProto
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
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for OneofDescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <OneofDescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for OneofDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.options) {
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
    pub options:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::FieldOptions>>,
    pub proto3_optional: bool,
}
impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
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
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut FieldDescriptorProto
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
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.number) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                4 => {
                    *MaybeRepeatedField::last_mut(&mut self.label) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            ::std::result::Result<
                                super::super::google::protobuf::field_descriptor_proto::Label,
                                i32,
                            >,
                        >>()?;
                }
                5 => {
                    *MaybeRepeatedField::last_mut(&mut self.type_) =
                        variant.to_native::<::puroro_internal::tags::Enum<
                            ::std::result::Result<
                                super::super::google::protobuf::field_descriptor_proto::Type,
                                i32,
                            >,
                        >>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    *MaybeRepeatedField::last_mut(&mut self.oneof_index) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    *MaybeRepeatedField::last_mut(&mut self.proto3_optional) =
                        variant.to_native::<::puroro_internal::tags::Bool>()?;
                }
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
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
                    MaybeRepeatedVariantField::extend(&mut self.number, first, iter);
                }
                4 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.label, first, iter);
                }
                5 => {
                    let values = ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro_internal::tags::Enum<::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>>>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?;
                    let mut iter = values.into_iter();
                    let first = iter
                        .next()
                        .ok_or(::puroro::PuroroError::ZeroLengthPackedField)?;
                    MaybeRepeatedVariantField::extend(&mut self.type_, first, iter);
                }
                6 => {
                    *MaybeRepeatedField::last_mut(&mut self.type_name) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.extendee) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                7 => {
                    *MaybeRepeatedField::last_mut(&mut self.default_value) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                9 => {
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
                    MaybeRepeatedVariantField::extend(&mut self.oneof_index, first, iter);
                }
                10 => {
                    *MaybeRepeatedField::last_mut(&mut self.json_name) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                17 => {
                    let values = ldd
                        .deserialize_as_variants()
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
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FieldDescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <FieldDescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for FieldDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            3,
            MaybeRepeatedField::iter(&self.number)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            ::std::result::Result<
                super::super::google::protobuf::field_descriptor_proto::Label,
                i32,
            >,
        >, _>(
            4,
            MaybeRepeatedField::iter(&self.label)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Enum<
            ::std::result::Result<
                super::super::google::protobuf::field_descriptor_proto::Type,
                i32,
            >,
        >, _>(
            5,
            MaybeRepeatedField::iter(&self.type_)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for string in MaybeRepeatedField::iter(&self.type_name) {
            serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.extendee) {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.default_value) {
            serializer.serialize_bytes_twice(7, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            9,
            MaybeRepeatedField::iter(&self.oneof_index)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for string in MaybeRepeatedField::iter(&self.json_name) {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.options) {
            serializer.serialize_message_twice(8, msg)?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Bool, _>(
            17,
            MaybeRepeatedField::iter(&self.proto3_optional)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        Ok(())
    }
}
impl ::puroro::Serializable for FieldDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro::serializer::default_serializer(write);
        <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
pub mod field_descriptor_proto {
    #[derive(Debug, Clone)]
    pub enum Label {
        LabelOptional = 1,
        LabelRequired = 2,
        LabelRepeated = 3,
    }
    impl std::convert::TryFrom<i32> for Label {
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
    impl std::convert::Into<i32> for Label {
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
    impl std::convert::TryFrom<i32> for Type {
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
    impl std::convert::Into<i32> for Type {
        fn into(self) -> i32 {
            self as i32
        }
    }
} // mod field_descriptor_proto
#[derive(Debug, Clone)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut ExtensionRangeOptions
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
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                999 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.uninterpreted_option);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for ExtensionRangeOptions {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <ExtensionRangeOptions as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for ExtensionRangeOptions {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for msg in MaybeRepeatedField::iter(&self.uninterpreted_option) {
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
#[derive(Debug, Clone)]
pub struct DescriptorProto {
    pub name: ::std::string::String,
    pub field: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    pub extension_range:
        ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec<super::super::google::protobuf::OneofDescriptorProto>,
    pub options:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::MessageOptions>>,
    pub reserved_range:
        ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
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
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut DescriptorProto
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
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.field);
                    ldd.deserialize_as_message(msg)?;
                }
                6 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.extension);
                    ldd.deserialize_as_message(msg)?;
                }
                3 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.nested_type);
                    ldd.deserialize_as_message(msg)?;
                }
                4 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.enum_type);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.extension_range);
                    ldd.deserialize_as_message(msg)?;
                }
                8 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.oneof_decl);
                    ldd.deserialize_as_message(msg)?;
                }
                7 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                9 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.reserved_range);
                    ldd.deserialize_as_message(msg)?;
                }
                10 => {
                    *MaybeRepeatedField::last_mut(&mut self.reserved_name) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
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
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
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
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for DescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <DescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for DescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for msg in MaybeRepeatedField::iter(&self.field) {
            serializer.serialize_message_twice(2, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.extension) {
            serializer.serialize_message_twice(6, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.nested_type) {
            serializer.serialize_message_twice(3, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.enum_type) {
            serializer.serialize_message_twice(4, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.extension_range) {
            serializer.serialize_message_twice(5, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.oneof_decl) {
            serializer.serialize_message_twice(8, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.options) {
            serializer.serialize_message_twice(7, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.reserved_range) {
            serializer.serialize_message_twice(9, msg)?;
        }
        for string in MaybeRepeatedField::iter(&self.reserved_name) {
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
pub mod descriptor_proto {
    #[derive(Debug, Clone)]
    pub struct ReservedRange {
        pub start: i32,
        pub end: i32,
    }
    impl ::std::default::Default for ReservedRange {
        fn default() -> Self {
            #[allow(unused)]
            use std::convert::TryInto;
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut ReservedRange {
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
                        *MaybeRepeatedField::last_mut(&mut self.start) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *MaybeRepeatedField::last_mut(&mut self.end) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
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
                        MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
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
                        MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for ReservedRange {
        fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            iter: I,
        ) -> ::puroro::Result<Self> {
            use puroro::deserializer::stream::Deserializer;
            let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            let mut msg = <ReservedRange as ::std::default::Default>::default();
            deserializer.deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro::serializer::Serializable for ReservedRange {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                MaybeRepeatedField::iter(&self.start)
                    .cloned()
                    .map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                MaybeRepeatedField::iter(&self.end).cloned().map(|v| Ok(v)),
            )?;
            Ok(())
        }
    }
    impl ::puroro::Serializable for ReservedRange {
        fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro::serializer::default_serializer(write);
            <Self as ::puroro::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[derive(Debug, Clone)]
    pub struct ExtensionRange {
        pub start: i32,
        pub end: i32,
        pub options: ::std::option::Optional<
            ::std::boxed::Box<super::super::super::google::protobuf::ExtensionRangeOptions>,
        >,
    }
    impl ::std::default::Default for ExtensionRange {
        fn default() -> Self {
            #[allow(unused)]
            use std::convert::TryInto;
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
                options: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler for &'a mut ExtensionRange {
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
                        *MaybeRepeatedField::last_mut(&mut self.start) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    2 => {
                        *MaybeRepeatedField::last_mut(&mut self.end) =
                            variant.to_native::<::puroro_internal::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
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
                        MaybeRepeatedVariantField::extend(&mut self.start, first, iter);
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
                        MaybeRepeatedVariantField::extend(&mut self.end, first, iter);
                    }
                    3 => {
                        let msg = MaybeRepeatedField::last_mut(&mut self.options);
                        ldd.deserialize_as_message(msg)?;
                    }
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
                },
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for ExtensionRange {
        fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(
            iter: I,
        ) -> ::puroro::Result<Self> {
            use puroro::deserializer::stream::Deserializer;
            let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
            let mut msg = <ExtensionRange as ::std::default::Default>::default();
            deserializer.deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro::serializer::Serializable for ExtensionRange {
        fn serialize<T: ::puroro::serializer::MessageSerializer>(
            &self,
            serializer: &mut T,
        ) -> ::puroro::Result<()> {
            use puroro::helpers::MaybeRepeatedField;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                1,
                MaybeRepeatedField::iter(&self.start)
                    .cloned()
                    .map(|v| Ok(v)),
            )?;
            serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
                2,
                MaybeRepeatedField::iter(&self.end).cloned().map(|v| Ok(v)),
            )?;
            for msg in MaybeRepeatedField::iter(&self.options) {
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
    pub options:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::FileOptions>>,
    pub source_code_info:
        ::std::option::Optional<::std::boxed::Box<super::super::google::protobuf::SourceCodeInfo>>,
    pub syntax: ::std::string::String,
}
impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
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
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut FileDescriptorProto
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    *MaybeRepeatedField::last_mut(&mut self.public_dependency) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                11 => {
                    *MaybeRepeatedField::last_mut(&mut self.weak_dependency) =
                        variant.to_native::<::puroro_internal::tags::Int32>()?;
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    *MaybeRepeatedField::last_mut(&mut self.name) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    *MaybeRepeatedField::last_mut(&mut self.package) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    *MaybeRepeatedField::last_mut(&mut self.dependency) = ldd
                        .deserialize_as_chars()
                        .collect::<::puroro::Result<_>>()?;
                }
                10 => {
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
                    MaybeRepeatedVariantField::extend(&mut self.public_dependency, first, iter);
                }
                11 => {
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
                    MaybeRepeatedVariantField::extend(&mut self.weak_dependency, first, iter);
                }
                4 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.message_type);
                    ldd.deserialize_as_message(msg)?;
                }
                5 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.enum_type);
                    ldd.deserialize_as_message(msg)?;
                }
                6 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.service);
                    ldd.deserialize_as_message(msg)?;
                }
                7 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.extension);
                    ldd.deserialize_as_message(msg)?;
                }
                8 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.options);
                    ldd.deserialize_as_message(msg)?;
                }
                9 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.source_code_info);
                    ldd.deserialize_as_message(msg)?;
                }
                12 => {
                    *MaybeRepeatedField::last_mut(&mut self.syntax) =
                        ldd.deserialize_as_chars()
                            .collect::<::puroro::Result<_>>()?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FileDescriptorProto {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <FileDescriptorProto as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for FileDescriptorProto {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for string in MaybeRepeatedField::iter(&self.name) {
            serializer.serialize_bytes_twice(1, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.package) {
            serializer.serialize_bytes_twice(2, string.bytes().map(|b| Ok(b)))?;
        }
        for string in MaybeRepeatedField::iter(&self.dependency) {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            10,
            MaybeRepeatedField::iter(&self.public_dependency)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        serializer.serialize_variants_twice::<::puroro_internal::tags::Int32, _>(
            11,
            MaybeRepeatedField::iter(&self.weak_dependency)
                .cloned()
                .map(|v| Ok(v)),
        )?;
        for msg in MaybeRepeatedField::iter(&self.message_type) {
            serializer.serialize_message_twice(4, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.enum_type) {
            serializer.serialize_message_twice(5, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.service) {
            serializer.serialize_message_twice(6, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.extension) {
            serializer.serialize_message_twice(7, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.options) {
            serializer.serialize_message_twice(8, msg)?;
        }
        for msg in MaybeRepeatedField::iter(&self.source_code_info) {
            serializer.serialize_message_twice(9, msg)?;
        }
        for string in MaybeRepeatedField::iter(&self.syntax) {
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
#[derive(Debug, Clone)]
pub struct FileDescriptorSet {
    pub file: ::std::vec::Vec<super::super::google::protobuf::FileDescriptorProto>,
}
impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        #[allow(unused)]
        use std::convert::TryInto;
        Self {
            file: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro::deserializer::stream::MessageDeserializeEventHandler
    for &'a mut FileDescriptorSet
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
            },
            ::puroro::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    let msg = MaybeRepeatedField::last_mut(&mut self.file);
                    ldd.deserialize_as_message(msg)?;
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            ::puroro::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => Err(::puroro::PuroroError::UnexpectedFieldId)?,
            },
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FileDescriptorSet {
    fn from_bytes<I: Iterator<Item = ::std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        use puroro::deserializer::stream::Deserializer;
        let mut deserializer = ::puroro::deserializer::stream::deserializer_from_bytes(iter);
        let mut msg = <FileDescriptorSet as ::std::default::Default>::default();
        deserializer.deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro::serializer::Serializable for FileDescriptorSet {
    fn serialize<T: ::puroro::serializer::MessageSerializer>(
        &self,
        serializer: &mut T,
    ) -> ::puroro::Result<()> {
        use puroro::helpers::MaybeRepeatedField;
        for msg in MaybeRepeatedField::iter(&self.file) {
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
