#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct GeneratedCodeInfo<'slice, S> {
    annotation: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> GeneratedCodeInfo<'slice, S> {
    fn new() -> Self {
        Self {
            annotation: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> GeneratedCodeInfo<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            annotation: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> GeneratedCodeInfo<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            annotation: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for GeneratedCodeInfo<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            annotation: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.annotation),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for GeneratedCodeInfo<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for GeneratedCodeInfo<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for GeneratedCodeInfo<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for GeneratedCodeInfo<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::generated_code_info::Annotation::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.annotation, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for GeneratedCodeInfo<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for GeneratedCodeInfo<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::GeneratedCodeInfoTrait for GeneratedCodeInfo<'slice, S> {
    type AnnotationElement<'this> where Self: 'this = self::generated_code_info::Annotation::<'slice, &'slice [u8]>;
    type AnnotationRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::generated_code_info::Annotation::<'slice, &'slice [u8]>>
        >;
    fn annotation<'this>(&'this self) -> Self::AnnotationRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.annotation.as_ref(),
            1,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for GeneratedCodeInfo<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::GeneratedCodeInfoTag> for GeneratedCodeInfo<'slice, S> {
}

pub mod generated_code_info {
#[derive(Debug)]
pub struct Annotation<'slice, S> {
    path: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    source_file: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    begin: ::std::option::Option::<i32>,
    end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> Annotation<'slice, S> {
    fn new() -> Self {
        Self {
            path: ::puroro_internal::FieldNew::new(),
            source_file: ::puroro_internal::FieldNew::new(),
            begin: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> Annotation<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            path: ::puroro_internal::FieldNew::new(),
            source_file: ::puroro_internal::FieldNew::new(),
            begin: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> Annotation<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            path: ::puroro_internal::FieldNew::new(),
            source_file: ::puroro_internal::FieldNew::new(),
            begin: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for Annotation<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            path: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.path),
            source_file: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.source_file),
            begin: <::std::option::Option::<i32> as Clone>::clone(&self.begin),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for Annotation<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for Annotation<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for Annotation<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for Annotation<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.path, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.source_file, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.begin, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for Annotation<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for Annotation<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::generated_code_info::AnnotationTrait for Annotation<'slice, S> {
    type PathRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Int32
        >;
    fn path<'this>(&'this self) -> Self::PathRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.path.as_ref(),
            1,
            &self.puroro_internal,
        )
    }
    fn source_file<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.source_file.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn begin<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.begin.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for Annotation<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::generated_code_info::AnnotationTag> for Annotation<'slice, S> {
}

} // mod generated_code_info
#[derive(Debug)]
pub struct SourceCodeInfo<'slice, S> {
    location: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> SourceCodeInfo<'slice, S> {
    fn new() -> Self {
        Self {
            location: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> SourceCodeInfo<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            location: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> SourceCodeInfo<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            location: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for SourceCodeInfo<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            location: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.location),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for SourceCodeInfo<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for SourceCodeInfo<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for SourceCodeInfo<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for SourceCodeInfo<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::source_code_info::Location::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.location, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for SourceCodeInfo<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for SourceCodeInfo<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::SourceCodeInfoTrait for SourceCodeInfo<'slice, S> {
    type LocationElement<'this> where Self: 'this = self::source_code_info::Location::<'slice, &'slice [u8]>;
    type LocationRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::source_code_info::Location::<'slice, &'slice [u8]>>
        >;
    fn location<'this>(&'this self) -> Self::LocationRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.location.as_ref(),
            1,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for SourceCodeInfo<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::SourceCodeInfoTag> for SourceCodeInfo<'slice, S> {
}

