#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct GeneratedCodeInfo<'bump> {
    pub annotation: ::puroro::bumpalo::collections::Vec::<'bump, self::generated_code_info::Annotation::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> GeneratedCodeInfo<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            annotation: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for GeneratedCodeInfo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            annotation: <::puroro::bumpalo::collections::Vec::<'bump, self::generated_code_info::Annotation::<'bump>> as Clone>::clone(&self.annotation),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for GeneratedCodeInfo<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::generated_code_info::Annotation::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::generated_code_info::Annotation::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.annotation, field, || self::generated_code_info::Annotation::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for GeneratedCodeInfo<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for GeneratedCodeInfo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, self::generated_code_info::Annotation::<'bump>> as FieldSer<
                tags::Message::<self::generated_code_info::Annotation::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.annotation, serializer, 1)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for GeneratedCodeInfo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::GeneratedCodeInfoTrait for GeneratedCodeInfo<'bump> {
    type AnnotationElement<'this> where Self: 'this = self::generated_code_info::Annotation::<'bump>;
    type AnnotationRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::generated_code_info::Annotation::<'bump>>;
    fn annotation<'this>(&'this self) -> Self::AnnotationRepeated::<'this> {
        &self.annotation
    }
}

impl<'bump> ::puroro::Message for GeneratedCodeInfo<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::GeneratedCodeInfoTag> for GeneratedCodeInfo<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for GeneratedCodeInfo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod generated_code_info {
#[derive(Debug)]
pub struct Annotation<'bump> {
    pub path: ::puroro::bumpalo::collections::Vec::<'bump, i32>,
    pub source_file: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub begin: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> Annotation<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            path: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            source_file: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            begin: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for Annotation<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            path: <::puroro::bumpalo::collections::Vec::<'bump, i32> as Clone>::clone(&self.path),
            source_file: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.source_file),
            begin: <::std::option::Option::<i32> as Clone>::clone(&self.begin),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for Annotation<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.source_file, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
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

impl<'bump> ::puroro::DeserializableFromIter for Annotation<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for Annotation<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
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

impl<'bump> ::puroro::Serializable for Annotation<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::generated_code_info::AnnotationTrait for Annotation<'bump> {
    type PathRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, i32>;
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

impl<'bump> ::puroro::Message for Annotation<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::generated_code_info::AnnotationTag> for Annotation<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for Annotation<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

} // mod generated_code_info
#[derive(Debug)]
pub struct SourceCodeInfo<'bump> {
    pub location: ::puroro::bumpalo::collections::Vec::<'bump, self::source_code_info::Location::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> SourceCodeInfo<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            location: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for SourceCodeInfo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            location: <::puroro::bumpalo::collections::Vec::<'bump, self::source_code_info::Location::<'bump>> as Clone>::clone(&self.location),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for SourceCodeInfo<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::source_code_info::Location::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::source_code_info::Location::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.location, field, || self::source_code_info::Location::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for SourceCodeInfo<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for SourceCodeInfo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, self::source_code_info::Location::<'bump>> as FieldSer<
                tags::Message::<self::source_code_info::Location::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.location, serializer, 1)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for SourceCodeInfo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::SourceCodeInfoTrait for SourceCodeInfo<'bump> {
    type LocationElement<'this> where Self: 'this = self::source_code_info::Location::<'bump>;
    type LocationRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::source_code_info::Location::<'bump>>;
    fn location<'this>(&'this self) -> Self::LocationRepeated::<'this> {
        &self.location
    }
}

impl<'bump> ::puroro::Message for SourceCodeInfo<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::SourceCodeInfoTag> for SourceCodeInfo<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for SourceCodeInfo<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod source_code_info {
#[derive(Debug)]
pub struct Location<'bump> {
    pub path: ::puroro::bumpalo::collections::Vec::<'bump, i32>,
    pub span: ::puroro::bumpalo::collections::Vec::<'bump, i32>,
    pub leading_comments: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub trailing_comments: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub leading_detached_comments: ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> Location<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            path: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            span: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            leading_comments: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            trailing_comments: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            leading_detached_comments: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for Location<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            path: <::puroro::bumpalo::collections::Vec::<'bump, i32> as Clone>::clone(&self.path),
            span: <::puroro::bumpalo::collections::Vec::<'bump, i32> as Clone>::clone(&self.span),
            leading_comments: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.leading_comments),
            trailing_comments: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.trailing_comments),
            leading_detached_comments: <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.leading_detached_comments),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for Location<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.span, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.leading_comments, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            4 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.trailing_comments, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            6 => {
                <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.leading_detached_comments, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for Location<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for Location<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.span, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.leading_comments, serializer, 3)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.trailing_comments, serializer, 4)?;
        <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.leading_detached_comments, serializer, 6)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for Location<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::source_code_info::LocationTrait for Location<'bump> {
    type PathRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, i32>;
    fn path<'this>(&'this self) -> Self::PathRepeated::<'this> {
        &self.path
    }
    type SpanRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, i32>;
    fn span<'this>(&'this self) -> Self::SpanRepeated::<'this> {
        &self.span
    }
    fn leading_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.leading_comments.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn trailing_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.trailing_comments.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type LeadingDetachedCommentsRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>;
    fn leading_detached_comments<'this>(&'this self) -> Self::LeadingDetachedCommentsRepeated::<'this> {
        &self.leading_detached_comments
    }
}

