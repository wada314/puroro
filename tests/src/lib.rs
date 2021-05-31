#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::marker::PhantomData;

trait Rf<'a> {
}
struct Rsf<'a, 'b> ( std::marker::PhantomData<(&'a (), &'b ())>);
impl<'a, 'b> Rf<'a> for Rsf<'a, 'b> {}

trait A {
    type AA<'a> : Rf<'a> where Self: 'a;
}
struct B<'b>(PhantomData<&'b ()>);
impl<'b> A for B<'b> {
    type AA<'a> where Self: 'a = Rsf<'a, 'b>;
}

pub trait MsgTrait: ::std::clone::Clone {
    type TheMapElement<'this>: self::msg::TheMapEntryTrait where Self: 'this;
    type TheMapRepeated<'this>: ::puroro::RepeatedField::<'this, ::std::borrow::Cow::<'this, <Self as MsgTrait>::TheMapElement::<'this>>>
        where Self: 'this;
    type TheMapRepeated2<'this>: ::puroro::RepeatedField::<'this, ::std::borrow::Cow::<'this, str>>
        where Self: 'this;
}

#[derive(Debug)]
pub struct MsgSliceView<'slice, S> {
    the_map: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for MsgSliceView<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro_internal::FieldClone;
        use ::puroro::InternalData;
        Self {
            the_map: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldClone>::clone(&self.the_map),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slicee, S: ::puroro_internal::SliceSource<'slicee>> MsgTrait for MsgSliceView<'slicee, S> {
    type TheMapElement<'thiss> where Self: 'thiss = self::msg::TheMapEntrySliceView::<'slicee, &'slicee [u8]>;
    type TheMapRepeated<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slicee,
            'this,
            S,
            ::puroro_internal::tags::Message::<self::msg::TheMapEntrySliceView::<'slicee, &'slicee [u8]>>
        >;
    type TheMapRepeated2<'this> where Self: 'this =
        ::puroro_internal::RepeatedSliceViewField::<
            'slicee,
            'this,
            S,
            ::puroro_internal::tags::String,
        >;
}

pub mod msg {
pub trait TheMapEntryTrait: ::std::clone::Clone {
    fn key<'this>(&'this self) -> ::std::borrow::Cow::<'this, str>;
    type ValueType<'this>: self::SubMsgTrait where Self: 'this;
    fn value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, <Self as TheMapEntryTrait>::ValueType::<'this>>>;
}

#[derive(Debug)]
pub struct TheMapEntrySliceView<'slice, S> {
    key: &'slice str,
    value: ::std::option::Option::<::puroro_internal::SliceViewField::<'slice>>,
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> TheMapEntrySliceView<'slice, S> {
    fn new() -> Self {
        Self {
            key: ::puroro_internal::FieldNew::new(),
            value: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}
impl<'slice> TheMapEntrySliceView<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            key: ::puroro_internal::FieldNew::new(),
            value: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}
impl<'slice, 'par, SS> TheMapEntrySliceView<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            key: ::puroro_internal::FieldNew::new(),
            value: ::puroro_internal::FieldNew::new(),
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for TheMapEntrySliceView<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro_internal::FieldClone;
        use ::puroro::InternalData;
        Self {
            key: <&'slice str as FieldClone>::clone(&self.key),
            value: <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldClone>::clone(&self.value),
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for TheMapEntrySliceView<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for TheMapEntrySliceView<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for TheMapEntrySliceView<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}


impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for TheMapEntrySliceView<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldDeserFromSlice;
        use ::puroro_internal::tags;
        match field_number {
            1 => {
                <&'slice str as FieldDeserFromSlice<
                    tags::String, 
                    tags::Optional3>>
                ::deser(&mut self.key, field, slice_from_this_field, enclosing_slice)?;
            }
            2 => {
                <::std::option::Option::<::puroro_internal::SliceViewField::<'slice>> as FieldDeserFromSlice<
                    tags::Message::<self::SubMsgSliceView::<'slice, S>>, 
                    tags::Optional3>>
                ::deser(&mut self.value, field, slice_from_this_field, enclosing_slice)?;
            }
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for TheMapEntrySliceView<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for TheMapEntrySliceView<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> TheMapEntryTrait for TheMapEntrySliceView<'slice, S> {
    fn key<'this>(&'this self) -> ::std::borrow::Cow::<'this, str> {
        ::std::borrow::Cow::Borrowed(self.key)
    }
    type ValueType<'this> where Self: 'this = self::SubMsgSliceView::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>;
    fn value<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, <Self as TheMapEntryTrait>::ValueType::<'this>>> {
        self.value.as_ref().map(|field| {
            ::std::borrow::Cow::Owned(
                self::SubMsgSliceView::<'slice, ::puroro_internal::SourceLdSlices<'slice, 'this, S>>::try_new_with_parent(
                    ::std::option::Option::Some(field),
                    2,
                    &self.puroro_internal
                ).expect("Invalid input slice. Consider checking the slice content earlier (TBD).")
            )
        })
    }
}

