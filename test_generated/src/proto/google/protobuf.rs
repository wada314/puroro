#![allow(unused_variables)]
#![allow(unused_imports)]

pub trait GeneratedCodeInfoTrait {
    type AnnotationType: self::generated_code_info::AnnotationTrait;
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a>: ::std::iter::Iterator<Item=&'a Self::AnnotationType>
        where Self::AnnotationType: 'a;
    fn for_each_annotation<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::AnnotationType);
    fn annotation_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::AnnotationType>>;
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_>;
}

#[derive(Debug)]
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

impl ::std::clone::Clone for GeneratedCodeInfo {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            annotation: <::std::vec::Vec<generated_code_info::Annotation> as FieldClone>::clone(&self.annotation),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for GeneratedCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::vec::Vec<generated_code_info::Annotation> as FieldDeserFromIter<
                    tags::Message<generated_code_info::Annotation>, 
                    tags::Repeated>>
                ::deser(&mut self.annotation, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for GeneratedCodeInfo {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<generated_code_info::Annotation> as FieldSer<
                tags::Message<generated_code_info::Annotation>, 
                tags::Repeated>>
            ::ser(&self.annotation, serializer, 1)?;
        Ok(())
    }
}

impl ::puroro::Serializable for GeneratedCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl GeneratedCodeInfoTrait for GeneratedCodeInfo {
    type AnnotationType = generated_code_info::Annotation;
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::AnnotationType>;
    fn for_each_annotation<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::AnnotationType) {
        for item in (self.annotation).iter() {
            (f)(item);
        }
    }
    fn annotation_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::AnnotationType>> {
        ::std::boxed::Box::new(self.annotation.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn annotation_iter(&self) -> Self::AnnotationIter<'_> {
        self.annotation.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for GeneratedCodeInfo<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct GeneratedCodeInfoBumpalo<'bump> {
    pub annotation: ::bumpalo::collections::Vec<'bump, generated_code_info::AnnotationBumpalo<'bump>>,
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
impl<'bump> ::std::clone::Clone for GeneratedCodeInfoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            annotation: <::bumpalo::collections::Vec<'bump, generated_code_info::AnnotationBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.annotation, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for GeneratedCodeInfoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, generated_code_info::AnnotationBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<generated_code_info::AnnotationBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.annotation, field, || generated_code_info::AnnotationBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for GeneratedCodeInfoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, generated_code_info::AnnotationBumpalo<'bump>> as FieldSer<
                tags::Message<generated_code_info::AnnotationBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.annotation, serializer, 1)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for GeneratedCodeInfoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> GeneratedCodeInfoTrait for GeneratedCodeInfoBumpalo<'bump> {
    type AnnotationType = generated_code_info::AnnotationBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type AnnotationIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::AnnotationType>;
    fn for_each_annotation<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::AnnotationType) {
        for item in (self.annotation).iter() {
            (f)(item);
        }
    }
    fn annotation_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::AnnotationType>> {
        ::std::boxed::Box::new(self.annotation.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub mod generated_code_info {
pub trait AnnotationTrait {
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a>: ::std::iter::Iterator<Item=i32>
        where i32: 'a;
    fn for_each_path<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_>;
    fn source_file(&'_ self) -> ::std::option::Option<&'_ str>;
    fn begin(&'_ self) -> ::std::option::Option<i32>;
    fn end(&'_ self) -> ::std::option::Option<i32>;
}

#[derive(Debug)]
pub struct Annotation {
    pub path: ::std::vec::Vec<i32>,
    pub source_file: ::std::option::Option<::std::string::String>,
    pub begin: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for Annotation {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl AnnotationTrait for Annotation {
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl ::std::iter::Iterator<Item = i32>;
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
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_> {
        self.path.iter().cloned()
    }
    fn source_file(&'_ self) -> ::std::option::Option<&'_ str> {
        self.source_file.as_deref()
    }
    fn begin(&'_ self) -> ::std::option::Option<i32> {
        self.begin.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Annotation<> {
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
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                ::deser(&mut self.source_file, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for AnnotationBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> AnnotationTrait for AnnotationBumpalo<'bump> {
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl ::std::iter::Iterator<Item = i32>;
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
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_> {
        self.path.iter().cloned()
    }
    fn source_file(&'_ self) -> ::std::option::Option<&'_ str> {
        self.source_file.as_deref()
    }
    fn begin(&'_ self) -> ::std::option::Option<i32> {
        self.begin.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
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
} // mod generated_code_info
pub trait SourceCodeInfoTrait {
    type LocationType: self::source_code_info::LocationTrait;
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a>: ::std::iter::Iterator<Item=&'a Self::LocationType>
        where Self::LocationType: 'a;
    fn for_each_location<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::LocationType);
    fn location_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::LocationType>>;
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_>;
}

#[derive(Debug)]
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

impl ::std::clone::Clone for SourceCodeInfo {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            location: <::std::vec::Vec<source_code_info::Location> as FieldClone>::clone(&self.location),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for SourceCodeInfo {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::vec::Vec<source_code_info::Location> as FieldDeserFromIter<
                    tags::Message<source_code_info::Location>, 
                    tags::Repeated>>
                ::deser(&mut self.location, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for SourceCodeInfo {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<source_code_info::Location> as FieldSer<
                tags::Message<source_code_info::Location>, 
                tags::Repeated>>
            ::ser(&self.location, serializer, 1)?;
        Ok(())
    }
}

impl ::puroro::Serializable for SourceCodeInfo {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl SourceCodeInfoTrait for SourceCodeInfo {
    type LocationType = source_code_info::Location;
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::LocationType>;
    fn for_each_location<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::LocationType) {
        for item in (self.location).iter() {
            (f)(item);
        }
    }
    fn location_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::LocationType>> {
        ::std::boxed::Box::new(self.location.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn location_iter(&self) -> Self::LocationIter<'_> {
        self.location.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for SourceCodeInfo<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
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
impl<'bump> ::std::clone::Clone for SourceCodeInfoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            location: <::bumpalo::collections::Vec<'bump, source_code_info::LocationBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.location, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for SourceCodeInfoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, source_code_info::LocationBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<source_code_info::LocationBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.location, field, || source_code_info::LocationBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for SourceCodeInfoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, source_code_info::LocationBumpalo<'bump>> as FieldSer<
                tags::Message<source_code_info::LocationBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.location, serializer, 1)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for SourceCodeInfoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> SourceCodeInfoTrait for SourceCodeInfoBumpalo<'bump> {
    type LocationType = source_code_info::LocationBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type LocationIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::LocationType>;
    fn for_each_location<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::LocationType) {
        for item in (self.location).iter() {
            (f)(item);
        }
    }
    fn location_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::LocationType>> {
        ::std::boxed::Box::new(self.location.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub mod source_code_info {
pub trait LocationTrait {
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a>: ::std::iter::Iterator<Item=i32>
        where i32: 'a;
    #[cfg(feature = "puroro-nightly")]
    type SpanIter<'a>: ::std::iter::Iterator<Item=i32>
        where i32: 'a;
    #[cfg(feature = "puroro-nightly")]
    type LeadingDetachedCommentsIter<'a>: ::std::iter::Iterator<Item=&'a str>
        where str: 'a;
    fn for_each_path<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn path_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    fn path_iter(&self) -> Self::PathIter<'_>;
    fn for_each_span<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn span_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    fn span_iter(&self) -> Self::SpanIter<'_>;
    fn leading_comments(&'_ self) -> ::std::option::Option<&'_ str>;
    fn trailing_comments(&'_ self) -> ::std::option::Option<&'_ str>;
    fn for_each_leading_detached_comments<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn leading_detached_comments_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_>;
}

#[derive(Debug)]
pub struct Location {
    pub path: ::std::vec::Vec<i32>,
    pub span: ::std::vec::Vec<i32>,
    pub leading_comments: ::std::option::Option<::std::string::String>,
    pub trailing_comments: ::std::option::Option<::std::string::String>,
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for Location {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl LocationTrait for Location {
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type SpanIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type LeadingDetachedCommentsIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
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
    #[cfg(feature = "puroro-nightly")]
    fn span_iter(&self) -> Self::SpanIter<'_> {
        self.span.iter().cloned()
    }
    fn leading_comments(&'_ self) -> ::std::option::Option<&'_ str> {
        self.leading_comments.as_deref()
    }
    fn trailing_comments(&'_ self) -> ::std::option::Option<&'_ str> {
        self.trailing_comments.as_deref()
    }
    fn for_each_leading_detached_comments<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.leading_detached_comments).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn leading_detached_comments_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.leading_detached_comments.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn leading_detached_comments_iter(&self) -> Self::LeadingDetachedCommentsIter<'_> {
        self.leading_detached_comments.iter().map(|v| v.as_ref())
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for Location<> {
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
            leading_detached_comments: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                ::deser(&mut self.leading_comments, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            4 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.trailing_comments, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            6 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.leading_detached_comments, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for LocationBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> LocationTrait for LocationBumpalo<'bump> {
    #[cfg(feature = "puroro-nightly")]
    type PathIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type SpanIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type LeadingDetachedCommentsIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
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
    #[cfg(feature = "puroro-nightly")]
    fn span_iter(&self) -> Self::SpanIter<'_> {
        self.span.iter().cloned()
    }
    fn leading_comments(&'_ self) -> ::std::option::Option<&'_ str> {
        self.leading_comments.as_deref()
    }
    fn trailing_comments(&'_ self) -> ::std::option::Option<&'_ str> {
        self.trailing_comments.as_deref()
    }
    fn for_each_leading_detached_comments<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.leading_detached_comments).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn leading_detached_comments_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.leading_detached_comments.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
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
} // mod source_code_info
pub trait UninterpretedOptionTrait {
    type NamePartType: self::uninterpreted_option::NamePartTrait;
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a>: ::std::iter::Iterator<Item=&'a Self::NamePartType>
        where Self::NamePartType: 'a;
    fn for_each_name<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::NamePartType);
    fn name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::NamePartType>>;
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_>;
    fn identifier_value(&'_ self) -> ::std::option::Option<&'_ str>;
    fn positive_int_value(&'_ self) -> ::std::option::Option<u64>;
    fn negative_int_value(&'_ self) -> ::std::option::Option<i64>;
    fn double_value(&'_ self) -> ::std::option::Option<f64>;
    fn string_value(&'_ self) -> ::std::option::Option<&'_ [u8]>;
    fn aggregate_value(&'_ self) -> ::std::option::Option<&'_ str>;
}

#[derive(Debug)]
pub struct UninterpretedOption {
    pub name: ::std::vec::Vec<uninterpreted_option::NamePart>,
    pub identifier_value: ::std::option::Option<::std::string::String>,
    pub positive_int_value: ::std::option::Option<u64>,
    pub negative_int_value: ::std::option::Option<i64>,
    pub double_value: ::std::option::Option<f64>,
    pub string_value: ::std::option::Option<::std::vec::Vec<u8>>,
    pub aggregate_value: ::std::option::Option<::std::string::String>,
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

impl ::std::clone::Clone for UninterpretedOption {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::vec::Vec<uninterpreted_option::NamePart> as FieldClone>::clone(&self.name),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            2 => {
                <::std::vec::Vec<uninterpreted_option::NamePart> as FieldDeserFromIter<
                    tags::Message<uninterpreted_option::NamePart>, 
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for UninterpretedOption {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<uninterpreted_option::NamePart> as FieldSer<
                tags::Message<uninterpreted_option::NamePart>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl UninterpretedOptionTrait for UninterpretedOption {
    type NamePartType = uninterpreted_option::NamePart;
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::NamePartType>;
    fn for_each_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::NamePartType) {
        for item in (self.name).iter() {
            (f)(item);
        }
    }
    fn name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::NamePartType>> {
        ::std::boxed::Box::new(self.name.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_> {
        self.name.iter()
    }
    fn identifier_value(&'_ self) -> ::std::option::Option<&'_ str> {
        self.identifier_value.as_deref()
    }
    fn positive_int_value(&'_ self) -> ::std::option::Option<u64> {
        self.positive_int_value.clone()
    }
    fn negative_int_value(&'_ self) -> ::std::option::Option<i64> {
        self.negative_int_value.clone()
    }
    fn double_value(&'_ self) -> ::std::option::Option<f64> {
        self.double_value.clone()
    }
    fn string_value(&'_ self) -> ::std::option::Option<&'_ [u8]> {
        self.string_value.as_deref()
    }
    fn aggregate_value(&'_ self) -> ::std::option::Option<&'_ str> {
        self.aggregate_value.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for UninterpretedOption<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct UninterpretedOptionBumpalo<'bump> {
    pub name: ::bumpalo::collections::Vec<'bump, uninterpreted_option::NamePartBumpalo<'bump>>,
    pub identifier_value: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub positive_int_value: ::std::option::Option<u64>,
    pub negative_int_value: ::std::option::Option<i64>,
    pub double_value: ::std::option::Option<f64>,
    pub string_value: ::std::option::Option<::bumpalo::collections::Vec<'bump, u8>>,
    pub aggregate_value: ::std::option::Option<::bumpalo::collections::String<'bump>>,
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
impl<'bump> ::std::clone::Clone for UninterpretedOptionBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::bumpalo::collections::Vec<'bump, uninterpreted_option::NamePartBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            2 => {
                <::bumpalo::collections::Vec<'bump, uninterpreted_option::NamePartBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<uninterpreted_option::NamePartBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.name, field, || uninterpreted_option::NamePartBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.identifier_value, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
                ::deser(&mut self.string_value, field, || ::bumpalo::collections::Vec::new_in(self.puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.aggregate_value, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for UninterpretedOptionBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, uninterpreted_option::NamePartBumpalo<'bump>> as FieldSer<
                tags::Message<uninterpreted_option::NamePartBumpalo<'bump>>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> UninterpretedOptionTrait for UninterpretedOptionBumpalo<'bump> {
    type NamePartType = uninterpreted_option::NamePartBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type NameIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::NamePartType>;
    fn for_each_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::NamePartType) {
        for item in (self.name).iter() {
            (f)(item);
        }
    }
    fn name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::NamePartType>> {
        ::std::boxed::Box::new(self.name.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn name_iter(&self) -> Self::NameIter<'_> {
        self.name.iter()
    }
    fn identifier_value(&'_ self) -> ::std::option::Option<&'_ str> {
        self.identifier_value.as_deref()
    }
    fn positive_int_value(&'_ self) -> ::std::option::Option<u64> {
        self.positive_int_value.clone()
    }
    fn negative_int_value(&'_ self) -> ::std::option::Option<i64> {
        self.negative_int_value.clone()
    }
    fn double_value(&'_ self) -> ::std::option::Option<f64> {
        self.double_value.clone()
    }
    fn string_value(&'_ self) -> ::std::option::Option<&'_ [u8]> {
        self.string_value.as_deref()
    }
    fn aggregate_value(&'_ self) -> ::std::option::Option<&'_ str> {
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
pub mod uninterpreted_option {
pub trait NamePartTrait {
    fn name_part(&'_ self) -> &'_ str;
    fn is_extension(&'_ self) -> bool;
}

#[derive(Debug)]
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for NamePart {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl NamePartTrait for NamePart {
    fn name_part(&'_ self) -> &'_ str {
        self.name_part.as_ref()
    }
    fn is_extension(&'_ self) -> bool {
        self.is_extension.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for NamePart<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
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
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::bumpalo::collections::String<'bump> as FieldDeserFromIter<
                    tags::String, 
                    tags::Required>>
                ::deser(&mut self.name_part, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <bool as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Required>>
                ::deser(&mut self.is_extension, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for NamePartBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> NamePartTrait for NamePartBumpalo<'bump> {
    fn name_part(&'_ self) -> &'_ str {
        self.name_part.as_ref()
    }
    fn is_extension(&'_ self) -> bool {
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
} // mod uninterpreted_option
pub trait MethodOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn deprecated(&'_ self) -> ::std::option::Option<bool>;
    fn idempotency_level(&'_ self) -> ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
pub struct MethodOptions {
    pub deprecated: ::std::option::Option<bool>,
    pub idempotency_level: ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>>,
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

impl ::std::clone::Clone for MethodOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            idempotency_level: <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldClone>::clone(&self.idempotency_level),
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MethodOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for MethodOptions {
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
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for MethodOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl MethodOptionsTrait for MethodOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn idempotency_level(&'_ self) -> ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> {
        self.idempotency_level.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MethodOptions<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct MethodOptionsBumpalo<'bump> {
    pub deprecated: ::std::option::Option<bool>,
    pub idempotency_level: ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>>,
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
impl<'bump> ::std::clone::Clone for MethodOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            idempotency_level: <::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> as FieldClone>::clone_in_bumpalo(&self.idempotency_level, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for MethodOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for MethodOptionsBumpalo<'bump> {
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
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MethodOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodOptionsTrait for MethodOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn idempotency_level(&'_ self) -> ::std::option::Option<::std::result::Result<method_options::IdempotencyLevel, i32>> {
        self.idempotency_level.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn deprecated(&'_ self) -> ::std::option::Option<bool>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
pub struct ServiceOptions {
    pub deprecated: ::std::option::Option<bool>,
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

impl ::std::clone::Clone for ServiceOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ServiceOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            33 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for ServiceOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ServiceOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl ServiceOptionsTrait for ServiceOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ServiceOptions<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ServiceOptionsBumpalo<'bump> {
    pub deprecated: ::std::option::Option<bool>,
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
impl<'bump> ::std::clone::Clone for ServiceOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ServiceOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            33 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for ServiceOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 33)?;
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ServiceOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceOptionsTrait for ServiceOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub trait EnumValueOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn deprecated(&'_ self) -> ::std::option::Option<bool>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
pub struct EnumValueOptions {
    pub deprecated: ::std::option::Option<bool>,
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

impl ::std::clone::Clone for EnumValueOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for EnumValueOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 1)?;
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumValueOptionsTrait for EnumValueOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumValueOptions<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumValueOptionsBumpalo<'bump> {
    pub deprecated: ::std::option::Option<bool>,
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
impl<'bump> ::std::clone::Clone for EnumValueOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.deprecated, field, ::std::default::Default::default)?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for EnumValueOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<bool> as FieldSer<
                tags::Bool, 
                tags::Optional2>>
            ::ser(&self.deprecated, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumValueOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueOptionsTrait for EnumValueOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub trait EnumOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn allow_alias(&'_ self) -> ::std::option::Option<bool>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
pub struct EnumOptions {
    pub allow_alias: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
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

impl ::std::clone::Clone for EnumOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option<bool> as FieldClone>::clone(&self.allow_alias),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for EnumOptions {
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
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumOptionsTrait for EnumOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn allow_alias(&'_ self) -> ::std::option::Option<bool> {
        self.allow_alias.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumOptions<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumOptionsBumpalo<'bump> {
    pub allow_alias: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
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
impl<'bump> ::std::clone::Clone for EnumOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.allow_alias, self.puroro_internal.bumpalo()),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone_in_bumpalo(&self.deprecated, self.puroro_internal.bumpalo()),
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for EnumOptionsBumpalo<'bump> {
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
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumOptionsTrait for EnumOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn allow_alias(&'_ self) -> ::std::option::Option<bool> {
        self.allow_alias.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub trait OneofOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
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

impl ::std::clone::Clone for OneofOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for OneofOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            999 => {
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for OneofOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for OneofOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl OneofOptionsTrait for OneofOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for OneofOptions<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
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
impl<'bump> ::std::clone::Clone for OneofOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for OneofOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            999 => {
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for OneofOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for OneofOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofOptionsTrait for OneofOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub trait FieldOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn ctype(&'_ self) -> ::std::option::Option<::std::result::Result<field_options::Ctype, i32>>;
    fn packed(&'_ self) -> ::std::option::Option<bool>;
    fn jstype(&'_ self) -> ::std::option::Option<::std::result::Result<field_options::Jstype, i32>>;
    fn lazy(&'_ self) -> ::std::option::Option<bool>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool>;
    fn weak(&'_ self) -> ::std::option::Option<bool>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
pub struct FieldOptions {
    pub ctype: ::std::option::Option<::std::result::Result<field_options::Ctype, i32>>,
    pub packed: ::std::option::Option<bool>,
    pub jstype: ::std::option::Option<::std::result::Result<field_options::Jstype, i32>>,
    pub lazy: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub weak: ::std::option::Option<bool>,
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
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FieldOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for FieldOptions {
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
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FieldOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl FieldOptionsTrait for FieldOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn ctype(&'_ self) -> ::std::option::Option<::std::result::Result<field_options::Ctype, i32>> {
        self.ctype.clone()
    }
    fn packed(&'_ self) -> ::std::option::Option<bool> {
        self.packed.clone()
    }
    fn jstype(&'_ self) -> ::std::option::Option<::std::result::Result<field_options::Jstype, i32>> {
        self.jstype.clone()
    }
    fn lazy(&'_ self) -> ::std::option::Option<bool> {
        self.lazy.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn weak(&'_ self) -> ::std::option::Option<bool> {
        self.weak.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FieldOptions<> {
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
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FieldOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for FieldOptionsBumpalo<'bump> {
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
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FieldOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldOptionsTrait for FieldOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn ctype(&'_ self) -> ::std::option::Option<::std::result::Result<field_options::Ctype, i32>> {
        self.ctype.clone()
    }
    fn packed(&'_ self) -> ::std::option::Option<bool> {
        self.packed.clone()
    }
    fn jstype(&'_ self) -> ::std::option::Option<::std::result::Result<field_options::Jstype, i32>> {
        self.jstype.clone()
    }
    fn lazy(&'_ self) -> ::std::option::Option<bool> {
        self.lazy.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn weak(&'_ self) -> ::std::option::Option<bool> {
        self.weak.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn message_set_wire_format(&'_ self) -> ::std::option::Option<bool>;
    fn no_standard_descriptor_accessor(&'_ self) -> ::std::option::Option<bool>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool>;
    fn map_entry(&'_ self) -> ::std::option::Option<bool>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
pub struct MessageOptions {
    pub message_set_wire_format: ::std::option::Option<bool>,
    pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
    pub deprecated: ::std::option::Option<bool>,
    pub map_entry: ::std::option::Option<bool>,
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

impl ::std::clone::Clone for MessageOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            message_set_wire_format: <::std::option::Option<bool> as FieldClone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <::std::option::Option<bool> as FieldClone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <::std::option::Option<bool> as FieldClone>::clone(&self.deprecated),
            map_entry: <::std::option::Option<bool> as FieldClone>::clone(&self.map_entry),
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for MessageOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for MessageOptions {
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
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for MessageOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl MessageOptionsTrait for MessageOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn message_set_wire_format(&'_ self) -> ::std::option::Option<bool> {
        self.message_set_wire_format.clone()
    }
    fn no_standard_descriptor_accessor(&'_ self) -> ::std::option::Option<bool> {
        self.no_standard_descriptor_accessor.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn map_entry(&'_ self) -> ::std::option::Option<bool> {
        self.map_entry.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MessageOptions<> {
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
    pub uninterpreted_option: ::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
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
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for MessageOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for MessageOptionsBumpalo<'bump> {
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
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for MessageOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MessageOptionsTrait for MessageOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn message_set_wire_format(&'_ self) -> ::std::option::Option<bool> {
        self.message_set_wire_format.clone()
    }
    fn no_standard_descriptor_accessor(&'_ self) -> ::std::option::Option<bool> {
        self.no_standard_descriptor_accessor.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn map_entry(&'_ self) -> ::std::option::Option<bool> {
        self.map_entry.clone()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub trait FileOptionsTrait {
    type UninterpretedOptionType: self::UninterpretedOptionTrait;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn java_package(&'_ self) -> ::std::option::Option<&'_ str>;
    fn java_outer_classname(&'_ self) -> ::std::option::Option<&'_ str>;
    fn java_multiple_files(&'_ self) -> ::std::option::Option<bool>;
    fn java_generate_equals_and_hash(&'_ self) -> ::std::option::Option<bool>;
    fn java_string_check_utf8(&'_ self) -> ::std::option::Option<bool>;
    fn optimize_for(&'_ self) -> ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>>;
    fn go_package(&'_ self) -> ::std::option::Option<&'_ str>;
    fn cc_generic_services(&'_ self) -> ::std::option::Option<bool>;
    fn java_generic_services(&'_ self) -> ::std::option::Option<bool>;
    fn py_generic_services(&'_ self) -> ::std::option::Option<bool>;
    fn php_generic_services(&'_ self) -> ::std::option::Option<bool>;
    fn deprecated(&'_ self) -> ::std::option::Option<bool>;
    fn cc_enable_arenas(&'_ self) -> ::std::option::Option<bool>;
    fn objc_class_prefix(&'_ self) -> ::std::option::Option<&'_ str>;
    fn csharp_namespace(&'_ self) -> ::std::option::Option<&'_ str>;
    fn swift_prefix(&'_ self) -> ::std::option::Option<&'_ str>;
    fn php_class_prefix(&'_ self) -> ::std::option::Option<&'_ str>;
    fn php_namespace(&'_ self) -> ::std::option::Option<&'_ str>;
    fn php_metadata_namespace(&'_ self) -> ::std::option::Option<&'_ str>;
    fn ruby_package(&'_ self) -> ::std::option::Option<&'_ str>;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
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
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for FileOptions {
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
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FileOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileOptionsTrait for FileOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn java_package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.java_package.as_deref()
    }
    fn java_outer_classname(&'_ self) -> ::std::option::Option<&'_ str> {
        self.java_outer_classname.as_deref()
    }
    fn java_multiple_files(&'_ self) -> ::std::option::Option<bool> {
        self.java_multiple_files.clone()
    }
    fn java_generate_equals_and_hash(&'_ self) -> ::std::option::Option<bool> {
        self.java_generate_equals_and_hash.clone()
    }
    fn java_string_check_utf8(&'_ self) -> ::std::option::Option<bool> {
        self.java_string_check_utf8.clone()
    }
    fn optimize_for(&'_ self) -> ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> {
        self.optimize_for.clone()
    }
    fn go_package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.go_package.as_deref()
    }
    fn cc_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.cc_generic_services.clone()
    }
    fn java_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.java_generic_services.clone()
    }
    fn py_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.py_generic_services.clone()
    }
    fn php_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.php_generic_services.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn cc_enable_arenas(&'_ self) -> ::std::option::Option<bool> {
        self.cc_enable_arenas.clone()
    }
    fn objc_class_prefix(&'_ self) -> ::std::option::Option<&'_ str> {
        self.objc_class_prefix.as_deref()
    }
    fn csharp_namespace(&'_ self) -> ::std::option::Option<&'_ str> {
        self.csharp_namespace.as_deref()
    }
    fn swift_prefix(&'_ self) -> ::std::option::Option<&'_ str> {
        self.swift_prefix.as_deref()
    }
    fn php_class_prefix(&'_ self) -> ::std::option::Option<&'_ str> {
        self.php_class_prefix.as_deref()
    }
    fn php_namespace(&'_ self) -> ::std::option::Option<&'_ str> {
        self.php_namespace.as_deref()
    }
    fn php_metadata_namespace(&'_ self) -> ::std::option::Option<&'_ str> {
        self.php_metadata_namespace.as_deref()
    }
    fn ruby_package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.ruby_package.as_deref()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileOptions<> {
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
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.java_package, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.java_outer_classname, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
                ::deser(&mut self.go_package, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
                ::deser(&mut self.objc_class_prefix, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            37 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.csharp_namespace, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            39 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.swift_prefix, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            40 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_class_prefix, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            41 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_namespace, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            44 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.php_metadata_namespace, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            45 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.ruby_package, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            999 => {
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for FileOptionsBumpalo<'bump> {
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
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileOptionsTrait for FileOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn java_package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.java_package.as_deref()
    }
    fn java_outer_classname(&'_ self) -> ::std::option::Option<&'_ str> {
        self.java_outer_classname.as_deref()
    }
    fn java_multiple_files(&'_ self) -> ::std::option::Option<bool> {
        self.java_multiple_files.clone()
    }
    fn java_generate_equals_and_hash(&'_ self) -> ::std::option::Option<bool> {
        self.java_generate_equals_and_hash.clone()
    }
    fn java_string_check_utf8(&'_ self) -> ::std::option::Option<bool> {
        self.java_string_check_utf8.clone()
    }
    fn optimize_for(&'_ self) -> ::std::option::Option<::std::result::Result<file_options::OptimizeMode, i32>> {
        self.optimize_for.clone()
    }
    fn go_package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.go_package.as_deref()
    }
    fn cc_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.cc_generic_services.clone()
    }
    fn java_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.java_generic_services.clone()
    }
    fn py_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.py_generic_services.clone()
    }
    fn php_generic_services(&'_ self) -> ::std::option::Option<bool> {
        self.php_generic_services.clone()
    }
    fn deprecated(&'_ self) -> ::std::option::Option<bool> {
        self.deprecated.clone()
    }
    fn cc_enable_arenas(&'_ self) -> ::std::option::Option<bool> {
        self.cc_enable_arenas.clone()
    }
    fn objc_class_prefix(&'_ self) -> ::std::option::Option<&'_ str> {
        self.objc_class_prefix.as_deref()
    }
    fn csharp_namespace(&'_ self) -> ::std::option::Option<&'_ str> {
        self.csharp_namespace.as_deref()
    }
    fn swift_prefix(&'_ self) -> ::std::option::Option<&'_ str> {
        self.swift_prefix.as_deref()
    }
    fn php_class_prefix(&'_ self) -> ::std::option::Option<&'_ str> {
        self.php_class_prefix.as_deref()
    }
    fn php_namespace(&'_ self) -> ::std::option::Option<&'_ str> {
        self.php_namespace.as_deref()
    }
    fn php_metadata_namespace(&'_ self) -> ::std::option::Option<&'_ str> {
        self.php_metadata_namespace.as_deref()
    }
    fn ruby_package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.ruby_package.as_deref()
    }
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn input_type(&'_ self) -> ::std::option::Option<&'_ str>;
    fn output_type(&'_ self) -> ::std::option::Option<&'_ str>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::MethodOptionsType>;
    fn client_streaming(&'_ self) -> ::std::option::Option<bool>;
    fn server_streaming(&'_ self) -> ::std::option::Option<bool>;
}

#[derive(Debug)]
pub struct MethodDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub input_type: ::std::option::Option<::std::string::String>,
    pub output_type: ::std::option::Option<::std::string::String>,
    pub options: ::std::option::Option<::std::boxed::Box<MethodOptions>>,
    pub client_streaming: ::std::option::Option<bool>,
    pub server_streaming: ::std::option::Option<bool>,
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

impl ::std::clone::Clone for MethodDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            input_type: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.input_type),
            output_type: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.output_type),
            options: <::std::option::Option<::std::boxed::Box<MethodOptions>> as FieldClone>::clone(&self.options),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::option::Option<::std::boxed::Box<MethodOptions>> as FieldDeserFromIter<
                    tags::Message<MethodOptions>, 
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for MethodDescriptorProto {
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
        <::std::option::Option<::std::boxed::Box<MethodOptions>> as FieldSer<
                tags::Message<MethodOptions>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl MethodDescriptorProtoTrait for MethodDescriptorProto {
    type MethodOptionsType = MethodOptions;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn input_type(&'_ self) -> ::std::option::Option<&'_ str> {
        self.input_type.as_deref()
    }
    fn output_type(&'_ self) -> ::std::option::Option<&'_ str> {
        self.output_type.as_deref()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::MethodOptionsType> {
        self.options.as_deref()
    }
    fn client_streaming(&'_ self) -> ::std::option::Option<bool> {
        self.client_streaming.clone()
    }
    fn server_streaming(&'_ self) -> ::std::option::Option<bool> {
        self.server_streaming.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for MethodDescriptorProto<> {
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
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, MethodOptionsBumpalo<'bump>>>,
    pub client_streaming: ::std::option::Option<bool>,
    pub server_streaming: ::std::option::Option<bool>,
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
impl<'bump> ::std::clone::Clone for MethodDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            input_type: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.input_type, self.puroro_internal.bumpalo()),
            output_type: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.output_type, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, MethodOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.input_type, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.output_type, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            4 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, MethodOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<MethodOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(MethodOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
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
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for MethodDescriptorProtoBumpalo<'bump> {
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
        <::std::option::Option<::bumpalo::boxed::Box<'bump, MethodOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<MethodOptionsBumpalo<'bump>>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> MethodDescriptorProtoTrait for MethodDescriptorProtoBumpalo<'bump> {
    type MethodOptionsType = MethodOptionsBumpalo<'bump>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn input_type(&'_ self) -> ::std::option::Option<&'_ str> {
        self.input_type.as_deref()
    }
    fn output_type(&'_ self) -> ::std::option::Option<&'_ str> {
        self.output_type.as_deref()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::MethodOptionsType> {
        self.options.as_deref()
    }
    fn client_streaming(&'_ self) -> ::std::option::Option<bool> {
        self.client_streaming.clone()
    }
    fn server_streaming(&'_ self) -> ::std::option::Option<bool> {
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
pub trait ServiceDescriptorProtoTrait {
    type MethodDescriptorProtoType: self::MethodDescriptorProtoTrait;
    type ServiceOptionsType: self::ServiceOptionsTrait;
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a>: ::std::iter::Iterator<Item=&'a Self::MethodDescriptorProtoType>
        where Self::MethodDescriptorProtoType: 'a;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn for_each_method<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::MethodDescriptorProtoType);
    fn method_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::MethodDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::ServiceOptionsType>;
}

#[derive(Debug)]
pub struct ServiceDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
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

impl ::std::clone::Clone for ServiceDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            method: <::std::vec::Vec<MethodDescriptorProto> as FieldClone>::clone(&self.method),
            options: <::std::option::Option<::std::boxed::Box<ServiceOptions>> as FieldClone>::clone(&self.options),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ServiceDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec<MethodDescriptorProto> as FieldDeserFromIter<
                    tags::Message<MethodDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.method, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<ServiceOptions>> as FieldDeserFromIter<
                    tags::Message<ServiceOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for ServiceDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec<MethodDescriptorProto> as FieldSer<
                tags::Message<MethodDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.method, serializer, 2)?;
        <::std::option::Option<::std::boxed::Box<ServiceOptions>> as FieldSer<
                tags::Message<ServiceOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ServiceDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl ServiceDescriptorProtoTrait for ServiceDescriptorProto {
    type MethodDescriptorProtoType = MethodDescriptorProto;
    type ServiceOptionsType = ServiceOptions;
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::MethodDescriptorProtoType>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn for_each_method<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::MethodDescriptorProtoType) {
        for item in (self.method).iter() {
            (f)(item);
        }
    }
    fn method_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::MethodDescriptorProtoType>> {
        ::std::boxed::Box::new(self.method.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_> {
        self.method.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::ServiceOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ServiceDescriptorProto<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ServiceDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
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
impl<'bump> ::std::clone::Clone for ServiceDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            method: <::bumpalo::collections::Vec<'bump, MethodDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.method, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, ServiceOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ServiceDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::bumpalo::collections::Vec<'bump, MethodDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<MethodDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.method, field, || MethodDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, ServiceOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<ServiceOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(ServiceOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for ServiceDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, MethodDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<MethodDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.method, serializer, 2)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, ServiceOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<ServiceOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ServiceDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ServiceDescriptorProtoTrait for ServiceDescriptorProtoBumpalo<'bump> {
    type MethodDescriptorProtoType = MethodDescriptorProtoBumpalo<'bump>;
    type ServiceOptionsType = ServiceOptionsBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type MethodIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::MethodDescriptorProtoType>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn for_each_method<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::MethodDescriptorProtoType) {
        for item in (self.method).iter() {
            (f)(item);
        }
    }
    fn method_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::MethodDescriptorProtoType>> {
        ::std::boxed::Box::new(self.method.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn method_iter(&self) -> Self::MethodIter<'_> {
        self.method.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::ServiceOptionsType> {
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
pub trait EnumValueDescriptorProtoTrait {
    type EnumValueOptionsType: self::EnumValueOptionsTrait;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn number(&'_ self) -> ::std::option::Option<i32>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::EnumValueOptionsType>;
}

#[derive(Debug)]
pub struct EnumValueDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub number: ::std::option::Option<i32>,
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

impl ::std::clone::Clone for EnumValueDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            number: <::std::option::Option<i32> as FieldClone>::clone(&self.number),
            options: <::std::option::Option<::std::boxed::Box<EnumValueOptions>> as FieldClone>::clone(&self.options),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::option::Option<::std::boxed::Box<EnumValueOptions>> as FieldDeserFromIter<
                    tags::Message<EnumValueOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for EnumValueDescriptorProto {
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
        <::std::option::Option<::std::boxed::Box<EnumValueOptions>> as FieldSer<
                tags::Message<EnumValueOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}

impl ::puroro::Serializable for EnumValueDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumValueDescriptorProtoTrait for EnumValueDescriptorProto {
    type EnumValueOptionsType = EnumValueOptions;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&'_ self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::EnumValueOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumValueDescriptorProto<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumValueDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub number: ::std::option::Option<i32>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, EnumValueOptionsBumpalo<'bump>>>,
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
impl<'bump> ::std::clone::Clone for EnumValueDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            number: <::std::option::Option<i32> as FieldClone>::clone_in_bumpalo(&self.number, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, EnumValueOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for EnumValueDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<i32> as FieldDeserFromIter<
                    tags::Int32, 
                    tags::Optional2>>
                ::deser(&mut self.number, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, EnumValueOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<EnumValueOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(EnumValueOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for EnumValueDescriptorProtoBumpalo<'bump> {
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
        <::std::option::Option<::bumpalo::boxed::Box<'bump, EnumValueOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<EnumValueOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for EnumValueDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumValueDescriptorProtoTrait for EnumValueDescriptorProtoBumpalo<'bump> {
    type EnumValueOptionsType = EnumValueOptionsBumpalo<'bump>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&'_ self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::EnumValueOptionsType> {
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
pub trait EnumDescriptorProtoTrait {
    type EnumValueDescriptorProtoType: self::EnumValueDescriptorProtoTrait;
    type EnumOptionsType: self::EnumOptionsTrait;
    type EnumReservedRangeType: self::enum_descriptor_proto::EnumReservedRangeTrait;
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a>: ::std::iter::Iterator<Item=&'a Self::EnumValueDescriptorProtoType>
        where Self::EnumValueDescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a>: ::std::iter::Iterator<Item=&'a Self::EnumReservedRangeType>
        where Self::EnumReservedRangeType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a>: ::std::iter::Iterator<Item=&'a str>
        where str: 'a;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn for_each_value<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::EnumValueDescriptorProtoType);
    fn value_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumValueDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::EnumOptionsType>;
    fn for_each_reserved_range<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::EnumReservedRangeType);
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumReservedRangeType>>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_>;
    fn for_each_reserved_name<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_>;
}

#[derive(Debug)]
pub struct EnumDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
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

impl ::std::clone::Clone for EnumDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            value: <::std::vec::Vec<EnumValueDescriptorProto> as FieldClone>::clone(&self.value),
            options: <::std::option::Option<::std::boxed::Box<EnumOptions>> as FieldClone>::clone(&self.options),
            reserved_range: <::std::vec::Vec<enum_descriptor_proto::EnumReservedRange> as FieldClone>::clone(&self.reserved_range),
            reserved_name: <::std::vec::Vec<::std::string::String> as FieldClone>::clone(&self.reserved_name),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for EnumDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec<EnumValueDescriptorProto> as FieldDeserFromIter<
                    tags::Message<EnumValueDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.value, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::option::Option<::std::boxed::Box<EnumOptions>> as FieldDeserFromIter<
                    tags::Message<EnumOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec<enum_descriptor_proto::EnumReservedRange> as FieldDeserFromIter<
                    tags::Message<enum_descriptor_proto::EnumReservedRange>, 
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for EnumDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec<EnumValueDescriptorProto> as FieldSer<
                tags::Message<EnumValueDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.value, serializer, 2)?;
        <::std::option::Option<::std::boxed::Box<EnumOptions>> as FieldSer<
                tags::Message<EnumOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        <::std::vec::Vec<enum_descriptor_proto::EnumReservedRange> as FieldSer<
                tags::Message<enum_descriptor_proto::EnumReservedRange>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumDescriptorProtoTrait for EnumDescriptorProto {
    type EnumValueDescriptorProtoType = EnumValueDescriptorProto;
    type EnumOptionsType = EnumOptions;
    type EnumReservedRangeType = enum_descriptor_proto::EnumReservedRange;
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumValueDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumReservedRangeType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn for_each_value<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumValueDescriptorProtoType) {
        for item in (self.value).iter() {
            (f)(item);
        }
    }
    fn value_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumValueDescriptorProtoType>> {
        ::std::boxed::Box::new(self.value.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_> {
        self.value.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::EnumOptionsType> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumReservedRangeType) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumReservedRangeType>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumDescriptorProto<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub value: ::bumpalo::collections::Vec<'bump, EnumValueDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, EnumOptionsBumpalo<'bump>>>,
    pub reserved_range: ::bumpalo::collections::Vec<'bump, enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>,
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
impl<'bump> ::std::clone::Clone for EnumDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            value: <::bumpalo::collections::Vec<'bump, EnumValueDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.value, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, EnumOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            reserved_range: <::bumpalo::collections::Vec<'bump, enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.reserved_range, self.puroro_internal.bumpalo()),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::bumpalo::collections::Vec<'bump, EnumValueDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<EnumValueDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.value, field, || EnumValueDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, EnumOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<EnumOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(EnumOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            4 => {
                <::bumpalo::collections::Vec<'bump, enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_range, field, || enum_descriptor_proto::EnumReservedRangeBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            5 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for EnumDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, EnumValueDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<EnumValueDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.value, serializer, 2)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, EnumOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<EnumOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 3)?;
        <::bumpalo::collections::Vec<'bump, enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>> as FieldSer<
                tags::Message<enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumDescriptorProtoTrait for EnumDescriptorProtoBumpalo<'bump> {
    type EnumValueDescriptorProtoType = EnumValueDescriptorProtoBumpalo<'bump>;
    type EnumOptionsType = EnumOptionsBumpalo<'bump>;
    type EnumReservedRangeType = enum_descriptor_proto::EnumReservedRangeBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type ValueIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumValueDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumReservedRangeType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn for_each_value<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumValueDescriptorProtoType) {
        for item in (self.value).iter() {
            (f)(item);
        }
    }
    fn value_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumValueDescriptorProtoType>> {
        ::std::boxed::Box::new(self.value.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn value_iter(&self) -> Self::ValueIter<'_> {
        self.value.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::EnumOptionsType> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumReservedRangeType) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumReservedRangeType>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
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
pub mod enum_descriptor_proto {
pub trait EnumReservedRangeTrait {
    fn start(&'_ self) -> ::std::option::Option<i32>;
    fn end(&'_ self) -> ::std::option::Option<i32>;
}

#[derive(Debug)]
pub struct EnumReservedRange {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for EnumReservedRange {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl EnumReservedRangeTrait for EnumReservedRange {
    fn start(&'_ self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for EnumReservedRange<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct EnumReservedRangeBumpalo<'bump> {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumReservedRangeBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for EnumReservedRangeBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> EnumReservedRangeTrait for EnumReservedRangeBumpalo<'bump> {
    fn start(&'_ self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
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
} // mod enum_descriptor_proto
pub trait OneofDescriptorProtoTrait {
    type OneofOptionsType: self::OneofOptionsTrait;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::OneofOptionsType>;
}

#[derive(Debug)]
pub struct OneofDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
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

impl ::std::clone::Clone for OneofDescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            options: <::std::option::Option<::std::boxed::Box<OneofOptions>> as FieldClone>::clone(&self.options),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for OneofDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::option::Option<::std::boxed::Box<OneofOptions>> as FieldDeserFromIter<
                    tags::Message<OneofOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for OneofDescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::std::boxed::Box<OneofOptions>> as FieldSer<
                tags::Message<OneofOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 2)?;
        Ok(())
    }
}

impl ::puroro::Serializable for OneofDescriptorProto {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl OneofDescriptorProtoTrait for OneofDescriptorProto {
    type OneofOptionsType = OneofOptions;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::OneofOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for OneofDescriptorProto<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct OneofDescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
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
impl<'bump> ::std::clone::Clone for OneofDescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, OneofOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for OneofDescriptorProtoBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, OneofOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<OneofOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(OneofOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for OneofDescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, OneofOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<OneofOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 2)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for OneofDescriptorProtoBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> OneofDescriptorProtoTrait for OneofDescriptorProtoBumpalo<'bump> {
    type OneofOptionsType = OneofOptionsBumpalo<'bump>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::OneofOptionsType> {
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
pub trait FieldDescriptorProtoTrait {
    type FieldOptionsType: self::FieldOptionsTrait;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn number(&'_ self) -> ::std::option::Option<i32>;
    fn label(&'_ self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>>;
    fn type_(&'_ self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>>;
    fn type_name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn extendee(&'_ self) -> ::std::option::Option<&'_ str>;
    fn default_value(&'_ self) -> ::std::option::Option<&'_ str>;
    fn oneof_index(&'_ self) -> ::std::option::Option<i32>;
    fn json_name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::FieldOptionsType>;
    fn proto3_optional(&'_ self) -> ::std::option::Option<bool>;
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
    pub options: ::std::option::Option<::std::boxed::Box<FieldOptions>>,
    pub proto3_optional: ::std::option::Option<bool>,
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
            options: <::std::option::Option<::std::boxed::Box<FieldOptions>> as FieldClone>::clone(&self.options),
            proto3_optional: <::std::option::Option<bool> as FieldClone>::clone(&self.proto3_optional),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FieldDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::option::Option<::std::boxed::Box<FieldOptions>> as FieldDeserFromIter<
                    tags::Message<FieldOptions>, 
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for FieldDescriptorProto {
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
        <::std::option::Option<::std::boxed::Box<FieldOptions>> as FieldSer<
                tags::Message<FieldOptions>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl FieldDescriptorProtoTrait for FieldDescriptorProto {
    type FieldOptionsType = FieldOptions;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&'_ self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn label(&'_ self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> {
        self.label.clone()
    }
    fn type_(&'_ self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> {
        self.type_.clone()
    }
    fn type_name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.type_name.as_deref()
    }
    fn extendee(&'_ self) -> ::std::option::Option<&'_ str> {
        self.extendee.as_deref()
    }
    fn default_value(&'_ self) -> ::std::option::Option<&'_ str> {
        self.default_value.as_deref()
    }
    fn oneof_index(&'_ self) -> ::std::option::Option<i32> {
        self.oneof_index.clone()
    }
    fn json_name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.json_name.as_deref()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::FieldOptionsType> {
        self.options.as_deref()
    }
    fn proto3_optional(&'_ self) -> ::std::option::Option<bool> {
        self.proto3_optional.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FieldDescriptorProto<> {
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
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, FieldOptionsBumpalo<'bump>>>,
    pub proto3_optional: ::std::option::Option<bool>,
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
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, FieldOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
                ::deser(&mut self.type_name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.extendee, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            7 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.default_value, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
                ::deser(&mut self.json_name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, FieldOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<FieldOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(FieldOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            17 => {
                <::std::option::Option<bool> as FieldDeserFromIter<
                    tags::Bool, 
                    tags::Optional2>>
                ::deser(&mut self.proto3_optional, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for FieldDescriptorProtoBumpalo<'bump> {
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
        <::std::option::Option<::bumpalo::boxed::Box<'bump, FieldOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<FieldOptionsBumpalo<'bump>>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldDescriptorProtoTrait for FieldDescriptorProtoBumpalo<'bump> {
    type FieldOptionsType = FieldOptionsBumpalo<'bump>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn number(&'_ self) -> ::std::option::Option<i32> {
        self.number.clone()
    }
    fn label(&'_ self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Label, i32>> {
        self.label.clone()
    }
    fn type_(&'_ self) -> ::std::option::Option<::std::result::Result<field_descriptor_proto::Type, i32>> {
        self.type_.clone()
    }
    fn type_name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.type_name.as_deref()
    }
    fn extendee(&'_ self) -> ::std::option::Option<&'_ str> {
        self.extendee.as_deref()
    }
    fn default_value(&'_ self) -> ::std::option::Option<&'_ str> {
        self.default_value.as_deref()
    }
    fn oneof_index(&'_ self) -> ::std::option::Option<i32> {
        self.oneof_index.clone()
    }
    fn json_name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.json_name.as_deref()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::FieldOptionsType> {
        self.options.as_deref()
    }
    fn proto3_optional(&'_ self) -> ::std::option::Option<bool> {
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
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a>: ::std::iter::Iterator<Item=&'a Self::UninterpretedOptionType>
        where Self::UninterpretedOptionType: 'a;
    fn for_each_uninterpreted_option<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType);
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>>;
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_>;
}

#[derive(Debug)]
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

impl ::std::clone::Clone for ExtensionRangeOptions {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::vec::Vec<UninterpretedOption> as FieldClone>::clone(&self.uninterpreted_option),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRangeOptions {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            999 => {
                <::std::vec::Vec<UninterpretedOption> as FieldDeserFromIter<
                    tags::Message<UninterpretedOption>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for ExtensionRangeOptions {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<UninterpretedOption> as FieldSer<
                tags::Message<UninterpretedOption>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}

impl ::puroro::Serializable for ExtensionRangeOptions {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl ExtensionRangeOptionsTrait for ExtensionRangeOptions {
    type UninterpretedOptionType = UninterpretedOption;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn uninterpreted_option_iter(&self) -> Self::UninterpretedOptionIter<'_> {
        self.uninterpreted_option.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ExtensionRangeOptions<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
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
impl<'bump> ::std::clone::Clone for ExtensionRangeOptionsBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.uninterpreted_option, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for ExtensionRangeOptionsBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            999 => {
                <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.uninterpreted_option, field, || UninterpretedOptionBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for ExtensionRangeOptionsBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, UninterpretedOptionBumpalo<'bump>> as FieldSer<
                tags::Message<UninterpretedOptionBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.uninterpreted_option, serializer, 999)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for ExtensionRangeOptionsBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeOptionsTrait for ExtensionRangeOptionsBumpalo<'bump> {
    type UninterpretedOptionType = UninterpretedOptionBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type UninterpretedOptionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::UninterpretedOptionType>;
    fn for_each_uninterpreted_option<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::UninterpretedOptionType) {
        for item in (self.uninterpreted_option).iter() {
            (f)(item);
        }
    }
    fn uninterpreted_option_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::UninterpretedOptionType>> {
        ::std::boxed::Box::new(self.uninterpreted_option.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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
pub trait DescriptorProtoTrait {
    type FieldDescriptorProtoType: self::FieldDescriptorProtoTrait;
    type DescriptorProtoType: self::DescriptorProtoTrait;
    type EnumDescriptorProtoType: self::EnumDescriptorProtoTrait;
    type ExtensionRangeType: self::descriptor_proto::ExtensionRangeTrait;
    type OneofDescriptorProtoType: self::OneofDescriptorProtoTrait;
    type MessageOptionsType: self::MessageOptionsTrait;
    type ReservedRangeType: self::descriptor_proto::ReservedRangeTrait;
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a>: ::std::iter::Iterator<Item=&'a Self::FieldDescriptorProtoType>
        where Self::FieldDescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a>: ::std::iter::Iterator<Item=&'a Self::FieldDescriptorProtoType>
        where Self::FieldDescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a>: ::std::iter::Iterator<Item=&'a Self::DescriptorProtoType>
        where Self::DescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a>: ::std::iter::Iterator<Item=&'a Self::EnumDescriptorProtoType>
        where Self::EnumDescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a>: ::std::iter::Iterator<Item=&'a Self::ExtensionRangeType>
        where Self::ExtensionRangeType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a>: ::std::iter::Iterator<Item=&'a Self::OneofDescriptorProtoType>
        where Self::OneofDescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a>: ::std::iter::Iterator<Item=&'a Self::ReservedRangeType>
        where Self::ReservedRangeType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a>: ::std::iter::Iterator<Item=&'a str>
        where str: 'a;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn for_each_field<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType);
    fn field_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_>;
    fn for_each_extension<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType);
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_>;
    fn for_each_nested_type<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::DescriptorProtoType);
    fn nested_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::DescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_>;
    fn for_each_enum_type<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::EnumDescriptorProtoType);
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_>;
    fn for_each_extension_range<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::ExtensionRangeType);
    fn extension_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ExtensionRangeType>>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_>;
    fn for_each_oneof_decl<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::OneofDescriptorProtoType);
    fn oneof_decl_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::OneofDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::MessageOptionsType>;
    fn for_each_reserved_range<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::ReservedRangeType);
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ReservedRangeType>>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_>;
    fn for_each_reserved_name<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_>;
}

#[derive(Debug)]
pub struct DescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
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

impl ::std::clone::Clone for DescriptorProto {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.name),
            field: <::std::vec::Vec<FieldDescriptorProto> as FieldClone>::clone(&self.field),
            extension: <::std::vec::Vec<FieldDescriptorProto> as FieldClone>::clone(&self.extension),
            nested_type: <::std::vec::Vec<DescriptorProto> as FieldClone>::clone(&self.nested_type),
            enum_type: <::std::vec::Vec<EnumDescriptorProto> as FieldClone>::clone(&self.enum_type),
            extension_range: <::std::vec::Vec<descriptor_proto::ExtensionRange> as FieldClone>::clone(&self.extension_range),
            oneof_decl: <::std::vec::Vec<OneofDescriptorProto> as FieldClone>::clone(&self.oneof_decl),
            options: <::std::option::Option<::std::boxed::Box<MessageOptions>> as FieldClone>::clone(&self.options),
            reserved_range: <::std::vec::Vec<descriptor_proto::ReservedRange> as FieldClone>::clone(&self.reserved_range),
            reserved_name: <::std::vec::Vec<::std::string::String> as FieldClone>::clone(&self.reserved_name),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for DescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::std::string::String> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, ::std::default::Default::default)?;
            }
            2 => {
                <::std::vec::Vec<FieldDescriptorProto> as FieldDeserFromIter<
                    tags::Message<FieldDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.field, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec<FieldDescriptorProto> as FieldDeserFromIter<
                    tags::Message<FieldDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, ::std::default::Default::default)?;
            }
            3 => {
                <::std::vec::Vec<DescriptorProto> as FieldDeserFromIter<
                    tags::Message<DescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.nested_type, field, ::std::default::Default::default)?;
            }
            4 => {
                <::std::vec::Vec<EnumDescriptorProto> as FieldDeserFromIter<
                    tags::Message<EnumDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec<descriptor_proto::ExtensionRange> as FieldDeserFromIter<
                    tags::Message<descriptor_proto::ExtensionRange>, 
                    tags::Repeated>>
                ::deser(&mut self.extension_range, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::vec::Vec<OneofDescriptorProto> as FieldDeserFromIter<
                    tags::Message<OneofDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.oneof_decl, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::option::Option<::std::boxed::Box<MessageOptions>> as FieldDeserFromIter<
                    tags::Message<MessageOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::vec::Vec<descriptor_proto::ReservedRange> as FieldDeserFromIter<
                    tags::Message<descriptor_proto::ReservedRange>, 
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for DescriptorProto {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::std::string::String> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::std::vec::Vec<FieldDescriptorProto> as FieldSer<
                tags::Message<FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.field, serializer, 2)?;
        <::std::vec::Vec<FieldDescriptorProto> as FieldSer<
                tags::Message<FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 6)?;
        <::std::vec::Vec<DescriptorProto> as FieldSer<
                tags::Message<DescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.nested_type, serializer, 3)?;
        <::std::vec::Vec<EnumDescriptorProto> as FieldSer<
                tags::Message<EnumDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 4)?;
        <::std::vec::Vec<descriptor_proto::ExtensionRange> as FieldSer<
                tags::Message<descriptor_proto::ExtensionRange>, 
                tags::Repeated>>
            ::ser(&self.extension_range, serializer, 5)?;
        <::std::vec::Vec<OneofDescriptorProto> as FieldSer<
                tags::Message<OneofDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.oneof_decl, serializer, 8)?;
        <::std::option::Option<::std::boxed::Box<MessageOptions>> as FieldSer<
                tags::Message<MessageOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 7)?;
        <::std::vec::Vec<descriptor_proto::ReservedRange> as FieldSer<
                tags::Message<descriptor_proto::ReservedRange>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl DescriptorProtoTrait for DescriptorProto {
    type FieldDescriptorProtoType = FieldDescriptorProto;
    type DescriptorProtoType = DescriptorProto;
    type EnumDescriptorProtoType = EnumDescriptorProto;
    type ExtensionRangeType = descriptor_proto::ExtensionRange;
    type OneofDescriptorProtoType = OneofDescriptorProto;
    type MessageOptionsType = MessageOptions;
    type ReservedRangeType = descriptor_proto::ReservedRange;
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FieldDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FieldDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::DescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::ExtensionRangeType>;
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::OneofDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::ReservedRangeType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn for_each_field<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType) {
        for item in (self.field).iter() {
            (f)(item);
        }
    }
    fn field_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>> {
        ::std::boxed::Box::new(self.field.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_> {
        self.field.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn for_each_nested_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::DescriptorProtoType) {
        for item in (self.nested_type).iter() {
            (f)(item);
        }
    }
    fn nested_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::DescriptorProtoType>> {
        ::std::boxed::Box::new(self.nested_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_> {
        self.nested_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumDescriptorProtoType) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumDescriptorProtoType>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_extension_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::ExtensionRangeType) {
        for item in (self.extension_range).iter() {
            (f)(item);
        }
    }
    fn extension_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ExtensionRangeType>> {
        ::std::boxed::Box::new(self.extension_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_> {
        self.extension_range.iter()
    }
    fn for_each_oneof_decl<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::OneofDescriptorProtoType) {
        for item in (self.oneof_decl).iter() {
            (f)(item);
        }
    }
    fn oneof_decl_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::OneofDescriptorProtoType>> {
        ::std::boxed::Box::new(self.oneof_decl.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_> {
        self.oneof_decl.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::MessageOptionsType> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::ReservedRangeType) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ReservedRangeType>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn reserved_name_iter(&self) -> Self::ReservedNameIter<'_> {
        self.reserved_name.iter().map(|v| v.as_ref())
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for DescriptorProto<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct DescriptorProtoBumpalo<'bump> {
    pub name: ::std::option::Option<::bumpalo::collections::String<'bump>>,
    pub field: ::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>>,
    pub extension: ::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>>,
    pub nested_type: ::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>>,
    pub enum_type: ::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>>,
    pub extension_range: ::bumpalo::collections::Vec<'bump, descriptor_proto::ExtensionRangeBumpalo<'bump>>,
    pub oneof_decl: ::bumpalo::collections::Vec<'bump, OneofDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, MessageOptionsBumpalo<'bump>>>,
    pub reserved_range: ::bumpalo::collections::Vec<'bump, descriptor_proto::ReservedRangeBumpalo<'bump>>,
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
impl<'bump> ::std::clone::Clone for DescriptorProtoBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldClone>::clone_in_bumpalo(&self.name, self.puroro_internal.bumpalo()),
            field: <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.field, self.puroro_internal.bumpalo()),
            extension: <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.extension, self.puroro_internal.bumpalo()),
            nested_type: <::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.nested_type, self.puroro_internal.bumpalo()),
            enum_type: <::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.enum_type, self.puroro_internal.bumpalo()),
            extension_range: <::bumpalo::collections::Vec<'bump, descriptor_proto::ExtensionRangeBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.extension_range, self.puroro_internal.bumpalo()),
            oneof_decl: <::bumpalo::collections::Vec<'bump, OneofDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.oneof_decl, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, MessageOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            reserved_range: <::bumpalo::collections::Vec<'bump, descriptor_proto::ReservedRangeBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.reserved_range, self.puroro_internal.bumpalo()),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<FieldDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.field, field, || FieldDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            6 => {
                <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<FieldDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, || FieldDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<DescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.nested_type, field, || DescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            4 => {
                <::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<EnumDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, || EnumDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            5 => {
                <::bumpalo::collections::Vec<'bump, descriptor_proto::ExtensionRangeBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<descriptor_proto::ExtensionRangeBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.extension_range, field, || descriptor_proto::ExtensionRangeBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            8 => {
                <::bumpalo::collections::Vec<'bump, OneofDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<OneofDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.oneof_decl, field, || OneofDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            7 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, MessageOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<MessageOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(MessageOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            9 => {
                <::bumpalo::collections::Vec<'bump, descriptor_proto::ReservedRangeBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<descriptor_proto::ReservedRangeBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_range, field, || descriptor_proto::ReservedRangeBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            10 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.reserved_name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for DescriptorProtoBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldSer<
                tags::String, 
                tags::Optional2>>
            ::ser(&self.name, serializer, 1)?;
        <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<FieldDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.field, serializer, 2)?;
        <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<FieldDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 6)?;
        <::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<DescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.nested_type, serializer, 3)?;
        <::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<EnumDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 4)?;
        <::bumpalo::collections::Vec<'bump, descriptor_proto::ExtensionRangeBumpalo<'bump>> as FieldSer<
                tags::Message<descriptor_proto::ExtensionRangeBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension_range, serializer, 5)?;
        <::bumpalo::collections::Vec<'bump, OneofDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<OneofDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.oneof_decl, serializer, 8)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, MessageOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<MessageOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 7)?;
        <::bumpalo::collections::Vec<'bump, descriptor_proto::ReservedRangeBumpalo<'bump>> as FieldSer<
                tags::Message<descriptor_proto::ReservedRangeBumpalo<'bump>>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> DescriptorProtoTrait for DescriptorProtoBumpalo<'bump> {
    type FieldDescriptorProtoType = FieldDescriptorProtoBumpalo<'bump>;
    type DescriptorProtoType = DescriptorProtoBumpalo<'bump>;
    type EnumDescriptorProtoType = EnumDescriptorProtoBumpalo<'bump>;
    type ExtensionRangeType = descriptor_proto::ExtensionRangeBumpalo<'bump>;
    type OneofDescriptorProtoType = OneofDescriptorProtoBumpalo<'bump>;
    type MessageOptionsType = MessageOptionsBumpalo<'bump>;
    type ReservedRangeType = descriptor_proto::ReservedRangeBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type FieldIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FieldDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FieldDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type NestedTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::DescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionRangeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::ExtensionRangeType>;
    #[cfg(feature = "puroro-nightly")]
    type OneofDeclIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::OneofDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedRangeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::ReservedRangeType>;
    #[cfg(feature = "puroro-nightly")]
    type ReservedNameIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn for_each_field<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType) {
        for item in (self.field).iter() {
            (f)(item);
        }
    }
    fn field_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>> {
        ::std::boxed::Box::new(self.field.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn field_iter(&self) -> Self::FieldIter<'_> {
        self.field.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn for_each_nested_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::DescriptorProtoType) {
        for item in (self.nested_type).iter() {
            (f)(item);
        }
    }
    fn nested_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::DescriptorProtoType>> {
        ::std::boxed::Box::new(self.nested_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn nested_type_iter(&self) -> Self::NestedTypeIter<'_> {
        self.nested_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumDescriptorProtoType) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumDescriptorProtoType>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_extension_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::ExtensionRangeType) {
        for item in (self.extension_range).iter() {
            (f)(item);
        }
    }
    fn extension_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ExtensionRangeType>> {
        ::std::boxed::Box::new(self.extension_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn extension_range_iter(&self) -> Self::ExtensionRangeIter<'_> {
        self.extension_range.iter()
    }
    fn for_each_oneof_decl<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::OneofDescriptorProtoType) {
        for item in (self.oneof_decl).iter() {
            (f)(item);
        }
    }
    fn oneof_decl_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::OneofDescriptorProtoType>> {
        ::std::boxed::Box::new(self.oneof_decl.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn oneof_decl_iter(&self) -> Self::OneofDeclIter<'_> {
        self.oneof_decl.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::MessageOptionsType> {
        self.options.as_deref()
    }
    fn for_each_reserved_range<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::ReservedRangeType) {
        for item in (self.reserved_range).iter() {
            (f)(item);
        }
    }
    fn reserved_range_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ReservedRangeType>> {
        ::std::boxed::Box::new(self.reserved_range.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn reserved_range_iter(&self) -> Self::ReservedRangeIter<'_> {
        self.reserved_range.iter()
    }
    fn for_each_reserved_name<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.reserved_name).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn reserved_name_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.reserved_name.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
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
pub mod descriptor_proto {
pub trait ReservedRangeTrait {
    fn start(&'_ self) -> ::std::option::Option<i32>;
    fn end(&'_ self) -> ::std::option::Option<i32>;
}

#[derive(Debug)]
pub struct ReservedRange {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for ReservedRange {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl ReservedRangeTrait for ReservedRange {
    fn start(&'_ self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ReservedRange<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
pub struct ReservedRangeBumpalo<'bump> {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ReservedRangeBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for ReservedRangeBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ReservedRangeTrait for ReservedRangeBumpalo<'bump> {
    fn start(&'_ self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
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
pub trait ExtensionRangeTrait {
    type ExtensionRangeOptionsType: self::super::ExtensionRangeOptionsTrait;
    fn start(&'_ self) -> ::std::option::Option<i32>;
    fn end(&'_ self) -> ::std::option::Option<i32>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::ExtensionRangeOptionsType>;
}

#[derive(Debug)]
pub struct ExtensionRange {
    pub start: ::std::option::Option<i32>,
    pub end: ::std::option::Option<i32>,
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for ExtensionRange {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl ExtensionRangeTrait for ExtensionRange {
    type ExtensionRangeOptionsType = super::ExtensionRangeOptions;
    fn start(&'_ self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::ExtensionRangeOptionsType> {
        self.options.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for ExtensionRange<> {
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
    puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct<'bump>,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeBumpalo<'bump> {
    pub fn new_in(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            start: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            end: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            options: ::puroro_internal::helpers::FieldNew::new_in_bumpalo(bump),
            puroro_internal: ::puroro_internal::helpers::InternalDataForBumpaloStruct::new(bump),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(super::ExtensionRangeOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for ExtensionRangeBumpalo<'bump> {
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ExtensionRangeTrait for ExtensionRangeBumpalo<'bump> {
    type ExtensionRangeOptionsType = super::ExtensionRangeOptionsBumpalo<'bump>;
    fn start(&'_ self) -> ::std::option::Option<i32> {
        self.start.clone()
    }
    fn end(&'_ self) -> ::std::option::Option<i32> {
        self.end.clone()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::ExtensionRangeOptionsType> {
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
} // mod descriptor_proto
pub trait FileDescriptorProtoTrait {
    type DescriptorProtoType: self::DescriptorProtoTrait;
    type EnumDescriptorProtoType: self::EnumDescriptorProtoTrait;
    type ServiceDescriptorProtoType: self::ServiceDescriptorProtoTrait;
    type FieldDescriptorProtoType: self::FieldDescriptorProtoTrait;
    type FileOptionsType: self::FileOptionsTrait;
    type SourceCodeInfoType: self::SourceCodeInfoTrait;
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a>: ::std::iter::Iterator<Item=&'a str>
        where str: 'a;
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a>: ::std::iter::Iterator<Item=i32>
        where i32: 'a;
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a>: ::std::iter::Iterator<Item=i32>
        where i32: 'a;
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a>: ::std::iter::Iterator<Item=&'a Self::DescriptorProtoType>
        where Self::DescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a>: ::std::iter::Iterator<Item=&'a Self::EnumDescriptorProtoType>
        where Self::EnumDescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a>: ::std::iter::Iterator<Item=&'a Self::ServiceDescriptorProtoType>
        where Self::ServiceDescriptorProtoType: 'a;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a>: ::std::iter::Iterator<Item=&'a Self::FieldDescriptorProtoType>
        where Self::FieldDescriptorProtoType: 'a;
    fn name(&'_ self) -> ::std::option::Option<&'_ str>;
    fn package(&'_ self) -> ::std::option::Option<&'_ str>;
    fn for_each_dependency<F>(&self, f: F)
    where
        F: FnMut(&'_ str);
    fn dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>>;
    #[cfg(feature = "puroro-nightly")]
    fn dependency_iter(&self) -> Self::DependencyIter<'_>;
    fn for_each_public_dependency<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn public_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    fn public_dependency_iter(&self) -> Self::PublicDependencyIter<'_>;
    fn for_each_weak_dependency<F>(&self, f: F)
    where
        F: FnMut(i32);
    fn weak_dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=i32>>;
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_>;
    fn for_each_message_type<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::DescriptorProtoType);
    fn message_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::DescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_>;
    fn for_each_enum_type<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::EnumDescriptorProtoType);
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_>;
    fn for_each_service<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::ServiceDescriptorProtoType);
    fn service_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ServiceDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_>;
    fn for_each_extension<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType);
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_>;
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::FileOptionsType>;
    fn source_code_info(&'_ self) -> ::std::option::Option<&'_ Self::SourceCodeInfoType>;
    fn syntax(&'_ self) -> ::std::option::Option<&'_ str>;
}

#[derive(Debug)]
pub struct FileDescriptorProto {
    pub name: ::std::option::Option<::std::string::String>,
    pub package: ::std::option::Option<::std::string::String>,
    pub dependency: ::std::vec::Vec<::std::string::String>,
    pub public_dependency: ::std::vec::Vec<i32>,
    pub weak_dependency: ::std::vec::Vec<i32>,
    pub message_type: ::std::vec::Vec<DescriptorProto>,
    pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,
    pub service: ::std::vec::Vec<ServiceDescriptorProto>,
    pub extension: ::std::vec::Vec<FieldDescriptorProto>,
    pub options: ::std::option::Option<::std::boxed::Box<FileOptions>>,
    pub source_code_info: ::std::option::Option<::std::boxed::Box<SourceCodeInfo>>,
    pub syntax: ::std::option::Option<::std::string::String>,
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
            message_type: <::std::vec::Vec<DescriptorProto> as FieldClone>::clone(&self.message_type),
            enum_type: <::std::vec::Vec<EnumDescriptorProto> as FieldClone>::clone(&self.enum_type),
            service: <::std::vec::Vec<ServiceDescriptorProto> as FieldClone>::clone(&self.service),
            extension: <::std::vec::Vec<FieldDescriptorProto> as FieldClone>::clone(&self.extension),
            options: <::std::option::Option<::std::boxed::Box<FileOptions>> as FieldClone>::clone(&self.options),
            source_code_info: <::std::option::Option<::std::boxed::Box<SourceCodeInfo>> as FieldClone>::clone(&self.source_code_info),
            syntax: <::std::option::Option<::std::string::String> as FieldClone>::clone(&self.syntax),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorProto {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
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
                <::std::vec::Vec<DescriptorProto> as FieldDeserFromIter<
                    tags::Message<DescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.message_type, field, ::std::default::Default::default)?;
            }
            5 => {
                <::std::vec::Vec<EnumDescriptorProto> as FieldDeserFromIter<
                    tags::Message<EnumDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, ::std::default::Default::default)?;
            }
            6 => {
                <::std::vec::Vec<ServiceDescriptorProto> as FieldDeserFromIter<
                    tags::Message<ServiceDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.service, field, ::std::default::Default::default)?;
            }
            7 => {
                <::std::vec::Vec<FieldDescriptorProto> as FieldDeserFromIter<
                    tags::Message<FieldDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, ::std::default::Default::default)?;
            }
            8 => {
                <::std::option::Option<::std::boxed::Box<FileOptions>> as FieldDeserFromIter<
                    tags::Message<FileOptions>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, ::std::default::Default::default)?;
            }
            9 => {
                <::std::option::Option<::std::boxed::Box<SourceCodeInfo>> as FieldDeserFromIter<
                    tags::Message<SourceCodeInfo>, 
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
        Ok(())
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

impl ::puroro_internal::ser::Serializable for FileDescriptorProto {
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
        <::std::vec::Vec<DescriptorProto> as FieldSer<
                tags::Message<DescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.message_type, serializer, 4)?;
        <::std::vec::Vec<EnumDescriptorProto> as FieldSer<
                tags::Message<EnumDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 5)?;
        <::std::vec::Vec<ServiceDescriptorProto> as FieldSer<
                tags::Message<ServiceDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.service, serializer, 6)?;
        <::std::vec::Vec<FieldDescriptorProto> as FieldSer<
                tags::Message<FieldDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 7)?;
        <::std::option::Option<::std::boxed::Box<FileOptions>> as FieldSer<
                tags::Message<FileOptions>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option<::std::boxed::Box<SourceCodeInfo>> as FieldSer<
                tags::Message<SourceCodeInfo>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileDescriptorProtoTrait for FileDescriptorProto {
    type DescriptorProtoType = DescriptorProto;
    type EnumDescriptorProtoType = EnumDescriptorProto;
    type ServiceDescriptorProtoType = ServiceDescriptorProto;
    type FieldDescriptorProtoType = FieldDescriptorProto;
    type FileOptionsType = FileOptions;
    type SourceCodeInfoType = SourceCodeInfo;
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::DescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::ServiceDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FieldDescriptorProtoType>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.package.as_deref()
    }
    fn for_each_dependency<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.dependency).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.dependency.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
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
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_> {
        self.weak_dependency.iter().cloned()
    }
    fn for_each_message_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::DescriptorProtoType) {
        for item in (self.message_type).iter() {
            (f)(item);
        }
    }
    fn message_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::DescriptorProtoType>> {
        ::std::boxed::Box::new(self.message_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_> {
        self.message_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumDescriptorProtoType) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumDescriptorProtoType>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_service<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::ServiceDescriptorProtoType) {
        for item in (self.service).iter() {
            (f)(item);
        }
    }
    fn service_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ServiceDescriptorProtoType>> {
        ::std::boxed::Box::new(self.service.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_> {
        self.service.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::FileOptionsType> {
        self.options.as_deref()
    }
    fn source_code_info(&'_ self) -> ::std::option::Option<&'_ Self::SourceCodeInfoType> {
        self.source_code_info.as_deref()
    }
    fn syntax(&'_ self) -> ::std::option::Option<&'_ str> {
        self.syntax.as_deref()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileDescriptorProto<> {
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
    pub message_type: ::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>>,
    pub enum_type: ::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>>,
    pub service: ::bumpalo::collections::Vec<'bump, ServiceDescriptorProtoBumpalo<'bump>>,
    pub extension: ::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>>,
    pub options: ::std::option::Option<::bumpalo::boxed::Box<'bump, FileOptionsBumpalo<'bump>>>,
    pub source_code_info: ::std::option::Option<::bumpalo::boxed::Box<'bump, SourceCodeInfoBumpalo<'bump>>>,
    pub syntax: ::std::option::Option<::bumpalo::collections::String<'bump>>,
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
            message_type: <::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.message_type, self.puroro_internal.bumpalo()),
            enum_type: <::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.enum_type, self.puroro_internal.bumpalo()),
            service: <::bumpalo::collections::Vec<'bump, ServiceDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.service, self.puroro_internal.bumpalo()),
            extension: <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.extension, self.puroro_internal.bumpalo()),
            options: <::std::option::Option<::bumpalo::boxed::Box<'bump, FileOptionsBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.options, self.puroro_internal.bumpalo()),
            source_code_info: <::std::option::Option<::bumpalo::boxed::Box<'bump, SourceCodeInfoBumpalo<'bump>>> as FieldClone>::clone_in_bumpalo(&self.source_code_info, self.puroro_internal.bumpalo()),
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
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.name, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            2 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.package, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            3 => {
                <::bumpalo::collections::Vec<'bump, ::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Repeated>>
                ::deser(&mut self.dependency, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
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
                <::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<DescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.message_type, field, || DescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            5 => {
                <::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<EnumDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.enum_type, field, || EnumDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            6 => {
                <::bumpalo::collections::Vec<'bump, ServiceDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<ServiceDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.service, field, || ServiceDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            7 => {
                <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<FieldDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.extension, field, || FieldDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            8 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, FileOptionsBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<FileOptionsBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.options, field, || ::bumpalo::boxed::Box::new_in(FileOptionsBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            9 => {
                <::std::option::Option<::bumpalo::boxed::Box<'bump, SourceCodeInfoBumpalo<'bump>>> as FieldDeserFromIter<
                    tags::Message<SourceCodeInfoBumpalo<'bump>>, 
                    tags::Optional2>>
                ::deser(&mut self.source_code_info, field, || ::bumpalo::boxed::Box::new_in(SourceCodeInfoBumpalo::new_in(self.puroro_internal.bumpalo()), self.puroro_internal.bumpalo()))?;
            }
            12 => {
                <::std::option::Option<::bumpalo::collections::String<'bump>> as FieldDeserFromIter<
                    tags::String, 
                    tags::Optional2>>
                ::deser(&mut self.syntax, field, || ::bumpalo::collections::String::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for FileDescriptorProtoBumpalo<'bump> {
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
        <::bumpalo::collections::Vec<'bump, DescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<DescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.message_type, serializer, 4)?;
        <::bumpalo::collections::Vec<'bump, EnumDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<EnumDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.enum_type, serializer, 5)?;
        <::bumpalo::collections::Vec<'bump, ServiceDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<ServiceDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.service, serializer, 6)?;
        <::bumpalo::collections::Vec<'bump, FieldDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<FieldDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.extension, serializer, 7)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, FileOptionsBumpalo<'bump>>> as FieldSer<
                tags::Message<FileOptionsBumpalo<'bump>>, 
                tags::Optional2>>
            ::ser(&self.options, serializer, 8)?;
        <::std::option::Option<::bumpalo::boxed::Box<'bump, SourceCodeInfoBumpalo<'bump>>> as FieldSer<
                tags::Message<SourceCodeInfoBumpalo<'bump>>, 
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
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorProtoTrait for FileDescriptorProtoBumpalo<'bump> {
    type DescriptorProtoType = DescriptorProtoBumpalo<'bump>;
    type EnumDescriptorProtoType = EnumDescriptorProtoBumpalo<'bump>;
    type ServiceDescriptorProtoType = ServiceDescriptorProtoBumpalo<'bump>;
    type FieldDescriptorProtoType = FieldDescriptorProtoBumpalo<'bump>;
    type FileOptionsType = FileOptionsBumpalo<'bump>;
    type SourceCodeInfoType = SourceCodeInfoBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type DependencyIter<'a> = impl ::std::iter::Iterator<Item = &'a str>;
    #[cfg(feature = "puroro-nightly")]
    type PublicDependencyIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type WeakDependencyIter<'a> = impl ::std::iter::Iterator<Item = i32>;
    #[cfg(feature = "puroro-nightly")]
    type MessageTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::DescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type EnumTypeIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::EnumDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ServiceIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::ServiceDescriptorProtoType>;
    #[cfg(feature = "puroro-nightly")]
    type ExtensionIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FieldDescriptorProtoType>;
    fn name(&'_ self) -> ::std::option::Option<&'_ str> {
        self.name.as_deref()
    }
    fn package(&'_ self) -> ::std::option::Option<&'_ str> {
        self.package.as_deref()
    }
    fn for_each_dependency<F>(&self, mut f: F)
    where
        F: FnMut(&'_ str) {
        for item in (self.dependency).iter().map(|v| v.as_ref()) {
            (f)(item);
        }
    }
    fn dependency_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ str>> {
        ::std::boxed::Box::new(self.dependency.iter().map(|v| v.as_ref()))
    }
    #[cfg(feature = "puroro-nightly")]
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
    #[cfg(feature = "puroro-nightly")]
    fn weak_dependency_iter(&self) -> Self::WeakDependencyIter<'_> {
        self.weak_dependency.iter().cloned()
    }
    fn for_each_message_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::DescriptorProtoType) {
        for item in (self.message_type).iter() {
            (f)(item);
        }
    }
    fn message_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::DescriptorProtoType>> {
        ::std::boxed::Box::new(self.message_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn message_type_iter(&self) -> Self::MessageTypeIter<'_> {
        self.message_type.iter()
    }
    fn for_each_enum_type<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::EnumDescriptorProtoType) {
        for item in (self.enum_type).iter() {
            (f)(item);
        }
    }
    fn enum_type_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::EnumDescriptorProtoType>> {
        ::std::boxed::Box::new(self.enum_type.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn enum_type_iter(&self) -> Self::EnumTypeIter<'_> {
        self.enum_type.iter()
    }
    fn for_each_service<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::ServiceDescriptorProtoType) {
        for item in (self.service).iter() {
            (f)(item);
        }
    }
    fn service_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::ServiceDescriptorProtoType>> {
        ::std::boxed::Box::new(self.service.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn service_iter(&self) -> Self::ServiceIter<'_> {
        self.service.iter()
    }
    fn for_each_extension<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FieldDescriptorProtoType) {
        for item in (self.extension).iter() {
            (f)(item);
        }
    }
    fn extension_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FieldDescriptorProtoType>> {
        ::std::boxed::Box::new(self.extension.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn extension_iter(&self) -> Self::ExtensionIter<'_> {
        self.extension.iter()
    }
    fn options(&'_ self) -> ::std::option::Option<&'_ Self::FileOptionsType> {
        self.options.as_deref()
    }
    fn source_code_info(&'_ self) -> ::std::option::Option<&'_ Self::SourceCodeInfoType> {
        self.source_code_info.as_deref()
    }
    fn syntax(&'_ self) -> ::std::option::Option<&'_ str> {
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
pub trait FileDescriptorSetTrait {
    type FileDescriptorProtoType: self::FileDescriptorProtoTrait;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a>: ::std::iter::Iterator<Item=&'a Self::FileDescriptorProtoType>
        where Self::FileDescriptorProtoType: 'a;
    fn for_each_file<F>(&self, f: F)
    where
        F: FnMut(&'_ Self::FileDescriptorProtoType);
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileDescriptorProtoType>>;
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_>;
}

#[derive(Debug)]
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

impl ::std::clone::Clone for FileDescriptorSet {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file: <::std::vec::Vec<FileDescriptorProto> as FieldClone>::clone(&self.file),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorSet {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::std::vec::Vec<FileDescriptorProto> as FieldDeserFromIter<
                    tags::Message<FileDescriptorProto>, 
                    tags::Repeated>>
                ::deser(&mut self.file, field, ::std::default::Default::default)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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

impl ::puroro_internal::ser::Serializable for FileDescriptorSet {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::std::vec::Vec<FileDescriptorProto> as FieldSer<
                tags::Message<FileDescriptorProto>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 1)?;
        Ok(())
    }
}

impl ::puroro::Serializable for FileDescriptorSet {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}

impl FileDescriptorSetTrait for FileDescriptorSet {
    type FileDescriptorProtoType = FileDescriptorProto;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FileDescriptorProtoType>;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FileDescriptorProtoType) {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileDescriptorProtoType>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
    #[cfg(feature = "puroro-nightly")]
    fn file_iter(&self) -> Self::FileIter<'_> {
        self.file.iter()
    }
}
impl<'a> ::puroro_internal::helpers::FieldNew<'a> for FileDescriptorSet<> {
    fn new() -> Self {
        Default::default()
    }
}
#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug)]
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
impl<'bump> ::std::clone::Clone for FileDescriptorSetBumpalo<'bump> {
    fn clone(&self) -> Self {
        use ::puroro_internal::helpers::FieldClone;
        use ::puroro::InternalData;
        Self {
            file: <::bumpalo::collections::Vec<'bump, FileDescriptorProtoBumpalo<'bump>> as FieldClone>::clone_in_bumpalo(&self.file, self.puroro_internal.bumpalo()),
        puroro_internal: self.puroro_internal.clone(),
        }
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro_internal::deser::DeserializableMessageFromIter for FileDescriptorSetBumpalo<'bump> {
    fn met_field<'a, 'b, I>(
        &mut self,
        field: ::puroro_internal::types::FieldData<
            &'a mut ::puroro_internal::deser::BytesIter<'b, I>>,
        field_number: usize,
    ) -> ::puroro::Result<()> 
    where
        I: Iterator<Item = ::std::io::Result<u8>>
    {
        use ::puroro_internal::helpers::FieldDeserFromIter;
        use ::puroro::InternalData;
        use ::puroro_internal::tags;
        use ::std::convert::TryInto;
        match field_number {
            1 => {
                <::bumpalo::collections::Vec<'bump, FileDescriptorProtoBumpalo<'bump>> as FieldDeserFromIter<
                    tags::Message<FileDescriptorProtoBumpalo<'bump>>, 
                    tags::Repeated>>
                ::deser(&mut self.file, field, || FileDescriptorProtoBumpalo::new_in(self.puroro_internal.bumpalo()))?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(())
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
impl<'bump> ::puroro_internal::ser::Serializable for FileDescriptorSetBumpalo<'bump> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        use ::puroro_internal::helpers::FieldSer;
        use ::puroro_internal::tags;
        <::bumpalo::collections::Vec<'bump, FileDescriptorProtoBumpalo<'bump>> as FieldSer<
                tags::Message<FileDescriptorProtoBumpalo<'bump>>, 
                tags::Repeated>>
            ::ser(&self.file, serializer, 1)?;
        Ok(())
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ::puroro::Serializable for FileDescriptorSetBumpalo<'bump> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::Serializable>::serialize(self, &mut serializer)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FileDescriptorSetTrait for FileDescriptorSetBumpalo<'bump> {
    type FileDescriptorProtoType = FileDescriptorProtoBumpalo<'bump>;
    #[cfg(feature = "puroro-nightly")]
    type FileIter<'a> = impl ::std::iter::Iterator<Item = &'a Self::FileDescriptorProtoType>;
    fn for_each_file<F>(&self, mut f: F)
    where
        F: FnMut(&'_ Self::FileDescriptorProtoType) {
        for item in (self.file).iter() {
            (f)(item);
        }
    }
    fn file_boxed_iter(&self)
        -> ::std::boxed::Box<dyn '_ + Iterator<Item=&'_ Self::FileDescriptorProtoType>> {
        ::std::boxed::Box::new(self.file.iter())
    }
    #[cfg(feature = "puroro-nightly")]
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

pub mod compiler;
