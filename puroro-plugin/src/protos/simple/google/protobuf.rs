#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct GeneratedCodeInfo {
    pub annotation: ::std::vec::Vec::<self::generated_code_info::Annotation>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl GeneratedCodeInfo {
    pub fn new() -> Self {
        Self {
            annotation: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            annotation: <::std::vec::Vec::<self::generated_code_info::Annotation> as Clone>::clone(&self.annotation),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for GeneratedCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec::<self::generated_code_info::Annotation> as FieldMergeFromIter<
                    tags::Message::<self::generated_code_info::Annotation>, 
                    tags::Repeated>>
                ::merge(&mut self.annotation, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for GeneratedCodeInfo {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for GeneratedCodeInfo {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for GeneratedCodeInfo {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<self::generated_code_info::Annotation> as FieldSer<
                tags::Message::<self::generated_code_info::Annotation>, 
                tags::Repeated>>
            ::ser(&self.annotation, serializer, 1)?;
        Ok(())
    }
}

impl ::puroro::Serializable for GeneratedCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::GeneratedCodeInfoTrait for GeneratedCodeInfo {
    type AnnotationElement<'this> where Self: 'this = self::generated_code_info::Annotation;
    type AnnotationRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::generated_code_info::Annotation>;
    fn annotation<'this>(&'this self) -> Self::AnnotationRepeated::<'this> {
        &self.annotation
    }
}

impl ::puroro::Message for GeneratedCodeInfo {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::GeneratedCodeInfoTag> for GeneratedCodeInfo {
}

impl<'a> ::puroro_internal::FieldNew<'a> for GeneratedCodeInfo {
    fn new() -> Self {
        Default::default()
    }
}

pub mod generated_code_info {
#[derive(Debug)]
pub struct Annotation {
    pub path: ::std::vec::Vec::<i32>,
    pub source_file: ::std::option::Option::<::std::string::String>,
    pub begin: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl Annotation {
    pub fn new() -> Self {
        Self {
            path: ::puroro_internal::FieldNew::new(),
            source_file: ::puroro_internal::FieldNew::new(),
            begin: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            path: <::std::vec::Vec::<i32> as Clone>::clone(&self.path),
            source_file: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.source_file),
            begin: <::std::option::Option::<i32> as Clone>::clone(&self.begin),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for Annotation {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for Annotation {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.source_file, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.begin, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for Annotation {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for Annotation {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for Annotation {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.source_file, serializer, 2)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.begin, serializer, 3)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 4)?;
        Ok(())
    }
}

impl ::puroro::Serializable for Annotation {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::super::traits::google::protobuf::generated_code_info::AnnotationTrait for Annotation {
    type PathRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<i32>;
    fn path<'this>(&'this self) -> Self::PathRepeated::<'this> {
        &self.path
    }
    fn source_file<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.source_file.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn begin<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.begin.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl ::puroro::Message for Annotation {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::generated_code_info::AnnotationTag> for Annotation {
}

impl<'a> ::puroro_internal::FieldNew<'a> for Annotation {
    fn new() -> Self {
        Default::default()
    }
}

} // mod generated_code_info
#[derive(Debug)]
pub struct SourceCodeInfo {
    pub location: ::std::vec::Vec::<self::source_code_info::Location>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl SourceCodeInfo {
    pub fn new() -> Self {
        Self {
            location: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            location: <::std::vec::Vec::<self::source_code_info::Location> as Clone>::clone(&self.location),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for SourceCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec::<self::source_code_info::Location> as FieldMergeFromIter<
                    tags::Message::<self::source_code_info::Location>, 
                    tags::Repeated>>
                ::merge(&mut self.location, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for SourceCodeInfo {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for SourceCodeInfo {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for SourceCodeInfo {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<self::source_code_info::Location> as FieldSer<
                tags::Message::<self::source_code_info::Location>, 
                tags::Repeated>>
            ::ser(&self.location, serializer, 1)?;
        Ok(())
    }
}

impl ::puroro::Serializable for SourceCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::SourceCodeInfoTrait for SourceCodeInfo {
    type LocationElement<'this> where Self: 'this = self::source_code_info::Location;
    type LocationRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::source_code_info::Location>;
    fn location<'this>(&'this self) -> Self::LocationRepeated::<'this> {
        &self.location
    }
}

impl ::puroro::Message for SourceCodeInfo {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::SourceCodeInfoTag> for SourceCodeInfo {
}

impl<'a> ::puroro_internal::FieldNew<'a> for SourceCodeInfo {
    fn new() -> Self {
        Default::default()
    }
}

pub mod source_code_info {
#[derive(Debug)]
pub struct Location {
    pub path: ::std::vec::Vec::<i32>,
    pub span: ::std::vec::Vec::<i32>,
    pub leading_comments: ::std::option::Option::<::std::string::String>,
    pub trailing_comments: ::std::option::Option::<::std::string::String>,
    pub leading_detached_comments: ::std::vec::Vec::<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl Location {
    pub fn new() -> Self {
        Self {
            path: ::puroro_internal::FieldNew::new(),
            span: ::puroro_internal::FieldNew::new(),
            leading_comments: ::puroro_internal::FieldNew::new(),
            trailing_comments: ::puroro_internal::FieldNew::new(),
            leading_detached_comments: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for Location {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            path: <::std::vec::Vec::<i32> as Clone>::clone(&self.path),
            span: <::std::vec::Vec::<i32> as Clone>::clone(&self.span),
            leading_comments: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.leading_comments),
            trailing_comments: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.trailing_comments),
            leading_detached_comments: <::std::vec::Vec::<::std::string::String> as Clone>::clone(&self.leading_detached_comments),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for Location {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.span, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.leading_comments, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.trailing_comments, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.leading_detached_comments, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for Location {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for Location {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for Location {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::std::vec::Vec::<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.span, serializer, 2)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.leading_comments, serializer, 3)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.trailing_comments, serializer, 4)?;
        <::std::vec::Vec::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.leading_detached_comments, serializer, 6)?;
        Ok(())
    }
}

impl ::puroro::Serializable for Location {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::super::traits::google::protobuf::source_code_info::LocationTrait for Location {
    type PathRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<i32>;
    fn path<'this>(&'this self) -> Self::PathRepeated::<'this> {
        &self.path
    }
    type SpanRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<i32>;
    fn span<'this>(&'this self) -> Self::SpanRepeated::<'this> {
        &self.span
    }
    fn leading_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.leading_comments.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn trailing_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.trailing_comments.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type LeadingDetachedCommentsRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<::std::string::String>;
    fn leading_detached_comments<'this>(&'this self) -> Self::LeadingDetachedCommentsRepeated::<'this> {
        &self.leading_detached_comments
    }
}

impl ::puroro::Message for Location {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::source_code_info::LocationTag> for Location {
}

impl<'a> ::puroro_internal::FieldNew<'a> for Location {
    fn new() -> Self {
        Default::default()
    }
}

} // mod source_code_info
#[derive(Debug)]
pub struct UninterpretedOption {
    pub name: ::std::vec::Vec::<self::uninterpreted_option::NamePart>,
    pub identifier_value: ::std::option::Option::<::std::string::String>,
    pub positive_int_value: ::std::option::Option::<u64>,
    pub negative_int_value: ::std::option::Option::<i64>,
    pub double_value: ::std::option::Option::<f64>,
    pub string_value: ::std::option::Option::<::std::vec::Vec::<u8>>,
    pub aggregate_value: ::std::option::Option::<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl UninterpretedOption {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            identifier_value: ::puroro_internal::FieldNew::new(),
            positive_int_value: ::puroro_internal::FieldNew::new(),
            negative_int_value: ::puroro_internal::FieldNew::new(),
            double_value: ::puroro_internal::FieldNew::new(),
            string_value: ::puroro_internal::FieldNew::new(),
            aggregate_value: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::vec::Vec::<self::uninterpreted_option::NamePart> as Clone>::clone(&self.name),
            identifier_value: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.identifier_value),
            positive_int_value: <::std::option::Option::<u64> as Clone>::clone(&self.positive_int_value),
            negative_int_value: <::std::option::Option::<i64> as Clone>::clone(&self.negative_int_value),
            double_value: <::std::option::Option::<f64> as Clone>::clone(&self.double_value),
            string_value: <::std::option::Option::<::std::vec::Vec::<u8>> as Clone>::clone(&self.string_value),
            aggregate_value: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.aggregate_value),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for UninterpretedOption {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            2 => {
                <::std::vec::Vec::<self::uninterpreted_option::NamePart> as FieldMergeFromIter<
                    tags::Message::<self::uninterpreted_option::NamePart>, 
                    tags::Repeated>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.identifier_value, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option::<u64> as FieldMergeFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::merge(&mut self.positive_int_value, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::option::Option::<i64> as FieldMergeFromIter<
                    tags::Int64, 
                    tags::Optional2>>
                ::merge(&mut self.negative_int_value, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option::<f64> as FieldMergeFromIter<
                    tags::Double, 
                    tags::Optional2>>
                ::merge(&mut self.double_value, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option::<::std::vec::Vec::<u8>> as FieldMergeFromIter<
                    tags::Bytes, 
                    tags::Optional2>>
                ::merge(&mut self.string_value, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.aggregate_value, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for UninterpretedOption {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for UninterpretedOption {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for UninterpretedOption {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<self::uninterpreted_option::NamePart> as FieldSer<
                tags::Message::<self::uninterpreted_option::NamePart>, 
                tags::Repeated>>
            ::ser(&self.name, serializer, 2)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.identifier_value, serializer, 3)?;
        <::std::option::Option::<u64> as FieldSer<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.positive_int_value, serializer, 4)?;
        <::std::option::Option::<i64> as FieldSer<
                tags::Int64, 
                tags::Optional2>>
            ::ser(&self.negative_int_value, serializer, 5)?;
        <::std::option::Option::<f64> as FieldSer<
                tags::Double, 
                tags::Optional2>>
            ::ser(&self.double_value, serializer, 6)?;
        <::std::option::Option::<::std::vec::Vec::<u8>> as FieldSer<
                tags::Bytes, 
                tags::Optional2>>
            ::ser(&self.string_value, serializer, 7)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.aggregate_value, serializer, 8)?;
        Ok(())
    }
}

impl ::puroro::Serializable for UninterpretedOption {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::UninterpretedOptionTrait for UninterpretedOption {
    type NameElement<'this> where Self: 'this = self::uninterpreted_option::NamePart;
    type NameRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::uninterpreted_option::NamePart>;
    fn name<'this>(&'this self) -> Self::NameRepeated::<'this> {
        &self.name
    }
    fn identifier_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.identifier_value.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn positive_int_value<'this>(&'this self) -> ::std::option::Option::<u64> {
        self.positive_int_value.clone()
    }
    fn negative_int_value<'this>(&'this self) -> ::std::option::Option::<i64> {
        self.negative_int_value.clone()
    }
    fn double_value<'this>(&'this self) -> ::std::option::Option::<f64> {
        self.double_value.clone()
    }
    fn string_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, [u8]>> {
        self.string_value.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn aggregate_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.aggregate_value.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for UninterpretedOption {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::UninterpretedOptionTag> for UninterpretedOption {
}

impl<'a> ::puroro_internal::FieldNew<'a> for UninterpretedOption {
    fn new() -> Self {
        Default::default()
    }
}

pub mod uninterpreted_option {
#[derive(Debug)]
pub struct NamePart {
    pub name_part: ::std::string::String,
    pub is_extension: bool,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl NamePart {
    pub fn new() -> Self {
        Self {
            name_part: ::puroro_internal::FieldNew::new(),
            is_extension: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for NamePart {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name_part: <::std::string::String as Clone>::clone(&self.name_part),
            is_extension: <bool as Clone>::clone(&self.is_extension),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for NamePart {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for NamePart {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::string::String as FieldMergeFromIter<
                    tags::String, 
                    tags::Required>>
                ::merge(&mut self.name_part, field, ::std::default::Default::default)?;
            }
            2 => {
                <bool as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Required>>
                ::merge(&mut self.is_extension, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for NamePart {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for NamePart {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for NamePart {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::string::String as FieldSer<
                tags::String, 
                tags::Required>>
            ::ser(&self.name_part, serializer, 1)?;
        <bool as FieldSer<
                tags::Bool, 
                tags::Required>>
            ::ser(&self.is_extension, serializer, 2)?;
        Ok(())
    }
}

impl ::puroro::Serializable for NamePart {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::super::traits::google::protobuf::uninterpreted_option::NamePartTrait for NamePart {
    fn name_part<'this>(&'this self) -> ::std::borrow::Cow::<'this, str> {
        ::std::borrow::Cow::Borrowed(self.name_part.as_ref())
    }
    fn is_extension<'this>(&'this self) -> bool {
        self.is_extension.clone()
    }
}

impl ::puroro::Message for NamePart {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::uninterpreted_option::NamePartTag> for NamePart {
}

impl<'a> ::puroro_internal::FieldNew<'a> for NamePart {
    fn new() -> Self {
        Default::default()
    }
}

} // mod uninterpreted_option
#[derive(Debug)]
pub struct MethodOptions {
    pub deprecated: ::std::option::Option::<bool>,
    pub idempotency_level: ::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel>,
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl MethodOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            idempotency_level: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            idempotency_level: <::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> as Clone>::clone(&self.idempotency_level),
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for MethodOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            33 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            34 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> as FieldMergeFromIter<
                    tags::Enum2::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel>, 
                    tags::Optional2>>
                ::merge(&mut self.idempotency_level, field, || 0i32.try_into().unwrap())?;
            }
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for MethodOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for MethodOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for MethodOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> as FieldSer<
                tags::Enum2::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel>, 
                tags::Optional2>>
            ::ser(&self.idempotency_level, serializer, 34)?;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for MethodOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::MethodOptionsTrait for MethodOptions {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    fn idempotency_level<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> {
        self.idempotency_level.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for MethodOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MethodOptionsTag> for MethodOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for MethodOptions {
    fn new() -> Self {
        Default::default()
    }
}

pub mod method_options {
} // mod method_options
#[derive(Debug)]
pub struct ServiceOptions {
    pub deprecated: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ServiceOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for ServiceOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            33 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ServiceOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ServiceOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for ServiceOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ServiceOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::ServiceOptionsTrait for ServiceOptions {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for ServiceOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ServiceOptionsTag> for ServiceOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for ServiceOptions {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct EnumValueOptions {
    pub deprecated: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumValueOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for EnumValueOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for EnumValueOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumValueOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumValueOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumValueOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 1)?;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::EnumValueOptionsTrait for EnumValueOptions {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for EnumValueOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumValueOptionsTag> for EnumValueOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for EnumValueOptions {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct EnumOptions {
    pub allow_alias: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumOptions {
    pub fn new() -> Self {
        Self {
            allow_alias: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option::<bool> as Clone>::clone(&self.allow_alias),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for EnumOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for EnumOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            2 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.allow_alias, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.allow_alias, serializer, 2)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::EnumOptionsTrait for EnumOptions {
    fn allow_alias<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.allow_alias.clone()
    }
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for EnumOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumOptionsTag> for EnumOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for EnumOptions {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct OneofOptions {
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl OneofOptions {
    pub fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for OneofOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for OneofOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for OneofOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for OneofOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for OneofOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::OneofOptionsTrait for OneofOptions {
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for OneofOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::OneofOptionsTag> for OneofOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for OneofOptions {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct FieldOptions {
    pub ctype: ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype>,
    pub packed: ::std::option::Option::<bool>,
    pub jstype: ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype>,
    pub lazy: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub weak: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl FieldOptions {
    pub fn new() -> Self {
        Self {
            ctype: ::puroro_internal::FieldNew::new(),
            packed: ::puroro_internal::FieldNew::new(),
            jstype: ::puroro_internal::FieldNew::new(),
            lazy: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            weak: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for FieldOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            ctype: <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype> as Clone>::clone(&self.ctype),
            packed: <::std::option::Option::<bool> as Clone>::clone(&self.packed),
            jstype: <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype> as Clone>::clone(&self.jstype),
            lazy: <::std::option::Option::<bool> as Clone>::clone(&self.lazy),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            weak: <::std::option::Option::<bool> as Clone>::clone(&self.weak),
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for FieldOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype> as FieldMergeFromIter<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_options::Ctype>, 
                    tags::Optional2>>
                ::merge(&mut self.ctype, field, || 0i32.try_into().unwrap())?;
            }
            2 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.packed, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype> as FieldMergeFromIter<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_options::Jstype>, 
                    tags::Optional2>>
                ::merge(&mut self.jstype, field, || 0i32.try_into().unwrap())?;
            }
            5 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.lazy, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.weak, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FieldOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FieldOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for FieldOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype> as FieldSer<
                tags::Enum2::<super::super::super::enums::google::protobuf::field_options::Ctype>, 
                tags::Optional2>>
            ::ser(&self.ctype, serializer, 1)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.packed, serializer, 2)?;
        <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype> as FieldSer<
                tags::Enum2::<super::super::super::enums::google::protobuf::field_options::Jstype>, 
                tags::Optional2>>
            ::ser(&self.jstype, serializer, 6)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.lazy, serializer, 5)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.weak, serializer, 10)?;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FieldOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::FieldOptionsTrait for FieldOptions {
    fn ctype<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype> {
        self.ctype.clone()
    }
    fn packed<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.packed.clone()
    }
    fn jstype<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype> {
        self.jstype.clone()
    }
    fn lazy<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.lazy.clone()
    }
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    fn weak<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.weak.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for FieldOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FieldOptionsTag> for FieldOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for FieldOptions {
    fn new() -> Self {
        Default::default()
    }
}

pub mod field_options {
} // mod field_options
#[derive(Debug)]
pub struct MessageOptions {
    pub message_set_wire_format: ::std::option::Option::<bool>,
    pub no_standard_descriptor_accessor: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub map_entry: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl MessageOptions {
    pub fn new() -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::FieldNew::new(),
            no_standard_descriptor_accessor: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            map_entry: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            message_set_wire_format: <::std::option::Option::<bool> as Clone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <::std::option::Option::<bool> as Clone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            map_entry: <::std::option::Option::<bool> as Clone>::clone(&self.map_entry),
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for MessageOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for MessageOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.message_set_wire_format, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.no_standard_descriptor_accessor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.map_entry, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for MessageOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for MessageOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for MessageOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.message_set_wire_format, serializer, 1)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.no_standard_descriptor_accessor, serializer, 2)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.map_entry, serializer, 7)?;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for MessageOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::MessageOptionsTrait for MessageOptions {
    fn message_set_wire_format<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.message_set_wire_format.clone()
    }
    fn no_standard_descriptor_accessor<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.no_standard_descriptor_accessor.clone()
    }
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    fn map_entry<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.map_entry.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for MessageOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MessageOptionsTag> for MessageOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for MessageOptions {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct FileOptions {
    pub java_package: ::std::option::Option::<::std::string::String>,
    pub java_outer_classname: ::std::option::Option::<::std::string::String>,
    pub java_multiple_files: ::std::option::Option::<bool>,
    pub java_generate_equals_and_hash: ::std::option::Option::<bool>,
    pub java_string_check_utf8: ::std::option::Option::<bool>,
    pub optimize_for: ::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode>,
    pub go_package: ::std::option::Option::<::std::string::String>,
    pub cc_generic_services: ::std::option::Option::<bool>,
    pub java_generic_services: ::std::option::Option::<bool>,
    pub py_generic_services: ::std::option::Option::<bool>,
    pub php_generic_services: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub cc_enable_arenas: ::std::option::Option::<bool>,
    pub objc_class_prefix: ::std::option::Option::<::std::string::String>,
    pub csharp_namespace: ::std::option::Option::<::std::string::String>,
    pub swift_prefix: ::std::option::Option::<::std::string::String>,
    pub php_class_prefix: ::std::option::Option::<::std::string::String>,
    pub php_namespace: ::std::option::Option::<::std::string::String>,
    pub php_metadata_namespace: ::std::option::Option::<::std::string::String>,
    pub ruby_package: ::std::option::Option::<::std::string::String>,
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl FileOptions {
    pub fn new() -> Self {
        Self {
            java_package: ::puroro_internal::FieldNew::new(),
            java_outer_classname: ::puroro_internal::FieldNew::new(),
            java_multiple_files: ::puroro_internal::FieldNew::new(),
            java_generate_equals_and_hash: ::puroro_internal::FieldNew::new(),
            java_string_check_utf8: ::puroro_internal::FieldNew::new(),
            optimize_for: ::puroro_internal::FieldNew::new(),
            go_package: ::puroro_internal::FieldNew::new(),
            cc_generic_services: ::puroro_internal::FieldNew::new(),
            java_generic_services: ::puroro_internal::FieldNew::new(),
            py_generic_services: ::puroro_internal::FieldNew::new(),
            php_generic_services: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            cc_enable_arenas: ::puroro_internal::FieldNew::new(),
            objc_class_prefix: ::puroro_internal::FieldNew::new(),
            csharp_namespace: ::puroro_internal::FieldNew::new(),
            swift_prefix: ::puroro_internal::FieldNew::new(),
            php_class_prefix: ::puroro_internal::FieldNew::new(),
            php_namespace: ::puroro_internal::FieldNew::new(),
            php_metadata_namespace: ::puroro_internal::FieldNew::new(),
            ruby_package: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for FileOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            java_package: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.java_package),
            java_outer_classname: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.java_outer_classname),
            java_multiple_files: <::std::option::Option::<bool> as Clone>::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: <::std::option::Option::<bool> as Clone>::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: <::std::option::Option::<bool> as Clone>::clone(&self.java_string_check_utf8),
            optimize_for: <::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode> as Clone>::clone(&self.optimize_for),
            go_package: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.go_package),
            cc_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.cc_generic_services),
            java_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.java_generic_services),
            py_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.py_generic_services),
            php_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.php_generic_services),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            cc_enable_arenas: <::std::option::Option::<bool> as Clone>::clone(&self.cc_enable_arenas),
            objc_class_prefix: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.objc_class_prefix),
            csharp_namespace: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.csharp_namespace),
            swift_prefix: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.swift_prefix),
            php_class_prefix: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.php_class_prefix),
            php_namespace: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.php_namespace),
            php_metadata_namespace: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.php_metadata_namespace),
            ruby_package: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.ruby_package),
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for FileOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for FileOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.java_package, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.java_outer_classname, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_multiple_files, field, ::std::default::Default::default)?;
            }
            20 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_generate_equals_and_hash, field, ::std::default::Default::default)?;
            }
            27 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_string_check_utf8, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode> as FieldMergeFromIter<
                    tags::Enum2::<super::super::super::enums::google::protobuf::file_options::OptimizeMode>, 
                    tags::Optional2>>
                ::merge(&mut self.optimize_for, field, || 1i32.try_into().unwrap())?;
            }
            11 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.go_package, field, ::std::default::Default::default)?;
            }
            16 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.cc_generic_services, field, ::std::default::Default::default)?;
            }
            17 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_generic_services, field, ::std::default::Default::default)?;
            }
            18 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.py_generic_services, field, ::std::default::Default::default)?;
            }
            42 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.php_generic_services, field, ::std::default::Default::default)?;
            }
            23 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            31 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.cc_enable_arenas, field, ::std::default::Default::default)?;
            }
            36 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.objc_class_prefix, field, ::std::default::Default::default)?;
            }
            37 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.csharp_namespace, field, ::std::default::Default::default)?;
            }
            39 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.swift_prefix, field, ::std::default::Default::default)?;
            }
            40 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_class_prefix, field, ::std::default::Default::default)?;
            }
            41 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_namespace, field, ::std::default::Default::default)?;
            }
            44 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_metadata_namespace, field, ::std::default::Default::default)?;
            }
            45 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.ruby_package, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FileOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FileOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for FileOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.java_package, serializer, 1)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.java_outer_classname, serializer, 8)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_multiple_files, serializer, 10)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_generate_equals_and_hash, serializer, 20)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_string_check_utf8, serializer, 27)?;
        <::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode> as FieldSer<
                tags::Enum2::<super::super::super::enums::google::protobuf::file_options::OptimizeMode>, 
                tags::Optional2>>
            ::ser(&self.optimize_for, serializer, 9)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.go_package, serializer, 11)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.cc_generic_services, serializer, 16)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_generic_services, serializer, 17)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.py_generic_services, serializer, 18)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.php_generic_services, serializer, 42)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 23)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.cc_enable_arenas, serializer, 31)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.objc_class_prefix, serializer, 36)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.csharp_namespace, serializer, 37)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.swift_prefix, serializer, 39)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_class_prefix, serializer, 40)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_namespace, serializer, 41)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_metadata_namespace, serializer, 44)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.ruby_package, serializer, 45)?;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FileOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::FileOptionsTrait for FileOptions {
    fn java_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.java_package.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn java_outer_classname<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.java_outer_classname.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn java_multiple_files<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.java_multiple_files.clone()
    }
    fn java_generate_equals_and_hash<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.java_generate_equals_and_hash.clone()
    }
    fn java_string_check_utf8<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.java_string_check_utf8.clone()
    }
    fn optimize_for<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode> {
        self.optimize_for.clone()
    }
    fn go_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.go_package.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn cc_generic_services<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.cc_generic_services.clone()
    }
    fn java_generic_services<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.java_generic_services.clone()
    }
    fn py_generic_services<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.py_generic_services.clone()
    }
    fn php_generic_services<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.php_generic_services.clone()
    }
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    fn cc_enable_arenas<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.cc_enable_arenas.clone()
    }
    fn objc_class_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.objc_class_prefix.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn csharp_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.csharp_namespace.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn swift_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.swift_prefix.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn php_class_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.php_class_prefix.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn php_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.php_namespace.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn php_metadata_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.php_metadata_namespace.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn ruby_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.ruby_package.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for FileOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileOptionsTag> for FileOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for FileOptions {
    fn new() -> Self {
        Default::default()
    }
}

