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
    pub source_ld_slices: Option<SourceLdSlices<'slice, 'par>>,
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
    pub fn new() -> Self {
        Self {
            source_ld_slices: None,
        }
    }

    pub fn new_with_slice(slice: &'slice [u8]) -> Self {
        Self {
            source_ld_slices: Some(SourceLdSlices::SingleLdSlice(LdSlice::new(slice))),
        }
    }

    pub fn new_with_parent(
        field_in_parent: Option<&'par SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,
    ) -> Self {
        Self {
            source_ld_slices: Some(SourceLdSlices::MaybeMultipleLdSlices {
                field_in_parent,
                field_number_in_parent,
                parent_internal_data,
            }),
        }
    }
}

impl<'slice, 'par> SourceLdSlices<'slice, 'par> {
    /// Get the list of the source slices of this Message.
    /// If your purpose to get a certain field's data then make sure that field ([`SliceViewField`])'s
    /// variant is [`SliceViewField::FieldInMultipleSlices`]. If it is [`SliceViewField::FieldInSingleSlice`],
    /// then you can use that enum variant's `ld_slice` field for shortcut.
    /// Note that iterating over this iterator takes O(n^2) time in total where n is the iterator length
    /// (Where it is O(n) in a normal iterator).
    /// We believe n (== the number of merged messages) is very small in most usecases.
    pub fn iter(&self) -> impl '_ + Iterator<Item = Result<LdSlice<'slice>>> {
        match self.clone() {
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
}

impl<'slice, 'par> IntoIterator for &SourceLdSlices<'slice, 'par> {
    type Item = Result<LdSlice<'slice>>;
    type IntoIter = impl Iterator<Item = Result<LdSlice<'slice>>>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct MultipleSourceLdSlicesIter<'slice, 'par> {
    field_number: usize,
    maybe_field: Option<&'par SliceViewField<'slice>>,
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
        maybe_field: Option<&'par SliceViewField<'slice>>,
        internal_data: &'par InternalDataForSliceViewStruct<'slice, 'par>,
    ) -> Self {
        Self {
            field_number,
            maybe_field,
            internal_data,
            prev_ld_slice: None,
        }
    }

    fn try_next(&mut self) -> Result<Option<LdSlice<'slice>>> {
        // An iterator of ld_slices which consists the message struct.
        // Note that this iterator type contains this [`MultipleSourceLdSlicesIter`] type
        // so we cannot remember this iterator as this struct's item.
        // (Memory allocation is prohibited in SliceView methods)
        let mut ld_slices_iter = self.maybe_field.into_iter().flat_map(|field| {
            field
                .field_data_iter(self.field_number, &self.internal_data.source_ld_slices)
                .filter_map_ok(|field_data| -> Option<Result<_>> {
                    if let FieldData::LengthDelimited(ld_slice) = field_data {
                        Some(Ok(ld_slice))
                    } else {
                        Some(Err(ErrorKind::UnexpectedWireType.into()))
                    }
                })
                .map(|rrfield| rrfield.flatten())
        });
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
