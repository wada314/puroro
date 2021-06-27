use crate::deser::LdSlice;
use crate::types::{FieldData, SliceViewField};
use crate::{hashbrown, ErrorKind, Result, ResultHelper};
use ::itertools::{Either, Itertools};
use ::puroro::tags;
use ::puroro::InternalData;
use ::std::collections::HashMap;
use std::marker::PhantomData;

pub trait GetInternalDataType {
    type Type: InternalData;
}

#[derive(Debug, Clone)]
pub struct InternalDataForSimpleStruct {
    unknown_fields: HashMap<usize, FieldData<Vec<u8>>>,
}
impl GetInternalDataType for tags::SimpleStruct {
    type Type = InternalDataForSimpleStruct;
}

impl InternalDataForSimpleStruct {
    pub fn new() -> Self {
        Self {
            unknown_fields: HashMap::new(),
        }
    }
}
impl InternalData for InternalDataForSimpleStruct {
    #[cfg(feature = "puroro-bumpalo")]
    fn bumpalo(&self) -> &crate::bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}

#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct InternalDataForBumpaloStruct<'bump> {
    unknown_fields: hashbrown::HashMap<
        usize,
        FieldData<crate::bumpalo::collections::Vec<'bump, u8>>,
        hashbrown::hash_map::DefaultHashBuilder,
        hashbrown::BumpWrapper<'bump>,
    >,
    pub bump: &'bump crate::bumpalo::Bump,
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> GetInternalDataType for tags::Bumpalo<'bump> {
    type Type = InternalDataForBumpaloStruct<'bump>;
}

#[cfg(feature = "puroro-bumpalo")]
impl<'bump> InternalDataForBumpaloStruct<'bump> {
    pub fn new_with_bumpalo(bump: &'bump crate::bumpalo::Bump) -> Self {
        Self {
            unknown_fields: hashbrown::HashMap::new_in(hashbrown::BumpWrapper(bump)),
            bump,
        }
    }
}
impl<'bump> InternalData for InternalDataForBumpaloStruct<'bump> {
    fn bumpalo(&self) -> &crate::bumpalo::Bump {
        self.bump
    }
}

#[derive(Debug, Clone)]
pub struct InternalDataForSliceViewStruct<'slice, S> {
    pub maybe_source_slices: Option<S>,
    phantom: PhantomData<&'slice ()>,
}
impl<'slice, S> GetInternalDataType for tags::SliceView<'slice, S> {
    type Type = InternalDataForSliceViewStruct<'slice, S>;
}

pub trait SliceSource<'slice>: Clone {
    type Iter: Iterator<Item = Result<LdSlice<'slice>>>;
    fn into_iter(self) -> Self::Iter;
}

impl<'slice> SliceSource<'slice> for &'slice [u8] {
    type Iter = impl Iterator<Item = Result<LdSlice<'slice>>>;

    fn into_iter(self) -> Self::Iter {
        std::iter::once(Ok(LdSlice::new(self)))
    }
}

#[derive(Debug, Clone)]
pub enum SourceLdSlices<'slice, 'par, S> {
    SingleLdSlice(LdSlice<'slice>),
    MaybeMultipleLdSlices {
        field_in_parent: Option<&'par SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par InternalDataForSliceViewStruct<'slice, S>,
    },
}

impl<'slice, S> InternalDataForSliceViewStruct<'slice, S> {
    pub fn new() -> Self {
        Self {
            maybe_source_slices: None,
            phantom: PhantomData,
        }
    }
}

impl<'slice> InternalDataForSliceViewStruct<'slice, &'slice [u8]> {
    pub fn new_with_slice(slice: &'slice [u8]) -> Self {
        Self {
            maybe_source_slices: Some(slice),
            phantom: PhantomData,
        }
    }
}

impl<'slice, 'par, SS> InternalDataForSliceViewStruct<'slice, SourceLdSlices<'slice, 'par, SS>> {
    pub fn new_with_parent(
        field_in_parent: Option<&'par SliceViewField<'slice>>,
        field_number_in_parent: usize,
        parent_internal_data: &'par InternalDataForSliceViewStruct<'slice, SS>,
    ) -> Self {
        match field_in_parent {
            None => Self {
                maybe_source_slices: None,
                phantom: PhantomData,
            },
            Some(_) => Self {
                maybe_source_slices: Some(SourceLdSlices::MaybeMultipleLdSlices {
                    field_in_parent,
                    field_number_in_parent: field_number_in_parent,
                    parent_internal_data: parent_internal_data,
                }),
                phantom: PhantomData,
            },
        }
    }
}

impl<'slice, S> InternalDataForSliceViewStruct<'slice, S>
where
    S: SliceSource<'slice>,
{
    pub fn source_ld_slices(&self) -> impl Iterator<Item = Result<LdSlice<'slice>>> {
        self.maybe_source_slices
            .clone()
            .into_iter()
            .flat_map(|source| source.into_iter())
    }
}

impl<'slice, 'par, S> SliceSource<'slice> for SourceLdSlices<'slice, 'par, S>
where
    S: SliceSource<'slice>,
{
    type Iter = impl Iterator<Item = Result<LdSlice<'slice>>>;
    /// Get the list of the source slices of this Message.
    /// If your purpose to get a certain field's data then make sure that field ([`SliceViewField`])'s
    /// variant is [`SliceViewField::FieldInMultipleSlices`]. If it is [`SliceViewField::FieldInSingleSlice`],
    /// then you can use that enum variant's `ld_slice` field for shortcut.
    fn into_iter(self) -> Self::Iter {
        match self.clone() {
            SourceLdSlices::SingleLdSlice(ld_slice) => Either::Left(std::iter::once(Ok(ld_slice))),
            SourceLdSlices::MaybeMultipleLdSlices {
                field_in_parent,
                field_number_in_parent,
                parent_internal_data,
            } => Either::Right(
                field_in_parent
                    .into_iter()
                    .flat_map(move |field| {
                        parent_internal_data
                            .maybe_source_slices
                            .clone()
                            .into_iter()
                            .flat_map(move |source_slices| {
                                field.field_data_iter(field_number_in_parent, source_slices)
                            })
                    })
                    .map_ok(|field_data| -> Result<_> {
                        if let FieldData::LengthDelimited(ld_slice) = field_data {
                            Ok(ld_slice)
                        } else {
                            Err(ErrorKind::UnexpectedWireType)?
                        }
                    })
                    .map(|rrldslice| rrldslice.flatten()),
            ),
        }
        .into_iter()
    }
}

impl<'slice, 'par, S> IntoIterator for &SourceLdSlices<'slice, 'par, S>
where
    S: SliceSource<'slice>,
{
    type Item = Result<LdSlice<'slice>>;
    type IntoIter = impl Iterator<Item = Result<LdSlice<'slice>>>;
    fn into_iter(self) -> Self::IntoIter {
        <SourceLdSlices<'slice, 'par, S> as SliceSource<'slice>>::into_iter(self.clone())
    }
}

impl<'slice, S> InternalData for InternalDataForSliceViewStruct<'slice, S> {
    fn bumpalo(&self) -> &crate::bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}
