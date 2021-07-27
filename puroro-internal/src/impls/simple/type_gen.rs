use super::{LabelWrappedLdType, LabelWrappedMessageType, LabelWrappedType, SimpleImpl};
use crate::{AnyFieldTypeGen, EnumTypeGen, FieldTypeGen, MsgTypeGen, StructInternalTypeGen};
use ::puroro::tags;
use ::std::convert::TryFrom;

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
}

impl<X, L> EnumTypeGen<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedType,
    X: tags::EnumTypeForSyntax,
{
    type EnumType<E: PartialEq + Clone> =
        <L as LabelWrappedType>::Type<<X as tags::EnumTypeForSyntax>::NativeType<E>>;
    fn default<E: Default + TryFrom<i32> + PartialEq + Clone>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumType<E> {
        <L as LabelWrappedType>::default_with(<X as tags::EnumTypeForSyntax>::default)
    }

    fn clone<E: Clone + PartialEq>(
        from: &<Self as EnumTypeGen<X, L>>::EnumType<E>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<X, L>>::EnumType<E> {
        Clone::clone(from)
    }
}

impl<X, L> MsgTypeGen<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedMessageType,
{
    type MsgType<M: Clone> = <L as LabelWrappedMessageType>::Type<M>;
    fn default<M: Clone>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgType<M> {
        <L as LabelWrappedMessageType>::default()
    }

    fn clone<M: Clone>(
        from: &<Self as MsgTypeGen<X, L>>::MsgType<M>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgType<M> {
        Clone::clone(from)
    }
}
