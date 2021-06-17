use std::borrow::Cow;

use super::{DoDefaultCheck, FieldMergeFromIter, WrappedFieldType};
use crate::deser::LdIter;
use crate::deser::LdSlice;
use crate::types::FieldData;
use crate::variant;
use crate::SliceViewField;
use crate::{ErrorKind, Result};
use ::puroro::tags;

pub trait FieldMergeFromSlice<'a, 'slice, TypeTag, LabelTag>
where
    TypeTag: tags::WireAndValueTypeTag,
    LabelTag: tags::FieldLabelTag,
{
    /// Deserialize binary data into this field.
    /// * `field` - A data of the field, where the wire type and (for length delimited wire
    /// type) the field length are already load. For variants and fixed bytes fields,
    /// the content data is also already load.
    /// * `slice_from_this_field` - a subslice of `enclosing_slice` starting from the field's
    /// first byte (including the bytes for wire_type, field_number and field_length).
    /// * `enclosing_slice` - Slice for this field's owner's fields. If the owner message is
    /// split into multiple instances in the input slice, then the instance of the one that
    /// this field is included.
    fn merge(
        &'a mut self,
        field: FieldData<LdSlice<'slice>>,
        slice_from_this_field: LdSlice<'slice>,
        enclosing_slice: LdSlice<'slice>,
    ) -> Result<()>;
}

pub trait NonRepeatedLabelTag {}
impl NonRepeatedLabelTag for tags::Required {}
impl NonRepeatedLabelTag for tags::Optional2 {}
impl NonRepeatedLabelTag for tags::Optional3 {}

// Variant types, non-repeated label
// reuse the `FieldMergeFromIter`.
impl<'a, 'slice, V, L, T> FieldMergeFromSlice<'a, 'slice, (tags::wire::Variant, V), L> for T
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
    L: tags::FieldLabelTag + NonRepeatedLabelTag,
    T: WrappedFieldType<L, Item = <V as variant::VariantTypeTag>::NativeType>
        + FieldMergeFromIter<
            'a,
            (tags::wire::Variant, V),
            L,
            Item = <V as variant::VariantTypeTag>::NativeType,
        >,
{
    fn merge(
        &'a mut self,
        field: FieldData<LdSlice<'slice>>,
        _: LdSlice<'slice>,
        _: LdSlice<'slice>,
    ) -> Result<()> {
        use std::io::Read;
        let mut iter_field = field.map(|ld_slice| LdIter::new(ld_slice.as_slice().bytes()));
        <T as FieldMergeFromIter<(tags::wire::Variant, V), L>>::merge(
            self,
            iter_field.as_mut(),
            || unimplemented!(),
        )?;
        Ok(())
    }
}