impl<'bump> ::puroro::Message for Location<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::source_code_info::LocationTag> for Location<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for Location<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

} // mod source_code_info
#[derive(Debug)]
pub struct UninterpretedOption<'bump> {
    pub name: ::puroro::bumpalo::collections::Vec::<'bump, self::uninterpreted_option::NamePart::<'bump>>,
    pub identifier_value: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub positive_int_value: ::std::option::Option::<u64>,
    pub negative_int_value: ::std::option::Option::<i64>,
    pub double_value: ::std::option::Option::<f64>,
    pub string_value: ::std::option::Option::<::puroro::bumpalo::collections::Vec::<'bump, u8>>,
    pub aggregate_value: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> UninterpretedOption<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            identifier_value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            positive_int_value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            negative_int_value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            double_value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            string_value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            aggregate_value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for UninterpretedOption<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::puroro::bumpalo::collections::Vec::<'bump, self::uninterpreted_option::NamePart::<'bump>> as Clone>::clone(&self.name),
            identifier_value: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.identifier_value),
            positive_int_value: <::std::option::Option::<u64> as Clone>::clone(&self.positive_int_value),
            negative_int_value: <::std::option::Option::<i64> as Clone>::clone(&self.negative_int_value),
            double_value: <::std::option::Option::<f64> as Clone>::clone(&self.double_value),
            string_value: <::std::option::Option::<::puroro::bumpalo::collections::Vec::<'bump, u8>> as Clone>::clone(&self.string_value),
            aggregate_value: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.aggregate_value),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for UninterpretedOption<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::uninterpreted_option::NamePart::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::uninterpreted_option::NamePart::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.name, field, || self::uninterpreted_option::NamePart::<'bump>::new_in(&puroro_internal.bump))?;
            }
            3 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.identifier_value, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
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
                <::std::option::Option::<::puroro::bumpalo::collections::Vec::<'bump, u8>> as FieldMergeFromIter<
                    tags::Bytes, 
                    tags::Optional2>>
                ::merge(&mut self.string_value, field, || ::puroro::bumpalo::collections::Vec::new_in(&puroro_internal.bump))?;
            }
            8 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.aggregate_value, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for UninterpretedOption<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for UninterpretedOption<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, self::uninterpreted_option::NamePart::<'bump>> as FieldSer<
                tags::Message::<self::uninterpreted_option::NamePart::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.name, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
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
        <::std::option::Option::<::puroro::bumpalo::collections::Vec::<'bump, u8>> as FieldSer<
                tags::Bytes, 
                tags::Optional2>>
            ::ser(&self.string_value, serializer, 7)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.aggregate_value, serializer, 8)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for UninterpretedOption<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::UninterpretedOptionTrait for UninterpretedOption<'bump> {
    type NameElement<'this> where Self: 'this = self::uninterpreted_option::NamePart::<'bump>;
    type NameRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::uninterpreted_option::NamePart::<'bump>>;
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

impl<'bump> ::puroro::Message for UninterpretedOption<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::UninterpretedOptionTag> for UninterpretedOption<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for UninterpretedOption<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod uninterpreted_option {
#[derive(Debug)]
pub struct NamePart<'bump> {
    pub name_part: ::puroro::bumpalo::collections::String::<'bump>,
    pub is_extension: bool,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> NamePart<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name_part: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            is_extension: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for NamePart<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name_part: <::puroro::bumpalo::collections::String::<'bump> as Clone>::clone(&self.name_part),
            is_extension: <bool as Clone>::clone(&self.is_extension),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for NamePart<'bump> {
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
                <::puroro::bumpalo::collections::String::<'bump> as FieldMergeFromIter<
                    tags::String, 
                    tags::Required>>
                ::merge(&mut self.name_part, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
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

impl<'bump> ::puroro::DeserializableFromIter for NamePart<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for NamePart<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::String::<'bump> as FieldSer<
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

impl<'bump> ::puroro::Serializable for NamePart<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::uninterpreted_option::NamePartTrait for NamePart<'bump> {
    fn name_part<'this>(&'this self) -> ::std::borrow::Cow::<'this, str> {
        ::std::borrow::Cow::Borrowed(self.name_part.as_ref())
    }
    fn is_extension<'this>(&'this self) -> bool {
        self.is_extension.clone()
    }
}

