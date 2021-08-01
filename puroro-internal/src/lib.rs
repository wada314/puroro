#![allow(incomplete_features)]
#![feature(generic_associated_types)]
pub mod de;
pub mod impls;
pub mod se;

use ::puroro::bool::{False, True};
use ::puroro::{tags, Enum, ErrorKind, Message, RepeatedField, Result};
use ::std::borrow::Cow;

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

type ScalarTypeForTrait<'this, V> = <V as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>;
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

    // Trait method implementations

    /// Get the scalar type for the trait getter method.
    fn get_scalar<'this>(
        from: &'this <Self as FieldTypeGen<X, L, V>>::Type,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> ScalarTypeForTrait<'this, V>
    where
        V: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>;
    /// Get `Option` wrapped scalar type for the trait getter method.
    fn get_scalar_optional<'this>(
        from: &'this <Self as FieldTypeGen<X, L, V>>::Type,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<ScalarTypeForTrait<'this, V>>
    where
        V: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsOptionalScalar = True>;
    /// Repeated field type for the trait.
    type TraitRepeatedFieldType<'this>: RepeatedField<'this, ScalarTypeForTrait<'this, V>>
    where
        V: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>;
    /// Get repeated field for the trait getter method.
    fn get_repeated<'this>(
        from: &'this <Self as FieldTypeGen<X, L, V>>::Type,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this>
    where
        V: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>;
}
pub trait EnumTypeGen<X, L>: StructInternalTypeGen {
    type EnumFieldType<E: Enum>;
    /// Default value of the field when the message is allocated
    fn default<E: Enum>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumFieldType<E>;
    /// Clone the field type
    fn clone<E: Enum>(
        from: &<Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumFieldType<E>;

    // Trait method implementations

    /// Get the scalar type for the trait getter method.
    fn get_scalar<'this, E: Enum>(
        from: &'this <Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> <X as tags::EnumTypeForSyntax>::NativeType<E>
    where
        X: tags::EnumTypeForSyntax,
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>;
    /// Get `Option` wrapped scalar type for the trait getter method.
    fn get_scalar_optional<'this, E: Enum>(
        from: &'this <Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<<X as tags::EnumTypeForSyntax>::NativeType<E>>
    where
        X: tags::EnumTypeForSyntax,
        L: tags::FieldLabelTag<IsOptionalScalar = True>;
    /// Repeated field type for the trait.
    type TraitRepeatedFieldType<'this, E: Enum>: RepeatedField<
        'this,
        <X as tags::EnumTypeForSyntax>::NativeType<E>,
    >
    where
        X: tags::EnumTypeForSyntax,
        <X as tags::EnumTypeForSyntax>::NativeType<E>: 'this,
        L: tags::FieldLabelTag<IsRepeated = True>;
    /// Get repeated field for the trait getter method.
    fn get_repeated<'this, E: Enum>(
        from: &'this <Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this, E>
    where
        X: tags::EnumTypeForSyntax,
        <X as tags::EnumTypeForSyntax>::NativeType<E>: 'this,
        L: tags::FieldLabelTag<IsRepeated = True>;
}
pub trait MsgTypeGen<X, L>: StructInternalTypeGen {
    type MsgFieldType<M: Message>;
    /// Default value of the field when the message is allocated
    fn default<M: Message>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgFieldType<M>;
    /// Clone the field type
    fn clone<M: Message>(
        from: &<Self as MsgTypeGen<X, L>>::MsgFieldType<M>,
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgFieldType<M>;

    // Trait method implementations

    /// The concrete message type for the `MessageType` type in `FieldN` traits.
    type MsgTypeInTrait<'this, M: Message>: Message;
    /// Get the scalar type for the trait getter method.
    fn get_scalar_optional<'this, M: Message>(
        from: &'this <Self as MsgTypeGen<X, L>>::MsgFieldType<M>,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<Cow<'this, Self::MsgTypeInTrait<'this, M>>>
    where
        L: tags::FieldLabelTag<IsRepeated = False>;

    /// Repeated field type for the trait.
    type TraitRepeatedFieldType<'this, M: Message>: RepeatedField<
        'this,
        Cow<'this, Self::MsgTypeInTrait<'this, M>>,
    >
    where
        Self::MsgTypeInTrait<'this, M>: 'this;

    /// Get repeated field for the trait getter method.
    fn get_repeated<'this, M: Message>(
        from: &'this <Self as MsgTypeGen<X, L>>::MsgFieldType<M>,
        internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this, M>
    where
        L: tags::FieldLabelTag<IsRepeated = True>;
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