pub mod file_options {
} // mod file_options
#[derive(Debug)]
pub struct MethodDescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub input_type: ::std::option::Option::<::std::string::String>,
    pub output_type: ::std::option::Option::<::std::string::String>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::MethodOptions>>,
    pub client_streaming: ::std::option::Option::<bool>,
    pub server_streaming: ::std::option::Option::<bool>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl MethodDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            input_type: ::puroro_internal::FieldNew::new(),
            output_type: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            client_streaming: ::puroro_internal::FieldNew::new(),
            server_streaming: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            input_type: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.input_type),
            output_type: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.output_type),
            options: <::std::option::Option::<::std::boxed::Box::<self::MethodOptions>> as Clone>::clone(&self.options),
            client_streaming: <::std::option::Option::<bool> as Clone>::clone(&self.client_streaming),
            server_streaming: <::std::option::Option::<bool> as Clone>::clone(&self.server_streaming),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for MethodDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.input_type, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.output_type, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option::<::std::boxed::Box::<self::MethodOptions>> as FieldMergeFromIter<
                    tags::Message::<self::MethodOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.client_streaming, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.server_streaming, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for MethodDescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for MethodDescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for MethodDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.input_type, serializer, 2)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.output_type, serializer, 3)?;
        <::std::option::Option::<::std::boxed::Box::<self::MethodOptions>> as FieldSer<
                tags::Message::<self::MethodOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 4)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.client_streaming, serializer, 5)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.server_streaming, serializer, 6)?;
        Ok(())
    }
}