impl<'bump> ::puroro::Message for NamePart<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::uninterpreted_option::NamePartTag> for NamePart<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for NamePart<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

} // mod uninterpreted_option
#[derive(Debug)]
pub struct MethodOptions<'bump> {
    pub deprecated: ::std::option::Option::<bool>,
    pub idempotency_level: ::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel>,
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> MethodOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            idempotency_level: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for MethodOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            idempotency_level: <::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> as Clone>::clone(&self.idempotency_level),
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for MethodOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for MethodOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for MethodOptions<'bump> {
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
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for MethodOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::MethodOptionsTrait for MethodOptions<'bump> {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    fn idempotency_level<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> {
        self.idempotency_level.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for MethodOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MethodOptionsTag> for MethodOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for MethodOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod method_options {
} // mod method_options
#[derive(Debug)]
pub struct ServiceOptions<'bump> {
    pub deprecated: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> ServiceOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for ServiceOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for ServiceOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for ServiceOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for ServiceOptions<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for ServiceOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::ServiceOptionsTrait for ServiceOptions<'bump> {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for ServiceOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ServiceOptionsTag> for ServiceOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for ServiceOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct EnumValueOptions<'bump> {
    pub deprecated: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> EnumValueOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for EnumValueOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for EnumValueOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for EnumValueOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumValueOptions<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 1)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for EnumValueOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::EnumValueOptionsTrait for EnumValueOptions<'bump> {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for EnumValueOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumValueOptionsTag> for EnumValueOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for EnumValueOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct EnumOptions<'bump> {
    pub allow_alias: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> EnumOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            allow_alias: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for EnumOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option::<bool> as Clone>::clone(&self.allow_alias),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for EnumOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for EnumOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumOptions<'bump> {
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
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for EnumOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::EnumOptionsTrait for EnumOptions<'bump> {
    fn allow_alias<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.allow_alias.clone()
    }
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for EnumOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumOptionsTag> for EnumOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for EnumOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct OneofOptions<'bump> {
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> OneofOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for OneofOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for OneofOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for OneofOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for OneofOptions<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for OneofOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::OneofOptionsTrait for OneofOptions<'bump> {
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for OneofOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::OneofOptionsTag> for OneofOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for OneofOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct FieldOptions<'bump> {
    pub ctype: ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype>,
    pub packed: ::std::option::Option::<bool>,
    pub jstype: ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype>,
    pub lazy: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub weak: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> FieldOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            ctype: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            packed: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            jstype: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            lazy: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            weak: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for FieldOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            ctype: <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype> as Clone>::clone(&self.ctype),
            packed: <::std::option::Option::<bool> as Clone>::clone(&self.packed),
            jstype: <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype> as Clone>::clone(&self.jstype),
            lazy: <::std::option::Option::<bool> as Clone>::clone(&self.lazy),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            weak: <::std::option::Option::<bool> as Clone>::clone(&self.weak),
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for FieldOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for FieldOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for FieldOptions<'bump> {
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
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for FieldOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::FieldOptionsTrait for FieldOptions<'bump> {
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
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for FieldOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FieldOptionsTag> for FieldOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for FieldOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod field_options {
} // mod field_options
#[derive(Debug)]
pub struct MessageOptions<'bump> {
    pub message_set_wire_format: ::std::option::Option::<bool>,
    pub no_standard_descriptor_accessor: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub map_entry: ::std::option::Option::<bool>,
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> MessageOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            no_standard_descriptor_accessor: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            map_entry: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for MessageOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            message_set_wire_format: <::std::option::Option::<bool> as Clone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <::std::option::Option::<bool> as Clone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            map_entry: <::std::option::Option::<bool> as Clone>::clone(&self.map_entry),
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for MessageOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for MessageOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for MessageOptions<'bump> {
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
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for MessageOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::MessageOptionsTrait for MessageOptions<'bump> {
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
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for MessageOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MessageOptionsTag> for MessageOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for MessageOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct FileOptions<'bump> {
    pub java_package: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub java_outer_classname: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub java_multiple_files: ::std::option::Option::<bool>,
    pub java_generate_equals_and_hash: ::std::option::Option::<bool>,
    pub java_string_check_utf8: ::std::option::Option::<bool>,
    pub optimize_for: ::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode>,
    pub go_package: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub cc_generic_services: ::std::option::Option::<bool>,
    pub java_generic_services: ::std::option::Option::<bool>,
    pub py_generic_services: ::std::option::Option::<bool>,
    pub php_generic_services: ::std::option::Option::<bool>,
    pub deprecated: ::std::option::Option::<bool>,
    pub cc_enable_arenas: ::std::option::Option::<bool>,
    pub objc_class_prefix: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub csharp_namespace: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub swift_prefix: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub php_class_prefix: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub php_namespace: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub php_metadata_namespace: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub ruby_package: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> FileOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            java_package: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            java_outer_classname: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            java_multiple_files: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            java_generate_equals_and_hash: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            java_string_check_utf8: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            optimize_for: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            go_package: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            cc_generic_services: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            java_generic_services: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            py_generic_services: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            php_generic_services: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            cc_enable_arenas: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            objc_class_prefix: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            csharp_namespace: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            swift_prefix: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            php_class_prefix: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            php_namespace: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            php_metadata_namespace: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            ruby_package: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for FileOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            java_package: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.java_package),
            java_outer_classname: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.java_outer_classname),
            java_multiple_files: <::std::option::Option::<bool> as Clone>::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: <::std::option::Option::<bool> as Clone>::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: <::std::option::Option::<bool> as Clone>::clone(&self.java_string_check_utf8),
            optimize_for: <::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode> as Clone>::clone(&self.optimize_for),
            go_package: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.go_package),
            cc_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.cc_generic_services),
            java_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.java_generic_services),
            py_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.py_generic_services),
            php_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.php_generic_services),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            cc_enable_arenas: <::std::option::Option::<bool> as Clone>::clone(&self.cc_enable_arenas),
            objc_class_prefix: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.objc_class_prefix),
            csharp_namespace: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.csharp_namespace),
            swift_prefix: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.swift_prefix),
            php_class_prefix: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.php_class_prefix),
            php_namespace: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.php_namespace),
            php_metadata_namespace: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.php_metadata_namespace),
            ruby_package: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.ruby_package),
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for FileOptions<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.java_package, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            8 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.java_outer_classname, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.go_package, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.objc_class_prefix, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            37 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.csharp_namespace, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            39 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.swift_prefix, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            40 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_class_prefix, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            41 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_namespace, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            44 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_metadata_namespace, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            45 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.ruby_package, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            999 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for FileOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for FileOptions<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.java_package, serializer, 1)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
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
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
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
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.objc_class_prefix, serializer, 36)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.csharp_namespace, serializer, 37)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.swift_prefix, serializer, 39)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_class_prefix, serializer, 40)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_namespace, serializer, 41)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_metadata_namespace, serializer, 44)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.ruby_package, serializer, 45)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for FileOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::FileOptionsTrait for FileOptions<'bump> {
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
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for FileOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileOptionsTag> for FileOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for FileOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod file_options {
} // mod file_options
#[derive(Debug)]
pub struct MethodDescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub input_type: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub output_type: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::MethodOptions::<'bump>>>,
    pub client_streaming: ::std::option::Option::<bool>,
    pub server_streaming: ::std::option::Option::<bool>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> MethodDescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            input_type: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            output_type: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            client_streaming: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            server_streaming: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for MethodDescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            input_type: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.input_type),
            output_type: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.output_type),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            client_streaming: <::std::option::Option::<bool> as Clone>::clone(&self.client_streaming),
            server_streaming: <::std::option::Option::<bool> as Clone>::clone(&self.server_streaming),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for MethodDescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.input_type, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            3 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.output_type, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            4 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::MethodOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::MethodOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::MethodOptions::<'bump>::new_in(&puroro_internal.bump))?;
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

