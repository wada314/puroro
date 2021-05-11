#![allow(unused_variables)]
#![allow(unused_imports)]

pub trait GeneratedCodeInfoTrait {
    type AnnotationType: self::generated_code_info::AnnotationTrait;
    type AnnotationRepeated: ::puroro::RepeatedField<Self::AnnotationType>;
    fn annotation(&self) -> &Self::AnnotationRepeated;
}

#[derive(Debug)]
pub struct GeneratedCodeInfo {
    pub annotation: ::std::vec::Vec<self::generated_code_info::Annotation>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl GeneratedCodeInfo {
    pub fn new() -> Self {
        Self {
            annotation: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for GeneratedCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            annotation: <::std::vec::Vec<self::generated_code_info::Annotation> as FieldClone>::clone(&self.annotation),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for GeneratedCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec<self::generated_code_info::Annotation> as FieldDeserFromIter<
                    tags::Message<self::generated_code_info::Annotation>, 
                    tags::Repeated>>
                ::deser(&mut self.annotation, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for GeneratedCodeInfo {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for GeneratedCodeInfo {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for GeneratedCodeInfo {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<self::generated_code_info::Annotation> as FieldSer<
                tags::Message<self::generated_code_info::Annotation>, 
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

impl GeneratedCodeInfoTrait for GeneratedCodeInfo {
    type AnnotationType = self::generated_code_info::Annotation;
    type AnnotationRepeated = ::std::vec::Vec<self::generated_code_info::Annotation>;
    fn annotation(&self) -> &Self::AnnotationRepeated {
        &self.annotation
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for GeneratedCodeInfo {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct GeneratedCodeInfoBumpalo<'bump> {
    pub annotation: ::bumpalo::collections::Vec<'bump, self::generated_code_info::AnnotationBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> GeneratedCodeInfoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            annotation: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for GeneratedCodeInfoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            annotation: <::bumpalo::collections::Vec<'bump, self::generated_code_info::AnnotationBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.annotation, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for GeneratedCodeInfoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, self::generated_code_info::AnnotationBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::generated_code_info::AnnotationBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.annotation, field, || self::generated_code_info::AnnotationBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for GeneratedCodeInfoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for GeneratedCodeInfoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for GeneratedCodeInfoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, self::generated_code_info::AnnotationBumpalo<'bump>> as FieldSer<
                tags::Message<self::generated_code_info::AnnotationBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.annotation, serializer, 1)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for GeneratedCodeInfoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> GeneratedCodeInfoTrait for GeneratedCodeInfoBumpalo<'bump> {
    type AnnotationType = self::generated_code_info::AnnotationBumpalo<'bump>;
    type AnnotationRepeated = ::bumpalo::collections::Vec<'bump, self::generated_code_info::AnnotationBumpalo<'bump>>;
    fn annotation(&self) -> &Self::AnnotationRepeated {
        &self.annotation
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

#[derive(Debug)]
pub struct GeneratedCodeInfoSliceView<'slice> {
    annotation: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> GeneratedCodeInfoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            annotation: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for GeneratedCodeInfoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            annotation: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.annotation),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for GeneratedCodeInfoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for GeneratedCodeInfoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for GeneratedCodeInfoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub mod generated_code_info {
pub trait AnnotationTrait {
    type PathRepeated: ::puroro::RepeatedField<i32>;
    fn path(&self) -> &Self::PathRepeated;
    fn source_file(&self) -> ::std::option::Option<&'_ str>;
    fn begin(&self) -> ::std::option::Option<i32>;
    fn end(&self) -> ::std::option::Option<i32>;
}

#[derive(Debug)]
pub struct Annotation {
    pub path: ::std::vec::Vec<i32>,
    pub source_file: ::std::option::Option<::std::string::String>,
    pub begin: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl Annotation {
    pub fn new() -> Self {
        Self {
            path: ::puroro_internal::helpers::FieldNew::new(),
            source_file: ::puroro_internal::helpers::FieldNew::new(),
            begin: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for Annotation {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for Annotation {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            path: <::std::vec::Vec<i32> as FieldClone>::clone(&self.path),
            source_file: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.source_file),
            begin: <::std::option::Option<i32> as FieldClone>::clone(&self.begin),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for Annotation {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.source_file, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.begin, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for Annotation {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for Annotation {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for Annotation {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.source_file, serializer, 2)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.begin, serializer, 3)?;
        <::std::option::Option<i32> as FieldSer<
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

impl AnnotationTrait for Annotation {
    type PathRepeated = ::std::vec::Vec<i32>;
    fn path(&self) -> &Self::PathRepeated {
        &self.path
    }
    fn source_file(&self) -> ::std::option::Option<&'_ str> {
        self.source_file.as_deref()
    }
    fn begin(&self) -> ::std::option::Option<i32> {
        self.begin.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Annotation {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct AnnotationBumpalo<'bump> {
    pub path: ::bumpalo::collections::Vec<'bump, i32>,
    pub source_file: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub begin: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> AnnotationBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            path: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            source_file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            begin: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for AnnotationBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            path: <::bumpalo::collections::Vec<'bump, i32> as FieldClone>::clone_in_bumpalo(&self.path, self.puroro_internal.bumpalo()),
            source_file: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.source_file, self.puroro_internal.bumpalo()),
            begin: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.begin, self.puroro_internal.bumpalo()),
            end: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.end, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for AnnotationBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.source_file, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.begin, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for AnnotationBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for AnnotationBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for AnnotationBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.source_file, serializer, 2)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.begin, serializer, 3)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 4)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for AnnotationBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> AnnotationTrait for AnnotationBumpalo<'bump> {
    type PathRepeated = ::bumpalo::collections::Vec<'bump, i32>;
    fn path(&self) -> &Self::PathRepeated {
        &self.path
    }
    fn source_file(&self) -> ::std::option::Option<&'_ str> {
        self.source_file.as_deref()
    }
    fn begin(&self) -> ::std::option::Option<i32> {
        self.begin.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
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

#[derive(Debug)]
pub struct AnnotationSliceView<'slice> {
    path: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    source_file: ::std::option::Option<&'slice str>,
    begin: ::std::option::Option<i32>,
    end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> AnnotationSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            path: ::puroro_internal::helpers::FieldNew::new(),
            source_file: ::puroro_internal::helpers::FieldNew::new(),
            begin: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for AnnotationSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            path: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.path),
            source_file: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.source_file),
            begin: <::std::option::Option<i32> as FieldClone>::clone(&self.begin),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for AnnotationSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for AnnotationSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for AnnotationSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
} // mod generated_code_info
pub trait SourceCodeInfoTrait {
    type LocationType: self::source_code_info::LocationTrait;
    type LocationRepeated: ::puroro::RepeatedField<Self::LocationType>;
    fn location(&self) -> &Self::LocationRepeated;
}

#[derive(Debug)]
pub struct SourceCodeInfo {
    pub location: ::std::vec::Vec<self::source_code_info::Location>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl SourceCodeInfo {
    pub fn new() -> Self {
        Self {
            location: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for SourceCodeInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            location: <::std::vec::Vec<self::source_code_info::Location> as FieldClone>::clone(&self.location),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for SourceCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec<self::source_code_info::Location> as FieldDeserFromIter<
                    tags::Message<self::source_code_info::Location>, 
                    tags::Repeated>>
                ::deser(&mut self.location, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for SourceCodeInfo {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for SourceCodeInfo {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for SourceCodeInfo {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<self::source_code_info::Location> as FieldSer<
                tags::Message<self::source_code_info::Location>, 
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

impl SourceCodeInfoTrait for SourceCodeInfo {
    type LocationType = self::source_code_info::Location;
    type LocationRepeated = ::std::vec::Vec<self::source_code_info::Location>;
    fn location(&self) -> &Self::LocationRepeated {
        &self.location
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for SourceCodeInfo {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct SourceCodeInfoBumpalo<'bump> {
    pub location: ::bumpalo::collections::Vec<'bump, self::source_code_info::LocationBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SourceCodeInfoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            location: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for SourceCodeInfoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            location: <::bumpalo::collections::Vec<'bump, self::source_code_info::LocationBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.location, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for SourceCodeInfoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, self::source_code_info::LocationBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::source_code_info::LocationBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.location, field, || self::source_code_info::LocationBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for SourceCodeInfoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for SourceCodeInfoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for SourceCodeInfoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, self::source_code_info::LocationBumpalo<'bump>> as FieldSer<
                tags::Message<self::source_code_info::LocationBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.location, serializer, 1)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for SourceCodeInfoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SourceCodeInfoTrait for SourceCodeInfoBumpalo<'bump> {
    type LocationType = self::source_code_info::LocationBumpalo<'bump>;
    type LocationRepeated = ::bumpalo::collections::Vec<'bump, self::source_code_info::LocationBumpalo<'bump>>;
    fn location(&self) -> &Self::LocationRepeated {
        &self.location
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

#[derive(Debug)]
pub struct SourceCodeInfoSliceView<'slice> {
    location: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> SourceCodeInfoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            location: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for SourceCodeInfoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            location: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.location),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for SourceCodeInfoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for SourceCodeInfoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for SourceCodeInfoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub mod source_code_info {
pub trait LocationTrait {
    type PathRepeated: ::puroro::RepeatedField<i32>;
    fn path(&self) -> &Self::PathRepeated;
    type SpanRepeated: ::puroro::RepeatedField<i32>;
    fn span(&self) -> &Self::SpanRepeated;
    fn leading_comments(&self) -> ::std::option::Option<&'_ str>;
    fn trailing_comments(&self) -> ::std::option::Option<&'_ str>;
    type LeadingDetachedCommentsRepeated: ::puroro::RepeatedField<str>;
    fn leading_detached_comments(&self) -> &Self::LeadingDetachedCommentsRepeated;
}

#[derive(Debug)]
pub struct Location {
    pub path: ::std::vec::Vec<i32>,
    pub span: ::std::vec::Vec<i32>,
    pub leading_comments: ::std::option::Option<::std::string::String>,
    pub trailing_comments: ::std::option::Option<::std::string::String>,
    pub leading_detached_comments: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl Location {
    pub fn new() -> Self {
        Self {
            path: ::puroro_internal::helpers::FieldNew::new(),
            span: ::puroro_internal::helpers::FieldNew::new(),
            leading_comments: ::puroro_internal::helpers::FieldNew::new(),
            trailing_comments: ::puroro_internal::helpers::FieldNew::new(),
            leading_detached_comments: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for Location {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            path: <::std::vec::Vec<i32> as FieldClone>::clone(&self.path),
            span: <::std::vec::Vec<i32> as FieldClone>::clone(&self.span),
            leading_comments: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.leading_comments),
            trailing_comments: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.trailing_comments),
            leading_detached_comments: <::std::vec::Vec<::std::string::String> as FieldClone>::clone(&self.leading_detached_comments),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for Location {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.span, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.leading_comments, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.trailing_comments, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.leading_detached_comments, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for Location {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for Location {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for Location {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::std::vec::Vec<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.span, serializer, 2)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.leading_comments, serializer, 3)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.trailing_comments, serializer, 4)?;
        <::std::vec::Vec<::std::string::String> as FieldSer<
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

impl LocationTrait for Location {
    type PathRepeated = ::std::vec::Vec<i32>;
    fn path(&self) -> &Self::PathRepeated {
        &self.path
    }
    type SpanRepeated = ::std::vec::Vec<i32>;
    fn span(&self) -> &Self::SpanRepeated {
        &self.span
    }
    fn leading_comments(&self) -> ::std::option::Option<&'_ str> {
        self.leading_comments.as_deref()
    }
    fn trailing_comments(&self) -> ::std::option::Option<&'_ str> {
        self.trailing_comments.as_deref()
    }
    type LeadingDetachedCommentsRepeated = ::std::vec::Vec<::std::string::String>;
    fn leading_detached_comments(&self) -> &Self::LeadingDetachedCommentsRepeated {
        &self.leading_detached_comments
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Location {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct LocationBumpalo<'bump> {
    pub path: ::bumpalo::collections::Vec<'bump, i32>,
    pub span: ::bumpalo::collections::Vec<'bump, i32>,
    pub leading_comments: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub trailing_comments: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub leading_detached_comments: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> LocationBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            path: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            span: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            leading_comments: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            trailing_comments: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            leading_detached_comments: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for LocationBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            path: <::bumpalo::collections::Vec<'bump, i32> as FieldClone>::clone_in_bumpalo(&self.path, self.puroro_internal.bumpalo()),
            span: <::bumpalo::collections::Vec<'bump, i32> as FieldClone>::clone_in_bumpalo(&self.span, self.puroro_internal.bumpalo()),
            leading_comments: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.leading_comments, self.puroro_internal.bumpalo()),
            trailing_comments: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.trailing_comments, self.puroro_internal.bumpalo()),
            leading_detached_comments: <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.leading_detached_comments, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for LocationBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.path, field, ::std::default::Default::default)?;
            }
            2 => {
                <::bumpalo::collections::Vec<'bump, i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.span, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.leading_comments, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            4 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.trailing_comments, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            6 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.leading_detached_comments, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for LocationBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for LocationBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for LocationBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.path, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.span, serializer, 2)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.leading_comments, serializer, 3)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.trailing_comments, serializer, 4)?;
        <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.leading_detached_comments, serializer, 6)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for LocationBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> LocationTrait for LocationBumpalo<'bump> {
    type PathRepeated = ::bumpalo::collections::Vec<'bump, i32>;
    fn path(&self) -> &Self::PathRepeated {
        &self.path
    }
    type SpanRepeated = ::bumpalo::collections::Vec<'bump, i32>;
    fn span(&self) -> &Self::SpanRepeated {
        &self.span
    }
    fn leading_comments(&self) -> ::std::option::Option<&'_ str> {
        self.leading_comments.as_deref()
    }
    fn trailing_comments(&self) -> ::std::option::Option<&'_ str> {
        self.trailing_comments.as_deref()
    }
    type LeadingDetachedCommentsRepeated = ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>;
    fn leading_detached_comments(&self) -> &Self::LeadingDetachedCommentsRepeated {
        &self.leading_detached_comments
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

#[derive(Debug)]
pub struct LocationSliceView<'slice> {
    path: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    span: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    leading_comments: ::std::option::Option<&'slice str>,
    trailing_comments: ::std::option::Option<&'slice str>,
    leading_detached_comments: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> LocationSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            path: ::puroro_internal::helpers::FieldNew::new(),
            span: ::puroro_internal::helpers::FieldNew::new(),
            leading_comments: ::puroro_internal::helpers::FieldNew::new(),
            trailing_comments: ::puroro_internal::helpers::FieldNew::new(),
            leading_detached_comments: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for LocationSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            path: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.path),
            span: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.span),
            leading_comments: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.leading_comments),
            trailing_comments: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.trailing_comments),
            leading_detached_comments: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.leading_detached_comments),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for LocationSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for LocationSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for LocationSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
} // mod source_code_info
pub trait UninterpretedOptionTrait {
    type NamePartType: self::uninterpreted_option::NamePartTrait;
    type NameRepeated: ::puroro::RepeatedField<Self::NamePartType>;
    fn name(&self) -> &Self::NameRepeated;
    fn identifier_value(&self) -> ::std::option::Option<&'_ str>;
    fn positive_int_value(&self) -> ::std::option::Option<u64>;
    fn negative_int_value(&self) -> ::std::option::Option<i64>;
    fn double_value(&self) -> ::std::option::Option<f64>;
    fn string_value(&self) -> ::std::option::Option<&'_ [u8]>;
    fn aggregate_value(&self) -> ::std::option::Option<&'_ str>;
}

#[derive(Debug)]
pub struct UninterpretedOption {
    pub name: ::std::vec::Vec<self::uninterpreted_option::NamePart>,
    pub identifier_value: ::std::option::Option<::std::string::String>,
    pub positive_int_value: ::std::option::Option<u64>,
    pub negative_int_value: ::std::option::Option<i64>,
    pub double_value: ::std::option::Option<f64>,
    pub string_value: ::std::option::Option<::std::vec::Vec<u8>>,
    pub aggregate_value: ::std::option::Option<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
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
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for UninterpretedOption {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::vec::Vec<self::uninterpreted_option::NamePart> as FieldClone>::clone(&self.name),
            identifier_value: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.identifier_value),
            positive_int_value: <::std::option::Option<u64> as FieldClone>::clone(&self.positive_int_value),
            negative_int_value: <::std::option::Option<i64> as FieldClone>::clone(&self.negative_int_value),
            double_value: <::std::option::Option<f64> as FieldClone>::clone(&self.double_value),
            string_value: <::std::option::Option<::std::vec::Vec<u8>> as FieldClone>::clone(&self.string_value),
            aggregate_value: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.aggregate_value),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for UninterpretedOption {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            2 => {
                <::std::vec::Vec<self::uninterpreted_option::NamePart> as FieldDeserFromIter<
                    tags::Message<self::uninterpreted_option::NamePart>, 
                    tags::Repeated>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.identifier_value, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<u64> as FieldDeserFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::deser(&mut self.positive_int_value, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::option::Option<i64> as FieldDeserFromIter<
                    tags::Int64, 
                    tags::Optional2>>
                ::deser(&mut self.negative_int_value, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option<f64> as FieldDeserFromIter<
                    tags::Double, 
                    tags::Optional2>>
                ::deser(&mut self.double_value, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option<::std::vec::Vec<u8>> as FieldDeserFromIter<
                    tags::Bytes, 
                    tags::Optional2>>
                ::deser(&mut self.string_value, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.aggregate_value, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for UninterpretedOption {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for UninterpretedOption {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for UninterpretedOption {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<self::uninterpreted_option::NamePart> as FieldSer<
                tags::Message<self::uninterpreted_option::NamePart>, 
                tags::Repeated>>
            ::ser(&self.name, serializer, 2)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.identifier_value, serializer, 3)?;
        <::std::option::Option<u64> as FieldSer<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.positive_int_value, serializer, 4)?;
        <::std::option::Option<i64> as FieldSer<
                tags::Int64, 
                tags::Optional2>>
            ::ser(&self.negative_int_value, serializer, 5)?;
        <::std::option::Option<f64> as FieldSer<
                tags::Double, 
                tags::Optional2>>
            ::ser(&self.double_value, serializer, 6)?;
        <::std::option::Option<::std::vec::Vec<u8>> as FieldSer<
                tags::Bytes, 
                tags::Optional2>>
            ::ser(&self.string_value, serializer, 7)?;
        <::std::option::Option<::std::string::String> as FieldSer<
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

impl UninterpretedOptionTrait for UninterpretedOption {
    type NamePartType = self::uninterpreted_option::NamePart;
    type NameRepeated = ::std::vec::Vec<self::uninterpreted_option::NamePart>;
    fn name(&self) -> &Self::NameRepeated {
        &self.name
    }
    fn identifier_value(&self) -> ::std::option::Option<&'_ str> {
        self.identifier_value.as_deref()
    }
    fn positive_int_value(&self) -> ::std::option::Option<u64> {
        self.positive_int_value.clone()
    }
    fn negative_int_value(&self) -> ::std::option::Option<i64> {
        self.negative_int_value.clone()
    }
    fn double_value(&self) -> ::std::option::Option<f64> {
        self.double_value.clone()
    }
    fn string_value(&self) -> ::std::option::Option<&'_ [u8]> {
        self.string_value.as_deref()
    }
    fn aggregate_value(&self) -> ::std::option::Option<&'_ str> {
        self.aggregate_value.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for UninterpretedOption {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct UninterpretedOptionBumpalo<'bump> {
    pub name: ::bumpalo::collections::Vec<'bump, self::uninterpreted_option::NamePartBumpalo<'bump>>,
    pub identifier_value: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub positive_int_value: ::std::option::Option<u64>,
    pub negative_int_value: ::std::option::Option<i64>,
    pub double_value: ::std::option::Option<f64>,
    pub string_value: ::std::option::Option<::bumpalo::collections::Vec<'bump, u8>>,
    pub aggregate_value: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for UninterpretedOptionBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::bumpalo::collections::Vec<'bump, self::uninterpreted_option::NamePartBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            identifier_value: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.identifier_value, self.puroro_internal.bumpalo()),
            positive_int_value: <::std::option::Option<u64> as FieldClone>::clone_in_bumpalo(&self.positive_int_value, self.puroro_internal.bumpalo()),
            negative_int_value: <::std::option::Option<i64> as FieldClone>::clone_in_bumpalo(&self.negative_int_value, self.puroro_internal.bumpalo()),
            double_value: <::std::option::Option<f64> as FieldClone>::clone_in_bumpalo(&self.double_value, self.puroro_internal.bumpalo()),
            string_value: <::std::option::Option<::bumpalo::collections::Vec<'bump, u8>> as FieldClone>::clone_in_bumpalo(&self.string_value, self.puroro_internal.bumpalo()),
            aggregate_value: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.aggregate_value, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for UninterpretedOptionBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            2 => {
                <::bumpalo::collections::Vec<'bump, self::uninterpreted_option::NamePartBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::uninterpreted_option::NamePartBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.name, field, || self::uninterpreted_option::NamePartBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.identifier_value, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            4 => {
                <::std::option::Option<u64> as FieldDeserFromIter<
                    tags::UInt64, 
                    tags::Optional2>>
                ::deser(&mut self.positive_int_value, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::option::Option<i64> as FieldDeserFromIter<
                    tags::Int64, 
                    tags::Optional2>>
                ::deser(&mut self.negative_int_value, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option<f64> as FieldDeserFromIter<
                    tags::Double, 
                    tags::Optional2>>
                ::deser(&mut self.double_value, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option<::bumpalo::collections::Vec<'bump, u8>> as FieldDeserFromIter<
                    tags::Bytes, 
                    tags::Optional2>>
                ::deser(&mut self.string_value, field, || ::bumpalo::collections::Vec::new_in(puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.aggregate_value, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for UninterpretedOptionBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for UninterpretedOptionBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for UninterpretedOptionBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, self::uninterpreted_option::NamePartBumpalo<'bump>> as FieldSer<
                tags::Message<self::uninterpreted_option::NamePartBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.name, serializer, 2)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.identifier_value, serializer, 3)?;
        <::std::option::Option<u64> as FieldSer<
                tags::UInt64, 
                tags::Optional2>>
            ::ser(&self.positive_int_value, serializer, 4)?;
        <::std::option::Option<i64> as FieldSer<
                tags::Int64, 
                tags::Optional2>>
            ::ser(&self.negative_int_value, serializer, 5)?;
        <::std::option::Option<f64> as FieldSer<
                tags::Double, 
                tags::Optional2>>
            ::ser(&self.double_value, serializer, 6)?;
        <::std::option::Option<::bumpalo::collections::Vec<'bump, u8>> as FieldSer<
                tags::Bytes, 
                tags::Optional2>>
            ::ser(&self.string_value, serializer, 7)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.aggregate_value, serializer, 8)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for UninterpretedOptionBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> UninterpretedOptionTrait for UninterpretedOptionBumpalo<'bump> {
    type NamePartType = self::uninterpreted_option::NamePartBumpalo<'bump>;
    type NameRepeated = ::bumpalo::collections::Vec<'bump, self::uninterpreted_option::NamePartBumpalo<'bump>>;
    fn name(&self) -> &Self::NameRepeated {
        &self.name
    }
    fn identifier_value(&self) -> ::std::option::Option<&'_ str> {
        self.identifier_value.as_deref()
    }
    fn positive_int_value(&self) -> ::std::option::Option<u64> {
        self.positive_int_value.clone()
    }
    fn negative_int_value(&self) -> ::std::option::Option<i64> {
        self.negative_int_value.clone()
    }
    fn double_value(&self) -> ::std::option::Option<f64> {
        self.double_value.clone()
    }
    fn string_value(&self) -> ::std::option::Option<&'_ [u8]> {
        self.string_value.as_deref()
    }
    fn aggregate_value(&self) -> ::std::option::Option<&'_ str> {
        self.aggregate_value.as_deref()
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

#[derive(Debug)]
pub struct UninterpretedOptionSliceView<'slice> {
    name: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    identifier_value: ::std::option::Option<&'slice str>,
    positive_int_value: ::std::option::Option<u64>,
    negative_int_value: ::std::option::Option<i64>,
    double_value: ::std::option::Option<f64>,
    string_value: ::std::option::Option<&'slice [u8]>,
    aggregate_value: ::std::option::Option<&'slice str>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> UninterpretedOptionSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            identifier_value: ::puroro_internal::helpers::FieldNew::new(),
            positive_int_value: ::puroro_internal::helpers::FieldNew::new(),
            negative_int_value: ::puroro_internal::helpers::FieldNew::new(),
            double_value: ::puroro_internal::helpers::FieldNew::new(),
            string_value: ::puroro_internal::helpers::FieldNew::new(),
            aggregate_value: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for UninterpretedOptionSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.name),
            identifier_value: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.identifier_value),
            positive_int_value: <::std::option::Option<u64> as FieldClone>::clone(&self.positive_int_value),
            negative_int_value: <::std::option::Option<i64> as FieldClone>::clone(&self.negative_int_value),
            double_value: <::std::option::Option<f64> as FieldClone>::clone(&self.double_value),
            string_value: <::std::option::Option<&'slice [u8]> as FieldClone>::clone(&self.string_value),
            aggregate_value: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.aggregate_value),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for UninterpretedOptionSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for UninterpretedOptionSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for UninterpretedOptionSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub mod uninterpreted_option {
pub trait NamePartTrait {
    fn name_part(&self) -> &'_ str;
    fn is_extension(&self) -> bool;
}

#[derive(Debug)]
pub struct NamePart {
    pub name_part: ::std::string::String,
    pub is_extension: bool,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl NamePart {
    pub fn new() -> Self {
        Self {
            name_part: ::puroro_internal::helpers::FieldNew::new(),
            is_extension: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for NamePart {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for NamePart {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name_part: <::std::string::String as FieldClone>::clone(&self.name_part),
            is_extension: <bool as FieldClone>::clone(&self.is_extension),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for NamePart {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::string::String as FieldDeserFromIter<
                    tags::String, 
                    tags::Required>>
                ::deser(&mut self.name_part, field, ::std::default::Default::default)?;
            }
            2 => {
                <bool as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Required>>
                ::deser(&mut self.is_extension, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for NamePart {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for NamePart {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for NamePart {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
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
#[derive(Debug)]
pub struct NamePartBumpalo<'bump> {
    pub name_part: ::bumpalo::collections::String<'bump>,
    pub is_extension: bool,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> NamePartBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name_part: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            is_extension: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for NamePartBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name_part: <::bumpalo::collections::String<'bump> as FieldClone>::clone_in_bumpalo(&self.name_part, self.puroro_internal.bumpalo()),
            is_extension: <bool as FieldClone>::clone_in_bumpalo(&self.is_extension, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for NamePartBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::bumpalo::collections::String<'bump> as FieldDeserFromIter<
                    tags::String, 
                    tags::Required>>
                ::deser(&mut self.name_part, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <bool as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Required>>
                ::deser(&mut self.is_extension, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for NamePartBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for NamePartBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for NamePartBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::String<'bump> as FieldSer<
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
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for NamePartBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
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

#[derive(Debug)]
pub struct NamePartSliceView<'slice> {
    name_part: &'slice str,
    is_extension: bool,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> NamePartSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name_part: ::puroro_internal::helpers::FieldNew::new(),
            is_extension: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for NamePartSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name_part: <&'slice str as FieldClone>::clone(&self.name_part),
            is_extension: <bool as FieldClone>::clone(&self.is_extension),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for NamePartSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for NamePartSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for NamePartSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
} // mod uninterpreted_option
pub trait MethodOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    fn deprecated(&self) -> ::std::option::Option<bool>;
    fn idempotency_level(&self) -> ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>>;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct MethodOptions {
    pub deprecated: ::std::option::Option<bool>,
    pub idempotency_level: ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>>,
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl MethodOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            idempotency_level: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MethodOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            idempotency_level: <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldClone>::clone(&self.idempotency_level),
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MethodOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            33 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            34 => {
                <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldDeserFromIter<
                    tags::Enum<method_options::IdempotencyLevel>, 
                    tags::Optional2>>
                ::deser(&mut self.idempotency_level, field, || 0i32.try_into())?;
            }
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for MethodOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for MethodOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for MethodOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldSer<
                tags::Enum<method_options::IdempotencyLevel>, 
                tags::Optional2>>
            ::ser(&self.idempotency_level, serializer, 34)?;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl MethodOptionsTrait for MethodOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn idempotency_level(&self) -> ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> {
        self.idempotency_level.clone()
    }
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MethodOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct MethodOptionsBumpalo<'bump> {
    pub deprecated: ::std::option::Option<bool>,
    pub idempotency_level: ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            idempotency_level: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for MethodOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            idempotency_level: <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldClone>::clone_in_bumpalo(&self.idempotency_level, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for MethodOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            33 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            34 => {
                <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldDeserFromIter<
                    tags::Enum<method_options::IdempotencyLevel>, 
                    tags::Optional2>>
                ::deser(&mut self.idempotency_level, field, || 0i32.try_into())?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for MethodOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for MethodOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for MethodOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldSer<
                tags::Enum<method_options::IdempotencyLevel>, 
                tags::Optional2>>
            ::ser(&self.idempotency_level, serializer, 34)?;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MethodOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodOptionsTrait for MethodOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn idempotency_level(&self) -> ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> {
        self.idempotency_level.clone()
    }
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct MethodOptionsSliceView<'slice> {
    deprecated: ::std::option::Option<bool>,
    idempotency_level: ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>>,
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> MethodOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            idempotency_level: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for MethodOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            idempotency_level: <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldClone>::clone(&self.idempotency_level),
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for MethodOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for MethodOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for MethodOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
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
impl ::std::convert::From<IdempotencyLevel> for i32 {
    fn from(val: IdempotencyLevel) -> i32 {
        val as i32
    }
}
} // mod method_options
pub trait ServiceOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    fn deprecated(&self) -> ::std::option::Option<bool>;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct ServiceOptions {
    pub deprecated: ::std::option::Option<bool>,
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ServiceOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ServiceOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ServiceOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            33 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ServiceOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for ServiceOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for ServiceOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl ServiceOptionsTrait for ServiceOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ServiceOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ServiceOptionsBumpalo<'bump> {
    pub deprecated: ::std::option::Option<bool>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for ServiceOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ServiceOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            33 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for ServiceOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for ServiceOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for ServiceOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ServiceOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceOptionsTrait for ServiceOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct ServiceOptionsSliceView<'slice> {
    deprecated: ::std::option::Option<bool>,
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> ServiceOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for ServiceOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for ServiceOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for ServiceOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for ServiceOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait EnumValueOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    fn deprecated(&self) -> ::std::option::Option<bool>;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct EnumValueOptions {
    pub deprecated: ::std::option::Option<bool>,
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumValueOptions {
    pub fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumValueOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumValueOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for EnumValueOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumValueOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 1)?;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl EnumValueOptionsTrait for EnumValueOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumValueOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumValueOptionsBumpalo<'bump> {
    pub deprecated: ::std::option::Option<bool>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for EnumValueOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for EnumValueOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for EnumValueOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumValueOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumValueOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueOptionsTrait for EnumValueOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct EnumValueOptionsSliceView<'slice> {
    deprecated: ::std::option::Option<bool>,
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> EnumValueOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for EnumValueOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for EnumValueOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for EnumValueOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for EnumValueOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait EnumOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    fn allow_alias(&self) -> ::std::option::Option<bool>;
    fn deprecated(&self) -> ::std::option::Option<bool>;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct EnumOptions {
    pub allow_alias: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumOptions {
    pub fn new() -> Self {
        Self {
            allow_alias: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option<bool> as FieldClone>::clone(&self.allow_alias),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            2 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.allow_alias, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for EnumOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.allow_alias, serializer, 2)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl EnumOptionsTrait for EnumOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    fn allow_alias(&self) -> ::std::option::Option<bool> {
        self.allow_alias.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumOptionsBumpalo<'bump> {
    pub allow_alias: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            allow_alias: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for EnumOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.allow_alias, self.puroro_internal.bumpalo()),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            2 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.allow_alias, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for EnumOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for EnumOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.allow_alias, serializer, 2)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumOptionsTrait for EnumOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    fn allow_alias(&self) -> ::std::option::Option<bool> {
        self.allow_alias.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct EnumOptionsSliceView<'slice> {
    allow_alias: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> EnumOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            allow_alias: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for EnumOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option<bool> as FieldClone>::clone(&self.allow_alias),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for EnumOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for EnumOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for EnumOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait OneofOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct OneofOptions {
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl OneofOptions {
    pub fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for OneofOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for OneofOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for OneofOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for OneofOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for OneofOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl OneofOptionsTrait for OneofOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for OneofOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct OneofOptionsBumpalo<'bump> {
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for OneofOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for OneofOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for OneofOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for OneofOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for OneofOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for OneofOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofOptionsTrait for OneofOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct OneofOptionsSliceView<'slice> {
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> OneofOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for OneofOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for OneofOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for OneofOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for OneofOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait FieldOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    fn ctype(&self) -> ::std::option::Option<::std::result::Result<field_options::Ctype, i32>>;
    fn packed(&self) -> ::std::option::Option<bool>;
    fn jstype(&self) -> ::std::option::Option<::std::result::Result<field_options::Jstype, i32>>;
    fn lazy(&self) -> ::std::option::Option<bool>;
    fn deprecated(&self) -> ::std::option::Option<bool>;
    fn weak(&self) -> ::std::option::Option<bool>;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct FieldOptions {
    pub ctype: ::std::option::Option<::std::result::Result<field_options::Ctype, i32>>,
    pub packed: ::std::option::Option<bool>,
    pub jstype: ::std::option::Option<::std::result::Result<field_options::Jstype, i32>>,
    pub lazy: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub weak: ::std::option::Option<bool>,
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
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
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FieldOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for FieldOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            ctype: <::std::option::Option<::std::result::Result<field_options::Ctype, i32>> as FieldClone>::clone(&self.ctype),
            packed: <::std::option::Option<bool> as FieldClone>::clone(&self.packed),
            jstype: <::std::option::Option<::std::result::Result<field_options::Jstype, i32>> as FieldClone>::clone(&self.jstype),
            lazy: <::std::option::Option<bool> as FieldClone>::clone(&self.lazy),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            weak: <::std::option::Option<bool> as FieldClone>::clone(&self.weak),
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FieldOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::result::Result<field_options::Ctype, i32>> as FieldDeserFromIter<
                    tags::Enum<field_options::Ctype>, 
                    tags::Optional2>>
                ::deser(&mut self.ctype, field, || 0i32.try_into())?;
            }
            2 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.packed, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option<::std::result::Result<field_options::Jstype, i32>> as FieldDeserFromIter<
                    tags::Enum<field_options::Jstype>, 
                    tags::Optional2>>
                ::deser(&mut self.jstype, field, || 0i32.try_into())?;
            }
            5 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.lazy, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.weak, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FieldOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for FieldOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for FieldOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::result::Result<field_options::Ctype, i32>> as FieldSer<
                tags::Enum<field_options::Ctype>, 
                tags::Optional2>>
            ::ser(&self.ctype, serializer, 1)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.packed, serializer, 2)?;
        <::std::option::Option<::std::result::Result<field_options::Jstype, i32>> as FieldSer<
                tags::Enum<field_options::Jstype>, 
                tags::Optional2>>
            ::ser(&self.jstype, serializer, 6)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.lazy, serializer, 5)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.weak, serializer, 10)?;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl FieldOptionsTrait for FieldOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    fn ctype(&self) -> ::std::option::Option<::std::result::Result<field_options::Ctype, i32>> {
        self.ctype.clone()
    }
    fn packed(&self) -> ::std::option::Option<bool> {
        self.packed.clone()
    }
    fn jstype(&self) -> ::std::option::Option<::std::result::Result<field_options::Jstype, i32>> {
        self.jstype.clone()
    }
    fn lazy(&self) -> ::std::option::Option<bool> {
        self.lazy.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn weak(&self) -> ::std::option::Option<bool> {
        self.weak.clone()
    }
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FieldOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct FieldOptionsBumpalo<'bump> {
    pub ctype: ::std::option::Option<::std::result::Result<field_options::Ctype, i32>>,
    pub packed: ::std::option::Option<bool>,
    pub jstype: ::std::option::Option<::std::result::Result<field_options::Jstype, i32>>,
    pub lazy: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub weak: ::std::option::Option<bool>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for FieldOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            ctype: <::std::option::Option<::std::result::Result<field_options::Ctype, i32>> as FieldClone>::clone_in_bumpalo(&self.ctype, self.puroro_internal.bumpalo()),
            packed: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.packed, self.puroro_internal.bumpalo()),
            jstype: <::std::option::Option<::std::result::Result<field_options::Jstype, i32>> as FieldClone>::clone_in_bumpalo(&self.jstype, self.puroro_internal.bumpalo()),
            lazy: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.lazy, self.puroro_internal.bumpalo()),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            weak: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.weak, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FieldOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::result::Result<field_options::Ctype, i32>> as FieldDeserFromIter<
                    tags::Enum<field_options::Ctype>, 
                    tags::Optional2>>
                ::deser(&mut self.ctype, field, || 0i32.try_into())?;
            }
            2 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.packed, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option<::std::result::Result<field_options::Jstype, i32>> as FieldDeserFromIter<
                    tags::Enum<field_options::Jstype>, 
                    tags::Optional2>>
                ::deser(&mut self.jstype, field, || 0i32.try_into())?;
            }
            5 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.lazy, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.weak, field, ::std::default::Default::default)?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for FieldOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for FieldOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for FieldOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::result::Result<field_options::Ctype, i32>> as FieldSer<
                tags::Enum<field_options::Ctype>, 
                tags::Optional2>>
            ::ser(&self.ctype, serializer, 1)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.packed, serializer, 2)?;
        <::std::option::Option<::std::result::Result<field_options::Jstype, i32>> as FieldSer<
                tags::Enum<field_options::Jstype>, 
                tags::Optional2>>
            ::ser(&self.jstype, serializer, 6)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.lazy, serializer, 5)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.weak, serializer, 10)?;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FieldOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldOptionsTrait for FieldOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    fn ctype(&self) -> ::std::option::Option<::std::result::Result<field_options::Ctype, i32>> {
        self.ctype.clone()
    }
    fn packed(&self) -> ::std::option::Option<bool> {
        self.packed.clone()
    }
    fn jstype(&self) -> ::std::option::Option<::std::result::Result<field_options::Jstype, i32>> {
        self.jstype.clone()
    }
    fn lazy(&self) -> ::std::option::Option<bool> {
        self.lazy.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn weak(&self) -> ::std::option::Option<bool> {
        self.weak.clone()
    }
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct FieldOptionsSliceView<'slice> {
    ctype: ::std::option::Option<::std::result::Result<field_options::Ctype, i32>>,
    packed: ::std::option::Option<bool>,
    jstype: ::std::option::Option<::std::result::Result<field_options::Jstype, i32>>,
    lazy: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    weak: ::std::option::Option<bool>,
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> FieldOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            ctype: ::puroro_internal::helpers::FieldNew::new(),
            packed: ::puroro_internal::helpers::FieldNew::new(),
            jstype: ::puroro_internal::helpers::FieldNew::new(),
            lazy: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            weak: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for FieldOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            ctype: <::std::option::Option<::std::result::Result<field_options::Ctype, i32>> as FieldClone>::clone(&self.ctype),
            packed: <::std::option::Option<bool> as FieldClone>::clone(&self.packed),
            jstype: <::std::option::Option<::std::result::Result<field_options::Jstype, i32>> as FieldClone>::clone(&self.jstype),
            lazy: <::std::option::Option<bool> as FieldClone>::clone(&self.lazy),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            weak: <::std::option::Option<bool> as FieldClone>::clone(&self.weak),
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for FieldOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for FieldOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for FieldOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
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
impl ::std::convert::From<Jstype> for i32 {
    fn from(val: Jstype) -> i32 {
        val as i32
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
impl ::std::convert::From<Ctype> for i32 {
    fn from(val: Ctype) -> i32 {
        val as i32
    }
}
} // mod field_options
pub trait MessageOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    fn message_set_wire_format(&self) -> ::std::option::Option<bool>;
    fn no_standard_descriptor_accessor(&self) -> ::std::option::Option<bool>;
    fn deprecated(&self) -> ::std::option::Option<bool>;
    fn map_entry(&self) -> ::std::option::Option<bool>;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct MessageOptions {
    pub message_set_wire_format: ::std::option::Option<bool>,
    pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub map_entry: ::std::option::Option<bool>,
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl MessageOptions {
    pub fn new() -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::helpers::FieldNew::new(),
            no_standard_descriptor_accessor: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            map_entry: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MessageOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            message_set_wire_format: <::std::option::Option<bool> as FieldClone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <::std::option::Option<bool> as FieldClone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            map_entry: <::std::option::Option<bool> as FieldClone>::clone(&self.map_entry),
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MessageOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.message_set_wire_format, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.no_standard_descriptor_accessor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.map_entry, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for MessageOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for MessageOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for MessageOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.message_set_wire_format, serializer, 1)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.no_standard_descriptor_accessor, serializer, 2)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.map_entry, serializer, 7)?;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl MessageOptionsTrait for MessageOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    fn message_set_wire_format(&self) -> ::std::option::Option<bool> {
        self.message_set_wire_format.clone()
    }
    fn no_standard_descriptor_accessor(&self) -> ::std::option::Option<bool> {
        self.no_standard_descriptor_accessor.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn map_entry(&self) -> ::std::option::Option<bool> {
        self.map_entry.clone()
    }
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MessageOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct MessageOptionsBumpalo<'bump> {
    pub message_set_wire_format: ::std::option::Option<bool>,
    pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub map_entry: ::std::option::Option<bool>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MessageOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            no_standard_descriptor_accessor: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            deprecated: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            map_entry: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for MessageOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            message_set_wire_format: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.message_set_wire_format, self.puroro_internal.bumpalo()),
            no_standard_descriptor_accessor: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.no_standard_descriptor_accessor, self.puroro_internal.bumpalo()),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            map_entry: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.map_entry, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for MessageOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.message_set_wire_format, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.no_standard_descriptor_accessor, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.map_entry, field, ::std::default::Default::default)?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for MessageOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for MessageOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for MessageOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.message_set_wire_format, serializer, 1)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.no_standard_descriptor_accessor, serializer, 2)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 3)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.map_entry, serializer, 7)?;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MessageOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MessageOptionsTrait for MessageOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    fn message_set_wire_format(&self) -> ::std::option::Option<bool> {
        self.message_set_wire_format.clone()
    }
    fn no_standard_descriptor_accessor(&self) -> ::std::option::Option<bool> {
        self.no_standard_descriptor_accessor.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn map_entry(&self) -> ::std::option::Option<bool> {
        self.map_entry.clone()
    }
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct MessageOptionsSliceView<'slice> {
    message_set_wire_format: ::std::option::Option<bool>,
    no_standard_descriptor_accessor: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    map_entry: ::std::option::Option<bool>,
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> MessageOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::helpers::FieldNew::new(),
            no_standard_descriptor_accessor: ::puroro_internal::helpers::FieldNew::new(),
            deprecated: ::puroro_internal::helpers::FieldNew::new(),
            map_entry: ::puroro_internal::helpers::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for MessageOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            message_set_wire_format: <::std::option::Option<bool> as FieldClone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <::std::option::Option<bool> as FieldClone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            map_entry: <::std::option::Option<bool> as FieldClone>::clone(&self.map_entry),
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for MessageOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for MessageOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for MessageOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait FileOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    fn java_package(&self) -> ::std::option::Option<&'_ str>;
    fn java_outer_classname(&self) -> ::std::option::Option<&'_ str>;
    fn java_multiple_files(&self) -> ::std::option::Option<bool>;
    fn java_generate_equals_and_hash(&self) -> ::std::option::Option<bool>;
    fn java_string_check_utf8(&self) -> ::std::option::Option<bool>;
    fn optimize_for(&self) -> ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>>;
    fn go_package(&self) -> ::std::option::Option<&'_ str>;
    fn cc_generic_services(&self) -> ::std::option::Option<bool>;
    fn java_generic_services(&self) -> ::std::option::Option<bool>;
    fn py_generic_services(&self) -> ::std::option::Option<bool>;
    fn php_generic_services(&self) -> ::std::option::Option<bool>;
    fn deprecated(&self) -> ::std::option::Option<bool>;
    fn cc_enable_arenas(&self) -> ::std::option::Option<bool>;
    fn objc_class_prefix(&self) -> ::std::option::Option<&'_ str>;
    fn csharp_namespace(&self) -> ::std::option::Option<&'_ str>;
    fn swift_prefix(&self) -> ::std::option::Option<&'_ str>;
    fn php_class_prefix(&self) -> ::std::option::Option<&'_ str>;
    fn php_namespace(&self) -> ::std::option::Option<&'_ str>;
    fn php_metadata_namespace(&self) -> ::std::option::Option<&'_ str>;
    fn ruby_package(&self) -> ::std::option::Option<&'_ str>;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct FileOptions {
    pub java_package: ::std::option::Option<::std::string::String>,
    pub java_outer_classname: ::std::option::Option<::std::string::String>,
    pub java_multiple_files: ::std::option::Option<bool>,
    pub java_generate_equals_and_hash: ::std::option::Option<bool>,
    pub java_string_check_utf8: ::std::option::Option<bool>,
    pub optimize_for: ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>>,
    pub go_package: ::std::option::Option<::std::string::String>,
    pub cc_generic_services: ::std::option::Option<bool>,
    pub java_generic_services: ::std::option::Option<bool>,
    pub py_generic_services: ::std::option::Option<bool>,
    pub php_generic_services: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub cc_enable_arenas: ::std::option::Option<bool>,
    pub objc_class_prefix: ::std::option::Option<::std::string::String>,
    pub csharp_namespace: ::std::option::Option<::std::string::String>,
    pub swift_prefix: ::std::option::Option<::std::string::String>,
    pub php_class_prefix: ::std::option::Option<::std::string::String>,
    pub php_namespace: ::std::option::Option<::std::string::String>,
    pub php_metadata_namespace: ::std::option::Option<::std::string::String>,
    pub ruby_package: ::std::option::Option<::std::string::String>,
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
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
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for FileOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            java_package: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.java_package),
            java_outer_classname: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.java_outer_classname),
            java_multiple_files: <::std::option::Option<bool> as FieldClone>::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: <::std::option::Option<bool> as FieldClone>::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: <::std::option::Option<bool> as FieldClone>::clone(&self.java_string_check_utf8),
            optimize_for: <::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> as FieldClone>::clone(&self.optimize_for),
            go_package: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.go_package),
            cc_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.cc_generic_services),
            java_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.java_generic_services),
            py_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.py_generic_services),
            php_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.php_generic_services),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            cc_enable_arenas: <::std::option::Option<bool> as FieldClone>::clone(&self.cc_enable_arenas),
            objc_class_prefix: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.objc_class_prefix),
            csharp_namespace: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.csharp_namespace),
            swift_prefix: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.swift_prefix),
            php_class_prefix: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.php_class_prefix),
            php_namespace: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.php_namespace),
            php_metadata_namespace: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.php_metadata_namespace),
            ruby_package: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.ruby_package),
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.java_package, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.java_outer_classname, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_multiple_files, field, ::std::default::Default::default)?;
            }
            20 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_generate_equals_and_hash, field, ::std::default::Default::default)?;
            }
            27 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_string_check_utf8, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> as FieldDeserFromIter<
                    tags::Enum<file_options::OptimizeMode>, 
                    tags::Optional2>>
                ::deser(&mut self.optimize_for, field, || 0i32.try_into())?;
            }
            11 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.go_package, field, ::std::default::Default::default)?;
            }
            16 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.cc_generic_services, field, ::std::default::Default::default)?;
            }
            17 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_generic_services, field, ::std::default::Default::default)?;
            }
            18 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.py_generic_services, field, ::std::default::Default::default)?;
            }
            42 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.php_generic_services, field, ::std::default::Default::default)?;
            }
            23 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            31 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.cc_enable_arenas, field, ::std::default::Default::default)?;
            }
            36 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.objc_class_prefix, field, ::std::default::Default::default)?;
            }
            37 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.csharp_namespace, field, ::std::default::Default::default)?;
            }
            39 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.swift_prefix, field, ::std::default::Default::default)?;
            }
            40 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_class_prefix, field, ::std::default::Default::default)?;
            }
            41 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_namespace, field, ::std::default::Default::default)?;
            }
            44 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_metadata_namespace, field, ::std::default::Default::default)?;
            }
            45 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.ruby_package, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FileOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for FileOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for FileOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.java_package, serializer, 1)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.java_outer_classname, serializer, 8)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_multiple_files, serializer, 10)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_generate_equals_and_hash, serializer, 20)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_string_check_utf8, serializer, 27)?;
        <::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> as FieldSer<
                tags::Enum<file_options::OptimizeMode>, 
                tags::Optional2>>
            ::ser(&self.optimize_for, serializer, 9)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.go_package, serializer, 11)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.cc_generic_services, serializer, 16)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_generic_services, serializer, 17)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.py_generic_services, serializer, 18)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.php_generic_services, serializer, 42)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 23)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.cc_enable_arenas, serializer, 31)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.objc_class_prefix, serializer, 36)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.csharp_namespace, serializer, 37)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.swift_prefix, serializer, 39)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_class_prefix, serializer, 40)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_namespace, serializer, 41)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_metadata_namespace, serializer, 44)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.ruby_package, serializer, 45)?;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl FileOptionsTrait for FileOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    fn java_package(&self) -> ::std::option::Option<&'_ str> {
        self.java_package.as_deref()
    }
    fn java_outer_classname(&self) -> ::std::option::Option<&'_ str> {
        self.java_outer_classname.as_deref()
    }
    fn java_multiple_files(&self) -> ::std::option::Option<bool> {
        self.java_multiple_files.clone()
    }
    fn java_generate_equals_and_hash(&self) -> ::std::option::Option<bool> {
        self.java_generate_equals_and_hash.clone()
    }
    fn java_string_check_utf8(&self) -> ::std::option::Option<bool> {
        self.java_string_check_utf8.clone()
    }
    fn optimize_for(&self) -> ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> {
        self.optimize_for.clone()
    }
    fn go_package(&self) -> ::std::option::Option<&'_ str> {
        self.go_package.as_deref()
    }
    fn cc_generic_services(&self) -> ::std::option::Option<bool> {
        self.cc_generic_services.clone()
    }
    fn java_generic_services(&self) -> ::std::option::Option<bool> {
        self.java_generic_services.clone()
    }
    fn py_generic_services(&self) -> ::std::option::Option<bool> {
        self.py_generic_services.clone()
    }
    fn php_generic_services(&self) -> ::std::option::Option<bool> {
        self.php_generic_services.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn cc_enable_arenas(&self) -> ::std::option::Option<bool> {
        self.cc_enable_arenas.clone()
    }
    fn objc_class_prefix(&self) -> ::std::option::Option<&'_ str> {
        self.objc_class_prefix.as_deref()
    }
    fn csharp_namespace(&self) -> ::std::option::Option<&'_ str> {
        self.csharp_namespace.as_deref()
    }
    fn swift_prefix(&self) -> ::std::option::Option<&'_ str> {
        self.swift_prefix.as_deref()
    }
    fn php_class_prefix(&self) -> ::std::option::Option<&'_ str> {
        self.php_class_prefix.as_deref()
    }
    fn php_namespace(&self) -> ::std::option::Option<&'_ str> {
        self.php_namespace.as_deref()
    }
    fn php_metadata_namespace(&self) -> ::std::option::Option<&'_ str> {
        self.php_metadata_namespace.as_deref()
    }
    fn ruby_package(&self) -> ::std::option::Option<&'_ str> {
        self.ruby_package.as_deref()
    }
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct FileOptionsBumpalo<'bump> {
    pub java_package: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub java_outer_classname: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub java_multiple_files: ::std::option::Option<bool>,
    pub java_generate_equals_and_hash: ::std::option::Option<bool>,
    pub java_string_check_utf8: ::std::option::Option<bool>,
    pub optimize_for: ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>>,
    pub go_package: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub cc_generic_services: ::std::option::Option<bool>,
    pub java_generic_services: ::std::option::Option<bool>,
    pub py_generic_services: ::std::option::Option<bool>,
    pub php_generic_services: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub cc_enable_arenas: ::std::option::Option<bool>,
    pub objc_class_prefix: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub csharp_namespace: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub swift_prefix: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub php_class_prefix: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub php_namespace: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub php_metadata_namespace: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub ruby_package: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            java_package: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            java_outer_classname: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            java_multiple_files: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            java_generate_equals_and_hash: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for FileOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            java_package: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.java_package, self.puroro_internal.bumpalo()),
            java_outer_classname: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.java_outer_classname, self.puroro_internal.bumpalo()),
            java_multiple_files: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.java_multiple_files, self.puroro_internal.bumpalo()),
            java_generate_equals_and_hash: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.java_generate_equals_and_hash, self.puroro_internal.bumpalo()),
            java_string_check_utf8: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.java_string_check_utf8, self.puroro_internal.bumpalo()),
            optimize_for: <::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> as FieldClone>::clone_in_bumpalo(&self.optimize_for, self.puroro_internal.bumpalo()),
            go_package: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.go_package, self.puroro_internal.bumpalo()),
            cc_generic_services: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.cc_generic_services, self.puroro_internal.bumpalo()),
            java_generic_services: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.java_generic_services, self.puroro_internal.bumpalo()),
            py_generic_services: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.py_generic_services, self.puroro_internal.bumpalo()),
            php_generic_services: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.php_generic_services, self.puroro_internal.bumpalo()),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            cc_enable_arenas: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.cc_enable_arenas, self.puroro_internal.bumpalo()),
            objc_class_prefix: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.objc_class_prefix, self.puroro_internal.bumpalo()),
            csharp_namespace: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.csharp_namespace, self.puroro_internal.bumpalo()),
            swift_prefix: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.swift_prefix, self.puroro_internal.bumpalo()),
            php_class_prefix: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.php_class_prefix, self.puroro_internal.bumpalo()),
            php_namespace: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.php_namespace, self.puroro_internal.bumpalo()),
            php_metadata_namespace: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.php_metadata_namespace, self.puroro_internal.bumpalo()),
            ruby_package: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.ruby_package, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.java_package, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.java_outer_classname, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            10 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_multiple_files, field, ::std::default::Default::default)?;
            }
            20 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_generate_equals_and_hash, field, ::std::default::Default::default)?;
            }
            27 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_string_check_utf8, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> as FieldDeserFromIter<
                    tags::Enum<file_options::OptimizeMode>, 
                    tags::Optional2>>
                ::deser(&mut self.optimize_for, field, || 0i32.try_into())?;
            }
            11 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.go_package, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            16 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.cc_generic_services, field, ::std::default::Default::default)?;
            }
            17 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.java_generic_services, field, ::std::default::Default::default)?;
            }
            18 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.py_generic_services, field, ::std::default::Default::default)?;
            }
            42 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.php_generic_services, field, ::std::default::Default::default)?;
            }
            23 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            31 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.cc_enable_arenas, field, ::std::default::Default::default)?;
            }
            36 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.objc_class_prefix, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            37 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.csharp_namespace, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            39 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.swift_prefix, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            40 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_class_prefix, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            41 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_namespace, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            44 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_metadata_namespace, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            45 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.ruby_package, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for FileOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for FileOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for FileOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.java_package, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.java_outer_classname, serializer, 8)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_multiple_files, serializer, 10)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_generate_equals_and_hash, serializer, 20)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_string_check_utf8, serializer, 27)?;
        <::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> as FieldSer<
                tags::Enum<file_options::OptimizeMode>, 
                tags::Optional2>>
            ::ser(&self.optimize_for, serializer, 9)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.go_package, serializer, 11)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.cc_generic_services, serializer, 16)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.java_generic_services, serializer, 17)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.py_generic_services, serializer, 18)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.php_generic_services, serializer, 42)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 23)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.cc_enable_arenas, serializer, 31)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.objc_class_prefix, serializer, 36)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.csharp_namespace, serializer, 37)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.swift_prefix, serializer, 39)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_class_prefix, serializer, 40)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_namespace, serializer, 41)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.php_metadata_namespace, serializer, 44)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.ruby_package, serializer, 45)?;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileOptionsTrait for FileOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    fn java_package(&self) -> ::std::option::Option<&'_ str> {
        self.java_package.as_deref()
    }
    fn java_outer_classname(&self) -> ::std::option::Option<&'_ str> {
        self.java_outer_classname.as_deref()
    }
    fn java_multiple_files(&self) -> ::std::option::Option<bool> {
        self.java_multiple_files.clone()
    }
    fn java_generate_equals_and_hash(&self) -> ::std::option::Option<bool> {
        self.java_generate_equals_and_hash.clone()
    }
    fn java_string_check_utf8(&self) -> ::std::option::Option<bool> {
        self.java_string_check_utf8.clone()
    }
    fn optimize_for(&self) -> ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> {
        self.optimize_for.clone()
    }
    fn go_package(&self) -> ::std::option::Option<&'_ str> {
        self.go_package.as_deref()
    }
    fn cc_generic_services(&self) -> ::std::option::Option<bool> {
        self.cc_generic_services.clone()
    }
    fn java_generic_services(&self) -> ::std::option::Option<bool> {
        self.java_generic_services.clone()
    }
    fn py_generic_services(&self) -> ::std::option::Option<bool> {
        self.py_generic_services.clone()
    }
    fn php_generic_services(&self) -> ::std::option::Option<bool> {
        self.php_generic_services.clone()
    }
    fn deprecated(&self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn cc_enable_arenas(&self) -> ::std::option::Option<bool> {
        self.cc_enable_arenas.clone()
    }
    fn objc_class_prefix(&self) -> ::std::option::Option<&'_ str> {
        self.objc_class_prefix.as_deref()
    }
    fn csharp_namespace(&self) -> ::std::option::Option<&'_ str> {
        self.csharp_namespace.as_deref()
    }
    fn swift_prefix(&self) -> ::std::option::Option<&'_ str> {
        self.swift_prefix.as_deref()
    }
    fn php_class_prefix(&self) -> ::std::option::Option<&'_ str> {
        self.php_class_prefix.as_deref()
    }
    fn php_namespace(&self) -> ::std::option::Option<&'_ str> {
        self.php_namespace.as_deref()
    }
    fn php_metadata_namespace(&self) -> ::std::option::Option<&'_ str> {
        self.php_metadata_namespace.as_deref()
    }
    fn ruby_package(&self) -> ::std::option::Option<&'_ str> {
        self.ruby_package.as_deref()
    }
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct FileOptionsSliceView<'slice> {
    java_package: ::std::option::Option<&'slice str>,
    java_outer_classname: ::std::option::Option<&'slice str>,
    java_multiple_files: ::std::option::Option<bool>,
    java_generate_equals_and_hash: ::std::option::Option<bool>,
    java_string_check_utf8: ::std::option::Option<bool>,
    optimize_for: ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>>,
    go_package: ::std::option::Option<&'slice str>,
    cc_generic_services: ::std::option::Option<bool>,
    java_generic_services: ::std::option::Option<bool>,
    py_generic_services: ::std::option::Option<bool>,
    php_generic_services: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    cc_enable_arenas: ::std::option::Option<bool>,
    objc_class_prefix: ::std::option::Option<&'slice str>,
    csharp_namespace: ::std::option::Option<&'slice str>,
    swift_prefix: ::std::option::Option<&'slice str>,
    php_class_prefix: ::std::option::Option<&'slice str>,
    php_namespace: ::std::option::Option<&'slice str>,
    php_metadata_namespace: ::std::option::Option<&'slice str>,
    ruby_package: ::std::option::Option<&'slice str>,
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> FileOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for FileOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            java_package: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.java_package),
            java_outer_classname: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.java_outer_classname),
            java_multiple_files: <::std::option::Option<bool> as FieldClone>::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: <::std::option::Option<bool> as FieldClone>::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: <::std::option::Option<bool> as FieldClone>::clone(&self.java_string_check_utf8),
            optimize_for: <::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> as FieldClone>::clone(&self.optimize_for),
            go_package: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.go_package),
            cc_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.cc_generic_services),
            java_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.java_generic_services),
            py_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.py_generic_services),
            php_generic_services: <::std::option::Option<bool> as FieldClone>::clone(&self.php_generic_services),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            cc_enable_arenas: <::std::option::Option<bool> as FieldClone>::clone(&self.cc_enable_arenas),
            objc_class_prefix: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.objc_class_prefix),
            csharp_namespace: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.csharp_namespace),
            swift_prefix: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.swift_prefix),
            php_class_prefix: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.php_class_prefix),
            php_namespace: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.php_namespace),
            php_metadata_namespace: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.php_metadata_namespace),
            ruby_package: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.ruby_package),
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for FileOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for FileOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for FileOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
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
impl ::std::convert::From<OptimizeMode> for i32 {
    fn from(val: OptimizeMode) -> i32 {
        val as i32
    }
}
} // mod file_options
pub trait MethodDescriptorProtoTrait {
    type MethodOptionsType: self::MethodOptionsTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    fn input_type(&self) -> ::std::option::Option<&'_ str>;
    fn output_type(&self) -> ::std::option::Option<&'_ str>;
    fn options(&self) -> ::std::option::Option<&'_ Self::MethodOptionsType>;
    fn client_streaming(&self) -> ::std::option::Option<bool>;
    fn server_streaming(&self) -> ::std::option::Option<bool>;
}