impl ::puroro::Serializable for MethodDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::MethodDescriptorProtoTrait for MethodDescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn input_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.input_type.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn output_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.output_type.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type OptionsType<'this> where Self: 'this = self::MethodOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn client_streaming<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.client_streaming.clone()
    }
    fn server_streaming<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.server_streaming.clone()
    }
}

impl ::puroro::Message for MethodDescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MethodDescriptorProtoTag> for MethodDescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for MethodDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct ServiceDescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub method: ::std::vec::Vec::<self::MethodDescriptorProto>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::ServiceOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ServiceDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            method: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            method: <::std::vec::Vec::<self::MethodDescriptorProto> as Clone>::clone(&self.method),
            options: <::std::option::Option::<::std::boxed::Box::<self::ServiceOptions>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for ServiceDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec::<self::MethodDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::MethodDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.method, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::boxed::Box::<self::ServiceOptions>> as FieldMergeFromIter<
                    tags::Message::<self::ServiceOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ServiceDescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ServiceDescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for ServiceDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec::<self::MethodDescriptorProto> as FieldSer<
                tags::Message::<self::MethodDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.method, serializer, 2)?;
        <::std::option::Option::<::std::boxed::Box::<self::ServiceOptions>> as FieldSer<
                tags::Message::<self::ServiceOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ServiceDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::ServiceDescriptorProtoTrait for ServiceDescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type MethodElement<'this> where Self: 'this = self::MethodDescriptorProto;
    type MethodRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::MethodDescriptorProto>;
    fn method<'this>(&'this self) -> Self::MethodRepeated::<'this> {
        &self.method
    }
    type OptionsType<'this> where Self: 'this = self::ServiceOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for ServiceDescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ServiceDescriptorProtoTag> for ServiceDescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for ServiceDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct EnumValueDescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub number: ::std::option::Option::<i32>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::EnumValueOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumValueDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            number: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            number: <::std::option::Option::<i32> as Clone>::clone(&self.number),
            options: <::std::option::Option::<::std::boxed::Box::<self::EnumValueOptions>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for EnumValueDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.number, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::boxed::Box::<self::EnumValueOptions>> as FieldMergeFromIter<
                    tags::Message::<self::EnumValueOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumValueDescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumValueDescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumValueDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.number, serializer, 2)?;
        <::std::option::Option::<::std::boxed::Box::<self::EnumValueOptions>> as FieldSer<
                tags::Message::<self::EnumValueOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::EnumValueDescriptorProtoTrait for EnumValueDescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn number<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.number.clone()
    }
    type OptionsType<'this> where Self: 'this = self::EnumValueOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for EnumValueDescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumValueDescriptorProtoTag> for EnumValueDescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for EnumValueDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct EnumDescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub value: ::std::vec::Vec::<self::EnumValueDescriptorProto>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::EnumOptions>>,
    pub reserved_range: ::std::vec::Vec::<self::enum_descriptor_proto::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec::<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            value: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            reserved_range: ::puroro_internal::FieldNew::new(),
            reserved_name: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            value: <::std::vec::Vec::<self::EnumValueDescriptorProto> as Clone>::clone(&self.value),
            options: <::std::option::Option::<::std::boxed::Box::<self::EnumOptions>> as Clone>::clone(&self.options),
            reserved_range: <::std::vec::Vec::<self::enum_descriptor_proto::EnumReservedRange> as Clone>::clone(&self.reserved_range),
            reserved_name: <::std::vec::Vec::<::std::string::String> as Clone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for EnumDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec::<self::EnumValueDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::EnumValueDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.value, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::boxed::Box::<self::EnumOptions>> as FieldMergeFromIter<
                    tags::Message::<self::EnumOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec::<self::enum_descriptor_proto::EnumReservedRange> as FieldMergeFromIter<
                    tags::Message::<self::enum_descriptor_proto::EnumReservedRange>, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_range, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_name, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumDescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumDescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec::<self::EnumValueDescriptorProto> as FieldSer<
                tags::Message::<self::EnumValueDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.value, serializer, 2)?;
        <::std::option::Option::<::std::boxed::Box::<self::EnumOptions>> as FieldSer<
                tags::Message::<self::EnumOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        <::std::vec::Vec::<self::enum_descriptor_proto::EnumReservedRange> as FieldSer<
                tags::Message::<self::enum_descriptor_proto::EnumReservedRange>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 4)?;
        <::std::vec::Vec::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.reserved_name, serializer, 5)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::EnumDescriptorProtoTrait for EnumDescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ValueElement<'this> where Self: 'this = self::EnumValueDescriptorProto;
    type ValueRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::EnumValueDescriptorProto>;
    fn value<'this>(&'this self) -> Self::ValueRepeated::<'this> {
        &self.value
    }
    type OptionsType<'this> where Self: 'this = self::EnumOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ReservedRangeElement<'this> where Self: 'this = self::enum_descriptor_proto::EnumReservedRange;
    type ReservedRangeRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::enum_descriptor_proto::EnumReservedRange>;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this> {
        &self.reserved_range
    }
    type ReservedNameRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<::std::string::String>;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this> {
        &self.reserved_name
    }
}