impl<'bump> ::puroro::DeserializableFromIter for MethodDescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for MethodDescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.input_type, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.output_type, serializer, 3)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::MethodOptions::<'bump>>> as FieldSer<
                tags::Message::<self::MethodOptions::<'bump>>, 
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

impl<'bump> ::puroro::Serializable for MethodDescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::MethodDescriptorProtoTrait for MethodDescriptorProto<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn input_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.input_type.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn output_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.output_type.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type OptionsType<'this> where Self: 'this = self::MethodOptions::<'bump>;
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

impl<'bump> ::puroro::Message for MethodDescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MethodDescriptorProtoTag> for MethodDescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for MethodDescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct ServiceDescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub method: ::puroro::bumpalo::collections::Vec::<'bump, self::MethodDescriptorProto::<'bump>>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::ServiceOptions::<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> ServiceDescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            method: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for ServiceDescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            method: <::puroro::bumpalo::collections::Vec::<'bump, self::MethodDescriptorProto::<'bump>> as Clone>::clone(&self.method),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for ServiceDescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::MethodDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::MethodDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.method, field, || self::MethodDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            3 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::ServiceOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::ServiceOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::ServiceOptions::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for ServiceDescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for ServiceDescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::MethodDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::MethodDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.method, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::ServiceOptions::<'bump>>> as FieldSer<
                tags::Message::<self::ServiceOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for ServiceDescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::ServiceDescriptorProtoTrait for ServiceDescriptorProto<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type MethodElement<'this> where Self: 'this = self::MethodDescriptorProto::<'bump>;
    type MethodRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::MethodDescriptorProto::<'bump>>;
    fn method<'this>(&'this self) -> Self::MethodRepeated::<'this> {
        &self.method
    }
    type OptionsType<'this> where Self: 'this = self::ServiceOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl<'bump> ::puroro::Message for ServiceDescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ServiceDescriptorProtoTag> for ServiceDescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for ServiceDescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct EnumValueDescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub number: ::std::option::Option::<i32>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::EnumValueOptions::<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> EnumValueDescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            number: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for EnumValueDescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            number: <::std::option::Option::<i32> as Clone>::clone(&self.number),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for EnumValueDescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.number, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::EnumValueOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::EnumValueOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::EnumValueOptions::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for EnumValueDescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumValueDescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.number, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::EnumValueOptions::<'bump>>> as FieldSer<
                tags::Message::<self::EnumValueOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for EnumValueDescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::EnumValueDescriptorProtoTrait for EnumValueDescriptorProto<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn number<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.number.clone()
    }
    type OptionsType<'this> where Self: 'this = self::EnumValueOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl<'bump> ::puroro::Message for EnumValueDescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumValueDescriptorProtoTag> for EnumValueDescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for EnumValueDescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct EnumDescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub value: ::puroro::bumpalo::collections::Vec::<'bump, self::EnumValueDescriptorProto::<'bump>>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::EnumOptions::<'bump>>>,
    pub reserved_range: ::puroro::bumpalo::collections::Vec::<'bump, self::enum_descriptor_proto::EnumReservedRange::<'bump>>,
    pub reserved_name: ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> EnumDescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            reserved_range: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            reserved_name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for EnumDescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            value: <::puroro::bumpalo::collections::Vec::<'bump, self::EnumValueDescriptorProto::<'bump>> as Clone>::clone(&self.value),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            reserved_range: <::puroro::bumpalo::collections::Vec::<'bump, self::enum_descriptor_proto::EnumReservedRange::<'bump>> as Clone>::clone(&self.reserved_range),
            reserved_name: <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for EnumDescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::EnumValueDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::EnumValueDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.value, field, || self::EnumValueDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            3 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::EnumOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::EnumOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::EnumOptions::<'bump>::new_in(&puroro_internal.bump))?;
            }
            4 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::enum_descriptor_proto::EnumReservedRange::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::enum_descriptor_proto::EnumReservedRange::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_range, field, || self::enum_descriptor_proto::EnumReservedRange::<'bump>::new_in(&puroro_internal.bump))?;
            }
            5 => {
                <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for EnumDescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumDescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::EnumValueDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::EnumValueDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.value, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::EnumOptions::<'bump>>> as FieldSer<
                tags::Message::<self::EnumOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::enum_descriptor_proto::EnumReservedRange::<'bump>> as FieldSer<
                tags::Message::<self::enum_descriptor_proto::EnumReservedRange::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 4)?;
        <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.reserved_name, serializer, 5)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for EnumDescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::EnumDescriptorProtoTrait for EnumDescriptorProto<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ValueElement<'this> where Self: 'this = self::EnumValueDescriptorProto::<'bump>;
    type ValueRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::EnumValueDescriptorProto::<'bump>>;
    fn value<'this>(&'this self) -> Self::ValueRepeated::<'this> {
        &self.value
    }
    type OptionsType<'this> where Self: 'this = self::EnumOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ReservedRangeElement<'this> where Self: 'this = self::enum_descriptor_proto::EnumReservedRange::<'bump>;
    type ReservedRangeRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::enum_descriptor_proto::EnumReservedRange::<'bump>>;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this> {
        &self.reserved_range
    }
    type ReservedNameRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this> {
        &self.reserved_name
    }
}

