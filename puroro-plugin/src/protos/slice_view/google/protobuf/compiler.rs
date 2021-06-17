#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[derive(Debug)]
pub struct CodeGeneratorResponse<'slice, S> {
    error: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    supported_features: ::std::option::Option::<u64>,
    file: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> CodeGeneratorResponse<'slice, S> {
    fn new() -> Self {
        Self {
            error: ::puroro_internal::FieldNew::new(),
            supported_features: ::puroro_internal::FieldNew::new(),
            file: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> CodeGeneratorResponse<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            error: ::puroro_internal::FieldNew::new(),
            supported_features: ::puroro_internal::FieldNew::new(),
            file: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> CodeGeneratorResponse<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            error: ::puroro_internal::FieldNew::new(),
            supported_features: ::puroro_internal::FieldNew::new(),
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

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for CodeGeneratorResponse<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            error: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.error),
            supported_features: <::std::option::Option::<u64> as Clone>::clone(&self.supported_features),
            file: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.file),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for CodeGeneratorResponse<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for CodeGeneratorResponse<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for CodeGeneratorResponse<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for CodeGeneratorResponse<'slice, S> {
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
                ::merge(&mut self.error, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<u64> as FieldMergeFromSlice<
                    tags::UInt64, 
                    tags::Optional2>>
                ::merge(&mut self.supported_features, field, slice_from_this_field, enclosing_slice)?;
            }
            
            15 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::code_generator_response::File::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.file, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for CodeGeneratorResponse<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for CodeGeneratorResponse<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::compiler::CodeGeneratorResponseTrait for CodeGeneratorResponse<'slice, S> {
    fn error<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.error.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn supported_features<'this>(&'this self) -> ::std::option::Option::<u64> {
        self.supported_features.clone()
    }
    type FileElement<'this> where Self: 'this = self::code_generator_response::File::<'slice, &'slice [u8]>;
    type FileRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<self::code_generator_response::File::<'slice, &'slice [u8]>>
        >;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.file.as_ref(),
            15,
            &self.puroro_internal,
        )
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for CodeGeneratorResponse<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::CodeGeneratorResponseTag> for CodeGeneratorResponse<'slice, S> {
}

pub mod code_generator_response {
#[derive(Debug)]
pub struct File<'slice, S> {
    name: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    insertion_point: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    content: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    generated_code_info: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> File<'slice, S> {
    fn new() -> Self {
        Self {
            name: ::puroro_internal::FieldNew::new(),
            insertion_point: ::puroro_internal::FieldNew::new(),
            content: ::puroro_internal::FieldNew::new(),
            generated_code_info: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> File<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            insertion_point: ::puroro_internal::FieldNew::new(),
            content: ::puroro_internal::FieldNew::new(),
            generated_code_info: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> File<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            name: ::puroro_internal::FieldNew::new(),
            insertion_point: ::puroro_internal::FieldNew::new(),
            content: ::puroro_internal::FieldNew::new(),
            generated_code_info: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for File<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            name: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.name),
            insertion_point: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.insertion_point),
            content: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.content),
            generated_code_info: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.generated_code_info),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for File<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for File<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for File<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for File<'slice, S> {
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
                ::merge(&mut self.insertion_point, field, slice_from_this_field, enclosing_slice)?;
            }
            
            15 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.content, field, slice_from_this_field, enclosing_slice)?;
            }
            
            16 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<super::super::GeneratedCodeInfo::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.generated_code_info, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for File<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for File<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::super::traits::google::protobuf::compiler::code_generator_response::FileTrait for File<'slice, S> {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.name.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn insertion_point<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.insertion_point.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    fn content<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.content.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type GeneratedCodeInfoType<'this> where Self: 'this = super::super::GeneratedCodeInfo::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn generated_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::GeneratedCodeInfoType::<'this>>> {
        self.generated_code_info.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                super::super::GeneratedCodeInfo::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    16,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for File<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::super::tags::google::protobuf::compiler::code_generator_response::FileTag> for File<'slice, S> {
}

} // mod code_generator_response
#[derive(Debug)]
pub struct CodeGeneratorRequest<'slice, S> {
    file_to_generate: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    parameter: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    proto_file: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    compiler_version: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> CodeGeneratorRequest<'slice, S> {
    fn new() -> Self {
        Self {
            file_to_generate: ::puroro_internal::FieldNew::new(),
            parameter: ::puroro_internal::FieldNew::new(),
            proto_file: ::puroro_internal::FieldNew::new(),
            compiler_version: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> CodeGeneratorRequest<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            file_to_generate: ::puroro_internal::FieldNew::new(),
            parameter: ::puroro_internal::FieldNew::new(),
            proto_file: ::puroro_internal::FieldNew::new(),
            compiler_version: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> CodeGeneratorRequest<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            file_to_generate: ::puroro_internal::FieldNew::new(),
            parameter: ::puroro_internal::FieldNew::new(),
            proto_file: ::puroro_internal::FieldNew::new(),
            compiler_version: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for CodeGeneratorRequest<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            file_to_generate: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.file_to_generate),
            parameter: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.parameter),
            proto_file: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.proto_file),
            compiler_version: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as Clone>::clone(&self.compiler_version),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for CodeGeneratorRequest<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for CodeGeneratorRequest<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for CodeGeneratorRequest<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for CodeGeneratorRequest<'slice, S> {
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
                    tags::String, 
                    tags::Repeated>>
                ::merge(&mut self.file_to_generate, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.parameter, field, slice_from_this_field, enclosing_slice)?;
            }
            
            15 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<super::FileDescriptorProto::<'slice, S>>, 
                    tags::Repeated>>
                ::merge(&mut self.proto_file, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldMergeFromSlice<
                    tags::Message::<self::Version::<'slice, S>>, 
                    tags::Optional2>>
                ::merge(&mut self.compiler_version, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for CodeGeneratorRequest<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for CodeGeneratorRequest<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::compiler::CodeGeneratorRequestTrait for CodeGeneratorRequest<'slice, S> {
    type FileToGenerateRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::String
        >;
    fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.file_to_generate.as_ref(),
            1,
            &self.puroro_internal,
        )
    }
    fn parameter<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.parameter.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
    type ProtoFileElement<'this> where Self: 'this = super::FileDescriptorProto::<'slice, &'slice [u8]>;
    type ProtoFileRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slice,
            'this,
            S,
            ::puroro::tags::Message::<super::FileDescriptorProto::<'slice, &'slice [u8]>>
        >;
    fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeated::<'this> {
        ::puroro_internal::RepeatedSliceViewField::new(
            self.proto_file.as_ref(),
            15,
            &self.puroro_internal,
        )
    }
    type CompilerVersionType<'this> where Self: 'this = self::Version::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn compiler_version<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::CompilerVersionType::<'this>>> {
        self.compiler_version.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::Version::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    3,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for CodeGeneratorRequest<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::CodeGeneratorRequestTag> for CodeGeneratorRequest<'slice, S> {
}

