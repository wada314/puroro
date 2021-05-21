use crate::deser::LdSlice;
use crate::variant::Variant;
use ::num_derive::FromPrimitive;

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
    /// so we cannot store the arbitrary length list of the source slice
    /// in a stack memory.
    /// In this case, we need to get the list of LdSlices from puroro_internal field.
    FieldInMultipleSlices {
        count: usize,
        first_enclosing_ld_slice: LdSlice<'slice>,
    },
}
