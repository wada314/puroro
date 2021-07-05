#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![allow(incomplete_features)]

mod error;

pub mod de;
pub mod tags;
pub mod variant;
pub use error::{ErrorKind, PuroroError};
pub type Result<T> = std::result::Result<T, PuroroError>;
pub use de::FieldData;

// Re-exports
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::hashbrown;

use de::from_iter::ScopedIter;

pub trait Message {
    type ImplTypeTag: StructInternalTypeGen;
    fn new_with_internal_data(
        internal_data: <Self::ImplTypeTag as StructInternalTypeGen>::Type,
    ) -> Self;
}
pub trait DeserFromBytesIter: Message {
    fn deser<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait StructInternalTypeGen: tags::ImplTypeTag {
    type Type;
}

pub trait FieldTypeGen<LabelAndType>: tags::ImplTypeTag + StructInternalTypeGen
// Not setting these bounds for code simplicity
// where
//    LT: tags::FieldLabelAndTypeTag,
{
    type Type;
    /// Default value of the field when the message is allocated
    fn default(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<LabelAndType>>::Type;
}

pub trait DeserFieldFromBytesIter<LabelAndType>:
    FieldTypeGen<LabelAndType> + StructInternalTypeGen
{
    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&mut de::from_iter::ScopedIter<I>>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserFieldFromBytesSlice<LabelAndType>:
    FieldTypeGen<LabelAndType> + StructInternalTypeGen
{
    fn deser_from_bytes_slice(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&[u8]>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()>;
}

impl<T, LabelAndType> DeserFieldFromBytesSlice<LabelAndType> for T
where
    T: DeserFieldFromBytesIter<LabelAndType>,
{
    fn deser_from_bytes_slice(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&[u8]>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> Result<()> {
        use std::io::Read as _;
        <Self as DeserFieldFromBytesIter<LabelAndType>>::deser_from_scoped_bytes_iter(
            field,
            data.map(|slice| ScopedIter::new(slice.bytes())).as_mut(),
            internal_data,
        )
    }
}