pub mod source_code_info {
#[derive(Debug)]
pub struct Location<'slice, S> {
    path: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    span: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    leading_comments: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    trailing_comments: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    leading_detached_comments: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> Location<'slice, S> {
    fn new() -> Self {
        Self {
            path: ::puroro_internal::FieldNew::new(),
            span: ::puroro_internal::FieldNew::new(),
            leading_comments: ::puroro_internal::FieldNew::new(),
            trailing_comments: ::puroro_internal::FieldNew::new(),
            leading_detached_comments: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> Location<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            path: ::puroro_internal::FieldNew::new(),
            span: ::puroro_internal::FieldNew::new(),
            leading_comments: ::puroro_internal::FieldNew::new(),
            trailing_comments: ::puroro_internal::FieldNew::new(),
            leading_detached_comments: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> Location<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            path: ::puroro_internal::FieldNew::new(),
            span: ::puroro_internal::FieldNew::new(),
            leading_comments: ::puroro_internal::FieldNew::new(),
            trailing_comments: ::puroro_internal::FieldNew::new(),
            leading_detached_comments: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for Location<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            path: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.path),
            span: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.span),
            leading_comments: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.leading_comments),
            trailing_comments: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.trailing_comments),
            leading_detached_comments: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.leading_detached_comments),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for Location<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for Location<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for Location<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for Location<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.path, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.span, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.leading_comments, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.trailing_comments, field, slice_from_this_field, enclosing_slice)?;
            }
            
            6 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.leading_detached_comments, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for Location<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for Location<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::source_code_info::LocationTrait for Location<'slice, S> {
    type PathRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Int32
        >;
    fn path<'this>(&'this self) -> Self::PathRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.path.as_ref(),
            1,
            &self.puroro_internal,
        )
    }
    type SpanRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Int32
        >;
    fn span<'this>(&'this self) -> Self::SpanRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.span.as_ref(),
            2,
            &self.puroro_internal,
        )
    }
    fn leading_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.leading_comments.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn trailing_comments<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.trailing_comments.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type LeadingDetachedCommentsRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::String
        >;
    fn leading_detached_comments<'this>(&'this self) -> Self::LeadingDetachedCommentsRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.leading_detached_comments.as_ref(),
            6,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for Location<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::source_code_info::LocationTag> for Location<'slice, S> {
}

} // mod source_code_info
#[derive(Debug)]
pub struct UninterpretedOption<'slice, S> {
    name: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    identifier_value: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    positive_int_value: ::std::option::Option::<u64>,
    negative_int_value: ::std::option::Option::<i64>,
    double_value: ::std::option::Option::<f64>,
    string_value: ::std::option::Option::<::std::borrow::Cow<'slice, [u8]>>,
    aggregate_value: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> UninterpretedOption<'slice, S> {
    fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            identifier_value: ::puroro_internal::FieldNew::new(),
            positive_int_value: ::puroro_internal::FieldNew::new(),
            negative_int_value: ::puroro_internal::FieldNew::new(),
            double_value: ::puroro_internal::FieldNew::new(),
            string_value: ::puroro_internal::FieldNew::new(),
            aggregate_value: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> UninterpretedOption<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            identifier_value: ::puroro_internal::FieldNew::new(),
            positive_int_value: ::puroro_internal::FieldNew::new(),
            negative_int_value: ::puroro_internal::FieldNew::new(),
            double_value: ::puroro_internal::FieldNew::new(),
            string_value: ::puroro_internal::FieldNew::new(),
            aggregate_value: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> UninterpretedOption<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            identifier_value: ::puroro_internal::FieldNew::new(),
            positive_int_value: ::puroro_internal::FieldNew::new(),
            negative_int_value: ::puroro_internal::FieldNew::new(),
            double_value: ::puroro_internal::FieldNew::new(),
            string_value: ::puroro_internal::FieldNew::new(),
            aggregate_value: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for UninterpretedOption<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.name),
            identifier_value: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.identifier_value),
            positive_int_value: <::std::option::Option::<u64> as Clone>::clone(&self.positive_int_value),
            negative_int_value: <::std::option::Option::<i64> as Clone>::clone(&self.negative_int_value),
            double_value: <::std::option::Option::<f64> as Clone>::clone(&self.double_value),
            string_value: <::std::option::Option::<::std::borrow::Cow<'slice, [u8]>> as Clone>::clone(&self.string_value),
            aggregate_value: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.aggregate_value),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for UninterpretedOption<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for UninterpretedOption<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for UninterpretedOption<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for UninterpretedOption<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            2 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::uninterpreted_option::NamePart::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.identifier_value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<u64> as FieldMergeFromSlice<
                    tags::UInt64, 
                    tags::Optional2>>
                ::merge(&mut self.positive_int_value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            5 => {
                <::std::option::Option::<i64> as FieldMergeFromSlice<
                    tags::Int64, 
                    tags::Optional2>>
                ::merge(&mut self.negative_int_value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            6 => {
                <::std::option::Option::<f64> as FieldMergeFromSlice<
                    tags::Double, 
                    tags::Optional2>>
                ::merge(&mut self.double_value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            7 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, [u8]>> as FieldMergeFromSlice<
                    tags::Bytes, 
                    tags::Optional2>>
                ::merge(&mut self.string_value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            8 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.aggregate_value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for UninterpretedOption<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for UninterpretedOption<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::UninterpretedOptionTrait for UninterpretedOption<'slice, S> {
    type NameElement<'this> where Self: 'this = self::uninterpreted_option::NamePart::<'slice, &'slice [u8]>;
    type NameRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::uninterpreted_option::NamePart::<'slice, &'slice [u8]>>
        >;
    fn name<'this>(&'this self) -> Self::NameRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.name.as_ref(),
            2,
            &self.puroro_internal,
        )
    }
    fn identifier_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.identifier_value.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
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
        self.string_value.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn aggregate_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.aggregate_value.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for UninterpretedOption<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::UninterpretedOptionTag> for UninterpretedOption<'slice, S> {
}

pub mod uninterpreted_option {
#[derive(Debug)]
pub struct NamePart<'slice, S> {
    name_part: ::std::borrow::Cow<'slice, str>,
    is_extension: bool,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> NamePart<'slice, S> {
    fn new() -> Self {
        Self {
            name_part: ::puroro_internal::FieldNew::new(),
            is_extension: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> NamePart<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name_part: ::puroro_internal::FieldNew::new(),
            is_extension: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> NamePart<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name_part: ::puroro_internal::FieldNew::new(),
            is_extension: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for NamePart<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name_part: <::std::borrow::Cow<'slice, str> as Clone>::clone(&self.name_part),
            is_extension: <bool as Clone>::clone(&self.is_extension),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for NamePart<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for NamePart<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for NamePart<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for NamePart<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::borrow::Cow<'slice, str> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Required>>
                ::merge(&mut self.name_part, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <bool as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Required>>
                ::merge(&mut self.is_extension, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for NamePart<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for NamePart<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::uninterpreted_option::NamePartTrait for NamePart<'slice, S> {
    fn name_part<'this>(&'this self) -> ::std::borrow::Cow::<'this, str> {
        ::std::borrow::Cow::Borrowed(self.name_part.as_ref())
    }
    fn is_extension<'this>(&'this self) -> bool {
        self.is_extension.clone()
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for NamePart<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::uninterpreted_option::NamePartTag> for NamePart<'slice, S> {
}

} // mod uninterpreted_option
#[derive(Debug)]
pub struct MethodOptions<'slice, S> {
    deprecated: ::std::option::Option::<bool>,
    idempotency_level: ::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel>,
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> MethodOptions<'slice, S> {
    fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            idempotency_level: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> MethodOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            idempotency_level: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> MethodOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            idempotency_level: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for MethodOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            idempotency_level: <::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> as Clone>::clone(&self.idempotency_level),
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for MethodOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for MethodOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for MethodOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for MethodOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            33 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, slice_from_this_field, enclosing_slice)?;
            }
            
            34 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> as FieldMergeFromSlice<
                    tags::Enum2::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel>, 
                    tags::Optional2>>
                ::merge(&mut self.idempotency_level, field, slice_from_this_field, enclosing_slice)?;
            }
            
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for MethodOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for MethodOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::MethodOptionsTrait for MethodOptions<'slice, S> {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    fn idempotency_level<'this>(&'this self) -> ::std::option::Option::<super::super::super::enums::google::protobuf::method_options::IdempotencyLevel> {
        self.idempotency_level.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for MethodOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MethodOptionsTag> for MethodOptions<'slice, S> {
}

