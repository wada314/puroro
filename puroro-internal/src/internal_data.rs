use crate::deser::LdSlice;
use crate::types::{FieldData, SliceViewField};
use crate::{ErrorKind, Result, ResultHelper};
use ::itertools::{Either, Itertools};
use ::puroro::InternalData;
use ::std::collections::HashMap;

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
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        self.bump
    }
}

#[derive(Debug, Clone)]
pub struct InternalDataForSliceViewStruct<'slice, 'par> {
    source_ld_slices: SourceLdSlices<'slice, 'par>,
}
#[derive(Debug, Clone)]
pub enum SourceLdSlices<'slice, 'par> {
    SingleLdSlice(LdSlice<'slice>),
    MaybeMultipleLdSlices {
        field_in_parent: Option<&'par SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,
    },
}

impl<'slice, 'par> InternalDataForSliceViewStruct<'slice, 'par> {
    pub fn new(slice: &'slice [u8]) -> Self {
        Self {
            source_ld_slices: SourceLdSlices::SingleLdSlice(LdSlice::new(slice)),
        }
    }

    pub fn new_with_parent(
        parent_field: Option<&'par SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,
    ) -> Self {
        Self {
            source_ld_slices: SourceLdSlices::MaybeMultipleLdSlices {
                field_in_parent: parent_field,
                field_number_in_parent,
                parent_internal_data,
            },
        }
    }

    /// Get the list of the slices which is the source of this Message.
    /// If your purpose to get a certain field's data then make sure that field ([`SliceViewField`])'s  
    /// variant is [`SliceViewField::FieldInMultipleSlices`]. If it is [`SliceViewField::FieldInSingleSlice`],
    /// then you can use that enum variant's `ld_slice` field for shortcut.
    /// Note that iterating over this iterator takes O(n^2) time where n is the iterator length.
    /// We believe n (== the number of messages merged) is very small in the most usecases.
    pub fn ld_slices_from_parent_message(
        &self,
    ) -> impl 'par + Iterator<Item = Result<LdSlice<'slice>>> {
        match self.source_ld_slices.clone() {
            SourceLdSlices::SingleLdSlice(ld_slice) => Either::Left(std::iter::once(Ok(ld_slice))),
            SourceLdSlices::MaybeMultipleLdSlices {
                field_in_parent,
                field_number_in_parent,
                parent_internal_data,
            } => Either::Right(MultipleSourceLdSlicesIter::<'slice, 'par>::new(
                field_number_in_parent,
                field_in_parent,
                parent_internal_data,
            )),
        }
        .into_iter()
    }

    /// Returns an iterator over the specifield field number's [`FieldData`] instance.
    pub fn field_data_iter(
        &'par self,
        field_number: usize,
        field: Option<&'par SliceViewField<'slice>>,
    ) -> impl 'par + Iterator<Item = Result<FieldData<LdSlice<'slice>>>> {
        // The iter of `ld_slice` which consists the specified field.
        // Note that this might be a smaller set when compared with the `ld_slice`s consisting
        // the message struct. For example, even if there's a message consist of 3 separated slices,
        // but a certain field k can be consist of only the 2nd slice.
        let ld_slices = field
            .into_iter()
            .map(move |field| {
                match field {
                    SliceViewField::FieldInSingleSlice { ld_slice, .. } => {
                        // The field is consist of a single slice. Easy case.
                        Either::Left(std::iter::once(Ok(ld_slice.clone())))
                    }
                    SliceViewField::FieldInMultipleSlices {
                        count,
                        first_enclosing_ld_slice,
                    } => {
                        // A difficult case. The field is consist of multiple separated slices.
                        // This case can happen if the message is merged from multiple instances.
                        Either::Right(
                            self.ld_slices_from_parent_message()
                                .skip_while(move |rld_slice| match rld_slice.as_ref() {
                                    Ok(ld_slice) => *ld_slice != *first_enclosing_ld_slice,
                                    Err(_) => true,
                                })
                                .take(*count),
                        )
                    }
                }
                .into_iter()
            })
            .flatten();
        ld_slices
            .map_ok(|ld_slice| ld_slice.fields())
            .flatten_ok()
            .map(|rrfield| rrfield.flatten())
            .filter_map_ok(move |field| {
                if field.number == field_number {
                    Some(field.data)
                } else {
                    None
                }
            })
    }
}

struct MultipleSourceLdSlicesIter<'slice, 'par> {
    field_number: usize,
    field: Option<&'par SliceViewField<'slice>>,
    internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,

    prev_ld_slice: Option<LdSlice<'slice>>,
}

impl<'slice, 'par> Iterator for MultipleSourceLdSlicesIter<'slice, 'par> {
    type Item = Result<LdSlice<'slice>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

impl<'slice, 'par> MultipleSourceLdSlicesIter<'slice, 'par> {
    fn new(
        field_number: usize,
        field: Option<&'par SliceViewField<'slice>>,
        internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,
    ) -> Self {
        Self {
            field_number,
            field,
            internal_data,
            prev_ld_slice: None,
        }
    }

    fn try_next(&mut self) -> Result<Option<LdSlice<'slice>>> {
        // An iterator of ld_slices which consists the message struct.
        // Note that this iterator type contains this [`MultipleSourceLdSlicesIter`] type
        // so we cannot remember this iterator as this struct's item.
        // (Memory allocation is prohibited in SliceView methods)
        let mut ld_slices_iter = self
            .internal_data
            .field_data_iter(self.field_number, self.field)
            .filter_map_ok(|field_data| -> Option<Result<_>> {
                if let FieldData::LengthDelimited(ld_slice) = field_data {
                    Some(Ok(ld_slice))
                } else {
                    Some(Err(ErrorKind::UnexpectedWireType.into()))
                }
            })
            .map(|rrfield| rrfield.flatten());
        let result = match self.prev_ld_slice.clone() {
            Some(prev_ld_slice) => {
                // Skip until we see the prev_ld_slice value, and then get the next value.
                ld_slices_iter
                    .skip_while(|rld_slice| {
                        rld_slice
                            .as_ref()
                            .map_or(true, |ld_slice| *ld_slice != prev_ld_slice)
                    })
                    .nth(1)
            }
            None => ld_slices_iter.next(),
        }
        .transpose()?;
        self.prev_ld_slice = result.clone().or(self.prev_ld_slice.clone());
        Ok(result)
    }
}

impl<'bump, 'slice, 'par> InternalData<'bump> for InternalDataForSliceViewStruct<'slice, 'par> {
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}