#[derive(Debug)]
pub struct MethodDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub input_type: ::std::option::Option<::std::string::String>,
    pub output_type: ::std::option::Option<::std::string::String>,
    pub options: ::std::option::Option<::std::boxed::Box<self::MethodOptions>>,
    pub client_streaming: ::std::option::Option<bool>,
    pub server_streaming: ::std::option::Option<bool>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
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
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for MethodDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            input_type: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.input_type),
            output_type: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.output_type),
            options: <::std::option::Option<::std::boxed::Box<self::MethodOptions>> as FieldClone>::clone(&self.options),
            client_streaming: <::std::option::Option<bool> as FieldClone>::clone(&self.client_streaming),
            server_streaming: <::std::option::Option<bool> as FieldClone>::clone(&self.server_streaming),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MethodDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.input_type, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.output_type, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::std::boxed::Box<self::MethodOptions>> as FieldDeserFromIter<
                    tags::Message<self::MethodOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.client_streaming, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.server_streaming, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for MethodDescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for MethodDescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for MethodDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.input_type, serializer, 2)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.output_type, serializer, 3)?;
        <::std::option::Option<::std::boxed::Box<self::MethodOptions>> as FieldSer<
                tags::Message<self::MethodOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 4)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.client_streaming, serializer, 5)?;
        <::std::option::Option<bool> as FieldSer<
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

