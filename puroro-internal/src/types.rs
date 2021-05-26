use crate::deser::LdSlice;
use crate::internal_data::SourceLdSlices;
use crate::variant::Variant;
use crate::{Result, ResultHelper};
use ::num_derive::FromPrimitive;
use itertools::{Either, Itertools};

#[non_exhaustive]
#[derive(FromPrimitive, Debug)]
pub enum WireType {
    Variant = 0,
    Bits64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bits32 = 5,
}

#[derive(Debug, Clone)]
pub enum FieldData<T> {
    Variant(Variant),
    LengthDelimited(T),
    Bits32([u8; 4]),
    Bits64([u8; 8]),
}

#[derive(Debug, Clone)]
pub enum SliceViewField<'slice> {
    /// This field is embedded in a single slice. Simple.
    FieldInSingleSlice {
        /// A subslice of `enclosing_ld_slice` starting from this field's item.
        ld_slice: LdSlice<'slice>,
        /// A total number of field items in the slice. Packed repeated field is counted as 1.
        count: usize,
        enclosing_ld_slice: LdSlice<'slice>,
    },
    /// The field's owner message is constructed from multiple slices (and merged then),
    /// so we cannot store the source slice list in the stack memory.
    /// In this case, we need to get the list of LdSlices from puroro_internal field.
    FieldInMultipleSlices {
        count: usize,
        first_enclosing_ld_slice: LdSlice<'slice>,
    },
}

impl<'slice> SliceViewField<'slice> {
    /// Returns an iterator over the specifield field number's [`FieldData`] instance.
    /// Requires [`SourceLdSlices`] struct, which is owned by the messages's `puroro_internal` field.
    pub fn field_data_iter<'msg, 'par>(
        &'msg self,
        field_number: usize,
        source_ld_slices: &'msg SourceLdSlices<'slice, 'par>,
    ) -> impl 'msg + Iterator<Item = Result<FieldData<LdSlice<'slice>>>> {
        // The iter of `ld_slice` which consists the specified field.
        // Note that this might be a smaller set when compared with the `ld_slice`s consisting
        // the message struct. For example, even if there's a message consist of 3 separated slices,
        // but a certain field k can be consist of only the 2nd slice.
        let ld_slices = match self {
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
                    source_ld_slices
                        .iter()
                        .skip_while(move |rld_slice| match rld_slice.as_ref() {
                            Ok(ld_slice) => *ld_slice != *first_enclosing_ld_slice,
                            Err(_) => true,
                        })
                        .take(*count),
                )
            }
        };
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
