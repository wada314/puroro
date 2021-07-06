#![allow(unstable_name_collisions)] // For ResultHelper::flatten.

pub mod de;
pub mod impls;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

// Impl symbols
pub use impls::simple::SimpleImpl;

use ::puroro::{tags, ErrorKind, Result};

pub use de::from_iter::ScopedIter;

pub trait MessageInternal: ::puroro::Message {
    type ImplTypeTag: StructInternalTypeGen;
    fn new_with_internal_data(
        internal_data: <Self::ImplTypeTag as StructInternalTypeGen>::Type,
    ) -> Self;
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
