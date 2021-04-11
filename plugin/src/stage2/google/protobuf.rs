pub mod compiler;
#[derive(Debug, Clone)]
pub struct GeneratedCodeInfo {
    pub annotation: ::std::vec::Vec<super::super::google::protobuf::generated_code_info::Annotation>,
}
impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            annotation: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut GeneratedCodeInfo {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.annotation.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for GeneratedCodeInfo {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for msg in &self.annotation {
            serializer.serialize_message_twice::<super::super::google::protobuf::generated_code_info::Annotation>(1, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for GeneratedCodeInfo {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod generated_code_info {
    #[derive(Debug, Clone)]
    pub struct Annotation {
        pub path: ::std::vec::Vec<i32>,
        pub source_file: String,
        pub begin: i32,
        pub end: i32,
    }
    impl ::std::default::Default for Annotation {
        fn default() -> Self {
            #[allow(unused)]
            use ::std::convert::TryInto;
            Self {
                path: ::std::default::Default::default(),
                source_file: ::std::default::Default::default(),
                begin: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut Annotation {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
        }
        fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro_serializer::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            match field {
                ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.path.push(variant.to_native::<::puroro::tags::Int32>()?);
                    }
                    2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    3 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.begin = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    4 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.path.append(&mut ldd.deserialize_as_variants().map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                        }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                    }
                    2 => {
                        self.source_file = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    3 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.begin = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    4 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
            use ::puroro_serializer::deserializer::stream::Deserializer;
            let mut msg = <Self as ::std::default::Default>::default();
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro_serializer::serializer::Serializable for Annotation {
        fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
            &self, serializer: &mut T) -> ::puroro::Result<()>
        {
            unimplemented!("Serializer for something else");
            serializer.serialize_bytes_twice(2, self.source_file.bytes().map(|b| Ok(b)))?;
            unimplemented!("Serializer for something else");
            unimplemented!("Serializer for something else");
            Ok(())
        }
    }
    impl ::puroro::Serializable for Annotation {
        fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
            <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
}
#[derive(Debug, Clone)]
pub struct SourceCodeInfo {
    pub location: ::std::vec::Vec<super::super::google::protobuf::source_code_info::Location>,
}
impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            location: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut SourceCodeInfo {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.location.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for SourceCodeInfo {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for msg in &self.location {
            serializer.serialize_message_twice::<super::super::google::protobuf::source_code_info::Location>(1, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for SourceCodeInfo {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod source_code_info {
    #[derive(Debug, Clone)]
    pub struct Location {
        pub path: ::std::vec::Vec<i32>,
        pub span: ::std::vec::Vec<i32>,
        pub leading_comments: String,
        pub trailing_comments: String,
        pub leading_detached_comments: ::std::vec::Vec<String>,
    }
    impl ::std::default::Default for Location {
        fn default() -> Self {
            #[allow(unused)]
            use ::std::convert::TryInto;
            Self {
                path: ::std::default::Default::default(),
                span: ::std::default::Default::default(),
                leading_comments: ::std::default::Default::default(),
                trailing_comments: ::std::default::Default::default(),
                leading_detached_comments: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut Location {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
        }
        fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro_serializer::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            match field {
                ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.path.push(variant.to_native::<::puroro::tags::Int32>()?);
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.span.push(variant.to_native::<::puroro::tags::Int32>()?);
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    4 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.path.append(&mut ldd.deserialize_as_variants().map(|rv| {
                            rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                        }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
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
            use ::puroro_serializer::deserializer::stream::Deserializer;
            let mut msg = <Self as ::std::default::Default>::default();
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro_serializer::serializer::Serializable for Location {
        fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
            &self, serializer: &mut T) -> ::puroro::Result<()>
        {
            unimplemented!("Serializer for something else");
            unimplemented!("Serializer for something else");
            serializer.serialize_bytes_twice(3, self.leading_comments.bytes().map(|b| Ok(b)))?;
            serializer.serialize_bytes_twice(4, self.trailing_comments.bytes().map(|b| Ok(b)))?;
            for string in &self.leading_detached_comments {
                serializer.serialize_bytes_twice(6, string.bytes().map(|b| Ok(b)))?;
            }
            Ok(())
        }
    }
    impl ::puroro::Serializable for Location {
        fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
            <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
}
#[derive(Debug, Clone)]
pub struct UninterpretedOption {
    pub name: ::std::vec::Vec<super::super::google::protobuf::uninterpreted_option::NamePart>,
    pub identifier_value: String,
    pub positive_int_value: u64,
    pub negative_int_value: i64,
    pub double_value: f64,
    pub string_value: ::std::vec::Vec<u8>,
    pub aggregate_value: String,
}
impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
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
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut UninterpretedOption {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.positive_int_value = variant.to_native::<::puroro::tags::UInt64>()?;
                }
                5 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.negative_int_value = variant.to_native::<::puroro::tags::Int64>()?;
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                2 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.name.push(msg);
                }
                3 => {
                    self.identifier_value = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                4 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.positive_int_value = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::UInt64>())?;
                }
                5 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.negative_int_value = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for UninterpretedOption {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for msg in &self.name {
            serializer.serialize_message_twice::<super::super::google::protobuf::uninterpreted_option::NamePart>(2, msg)?;
        }
        serializer.serialize_bytes_twice(3, self.identifier_value.bytes().map(|b| Ok(b)))?;
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for bits64");
        serializer.serialize_bytes_twice(7, self.string_value.iter().map(|b| Ok(*b)))?;
        serializer.serialize_bytes_twice(8, self.aggregate_value.bytes().map(|b| Ok(b)))?;
        Ok(())
    }
}
impl ::puroro::Serializable for UninterpretedOption {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod uninterpreted_option {
    #[derive(Debug, Clone)]
    pub struct NamePart {
        pub name_part: String,
        pub is_extension: bool,
    }
    impl ::std::default::Default for NamePart {
        fn default() -> Self {
            #[allow(unused)]
            use ::std::convert::TryInto;
            Self {
                name_part: ::std::default::Default::default(),
                is_extension: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut NamePart {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
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
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.is_extension = variant.to_native::<::puroro::tags::Bool>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        self.name_part = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.is_extension = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
            use ::puroro_serializer::deserializer::stream::Deserializer;
            let mut msg = <Self as ::std::default::Default>::default();
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro_serializer::serializer::Serializable for NamePart {
        fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
            &self, serializer: &mut T) -> ::puroro::Result<()>
        {
            serializer.serialize_bytes_twice(1, self.name_part.bytes().map(|b| Ok(b)))?;
            unimplemented!("Serializer for something else");
            Ok(())
        }
    }
    impl ::puroro::Serializable for NamePart {
        fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
            <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
}
#[derive(Debug, Clone)]
pub struct MethodOptions {
    pub deprecated: bool,
    pub idempotency_level: ::std::result::Result<super::super::google::protobuf::method_options::IdempotencyLevel, i32>,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            idempotency_level: 0i32.try_into(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut MethodOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                33 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                34 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.idempotency_level = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                33 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                34 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.idempotency_level = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?.try_into();
                }
                999 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for MethodOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        unimplemented!("Serializer for something else");
        serializer.serialize_variant::<::puroro::tags::Int32>(
            34, 
            self.idempotency_level.clone().map_or_else(|e| e, |v| v as i32)
        )?;
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for MethodOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod method_options {
    #[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ServiceOptions {
    pub deprecated: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut ServiceOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                33 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                33 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for ServiceOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        unimplemented!("Serializer for something else");
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for ServiceOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
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
        use ::std::convert::TryInto;
        Self {
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut EnumValueOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for EnumValueOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        unimplemented!("Serializer for something else");
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for EnumValueOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
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
        use ::std::convert::TryInto;
        Self {
            allow_alias: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut EnumOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                2 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.allow_alias = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                2 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.allow_alias = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                3 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for EnumOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for EnumOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct OneofOptions {
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut OneofOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for OneofOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for OneofOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct FieldOptions {
    pub ctype: ::std::result::Result<super::super::google::protobuf::field_options::CType, i32>,
    pub packed: bool,
    pub jstype: ::std::result::Result<super::super::google::protobuf::field_options::JSType, i32>,
    pub lazy: bool,
    pub deprecated: bool,
    pub weak: bool,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
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
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut FieldOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.ctype = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                2 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.packed = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.jstype = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                5 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.lazy = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                10 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.weak = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.ctype = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?.try_into();
                }
                2 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.packed = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                6 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.jstype = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?.try_into();
                }
                5 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.lazy = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                3 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                10 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.weak = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for FieldOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_variant::<::puroro::tags::Int32>(
            1, 
            self.ctype.clone().map_or_else(|e| e, |v| v as i32)
        )?;
        unimplemented!("Serializer for something else");
        serializer.serialize_variant::<::puroro::tags::Int32>(
            6, 
            self.jstype.clone().map_or_else(|e| e, |v| v as i32)
        )?;
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for FieldOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod field_options {
    #[derive(Debug, Clone)]
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
    #[derive(Debug, Clone)]
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
        use ::std::convert::TryInto;
        Self {
            message_set_wire_format: ::std::default::Default::default(),
            no_standard_descriptor_accessor: ::std::default::Default::default(),
            deprecated: ::std::default::Default::default(),
            map_entry: ::std::default::Default::default(),
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut MessageOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
    }
    fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
        &mut self,
        field: ::puroro_serializer::deserializer::stream::Field<T>,
        field_number: usize,
    ) -> ::puroro::Result<()> {
        match field {
            ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                1 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.message_set_wire_format = variant.to_native::<::puroro::tags::Bool>()?;
                }
                2 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.no_standard_descriptor_accessor = variant.to_native::<::puroro::tags::Bool>()?;
                }
                3 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                7 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.map_entry = variant.to_native::<::puroro::tags::Bool>()?;
                }
                999 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.message_set_wire_format = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                2 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.no_standard_descriptor_accessor = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                3 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                7 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.map_entry = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                999 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for MessageOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for MessageOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct FileOptions {
    pub java_package: String,
    pub java_outer_classname: String,
    pub java_multiple_files: bool,
    pub java_generate_equals_and_hash: bool,
    pub java_string_check_utf8: bool,
    pub optimize_for: ::std::result::Result<super::super::google::protobuf::file_options::OptimizeMode, i32>,
    pub go_package: String,
    pub cc_generic_services: bool,
    pub java_generic_services: bool,
    pub py_generic_services: bool,
    pub php_generic_services: bool,
    pub deprecated: bool,
    pub cc_enable_arenas: bool,
    pub objc_class_prefix: String,
    pub csharp_namespace: String,
    pub swift_prefix: String,
    pub php_class_prefix: String,
    pub php_namespace: String,
    pub php_metadata_namespace: String,
    pub ruby_package: String,
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for FileOptions {
    fn default() -> Self {
        #[allow(unused)]
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
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut FileOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_multiple_files = variant.to_native::<::puroro::tags::Bool>()?;
                }
                20 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_generate_equals_and_hash = variant.to_native::<::puroro::tags::Bool>()?;
                }
                27 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_string_check_utf8 = variant.to_native::<::puroro::tags::Bool>()?;
                }
                9 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.optimize_for = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                11 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                16 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.cc_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                17 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                18 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.py_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                42 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.php_generic_services = variant.to_native::<::puroro::tags::Bool>()?;
                }
                23 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = variant.to_native::<::puroro::tags::Bool>()?;
                }
                31 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_multiple_files = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                20 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_generate_equals_and_hash = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                27 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_string_check_utf8 = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                9 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.optimize_for = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?.try_into();
                }
                11 => {
                    self.go_package = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                16 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.cc_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                17 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.java_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                18 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.py_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                42 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.php_generic_services = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                23 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.deprecated = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                31 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.cc_enable_arenas = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for FileOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.java_package.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(8, self.java_outer_classname.bytes().map(|b| Ok(b)))?;
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        serializer.serialize_variant::<::puroro::tags::Int32>(
            9, 
            self.optimize_for.clone().map_or_else(|e| e, |v| v as i32)
        )?;
        serializer.serialize_bytes_twice(11, self.go_package.bytes().map(|b| Ok(b)))?;
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        serializer.serialize_bytes_twice(36, self.objc_class_prefix.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(37, self.csharp_namespace.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(39, self.swift_prefix.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(40, self.php_class_prefix.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(41, self.php_namespace.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(44, self.php_metadata_namespace.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(45, self.ruby_package.bytes().map(|b| Ok(b)))?;
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for FileOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod file_options {
    #[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct MethodDescriptorProto {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MethodOptions>>,
    pub client_streaming: bool,
    pub server_streaming: bool,
}
impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
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
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut MethodDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.client_streaming = variant.to_native::<::puroro::tags::Bool>()?;
                }
                6 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
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
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
                }
                5 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.client_streaming = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Bool>())?;
                }
                6 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.server_streaming = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for MethodDescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(2, self.input_type.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(3, self.output_type.bytes().map(|b| Ok(b)))?;
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::MethodOptions>(4, msg)?;
        }
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        Ok(())
    }
}
impl ::puroro::Serializable for MethodDescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct ServiceDescriptorProto {
    pub name: String,
    pub method: ::std::vec::Vec<super::super::google::protobuf::MethodDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::ServiceOptions>>,
}
impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            method: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut ServiceDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.method.push(msg);
                }
                3 => {
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for ServiceDescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        for msg in &self.method {
            serializer.serialize_message_twice::<super::super::google::protobuf::MethodDescriptorProto>(2, msg)?;
        }
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::ServiceOptions>(3, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for ServiceDescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct EnumValueDescriptorProto {
    pub name: String,
    pub number: i32,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumValueOptions>>,
}
impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut EnumValueDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.number = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                3 => {
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for EnumValueDescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        unimplemented!("Serializer for something else");
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::EnumValueOptions>(3, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for EnumValueDescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct EnumDescriptorProto {
    pub name: String,
    pub value: ::std::vec::Vec<super::super::google::protobuf::EnumValueDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::EnumOptions>>,
    pub reserved_range: ::std::vec::Vec<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec<String>,
}
impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            value: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            reserved_range: ::std::default::Default::default(),
            reserved_name: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut EnumDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.value.push(msg);
                }
                3 => {
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
                }
                4 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.reserved_range.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for EnumDescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        for msg in &self.value {
            serializer.serialize_message_twice::<super::super::google::protobuf::EnumValueDescriptorProto>(2, msg)?;
        }
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::EnumOptions>(3, msg)?;
        }
        for msg in &self.reserved_range {
            serializer.serialize_message_twice::<super::super::google::protobuf::enum_descriptor_proto::EnumReservedRange>(4, msg)?;
        }
        for string in &self.reserved_name {
            serializer.serialize_bytes_twice(5, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for EnumDescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod enum_descriptor_proto {
    #[derive(Debug, Clone)]
    pub struct EnumReservedRange {
        pub start: i32,
        pub end: i32,
    }
    impl ::std::default::Default for EnumReservedRange {
        fn default() -> Self {
            #[allow(unused)]
            use ::std::convert::TryInto;
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut EnumReservedRange {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
        }
        fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro_serializer::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            match field {
                ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.start = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.start = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
            use ::puroro_serializer::deserializer::stream::Deserializer;
            let mut msg = <Self as ::std::default::Default>::default();
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro_serializer::serializer::Serializable for EnumReservedRange {
        fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
            &self, serializer: &mut T) -> ::puroro::Result<()>
        {
            unimplemented!("Serializer for something else");
            unimplemented!("Serializer for something else");
            Ok(())
        }
    }
    impl ::puroro::Serializable for EnumReservedRange {
        fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
            <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
}
#[derive(Debug, Clone)]
pub struct OneofDescriptorProto {
    pub name: String,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::OneofOptions>>,
}
impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            name: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut OneofDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for OneofDescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::OneofOptions>(2, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for OneofDescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct FieldDescriptorProto {
    pub name: String,
    pub number: i32,
    pub label: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Label, i32>,
    pub type_: ::std::result::Result<super::super::google::protobuf::field_descriptor_proto::Type, i32>,
    pub type_name: String,
    pub extendee: String,
    pub default_value: String,
    pub oneof_index: i32,
    pub json_name: String,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FieldOptions>>,
    pub proto3_optional: bool,
}
impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
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
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut FieldDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.number = variant.to_native::<::puroro::tags::Int32>()?;
                }
                4 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.label = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                5 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.type_ = variant.to_native::<::puroro::tags::Int32>()?.try_into();
                }
                6 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                2 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                7 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                9 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.oneof_index = variant.to_native::<::puroro::tags::Int32>()?;
                }
                10 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                8 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                17 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.proto3_optional = variant.to_native::<::puroro::tags::Bool>()?;
                }
                _ => todo!("Unknown field number"),
            }
            ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                1 => {
                    self.name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                3 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.number = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                4 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.label = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?.try_into();
                }
                5 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.type_ = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?.try_into();
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.oneof_index = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                        .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                }
                10 => {
                    self.json_name = ldd.deserialize_as_chars().collect::<::puroro::Result<_>>()?;
                }
                8 => {
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
                }
                17 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.proto3_optional = ldd.deserialize_as_variants()
                        .last()
                        .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for FieldDescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        unimplemented!("Serializer for something else");
        serializer.serialize_variant::<::puroro::tags::Int32>(
            4, 
            self.label.clone().map_or_else(|e| e, |v| v as i32)
        )?;
        serializer.serialize_variant::<::puroro::tags::Int32>(
            5, 
            self.type_.clone().map_or_else(|e| e, |v| v as i32)
        )?;
        serializer.serialize_bytes_twice(6, self.type_name.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(2, self.extendee.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(7, self.default_value.bytes().map(|b| Ok(b)))?;
        unimplemented!("Serializer for something else");
        serializer.serialize_bytes_twice(10, self.json_name.bytes().map(|b| Ok(b)))?;
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::FieldOptions>(8, msg)?;
        }
        unimplemented!("Serializer for something else");
        Ok(())
    }
}
impl ::puroro::Serializable for FieldDescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod field_descriptor_proto {
    #[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: ::std::vec::Vec<super::super::google::protobuf::UninterpretedOption>,
}
impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            uninterpreted_option: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut ExtensionRangeOptions {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.uninterpreted_option.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for ExtensionRangeOptions {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for msg in &self.uninterpreted_option {
            serializer.serialize_message_twice::<super::super::google::protobuf::UninterpretedOption>(999, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for ExtensionRangeOptions {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct DescriptorProto {
    pub name: String,
    pub field: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    pub extension_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec<super::super::google::protobuf::OneofDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::MessageOptions>>,
    pub reserved_range: ::std::vec::Vec<super::super::google::protobuf::descriptor_proto::ReservedRange>,
    pub reserved_name: ::std::vec::Vec<String>,
}
impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
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
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut DescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.field.push(msg);
                }
                6 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.extension.push(msg);
                }
                3 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.nested_type.push(msg);
                }
                4 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.enum_type.push(msg);
                }
                5 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.extension_range.push(msg);
                }
                8 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.oneof_decl.push(msg);
                }
                7 => {
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
                }
                9 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.reserved_range.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for DescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        for msg in &self.field {
            serializer.serialize_message_twice::<super::super::google::protobuf::FieldDescriptorProto>(2, msg)?;
        }
        for msg in &self.extension {
            serializer.serialize_message_twice::<super::super::google::protobuf::FieldDescriptorProto>(6, msg)?;
        }
        for msg in &self.nested_type {
            serializer.serialize_message_twice::<super::super::google::protobuf::DescriptorProto>(3, msg)?;
        }
        for msg in &self.enum_type {
            serializer.serialize_message_twice::<super::super::google::protobuf::EnumDescriptorProto>(4, msg)?;
        }
        for msg in &self.extension_range {
            serializer.serialize_message_twice::<super::super::google::protobuf::descriptor_proto::ExtensionRange>(5, msg)?;
        }
        for msg in &self.oneof_decl {
            serializer.serialize_message_twice::<super::super::google::protobuf::OneofDescriptorProto>(8, msg)?;
        }
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::MessageOptions>(7, msg)?;
        }
        for msg in &self.reserved_range {
            serializer.serialize_message_twice::<super::super::google::protobuf::descriptor_proto::ReservedRange>(9, msg)?;
        }
        for string in &self.reserved_name {
            serializer.serialize_bytes_twice(10, string.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for DescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
mod descriptor_proto {
    #[derive(Debug, Clone)]
    pub struct ReservedRange {
        pub start: i32,
        pub end: i32,
    }
    impl ::std::default::Default for ReservedRange {
        fn default() -> Self {
            #[allow(unused)]
            use ::std::convert::TryInto;
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut ReservedRange {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
        }
        fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro_serializer::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            match field {
                ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.start = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.start = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
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
            use ::puroro_serializer::deserializer::stream::Deserializer;
            let mut msg = <Self as ::std::default::Default>::default();
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro_serializer::serializer::Serializable for ReservedRange {
        fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
            &self, serializer: &mut T) -> ::puroro::Result<()>
        {
            unimplemented!("Serializer for something else");
            unimplemented!("Serializer for something else");
            Ok(())
        }
    }
    impl ::puroro::Serializable for ReservedRange {
        fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
            <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
    #[derive(Debug, Clone)]
    pub struct ExtensionRange {
        pub start: i32,
        pub end: i32,
        pub options: ::std::option::Option<::std::boxed::Box<super::super::super::google::protobuf::ExtensionRangeOptions>>,
    }
    impl ::std::default::Default for ExtensionRange {
        fn default() -> Self {
            #[allow(unused)]
            use ::std::convert::TryInto;
            Self {
                start: ::std::default::Default::default(),
                end: ::std::default::Default::default(),
                options: ::std::default::Default::default(),
            }
        }
    }
    impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut ExtensionRange {
        type Target = ();
        fn finish(self) -> ::puroro::Result<Self::Target> {
            Ok(())
        }
        fn met_field<T: ::puroro_serializer::deserializer::stream::LengthDelimitedDeserializer>(
            &mut self,
            field: ::puroro_serializer::deserializer::stream::Field<T>,
            field_number: usize,
        ) -> ::puroro::Result<()> {
            match field {
                ::puroro_serializer::deserializer::stream::Field::Variant(variant) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.start = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = variant.to_native::<::puroro::tags::Int32>()?;
                    }
                    3 => Err(::puroro::PuroroError::UnexpectedWireType)?,
                    _ => todo!("Unknown field number"),
                }
                ::puroro_serializer::deserializer::stream::Field::LengthDelimited(ldd) => match field_number {
                    1 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.start = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    2 => {
                        #[allow(unused)]
                        use ::std::convert::TryInto;
                        self.end = ldd.deserialize_as_variants()
                            .last()
                            .unwrap_or(Err(::puroro::PuroroError::ZeroLengthPackedField))
                            .and_then(|variant| variant.to_native::<::puroro::tags::Int32>())?;
                    }
                    3 => {
                        let boxed_msg = self.options.get_or_insert_with(
                            || ::std::boxed::Box::new(::std::default::Default::default()));
                        ldd.deserialize_as_message(boxed_msg.as_mut())?;
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
            use ::puroro_serializer::deserializer::stream::Deserializer;
            let mut msg = <Self as ::std::default::Default>::default();
            ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
            Ok(msg)
        }
    }
    impl ::puroro_serializer::serializer::Serializable for ExtensionRange {
        fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
            &self, serializer: &mut T) -> ::puroro::Result<()>
        {
            unimplemented!("Serializer for something else");
            unimplemented!("Serializer for something else");
            if let Some(msg) = &self.options {
                serializer.serialize_message_twice::<super::super::super::google::protobuf::ExtensionRangeOptions>(3, msg)?;
            }
            Ok(())
        }
    }
    impl ::puroro::Serializable for ExtensionRange {
        fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
            let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
            <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
        }
    }
}
#[derive(Debug, Clone)]
pub struct FileDescriptorProto {
    pub name: String,
    pub package: String,
    pub dependency: ::std::vec::Vec<String>,
    pub public_dependency: ::std::vec::Vec<i32>,
    pub weak_dependency: ::std::vec::Vec<i32>,
    pub message_type: ::std::vec::Vec<super::super::google::protobuf::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<super::super::google::protobuf::EnumDescriptorProto>,
    pub service: ::std::vec::Vec<super::super::google::protobuf::ServiceDescriptorProto>,
    pub extension: ::std::vec::Vec<super::super::google::protobuf::FieldDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::FileOptions>>,
    pub source_code_info: ::std::option::Option<::std::boxed::Box<super::super::google::protobuf::SourceCodeInfo>>,
    pub syntax: String,
}
impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
        #[allow(unused)]
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
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut FileDescriptorProto {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.public_dependency.push(variant.to_native::<::puroro::tags::Int32>()?);
                }
                11 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
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
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.public_dependency.append(&mut ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                }
                11 => {
                    #[allow(unused)]
                    use ::std::convert::TryInto;
                    self.weak_dependency.append(&mut ldd.deserialize_as_variants().map(|rv| {
                        rv.and_then(|variant| variant.to_native::<::puroro::tags::Int32>())
                    }).collect::<::puroro::Result<::std::vec::Vec<_>>>()?);
                }
                4 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.message_type.push(msg);
                }
                5 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.enum_type.push(msg);
                }
                6 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.service.push(msg);
                }
                7 => {
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.extension.push(msg);
                }
                8 => {
                    let boxed_msg = self.options.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
                }
                9 => {
                    let boxed_msg = self.source_code_info.get_or_insert_with(
                        || ::std::boxed::Box::new(::std::default::Default::default()));
                    ldd.deserialize_as_message(boxed_msg.as_mut())?;
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for FileDescriptorProto {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        serializer.serialize_bytes_twice(1, self.name.bytes().map(|b| Ok(b)))?;
        serializer.serialize_bytes_twice(2, self.package.bytes().map(|b| Ok(b)))?;
        for string in &self.dependency {
            serializer.serialize_bytes_twice(3, string.bytes().map(|b| Ok(b)))?;
        }
        unimplemented!("Serializer for something else");
        unimplemented!("Serializer for something else");
        for msg in &self.message_type {
            serializer.serialize_message_twice::<super::super::google::protobuf::DescriptorProto>(4, msg)?;
        }
        for msg in &self.enum_type {
            serializer.serialize_message_twice::<super::super::google::protobuf::EnumDescriptorProto>(5, msg)?;
        }
        for msg in &self.service {
            serializer.serialize_message_twice::<super::super::google::protobuf::ServiceDescriptorProto>(6, msg)?;
        }
        for msg in &self.extension {
            serializer.serialize_message_twice::<super::super::google::protobuf::FieldDescriptorProto>(7, msg)?;
        }
        if let Some(msg) = &self.options {
            serializer.serialize_message_twice::<super::super::google::protobuf::FileOptions>(8, msg)?;
        }
        if let Some(msg) = &self.source_code_info {
            serializer.serialize_message_twice::<super::super::google::protobuf::SourceCodeInfo>(9, msg)?;
        }
        serializer.serialize_bytes_twice(12, self.syntax.bytes().map(|b| Ok(b)))?;
        Ok(())
    }
}
impl ::puroro::Serializable for FileDescriptorProto {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
#[derive(Debug, Clone)]
pub struct FileDescriptorSet {
    pub file: ::std::vec::Vec<super::super::google::protobuf::FileDescriptorProto>,
}
impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        #[allow(unused)]
        use ::std::convert::TryInto;
        Self {
            file: ::std::default::Default::default(),
        }
    }
}
impl<'a> ::puroro_serializer::deserializer::stream::MessageDeserializeEventHandler for &'a mut FileDescriptorSet {
    type Target = ();
    fn finish(self) -> ::puroro::Result<Self::Target> {
        Ok(())
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
                    let mut msg = ::std::default::Default::default();
                    ldd.deserialize_as_message(&mut msg)?;
                    self.file.push(msg);
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
        use ::puroro_serializer::deserializer::stream::Deserializer;
        let mut msg = <Self as ::std::default::Default>::default();
        ::puroro_serializer::deserializer::stream::deserializer_from_bytes(iter).deserialize(&mut msg)?;
        Ok(msg)
    }
}
impl ::puroro_serializer::serializer::Serializable for FileDescriptorSet {
    fn serialize<T: ::puroro_serializer::serializer::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for msg in &self.file {
            serializer.serialize_message_twice::<super::super::google::protobuf::FileDescriptorProto>(1, msg)?;
        }
        Ok(())
    }
}
impl ::puroro::Serializable for FileDescriptorSet {
    fn serialize<W: ::std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_serializer::serializer::default_serializer(write);
        <Self as ::puroro_serializer::serializer::Serializable>::serialize(self, &mut serializer)
    }
}