pub mod method_options {
} // mod method_options
#[derive(Debug)]
pub struct ServiceOptions<'slice, S> {
    deprecated: ::std::option::Option::<bool>,
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> ServiceOptions<'slice, S> {
    fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> ServiceOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> ServiceOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for ServiceOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for ServiceOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ServiceOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for ServiceOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for ServiceOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            33 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, slice_from_this_field, enclosing_slice)?;
            }
            
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for ServiceOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for ServiceOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::ServiceOptionsTrait for ServiceOptions<'slice, S> {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for ServiceOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ServiceOptionsTag> for ServiceOptions<'slice, S> {
}

#[derive(Debug)]
pub struct EnumValueOptions<'slice, S> {
    deprecated: ::std::option::Option::<bool>,
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> EnumValueOptions<'slice, S> {
    fn new() -> Self {
        Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> EnumValueOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> EnumValueOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for EnumValueOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for EnumValueOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumValueOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for EnumValueOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for EnumValueOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, slice_from_this_field, enclosing_slice)?;
            }
            
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for EnumValueOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for EnumValueOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::EnumValueOptionsTrait for EnumValueOptions<'slice, S> {
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for EnumValueOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumValueOptionsTag> for EnumValueOptions<'slice, S> {
}

#[derive(Debug)]
pub struct EnumOptions<'slice, S> {
    allow_alias: ::std::option::Option::<bool>,
    deprecated: ::std::option::Option::<bool>,
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> EnumOptions<'slice, S> {
    fn new() -> Self {
        Self {
            allow_alias: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> EnumOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            allow_alias: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> EnumOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            allow_alias: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for EnumOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            allow_alias: <::std::option::Option::<bool> as Clone>::clone(&self.allow_alias),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for EnumOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for EnumOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for EnumOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            2 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.allow_alias, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, slice_from_this_field, enclosing_slice)?;
            }
            
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for EnumOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for EnumOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::EnumOptionsTrait for EnumOptions<'slice, S> {
    fn allow_alias<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.allow_alias.clone()
    }
    fn deprecated<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.deprecated.clone()
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for EnumOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumOptionsTag> for EnumOptions<'slice, S> {
}

#[derive(Debug)]
pub struct OneofOptions<'slice, S> {
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> OneofOptions<'slice, S> {
    fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> OneofOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> OneofOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for OneofOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for OneofOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for OneofOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for OneofOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for OneofOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for OneofOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for OneofOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::OneofOptionsTrait for OneofOptions<'slice, S> {
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for OneofOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::OneofOptionsTag> for OneofOptions<'slice, S> {
}

#[derive(Debug)]
pub struct FieldOptions<'slice, S> {
    ctype: ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype>,
    packed: ::std::option::Option::<bool>,
    jstype: ::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype>,
    lazy: ::std::option::Option::<bool>,
    deprecated: ::std::option::Option::<bool>,
    weak: ::std::option::Option::<bool>,
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> FieldOptions<'slice, S> {
    fn new() -> Self {
        Self {
            ctype: ::puroro_internal::FieldNew::new(),
            packed: ::puroro_internal::FieldNew::new(),
            jstype: ::puroro_internal::FieldNew::new(),
            lazy: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            weak: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> FieldOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            ctype: ::puroro_internal::FieldNew::new(),
            packed: ::puroro_internal::FieldNew::new(),
            jstype: ::puroro_internal::FieldNew::new(),
            lazy: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            weak: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> FieldOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            ctype: ::puroro_internal::FieldNew::new(),
            packed: ::puroro_internal::FieldNew::new(),
            jstype: ::puroro_internal::FieldNew::new(),
            lazy: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            weak: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for FieldOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            ctype: <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype> as Clone>::clone(&self.ctype),
            packed: <::std::option::Option::<bool> as Clone>::clone(&self.packed),
            jstype: <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype> as Clone>::clone(&self.jstype),
            lazy: <::std::option::Option::<bool> as Clone>::clone(&self.lazy),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            weak: <::std::option::Option::<bool> as Clone>::clone(&self.weak),
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for FieldOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FieldOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for FieldOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for FieldOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Ctype> as FieldMergeFromSlice<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_options::Ctype>, 
                    tags::Optional2>>
                ::merge(&mut self.ctype, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.packed, field, slice_from_this_field, enclosing_slice)?;
            }
            
            6 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_options::Jstype> as FieldMergeFromSlice<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_options::Jstype>, 
                    tags::Optional2>>
                ::merge(&mut self.jstype, field, slice_from_this_field, enclosing_slice)?;
            }
            
            5 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.lazy, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, slice_from_this_field, enclosing_slice)?;
            }
            
            10 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.weak, field, slice_from_this_field, enclosing_slice)?;
            }
            
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for FieldOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for FieldOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::FieldOptionsTrait for FieldOptions<'slice, S> {
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
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for FieldOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FieldOptionsTag> for FieldOptions<'slice, S> {
}