// Bytes type, non-repeated label
impl<'a, 'slice, L, T> FieldMergeFromSlice<'a, 'slice, tags::Bytes, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck + NonRepeatedLabelTag,
    T: WrappedFieldType<L, Item = Cow<'slice, [u8]>>,
{
    fn merge(
        &'a mut self,
        field: FieldData<LdSlice<'slice>>,
        _: LdSlice<'slice>,
        _: LdSlice<'slice>,
    ) -> Result<()> {
        if let FieldData::LengthDelimited(ld_slice) = field {
            let new_value = Cow::Borrowed(ld_slice.as_slice());
            if !L::DO_DEFAULT_CHECK || !new_value.is_empty() {
                *self.get_or_insert_with(Default::default) = new_value;
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

// String type, non-repeated label
impl<'a, 'slice, L, T> FieldMergeFromSlice<'a, 'slice, tags::String, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck + NonRepeatedLabelTag,
    T: WrappedFieldType<L, Item = Cow<'slice, str>>,
{
    fn merge(
        &'a mut self,
        field: FieldData<LdSlice<'slice>>,
        _: LdSlice<'slice>,
        _: LdSlice<'slice>,
    ) -> Result<()> {
        if let FieldData::LengthDelimited(ld_slice) = field {
            let new_value = String::from_utf8_lossy(ld_slice.as_slice());
            if !L::DO_DEFAULT_CHECK || !new_value.is_empty() {
                *self.get_or_insert_with(Default::default) = new_value;
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

// Message types, non-repeated label
impl<'a, 'slice, 'msg, M, L> FieldMergeFromSlice<'a, 'slice, tags::Message<M>, L>
    for Option<SliceViewField<'slice>>
where
    L: tags::FieldLabelTag + NonRepeatedLabelTag,
{
    fn merge(
        &'a mut self,
        _: FieldData<LdSlice<'slice>>,
        slice_from_this_field: LdSlice<'slice>,
        enclosing_slice: LdSlice<'slice>,
    ) -> Result<()> {
        update_slice_view_field(self, slice_from_this_field, enclosing_slice);
        Ok(())
    }
}

// Bits32 types, non-repeated label
impl<'a, 'slice, V, L, T> FieldMergeFromSlice<'a, 'slice, (tags::wire::Bits32, V), L> for T
where
    V: tags::Bits32TypeTag + super::Bits32TypeTag<NativeType = T::Item>,
    L: tags::FieldLabelTag + DoDefaultCheck + NonRepeatedLabelTag,
    T: WrappedFieldType<L>,
    T::Item: super::Default,
{
    fn merge(
        &'a mut self,
        field: FieldData<LdSlice<'slice>>,
        _: LdSlice<'slice>,
        _: LdSlice<'slice>,
    ) -> Result<()> {
        if let FieldData::Bits32(array) = field {
            if !L::DO_DEFAULT_CHECK || array.iter().any(|b| *b != 0) {
                *self.get_or_insert_with(super::Default::default) =
                    <V as super::Bits32TypeTag>::from_bytes(array);
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

// Bits64 types, non-repeated label
impl<'a, 'slice, V, L, T> FieldMergeFromSlice<'a, 'slice, (tags::wire::Bits64, V), L> for T
where
    V: tags::Bits64TypeTag + super::Bits64TypeTag<NativeType = T::Item>,
    L: tags::FieldLabelTag + DoDefaultCheck + NonRepeatedLabelTag,
    T: WrappedFieldType<L>,
    T::Item: super::Default,
{
    fn merge(
        &'a mut self,
        field: FieldData<LdSlice<'slice>>,
        _: LdSlice<'slice>,
        _: LdSlice<'slice>,
    ) -> Result<()> {
        if let FieldData::Bits64(array) = field {
            if !L::DO_DEFAULT_CHECK || array.iter().any(|b| *b != 0) {
                *self.get_or_insert_with(super::Default::default) =
                    <V as super::Bits64TypeTag>::from_bytes(array);
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

// Repeated fields
impl<'a, 'slice, 'msg, WireAndValue> FieldMergeFromSlice<'a, 'slice, WireAndValue, tags::Repeated>
    for Option<SliceViewField<'slice>>
where
    WireAndValue: tags::WireAndValueTypeTag,
{
    fn merge(
        &'a mut self,
        _: FieldData<LdSlice<'slice>>,
        slice_from_this_field: LdSlice<'slice>,
        enclosing_slice: LdSlice<'slice>,
    ) -> Result<()> {
        update_slice_view_field(self, slice_from_this_field, enclosing_slice);
        Ok(())
    }
}

fn update_slice_view_field<'slice>(
    field: &mut Option<SliceViewField<'slice>>,
    ld_slice_from_this_field: LdSlice<'slice>,
    enclosing_ld_slice: LdSlice<'slice>,
) {
    *field = match field.clone() {
        None => Some(SliceViewField::FieldInSingleSlice {
            ld_slice: ld_slice_from_this_field,
            count: 1,
            enclosing_ld_slice,
        }),
        Some(SliceViewField::FieldInSingleSlice {
            ld_slice,
            count,
            enclosing_ld_slice: existing_fields_enclosing_ld_slice,
        }) => Some(
            if enclosing_ld_slice == existing_fields_enclosing_ld_slice {
                SliceViewField::FieldInSingleSlice {
                    ld_slice,
                    count: count + 1,
                    enclosing_ld_slice: existing_fields_enclosing_ld_slice,
                }
            } else {
                SliceViewField::FieldInMultipleSlices {
                    count: count + 1,
                    first_enclosing_ld_slice: existing_fields_enclosing_ld_slice,
                }
            },
        ),
        Some(SliceViewField::FieldInMultipleSlices {
            count,
            first_enclosing_ld_slice,
        }) => Some(SliceViewField::FieldInMultipleSlices {
            count: count + 1,
            first_enclosing_ld_slice,
        }),
    };
}