impl MethodDescriptorProtoTrait for MethodDescriptorProto {
    type MethodOptionsType = self::MethodOptions;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn input_type(&self) -> ::std::option::Option<&'_ str> {
        self.input_type.as_deref()
    }
    fn output_type(&self) -> ::std::option::Option<&'_ str> {
        self.output_type.as_deref()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::MethodOptionsType> {
        self.options.as_deref()
    }
    fn client_streaming(&self) -> ::std::option::Option<bool> {
        self.client_streaming.clone()
    }
    fn server_streaming(&self) -> ::std::option::Option<bool> {
        self.server_streaming.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MethodDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct MethodDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub input_type: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub output_type: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::MethodOptionsBumpalo<'bump>>>,
    pub client_streaming: ::std::option::Option<bool>,
    pub server_streaming: ::std::option::Option<bool>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for MethodDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            input_type: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.input_type, self.puroro_internal.bumpalo()),
            output_type: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.output_type, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::MethodOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            client_streaming: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.client_streaming, self.puroro_internal.bumpalo()),
            server_streaming: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.server_streaming, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for MethodDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.input_type, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.output_type, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            4 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::MethodOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::MethodOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::MethodOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            5 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.client_streaming, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.server_streaming, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for MethodDescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for MethodDescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for MethodDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.input_type, serializer, 2)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.output_type, serializer, 3)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::MethodOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::MethodOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 4)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.client_streaming, serializer, 5)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.server_streaming, serializer, 6)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MethodDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodDescriptorProtoTrait for MethodDescriptorProtoBumpalo<'bump> {
    type MethodOptionsType = self::MethodOptionsBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn input_type(&self) -> ::std::option::Option<&'_ str> {
        self.input_type.as_deref()
    }
    fn output_type(&self) -> ::std::option::Option<&'_ str> {
        self.output_type.as_deref()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::MethodOptionsType> {
        self.options.as_deref()
    }
    fn client_streaming(&self) -> ::std::option::Option<bool> {
        self.client_streaming.clone()
    }
    fn server_streaming(&self) -> ::std::option::Option<bool> {
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

#[derive(Debug)]
pub struct MethodDescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    input_type: ::std::option::Option<&'slice str>,
    output_type: ::std::option::Option<&'slice str>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    client_streaming: ::std::option::Option<bool>,
    server_streaming: ::std::option::Option<bool>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> MethodDescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            input_type: ::puroro_internal::helpers::FieldNew::new(),
            output_type: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            client_streaming: ::puroro_internal::helpers::FieldNew::new(),
            server_streaming: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for MethodDescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            input_type: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.input_type),
            output_type: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.output_type),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            client_streaming: <::std::option::Option<bool> as FieldClone>::clone(&self.client_streaming),
            server_streaming: <::std::option::Option<bool> as FieldClone>::clone(&self.server_streaming),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for MethodDescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for MethodDescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for MethodDescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait ServiceDescriptorProtoTrait {
    type MethodDescriptorProtoType: self::MethodDescriptorProtoTrait;
    type ServiceOptionsType: self::ServiceOptionsTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    type MethodRepeated: ::puroro::RepeatedField<Self::MethodDescriptorProtoType>;
    fn method(&self) -> &Self::MethodRepeated;
    fn options(&self) -> ::std::option::Option<&'_ Self::ServiceOptionsType>;
}