pub mod field_options {
} // mod field_options
#[derive(Debug)]
pub struct MessageOptions<'slice, S> {
    message_set_wire_format: ::std::option::Option::<bool>,
    no_standard_descriptor_accessor: ::std::option::Option::<bool>,
    deprecated: ::std::option::Option::<bool>,
    map_entry: ::std::option::Option::<bool>,
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> MessageOptions<'slice, S> {
    fn new() -> Self {
        Self {
            message_set_wire_format: ::puroro_internal::FieldNew::new(),
            no_standard_descriptor_accessor: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            map_entry: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> MessageOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            message_set_wire_format: ::puroro_internal::FieldNew::new(),
            no_standard_descriptor_accessor: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            map_entry: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> MessageOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            message_set_wire_format: ::puroro_internal::FieldNew::new(),
            no_standard_descriptor_accessor: ::puroro_internal::FieldNew::new(),
            deprecated: ::puroro_internal::FieldNew::new(),
            map_entry: ::puroro_internal::FieldNew::new(),
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for MessageOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            message_set_wire_format: <::std::option::Option::<bool> as Clone>::clone(&self.message_set_wire_format),
            no_standard_descriptor_accessor: <::std::option::Option::<bool> as Clone>::clone(&self.no_standard_descriptor_accessor),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            map_entry: <::std::option::Option::<bool> as Clone>::clone(&self.map_entry),
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for MessageOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for MessageOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for MessageOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for MessageOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.message_set_wire_format, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.no_standard_descriptor_accessor, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, slice_from_this_field, enclosing_slice)?;
            }
            
            7 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.map_entry, field, slice_from_this_field, enclosing_slice)?;
            }
            
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for MessageOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for MessageOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::MessageOptionsTrait for MessageOptions<'slice, S> {
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
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for MessageOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MessageOptionsTag> for MessageOptions<'slice, S> {
}

#[derive(Debug)]
pub struct FileOptions<'slice, S> {
    java_package: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    java_outer_classname: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    java_multiple_files: ::std::option::Option::<bool>,
    java_generate_equals_and_hash: ::std::option::Option::<bool>,
    java_string_check_utf8: ::std::option::Option::<bool>,
    optimize_for: ::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode>,
    go_package: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    cc_generic_services: ::std::option::Option::<bool>,
    java_generic_services: ::std::option::Option::<bool>,
    py_generic_services: ::std::option::Option::<bool>,
    php_generic_services: ::std::option::Option::<bool>,
    deprecated: ::std::option::Option::<bool>,
    cc_enable_arenas: ::std::option::Option::<bool>,
    objc_class_prefix: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    csharp_namespace: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    swift_prefix: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    php_class_prefix: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    php_namespace: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    php_metadata_namespace: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    ruby_package: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> FileOptions<'slice, S> {
    fn new() -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> FileOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> FileOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for FileOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            java_package: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.java_package),
            java_outer_classname: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.java_outer_classname),
            java_multiple_files: <::std::option::Option::<bool> as Clone>::clone(&self.java_multiple_files),
            java_generate_equals_and_hash: <::std::option::Option::<bool> as Clone>::clone(&self.java_generate_equals_and_hash),
            java_string_check_utf8: <::std::option::Option::<bool> as Clone>::clone(&self.java_string_check_utf8),
            optimize_for: <::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode> as Clone>::clone(&self.optimize_for),
            go_package: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.go_package),
            cc_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.cc_generic_services),
            java_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.java_generic_services),
            py_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.py_generic_services),
            php_generic_services: <::std::option::Option::<bool> as Clone>::clone(&self.php_generic_services),
            deprecated: <::std::option::Option::<bool> as Clone>::clone(&self.deprecated),
            cc_enable_arenas: <::std::option::Option::<bool> as Clone>::clone(&self.cc_enable_arenas),
            objc_class_prefix: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.objc_class_prefix),
            csharp_namespace: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.csharp_namespace),
            swift_prefix: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.swift_prefix),
            php_class_prefix: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.php_class_prefix),
            php_namespace: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.php_namespace),
            php_metadata_namespace: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.php_metadata_namespace),
            ruby_package: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.ruby_package),
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for FileOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FileOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for FileOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for FileOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.java_package, field, slice_from_this_field, enclosing_slice)?;
            }
            
            8 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.java_outer_classname, field, slice_from_this_field, enclosing_slice)?;
            }
            
            10 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_multiple_files, field, slice_from_this_field, enclosing_slice)?;
            }
            
            20 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_generate_equals_and_hash, field, slice_from_this_field, enclosing_slice)?;
            }
            
            27 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_string_check_utf8, field, slice_from_this_field, enclosing_slice)?;
            }
            
            9 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::file_options::OptimizeMode> as FieldMergeFromSlice<
                    tags::Enum2::<super::super::super::enums::google::protobuf::file_options::OptimizeMode>, 
                    tags::Optional2>>
                ::merge(&mut self.optimize_for, field, slice_from_this_field, enclosing_slice)?;
            }
            
            11 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.go_package, field, slice_from_this_field, enclosing_slice)?;
            }
            
            16 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.cc_generic_services, field, slice_from_this_field, enclosing_slice)?;
            }
            
            17 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.java_generic_services, field, slice_from_this_field, enclosing_slice)?;
            }
            
            18 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.py_generic_services, field, slice_from_this_field, enclosing_slice)?;
            }
            
            42 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.php_generic_services, field, slice_from_this_field, enclosing_slice)?;
            }
            
            23 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.deprecated, field, slice_from_this_field, enclosing_slice)?;
            }
            
            31 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.cc_enable_arenas, field, slice_from_this_field, enclosing_slice)?;
            }
            
            36 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.objc_class_prefix, field, slice_from_this_field, enclosing_slice)?;
            }
            
            37 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.csharp_namespace, field, slice_from_this_field, enclosing_slice)?;
            }
            
            39 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.swift_prefix, field, slice_from_this_field, enclosing_slice)?;
            }
            
            40 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_class_prefix, field, slice_from_this_field, enclosing_slice)?;
            }
            
            41 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_namespace, field, slice_from_this_field, enclosing_slice)?;
            }
            
            44 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.php_metadata_namespace, field, slice_from_this_field, enclosing_slice)?;
            }
            
            45 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.ruby_package, field, slice_from_this_field, enclosing_slice)?;
            }
            
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for FileOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for FileOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::FileOptionsTrait for FileOptions<'slice, S> {
    fn java_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.java_package.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn java_outer_classname<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.java_outer_classname.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
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
        self.go_package.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
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
        self.objc_class_prefix.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn csharp_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.csharp_namespace.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn swift_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.swift_prefix.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn php_class_prefix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.php_class_prefix.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn php_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.php_namespace.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn php_metadata_namespace<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.php_metadata_namespace.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn ruby_package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.ruby_package.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for FileOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileOptionsTag> for FileOptions<'slice, S> {
}