#[derive(Debug)]
pub struct Version<'slice, S> {
    major: ::std::option::Option::<i32>,
    minor: ::std::option::Option::<i32>,
    patch: ::std::option::Option::<i32>,
    suffix: ::std::option::Option::<::std::borrow::Cow<'slice, str>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> Version<'slice, S> {
    fn new() -> Self {
        Self {
            major: ::puroro_internal::FieldNew::new(),
            minor: ::puroro_internal::FieldNew::new(),
            patch: ::puroro_internal::FieldNew::new(),
            suffix: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}

impl<'slice> Version<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            major: ::puroro_internal::FieldNew::new(),
            minor: ::puroro_internal::FieldNew::new(),
            patch: ::puroro_internal::FieldNew::new(),
            suffix: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}

impl<'slice, 'par, SS> Version<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            major: ::puroro_internal::FieldNew::new(),
            minor: ::puroro_internal::FieldNew::new(),
            patch: ::puroro_internal::FieldNew::new(),
            suffix: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for Version<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro::InternalData;
        Self {
            major: <::std::option::Option::<i32> as Clone>::clone(&self.major),
            minor: <::std::option::Option::<i32> as Clone>::clone(&self.minor),
            patch: <::std::option::Option::<i32> as Clone>::clone(&self.patch),
            suffix: <::std::option::Option::<::std::borrow::Cow<'slice, str>> as Clone>::clone(&self.suffix),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for Version<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for Version<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for Version<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}

impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for Version<'slice, S> {
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
                ::merge(&mut self.major, field, slice_from_this_field, enclosing_slice)?;
            }
            
            2 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.minor, field, slice_from_this_field, enclosing_slice)?;
            }
            
            3 => {
                <::std::option::Option::<i32> as FieldMergeFromSlice<
                    tags::Int32, 
                    tags::Optional2>>
                ::merge(&mut self.patch, field, slice_from_this_field, enclosing_slice)?;
            }
            
            4 => {
                <::std::option::Option::<::std::borrow::Cow<'slice, str>> as FieldMergeFromSlice<
                    tags::String, 
                    tags::Optional2>>
                ::merge(&mut self.suffix, field, slice_from_this_field, enclosing_slice)?;
            }
            
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for Version<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for Version<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> super::super::super::super::traits::google::protobuf::compiler::VersionTrait for Version<'slice, S> {
    fn major<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.major.clone()
    }
    fn minor<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.minor.clone()
    }
    fn patch<'this>(&'this self) -> ::std::option::Option::<i32> {
        self.patch.clone()
    }
    fn suffix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>> {
        self.suffix.as_ref().map(|x| {
            ::std::borrow::Cow::Borrowed(x.as_ref())
        })
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message for Version<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
    type BoxedType = ::std::boxed::Box::<Self>;
    fn into_boxed(self) -> Self::BoxedType {
        ::std::boxed::Box::new(self)    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::IsMessageImplOfTag<super::super::super::super::tags::google::protobuf::compiler::VersionTag> for Version<'slice, S> {
}

