#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![allow(incomplete_features)]

mod error;

pub mod tags;

pub use error::{ErrorKind, PuroroError};
pub type Result<T> = std::result::Result<T, PuroroError>;

// Re-exports
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::hashbrown;

pub trait FunctorForFieldMut {
    type ImplTypeTag;
    fn apply_mut<LabelAndType>(
        &mut self,
        field: &mut <Self::ImplTypeTag as FieldTypeGen<LabelAndType>>::Type,
    ) -> Result<()>
    where
        Self::ImplTypeTag: FieldTypeGen<LabelAndType>;
}

pub trait Message {
    type ImplTypeTag;
    // TODO: When the field does not exists
    fn apply_mut_to_field_with_number<F>(&mut self, number: i32, f: F) -> Result<()>
    where
        F: FunctorForFieldMut<ImplTypeTag = Self::ImplTypeTag>;
}

pub trait StructInternalTypeGen: tags::ImplTypeTag {
    type Type;
}

pub trait FieldTypeGen<LT>: tags::ImplTypeTag
// Not setting these bounds for code simplicity
// where
//    LT: tags::FieldLabelAndTypeTag,
{
    type Type;
}