pub mod file_options {
} // mod file_options
#[derive(Debug)]
pub struct MethodDescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    input_type: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    output_type: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    client_streaming: ::std::option::Option::<bool>,
    server_streaming: ::std::option::Option::<bool>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> MethodDescriptorProto<'slice, S> {
    fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            input_type: ::puroro_internal::FieldNew::new(),
            output_type: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            client_streaming: ::puroro_internal::FieldNew::new(),
            server_streaming: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> MethodDescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            input_type: ::puroro_internal::FieldNew::new(),
            output_type: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            client_streaming: ::puroro_internal::FieldNew::new(),
            server_streaming: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> MethodDescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            input_type: ::puroro_internal::FieldNew::new(),
            output_type: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            client_streaming: ::puroro_internal::FieldNew::new(),
            server_streaming: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for MethodDescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            input_type: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.input_type),
            output_type: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.output_type),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            client_streaming: <::std::option::Option::<bool> as Clone>::clone(&self.client_streaming),
            server_streaming: <::std::option::Option::<bool> as Clone>::clone(&self.server_streaming),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for MethodDescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for MethodDescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for MethodDescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for MethodDescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.input_type, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.output_type, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::MethodOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            5 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.client_streaming, field, slice_from_this_field, enclosing_slice)?;
            }
            
            6 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.server_streaming, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for MethodDescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for MethodDescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::MethodDescriptorProtoTrait for MethodDescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn input_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.input_type.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn output_type<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.output_type.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type OptionsType<'this> where Self: 'this = self::MethodOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::MethodOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    4,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
    fn client_streaming<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.client_streaming.clone()
    }
    fn server_streaming<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.server_streaming.clone()
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for MethodDescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::MethodDescriptorProtoTag> for MethodDescriptorProto<'slice, S> {
}

#[derive(Debug)]
pub struct ServiceDescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    method: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> ServiceDescriptorProto<'slice, S> {
    fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            method: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> ServiceDescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            method: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> ServiceDescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            method: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for ServiceDescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            method: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.method),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for ServiceDescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ServiceDescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for ServiceDescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for ServiceDescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::MethodDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.method, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::ServiceOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for ServiceDescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for ServiceDescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::ServiceDescriptorProtoTrait for ServiceDescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type MethodElement<'this> where Self: 'this = self::MethodDescriptorProto::<'slice, &'slice [u8]>;
    type MethodRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::MethodDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn method<'this>(&'this self) -> Self::MethodRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.method.as_ref(),
            2,
            &self.puroro_internal,
        )
    }
    type OptionsType<'this> where Self: 'this = self::ServiceOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::ServiceOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    3,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for ServiceDescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ServiceDescriptorProtoTag> for ServiceDescriptorProto<'slice, S> {
}

#[derive(Debug)]
pub struct EnumValueDescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    number: ::std::option::Option::<i32>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> EnumValueDescriptorProto<'slice, S> {
    fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            number: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> EnumValueDescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            number: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> EnumValueDescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            number: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for EnumValueDescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            number: <::std::option::Option::<i32> as Clone>::clone(&self.number),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for EnumValueDescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumValueDescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for EnumValueDescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for EnumValueDescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.number, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::EnumValueOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for EnumValueDescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for EnumValueDescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::EnumValueDescriptorProtoTrait for EnumValueDescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn number<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.number.clone()
    }
    type OptionsType<'this> where Self: 'this = self::EnumValueOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::EnumValueOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    3,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for EnumValueDescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumValueDescriptorProtoTag> for EnumValueDescriptorProto<'slice, S> {
}

#[derive(Debug)]
pub struct EnumDescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    value: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    reserved_range: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    reserved_name: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> EnumDescriptorProto<'slice, S> {
    fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            value: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            reserved_range: ::puroro_internal::FieldNew::new(),
            reserved_name: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> EnumDescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            value: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            reserved_range: ::puroro_internal::FieldNew::new(),
            reserved_name: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> EnumDescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            value: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            reserved_range: ::puroro_internal::FieldNew::new(),
            reserved_name: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for EnumDescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            value: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.value),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            reserved_range: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.reserved_range),
            reserved_name: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for EnumDescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumDescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for EnumDescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for EnumDescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::EnumValueDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::EnumOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::enum_descriptor_proto::EnumReservedRange::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_range, field, slice_from_this_field, enclosing_slice)?;
            }
            
            5 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for EnumDescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for EnumDescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::EnumDescriptorProtoTrait for EnumDescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type ValueElement<'this> where Self: 'this = self::EnumValueDescriptorProto::<'slice, &'slice [u8]>;
    type ValueRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::EnumValueDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn value<'this>(&'this self) -> Self::ValueRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.value.as_ref(),
            2,
            &self.puroro_internal,
        )
    }
    type OptionsType<'this> where Self: 'this = self::EnumOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::EnumOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    3,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
    type ReservedRangeElement<'this> where Self: 'this = self::enum_descriptor_proto::EnumReservedRange::<'slice, &'slice [u8]>;
    type ReservedRangeRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::enum_descriptor_proto::EnumReservedRange::<'slice, &'slice [u8]>>
        >;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.reserved_range.as_ref(),
            4,
            &self.puroro_internal,
        )
    }
    type ReservedNameRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::String
        >;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.reserved_name.as_ref(),
            5,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for EnumDescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::EnumDescriptorProtoTag> for EnumDescriptorProto<'slice, S> {
}

