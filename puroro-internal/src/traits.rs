use crate::{TraitEnumTypeGen, TraitFieldTypeGen, TraitMsgTypeGen};
use ::puroro::tags;
use ::std::borrow::Cow;

impl<T, X, L, V, _1, _2> TraitFieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>> for T
where
    tags::wire::NonLD<V, _1, _2>: tags::NumericalTypeTag,
{
    type ScalarGetterReturnType<'this> =
        <tags::wire::NonLD<V, _1, _2> as tags::NumericalTypeTag>::NativeType;
}

impl<T, X, L> TraitFieldTypeGen<X, L, tags::Bytes> for T {
    type ScalarGetterReturnType<'this> = Cow<'this, [u8]>;
}

impl<T, X, L> TraitFieldTypeGen<X, L, tags::String> for T {
    type ScalarGetterReturnType<'this> = Cow<'this, str>;
}

impl<T, X, L> TraitEnumTypeGen<X, L> for T
where
    X: tags::EnumTypeForSyntax,
{
    type ScalarGetterReturnType<'this, E: puroro::Enum> =
        <X as tags::EnumTypeForSyntax>::NativeType<E>;
}

impl<T, X, L> TraitMsgTypeGen<X, L> for T {
    type ScalarGetterReturnType<'this, M: 'this + puroro::Message> = Cow<'this, M>;
}
