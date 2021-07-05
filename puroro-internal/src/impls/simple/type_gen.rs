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

    fn default(
        _internal_data: &<Self as puroro::StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(L, (X, tags::wire::NonLD<V, _1, _2>))>>::Type {
        <<(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::NativeType as LabelWrappedType<L>>::default_with(
            <(X, tags::wire::NonLD<V, _1, _2>) as tags::NumericalFieldTypeTag>::default
        )
    }
}

// For length delimited types

impl<L, X> FieldTypeGen<(L, (X, tags::Bytes))> for SimpleImpl 
where [u8]: LabelWrappedLDType<L, X>
{
    type Type = <[u8] as LabelWrappedLDType<L, X>>::Type;

    fn default(
        _internal_data: &<Self as puroro::StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(L, (X, tags::Bytes))>>::Type {
        <[u8] as LabelWrappedLDType<L, X>>::default()
    }
    
}
impl<L, X> FieldTypeGen<(L, (X, tags::String))> for SimpleImpl 
where str: LabelWrappedLDType<L, X>
{
    type Type = <str as LabelWrappedLDType<L, X>>::Type;
    
    fn default(
        _internal_data: &<Self as puroro::StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(L, (X, tags::String))>>::Type {
        <str as LabelWrappedLDType<L, X>>::default()
    }
}

impl<X, M, _1, _2> FieldTypeGen<(tags::NonRepeated<_1, _2>, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;

    fn default(
        _internal_data: &<Self as puroro::StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(tags::NonRepeated<_1, _2>, (X, tags::Message<M>))>>::Type {
        None
    }
}
impl<X, M> FieldTypeGen<(tags::Repeated, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Vec<M>;

    fn default(
        _internal_data: &<Self as puroro::StructInternalTypeGen>::Type,
    ) -> <Self as FieldTypeGen<(tags::Repeated, (X, tags::Message<M>))>>::Type {
        Vec::new()
    }
}