pub mod enum_descriptor_proto {
#[derive(Debug)]
pub struct EnumReservedRange<'slice, S> {
    start: ::std::option::Option::<i32>,
    end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> EnumReservedRange<'slice, S> {
    fn new() -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> EnumReservedRange<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> EnumReservedRange<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for EnumReservedRange<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for EnumReservedRange<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for EnumReservedRange<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for EnumReservedRange<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for EnumReservedRange<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.start, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for EnumReservedRange<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for EnumReservedRange<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::enum_descriptor_proto::EnumReservedRangeTrait for EnumReservedRange<'slice, S> {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for EnumReservedRange<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::enum_descriptor_proto::EnumReservedRangeTag> for EnumReservedRange<'slice, S> {
}

} // mod enum_descriptor_proto
#[derive(Debug)]
pub struct OneofDescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> OneofDescriptorProto<'slice, S> {
    fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> OneofDescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> OneofDescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for OneofDescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for OneofDescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for OneofDescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for OneofDescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for OneofDescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::OneofOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for OneofDescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for OneofDescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::OneofDescriptorProtoTrait for OneofDescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type OptionsType<'this> where Self: 'this = self::OneofOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::OneofOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    2,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for OneofDescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::OneofDescriptorProtoTag> for OneofDescriptorProto<'slice, S> {
}

#[derive(Debug)]
pub struct FieldDescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    number: ::std::option::Option::<i32>,
    label: ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label>,
    type_: ::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type>,
    type_name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    extendee: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    default_value: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    oneof_index: ::std::option::Option::<i32>,
    json_name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    proto3_optional: ::std::option::Option::<bool>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> FieldDescriptorProto<'slice, S> {
    fn new() -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> FieldDescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> FieldDescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for FieldDescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            number: <::std::option::Option::<i32> as Clone>::clone(&self.number),
            label: <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label> as Clone>::clone(&self.label),
            type_: <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type> as Clone>::clone(&self.type_),
            type_name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.type_name),
            extendee: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.extendee),
            default_value: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.default_value),
            oneof_index: <::std::option::Option::<i32> as Clone>::clone(&self.oneof_index),
            json_name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.json_name),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            proto3_optional: <::std::option::Option::<bool> as Clone>::clone(&self.proto3_optional),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for FieldDescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FieldDescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for FieldDescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for FieldDescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.number, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label> as FieldMergeFromSlice<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_descriptor_proto::Label>, 
                    tags::Optional2>>
                ::merge(&mut self.label, field, slice_from_this_field, enclosing_slice)?;
            }
            
            5 => {
                <::std::option::Option::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type> as FieldMergeFromSlice<
                    tags::Enum2::<super::super::super::enums::google::protobuf::field_descriptor_proto::Type>, 
                    tags::Optional2>>
                ::merge(&mut self.type_, field, slice_from_this_field, enclosing_slice)?;
            }
            
            6 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.type_name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.extendee, field, slice_from_this_field, enclosing_slice)?;
            }
            
            7 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.default_value, field, slice_from_this_field, enclosing_slice)?;
            }
            
            9 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.oneof_index, field, slice_from_this_field, enclosing_slice)?;
            }
            
            10 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.json_name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            8 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::FieldOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            17 => {
                <::std::option::Option::<bool> as FieldMergeFromSlice<
                    tags::Bool, 
                    tags::Optional2>>
                ::merge(&mut self.proto3_optional, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for FieldDescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for FieldDescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::FieldDescriptorProtoTrait for FieldDescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
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
        self.type_name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn extendee<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.extendee.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn default_value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.default_value.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn oneof_index<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.oneof_index.clone()
    }
    fn json_name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.json_name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type OptionsType<'this> where Self: 'this = self::FieldOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::FieldOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    8,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
    fn proto3_optional<'this>(&'this self) -> ::std::option::Option::<bool> {
        self.proto3_optional.clone()
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for FieldDescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FieldDescriptorProtoTag> for FieldDescriptorProto<'slice, S> {
}

pub mod field_descriptor_proto {
} // mod field_descriptor_proto
#[derive(Debug)]
pub struct ExtensionRangeOptions<'slice, S> {
    uninterpreted_option: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> ExtensionRangeOptions<'slice, S> {
    fn new() -> Self {
        Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> ExtensionRangeOptions<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> ExtensionRangeOptions<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            uninterpreted_option: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for ExtensionRangeOptions<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            uninterpreted_option: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.uninterpreted_option),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for ExtensionRangeOptions<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ExtensionRangeOptions<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for ExtensionRangeOptions<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for ExtensionRangeOptions<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            999 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::UninterpretedOption::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.uninterpreted_option, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for ExtensionRangeOptions<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for ExtensionRangeOptions<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::ExtensionRangeOptionsTrait for ExtensionRangeOptions<'slice, S> {
    type UninterpretedOptionElement<'this> where Self: 'this = self::UninterpretedOption::<'slice, &'slice [u8]>;
    type UninterpretedOptionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::UninterpretedOption::<'slice, &'slice [u8]>>
        >;
    fn uninterpreted_option<'this>(&'this self) -> Self::UninterpretedOptionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.uninterpreted_option.as_ref(),
            999,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for ExtensionRangeOptions<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::ExtensionRangeOptionsTag> for ExtensionRangeOptions<'slice, S> {
}