#[derive(Debug)]
pub struct ServiceDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub method: ::std::vec::Vec<self::MethodDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::ServiceOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ServiceDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            method: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ServiceDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            method: <::std::vec::Vec<self::MethodDescriptorProto> as FieldClone>::clone(&self.method),
            options: <::std::option::Option<::std::boxed::Box<self::ServiceOptions>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ServiceDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec<self::MethodDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::MethodDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.method, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<self::ServiceOptions>> as FieldDeserFromIter<
                    tags::Message<self::ServiceOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ServiceDescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for ServiceDescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for ServiceDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec<self::MethodDescriptorProto> as FieldSer<
                tags::Message<self::MethodDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.method, serializer, 2)?;
        <::std::option::Option<::std::boxed::Box<self::ServiceOptions>> as FieldSer<
                tags::Message<self::ServiceOptions>, 
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

impl ServiceDescriptorProtoTrait for ServiceDescriptorProto {
    type MethodDescriptorProtoType = self::MethodDescriptorProto;
    type ServiceOptionsType = self::ServiceOptions;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    type MethodRepeated = ::std::vec::Vec<self::MethodDescriptorProto>;
    fn method(&self) -> &Self::MethodRepeated {
        &self.method
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::ServiceOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ServiceDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ServiceDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub method: ::bumpalo::collections::Vec<'bump, self::MethodDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::ServiceOptionsBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            method: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for ServiceDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            method: <::bumpalo::collections::Vec<'bump, self::MethodDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.method, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::ServiceOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ServiceDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::bumpalo::collections::Vec<'bump, self::MethodDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::MethodDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.method, field, || self::MethodDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::ServiceOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::ServiceOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::ServiceOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for ServiceDescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for ServiceDescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for ServiceDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, self::MethodDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::MethodDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.method, serializer, 2)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::ServiceOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::ServiceOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ServiceDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceDescriptorProtoTrait for ServiceDescriptorProtoBumpalo<'bump> {
    type MethodDescriptorProtoType = self::MethodDescriptorProtoBumpalo<'bump>;
    type ServiceOptionsType = self::ServiceOptionsBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    type MethodRepeated = ::bumpalo::collections::Vec<'bump, self::MethodDescriptorProtoBumpalo<'bump>>;
    fn method(&self) -> &Self::MethodRepeated {
        &self.method
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::ServiceOptionsType> {
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

#[derive(Debug)]
pub struct ServiceDescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    method: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> ServiceDescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            method: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for ServiceDescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            method: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.method),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for ServiceDescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for ServiceDescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for ServiceDescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait EnumValueDescriptorProtoTrait {
    type EnumValueOptionsType: self::EnumValueOptionsTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    fn number(&self) -> ::std::option::Option<i32>;
    fn options(&self) -> ::std::option::Option<&'_ Self::EnumValueOptionsType>;
}

#[derive(Debug)]
pub struct EnumValueDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub number: ::std::option::Option<i32>,
    pub options: ::std::option::Option<::std::boxed::Box<self::EnumValueOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumValueDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            number: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumValueDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            number: <::std::option::Option<i32> as FieldClone>::clone(&self.number),
            options: <::std::option::Option<::std::boxed::Box<self::EnumValueOptions>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.number, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<self::EnumValueOptions>> as FieldDeserFromIter<
                    tags::Message<self::EnumValueOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumValueDescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for EnumValueDescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumValueDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.number, serializer, 2)?;
        <::std::option::Option<::std::boxed::Box<self::EnumValueOptions>> as FieldSer<
                tags::Message<self::EnumValueOptions>, 
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

impl EnumValueDescriptorProtoTrait for EnumValueDescriptorProto {
    type EnumValueOptionsType = self::EnumValueOptions;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::EnumValueOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumValueDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumValueDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub number: ::std::option::Option<i32>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumValueOptionsBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            number: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for EnumValueDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            number: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.number, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumValueOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.number, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumValueOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::EnumValueOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::EnumValueOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for EnumValueDescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for EnumValueDescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumValueDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.number, serializer, 2)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumValueOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::EnumValueOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumValueDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoBumpalo<'bump> {
    type EnumValueOptionsType = self::EnumValueOptionsBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::EnumValueOptionsType> {
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

#[derive(Debug)]
pub struct EnumValueDescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    number: ::std::option::Option<i32>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> EnumValueDescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            number: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for EnumValueDescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            number: <::std::option::Option<i32> as FieldClone>::clone(&self.number),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for EnumValueDescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for EnumValueDescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for EnumValueDescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait EnumDescriptorProtoTrait {
    type EnumValueDescriptorProtoType: self::EnumValueDescriptorProtoTrait;
    type EnumOptionsType: self::EnumOptionsTrait;
    type EnumReservedRangeType: self::enum_descriptor_proto::EnumReservedRangeTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    type ValueRepeated: ::puroro::RepeatedField<Self::EnumValueDescriptorProtoType>;
    fn value(&self) -> &Self::ValueRepeated;
    fn options(&self) -> ::std::option::Option<&'_ Self::EnumOptionsType>;
    type ReservedRangeRepeated: ::puroro::RepeatedField<Self::EnumReservedRangeType>;
    fn reserved_range(&self) -> &Self::ReservedRangeRepeated;
    type ReservedNameRepeated: ::puroro::RepeatedField<str>;
    fn reserved_name(&self) -> &Self::ReservedNameRepeated;
}

#[derive(Debug)]
pub struct EnumDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub value: ::std::vec::Vec<self::EnumValueDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::EnumOptions>>,
    pub reserved_range: ::std::vec::Vec<self::enum_descriptor_proto::EnumReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            value: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            reserved_range: ::puroro_internal::helpers::FieldNew::new(),
            reserved_name: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            value: <::std::vec::Vec<self::EnumValueDescriptorProto> as FieldClone>::clone(&self.value),
            options: <::std::option::Option<::std::boxed::Box<self::EnumOptions>> as FieldClone>::clone(&self.options),
            reserved_range: <::std::vec::Vec<self::enum_descriptor_proto::EnumReservedRange> as FieldClone>::clone(&self.reserved_range),
            reserved_name: <::std::vec::Vec<::std::string::String> as FieldClone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec<self::EnumValueDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::EnumValueDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.value, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<self::EnumOptions>> as FieldDeserFromIter<
                    tags::Message<self::EnumOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec<self::enum_descriptor_proto::EnumReservedRange> as FieldDeserFromIter<
                    tags::Message<self::enum_descriptor_proto::EnumReservedRange>, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_range, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_name, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumDescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for EnumDescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec<self::EnumValueDescriptorProto> as FieldSer<
                tags::Message<self::EnumValueDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.value, serializer, 2)?;
        <::std::option::Option<::std::boxed::Box<self::EnumOptions>> as FieldSer<
                tags::Message<self::EnumOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        <::std::vec::Vec<self::enum_descriptor_proto::EnumReservedRange> as FieldSer<
                tags::Message<self::enum_descriptor_proto::EnumReservedRange>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 4)?;
        <::std::vec::Vec<::std::string::String> as FieldSer<
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

impl EnumDescriptorProtoTrait for EnumDescriptorProto {
    type EnumValueDescriptorProtoType = self::EnumValueDescriptorProto;
    type EnumOptionsType = self::EnumOptions;
    type EnumReservedRangeType = self::enum_descriptor_proto::EnumReservedRange;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    type ValueRepeated = ::std::vec::Vec<self::EnumValueDescriptorProto>;
    fn value(&self) -> &Self::ValueRepeated {
        &self.value
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::EnumOptionsType> {
        self.options.as_deref()
    }
    type ReservedRangeRepeated = ::std::vec::Vec<self::enum_descriptor_proto::EnumReservedRange>;
    fn reserved_range(&self) -> &Self::ReservedRangeRepeated {
        &self.reserved_range
    }
    type ReservedNameRepeated = ::std::vec::Vec<::std::string::String>;
    fn reserved_name(&self) -> &Self::ReservedNameRepeated {
        &self.reserved_name
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub value: ::bumpalo::collections::Vec<'bump, self::EnumValueDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumOptionsBumpalo<'bump>>>,
    pub reserved_range: ::bumpalo::collections::Vec<'bump, self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>,
    pub reserved_name: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for EnumDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            value: <::bumpalo::collections::Vec<'bump, self::EnumValueDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.value, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            reserved_range: <::bumpalo::collections::Vec<'bump, self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.reserved_range, self.puroro_internal.bumpalo()),
            reserved_name: <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.reserved_name, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::bumpalo::collections::Vec<'bump, self::EnumValueDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::EnumValueDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.value, field, || self::EnumValueDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::EnumOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::EnumOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            4 => {
                <::bumpalo::collections::Vec<'bump, self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_range, field, || self::enum_descriptor_proto::EnumReservedRangeBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            5 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for EnumDescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for EnumDescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, self::EnumValueDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::EnumValueDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.value, serializer, 2)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::EnumOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::EnumOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        <::bumpalo::collections::Vec<'bump, self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>> as FieldSer<
                tags::Message<self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 4)?;
        <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.reserved_name, serializer, 5)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumDescriptorProtoTrait for EnumDescriptorProtoBumpalo<'bump> {
    type EnumValueDescriptorProtoType = self::EnumValueDescriptorProtoBumpalo<'bump>;
    type EnumOptionsType = self::EnumOptionsBumpalo<'bump>;
    type EnumReservedRangeType = self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    type ValueRepeated = ::bumpalo::collections::Vec<'bump, self::EnumValueDescriptorProtoBumpalo<'bump>>;
    fn value(&self) -> &Self::ValueRepeated {
        &self.value
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::EnumOptionsType> {
        self.options.as_deref()
    }
    type ReservedRangeRepeated = ::bumpalo::collections::Vec<'bump, self::enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>;
    fn reserved_range(&self) -> &Self::ReservedRangeRepeated {
        &self.reserved_range
    }
    type ReservedNameRepeated = ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>;
    fn reserved_name(&self) -> &Self::ReservedNameRepeated {
        &self.reserved_name
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

#[derive(Debug)]
pub struct EnumDescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    value: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    reserved_range: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    reserved_name: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> EnumDescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            value: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            reserved_range: ::puroro_internal::helpers::FieldNew::new(),
            reserved_name: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for EnumDescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            value: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.value),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            reserved_range: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.reserved_range),
            reserved_name: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for EnumDescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for EnumDescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for EnumDescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub mod enum_descriptor_proto {
pub trait EnumReservedRangeTrait {
    fn start(&self) -> ::std::option::Option<i32>;
    fn end(&self) -> ::std::option::Option<i32>;
}

#[derive(Debug)]
pub struct EnumReservedRange {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl EnumReservedRange {
    pub fn new() -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for EnumReservedRange {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for EnumReservedRange {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone(&self.start),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumReservedRange {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for EnumReservedRange {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for EnumReservedRange {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for EnumReservedRange {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
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

impl EnumReservedRangeTrait for EnumReservedRange {
    fn start(&self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumReservedRange {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumReservedRangeBumpalo<'bump> {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumReservedRangeBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for EnumReservedRangeBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.start, self.puroro_internal.bumpalo()),
            end: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.end, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumReservedRangeBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for EnumReservedRangeBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for EnumReservedRangeBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for EnumReservedRangeBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 2)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumReservedRangeBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumReservedRangeTrait for EnumReservedRangeBumpalo<'bump> {
    fn start(&self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
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

#[derive(Debug)]
pub struct EnumReservedRangeSliceView<'slice> {
    start: ::std::option::Option<i32>,
    end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> EnumReservedRangeSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for EnumReservedRangeSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone(&self.start),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for EnumReservedRangeSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for EnumReservedRangeSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for EnumReservedRangeSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
} // mod enum_descriptor_proto
pub trait OneofDescriptorProtoTrait {
    type OneofOptionsType: self::OneofOptionsTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    fn options(&self) -> ::std::option::Option<&'_ Self::OneofOptionsType>;
}

#[derive(Debug)]
pub struct OneofDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub options: ::std::option::Option<::std::boxed::Box<self::OneofOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl OneofDescriptorProto {
    pub fn new() -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for OneofDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            options: <::std::option::Option<::std::boxed::Box<self::OneofOptions>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for OneofDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::boxed::Box<self::OneofOptions>> as FieldDeserFromIter<
                    tags::Message<self::OneofOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for OneofDescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for OneofDescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for OneofDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::std::boxed::Box<self::OneofOptions>> as FieldSer<
                tags::Message<self::OneofOptions>, 
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

impl OneofDescriptorProtoTrait for OneofDescriptorProto {
    type OneofOptionsType = self::OneofOptions;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::OneofOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for OneofDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct OneofDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::OneofOptionsBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofDescriptorProtoBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for OneofDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::OneofOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for OneofDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::OneofOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::OneofOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::OneofOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for OneofDescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for OneofDescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for OneofDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::OneofOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::OneofOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 2)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for OneofDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofDescriptorProtoTrait for OneofDescriptorProtoBumpalo<'bump> {
    type OneofOptionsType = self::OneofOptionsBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::OneofOptionsType> {
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

#[derive(Debug)]
pub struct OneofDescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> OneofDescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            name: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for OneofDescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for OneofDescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for OneofDescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for OneofDescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait FieldDescriptorProtoTrait {
    type FieldOptionsType: self::FieldOptionsTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    fn number(&self) -> ::std::option::Option<i32>;
    fn label(&self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>>;
    fn type_(&self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>>;
    fn type_name(&self) -> ::std::option::Option<&'_ str>;
    fn extendee(&self) -> ::std::option::Option<&'_ str>;
    fn default_value(&self) -> ::std::option::Option<&'_ str>;
    fn oneof_index(&self) -> ::std::option::Option<i32>;
    fn json_name(&self) -> ::std::option::Option<&'_ str>;
    fn options(&self) -> ::std::option::Option<&'_ Self::FieldOptionsType>;
    fn proto3_optional(&self) -> ::std::option::Option<bool>;
}

#[derive(Debug)]
pub struct FieldDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub number: ::std::option::Option<i32>,
    pub label: ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>>,
    pub type_: ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>>,
    pub type_name: ::std::option::Option<::std::string::String>,
    pub extendee: ::std::option::Option<::std::string::String>,
    pub default_value: ::std::option::Option<::std::string::String>,
    pub oneof_index: ::std::option::Option<i32>,
    pub json_name: ::std::option::Option<::std::string::String>,
    pub options: ::std::option::Option<::std::boxed::Box<self::FieldOptions>>,
    pub proto3_optional: ::std::option::Option<bool>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
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
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FieldDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for FieldDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            number: <::std::option::Option<i32> as FieldClone>::clone(&self.number),
            label: <::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> as FieldClone>::clone(&self.label),
            type_: <::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> as FieldClone>::clone(&self.type_),
            type_name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.type_name),
            extendee: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.extendee),
            default_value: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.default_value),
            oneof_index: <::std::option::Option<i32> as FieldClone>::clone(&self.oneof_index),
            json_name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.json_name),
            options: <::std::option::Option<::std::boxed::Box<self::FieldOptions>> as FieldClone>::clone(&self.options),
            proto3_optional: <::std::option::Option<bool> as FieldClone>::clone(&self.proto3_optional),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FieldDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.number, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> as FieldDeserFromIter<
                    tags::Enum<field_descriptor_proto::Label>, 
                    tags::Optional2>>
                ::deser(&mut self.label, field, || 0i32.try_into())?;
            }
            5 => {
                <::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> as FieldDeserFromIter<
                    tags::Enum<field_descriptor_proto::Type>, 
                    tags::Optional2>>
                ::deser(&mut self.type_, field, || 0i32.try_into())?;
            }
            6 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.type_name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.extendee, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.default_value, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.oneof_index, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.json_name, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option<::std::boxed::Box<self::FieldOptions>> as FieldDeserFromIter<
                    tags::Message<self::FieldOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            17 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.proto3_optional, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FieldDescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for FieldDescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for FieldDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.number, serializer, 3)?;
        <::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> as FieldSer<
                tags::Enum<field_descriptor_proto::Label>, 
                tags::Optional2>>
            ::ser(&self.label, serializer, 4)?;
        <::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> as FieldSer<
                tags::Enum<field_descriptor_proto::Type>, 
                tags::Optional2>>
            ::ser(&self.type_, serializer, 5)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.type_name, serializer, 6)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.extendee, serializer, 2)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.default_value, serializer, 7)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.oneof_index, serializer, 9)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.json_name, serializer, 10)?;
        <::std::option::Option<::std::boxed::Box<self::FieldOptions>> as FieldSer<
                tags::Message<self::FieldOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option<bool> as FieldSer<
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

impl FieldDescriptorProtoTrait for FieldDescriptorProto {
    type FieldOptionsType = self::FieldOptions;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn label(&self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> {
        self.label.clone()
    }
    fn type_(&self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> {
        self.type_.clone()
    }
    fn type_name(&self) -> ::std::option::Option<&'_ str> {
        self.type_name.as_deref()
    }
    fn extendee(&self) -> ::std::option::Option<&'_ str> {
        self.extendee.as_deref()
    }
    fn default_value(&self) -> ::std::option::Option<&'_ str> {
        self.default_value.as_deref()
    }
    fn oneof_index(&self) -> ::std::option::Option<i32> {
        self.oneof_index.clone()
    }
    fn json_name(&self) -> ::std::option::Option<&'_ str> {
        self.json_name.as_deref()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::FieldOptionsType> {
        self.options.as_deref()
    }
    fn proto3_optional(&self) -> ::std::option::Option<bool> {
        self.proto3_optional.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FieldDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct FieldDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub number: ::std::option::Option<i32>,
    pub label: ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>>,
    pub type_: ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>>,
    pub type_name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub extendee: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub default_value: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub oneof_index: ::std::option::Option<i32>,
    pub json_name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::FieldOptionsBumpalo<'bump>>>,
    pub proto3_optional: ::std::option::Option<bool>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for FieldDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            number: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.number, self.puroro_internal.bumpalo()),
            label: <::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> as FieldClone>::clone_in_bumpalo(&self.label, self.puroro_internal.bumpalo()),
            type_: <::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> as FieldClone>::clone_in_bumpalo(&self.type_, self.puroro_internal.bumpalo()),
            type_name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.type_name, self.puroro_internal.bumpalo()),
            extendee: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.extendee, self.puroro_internal.bumpalo()),
            default_value: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.default_value, self.puroro_internal.bumpalo()),
            oneof_index: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.oneof_index, self.puroro_internal.bumpalo()),
            json_name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.json_name, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::FieldOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            proto3_optional: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.proto3_optional, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FieldDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.number, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> as FieldDeserFromIter<
                    tags::Enum<field_descriptor_proto::Label>, 
                    tags::Optional2>>
                ::deser(&mut self.label, field, || 0i32.try_into())?;
            }
            5 => {
                <::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> as FieldDeserFromIter<
                    tags::Enum<field_descriptor_proto::Type>, 
                    tags::Optional2>>
                ::deser(&mut self.type_, field, || 0i32.try_into())?;
            }
            6 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.type_name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.extendee, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            7 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.default_value, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            9 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.oneof_index, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.json_name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::FieldOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::FieldOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::FieldOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            17 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.proto3_optional, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for FieldDescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for FieldDescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for FieldDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.number, serializer, 3)?;
        <::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> as FieldSer<
                tags::Enum<field_descriptor_proto::Label>, 
                tags::Optional2>>
            ::ser(&self.label, serializer, 4)?;
        <::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> as FieldSer<
                tags::Enum<field_descriptor_proto::Type>, 
                tags::Optional2>>
            ::ser(&self.type_, serializer, 5)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.type_name, serializer, 6)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.extendee, serializer, 2)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.default_value, serializer, 7)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.oneof_index, serializer, 9)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.json_name, serializer, 10)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::FieldOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::FieldOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.proto3_optional, serializer, 17)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FieldDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldDescriptorProtoTrait for FieldDescriptorProtoBumpalo<'bump> {
    type FieldOptionsType = self::FieldOptionsBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn label(&self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> {
        self.label.clone()
    }
    fn type_(&self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> {
        self.type_.clone()
    }
    fn type_name(&self) -> ::std::option::Option<&'_ str> {
        self.type_name.as_deref()
    }
    fn extendee(&self) -> ::std::option::Option<&'_ str> {
        self.extendee.as_deref()
    }
    fn default_value(&self) -> ::std::option::Option<&'_ str> {
        self.default_value.as_deref()
    }
    fn oneof_index(&self) -> ::std::option::Option<i32> {
        self.oneof_index.clone()
    }
    fn json_name(&self) -> ::std::option::Option<&'_ str> {
        self.json_name.as_deref()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::FieldOptionsType> {
        self.options.as_deref()
    }
    fn proto3_optional(&self) -> ::std::option::Option<bool> {
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

#[derive(Debug)]
pub struct FieldDescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    number: ::std::option::Option<i32>,
    label: ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>>,
    type_: ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>>,
    type_name: ::std::option::Option<&'slice str>,
    extendee: ::std::option::Option<&'slice str>,
    default_value: ::std::option::Option<&'slice str>,
    oneof_index: ::std::option::Option<i32>,
    json_name: ::std::option::Option<&'slice str>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    proto3_optional: ::std::option::Option<bool>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> FieldDescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for FieldDescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            number: <::std::option::Option<i32> as FieldClone>::clone(&self.number),
            label: <::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> as FieldClone>::clone(&self.label),
            type_: <::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> as FieldClone>::clone(&self.type_),
            type_name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.type_name),
            extendee: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.extendee),
            default_value: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.default_value),
            oneof_index: <::std::option::Option<i32> as FieldClone>::clone(&self.oneof_index),
            json_name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.json_name),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            proto3_optional: <::std::option::Option<bool> as FieldClone>::clone(&self.proto3_optional),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for FieldDescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for FieldDescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for FieldDescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
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
impl ::std::convert::From<Label> for i32 {
    fn from(val: Label) -> i32 {
        val as i32
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
impl ::std::convert::From<Type> for i32 {
    fn from(val: Type) -> i32 {
        val as i32
    }
}
} // mod field_descriptor_proto
pub trait ExtensionRangeOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    type UninterpretedOptionRepeated: ::puroro::RepeatedField<Self::UninterpretedOptionType>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated;
}