impl ::puroro::Message for EnumDescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumDescriptorProtoTag> for EnumDescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for EnumDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

pub mod enum_descriptor_proto {
#[derive(Debug)]
pub struct EnumReservedRange {
    pub start: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumReservedRange {
    pub fn new() -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for EnumReservedRange {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for EnumReservedRange {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for EnumReservedRange {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumReservedRange {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumReservedRange {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumReservedRange {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 2)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumReservedRange {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::super::traits::google::protobuf::enum_descriptor_proto::EnumReservedRangeTrait for EnumReservedRange {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl ::puroro::Message for EnumReservedRange {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::enum_descriptor_proto::EnumReservedRangeTag> for EnumReservedRange {
}

impl<'a> ::puroro_internal::FieldNew<'a> for EnumReservedRange {
    fn new() -> Self {
        Default::default()
    }
}

} // mod enum_descriptor_proto
#[derive(Debug)]
pub struct OneofDescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::OneofOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl OneofDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            options: <::std::option::Option::<::std::boxed::Box::<self::OneofOptions>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for OneofDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::std::boxed::Box::<self::OneofOptions>> as FieldMergeFromIter<
                    tags::Message::<self::OneofOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for OneofDescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for OneofDescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for OneofDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::std::boxed::Box::<self::OneofOptions>> as FieldSer<
                tags::Message::<self::OneofOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 2)?;
        Ok(())
    }
}

impl ::puroro::Serializable for OneofDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::OneofDescriptorProtoTrait for OneofDescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type OptionsType<'this> where Self: 'this = self::OneofOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for OneofDescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::OneofDescriptorProtoTag> for OneofDescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for OneofDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct FieldDescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub number: ::std::option::Option::<i32>,
    pub label: ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label>,
    pub type_: ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type>,
    pub type_name: ::std::option::Option::<::std::string::String>,
    pub extendee: ::std::option::Option::<::std::string::String>,
    pub default_value: ::std::option::Option::<::std::string::String>,
    pub oneof_index: ::std::option::Option::<i32>,
    pub json_name: ::std::option::Option::<::std::string::String>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::FieldOptions>>,
    pub proto3_optional: ::std::option::Option::<bool>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl FieldDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            number: ::puroro_internal::FieldNew::new(),
            label: ::puroro_internal::FieldNew::new(),
            type_: ::puroro_internal::FieldNew::new(),
            type_name: ::puroro_internal::FieldNew::new(),
            extendee: ::puroro_internal::FieldNew::new(),
            default_value: ::puroro_internal::FieldNew::new(),
            oneof_index: ::puroro_internal::FieldNew::new(),
            json_name: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            proto3_optional: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for FieldDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            number: <::std::option::Option::<i32> as Clone>::clone(&self.number),
            label: <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label> as Clone>::clone(&self.label),
            type_: <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type> as Clone>::clone(&self.type_),
            type_name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.type_name),
            extendee: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.extendee),
            default_value: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.default_value),
            oneof_index: <::std::option::Option::<i32> as Clone>::clone(&self.oneof_index),
            json_name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.json_name),
            options: <::std::option::Option::<::std::boxed::Box::<self::FieldOptions>> as Clone>::clone(&self.options),
            proto3_optional: <::std::option::Option::<bool> as Clone>::clone(&self.proto3_optional),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for FieldDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.number, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label> as FieldMergeFromIter<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label>, 
                    tags::Optional2>>
                ::merge(&mut self.label, field, || 1i32.try_into().unwrap())?;
            }
            5 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type> as FieldMergeFromIter<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type>, 
                    tags::Optional2>>
                ::merge(&mut self.type_, field, || 1i32.try_into().unwrap())?;
            }
            6 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.type_name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.extendee, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.default_value, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.oneof_index, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.json_name, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option::<::std::boxed::Box::<self::FieldOptions>> as FieldMergeFromIter<
                    tags::Message::<self::FieldOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            17 => {
                <::std::option::Option::<bool> as FieldMergeFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.proto3_optional, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FieldDescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FieldDescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for FieldDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.number, serializer, 3)?;
        <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label> as FieldSer<
                tags::Enum2::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label>, 
                tags::Optional2>>
            ::ser(&self.label, serializer, 4)?;
        <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type> as FieldSer<
                tags::Enum2::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type>, 
                tags::Optional2>>
            ::ser(&self.type_, serializer, 5)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.type_name, serializer, 6)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.extendee, serializer, 2)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.default_value, serializer, 7)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.oneof_index, serializer, 9)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.json_name, serializer, 10)?;
        <::std::option::Option::<::std::boxed::Box::<self::FieldOptions>> as FieldSer<
                tags::Message::<self::FieldOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.proto3_optional, serializer, 17)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FieldDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::FieldDescriptorProtoTrait for FieldDescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn number<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.number.clone()
    }
    fn label<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label> {
        self.label.clone()
    }
    fn type_<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type> {
        self.type_.clone()
    }
    fn type_name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.type_name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn extendee<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.extendee.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn default_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.default_value.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn oneof_index<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.oneof_index.clone()
    }
    fn json_name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.json_name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type OptionsType<'this> where Self: 'this = self::FieldOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn proto3_optional<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.proto3_optional.clone()
    }
}