impl<'slice, 'bump, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message<'bump> for TheMapEntrySliceView<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
}
pub trait SubMsgTrait: ::std::clone::Clone {
}

#[derive(Debug)]
pub struct SubMsgSliceView<'slice, S> {
    puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>,
}

impl<'slice, S> SubMsgSliceView<'slice, S> {
    fn new() -> Self {
        Self {
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new(),
        }
    }
}
impl<'slice> SubMsgSliceView<'slice, &'slice [u8]> {
    fn try_new_with_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_slice(slice),
        };
        let ld_slice = ::puroro_internal::deser::LdSlice::new(slice);
        ld_slice.merge_into_message(&mut new_self)?;
        Ok(new_self)
    }
}
impl<'slice, 'par, SS> SubMsgSliceView<'slice, ::puroro_internal::SourceLdSlices<'slice, 'par, SS>> 
    where ::puroro_internal::SourceLdSlices<'slice, 'par, SS>: ::puroro_internal::SliceSource<'slice>,
{
    fn try_new_with_parent(
        field_in_parent: ::std::option::Option<&'par ::puroro_internal::SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par ::puroro_internal::InternalDataForSliceViewStruct<'slice, SS>,
    ) -> ::puroro::Result<Self> {
        let mut new_self = Self {
            puroro_internal: ::puroro_internal::InternalDataForSliceViewStruct::new_with_parent(
                field_in_parent, field_number_in_parent, parent_internal_data),
        };
        for ld_slice in new_self.puroro_internal.source_ld_slices() {
            ld_slice?.merge_into_message(&mut new_self)?;
        }
        Ok(new_self)
    }
}

impl<'slice, S: ::std::clone::Clone> ::std::clone::Clone for SubMsgSliceView<'slice, S> {
    fn clone(&self) -> Self {
        use ::puroro_internal::FieldClone;
        use ::puroro::InternalData;
        Self {
            puroro_internal: self.puroro_internal.clone(),
        }
    }
}

impl<'slice, S> ::std::default::Default for SubMsgSliceView<'slice, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'slice> ::puroro::DeserializableFromSlice<'slice> for SubMsgSliceView<'slice, &'slice [u8]> {
    fn deser_from_slice(slice: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(slice)
    }
}

impl<'slice> ::std::convert::TryFrom<&'slice [u8]> for SubMsgSliceView<'slice, &'slice [u8]> {
    type Error = ::puroro::PuroroError;
    fn try_from(value: &'slice [u8]) -> ::puroro::Result<Self> {
        Self::try_new_with_slice(value)
    }
}


impl<'slice, S> ::puroro_internal::deser::DeserializableMessageFromSlice<'slice> for SubMsgSliceView<'slice, S> {
    fn met_field_at(
        &mut self,
        field: ::puroro_internal::types::FieldData<::puroro_internal::deser::LdSlice<'slice>>, 
        field_number: usize,
        slice_from_this_field: ::puroro_internal::deser::LdSlice<'slice>,
        enclosing_slice: ::puroro_internal::deser::LdSlice<'slice>,
    ) -> ::puroro::Result<bool>
    {
        use ::puroro_internal::FieldDeserFromSlice;
        use ::puroro_internal::tags;
        match field_number {
            _ => Err(::puroro::ErrorKind::UnexpectedFieldId)?,
        }
        Ok(true)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro_internal::ser::SerializableMessage for SubMsgSliceView<'slice, S> {
    fn serialize<T: ::puroro_internal::ser::MessageSerializer>(
        &self, serializer: &mut T) -> ::puroro::Result<()>
    {
        for ld_slice in self.puroro_internal.source_ld_slices() {
            serializer.serialize_raw_fields(ld_slice?.as_slice())?;
        }
        Ok(())
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Serializable for SubMsgSliceView<'slice, S> {
    fn serialize<W: std::io::Write>(&self, write: &mut W) -> ::puroro::Result<()> {
        let mut serializer = ::puroro_internal::ser::default_serializer(write);
        <Self as ::puroro_internal::ser::SerializableMessage>::serialize(self, &mut serializer)
    }
}

impl<'slice, S: ::puroro_internal::SliceSource<'slice>> SubMsgTrait for SubMsgSliceView<'slice, S> {
}

impl<'slice, 'bump, S: ::puroro_internal::SliceSource<'slice>> ::puroro::Message<'bump> for SubMsgSliceView<'slice, S> {
    type InternalData = ::puroro_internal::InternalDataForSliceViewStruct<'slice, S>;
    fn puroro_internal_data(&self) -> &Self::InternalData {
        &self.puroro_internal
    }
}
} // mod msg


#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[test]
    fn it_works() {

        assert_eq!(2 + 2, 4);
    }
}
