use crate::deser::LdSlice;
use crate::types::{FieldData, SliceViewFields};
use ::itertools::Either;
use puroro::InternalData;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InternalDataForNormalStruct {
    unknown_fields: Option<HashMap<usize, FieldData<Vec<u8>>>>,
}
impl InternalDataForNormalStruct {
    pub fn new() -> Self {
        Self {
            unknown_fields: None,
        }
    }
}
impl<'bump> InternalData<'bump> for InternalDataForNormalStruct {
    #[cfg(feature = "puroro-bumpalo")]
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}

#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct InternalDataForBumpaloStruct<'bump> {
    // No hashmap implementation in bumpalo...
    unknown_fields: Option<
        ::bumpalo::collections::Vec<
            'bump,
            (usize, FieldData<::bumpalo::collections::Vec<'bump, u8>>),
        >,
    >,
    bump: &'bump ::bumpalo::Bump,
}

#[cfg(feature = "puroro-bumpalo")]
impl<'bump> InternalDataForBumpaloStruct<'bump> {
    pub fn new(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            unknown_fields: None,
            bump,
        }
    }
}
impl<'bump> InternalData<'bump> for InternalDataForBumpaloStruct<'bump> {
    /// Note that the returned bumpalo lifetime is not `'bump' but `'_`.
    /// This is because I don't want to introduce the lifetime parameter
    /// `'b` into the trait's definition. The lifetime `'_` might be shorter
    /// than `'b`, but I believe it's not a problem.
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        self.bump
    }
}

#[derive(Debug, Clone)]
pub struct InternalDataForSliceViewStruct<'slice, 'p> {
    source_slices: SourceSlicesView<'slice, 'p>,
}
#[derive(Debug, Clone)]
pub enum SourceSlicesView<'slice, 'p> {
    SingleSlice(LdSlice<'slice>),
    MaybeMultipleSlice {
        field_in_parent: Option<&'p SliceViewFields<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'p InternalDataForSliceViewStruct<'slice, 'p>,
    },
}

impl<'slice, 'p> InternalDataForSliceViewStruct<'slice, 'p> {
    pub fn new(slice: &'slice [u8]) -> Self {
        Self {
            source_slices: SourceSlicesView::SingleSlice(LdSlice::new(slice)),
        }
    }

    pub fn new_with_parent(
        parent_field: &'p Option<SliceViewFields<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'p InternalDataForSliceViewStruct<'slice, 'p>,
    ) -> Self {
        Self {
            source_slices: SourceSlicesView::MaybeMultipleSlice {
                field_in_parent: parent_field.as_ref(),
                field_number_in_parent,
                parent_internal_data: parent_internal_data,
            },
        }
    }
}
impl<'slice, 'p> SourceSlicesView<'slice, 'p> {
    pub fn iter(&self) -> impl Iterator<Item = LdSlice<'slice>> {
        match self.clone() {
            SourceSlicesView::SingleSlice(ld_slice) => {
                Either::Left(std::iter::once(ld_slice.clone()))
            }
            SourceSlicesView::MaybeMultipleSlice {
                field_in_parent,
                field_number_in_parent,
                parent_internal_data,
            } => Either::Right(match field_in_parent.cloned() {
                None => Either::Left(std::iter::empty()),
                Some(SliceViewFields::FieldsInSingleSlice {
                    slice: slice_of_fields_in_parent,
                    count,
                    enclosing_slice: _,
                }) => {
                    let ld_slice = LdSlice::new(slice_of_fields_in_parent);
                    let iter = ld_slice
                        .map(|x| {
                            x.expect(
                                "An error occured while deserializing the input. \
                                    Consider check the data validity in earlier stage to catch this error.",
                            )
                        })
                        .filter_map(|(field_number, field_data)| {
                            if field_number == field_number_in_parent {
                                match field_data {
                                    FieldData::LengthDelimited(inner_ld_slice) => {
                                        Some(inner_ld_slice)
                                    }
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        })
                        .take(count);
                    Either::Right(iter)
                }
                Some(SliceViewFields::FieldsInMultipleSlices {
                    count,
                    first_enclosing_slice,
                }) => {
                    todo!()
                }
            }.into_iter()),
        }
        .into_iter()
    }
}

impl<'bump, 'slice, 'p> InternalData<'bump> for InternalDataForSliceViewStruct<'slice, 'p> {
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}