impl<'bump> ::puroro::Message for EnumDescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumDescriptorProtoTag> for EnumDescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for EnumDescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod enum_descriptor_proto {
#[derive(Debug)]
pub struct EnumReservedRange<'bump> {
    pub start: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> EnumReservedRange<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for EnumReservedRange<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for EnumReservedRange<'bump> {
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

impl<'bump> ::puroro::DeserializableFromIter for EnumReservedRange<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumReservedRange<'bump> {
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

impl<'bump> ::puroro::Serializable for EnumReservedRange<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::enum_descriptor_proto::EnumReservedRangeTrait for EnumReservedRange<'bump> {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl<'bump> ::puroro::Message for EnumReservedRange<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::enum_descriptor_proto::EnumReservedRangeTag> for EnumReservedRange<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for EnumReservedRange<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

} // mod enum_descriptor_proto
#[derive(Debug)]
pub struct OneofDescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::OneofOptions::<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> OneofDescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for OneofDescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for OneofDescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::OneofOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::OneofOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::OneofOptions::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for OneofDescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for OneofDescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::OneofOptions::<'bump>>> as FieldSer<
                tags::Message::<self::OneofOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 2)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for OneofDescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::OneofDescriptorProtoTrait for OneofDescriptorProto<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type OptionsType<'this> where Self: 'this = self::OneofOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl<'bump> ::puroro::Message for OneofDescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::OneofDescriptorProtoTag> for OneofDescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for OneofDescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct FieldDescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub number: ::std::option::Option::<i32>,
    pub label: ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label>,
    pub type_: ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type>,
    pub type_name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub extendee: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub default_value: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub oneof_index: ::std::option::Option::<i32>,
    pub json_name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::FieldOptions::<'bump>>>,
    pub proto3_optional: ::std::option::Option::<bool>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> FieldDescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            number: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            label: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            type_: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            type_name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            extendee: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            default_value: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            oneof_index: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            json_name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            proto3_optional: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for FieldDescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            number: <::std::option::Option::<i32> as Clone>::clone(&self.number),
            label: <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label> as Clone>::clone(&self.label),
            type_: <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type> as Clone>::clone(&self.type_),
            type_name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.type_name),
            extendee: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.extendee),
            default_value: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.default_value),
            oneof_index: <::std::option::Option::<i32> as Clone>::clone(&self.oneof_index),
            json_name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.json_name),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            proto3_optional: <::std::option::Option::<bool> as Clone>::clone(&self.proto3_optional),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for FieldDescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.type_name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.extendee, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            7 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.default_value, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            9 => {
                <::std::option::Option::<i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.oneof_index, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.json_name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            8 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::FieldOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::FieldOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::FieldOptions::<'bump>::new_in(&puroro_internal.bump))?;
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

impl<'bump> ::puroro::DeserializableFromIter for FieldDescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for FieldDescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
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
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.type_name, serializer, 6)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.extendee, serializer, 2)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.default_value, serializer, 7)?;
        <::std::option::Option::<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.oneof_index, serializer, 9)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.json_name, serializer, 10)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::FieldOptions::<'bump>>> as FieldSer<
                tags::Message::<self::FieldOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option::<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.proto3_optional, serializer, 17)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for FieldDescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::FieldDescriptorProtoTrait for FieldDescriptorProto<'bump> {
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
    type OptionsType<'this> where Self: 'this = self::FieldOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn proto3_optional<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.proto3_optional.clone()
    }
}

impl<'bump> ::puroro::Message for FieldDescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FieldDescriptorProtoTag> for FieldDescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for FieldDescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod field_descriptor_proto {
} // mod field_descriptor_proto
#[derive(Debug)]
pub struct ExtensionRangeOptions<'bump> {
    pub uninterpreted_option: ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> ExtensionRangeOptions<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for ExtensionRangeOptions<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for ExtensionRangeOptions<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::UninterpretedOption::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, || self::UninterpretedOption::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for ExtensionRangeOptions<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for ExtensionRangeOptions<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>> as FieldSer<
                tags::Message::<self::UninterpretedOption::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for ExtensionRangeOptions<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::ExtensionRangeOptionsTrait for ExtensionRangeOptions<'bump> {
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'bump>;
    type UninterpretedOptionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::UninterpretedOption::<'bump>>;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        &self.uninterpreted_option
    }
}

