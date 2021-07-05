use super::{LabelWrappedType, LabelWrappedLDType, SimpleImpl};
use puroro::{tags, FieldTypeGen};
use std::borrow::Cow;

// For variant types
impl<L, X, V> FieldTypeGen<(L, (X, tags::wire::Variant<V>))> for SimpleImpl
where
    (X, tags::wire::Variant<V>): tags::NumericalFieldTypeTag,
    <(X, tags::wire::Variant<V>) as tags::NumericalFieldTypeTag>::NativeType: LabelWrappedType<L>,
{
    type Type = <
        <(X, tags::wire::Variant<V>) as tags::NumericalFieldTypeTag>::NativeType 
            as LabelWrappedType<L>
    >::Type;
}
// For bits32 types
impl<L, X, V> FieldTypeGen<(L, (X, tags::wire::Bits32<V>))> for SimpleImpl
where
    (X, tags::wire::Bits32<V>): tags::NumericalFieldTypeTag,
    <(X, tags::wire::Bits32<V>) as tags::NumericalFieldTypeTag>::NativeType: LabelWrappedType<L>,
{
    type Type = <
        <(X, tags::wire::Bits32<V>) as tags::NumericalFieldTypeTag>::NativeType 
            as LabelWrappedType<L>
    >::Type;
}
// For bits64 types
impl<L, X, V> FieldTypeGen<(L, (X, tags::wire::Bits64<V>))> for SimpleImpl
where
    (X, tags::wire::Bits64<V>): tags::NumericalFieldTypeTag,
    <(X, tags::wire::Bits64<V>) as tags::NumericalFieldTypeTag>::NativeType: LabelWrappedType<L>,
{
    type Type = <
        <(X, tags::wire::Bits64<V>) as tags::NumericalFieldTypeTag>::NativeType 
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

impl<X, M> FieldTypeGen<(tags::Required, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;
}
impl<X, M> FieldTypeGen<(tags::Optional, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;
}
impl<X, M> FieldTypeGen<(tags::Unlabeled, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;
}
impl<X, M> FieldTypeGen<(tags::Repeated, (X, tags::Message<M>))> for SimpleImpl {
    type Type = Vec<M>;
}
