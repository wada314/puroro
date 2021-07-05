use super::{LabelWrappedType, LabelWrappedLDType, SimpleImpl};
use puroro::{tags, FieldTypeGen};

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
}

// For length delimited types

impl<L, X> FieldTypeGen<(L, (X, tags::Bytes))> for SimpleImpl 
where [u8]: LabelWrappedLDType<L, X>
{
    type Type = <[u8] as LabelWrappedLDType<L, X>>::Type;
}
impl<L, X> FieldTypeGen<(L, (X, tags::String))> for SimpleImpl 
where str: LabelWrappedLDType<L, X>
{
    type Type = <str as LabelWrappedLDType<L, X>>::Type;
}

impl<X, M, _1, _2> FieldTypeGen<(tags::NonRepeated<_1, _2>, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;
}
impl<X, M> FieldTypeGen<(tags::Repeated, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Vec<M>;
}