impl<'bump> ::puroro::Message for ExtensionRangeOptions<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ExtensionRangeOptionsTag> for ExtensionRangeOptions<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for ExtensionRangeOptions<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct DescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub field: ::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>>,
    pub extension: ::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>>,
    pub nested_type: ::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>>,
    pub enum_type: ::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>>,
    pub extension_range: ::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ExtensionRange::<'bump>>,
    pub oneof_decl: ::puroro::bumpalo::collections::Vec::<'bump, self::OneofDescriptorProto::<'bump>>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::MessageOptions::<'bump>>>,
    pub reserved_range: ::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ReservedRange::<'bump>>,
    pub reserved_name: ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> DescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            field: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            extension: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            nested_type: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            enum_type: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            extension_range: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            oneof_decl: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            reserved_range: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            reserved_name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for DescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            field: <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as Clone>::clone(&self.field),
            extension: <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as Clone>::clone(&self.extension),
            nested_type: <::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>> as Clone>::clone(&self.nested_type),
            enum_type: <::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>> as Clone>::clone(&self.enum_type),
            extension_range: <::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ExtensionRange::<'bump>> as Clone>::clone(&self.extension_range),
            oneof_decl: <::puroro::bumpalo::collections::Vec::<'bump, self::OneofDescriptorProto::<'bump>> as Clone>::clone(&self.oneof_decl),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            reserved_range: <::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ReservedRange::<'bump>> as Clone>::clone(&self.reserved_range),
            reserved_name: <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for DescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::FieldDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.field, field, || self::FieldDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            6 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::FieldDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.extension, field, || self::FieldDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            3 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::DescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.nested_type, field, || self::DescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            4 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::EnumDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.enum_type, field, || self::EnumDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            5 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ExtensionRange::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::descriptor_proto::ExtensionRange::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.extension_range, field, || self::descriptor_proto::ExtensionRange::<'bump>::new_in(&puroro_internal.bump))?;
            }
            8 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::OneofDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::OneofDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.oneof_decl, field, || self::OneofDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            7 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::MessageOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::MessageOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::MessageOptions::<'bump>::new_in(&puroro_internal.bump))?;
            }
            9 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ReservedRange::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::descriptor_proto::ReservedRange::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_range, field, || self::descriptor_proto::ReservedRange::<'bump>::new_in(&puroro_internal.bump))?;
            }
            10 => {
                <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for DescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for DescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::FieldDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.field, serializer, 2)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::FieldDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 6)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::DescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.nested_type, serializer, 3)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::EnumDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 4)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ExtensionRange::<'bump>> as FieldSer<
                tags::Message::<self::descriptor_proto::ExtensionRange::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension_range, serializer, 5)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::OneofDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::OneofDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.oneof_decl, serializer, 8)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::MessageOptions::<'bump>>> as FieldSer<
                tags::Message::<self::MessageOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 7)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ReservedRange::<'bump>> as FieldSer<
                tags::Message::<self::descriptor_proto::ReservedRange::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 9)?;
        <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.reserved_name, serializer, 10)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for DescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::DescriptorProtoTrait for DescriptorProto<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type FieldElement<'this> where Self: 'this = self::FieldDescriptorProto::<'bump>;
    type FieldRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>>;
    fn field<'this>(&'this self) -> Self::FieldRepeated::<'this> {
        &self.field
    }
    type ExtensionElement<'this> where Self: 'this = self::FieldDescriptorProto::<'bump>;
    type ExtensionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>>;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this> {
        &self.extension
    }
    type NestedTypeElement<'this> where Self: 'this = self::DescriptorProto::<'bump>;
    type NestedTypeRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>>;
    fn nested_type<'this>(&'this self) -> Self::NestedTypeRepeated::<'this> {
        &self.nested_type
    }
    type EnumTypeElement<'this> where Self: 'this = self::EnumDescriptorProto::<'bump>;
    type EnumTypeRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>>;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this> {
        &self.enum_type
    }
    type ExtensionRangeElement<'this> where Self: 'this = self::descriptor_proto::ExtensionRange::<'bump>;
    type ExtensionRangeRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ExtensionRange::<'bump>>;
    fn extension_range<'this>(&'this self) -> Self::ExtensionRangeRepeated::<'this> {
        &self.extension_range
    }
    type OneofDeclElement<'this> where Self: 'this = self::OneofDescriptorProto::<'bump>;
    type OneofDeclRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::OneofDescriptorProto::<'bump>>;
    fn oneof_decl<'this>(&'this self) -> Self::OneofDeclRepeated::<'this> {
        &self.oneof_decl
    }
    type OptionsType<'this> where Self: 'this = self::MessageOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type ReservedRangeElement<'this> where Self: 'this = self::descriptor_proto::ReservedRange::<'bump>;
    type ReservedRangeRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::descriptor_proto::ReservedRange::<'bump>>;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this> {
        &self.reserved_range
    }
    type ReservedNameRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this> {
        &self.reserved_name
    }
}