impl ::puroro::Message for FieldDescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FieldDescriptorProtoTag> for FieldDescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for FieldDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

pub mod field_descriptor_proto {
} // mod field_descriptor_proto
#[derive(Debug)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: ::std::vec::Vec::<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ExtensionRangeOptions {
    pub fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::vec::Vec::<self::UninterpretedOption> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for ExtensionRangeOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            999 => {
                <::std::vec::Vec::<self::UninterpretedOption> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ExtensionRangeOptions {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ExtensionRangeOptions {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for ExtensionRangeOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<self::UninterpretedOption> as FieldSer<
                tags::Message::<self::UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ExtensionRangeOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::ExtensionRangeOptionsTrait for ExtensionRangeOptions {
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::UninterpretedOption>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl ::puroro::Message for ExtensionRangeOptions {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ExtensionRangeOptionsTag> for ExtensionRangeOptions {
}

impl<'a> ::puroro_internal::FieldNew<'a> for ExtensionRangeOptions {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct DescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub field: ::std::vec::Vec::<self::FieldDescriptorProto>,
    pub extension: ::std::vec::Vec::<self::FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec::<self::DescriptorProto>,
    pub enum_type: ::std::vec::Vec::<self::EnumDescriptorProto>,
    pub extension_range: ::std::vec::Vec::<self::descriptor_proto::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec::<self::OneofDescriptorProto>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::MessageOptions>>,
    pub reserved_range: ::std::vec::Vec::<self::descriptor_proto::ReservedRange>,
    pub reserved_name: ::std::vec::Vec::<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl DescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            field: ::puroro_internal::FieldNew::new(),
            extension: ::puroro_internal::FieldNew::new(),
            nested_type: ::puroro_internal::FieldNew::new(),
            enum_type: ::puroro_internal::FieldNew::new(),
            extension_range: ::puroro_internal::FieldNew::new(),
            oneof_decl: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            reserved_range: ::puroro_internal::FieldNew::new(),
            reserved_name: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            field: <::std::vec::Vec::<self::FieldDescriptorProto> as Clone>::clone(&self.field),
            extension: <::std::vec::Vec::<self::FieldDescriptorProto> as Clone>::clone(&self.extension),
            nested_type: <::std::vec::Vec::<self::DescriptorProto> as Clone>::clone(&self.nested_type),
            enum_type: <::std::vec::Vec::<self::EnumDescriptorProto> as Clone>::clone(&self.enum_type),
            extension_range: <::std::vec::Vec::<self::descriptor_proto::ExtensionRange> as Clone>::clone(&self.extension_range),
            oneof_decl: <::std::vec::Vec::<self::OneofDescriptorProto> as Clone>::clone(&self.oneof_decl),
            options: <::std::option::Option::<::std::boxed::Box::<self::MessageOptions>> as Clone>::clone(&self.options),
            reserved_range: <::std::vec::Vec::<self::descriptor_proto::ReservedRange> as Clone>::clone(&self.reserved_range),
            reserved_name: <::std::vec::Vec::<::std::string::String> as Clone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for DescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec::<self::FieldDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::FieldDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.field, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec::<self::FieldDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::FieldDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.extension, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::vec::Vec::<self::DescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::DescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.nested_type, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec::<self::EnumDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::EnumDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.enum_type, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec::<self::descriptor_proto::ExtensionRange> as FieldMergeFromIter<
                    tags::Message::<self::descriptor_proto::ExtensionRange>, 
                    tags::Repeated>>
                ::merge(&mut self.extension_range, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::vec::Vec::<self::OneofDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::OneofDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.oneof_decl, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option::<::std::boxed::Box::<self::MessageOptions>> as FieldMergeFromIter<
                    tags::Message::<self::MessageOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::vec::Vec::<self::descriptor_proto::ReservedRange> as FieldMergeFromIter<
                    tags::Message::<self::descriptor_proto::ReservedRange>, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_range, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::vec::Vec::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_name, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for DescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for DescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for DescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec::<self::FieldDescriptorProto> as FieldSer<
                tags::Message::<self::FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.field, serializer, 2)?;
        <::std::vec::Vec::<self::FieldDescriptorProto> as FieldSer<
                tags::Message::<self::FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 6)?;
        <::std::vec::Vec::<self::DescriptorProto> as FieldSer<
                tags::Message::<self::DescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.nested_type, serializer, 3)?;
        <::std::vec::Vec::<self::EnumDescriptorProto> as FieldSer<
                tags::Message::<self::EnumDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 4)?;
        <::std::vec::Vec::<self::descriptor_proto::ExtensionRange> as FieldSer<
                tags::Message::<self::descriptor_proto::ExtensionRange>, 
                tags::Repeated>>
            ::ser(&self.extension_range, serializer, 5)?;
        <::std::vec::Vec::<self::OneofDescriptorProto> as FieldSer<
                tags::Message::<self::OneofDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.oneof_decl, serializer, 8)?;
        <::std::option::Option::<::std::boxed::Box::<self::MessageOptions>> as FieldSer<
                tags::Message::<self::MessageOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 7)?;
        <::std::vec::Vec::<self::descriptor_proto::ReservedRange> as FieldSer<
                tags::Message::<self::descriptor_proto::ReservedRange>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 9)?;
        <::std::vec::Vec::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.reserved_name, serializer, 10)?;
        Ok(())
    }
}

