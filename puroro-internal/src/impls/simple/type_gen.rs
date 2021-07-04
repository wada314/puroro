
use ::std::borrow::Cow;
use ::puroro::{tags, FieldTypeGen};
use super::{SimpleImpl, LabelWrappedType};

// For variant types
impl<L, S, V> FieldTypeGen<(L, (S, tags::wire::Variant<V>))> for SimpleImpl
where
    (S, tags::wire::Variant<V>): tags::NumericalFieldTypeTag,
    <
        (S, tags::wire::Variant<V>) as tags::NumericalFieldTypeTag
    >::NativeType: LabelWrappedType<L>,
{
    type Type = <
        <(S, tags::wire::Variant<V>) as tags::NumericalFieldTypeTag>::NativeType 
            as LabelWrappedType<L>
    >::Type;
}
// For bits32 types
impl<L, S, V> FieldTypeGen<(L, (S, tags::wire::Bits32<V>))> for SimpleImpl
where
    (S, tags::wire::Bits32<V>): tags::NumericalFieldTypeTag,
    <
        (S, tags::wire::Bits32<V>) as tags::NumericalFieldTypeTag
    >::NativeType: LabelWrappedType<L>,
{
    type Type = <
        <(S, tags::wire::Bits32<V>) as tags::NumericalFieldTypeTag>::NativeType 
            as LabelWrappedType<L>
    >::Type;
}
// For bits64 types
impl<L, S, V> FieldTypeGen<(L, (S, tags::wire::Bits64<V>))> for SimpleImpl
where
    (S, tags::wire::Bits64<V>): tags::NumericalFieldTypeTag,
    <
        (S, tags::wire::Bits64<V>) as tags::NumericalFieldTypeTag
    >::NativeType: LabelWrappedType<L>,
{
    type Type = <
        <(S, tags::wire::Bits64<V>) as tags::NumericalFieldTypeTag>::NativeType 
            as LabelWrappedType<L>
    >::Type;
}

// For length delimited types

impl FieldTypeGen<(tags::Required, (tags::Proto2, tags::Bytes))> for SimpleImpl {
    type Type = Option<Cow<'static, [u8]>>;
}
impl FieldTypeGen<(tags::Optional, (tags::Proto2, tags::Bytes))> for SimpleImpl {
    type Type = Option<Cow<'static, [u8]>>;
}
impl FieldTypeGen<(tags::Unlabeled, (tags::Proto3, tags::Bytes))> for SimpleImpl {
    type Type = Vec<u8>;
}
impl FieldTypeGen<(tags::Optional, (tags::Proto3, tags::Bytes))> for SimpleImpl {
    type Type = Option<Vec<u8>>;
}
impl<S> FieldTypeGen<(tags::Repeated, (S, tags::Bytes))> for SimpleImpl {
    type Type = Vec<Vec<u8>>;
}

impl FieldTypeGen<(tags::Required, (tags::Proto2, tags::String))> for SimpleImpl {
    type Type = Option<Cow<'static, str>>;
}
impl FieldTypeGen<(tags::Optional, (tags::Proto2, tags::String))> for SimpleImpl {
    type Type = Option<Cow<'static, str>>;
}
impl FieldTypeGen<(tags::Unlabeled, (tags::Proto3, tags::String))> for SimpleImpl {
    type Type = String;
}
impl FieldTypeGen<(tags::Optional, (tags::Proto3, tags::String))> for SimpleImpl {
    type Type = Option<String>;
}
impl<S> FieldTypeGen<(tags::Repeated, (S, tags::String))> for SimpleImpl {
    type Type = Vec<String>;
}

impl<S, M> FieldTypeGen<(tags::Required, (S, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;
}
impl<S, M> FieldTypeGen<(tags::Optional, (S, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;
}
impl<S, M> FieldTypeGen<(tags::Unlabeled, (S, tags::Message<M>))> for SimpleImpl {
    type Type = Option<Box<M>>;
}
impl<S, M> FieldTypeGen<(tags::Repeated, (S, tags::Message<M>))> for SimpleImpl {
    type Type = Vec<M>;
}
