#![allow(incomplete_features)]
#![feature(generic_associated_types)]
pub mod de;
pub mod impls;
pub mod se;

use ::std::convert::TryFrom;

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
    fn new_with_parents_internal_data(
        parents_internal_data: &<Self::ImplTypeTag as StructInternalTypeGen>::Type,
    ) -> Self;
}

pub trait ChooseStructVisibility<Public, Private> {
    type Type;
}

pub trait StructInternalTypeGen: tags::ImplTypeTag {
    type Type;
}

pub trait FieldTypeGen<X, L, V>: StructInternalTypeGen {
    type Type;
    /// Default value of the field when the message is allocated
    fn default(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, V>>::Type;
}
pub trait EnumTypeGen<X, L>: StructInternalTypeGen {
    type EnumType<E: PartialEq>;
    /// Default value of the field when the message is allocated
    fn default<E: Default + TryFrom<i32> + PartialEq>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumType<E>;
}
pub trait MsgTypeGen<X, L>: StructInternalTypeGen {
    type MsgType<M>;
    /// Default value of the field when the message is allocated
    fn default<M>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgType<M>;
}