#[derive(Debug)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: ::std::vec::Vec<self::UninterpretedOption>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ExtensionRangeOptions {
    pub fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ExtensionRangeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::vec::Vec<self::UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRangeOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            999 => {
                <::std::vec::Vec<self::UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ExtensionRangeOptions {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for ExtensionRangeOptions {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for ExtensionRangeOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<self::UninterpretedOption> as FieldSer<
                tags::Message<self::UninterpretedOption>, 
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

impl ExtensionRangeOptionsTrait for ExtensionRangeOptions {
    type UninterpretedOptionType = self::UninterpretedOption;
    type UninterpretedOptionRepeated = ::std::vec::Vec<self::UninterpretedOption>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ExtensionRangeOptions {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ExtensionRangeOptionsBumpalo<'bump> {
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeOptionsBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for ExtensionRangeOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRangeOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            999 => {
                <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || self::UninterpretedOptionBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for ExtensionRangeOptionsBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for ExtensionRangeOptionsBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for ExtensionRangeOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<self::UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ExtensionRangeOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeOptionsTrait for ExtensionRangeOptionsBumpalo<'bump> {
    type UninterpretedOptionType = self::UninterpretedOptionBumpalo<'bump>;
    type UninterpretedOptionRepeated = ::bumpalo::collections::Vec<'bump, self::UninterpretedOptionBumpalo<'bump>>;
    fn uninterpreted_option(&self) -> &Self::UninterpretedOptionRepeated {
        &self.uninterpreted_option
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

#[derive(Debug)]
pub struct ExtensionRangeOptionsSliceView<'slice> {
    uninterpreted_option: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> ExtensionRangeOptionsSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for ExtensionRangeOptionsSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for ExtensionRangeOptionsSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for ExtensionRangeOptionsSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for ExtensionRangeOptionsSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait DescriptorProtoTrait {
    type FieldDescriptorProtoType: self::FieldDescriptorProtoTrait;
    type DescriptorProtoType: self::DescriptorProtoTrait;
    type EnumDescriptorProtoType: self::EnumDescriptorProtoTrait;
    type ExtensionRangeType: self::descriptor_proto::ExtensionRangeTrait;
    type OneofDescriptorProtoType: self::OneofDescriptorProtoTrait;
    type MessageOptionsType: self::MessageOptionsTrait;
    type ReservedRangeType: self::descriptor_proto::ReservedRangeTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    type FieldRepeated: ::puroro::RepeatedField<Self::FieldDescriptorProtoType>;
    fn field(&self) -> &Self::FieldRepeated;
    type ExtensionRepeated: ::puroro::RepeatedField<Self::FieldDescriptorProtoType>;
    fn extension(&self) -> &Self::ExtensionRepeated;
    type NestedTypeRepeated: ::puroro::RepeatedField<Self::DescriptorProtoType>;
    fn nested_type(&self) -> &Self::NestedTypeRepeated;
    type EnumTypeRepeated: ::puroro::RepeatedField<Self::EnumDescriptorProtoType>;
    fn enum_type(&self) -> &Self::EnumTypeRepeated;
    type ExtensionRangeRepeated: ::puroro::RepeatedField<Self::ExtensionRangeType>;
    fn extension_range(&self) -> &Self::ExtensionRangeRepeated;
    type OneofDeclRepeated: ::puroro::RepeatedField<Self::OneofDescriptorProtoType>;
    fn oneof_decl(&self) -> &Self::OneofDeclRepeated;
    fn options(&self) -> ::std::option::Option<&'_ Self::MessageOptionsType>;
    type ReservedRangeRepeated: ::puroro::RepeatedField<Self::ReservedRangeType>;
    fn reserved_range(&self) -> &Self::ReservedRangeRepeated;
    type ReservedNameRepeated: ::puroro::RepeatedField<str>;
    fn reserved_name(&self) -> &Self::ReservedNameRepeated;
}

#[derive(Debug)]
pub struct DescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub field: ::std::vec::Vec<self::FieldDescriptorProto>,
    pub extension: ::std::vec::Vec<self::FieldDescriptorProto>,
    pub nested_type: ::std::vec::Vec<self::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<self::EnumDescriptorProto>,
    pub extension_range: ::std::vec::Vec<self::descriptor_proto::ExtensionRange>,
    pub oneof_decl: ::std::vec::Vec<self::OneofDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::MessageOptions>>,
    pub reserved_range: ::std::vec::Vec<self::descriptor_proto::ReservedRange>,
    pub reserved_name: ::std::vec::Vec<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
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
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for DescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            field: <::std::vec::Vec<self::FieldDescriptorProto> as FieldClone>::clone(&self.field),
            extension: <::std::vec::Vec<self::FieldDescriptorProto> as FieldClone>::clone(&self.extension),
            nested_type: <::std::vec::Vec<self::DescriptorProto> as FieldClone>::clone(&self.nested_type),
            enum_type: <::std::vec::Vec<self::EnumDescriptorProto> as FieldClone>::clone(&self.enum_type),
            extension_range: <::std::vec::Vec<self::descriptor_proto::ExtensionRange> as FieldClone>::clone(&self.extension_range),
            oneof_decl: <::std::vec::Vec<self::OneofDescriptorProto> as FieldClone>::clone(&self.oneof_decl),
            options: <::std::option::Option<::std::boxed::Box<self::MessageOptions>> as FieldClone>::clone(&self.options),
            reserved_range: <::std::vec::Vec<self::descriptor_proto::ReservedRange> as FieldClone>::clone(&self.reserved_range),
            reserved_name: <::std::vec::Vec<::std::string::String> as FieldClone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for DescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec<self::FieldDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::FieldDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.field, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec<self::FieldDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::FieldDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::vec::Vec<self::DescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::DescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.nested_type, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec<self::EnumDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::EnumDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec<self::descriptor_proto::ExtensionRange> as FieldDeserFromIter<
                    tags::Message<self::descriptor_proto::ExtensionRange>, 
                    tags::Repeated>>
                ::deser(&mut self.extension_range, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::vec::Vec<self::OneofDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::OneofDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.oneof_decl, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option<::std::boxed::Box<self::MessageOptions>> as FieldDeserFromIter<
                    tags::Message<self::MessageOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::vec::Vec<self::descriptor_proto::ReservedRange> as FieldDeserFromIter<
                    tags::Message<self::descriptor_proto::ReservedRange>, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_range, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::vec::Vec<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_name, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for DescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for DescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for DescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec<self::FieldDescriptorProto> as FieldSer<
                tags::Message<self::FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.field, serializer, 2)?;
        <::std::vec::Vec<self::FieldDescriptorProto> as FieldSer<
                tags::Message<self::FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 6)?;
        <::std::vec::Vec<self::DescriptorProto> as FieldSer<
                tags::Message<self::DescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.nested_type, serializer, 3)?;
        <::std::vec::Vec<self::EnumDescriptorProto> as FieldSer<
                tags::Message<self::EnumDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 4)?;
        <::std::vec::Vec<self::descriptor_proto::ExtensionRange> as FieldSer<
                tags::Message<self::descriptor_proto::ExtensionRange>, 
                tags::Repeated>>
            ::ser(&self.extension_range, serializer, 5)?;
        <::std::vec::Vec<self::OneofDescriptorProto> as FieldSer<
                tags::Message<self::OneofDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.oneof_decl, serializer, 8)?;
        <::std::option::Option<::std::boxed::Box<self::MessageOptions>> as FieldSer<
                tags::Message<self::MessageOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 7)?;
        <::std::vec::Vec<self::descriptor_proto::ReservedRange> as FieldSer<
                tags::Message<self::descriptor_proto::ReservedRange>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 9)?;
        <::std::vec::Vec<::std::string::String> as FieldSer<
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

impl DescriptorProtoTrait for DescriptorProto {
    type FieldDescriptorProtoType = self::FieldDescriptorProto;
    type DescriptorProtoType = self::DescriptorProto;
    type EnumDescriptorProtoType = self::EnumDescriptorProto;
    type ExtensionRangeType = self::descriptor_proto::ExtensionRange;
    type OneofDescriptorProtoType = self::OneofDescriptorProto;
    type MessageOptionsType = self::MessageOptions;
    type ReservedRangeType = self::descriptor_proto::ReservedRange;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    type FieldRepeated = ::std::vec::Vec<self::FieldDescriptorProto>;
    fn field(&self) -> &Self::FieldRepeated {
        &self.field
    }
    type ExtensionRepeated = ::std::vec::Vec<self::FieldDescriptorProto>;
    fn extension(&self) -> &Self::ExtensionRepeated {
        &self.extension
    }
    type NestedTypeRepeated = ::std::vec::Vec<self::DescriptorProto>;
    fn nested_type(&self) -> &Self::NestedTypeRepeated {
        &self.nested_type
    }
    type EnumTypeRepeated = ::std::vec::Vec<self::EnumDescriptorProto>;
    fn enum_type(&self) -> &Self::EnumTypeRepeated {
        &self.enum_type
    }
    type ExtensionRangeRepeated = ::std::vec::Vec<self::descriptor_proto::ExtensionRange>;
    fn extension_range(&self) -> &Self::ExtensionRangeRepeated {
        &self.extension_range
    }
    type OneofDeclRepeated = ::std::vec::Vec<self::OneofDescriptorProto>;
    fn oneof_decl(&self) -> &Self::OneofDeclRepeated {
        &self.oneof_decl
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::MessageOptionsType> {
        self.options.as_deref()
    }
    type ReservedRangeRepeated = ::std::vec::Vec<self::descriptor_proto::ReservedRange>;
    fn reserved_range(&self) -> &Self::ReservedRangeRepeated {
        &self.reserved_range
    }
    type ReservedNameRepeated = ::std::vec::Vec<::std::string::String>;
    fn reserved_name(&self) -> &Self::ReservedNameRepeated {
        &self.reserved_name
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for DescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct DescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub field: ::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>>,
    pub extension: ::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>>,
    pub nested_type: ::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>>,
    pub enum_type: ::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>>,
    pub extension_range: ::bumpalo::collections::Vec<'bump, self::descriptor_proto::ExtensionRangeBumpalo<'bump>>,
    pub oneof_decl: ::bumpalo::collections::Vec<'bump, self::OneofDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::MessageOptionsBumpalo<'bump>>>,
    pub reserved_range: ::bumpalo::collections::Vec<'bump, self::descriptor_proto::ReservedRangeBumpalo<'bump>>,
    pub reserved_name: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for DescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            field: <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.field, self.puroro_internal.bumpalo()),
            extension: <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.extension, self.puroro_internal.bumpalo()),
            nested_type: <::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.nested_type, self.puroro_internal.bumpalo()),
            enum_type: <::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.enum_type, self.puroro_internal.bumpalo()),
            extension_range: <::bumpalo::collections::Vec<'bump, self::descriptor_proto::ExtensionRangeBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.extension_range, self.puroro_internal.bumpalo()),
            oneof_decl: <::bumpalo::collections::Vec<'bump, self::OneofDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.oneof_decl, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::MessageOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            reserved_range: <::bumpalo::collections::Vec<'bump, self::descriptor_proto::ReservedRangeBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.reserved_range, self.puroro_internal.bumpalo()),
            reserved_name: <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.reserved_name, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for DescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::FieldDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.field, field, || self::FieldDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            6 => {
                <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::FieldDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, || self::FieldDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::DescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.nested_type, field, || self::DescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            4 => {
                <::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::EnumDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, || self::EnumDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            5 => {
                <::bumpalo::collections::Vec<'bump, self::descriptor_proto::ExtensionRangeBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::descriptor_proto::ExtensionRangeBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.extension_range, field, || self::descriptor_proto::ExtensionRangeBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            8 => {
                <::bumpalo::collections::Vec<'bump, self::OneofDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::OneofDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.oneof_decl, field, || self::OneofDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            7 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::MessageOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::MessageOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::MessageOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            9 => {
                <::bumpalo::collections::Vec<'bump, self::descriptor_proto::ReservedRangeBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::descriptor_proto::ReservedRangeBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_range, field, || self::descriptor_proto::ReservedRangeBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            10 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for DescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for DescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for DescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::FieldDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.field, serializer, 2)?;
        <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::FieldDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 6)?;
        <::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::DescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.nested_type, serializer, 3)?;
        <::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::EnumDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 4)?;
        <::bumpalo::collections::Vec<'bump, self::descriptor_proto::ExtensionRangeBumpalo<'bump>> as FieldSer<
                tags::Message<self::descriptor_proto::ExtensionRangeBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension_range, serializer, 5)?;
        <::bumpalo::collections::Vec<'bump, self::OneofDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::OneofDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.oneof_decl, serializer, 8)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::MessageOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::MessageOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 7)?;
        <::bumpalo::collections::Vec<'bump, self::descriptor_proto::ReservedRangeBumpalo<'bump>> as FieldSer<
                tags::Message<self::descriptor_proto::ReservedRangeBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.reserved_range, serializer, 9)?;
        <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.reserved_name, serializer, 10)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for DescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> DescriptorProtoTrait for DescriptorProtoBumpalo<'bump> {
    type FieldDescriptorProtoType = self::FieldDescriptorProtoBumpalo<'bump>;
    type DescriptorProtoType = self::DescriptorProtoBumpalo<'bump>;
    type EnumDescriptorProtoType = self::EnumDescriptorProtoBumpalo<'bump>;
    type ExtensionRangeType = self::descriptor_proto::ExtensionRangeBumpalo<'bump>;
    type OneofDescriptorProtoType = self::OneofDescriptorProtoBumpalo<'bump>;
    type MessageOptionsType = self::MessageOptionsBumpalo<'bump>;
    type ReservedRangeType = self::descriptor_proto::ReservedRangeBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    type FieldRepeated = ::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>>;
    fn field(&self) -> &Self::FieldRepeated {
        &self.field
    }
    type ExtensionRepeated = ::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>>;
    fn extension(&self) -> &Self::ExtensionRepeated {
        &self.extension
    }
    type NestedTypeRepeated = ::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>>;
    fn nested_type(&self) -> &Self::NestedTypeRepeated {
        &self.nested_type
    }
    type EnumTypeRepeated = ::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>>;
    fn enum_type(&self) -> &Self::EnumTypeRepeated {
        &self.enum_type
    }
    type ExtensionRangeRepeated = ::bumpalo::collections::Vec<'bump, self::descriptor_proto::ExtensionRangeBumpalo<'bump>>;
    fn extension_range(&self) -> &Self::ExtensionRangeRepeated {
        &self.extension_range
    }
    type OneofDeclRepeated = ::bumpalo::collections::Vec<'bump, self::OneofDescriptorProtoBumpalo<'bump>>;
    fn oneof_decl(&self) -> &Self::OneofDeclRepeated {
        &self.oneof_decl
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::MessageOptionsType> {
        self.options.as_deref()
    }
    type ReservedRangeRepeated = ::bumpalo::collections::Vec<'bump, self::descriptor_proto::ReservedRangeBumpalo<'bump>>;
    fn reserved_range(&self) -> &Self::ReservedRangeRepeated {
        &self.reserved_range
    }
    type ReservedNameRepeated = ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>;
    fn reserved_name(&self) -> &Self::ReservedNameRepeated {
        &self.reserved_name
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

#[derive(Debug)]
pub struct DescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    field: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    extension: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    nested_type: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    enum_type: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    extension_range: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    oneof_decl: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    reserved_range: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    reserved_name: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> DescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for DescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            field: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.field),
            extension: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.extension),
            nested_type: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.nested_type),
            enum_type: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.enum_type),
            extension_range: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.extension_range),
            oneof_decl: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.oneof_decl),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            reserved_range: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.reserved_range),
            reserved_name: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for DescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for DescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for DescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub mod descriptor_proto {
pub trait ReservedRangeTrait {
    fn start(&self) -> ::std::option::Option<i32>;
    fn end(&self) -> ::std::option::Option<i32>;
}

#[derive(Debug)]
pub struct ReservedRange {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ReservedRange {
    pub fn new() -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ReservedRange {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for ReservedRange {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone(&self.start),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ReservedRange {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ReservedRange {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for ReservedRange {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for ReservedRange {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
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

impl ReservedRangeTrait for ReservedRange {
    fn start(&self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ReservedRange {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ReservedRangeBumpalo<'bump> {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ReservedRangeBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for ReservedRangeBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.start, self.puroro_internal.bumpalo()),
            end: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.end, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ReservedRangeBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for ReservedRangeBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for ReservedRangeBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for ReservedRangeBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 2)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ReservedRangeBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ReservedRangeTrait for ReservedRangeBumpalo<'bump> {
    fn start(&self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
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

#[derive(Debug)]
pub struct ReservedRangeSliceView<'slice> {
    start: ::std::option::Option<i32>,
    end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> ReservedRangeSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for ReservedRangeSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone(&self.start),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for ReservedRangeSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for ReservedRangeSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for ReservedRangeSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait ExtensionRangeTrait {
    type ExtensionRangeOptionsType: super::ExtensionRangeOptionsTrait;
    fn start(&self) -> ::std::option::Option<i32>;
    fn end(&self) -> ::std::option::Option<i32>;
    fn options(&self) -> ::std::option::Option<&'_ Self::ExtensionRangeOptionsType>;
}

#[derive(Debug)]
pub struct ExtensionRange {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    pub options: ::std::option::Option<::std::boxed::Box<super::ExtensionRangeOptions>>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl ExtensionRange {
    pub fn new() -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for ExtensionRange {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for ExtensionRange {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone(&self.start),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            options: <::std::option::Option<::std::boxed::Box<super::ExtensionRangeOptions>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRange {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<super::ExtensionRangeOptions>> as FieldDeserFromIter<
                    tags::Message<super::ExtensionRangeOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for ExtensionRange {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for ExtensionRange {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for ExtensionRange {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 2)?;
        <::std::option::Option<::std::boxed::Box<super::ExtensionRangeOptions>> as FieldSer<
                tags::Message<super::ExtensionRangeOptions>, 
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

impl ExtensionRangeTrait for ExtensionRange {
    type ExtensionRangeOptionsType = super::ExtensionRangeOptions;
    fn start(&self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::ExtensionRangeOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ExtensionRange {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ExtensionRangeBumpalo<'bump> {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, super::ExtensionRangeOptionsBumpalo<'bump>>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for ExtensionRangeBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.start, self.puroro_internal.bumpalo()),
            end: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.end, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, super::ExtensionRangeOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRangeBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.start, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.end, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, super::ExtensionRangeOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<super::ExtensionRangeOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(super::ExtensionRangeOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for ExtensionRangeBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for ExtensionRangeBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for ExtensionRangeBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.start, serializer, 1)?;
        <::std::option::Option<i32> as FieldSer<
                tags::Int32, 
                tags::Optional2>>
            ::ser(&self.end, serializer, 2)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, super::ExtensionRangeOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<super::ExtensionRangeOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ExtensionRangeBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeTrait for ExtensionRangeBumpalo<'bump> {
    type ExtensionRangeOptionsType = super::ExtensionRangeOptionsBumpalo<'bump>;
    fn start(&self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::ExtensionRangeOptionsType> {
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

#[derive(Debug)]
pub struct ExtensionRangeSliceView<'slice> {
    start: ::std::option::Option<i32>,
    end: ::std::option::Option<i32>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> ExtensionRangeSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new(),
            end: ::puroro_internal::helpers::FieldNew::new(),
            options: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for ExtensionRangeSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option<i32> as FieldClone>::clone(&self.start),
            end: <::std::option::Option<i32> as FieldClone>::clone(&self.end),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for ExtensionRangeSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for ExtensionRangeSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for ExtensionRangeSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
} // mod descriptor_proto
pub trait FileDescriptorProtoTrait {
    type DescriptorProtoType: self::DescriptorProtoTrait;
    type EnumDescriptorProtoType: self::EnumDescriptorProtoTrait;
    type ServiceDescriptorProtoType: self::ServiceDescriptorProtoTrait;
    type FieldDescriptorProtoType: self::FieldDescriptorProtoTrait;
    type FileOptionsType: self::FileOptionsTrait;
    type SourceCodeInfoType: self::SourceCodeInfoTrait;
    fn name(&self) -> ::std::option::Option<&'_ str>;
    fn package(&self) -> ::std::option::Option<&'_ str>;
    type DependencyRepeated: ::puroro::RepeatedField<str>;
    fn dependency(&self) -> &Self::DependencyRepeated;
    type PublicDependencyRepeated: ::puroro::RepeatedField<i32>;
    fn public_dependency(&self) -> &Self::PublicDependencyRepeated;
    type WeakDependencyRepeated: ::puroro::RepeatedField<i32>;
    fn weak_dependency(&self) -> &Self::WeakDependencyRepeated;
    type MessageTypeRepeated: ::puroro::RepeatedField<Self::DescriptorProtoType>;
    fn message_type(&self) -> &Self::MessageTypeRepeated;
    type EnumTypeRepeated: ::puroro::RepeatedField<Self::EnumDescriptorProtoType>;
    fn enum_type(&self) -> &Self::EnumTypeRepeated;
    type ServiceRepeated: ::puroro::RepeatedField<Self::ServiceDescriptorProtoType>;
    fn service(&self) -> &Self::ServiceRepeated;
    type ExtensionRepeated: ::puroro::RepeatedField<Self::FieldDescriptorProtoType>;
    fn extension(&self) -> &Self::ExtensionRepeated;
    fn options(&self) -> ::std::option::Option<&'_ Self::FileOptionsType>;
    fn source_code_info(&self) -> ::std::option::Option<&'_ Self::SourceCodeInfoType>;
    fn syntax(&self) -> ::std::option::Option<&'_ str>;
}

#[derive(Debug)]
pub struct FileDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub package: ::std::option::Option<::std::string::String>,
    pub dependency: ::std::vec::Vec<::std::string::String>,
    pub public_dependency: ::std::vec::Vec<i32>,
    pub weak_dependency: ::std::vec::Vec<i32>,
    pub message_type: ::std::vec::Vec<self::DescriptorProto>,
    pub enum_type: ::std::vec::Vec<self::EnumDescriptorProto>,
    pub service: ::std::vec::Vec<self::ServiceDescriptorProto>,
    pub extension: ::std::vec::Vec<self::FieldDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<self::FileOptions>>,
    pub source_code_info: ::std::option::Option<::std::boxed::Box<self::SourceCodeInfo>>,
    pub syntax: ::std::option::Option<::std::string::String>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
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
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileDescriptorProto {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for FileDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            package: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.package),
            dependency: <::std::vec::Vec<::std::string::String> as FieldClone>::clone(&self.dependency),
            public_dependency: <::std::vec::Vec<i32> as FieldClone>::clone(&self.public_dependency),
            weak_dependency: <::std::vec::Vec<i32> as FieldClone>::clone(&self.weak_dependency),
            message_type: <::std::vec::Vec<self::DescriptorProto> as FieldClone>::clone(&self.message_type),
            enum_type: <::std::vec::Vec<self::EnumDescriptorProto> as FieldClone>::clone(&self.enum_type),
            service: <::std::vec::Vec<self::ServiceDescriptorProto> as FieldClone>::clone(&self.service),
            extension: <::std::vec::Vec<self::FieldDescriptorProto> as FieldClone>::clone(&self.extension),
            options: <::std::option::Option<::std::boxed::Box<self::FileOptions>> as FieldClone>::clone(&self.options),
            source_code_info: <::std::option::Option<::std::boxed::Box<self::SourceCodeInfo>> as FieldClone>::clone(&self.source_code_info),
            syntax: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.syntax),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.package, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::vec::Vec<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.dependency, field, ::std::default::Default::default)?;
            }
            10 => {
                <::std::vec::Vec<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.public_dependency, field, ::std::default::Default::default)?;
            }
            11 => {
                <::std::vec::Vec<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.weak_dependency, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec<self::DescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::DescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.message_type, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec<self::EnumDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::EnumDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec<self::ServiceDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::ServiceDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.service, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::vec::Vec<self::FieldDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::FieldDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option<::std::boxed::Box<self::FileOptions>> as FieldDeserFromIter<
                    tags::Message<self::FileOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option<::std::boxed::Box<self::SourceCodeInfo>> as FieldDeserFromIter<
                    tags::Message<self::SourceCodeInfo>, 
                    tags::Optional2>>
                ::deser(&mut self.source_code_info, field, ::std::default::Default::default)?;
            }
            12 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.syntax, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FileDescriptorProto {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for FileDescriptorProto {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for FileDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.package, serializer, 2)?;
        <::std::vec::Vec<::std::string::String> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.dependency, serializer, 3)?;
        <::std::vec::Vec<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.public_dependency, serializer, 10)?;
        <::std::vec::Vec<i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.weak_dependency, serializer, 11)?;
        <::std::vec::Vec<self::DescriptorProto> as FieldSer<
                tags::Message<self::DescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.message_type, serializer, 4)?;
        <::std::vec::Vec<self::EnumDescriptorProto> as FieldSer<
                tags::Message<self::EnumDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 5)?;
        <::std::vec::Vec<self::ServiceDescriptorProto> as FieldSer<
                tags::Message<self::ServiceDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.service, serializer, 6)?;
        <::std::vec::Vec<self::FieldDescriptorProto> as FieldSer<
                tags::Message<self::FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 7)?;
        <::std::option::Option<::std::boxed::Box<self::FileOptions>> as FieldSer<
                tags::Message<self::FileOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option<::std::boxed::Box<self::SourceCodeInfo>> as FieldSer<
                tags::Message<self::SourceCodeInfo>, 
                tags::Optional2>>
            ::ser(&self.source_code_info, serializer, 9)?;
        <::std::option::Option<::std::string::String> as FieldSer<
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

impl FileDescriptorProtoTrait for FileDescriptorProto {
    type DescriptorProtoType = self::DescriptorProto;
    type EnumDescriptorProtoType = self::EnumDescriptorProto;
    type ServiceDescriptorProtoType = self::ServiceDescriptorProto;
    type FieldDescriptorProtoType = self::FieldDescriptorProto;
    type FileOptionsType = self::FileOptions;
    type SourceCodeInfoType = self::SourceCodeInfo;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn package(&self) -> ::std::option::Option<&'_ str> {
        self.package.as_deref()
    }
    type DependencyRepeated = ::std::vec::Vec<::std::string::String>;
    fn dependency(&self) -> &Self::DependencyRepeated {
        &self.dependency
    }
    type PublicDependencyRepeated = ::std::vec::Vec<i32>;
    fn public_dependency(&self) -> &Self::PublicDependencyRepeated {
        &self.public_dependency
    }
    type WeakDependencyRepeated = ::std::vec::Vec<i32>;
    fn weak_dependency(&self) -> &Self::WeakDependencyRepeated {
        &self.weak_dependency
    }
    type MessageTypeRepeated = ::std::vec::Vec<self::DescriptorProto>;
    fn message_type(&self) -> &Self::MessageTypeRepeated {
        &self.message_type
    }
    type EnumTypeRepeated = ::std::vec::Vec<self::EnumDescriptorProto>;
    fn enum_type(&self) -> &Self::EnumTypeRepeated {
        &self.enum_type
    }
    type ServiceRepeated = ::std::vec::Vec<self::ServiceDescriptorProto>;
    fn service(&self) -> &Self::ServiceRepeated {
        &self.service
    }
    type ExtensionRepeated = ::std::vec::Vec<self::FieldDescriptorProto>;
    fn extension(&self) -> &Self::ExtensionRepeated {
        &self.extension
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::FileOptionsType> {
        self.options.as_deref()
    }
    fn source_code_info(&self) -> ::std::option::Option<&'_ Self::SourceCodeInfoType> {
        self.source_code_info.as_deref()
    }
    fn syntax(&self) -> ::std::option::Option<&'_ str> {
        self.syntax.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileDescriptorProto {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct FileDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub package: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub dependency: ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>,
    pub public_dependency: ::bumpalo::collections::Vec<'bump, i32>,
    pub weak_dependency: ::bumpalo::collections::Vec<'bump, i32>,
    pub message_type: ::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>>,
    pub enum_type: ::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>>,
    pub service: ::bumpalo::collections::Vec<'bump, self::ServiceDescriptorProtoBumpalo<'bump>>,
    pub extension: ::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::FileOptionsBumpalo<'bump>>>,
    pub source_code_info: ::std::option::Option<::bumpalo::boxed::Box<'bump, self::SourceCodeInfoBumpalo<'bump>>>,
    pub syntax: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for FileDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            package: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.package, self.puroro_internal.bumpalo()),
            dependency: <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.dependency, self.puroro_internal.bumpalo()),
            public_dependency: <::bumpalo::collections::Vec<'bump, i32> as FieldClone>::clone_in_bumpalo(&self.public_dependency, self.puroro_internal.bumpalo()),
            weak_dependency: <::bumpalo::collections::Vec<'bump, i32> as FieldClone>::clone_in_bumpalo(&self.weak_dependency, self.puroro_internal.bumpalo()),
            message_type: <::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.message_type, self.puroro_internal.bumpalo()),
            enum_type: <::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.enum_type, self.puroro_internal.bumpalo()),
            service: <::bumpalo::collections::Vec<'bump, self::ServiceDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.service, self.puroro_internal.bumpalo()),
            extension: <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.extension, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::FileOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            source_code_info: <::std::option::Option<::bumpalo::boxed::Box<'bump, self::SourceCodeInfoBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.source_code_info, self.puroro_internal.bumpalo()),
            syntax: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.syntax, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.package, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            3 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.dependency, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            10 => {
                <::bumpalo::collections::Vec<'bump, i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.public_dependency, field, ::std::default::Default::default)?;
            }
            11 => {
                <::bumpalo::collections::Vec<'bump, i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Repeated>>
                ::deser(&mut self.weak_dependency, field, ::std::default::Default::default)?;
            }
            4 => {
                <::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::DescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.message_type, field, || self::DescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            5 => {
                <::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::EnumDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, || self::EnumDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            6 => {
                <::bumpalo::collections::Vec<'bump, self::ServiceDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::ServiceDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.service, field, || self::ServiceDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            7 => {
                <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::FieldDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, || self::FieldDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::FileOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::FileOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(self::FileOptionsBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            9 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, self::SourceCodeInfoBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<self::SourceCodeInfoBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.source_code_info, field, || ::bumpalo::boxed::Box::new_in(self::SourceCodeInfoBumpalo::new_in(puroro_internal.bumpalo()), puroro_internal.bumpalo()))?;
            }
            12 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.syntax, field, || ::bumpalo::collections::String::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for FileDescriptorProtoBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for FileDescriptorProtoBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for FileDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.package, serializer, 2)?;
        <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Repeated>>
            ::ser(&self.dependency, serializer, 3)?;
        <::bumpalo::collections::Vec<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.public_dependency, serializer, 10)?;
        <::bumpalo::collections::Vec<'bump, i32> as FieldSer<
                tags::Int32, 
                tags::Repeated>>
            ::ser(&self.weak_dependency, serializer, 11)?;
        <::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::DescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.message_type, serializer, 4)?;
        <::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::EnumDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 5)?;
        <::bumpalo::collections::Vec<'bump, self::ServiceDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::ServiceDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.service, serializer, 6)?;
        <::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::FieldDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 7)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::FileOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<self::FileOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, self::SourceCodeInfoBumpalo<'bump>>> as FieldSer<
                tags::Message<self::SourceCodeInfoBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.source_code_info, serializer, 9)?;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.syntax, serializer, 12)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorProtoTrait for FileDescriptorProtoBumpalo<'bump> {
    type DescriptorProtoType = self::DescriptorProtoBumpalo<'bump>;
    type EnumDescriptorProtoType = self::EnumDescriptorProtoBumpalo<'bump>;
    type ServiceDescriptorProtoType = self::ServiceDescriptorProtoBumpalo<'bump>;
    type FieldDescriptorProtoType = self::FieldDescriptorProtoBumpalo<'bump>;
    type FileOptionsType = self::FileOptionsBumpalo<'bump>;
    type SourceCodeInfoType = self::SourceCodeInfoBumpalo<'bump>;
    fn name(&self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn package(&self) -> ::std::option::Option<&'_ str> {
        self.package.as_deref()
    }
    type DependencyRepeated = ::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>>;
    fn dependency(&self) -> &Self::DependencyRepeated {
        &self.dependency
    }
    type PublicDependencyRepeated = ::bumpalo::collections::Vec<'bump, i32>;
    fn public_dependency(&self) -> &Self::PublicDependencyRepeated {
        &self.public_dependency
    }
    type WeakDependencyRepeated = ::bumpalo::collections::Vec<'bump, i32>;
    fn weak_dependency(&self) -> &Self::WeakDependencyRepeated {
        &self.weak_dependency
    }
    type MessageTypeRepeated = ::bumpalo::collections::Vec<'bump, self::DescriptorProtoBumpalo<'bump>>;
    fn message_type(&self) -> &Self::MessageTypeRepeated {
        &self.message_type
    }
    type EnumTypeRepeated = ::bumpalo::collections::Vec<'bump, self::EnumDescriptorProtoBumpalo<'bump>>;
    fn enum_type(&self) -> &Self::EnumTypeRepeated {
        &self.enum_type
    }
    type ServiceRepeated = ::bumpalo::collections::Vec<'bump, self::ServiceDescriptorProtoBumpalo<'bump>>;
    fn service(&self) -> &Self::ServiceRepeated {
        &self.service
    }
    type ExtensionRepeated = ::bumpalo::collections::Vec<'bump, self::FieldDescriptorProtoBumpalo<'bump>>;
    fn extension(&self) -> &Self::ExtensionRepeated {
        &self.extension
    }
    fn options(&self) -> ::std::option::Option<&'_ Self::FileOptionsType> {
        self.options.as_deref()
    }
    fn source_code_info(&self) -> ::std::option::Option<&'_ Self::SourceCodeInfoType> {
        self.source_code_info.as_deref()
    }
    fn syntax(&self) -> ::std::option::Option<&'_ str> {
        self.syntax.as_deref()
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

#[derive(Debug)]
pub struct FileDescriptorProtoSliceView<'slice> {
    name: ::std::option::Option<&'slice str>,
    package: ::std::option::Option<&'slice str>,
    dependency: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    public_dependency: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    weak_dependency: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    message_type: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    enum_type: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    service: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    extension: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    options: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    source_code_info: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    syntax: ::std::option::Option<&'slice str>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> FileDescriptorProtoSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for FileDescriptorProtoSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.name),
            package: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.package),
            dependency: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.dependency),
            public_dependency: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.public_dependency),
            weak_dependency: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.weak_dependency),
            message_type: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.message_type),
            enum_type: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.enum_type),
            service: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.service),
            extension: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.extension),
            options: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.options),
            source_code_info: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.source_code_info),
            syntax: <::std::option::Option<&'slice str> as FieldClone>::clone(&self.syntax),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for FileDescriptorProtoSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for FileDescriptorProtoSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for FileDescriptorProtoSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
