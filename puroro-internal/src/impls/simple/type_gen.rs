use super::{LabelWrappedType, LabelWrappedLdType, SimpleImpl};
use ::puroro::{tags, GetImpl};
use crate::{FieldTypeGen, StructInternalTypeGen};

// For numerical types
impl<L, X, V, _1, _2> FieldTypeGen<(L, (X, tags::wire::NonLD<V, _1, _2>))> for SimpleImpl
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
    ) -> <Self as FieldTypeGen<(L, (X, tags::wire::NonLD<V, _1, _2>))>>::Type {
        <<(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::NativeType as LabelWrappedType<L>>::default_with(
            <(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::default
        )
    }
}

// For length delimited types

impl<L, X> FieldTypeGen<(L, (X, tags::Bytes))> for SimpleImpl 
where [u8]: LabelWrappedLdType<L, X>
{
    type Type = <[u8] as LabelWrappedLdType<L, X>>::Type;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(L, (X, tags::Bytes))>>::Type {
        <[u8] as LabelWrappedLdType<L, X>>::default()
    }
    
}
impl<L, X> FieldTypeGen<(L, (X, tags::String))> for SimpleImpl 
where str: LabelWrappedLdType<L, X>
{
    type Type = <str as LabelWrappedLdType<L, X>>::Type;
    
    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(L, (X, tags::String))>>::Type {
        <str as LabelWrappedLdType<L, X>>::default()
    }
}

impl<X, M, _1, _2> FieldTypeGen<(tags::NonRepeated<_1, _2>, (X, tags::Message<M>))> for SimpleImpl
    where M: GetImpl<SimpleImpl>
{
    type Type = Option<Box<<M as GetImpl<SimpleImpl>>::Type>>;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(tags::NonRepeated<_1, _2>, (X, tags::Message<M>))>>::Type {
        None
    }
}
impl<X, M> FieldTypeGen<(tags::Repeated, (X, tags::Message<M>))> for SimpleImpl 
    where M: GetImpl<SimpleImpl>
{
    type Type = Vec<<M as GetImpl<SimpleImpl>>::Type>;

    fn default(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(tags::Repeated, (X, tags::Message<M>))>>::Type {
        Vec::new()
    }
}
