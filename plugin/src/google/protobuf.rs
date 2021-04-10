pub struct GeneratedCodeInfo {
    annotation: ::std::vec::Vec<super::super::google::protobuf::generated_code_info::Annotation>,
}
impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        Self {
            annotation: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for GeneratedCodeInfo {
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
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.annotation.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::generated_code_info::Annotation as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for GeneratedCodeInfo {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod generated_code_info {
    pub struct Annotation {
        path: ::std::vec::Vec<i32>,
        source_file: String,
        begin: i32,
        end: i32,
    }
    impl ::std::default::Default for Annotation {
        fn default() -> Self {
            Self {
                path: ::std::default::Default::default(),
                source_file: ::std::default::Default::default(),
                begin: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for Annotation {
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
                        self.path.push(variant.to_native::<::puroro::tags::Int32>()?);
                    }
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => {
                        self.begin = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    4 => {
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.path.append(&mut ldd.deserialize_as_variants().map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                        }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                    }
                    2 => {
                        self.source_file = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        self.begin = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    4 => {
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
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
    impl ::puroro::Deserializable for Annotation {
        fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
                <Self as ::std::default::Default>::default())
        }
    }}
pub struct SourceCodeInfo {
    location: ::std::vec::Vec<super::super::google::protobuf::source_code_info::Location>,
}
impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        Self {
            location: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for SourceCodeInfo {
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
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.location.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::source_code_info::Location as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for SourceCodeInfo {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod source_code_info {
    pub struct Location {
        path: ::std::vec::Vec<i32>,
        span: ::std::vec::Vec<i32>,
        leading_comments: String,
        trailing_comments: String,
        leading_detached_comments: ::std::vec::Vec<String>,
    }
    impl ::std::default::Default for Location {
        fn default() -> Self {
            Self {
                path: ::std::default::Default::default(),
                span: ::std::default::Default::default(),
                leading_comments: ::std::default::Default::default(),
                trailing_comments: ::std::default::Default::default(),
                leading_detached_comments: ::std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for Location {
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
                        self.path.push(variant.to_native::<::puroro::tags::Int32>()?);
                    }
                    2 => {
                        self.span.push(variant.to_native::<::puroro::tags::Int32>()?);
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.path.append(&mut ldd.deserialize_as_variants().map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                        }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                    }
                    2 => {
                        self.span.append(&mut ldd.deserialize_as_variants().map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                        }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                    }
                    3 => {
                        self.leading_comments = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    4 => {
                        self.trailing_comments = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    6 => {
                        self.leading_detached_comments.push(ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?);
                    }
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for Location {
        fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
                <Self as ::std::default::Default>::default())
        }
    }}
pub struct UninterpretedOption {
    name: ::std::vec::Vec<super::super::google::protobuf::uninterpreted_option::NamePart>,
    identifier_value: String,
    positive_int_value: u64,
    negative_int_value: i64,
    double_value: f64,
    string_value: ::std::vec::Vec<u8>,
    aggregate_value: String,
}
impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
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
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for UninterpretedOption {
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
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => {
                    self.positive_int_value = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                5 => {
                    self.negative_int_value = variant.to_native::<::puroro::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                2 => {
                    self.name.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::uninterpreted_option::NamePart as ::std::default::Default>::default())?
                    );
                }
                3 => {
                    self.identifier_value = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    self.positive_int_value = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())?;
                }
                5 => {
                    self.negative_int_value = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int64>())?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => {
                    self.string_value = ldd.deserialize_as_bytes().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    self.aggregate_value = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => {
                    self.double_value = f64::from_le_bytes(bytes);
                }
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for UninterpretedOption {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod uninterpreted_option {
    pub struct NamePart {
        name_part: String,
        is_extension: bool,
    }
    impl ::std::default::Default for NamePart {
        fn default() -> Self {
            Self {
                name_part: ::std::default::Default::default(),
                is_extension: ::std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for NamePart {
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
                        self.is_extension = variant.to_native::<::puroro::tags::Bool>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.name_part = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        self.is_extension = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                    }
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for NamePart {
        fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
                <Self as ::std::default::Default>::default())
        }
    }}
pub struct MethodOptions {
    deprecated: bool,
    idempotency_level: ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        Self {
            deprecated: ::std::default::Default::default(),
            idempotency_level: 0i32.try_into(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for MethodOptions {
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
                33 => {
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                34 => {
                    self.idempotency_level = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                33 => {
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                34 => {
                    self.idempotency_level = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                34 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                34 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for MethodOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod method_options {
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        NoSideEffects = 1,
        Idempotent = 2,
    }
    impl std::convert::TryFrom<i32> for IdempotencyLevel {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::IdempotencyUnknown),
                1 => Ok(Self::NoSideEffects),
                2 => Ok(Self::Idempotent),
                x => Err(x),
            }
        }
    }
}
pub struct ServiceOptions {
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for ServiceOptions {
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
                33 => {
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                33 => {
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                33 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for ServiceOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct EnumValueOptions {
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for EnumValueOptions {
    fn default() -> Self {
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for EnumValueOptions {
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
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumValueOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct EnumOptions {
    allow_alias: bool,
    deprecated: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for EnumOptions {
    fn default() -> Self {
        Self {
            allow_alias: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for EnumOptions {
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
                2 => {
                    self.allow_alias = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                2 => {
                    self.allow_alias = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                3 => {
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct OneofOptions {
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        Self {
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for OneofOptions {
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
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for OneofOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct FieldOptions {
    ctype: ::std::result::Result<super::super::google::protobuf::field_options::CType, i32>,
    packed: bool,
    jstype: ::std::result::Result<super::super::google::protobuf::field_options::JSType, i32>,
    lazy: bool,
    deprecated: bool,
    weak: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
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
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for FieldOptions {
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
                    self.ctype = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                2 => {
                    self.packed = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    self.jstype = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                5 => {
                    self.lazy = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                10 => {
                    self.weak = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.ctype = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                2 => {
                    self.packed = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                6 => {
                    self.jstype = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                5 => {
                    self.lazy = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                3 => {
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                10 => {
                    self.weak = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FieldOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod field_options {
    pub enum JSType {
        JsNormal = 0,
        JsString = 1,
        JsNumber = 2,
    }
    impl std::convert::TryFrom<i32> for JSType {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::JsNormal),
                1 => Ok(Self::JsString),
                2 => Ok(Self::JsNumber),
                x => Err(x),
            }
        }
    }
    pub enum CType {
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
    impl std::convert::TryFrom<i32> for CType {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                0 => Ok(Self::String),
                1 => Ok(Self::Cord),
                2 => Ok(Self::StringPiece),
                x => Err(x),
            }
        }
    }
}
pub struct MessageOptions {
    message_set_wire_format: bool,
    no_standard_descriptor_accessor: bool,
    deprecated: bool,
    map_entry: bool,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for MessageOptions {
    fn default() -> Self {
        Self {
            message_set_wire_format: ::std::default::Default::default(),
            no_standard_descriptor_accessor: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            map_entry: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for MessageOptions {
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
                    self.message_set_wire_format = variant.to_native::<::puroro::tags::Bool>()?;
                }
                2 => {
                    self.no_standard_descriptor_accessor = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                7 => {
                    self.map_entry = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.message_set_wire_format = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                2 => {
                    self.no_standard_descriptor_accessor = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                3 => {
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                7 => {
                    self.map_entry = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for MessageOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct FileOptions {
    java_package: String,
    java_outer_classname: String,
    java_multiple_files: bool,
    java_generate_equals_and_hash: bool,
    java_string_check_utf8: bool,
    optimize_for: ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
    go_package: String,
    cc_generic_services: bool,
    java_generic_services: bool,
    py_generic_services: bool,
    php_generic_services: bool,
    deprecated: bool,
    cc_enable_arenas: bool,
    objc_class_prefix: String,
    csharp_namespace: String,
    swift_prefix: String,
    php_class_prefix: String,
    php_namespace: String,
    php_metadata_namespace: String,
    ruby_package: String,
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for FileOptions {
    fn default() -> Self {
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
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for FileOptions {
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
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    self.java_multiple_files = variant.to_native::<::puroro::tags::Bool>()?;
                }
                20 => {
                    self.java_generate_equals_and_hash = variant.to_native::<::puroro::tags::Bool>()?;
                }
                27 => {
                    self.java_string_check_utf8 = variant.to_native::<::puroro::tags::Bool>()?;
                }
                9 => {
                    self.optimize_for = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    self.cc_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                17 => {
                    self.java_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                18 => {
                    self.py_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                42 => {
                    self.php_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                23 => {
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                31 => {
                    self.cc_enable_arenas = variant.to_native::<::puroro::tags::Bool>()?;
                }
                36 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                37 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                39 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                40 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                41 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                44 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                45 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.java_package = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    self.java_outer_classname = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                10 => {
                    self.java_multiple_files = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                20 => {
                    self.java_generate_equals_and_hash = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                27 => {
                    self.java_string_check_utf8 = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                9 => {
                    self.optimize_for = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                11 => {
                    self.go_package = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                16 => {
                    self.cc_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                17 => {
                    self.java_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                18 => {
                    self.py_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                42 => {
                    self.php_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                23 => {
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                31 => {
                    self.cc_enable_arenas = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                36 => {
                    self.objc_class_prefix = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                37 => {
                    self.csharp_namespace = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                39 => {
                    self.swift_prefix = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                40 => {
                    self.php_class_prefix = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                41 => {
                    self.php_namespace = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                44 => {
                    self.php_metadata_namespace = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                45 => {
                    self.ruby_package = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FileOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod file_options {
    pub enum OptimizeMode {
        Speed = 1,
        CodeSize = 2,
        LiteRuntime = 3,
    }
    impl std::convert::TryFrom<i32> for OptimizeMode {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                1 => Ok(Self::Speed),
                2 => Ok(Self::CodeSize),
                3 => Ok(Self::LiteRuntime),
                x => Err(x),
            }
        }
    }
}
pub struct MethodDescriptorProto {
    name: String,
    input_type: String,
    output_type: String,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MethodOptions>>,
    client_streaming: bool,
    server_streaming: bool,
}
impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
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
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for MethodDescriptorProto {
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => {
                    self.client_streaming = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    self.server_streaming = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.input_type = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    self.output_type = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::MethodOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                5 => {
                    self.client_streaming = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                6 => {
                    self.server_streaming = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for MethodDescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct ServiceDescriptorProto {
    name: String,
    method: ::std::vec::Vec<super::super::google::protobuf::MethodDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::ServiceOptions>>,
}
impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        Self {
            name: ::std::default::Default::default(),
            method: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for ServiceDescriptorProto {
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.method.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::MethodDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                3 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::ServiceOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for ServiceDescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct EnumValueDescriptorProto {
    name: String,
    number: i32,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumValueOptions>>,
}
impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        Self {
            name: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for EnumValueDescriptorProto {
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
                    self.number = variant.to_native::<::puroro::tags::Int32>()?;
                }
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.number = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                3 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::EnumValueOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumValueDescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct EnumDescriptorProto {
    name: String,
    value: ::std::vec::Vec<super::super::google::protobuf::EnumValueDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumOptions>>,
    reserved_range: ::std::vec::Vec<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>,
    reserved_name: ::std::vec::Vec<String>,
}
impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        Self {
            name: ::std::default::Default::default(),
            value: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            reserved_range: ::std::default::Default::default(),
            reserved_name: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for EnumDescriptorProto {
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.value.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::EnumValueDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                3 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::EnumOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                4 => {
                    self.reserved_range.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange as ::std::default::Default>::default())?
                    );
                }
                5 => {
                    self.reserved_name.push(ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?);
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for EnumDescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod enum_descriptor_proto {
    pub struct EnumReservedRange {
        start: i32,
        end: i32,
    }
    impl ::std::default::Default for EnumReservedRange {
        fn default() -> Self {
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for EnumReservedRange {
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
                        self.start = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    2 => {
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.start = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    2 => {
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for EnumReservedRange {
        fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
                <Self as ::std::default::Default>::default())
        }
    }}
pub struct OneofDescriptorProto {
    name: String,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::OneofOptions>>,
}
impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        Self {
            name: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for OneofDescriptorProto {
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
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::OneofOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for OneofDescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct FieldDescriptorProto {
    name: String,
    number: i32,
    label: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>,
    type_: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>,
    type_name: String,
    extendee: String,
    default_value: String,
    oneof_index: i32,
    json_name: String,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FieldOptions>>,
    proto3_optional: bool,
}
impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
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
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for FieldDescriptorProto {
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
                3 => {
                    self.number = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    self.label = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                5 => {
                    self.type_ = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    self.oneof_index = variant.to_native::<::puroro::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    self.proto3_optional = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    self.number = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                4 => {
                    self.label = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                5 => {
                    self.type_ = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                6 => {
                    self.type_name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.extendee = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                7 => {
                    self.default_value = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                9 => {
                    self.oneof_index = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                10 => {
                    self.json_name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::FieldOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                17 => {
                    self.proto3_optional = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FieldDescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod field_descriptor_proto {
    pub enum Label {
        LabelOptional = 1,
        LabelRequired = 2,
        LabelRepeated = 3,
    }
    impl std::convert::TryFrom<i32> for Label {
        type Error = i32; 
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
            match val {
                1 => Ok(Self::LabelOptional),
                2 => Ok(Self::LabelRequired),
                3 => Ok(Self::LabelRepeated),
                x => Err(x),
            }
        }
    }
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
        fn try_from(val: i32) -> std::result::Result<Self, i32> {
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
}
pub struct ExtensionRangeOptions {
    uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        Self {
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for ExtensionRangeOptions {
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
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                999 => {
                    self.uninterpreted_option.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::UninterpretedOption as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for ExtensionRangeOptions {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct DescriptorProto {
    name: String,
    field: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    nested_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    extension_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ExtensionRange>,
    oneof_decl: ::std::vec::Vec<super::super::google::protobuf::OneofDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MessageOptions>>,
    reserved_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ReservedRange>,
    reserved_name: ::std::vec::Vec<String>,
}
impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
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
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for DescriptorProto {
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
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.field.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::FieldDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                6 => {
                    self.extension.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::FieldDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                3 => {
                    self.nested_type.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::DescriptorProto as ::std::default::Default>::default())?
                    );
                }
                4 => {
                    self.enum_type.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::EnumDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                5 => {
                    self.extension_range.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::descriptor_proto::ExtensionRange as ::std::default::Default>::default())?
                    );
                }
                8 => {
                    self.oneof_decl.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::OneofDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                7 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::MessageOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                9 => {
                    self.reserved_range.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::descriptor_proto::ReservedRange as ::std::default::Default>::default())?
                    );
                }
                10 => {
                    self.reserved_name.push(ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?);
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for DescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}mod descriptor_proto {
    pub struct ReservedRange {
        start: i32,
        end: i32,
    }
    impl ::std::default::Default for ReservedRange {
        fn default() -> Self {
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for ReservedRange {
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
                        self.start = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    2 => {
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.start = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    2 => {
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for ReservedRange {
        fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
                <Self as ::std::default::Default>::default())
        }
    }pub struct ExtensionRange {
        start: i32,
        end: i32,
        options: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::ExtensionRangeOptions>>,
    }
    impl ::std::default::Default for ExtensionRange {
        fn default() -> Self {
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
                options: ::std::default::Default::default(),
            }
        }
    }
    impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for ExtensionRange {
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
                        self.start = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    2 => {
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.start = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    2 => {
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(::puroro::PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    3 => {
                        let msg = self.options.get_or_insert_with(<super::super::super::google::protobuf::ExtensionRangeOptions as ::std::default::Default>::default);
                        self.options = Some(ldd.deserialize_as_message(msg)?);
                    }
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                    1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown filed number"),
                }
                _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
            }
            Ok(())
        }
    }
    impl ::puroro::Deserializable for ExtensionRange {
        fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
                <Self as ::std::default::Default>::default())
        }
    }}
pub struct FileDescriptorProto {
    name: String,
    package: String,
    dependency: ::std::vec::Vec<String>,
    public_dependency: ::std::vec::Vec<i32>,
    weak_dependency: ::std::vec::Vec<i32>,
    message_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    service: ::std::vec::Vec<super::super::google::protobuf::ServiceDescriptorProto>,
    extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FileOptions>>,
    source_code_info: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::SourceCodeInfo>>,
    syntax: String,
}
impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
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
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for FileDescriptorProto {
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
                3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                10 => {
                    self.public_dependency.push(variant.to_native::<::puroro::tags::Int32>()?);
                }
                11 => {
                    self.weak_dependency.push(variant.to_native::<::puroro::tags::Int32>()?);
                }
                4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                5 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                12 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                2 => {
                    self.package = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    self.dependency.push(ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?);
                }
                10 => {
                    self.public_dependency.append(&mut ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                }
                11 => {
                    self.weak_dependency.append(&mut ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                }
                4 => {
                    self.message_type.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::DescriptorProto as ::std::default::Default>::default())?
                    );
                }
                5 => {
                    self.enum_type.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::EnumDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                6 => {
                    self.service.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::ServiceDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                7 => {
                    self.extension.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::FieldDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                8 => {
                    let msg = self.options.get_or_insert_with(<super::super::google::protobuf::FileOptions as ::std::default::Default>::default);
                    self.options = Some(ldd.deserialize_as_message(msg)?);
                }
                9 => {
                    let msg = self.source_code_info.get_or_insert_with(<super::super::google::protobuf::SourceCodeInfo as ::std::default::Default>::default);
                    self.source_code_info = Some(ldd.deserialize_as_message(msg)?);
                }
                12 => {
                    self.syntax = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
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
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FileDescriptorProto {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}pub struct FileDescriptorSet {
    file: ::std::vec::Vec<super::super::google::protobuf::FileDescriptorProto>,
}
impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        Self {
            file: ::std::default::Default::default(),
        }
    }
}
impl ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for FileDescriptorSet {
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
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.file.push(ldd.deserialize_as_message(
                        <super::super::google::protobuf::FileDescriptorProto as ::std::default::Default>::default())?
                    );
                }
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits32(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            ::puroro_serializer::deserializer::stream::Field::Bits64(bytes) => match field_number {
                1 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown filed number"),
            }
            _ => Err(::puroro::PuroroError::UnexpectedFieldType)?,
        }
        Ok(())
    }
}
impl ::puroro::Deserializable for FileDescriptorSet {
    fn from_bytes<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> ::puroro::Result<Self> {
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(
            <Self as ::std::default::Default>::default())
    }
}