pub trait FileDescriptorSetTrait {
    type FileDescriptorProtoType: self::FileDescriptorProtoTrait;
    type FileRepeated: ::puroro::RepeatedField<Self::FileDescriptorProtoType>;
    fn file(&self) -> &Self::FileRepeated;
}

#[derive(Debug)]
pub struct FileDescriptorSet {
    pub file: ::std::vec::Vec<self::FileDescriptorProto>,
    puroro_internal: ::puroro_internal::InternalDataForNormalStruct,
}

impl FileDescriptorSet {
    pub fn new() -> Self {
        Self {
            file: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForNormalStruct::new(),
        }
    }
}

impl ::std::default::Default for FileDescriptorSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file: <::std::vec::Vec<self::FileDescriptorProto> as FieldClone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorSet {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::std::vec::Vec<self::FileDescriptorProto> as FieldDeserFromIter<
                    tags::Message<self::FileDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.file, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl ::puroro::DeserializableFromIter for FileDescriptorSet {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}

impl ::puroro::DeserializableFromSlice for FileDescriptorSet {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}

impl ::puroro_internal::ser::SerializableMessage for FileDescriptorSet {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<self::FileDescriptorProto> as FieldSer<
                tags::Message<self::FileDescriptorProto>, 
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

impl FileDescriptorSetTrait for FileDescriptorSet {
    type FileDescriptorProtoType = self::FileDescriptorProto;
    type FileRepeated = ::std::vec::Vec<self::FileDescriptorProto>;
    fn file(&self) -> &Self::FileRepeated {
        &self.file
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileDescriptorSet {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct FileDescriptorSetBumpalo<'bump> {
    pub file: ::bumpalo::collections::Vec<'bump, self::FileDescriptorProtoBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorSetBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            file: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::InternalDataForBumpaloStruct::new(bump),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::std::clone::Clone for FileDescriptorSetBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file: <::bumpalo::collections::Vec<'bump, self::FileDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.file, self.puroro_internal.bumpalo()),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorSetBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::LdIter<I>>,
        field_number: usize,
    ) -> ::puroro::Result<bool> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        let puroro_internal = &self.puroro_internal;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, self::FileDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<self::FileDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.file, field, || self::FileDescriptorProtoBumpalo::new_in(puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromIter for FileDescriptorSetBumpalo<'bump> {
    fn deser_from_iter<I>(&mut self, iter: &mut I) -> ::puroro::Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>> 
    {
        <Self as ::puroro_internal::deser::DeserializableMessageFromIter>
            ::deser_from_iter(self, iter)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::DeserializableFromSlice for FileDescriptorSetBumpalo<'bump> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> ::puroro::Result<()> {
        let mut from_slice = ::puroro_internal::deser::FromIterToFromSlice::new(self);
        let mut wrapped_slice = ::puroro_internal::deser::LdSlice::new(slice);
        wrapped_slice.deser_message(&mut from_slice)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::ser::SerializableMessage for FileDescriptorSetBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, self::FileDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<self::FileDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 1)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileDescriptorSetBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorSetTrait for FileDescriptorSetBumpalo<'bump> {
    type FileDescriptorProtoType = self::FileDescriptorProtoBumpalo<'bump>;
    type FileRepeated = ::bumpalo::collections::Vec<'bump, self::FileDescriptorProtoBumpalo<'bump>>;
    fn file(&self) -> &Self::FileRepeated {
        &self.file
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

#[derive(Debug)]
pub struct FileDescriptorSetSliceView<'slice> {
    file: ::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice>,
}

impl<'slice> FileDescriptorSetSliceView<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Self {
            file: ::puroro_internal::helpers::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(slice),
        }
    }
}

impl<'slice> ::std::clone::Clone for FileDescriptorSetSliceView<'slice> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file: <::std::option::Option<::puroro_internal::types::SliceViewFields<'slice>> as FieldClone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice> ::puroro_internal::deser::DeserializableMessageFromSlice for FileDescriptorSetSliceView<'slice> {
    fn met_field_at<'slice2>(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice2>>, 
        field_number: usize,
        _: &'slice2 [u8],
        _: &'slice2 [u8],
    ) -> ::puroro::Result<bool>
    {
        todo!();
        
        Ok(true)
    }
}

impl<'slice> ::puroro_internal::ser::SerializableMessage for FileDescriptorSetSliceView<'slice> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        todo!()
    }
}

impl<'slice> ::puroro::Serializable for FileDescriptorSetSliceView<'slice> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

pub mod compiler;
