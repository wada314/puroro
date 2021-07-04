#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![allow(incomplete_features)]

mod error;

pub mod de;
pub mod tags;
pub mod variant;
use de::from_iter::ScopedIter;
pub use error::{ErrorKind, PuroroError};
pub type Result<T> = std::result::Result<T, PuroroError>;
pub use de::FieldData;

// Re-exports
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::hashbrown;

pub trait Message {
    type ImplTypeTag;
}

pub trait StructInternalTypeGen: tags::ImplTypeTag {
    type Type;
}

pub trait FieldTypeGen<LabelAndType>: tags::ImplTypeTag
// Not setting these bounds for code simplicity
// where
//    LT: tags::FieldLabelAndTypeTag,
{
    type Type;
}

pub trait DeserFieldFromBytesIter<LabelAndType>: FieldTypeGen<LabelAndType> {
    fn deser_from_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<I>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        Self::deser_from_scoped_bytes_iter(field, data.map(|iter| ScopedIter::new(iter)).as_mut())
    }

    fn deser_from_scoped_bytes_iter<I>(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&mut de::from_iter::ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}

pub trait DeserFieldFromBytesSlice<LabelAndType>: FieldTypeGen<LabelAndType> {
    fn deser_from_bytes_slice(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&[u8]>,
    ) -> Result<()>;
}

impl<T, LabelAndType> DeserFieldFromBytesSlice<LabelAndType> for T
where
    T: DeserFieldFromBytesIter<LabelAndType>,
{
    fn deser_from_bytes_slice(
        field: &mut <Self as FieldTypeGen<LabelAndType>>::Type,
        data: FieldData<&[u8]>,
    ) -> Result<()> {
        use std::io::Read as _;
        <Self as DeserFieldFromBytesIter<LabelAndType>>::deser_from_bytes_iter(
            field,
            data.map(|slice| slice.bytes()),
        )
    }
}
