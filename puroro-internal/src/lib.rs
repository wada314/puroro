#![allow(incomplete_features)]
#![feature(generic_associated_types)]
pub mod de;
pub mod impls;
pub mod se;
mod traits;

use ::puroro::{tags, Enum, ErrorKind, Message, Result};

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

// Impl symbols
pub use de::from_iter::ScopedIter;
pub use de::DeserAnyFieldFromBytesIter;
pub use impls::simple::SimpleImpl;
pub use se::SerAnyFieldToIoWrite;

pub trait MessageInternal: ::puroro::Message {
    type ImplTypeTag: StructInternalTypeGen;
    fn new_with_internal_data(
        internal_data: <Self::ImplTypeTag as StructInternalTypeGen>::Type,
    ) -> Self;
}

pub trait ChooseStructVisibility<Public, Private> {
    type Type;
}

pub trait StructInternalTypeGen: tags::ImplTypeTag {
    type Type: Clone;
}

pub trait FieldTypeGen<X, L, V>: StructInternalTypeGen {
    type Type;
    /// Default value of the field when the message is allocated
    fn default(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, V>>::Type;
    /// Clone the field type
    fn clone(
        from: &<Self as FieldTypeGen<X, L, V>>::Type,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, V>>::Type;
}
pub trait EnumTypeGen<X, L>: StructInternalTypeGen {
    type EnumType<E: Enum>;
    /// Default value of the field when the message is allocated
    fn default<E: Enum>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumType<E>;
    /// Clone the field type
    fn clone<E: Enum>(
        from: &<Self as EnumTypeGen<X, L>>::EnumType<E>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumType<E>;
}
pub trait MsgTypeGen<X, L>: StructInternalTypeGen {
    type MsgType<M: Message>;
    /// Default value of the field when the message is allocated
    fn default<M: Message>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgType<M>;
    /// Clone the field type
    fn clone<M: Message>(
        from: &<Self as MsgTypeGen<X, L>>::MsgType<M>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgType<M>;
}

pub trait AnyFieldTypeGen:
    EnumTypeGen<tags::Proto2, tags::Required>
    + EnumTypeGen<tags::Proto2, tags::Optional>
    + EnumTypeGen<tags::Proto2, tags::Repeated>
    + EnumTypeGen<tags::Proto3, tags::Unlabeled>
    + EnumTypeGen<tags::Proto3, tags::Optional>
    + EnumTypeGen<tags::Proto3, tags::Repeated>
    + MsgTypeGen<tags::Proto2, tags::Required>
    + MsgTypeGen<tags::Proto2, tags::Optional>
    + MsgTypeGen<tags::Proto2, tags::Repeated>
    + MsgTypeGen<tags::Proto3, tags::Unlabeled>
    + MsgTypeGen<tags::Proto3, tags::Optional>
    + MsgTypeGen<tags::Proto3, tags::Repeated>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Int32>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::UInt32>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::SInt32>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Int64>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::UInt64>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::SInt64>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Bool>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Bytes>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::String>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Float>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Double>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::SFixed32>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Fixed32>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::SFixed64>
    + FieldTypeGen<tags::Proto2, tags::Required, tags::Fixed64>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Int32>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::UInt32>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::SInt32>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Int64>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::UInt64>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::SInt64>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Bool>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Bytes>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::String>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Float>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Double>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::SFixed32>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Fixed32>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::SFixed64>
    + FieldTypeGen<tags::Proto2, tags::Optional, tags::Fixed64>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Int32>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::UInt32>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::SInt32>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Int64>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::UInt64>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::SInt64>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Bool>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Bytes>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::String>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Float>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Double>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::SFixed32>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Fixed32>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::SFixed64>
    + FieldTypeGen<tags::Proto2, tags::Repeated, tags::Fixed64>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Int32>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::UInt32>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::SInt32>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Int64>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::UInt64>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::SInt64>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Bool>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Bytes>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::String>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Float>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Double>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::SFixed32>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Fixed32>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::SFixed64>
    + FieldTypeGen<tags::Proto3, tags::Unlabeled, tags::Fixed64>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Int32>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::UInt32>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::SInt32>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Int64>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::UInt64>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::SInt64>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Bool>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Bytes>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::String>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Float>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Double>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::SFixed32>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Fixed32>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::SFixed64>
    + FieldTypeGen<tags::Proto3, tags::Optional, tags::Fixed64>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Int32>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::UInt32>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::SInt32>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Int64>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::UInt64>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::SInt64>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Bool>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Bytes>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::String>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Float>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Double>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::SFixed32>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Fixed32>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::SFixed64>
    + FieldTypeGen<tags::Proto3, tags::Repeated, tags::Fixed64>
{
}