impl ::puroro::Serializable for DescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::DescriptorProtoTrait for DescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type FieldElement<'this> where Self: 'this = self::FieldDescriptorProto;
    type FieldRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::FieldDescriptorProto>;
    fn field<'this>(&'this self) -> Self::FieldRepeated::<'this> {
        &self.field
    }
    type ExtensionElement<'this> where Self: 'this = self::FieldDescriptorProto;
    type ExtensionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::FieldDescriptorProto>;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this> {
        &self.extension
    }
    type NestedTypeElement<'this> where Self: 'this = self::DescriptorProto;
    type NestedTypeRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::DescriptorProto>;
    fn nested_type<'this>(&'this self) -> Self::NestedTypeRepeated::<'this> {
        &self.nested_type
    }
    type EnumTypeElement<'this> where Self: 'this = self::EnumDescriptorProto;
    type EnumTypeRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::EnumDescriptorProto>;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this> {
        &self.enum_type
    }
    type ExtensionRangeElement<'this> where Self: 'this = self::descriptor_proto::ExtensionRange;
    type ExtensionRangeRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::descriptor_proto::ExtensionRange>;
    fn extension_range<'this>(&'this self) -> Self::ExtensionRangeRepeated::<'this> {
        &self.extension_range
    }
    type OneofDeclElement<'this> where Self: 'this = self::OneofDescriptorProto;
    type OneofDeclRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::OneofDescriptorProto>;
    fn oneof_decl<'this>(&'this self) -> Self::OneofDeclRepeated::<'this> {
        &self.oneof_decl
    }
    type OptionsType<'this> where Self: 'this = self::MessageOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ReservedRangeElement<'this> where Self: 'this = self::descriptor_proto::ReservedRange;
    type ReservedRangeRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::descriptor_proto::ReservedRange>;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this> {
        &self.reserved_range
    }
    type ReservedNameRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<::std::string::String>;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this> {
        &self.reserved_name
    }
}

