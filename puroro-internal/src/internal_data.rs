use crate::deser::LdSlice;
use crate::types::{FieldData, SliceViewFields};
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

    pub fn ld_slices(&self) -> impl Iterator<Item = LdSlice<'slice>> {
        self.source_slices.iter()
    }
}
impl<'slice, 'p> SourceSlicesView<'slice, 'p> {
    pub fn iter(&self) -> impl Iterator<Item = LdSlice<'slice>> {
        match self.clone() {
            SourceSlicesView::SingleSlice(ld_slice) => {
                Either4::One(std::iter::once(ld_slice.clone()))
            }
            SourceSlicesView::MaybeMultipleSlice {
                field_in_parent,
                field_number_in_parent,
                parent_internal_data,
            } => match field_in_parent.cloned() {
                None => {
                    // This can happen if a scalar message field exists in .proto definition
                    // but it does not appear in the serialized input stream.
                    Either4::Two(std::iter::empty())
                }
                Some(SliceViewFields::FieldsInSingleSlice {
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
                Some(SliceViewFields::FieldsInMultipleSlices {
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

impl<'bump, 'slice, 'p> InternalData<'bump> for InternalDataForSliceViewStruct<'slice, 'p> {
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}

fn start_address(s: &[u8]) -> usize {
    s.as_ptr_range().start as usize
}

fn end_address(s: &[u8]) -> usize {
    s.as_ptr_range().end as usize
}

// Inclusive.
fn is_subslice_of(smaller: &[u8], larger: &[u8]) -> bool {
    let s = smaller.as_ptr_range();
    let l = larger.as_ptr_range();
    l.start <= s.start && s.end <= l.end
}

/// ```
/// let a = &[1, 2, 3, 4, 5];
/// let subview = &a[1:3];
/// assert_eq!(slice_after(a, subview), &[4, 5]);
/// ```
fn slice_after<'a, 'b>(source: &'a [u8], prev: &'b [u8]) -> &'a [u8] {
    let mut new_start_index = end_address(prev) - start_address(source);
    if new_start_index > source.len() {
        new_start_index = source.len();
    }
    source.split_at(new_start_index).1
}

fn next_field_item_internal<'slice, 'p>(
    depth: usize,
    prev_child_field_slice: &'slice [u8],
    repeated_field: &'p Option<SliceViewFields<'slice>>,
    repeated_field_number: usize,
    internal_data: InternalDataForSliceViewStruct<'slice, 'p>,
) -> Result<Option<LdSlice<'slice>>> {
    Ok(match repeated_field.clone() {
        Some(SliceViewFields::FieldsInSingleSlice { mut ld_slice, .. }) => {
            ld_slice.skip_until_end_of(prev_child_field_slice);
            for rfield in ld_slice.fields() {
                let field = rfield?;
                if field.number == repeated_field_number {
                    if let FieldData::LengthDelimited(ld_slice) = field.data {
                        return Ok(Some(ld_slice));
                    } else {
                        Err(ErrorKind::InvalidWireType)?
                    }
                }
            }
            None
        }
        Some(SliceViewFields::FieldsInMultipleSlices {
            count,
            first_enclosing_ld_slice: first_enclosing_slice,
        }) => {
            todo!()
        }
        None => None,
    })
}
