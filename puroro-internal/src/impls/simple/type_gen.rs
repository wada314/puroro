use super::{LabelWrappedType, LabelWrappedLdType, SimpleImpl};
use ::puroro::{tags};
use crate::{FieldTypeGen, StructInternalTypeGen, MsgTypeGen, EnumTypeGen};

// For numerical types
impl<L, X, V, _1, _2> FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>> for SimpleImpl
where
    (X, tags::wire::NonLD<V, _1, _2>): tags::NumericalFieldTypeTag,
    <(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::NativeType: LabelWrappedType<L>,
{
    type Type = <
        <(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::NativeType 
            as LabelWrappedType<L>
    >::Type;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type {
        <<(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::NativeType as LabelWrappedType<L>>::default_with(
            <(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::default
        )
    }
}

// For length delimited types

impl<L, X> FieldTypeGen<X, L, tags::Bytes> for SimpleImpl 
where [u8]: LabelWrappedLdType<L, X>
{
    type Type = <[u8] as LabelWrappedLdType<L, X>>::Type;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L, tags::Bytes>>::Type {
        <[u8] as LabelWrappedLdType<L, X>>::default()
    }
    
}
impl<L, X> FieldTypeGen<X, L, tags::String> for SimpleImpl 
where str: LabelWrappedLdType<L, X>
{
    type Type = <str as LabelWrappedLdType<L, X>>::Type;
    
    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<X, L,  tags::String>>::Type {
        <str as LabelWrappedLdType<L, X>>::default()
    }
}

impl<L> EnumTypeGen<tags::Proto2, L> for SimpleImpl where Self: StructInternalTypeGen {
    type EnumType<E> = <E as LabelWrappedType<L>>::Type;
    fn default<E: Default>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<tags::Proto2, L>>::EnumType<E> {
        <E as LabelWrappedType<L, tags::Proto2>>::default_with(Default::default)
    }
}

impl<L> EnumTypeGen<tags::Proto3, L> for SimpleImpl where Self: StructInternalTypeGen {
    type EnumType<E> = <::std::result::Result<E, i32> as LabelWrappedType<L>>::Type;
    fn default<E: Default>(
        internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as EnumTypeGen<tags::Proto3, L>>::EnumType<E> {
        <E as LabelWrappedType<L, tags::Proto3>>::default_with(Default::default)
    }
}

impl<X, _1, _2> MsgTypeGen<X, tags::NonRepeated<_1, _2>> for SimpleImpl
where Self: StructInternalTypeGen
{
    type MsgType<M> = Option<Box<M>>;
    fn default<M>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, tags::NonRepeated<_1, _2>>>::MsgType<M> {
        None
    }
}
impl<X> MsgTypeGen<X, tags::Repeated> for SimpleImpl
{
    type MsgType<M> = Vec<M>;
    fn default<M>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, tags::Repeated>>::MsgType<M> {
        Vec::new()
    }
}