impl<'bump> ::puroro::Message for DescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::DescriptorProtoTag> for DescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for DescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

pub mod descriptor_proto {
#[derive(Debug)]
pub struct ReservedRange<'bump> {
    pub start: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> ReservedRange<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for ReservedRange<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for ReservedRange<'bump> {
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

impl<'bump> ::puroro::DeserializableFromIter for ReservedRange<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for ReservedRange<'bump> {
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

impl<'bump> ::puroro::Serializable for ReservedRange<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::descriptor_proto::ReservedRangeTrait for ReservedRange<'bump> {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl<'bump> ::puroro::Message for ReservedRange<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::descriptor_proto::ReservedRangeTag> for ReservedRange<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for ReservedRange<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct ExtensionRange<'bump> {
    pub start: ::std::option::Option::<i32>,
    pub end: ::std::option::Option::<i32>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, super::ExtensionRangeOptions::<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> ExtensionRange<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for ExtensionRange<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for ExtensionRange<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, super::ExtensionRangeOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<super::ExtensionRangeOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || super::ExtensionRangeOptions::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for ExtensionRange<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for ExtensionRange<'bump> {
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
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, super::ExtensionRangeOptions::<'bump>>> as FieldSer<
                tags::Message::<super::ExtensionRangeOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for ExtensionRange<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::super::traits::google::protobuf::descriptor_proto::ExtensionRangeTrait for ExtensionRange<'bump> {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
    type OptionsType<'this> where Self: 'this = super::ExtensionRangeOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl<'bump> ::puroro::Message for ExtensionRange<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::descriptor_proto::ExtensionRangeTag> for ExtensionRange<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for ExtensionRange<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

} // mod descriptor_proto
#[derive(Debug)]
pub struct FileDescriptorProto<'bump> {
    pub name: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub package: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    pub dependency: ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>,
    pub public_dependency: ::puroro::bumpalo::collections::Vec::<'bump, i32>,
    pub weak_dependency: ::puroro::bumpalo::collections::Vec::<'bump, i32>,
    pub message_type: ::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>>,
    pub enum_type: ::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>>,
    pub service: ::puroro::bumpalo::collections::Vec::<'bump, self::ServiceDescriptorProto::<'bump>>,
    pub extension: ::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>>,
    pub options: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::FileOptions::<'bump>>>,
    pub source_code_info: ::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::SourceCodeInfo::<'bump>>>,
    pub syntax: ::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> FileDescriptorProto<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            package: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            dependency: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            public_dependency: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            weak_dependency: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            message_type: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            enum_type: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            service: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            extension: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            source_code_info: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            syntax: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for FileDescriptorProto<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.name),
            package: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.package),
            dependency: <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.dependency),
            public_dependency: <::puroro::bumpalo::collections::Vec::<'bump, i32> as Clone>::clone(&self.public_dependency),
            weak_dependency: <::puroro::bumpalo::collections::Vec::<'bump, i32> as Clone>::clone(&self.weak_dependency),
            message_type: <::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>> as Clone>::clone(&self.message_type),
            enum_type: <::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>> as Clone>::clone(&self.enum_type),
            service: <::puroro::bumpalo::collections::Vec::<'bump, self::ServiceDescriptorProto::<'bump>> as Clone>::clone(&self.service),
            extension: <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as Clone>::clone(&self.extension),
            options: self.options.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            source_code_info: self.source_code_info.as_ref().map(|v| ::puroro::bumpalo::boxed::Box::new_in((*v).clone(), &self.puroro_internal.bump)),
            syntax: <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as Clone>::clone(&self.syntax),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for FileDescriptorProto<'bump> {
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
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            2 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.package, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            3 => {
                <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.dependency, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            10 => {
                <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.public_dependency, field, ::std::default::Default::default)?;
            }
            11 => {
                <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldMergeFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.weak_dependency, field, ::std::default::Default::default)?;
            }
            4 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::DescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.message_type, field, || self::DescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            5 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::EnumDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.enum_type, field, || self::EnumDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            6 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::ServiceDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::ServiceDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.service, field, || self::ServiceDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            7 => {
                <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::FieldDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.extension, field, || self::FieldDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            8 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::FileOptions::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::FileOptions::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, || self::FileOptions::<'bump>::new_in(&puroro_internal.bump))?;
            }
            9 => {
                <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::SourceCodeInfo::<'bump>>> as FieldMergeFromIter<
                    tags::Message::<self::SourceCodeInfo::<'bump>>, 
                    tags::Optional2>>
                ::merge(&mut self.source_code_info, field, || self::SourceCodeInfo::<'bump>::new_in(&puroro_internal.bump))?;
            }
            12 => {
                <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldMergeFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.syntax, field, || ::puroro::bumpalo::collections::String::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for FileDescriptorProto<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for FileDescriptorProto<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.package, serializer, 2)?;
        <::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.dependency, serializer, 3)?;
        <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.public_dependency, serializer, 10)?;
        <::puroro::bumpalo::collections::Vec::<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.weak_dependency, serializer, 11)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::DescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.message_type, serializer, 4)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::EnumDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 5)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::ServiceDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::ServiceDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.service, serializer, 6)?;
        <::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::FieldDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 7)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::FileOptions::<'bump>>> as FieldSer<
                tags::Message::<self::FileOptions::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option::<::puroro::bumpalo::boxed::Box::<'bump, self::SourceCodeInfo::<'bump>>> as FieldSer<
                tags::Message::<self::SourceCodeInfo::<'bump>>, 
                tags::Optional2>>
            ::ser(&self.source_code_info, serializer, 9)?;
        <::std::option::Option::<::puroro::bumpalo::collections::String::<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.syntax, serializer, 12)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for FileDescriptorProto<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::FileDescriptorProtoTrait for FileDescriptorProto<'bump> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.package.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type DependencyRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, ::puroro::bumpalo::collections::String::<'bump>>;
    fn dependency<'this>(&'this self) -> Self::DependencyRepeated::<'this> {
        &self.dependency
    }
    type PublicDependencyRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, i32>;
    fn public_dependency<'this>(&'this self) -> Self::PublicDependencyRepeated::<'this> {
        &self.public_dependency
    }
    type WeakDependencyRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, i32>;
    fn weak_dependency<'this>(&'this self) -> Self::WeakDependencyRepeated::<'this> {
        &self.weak_dependency
    }
    type MessageTypeElement<'this> where Self: 'this = self::DescriptorProto::<'bump>;
    type MessageTypeRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::DescriptorProto::<'bump>>;
    fn message_type<'this>(&'this self) -> Self::MessageTypeRepeated::<'this> {
        &self.message_type
    }
    type EnumTypeElement<'this> where Self: 'this = self::EnumDescriptorProto::<'bump>;
    type EnumTypeRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::EnumDescriptorProto::<'bump>>;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this> {
        &self.enum_type
    }
    type ServiceElement<'this> where Self: 'this = self::ServiceDescriptorProto::<'bump>;
    type ServiceRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::ServiceDescriptorProto::<'bump>>;
    fn service<'this>(&'this self) -> Self::ServiceRepeated::<'this> {
        &self.service
    }
    type ExtensionElement<'this> where Self: 'this = self::FieldDescriptorProto::<'bump>;
    type ExtensionRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::FieldDescriptorProto::<'bump>>;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this> {
        &self.extension
    }
    type OptionsType<'this> where Self: 'this = self::FileOptions::<'bump>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    type SourceCodeInfoType<'this> where Self: 'this = self::SourceCodeInfo::<'bump>;
    fn source_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::SourceCodeInfoType::<'this>>> {
        self.source_code_info.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
    fn syntax<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.syntax.as_deref().map(|r| ::std::borrow::Cow::Borrowed(r))
    }
}

