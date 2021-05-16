use crate::deser::LdSlice;
use crate::types::{FieldData, SliceViewField};
use crate::{ErrorKind, Result};
use ::either_n::Either4;
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
    pub fn new_with_bumpalo(bump: &'bump ::bumpalo::Bump) -> Self {
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
    source_ld_slices: SourceLdSlices<'slice, 'p>,
}
#[derive(Debug, Clone)]
pub enum SourceLdSlices<'slice, 'p> {
    SingleLdSlice(LdSlice<'slice>),
    MaybeMultipleLdSlices {
        field_in_parent: Option<&'p SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'p InternalDataForSliceViewStruct<'slice, 'p>,
    },
}

impl<'slice, 'p> InternalDataForSliceViewStruct<'slice, 'p> {
    pub fn new(slice: &'slice [u8]) -> Self {
        Self {
            source_ld_slices: SourceLdSlices::SingleLdSlice(LdSlice::new(slice)),
        }
    }

    pub fn new_with_parent(
        parent_field: &'p Option<SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'p InternalDataForSliceViewStruct<'slice, 'p>,
    ) -> Self {
        Self {
            source_ld_slices: SourceLdSlices::MaybeMultipleLdSlices {
                field_in_parent: parent_field.as_ref(),
                field_number_in_parent,
                parent_internal_data,
            },
        }
    }

    pub fn ld_slices(&self) -> impl Iterator<Item = LdSlice<'slice>> {
        self.source_ld_slices.iter()
    }
}
impl<'slice, 'p> SourceLdSlices<'slice, 'p> {
    pub fn iter(&self) -> impl Iterator<Item = LdSlice<'slice>> {
        match self.clone() {
            SourceLdSlices::SingleLdSlice(ld_slice) => {
                Either4::One(std::iter::once(ld_slice.clone()))
            }
            SourceLdSlices::MaybeMultipleLdSlices {
                field_in_parent,
                field_number_in_parent,
                parent_internal_data,
            } => match field_in_parent.cloned() {
                None => {
                    // This can happen if a scalar message field exists in .proto definition
                    // but it does not appear in the serialized input stream.
                    Either4::Two(std::iter::empty())
                }
                Some(SliceViewField::FieldInSingleSlice {
                    ld_slice, count, ..
                }) => {
                    // iterate over a single slice given from a parent message,
                    // and choose out the fields that has proper field number.
                    let iter = ld_slice
                        .fields()
                        .filter_map(move |rfield| {
                            let field = rfield.expect(
                                "An error occured while deserializing the input. \
                                Consider checking the data in earlier stage to catch this error.",
                            );
                            if field.number == field_number_in_parent {
                                if let FieldData::LengthDelimited(inner_ld_slice) = field.data {
                                    Some(inner_ld_slice)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .take(count);
                    Either4::Three(iter)
                }
                Some(SliceViewField::FieldInMultipleSlices {
                    count,
                    first_enclosing_ld_slice: first_enclosing_slice,
                    ..
                }) => {
                    // The parent message instance is scattering around multiple slices.
                    // In this case we need to iterate in the parent message recursively,
                    // that means we cannot use a static dispatching iterator because the type
                    // of the iterator recursives infinitely.
                    let iter = parent_internal_data
                        .ld_slices()
                        .skip_while(move |ld_slice| *ld_slice != first_enclosing_slice)
                        .flat_map(move |ld_slice| {
                            ld_slice.fields().filter_map(move |rfield| {
                                let field = rfield.expect(
                                    "An error occured while deserializing the input. \
                                Consider checking the data in earlier stage to catch this error.",
                                );
                                if field.number == field_number_in_parent {
                                    if let FieldData::LengthDelimited(inner_ld_slice) = field.data {
                                        Some(inner_ld_slice)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                        })
                        .take(count);
                    // Dynamic dispatching iterator to avoid an infinite recursive type...
                    Either4::Four(Box::new(iter) as Box<dyn Iterator<Item = LdSlice<'slice>>>)
                }
            },
        }
        .into_iter()
    }
}

struct SourceLdSlicesIter<'slice, 'p> {
    container: &'p SourceLdSlices<'slice, 'p>,
    prev_ld_slice: Option<LdSlice<'slice>>,
}

impl<'bump, 'slice, 'p> InternalData<'bump> for InternalDataForSliceViewStruct<'slice, 'p> {
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}