#[derive(Debug)]
pub struct DescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    field: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    extension: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    nested_type: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    enum_type: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    extension_range: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    oneof_decl: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    reserved_range: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    reserved_name: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> DescriptorProto<'slice, S> {
    fn new() -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> DescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> DescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for DescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            field: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.field),
            extension: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.extension),
            nested_type: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.nested_type),
            enum_type: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.enum_type),
            extension_range: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.extension_range),
            oneof_decl: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.oneof_decl),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            reserved_range: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.reserved_range),
            reserved_name: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.reserved_name),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for DescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for DescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for DescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for DescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::FieldDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.field, field, slice_from_this_field, enclosing_slice)?;
            }
            
            6 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::FieldDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.extension, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::DescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.nested_type, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::EnumDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.enum_type, field, slice_from_this_field, enclosing_slice)?;
            }
            
            5 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::descriptor_proto::ExtensionRange::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.extension_range, field, slice_from_this_field, enclosing_slice)?;
            }
            
            8 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::OneofDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.oneof_decl, field, slice_from_this_field, enclosing_slice)?;
            }
            
            7 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::MessageOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            9 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::descriptor_proto::ReservedRange::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_range, field, slice_from_this_field, enclosing_slice)?;
            }
            
            10 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.reserved_name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for DescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for DescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::DescriptorProtoTrait for DescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type FieldElement<'this> where Self: 'this = self::FieldDescriptorProto::<'slice, &'slice [u8]>;
    type FieldRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::FieldDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn field<'this>(&'this self) -> Self::FieldRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.field.as_ref(),
            2,
            &self.puroro_internal,
        )
    }
    type ExtensionElement<'this> where Self: 'this = self::FieldDescriptorProto::<'slice, &'slice [u8]>;
    type ExtensionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::FieldDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.extension.as_ref(),
            6,
            &self.puroro_internal,
        )
    }
    type NestedTypeElement<'this> where Self: 'this = self::DescriptorProto::<'slice, &'slice [u8]>;
    type NestedTypeRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::DescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn nested_type<'this>(&'this self) -> Self::NestedTypeRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.nested_type.as_ref(),
            3,
            &self.puroro_internal,
        )
    }
    type EnumTypeElement<'this> where Self: 'this = self::EnumDescriptorProto::<'slice, &'slice [u8]>;
    type EnumTypeRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::EnumDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.enum_type.as_ref(),
            4,
            &self.puroro_internal,
        )
    }
    type ExtensionRangeElement<'this> where Self: 'this = self::descriptor_proto::ExtensionRange::<'slice, &'slice [u8]>;
    type ExtensionRangeRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::descriptor_proto::ExtensionRange::<'slice, &'slice [u8]>>
        >;
    fn extension_range<'this>(&'this self) -> Self::ExtensionRangeRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.extension_range.as_ref(),
            5,
            &self.puroro_internal,
        )
    }
    type OneofDeclElement<'this> where Self: 'this = self::OneofDescriptorProto::<'slice, &'slice [u8]>;
    type OneofDeclRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::OneofDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn oneof_decl<'this>(&'this self) -> Self::OneofDeclRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.oneof_decl.as_ref(),
            8,
            &self.puroro_internal,
        )
    }
    type OptionsType<'this> where Self: 'this = self::MessageOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::MessageOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    7,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
    type ReservedRangeElement<'this> where Self: 'this = self::descriptor_proto::ReservedRange::<'slice, &'slice [u8]>;
    type ReservedRangeRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::descriptor_proto::ReservedRange::<'slice, &'slice [u8]>>
        >;
    fn reserved_range<'this>(&'this self) -> Self::ReservedRangeRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.reserved_range.as_ref(),
            9,
            &self.puroro_internal,
        )
    }
    type ReservedNameRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::String
        >;
    fn reserved_name<'this>(&'this self) -> Self::ReservedNameRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.reserved_name.as_ref(),
            10,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for DescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::DescriptorProtoTag> for DescriptorProto<'slice, S> {
}

