use std::borrow::Cow;

use super::{
    LabelWrappedLdType, LabelWrappedMessageType, LabelWrappedType, RepeatedFieldImplForLdTypes,
    RepeatedFieldImplForNonLdTypes, SimpleImpl,
};
use crate::{
    AnyFieldTypeGen, EnumTypeGen, FieldTypeGen, MessageInternal, MsgTypeGen, StructInternalTypeGen,
    SwitchImpl,
};
use ::puroro::bool::{False, True};
use ::puroro::{tags, Enum};

// All-in-one typegen trait
impl AnyFieldTypeGen for SimpleImpl {}

// For numerical types
#[rustfmt::skip]
impl<L, X, V, _1, _2> FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>> for SimpleImpl
where
    tags::wire::NonLD<V, _1, _2>: tags::NumericalTypeTag,
    for<'a> tags::wire::NonLD<V, _1, _2>: tags::SelfContainedTypeTag<
        ScalarTypeForTrait<'a> = <tags::wire::NonLD<V, _1, _2> as tags::NumericalTypeTag>::NativeType,
    >,
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

    fn get_scalar<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> <tags::wire::NonLD<V, _1, _2> as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>
    where
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>,
    {
        <L as LabelWrappedType>::get_scalar(from)
    }

    fn get_scalar_optional<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<
        <tags::wire::NonLD<V, _1, _2> as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>,
    >
    where
        L: tags::FieldLabelTag<IsOptionalScalar = True>,
    {
        <L as LabelWrappedType>::get_scalar_optional(from)
    }

    type TraitRepeatedFieldType<'this>
    where
        tags::wire::NonLD<V, _1, _2>: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>,
    = RepeatedFieldImplForNonLdTypes<
        'this,
        <tags::wire::NonLD<V, _1, _2> as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>,
    >;
    fn get_repeated<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::wire::NonLD<V, _1, _2>>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this>
    where
        tags::wire::NonLD<V, _1, _2>: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>,
    {
        RepeatedFieldImplForNonLdTypes(<L as LabelWrappedType>::get_repeated(from))
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

    fn get_scalar<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::Bytes>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> <tags::Bytes as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>
    where
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>,
    {
        <(X, L) as LabelWrappedLdType>::get_scalar(from)
    }
    fn get_scalar_optional<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::Bytes>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<<tags::Bytes as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>>
    where
        L: tags::FieldLabelTag<IsOptionalScalar = True>,
    {
        <(X, L) as LabelWrappedLdType>::get_scalar_optional(from)
    }

    type TraitRepeatedFieldType<'this>
    where
        tags::Bytes: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>,
    = RepeatedFieldImplForLdTypes<'this, [u8]>;

    fn get_repeated<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::Bytes>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this>
    where
        tags::Bytes: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>,
    {
        RepeatedFieldImplForLdTypes(<(X, L) as LabelWrappedLdType>::get_repeated(from))
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

    fn get_scalar<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::String>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> <tags::String as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>
    where
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>,
    {
        <(X, L) as LabelWrappedLdType>::get_scalar(from)
    }
    fn get_scalar_optional<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::String>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<<tags::String as tags::SelfContainedTypeTag>::ScalarTypeForTrait<'this>>
    where
        L: tags::FieldLabelTag<IsOptionalScalar = True>,
    {
        <(X, L) as LabelWrappedLdType>::get_scalar_optional(from)
    }

    type TraitRepeatedFieldType<'this>
    where
        tags::Bytes: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>,
    = RepeatedFieldImplForLdTypes<'this, str>;

    fn get_repeated<'this>(
        from: &'this <Self as FieldTypeGen<X, L, tags::String>>::Type,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this>
    where
        tags::Bytes: tags::SelfContainedTypeTag,
        L: tags::FieldLabelTag<IsRepeated = True>,
    {
        RepeatedFieldImplForLdTypes(<(X, L) as LabelWrappedLdType>::get_repeated(from))
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

    fn get_scalar<'this, E: Enum>(
        from: &'this <Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> <X as tags::EnumTypeForSyntax>::NativeType<E>
    where
        L: tags::FieldLabelTag<IsNonOptionalScalar = True>,
    {
        <L as LabelWrappedType>::get_scalar(from)
    }

    fn get_scalar_optional<'this, E: Enum>(
        from: &'this <Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<<X as tags::EnumTypeForSyntax>::NativeType<E>>
    where
        L: tags::FieldLabelTag<IsOptionalScalar = True>,
    {
        <L as LabelWrappedType>::get_scalar_optional(from)
    }

    type TraitRepeatedFieldType<'this, E: Enum>
    where
        X: tags::EnumTypeForSyntax,
        <X as tags::EnumTypeForSyntax>::NativeType<E>: 'this,
        L: tags::FieldLabelTag<IsRepeated = True>,
    = RepeatedFieldImplForNonLdTypes<'this, <X as tags::EnumTypeForSyntax>::NativeType<E>>;

    fn get_repeated<'this, E: Enum>(
        from: &'this <Self as EnumTypeGen<X, L>>::EnumFieldType<E>,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this, E>
    where
        X: tags::EnumTypeForSyntax,
        <X as tags::EnumTypeForSyntax>::NativeType<E>: 'this,
        L: tags::FieldLabelTag<IsRepeated = True>,
    {
        RepeatedFieldImplForNonLdTypes(<L as LabelWrappedType>::get_repeated(from))
    }
}

impl<X, L> MsgTypeGen<X, L> for SimpleImpl
where
    Self: StructInternalTypeGen,
    L: LabelWrappedMessageType,
{
    type MsgFieldType<M: MessageInternal<ImplTypeTag = Self> + SwitchImpl> =
        <L as LabelWrappedMessageType>::Type<<M as SwitchImpl>::Type<SimpleImpl>>;

    fn default<M: MessageInternal<ImplTypeTag = Self> + SwitchImpl>(
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgFieldType<M> {
        <L as LabelWrappedMessageType>::default()
    }

    fn clone<M: MessageInternal<ImplTypeTag = Self> + SwitchImpl>(
        from: &<Self as MsgTypeGen<X, L>>::MsgFieldType<M>,
        _internal_data: &<Self as StructInternalTypeGen>::Type,
    ) -> <Self as MsgTypeGen<X, L>>::MsgFieldType<M> {
        Clone::clone(from)
    }

    type ImplTagForChildMessage<'this> = SimpleImpl;
    fn get_scalar_optional<'this, M: 'this + MessageInternal<ImplTypeTag = Self> + SwitchImpl>(
        from: &'this <Self as MsgTypeGen<X, L>>::MsgFieldType<M>,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Option<Cow<'this, <M as SwitchImpl>::Type<Self::ImplTagForChildMessage<'this>>>>
    where
        L: tags::FieldLabelTag<IsRepeated = False>,
    {
        <L as LabelWrappedMessageType>::get_scalar_optional(from)
    }

    type TraitRepeatedFieldType<'this, M>
    where
        M: MessageInternal<ImplTypeTag = Self> + SwitchImpl,
        <M as SwitchImpl>::Type<Self::ImplTagForChildMessage<'this>>: 'this,
    = RepeatedFieldImplForLdTypes<'this, <M as SwitchImpl>::Type<SimpleImpl>>;

    /// Get repeated field for the trait getter method.
    fn get_repeated<'this, M: MessageInternal<ImplTypeTag = Self> + SwitchImpl>(
        from: &'this <Self as MsgTypeGen<X, L>>::MsgFieldType<M>,
        _internal_data: &'this <Self as StructInternalTypeGen>::Type,
    ) -> Self::TraitRepeatedFieldType<'this, M>
    where
        M: MessageInternal<ImplTypeTag = Self> + SwitchImpl,
        L: tags::FieldLabelTag<IsRepeated = True>,
    {
        RepeatedFieldImplForLdTypes(<L as LabelWrappedMessageType>::get_repeated(from))
    }
}