impl<'bump> ::puroro::Message for FileDescriptorProto<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileDescriptorProtoTag> for FileDescriptorProto<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for FileDescriptorProto<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}

#[derive(Debug)]
pub struct FileDescriptorSet<'bump> {
    pub file: ::puroro::bumpalo::collections::Vec::<'bump, self::FileDescriptorProto::<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}

impl<'bump> FileDescriptorSet<'bump> {
    pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self {
            file: ::puroro_internal::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new_with_bumpalo(bump),
        }
    }
}

impl<'bump> ::std::clone::Clone for FileDescriptorSet<'bump> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            file: <::puroro::bumpalo::collections::Vec::<'bump, self::FileDescriptorProto::<'bump>> as Clone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'bump> ::puroro_internal::deser::MergeableMessageFromIter for FileDescriptorSet<'bump> {
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
                <::puroro::bumpalo::collections::Vec::<'bump, self::FileDescriptorProto::<'bump>> as FieldMergeFromIter<
                    tags::Message::<self::FileDescriptorProto::<'bump>>, 
                    tags::Repeated>>
                ::merge(&mut self.file, field, || self::FileDescriptorProto::<'bump>::new_in(&puroro_internal.bump))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'bump> ::puroro::DeserializableFromIter for FileDescriptorSet<'bump> {
    fn merge_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::MergeableMessageFromIter>
            ::merge_from_iter(self, iter)
    }
}

impl<'bump> ::puroro_internal::ser::SerializableMessage for FileDescriptorSet<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::FieldSer;
        use ::puroro::tags;
        <::puroro::bumpalo::collections::Vec::<'bump, self::FileDescriptorProto::<'bump>> as FieldSer<
                tags::Message::<self::FileDescriptorProto::<'bump>>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 1)?;
        Ok(())
    }
}

impl<'bump> ::puroro::Serializable for FileDescriptorSet<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'bump> super::super::super::traits::google::protobuf::FileDescriptorSetTrait for FileDescriptorSet<'bump> {
    type FileElement<'this> where Self: 'this = self::FileDescriptorProto::<'bump>;
    type FileRepeated<'this> where Self: 'this = &'this ::puroro::bumpalo::collections::Vec::<'bump, self::FileDescriptorProto::<'bump>>;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this> {
        &self.file
    }
}

impl<'bump> ::puroro::Message for FileDescriptorSet<'bump> {
    type InternalData = ::puroro_internal::InternalDataForBumpaloStruct<'bump>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::puroro::bumpalo::boxed::Box::<'bump, Self>;
    fn into_boxed(self) -> Self::BoxedType {
        {
            let bump = self.puroro_internal.bump;
            ::puroro::bumpalo::boxed::Box::new_in(self, bump)
        }
    }
}

impl<'bump> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileDescriptorSetTag> for FileDescriptorSet<'bump> {
}

impl<'bump> ::puroro_internal::FieldNew<'bump> for FileDescriptorSet<'bump> {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_in_bumpalo(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
        Self::new_in(bump)
    }
}


pub mod compiler;