pub mod descriptor_proto {
#[derive(Debug)]
pub struct ReservedRange<'slice, S> {
    start: ::std::option::Option::<i32>,
    end: ::std::option::Option::<i32>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> ReservedRange<'slice, S> {
    fn new() -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> ReservedRange<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> ReservedRange<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for ReservedRange<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for ReservedRange<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ReservedRange<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for ReservedRange<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for ReservedRange<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.start, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for ReservedRange<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for ReservedRange<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::descriptor_proto::ReservedRangeTrait for ReservedRange<'slice, S> {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for ReservedRange<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::descriptor_proto::ReservedRangeTag> for ReservedRange<'slice, S> {
}

#[derive(Debug)]
pub struct ExtensionRange<'slice, S> {
    start: ::std::option::Option::<i32>,
    end: ::std::option::Option::<i32>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> ExtensionRange<'slice, S> {
    fn new() -> Self {
        Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> ExtensionRange<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> ExtensionRange<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            start: ::puroro_internal::FieldNew::new(),
            end: ::puroro_internal::FieldNew::new(),
            options: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for ExtensionRange<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            start: <::std::option::Option::<i32> as Clone>::clone(&self.start),
            end: <::std::option::Option::<i32> as Clone>::clone(&self.end),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for ExtensionRange<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for ExtensionRange<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for ExtensionRange<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for ExtensionRange<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.start, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.end, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<super::ExtensionRangeOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for ExtensionRange<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for ExtensionRange<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::descriptor_proto::ExtensionRangeTrait for ExtensionRange<'slice, S> {
    fn start<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.start.clone()
    }
    fn end<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.end.clone()
    }
    type OptionsType<'this> where Self: 'this = super::ExtensionRangeOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                super::ExtensionRangeOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    3,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for ExtensionRange<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::descriptor_proto::ExtensionRangeTag> for ExtensionRange<'slice, S> {
}

} // mod descriptor_proto
#[derive(Debug)]
pub struct FileDescriptorProto<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    package: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    dependency: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    public_dependency: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    weak_dependency: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    message_type: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    enum_type: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    service: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    extension: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    options: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    source_code_info: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    syntax: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> FileDescriptorProto<'slice, S> {
    fn new() -> Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> FileDescriptorProto<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> FileDescriptorProto<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
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
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for FileDescriptorProto<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            package: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.package),
            dependency: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.dependency),
            public_dependency: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.public_dependency),
            weak_dependency: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.weak_dependency),
            message_type: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.message_type),
            enum_type: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.enum_type),
            service: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.service),
            extension: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.extension),
            options: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.options),
            source_code_info: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.source_code_info),
            syntax: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.syntax),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for FileDescriptorProto<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FileDescriptorProto<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for FileDescriptorProto<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for FileDescriptorProto<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.name, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.package, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.dependency, field, slice_from_this_field, enclosing_slice)?;
            }
            
            10 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.public_dependency, field, slice_from_this_field, enclosing_slice)?;
            }
            
            11 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Repeated>>
                ::merge(&mut self.weak_dependency, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::DescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.message_type, field, slice_from_this_field, enclosing_slice)?;
            }
            
            5 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::EnumDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.enum_type, field, slice_from_this_field, enclosing_slice)?;
            }
            
            6 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::ServiceDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.service, field, slice_from_this_field, enclosing_slice)?;
            }
            
            7 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::FieldDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.extension, field, slice_from_this_field, enclosing_slice)?;
            }
            
            8 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::FileOptions::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.options, field, slice_from_this_field, enclosing_slice)?;
            }
            
            9 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::SourceCodeInfo::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.source_code_info, field, slice_from_this_field, enclosing_slice)?;
            }
            
            12 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.syntax, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for FileDescriptorProto<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for FileDescriptorProto<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::FileDescriptorProtoTrait for FileDescriptorProto<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn package<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.package.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type DependencyRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::String
        >;
    fn dependency<'this>(&'this self) -> Self::DependencyRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.dependency.as_ref(),
            3,
            &self.puroro_internal,
        )
    }
    type PublicDependencyRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Int32
        >;
    fn public_dependency<'this>(&'this self) -> Self::PublicDependencyRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.public_dependency.as_ref(),
            10,
            &self.puroro_internal,
        )
    }
    type WeakDependencyRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Int32
        >;
    fn weak_dependency<'this>(&'this self) -> Self::WeakDependencyRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.weak_dependency.as_ref(),
            11,
            &self.puroro_internal,
        )
    }
    type MessageTypeElement<'this> where Self: 'this = self::DescriptorProto::<'slice, &'slice [u8]>;
    type MessageTypeRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::DescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn message_type<'this>(&'this self) -> Self::MessageTypeRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.message_type.as_ref(),
            4,
            &self.puroro_internal,
        )
    }
    type EnumTypeElement<'this> where Self: 'this = self::EnumDescriptorProto::<'slice, &'slice [u8]>;
    type EnumTypeRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::EnumDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn enum_type<'this>(&'this self) -> Self::EnumTypeRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.enum_type.as_ref(),
            5,
            &self.puroro_internal,
        )
    }
    type ServiceElement<'this> where Self: 'this = self::ServiceDescriptorProto::<'slice, &'slice [u8]>;
    type ServiceRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::ServiceDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn service<'this>(&'this self) -> Self::ServiceRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.service.as_ref(),
            6,
            &self.puroro_internal,
        )
    }
    type ExtensionElement<'this> where Self: 'this = self::FieldDescriptorProto::<'slice, &'slice [u8]>;
    type ExtensionRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::FieldDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn extension<'this>(&'this self) -> Self::ExtensionRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.extension.as_ref(),
            7,
            &self.puroro_internal,
        )
    }
    type OptionsType<'this> where Self: 'this = self::FileOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn options<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::OptionsType::<'this>>> {
        self.options.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::FileOptions::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    8,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
    type SourceCodeInfoType<'this> where Self: 'this = self::SourceCodeInfo::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn source_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::SourceCodeInfoType::<'this>>> {
        self.source_code_info.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::SourceCodeInfo::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    9,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
    fn syntax<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.syntax.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for FileDescriptorProto<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileDescriptorProtoTag> for FileDescriptorProto<'slice, S> {
}

#[derive(Debug)]
pub struct FileDescriptorSet<'slice, S> {
    file: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> FileDescriptorSet<'slice, S> {
    fn new() -> Self {
        Self {
            file: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> FileDescriptorSet<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            file: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> FileDescriptorSet<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            file: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for FileDescriptorSet<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            file: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for FileDescriptorSet<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for FileDescriptorSet<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for FileDescriptorSet<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for FileDescriptorSet<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldMergeFromSlice;
        use ::puroro::tags;
        match field_number {
            1 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::FileDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.file, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for FileDescriptorSet<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for FileDescriptorSet<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::traits::google::protobuf::FileDescriptorSetTrait for FileDescriptorSet<'slice, S> {
    type FileElement<'this> where Self: 'this = self::FileDescriptorProto::<'slice, &'slice [u8]>;
    type FileRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::FileDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.file.as_ref(),
            1,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for FileDescriptorSet<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::tags::google::protobuf::FileDescriptorSetTag> for FileDescriptorSet<'slice, S> {
}


pub mod compiler;