impl ::puroro::Message for DescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::DescriptorProtoTag> for DescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for DescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

pub mod descriptor_proto {
#[derive(Debug)]
pub struct ReservedRange {
    pub start: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ReservedRange {
    pub fn new() -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for ReservedRange {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for ReservedRange {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for ReservedRange {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ReservedRange {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ReservedRange {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for ReservedRange {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 2)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ReservedRange {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::super::traits::google::protobuf::descriptor_proto::ReservedRangeTrait for ReservedRange {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl ::puroro::Message for ReservedRange {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::descriptor_proto::ReservedRangeTag> for ReservedRange {
}

impl<'a> ::puroro_internal::FieldNew<'a> for ReservedRange {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct ExtensionRange {
    pub start: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    pub options: ::std::option::Option::<::std::boxed::Box::<super::ExtensionRangeOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ExtensionRange {
    pub fn new() -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for ExtensionRange {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            options: <::std::option::Option::<::std::boxed::Box::<super::ExtensionRangeOptions>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for ExtensionRange {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for ExtensionRange {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::std::boxed::Box::<super::ExtensionRangeOptions>> as FieldMergeFromIter<
                    tags::Message::<super::ExtensionRangeOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ExtensionRange {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ExtensionRange {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for ExtensionRange {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 2)?;
        <::std::option::Option::<::std::boxed::Box::<super::ExtensionRangeOptions>> as FieldSer<
                tags::Message::<super::ExtensionRangeOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ExtensionRange {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::super::traits::google::protobuf::descriptor_proto::ExtensionRangeTrait for ExtensionRange {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
    type OptionsType<'this> where Self: 'this = super::ExtensionRangeOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for ExtensionRange {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::descriptor_proto::ExtensionRangeTag> for ExtensionRange {
}

impl<'a> ::puroro_internal::FieldNew<'a> for ExtensionRange {
    fn new() -> Self {
        Default::default()
    }
}

} // mod descriptor_proto
#[derive(Debug)]
pub struct FileDescriptorProto {
    pub name: ::std::option::Option::<::std::string::String>,
    pub package: ::std::option::Option::<::std::string::String>,
    pub dependency: ::std::vec::Vec::<::std::string::String>,
    pub public_dependency: ::std::vec::Vec::<i32>,
    pub weak_dependency: ::std::vec::Vec::<i32>,
    pub message_type: ::std::vec::Vec::<self::DescriptorProto>,
    pub enum_type: ::std::vec::Vec::<self::EnumDescriptorProto>,
    pub service: ::std::vec::Vec::<self::ServiceDescriptorProto>,
    pub extension: ::std::vec::Vec::<self::FieldDescriptorProto>,
    pub options: ::std::option::Option::<::std::boxed::Box::<self::FileOptions>>,
    pub source_code_info: ::std::option::Option::<::std::boxed::Box::<self::SourceCodeInfo>>,
    pub syntax: ::std::option::Option::<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl FileDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            package: ::puroro_internal::FieldNew::new(),
            dependency: ::puroro_internal::FieldNew::new(),
            public_dependency: ::puroro_internal::FieldNew::new(),
            weak_dependency: ::puroro_internal::FieldNew::new(),
            message_type: ::puroro_internal::FieldNew::new(),
            enum_type: ::puroro_internal::FieldNew::new(),
            service: ::puroro_internal::FieldNew::new(),
            extension: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            source_code_info: ::puroro_internal::FieldNew::new(),
            syntax: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for FileDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.name),
            package: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.package),
            dependency: <::std::vec::Vec::<::std::string::String> as Clone>::clone(&self.dependency),
            public_dependency: <::std::vec::Vec::<i32> as Clone>::clone(&self.public_dependency),
            weak_dependency: <::std::vec::Vec::<i32> as Clone>::clone(&self.weak_dependency),
            message_type: <::std::vec::Vec::<self::DescriptorProto> as Clone>::clone(&self.message_type),
            enum_type: <::std::vec::Vec::<self::EnumDescriptorProto> as Clone>::clone(&self.enum_type),
            service: <::std::vec::Vec::<self::ServiceDescriptorProto> as Clone>::clone(&self.service),
            extension: <::std::vec::Vec::<self::FieldDescriptorProto> as Clone>::clone(&self.extension),
            options: <::std::option::Option::<::std::boxed::Box::<self::FileOptions>> as Clone>::clone(&self.options),
            source_code_info: <::std::option::Option::<::std::boxed::Box::<self::SourceCodeInfo>> as Clone>::clone(&self.source_code_info),
            syntax: <::std::option::Option::<::std::string::String> as Clone>::clone(&self.syntax),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for FileDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.package, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::vec::Vec::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.dependency, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::vec::Vec::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.public_dependency, field, ::std::default::Default::default)?;
            }
            11 => {
                <::std::vec::Vec::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.weak_dependency, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec::<self::DescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::DescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.message_type, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec::<self::EnumDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::EnumDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.enum_type, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec::<self::ServiceDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::ServiceDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.service, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::vec::Vec::<self::FieldDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::FieldDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.extension, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option::<::std::boxed::Box::<self::FileOptions>> as FieldMergeFromIter<
                    tags::Message::<self::FileOptions>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option::<::std::boxed::Box::<self::SourceCodeInfo>> as FieldMergeFromIter<
                    tags::Message::<self::SourceCodeInfo>, 
                    tags::Optional2>>
                ::merge(&mut self.source_code_info, field, ::std::default::Default::default)?;
            }
            12 => {
                <::std::option::Option::<::std::string::String> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.syntax, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FileDescriptorProto {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FileDescriptorProto {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for FileDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.package, serializer, 2)?;
        <::std::vec::Vec::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.dependency, serializer, 3)?;
        <::std::vec::Vec::<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.public_dependency, serializer, 10)?;
        <::std::vec::Vec::<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.weak_dependency, serializer, 11)?;
        <::std::vec::Vec::<self::DescriptorProto> as FieldSer<
                tags::Message::<self::DescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.message_type, serializer, 4)?;
        <::std::vec::Vec::<self::EnumDescriptorProto> as FieldSer<
                tags::Message::<self::EnumDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 5)?;
        <::std::vec::Vec::<self::ServiceDescriptorProto> as FieldSer<
                tags::Message::<self::ServiceDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.service, serializer, 6)?;
        <::std::vec::Vec::<self::FieldDescriptorProto> as FieldSer<
                tags::Message::<self::FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 7)?;
        <::std::option::Option::<::std::boxed::Box::<self::FileOptions>> as FieldSer<
                tags::Message::<self::FileOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option::<::std::boxed::Box::<self::SourceCodeInfo>> as FieldSer<
                tags::Message::<self::SourceCodeInfo>, 
                tags::Optional2>>
            ::ser(&self.source_code_info, serializer, 9)?;
        <::std::option::Option::<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.syntax, serializer, 12)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FileDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::FileDescriptorProtoTrait for FileDescriptorProto {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.package.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type DependencyRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<::std::string::String>;
    fn dependency<'this>(&'this self) -> Self::DependencyRepeated::<'this> {
        &self.dependency
    }
    type PublicDependencyRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<i32>;
    fn public_dependency<'this>(&'this self) -> Self::PublicDependencyRepeated::<'this> {
        &self.public_dependency
    }
    type WeakDependencyRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<i32>;
    fn weak_dependency<'this>(&'this self) -> Self::WeakDependencyRepeated::<'this> {
        &self.weak_dependency
    }
    type MessageTypeElement<'this> where Self: 'this = self::DescriptorProto;
    type MessageTypeRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::DescriptorProto>;
    fn message_type<'this>(&'this self) -> Self::MessageTypeRepeated::<'this> {
        &self.message_type
    }
    type EnumTypeElement<'this> where Self: 'this = self::EnumDescriptorProto;
    type EnumTypeRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::EnumDescriptorProto>;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this> {
        &self.enum_type
    }
    type ServiceElement<'this> where Self: 'this = self::ServiceDescriptorProto;
    type ServiceRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::ServiceDescriptorProto>;
    fn service<'this>(&'this self) -> Self::ServiceRepeated::<'this> {
        &self.service
    }
    type ExtensionElement<'this> where Self: 'this = self::FieldDescriptorProto;
    type ExtensionRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::FieldDescriptorProto>;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this> {
        &self.extension
    }
    type OptionsType<'this> where Self: 'this = self::FileOptions;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type SourceCodeInfoType<'this> where Self: 'this = self::SourceCodeInfo;
    fn source_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::SourceCodeInfoType::<'this>>> {
        self.source_code_info.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn syntax<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.syntax.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl ::puroro::Message for FileDescriptorProto {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileDescriptorProtoTag> for FileDescriptorProto {
}

impl<'a> ::puroro_internal::FieldNew<'a> for FileDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct FileDescriptorSet {
    pub file: ::std::vec::Vec::<self::FileDescriptorProto>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl FileDescriptorSet {
    pub fn new() -> Self {
        Self {
            file: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            file: <::std::vec::Vec::<self::FileDescriptorProto> as Clone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ::puroro_internal::deser::MergeableMessageFromIter for FileDescriptorSet {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::FieldMergeFromIter;
        use ::puroro::InternalData;
        use ::puroro::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec::<self::FileDescriptorProto> as FieldMergeFromIter<
                    tags::Message::<self::FileDescriptorProto>, 
                    tags::Repeated>>
                ::merge(&mut self.file, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FileDescriptorSet {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FileDescriptorSet {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut message = ::std::default::Default::default();
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(
            &mut message
        );
        let wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.merge_into_message(&mut from_slice)?;
        Ok(message)
    }
}

impl ::puroro_internal::ser::SerializableMessage for FileDescriptorSet {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::vec::Vec::<self::FileDescriptorProto> as FieldSer<
                tags::Message::<self::FileDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 1)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FileDescriptorSet {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl super::super::super::traits::google::protobuf::FileDescriptorSetTrait for FileDescriptorSet {
    type FileElement<'this> where Self: 'this = self::FileDescriptorProto;
    type FileRepeated<'this> where Self: 'this = &'this ::std::vec::Vec::<self::FileDescriptorProto>;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this> {
        &self.file
    }
}

impl ::puroro::Message for FileDescriptorSet {
    type InternalData = ::puroro_internal::InternalDataForNormalStruct;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileDescriptorSetTag> for FileDescriptorSet {
}

impl<'a> ::puroro_internal::FieldNew<'a> for FileDescriptorSet {
    fn new() -> Self {
        Default::default()
    }
}


pub mod compiler;
