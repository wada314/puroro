use std::borrow::Cow;

use super::{LabelWrappedLdType, LabelWrappedMessageType, LabelWrappedType, SimpleImpl};
use crate::{AnyFieldTypeGen, EnumTypeGen, FieldTypeGen, MsgTypeGen, StructInternalTypeGen};
use ::puroro::bool::True;
use ::puroro::{tags, Enum, Message};

// All-in-one typegen trait
impl AnyFieldTypeGen for SimpleImpl {}

// For numerical types
impl<L, X, V, _1, _2> FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>> for SimpleImpl
where
    tags::wire::NonLD<V, _1, _2>: tags::NumericalTypeTag,
    L: LabelWrappedType,
{
    type Type = <L as LabelWrappedType>::Type<
        <tags::wire::NonLD<V, _1, _2> as tags::NumericalTypeTag>::NativeType,
    >;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type {
        <L as LabelWrappedType>::default_with(Default::default)
    }

    fn clone(
        from: &<Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type {
        Clone::clone(from)
    }

    type ScalarGetterType<'this> =
        <tags::wire::NonLD<V, _1, _2> as tags::NumericalTypeTag>::NativeType;
    fn get_scalar<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::ScalarGetterType<'this>
    where
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>,
    {
        <L as LabelWrappedType>::get_scalar(from)
    }
}

// For length delimited types

impl<L, X> FieldTypeGen<X, L, tags::Bytes> for SimpleImpl
where
    (X, L): LabelWrappedLdType,
{
    type Type = <(X, L) as LabelWrappedLdType>::Type<[u8]>;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::Bytes>>::Type {
        <(X, L) as LabelWrappedLdType>::default::<[u8]>()
    }

    fn clone(
        from: &<Self as FieldTypeGen<X, L, tags::Bytes>>::Type,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::Bytes>>::Type {
        Clone::clone(from)
    }

    type ScalarGetterType<'this> = Cow<'this, [u8]>;
    fn get_scalar<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::Bytes>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::ScalarGetterType<'this>
    where
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>,
    {
        <(X, L) as LabelWrappedLdType>::get_scalar(from)
    }
}
impl<L, X> FieldTypeGen<X, L, tags::String> for SimpleImpl
where
    (X, L): LabelWrappedLdType,
{
    type Type = <(X, L) as LabelWrappedLdType>::Type<str>;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::String>>::Type {
        <(X, L) as LabelWrappedLdType>::default::<str>()
    }

    fn clone(
        from: &<Self as FieldTypeGen<X, L, tags::String>>::Type,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::String>>::Type {
        Clone::clone(from)
    }

    type ScalarGetterType<'this> = Cow<'this, str>;
    fn get_scalar<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::String>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::ScalarGetterType<'this>
    where
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>,
    {
        <(X, L) as LabelWrappedLdType>::get_scalar(from)
    }
}

impl<X, L> EnumTypeGen<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedType,
    X: tags::EnumTypeForSyntax,
{
    type EnumFieldType<E: Enum> =
        <L as LabelWrappedType>::Type<<X as tags::EnumTypeForSyntax>::NativeType<E>>;
    fn default<E: Enum>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumFieldType<E> {
        <L as LabelWrappedType>::default_with(<X as tags::EnumTypeForSyntax>::default)
    }

    fn clone<E: Enum>(
        from: &<Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumFieldType<E> {
        Clone::clone(from)
    }
}

impl<X, L> MsgTypeGen<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedMessageType,
{
    type MsgFieldType<M: Message> = <L as LabelWrappedMessageType>::Type<M>;
    type MsgTypeInTrait<M: Message> = M;

    fn default<M: Message>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgFieldType<M> {
        <L as LabelWrappedMessageType>::default()
    }

    fn clone<M: Message>(
        from: &<Self as MsgTypeGen<X, L>>::MsgFieldType<M>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgFieldType<M> {
        Clone::clone(from)
    }
}
