use super::{LabelWrappedLdType, LabelWrappedType, SimpleImpl};
use crate::{EnumTypeGen, FieldTypeGen, MsgTypeGen, StructInternalTypeGen};
use ::puroro::tags;
use ::std::convert::TryFrom;

// For numerical types
impl<L, X, V, _1, _2> FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>> for SimpleImpl
where
    (X, tags::wire::NonLD<V, _1, _2>): tags::NumericalFieldTypeTag,
    L: LabelWrappedType,
{
    type Type = <L as LabelWrappedType>::Type<
        <(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::NativeType,
    >;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type {
        <L as LabelWrappedType>::default_with(
            <(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::default,
        )
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
}

impl<L> EnumTypeGen<tags::Proto2, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedType,
{
    type EnumType<E> = <L as LabelWrappedType>::Type<E>;
    fn default<E: Default>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<tags::Proto2, L>>::EnumType<E> {
        <L as LabelWrappedType>::default_with(Default::default)
    }
}

impl<L> EnumTypeGen<tags::Proto3, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedType,
{
    type EnumType<E> = <L as LabelWrappedType>::Type<::std::result::Result<E, i32>>;
    fn default<E: TryFrom<i32>>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<tags::Proto3, L>>::EnumType<E> {
        <L as LabelWrappedType>::default_with(|| TryFrom::try_from(0i32).map_err(|_| 0i32))
    }
}

impl<X, _1, _2> MsgTypeGen<X, tags::NonRepeated<_1, _2>> for SimpleImpl
where
    Self: StructInternalTypeGen,
{
    type MsgType<M> = Option<Box<M>>;
    fn default<M>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, tags::NonRepeated<_1, _2>>>::MsgType<M> {
        None
    }
}
impl<X> MsgTypeGen<X, tags::Repeated> for SimpleImpl {
    type MsgType<M> = Vec<M>;
    fn default<M>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, tags::Repeated>>::MsgType<M> {
        Vec::new()
    }
}
