use crate::{GetBumpaloStructImplFor, GetSimpleStructImplFor};
use ::puroro::{bumpalo, tags};

// MAPS???

pub trait FieldTypeGen<WireAndValueTag> {
    type Type;
}

// SimpleStruct, all labels, variant types
impl<L, V> FieldTypeGen<(L, (tags::wire::Variant, V))> for tags::SimpleStruct
where
    V: tags::VariantTypeTag,
    <V as tags::VariantTypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::SimpleStruct>,
{
    type Type = <<V as tags::VariantTypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::SimpleStruct,
    >>::Type;
}

// SimpleStruct, all labels, String & Bytes
impl<L> FieldTypeGen<(L, tags::String)> for tags::SimpleStruct
where
    String: TypicalWrapperTypeFor<L, tags::SimpleStruct>,
{
    type Type = <String as TypicalWrapperTypeFor<L, tags::SimpleStruct>>::Type;
}
impl<L> FieldTypeGen<(L, tags::Bytes)> for tags::SimpleStruct
where
    Vec<u8>: TypicalWrapperTypeFor<L, tags::SimpleStruct>,
{
    type Type = <Vec<u8> as TypicalWrapperTypeFor<L, tags::SimpleStruct>>::Type;
}

// SimpleStruct, all labels, fixed32 / fixed64 types
impl<L, V> FieldTypeGen<(L, (tags::wire::Bits32, V))> for tags::SimpleStruct
where
    V: tags::Bits32TypeTag,
    <V as tags::Bits32TypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::SimpleStruct>,
{
    type Type = <<V as tags::Bits32TypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::SimpleStruct,
    >>::Type;
}
impl<L, V> FieldTypeGen<(L, (tags::wire::Bits64, V))> for tags::SimpleStruct
where
    V: tags::Bits64TypeTag,
    <V as tags::Bits64TypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::SimpleStruct>,
{
    type Type = <<V as tags::Bits64TypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::SimpleStruct,
    >>::Type;
}

// SimpleStruct, each labels, message types
impl<M> FieldTypeGen<(tags::Required, tags::Message<M>)> for tags::SimpleStruct
where
    M: GetSimpleStructImplFor,
    <M as GetSimpleStructImplFor>::Type: ::puroro::Message,
{
    type Type = <M as GetSimpleStructImplFor>::Type;
}
impl<M> FieldTypeGen<(tags::Optional2, tags::Message<M>)> for tags::SimpleStruct
where
    M: GetSimpleStructImplFor,
    <M as GetSimpleStructImplFor>::Type: ::puroro::Message,
{
    type Type = Option<<<M as GetSimpleStructImplFor>::Type as ::puroro::Message>::BoxedType>;
}
impl<M> FieldTypeGen<(tags::Optional3, tags::Message<M>)> for tags::SimpleStruct
where
    M: GetSimpleStructImplFor,
    <M as GetSimpleStructImplFor>::Type: ::puroro::Message,
{
    type Type = Option<<<M as GetSimpleStructImplFor>::Type as ::puroro::Message>::BoxedType>;
}
impl<M> FieldTypeGen<(tags::Repeated, tags::Message<M>)> for tags::SimpleStruct
where
    M: GetSimpleStructImplFor,
    <M as GetSimpleStructImplFor>::Type: ::puroro::Message,
{
    type Type = Vec<<M as GetSimpleStructImplFor>::Type>;
}

// Bumpalo, all labels, variant types
impl<'bump, L, V> FieldTypeGen<(L, (tags::wire::Variant, V))> for tags::Bumpalo<'bump>
where
    V: tags::VariantTypeTag,
    <V as tags::VariantTypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::Bumpalo<'bump>>,
{
    type Type = <<V as tags::VariantTypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::Bumpalo<'bump>,
    >>::Type;
}

// Bumpalo, all labels, String & Bytes types
impl<'bump, L> FieldTypeGen<(L, tags::String)> for tags::Bumpalo<'bump>
where
    bumpalo::collections::String<'bump>: TypicalWrapperTypeFor<L, tags::Bumpalo<'bump>>,
{
    type Type = <bumpalo::collections::String<'bump> as TypicalWrapperTypeFor<
        L,
        tags::Bumpalo<'bump>,
    >>::Type;
}
impl<'bump, L> FieldTypeGen<(L, tags::Bytes)> for tags::Bumpalo<'bump>
where
    bumpalo::collections::String<'bump>: TypicalWrapperTypeFor<L, tags::Bumpalo<'bump>>,
{
    type Type = <bumpalo::collections::String<'bump> as TypicalWrapperTypeFor<
        L,
        tags::Bumpalo<'bump>,
    >>::Type;
}

// Bumpalo, all labels, fixed32 / 64 types
impl<'bump, L, V> FieldTypeGen<(L, (tags::wire::Bits32, V))> for tags::Bumpalo<'bump>
where
    V: tags::Bits32TypeTag,
    <V as tags::Bits32TypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::Bumpalo<'bump>>,
{
    type Type = <<V as tags::Bits32TypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::Bumpalo<'bump>,
    >>::Type;
}
impl<'bump, L, V> FieldTypeGen<(L, (tags::wire::Bits64, V))> for tags::Bumpalo<'bump>
where
    V: tags::Bits64TypeTag,
    <V as tags::Bits64TypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::Bumpalo<'bump>>,
{
    type Type = <<V as tags::Bits64TypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::Bumpalo<'bump>,
    >>::Type;
}

// Bumpalo, each labels, message types
impl<'bump, M> FieldTypeGen<(tags::Required, tags::Message<M>)> for tags::Bumpalo<'bump>
where
    M: GetBumpaloStructImplFor<'bump>,
    <M as GetBumpaloStructImplFor<'bump>>::Type: ::puroro::Message,
{
    type Type = <M as GetBumpaloStructImplFor<'bump>>::Type;
}
impl<'bump, M> FieldTypeGen<(tags::Optional2, tags::Message<M>)> for tags::Bumpalo<'bump>
where
    M: GetBumpaloStructImplFor<'bump>,
    <M as GetBumpaloStructImplFor<'bump>>::Type: ::puroro::Message,
{
    type Type =
        Option<<<M as GetBumpaloStructImplFor<'bump>>::Type as ::puroro::Message>::BoxedType>;
}
impl<'bump, M> FieldTypeGen<(tags::Optional3, tags::Message<M>)> for tags::Bumpalo<'bump>
where
    M: GetBumpaloStructImplFor<'bump>,
    <M as GetBumpaloStructImplFor<'bump>>::Type: ::puroro::Message,
{
    type Type =
        Option<<<M as GetBumpaloStructImplFor<'bump>>::Type as ::puroro::Message>::BoxedType>;
}
impl<'bump, M> FieldTypeGen<(tags::Repeated, tags::Message<M>)> for tags::Bumpalo<'bump>
where
    M: GetBumpaloStructImplFor<'bump>,
    <M as GetBumpaloStructImplFor<'bump>>::Type: 'bump + ::puroro::Message,
{
    type Type = bumpalo::collections::Vec<'bump, <M as GetBumpaloStructImplFor<'bump>>::Type>;
}

// SliceView, all labels, variant types
impl<'slice, L, V> FieldTypeGen<(L, (tags::wire::Variant, V))> for tags::SliceView<'slice>
where
    V: tags::VariantTypeTag,
    <V as tags::VariantTypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::SliceView<'slice>>,
{
    type Type = <<V as tags::VariantTypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::SliceView<'slice>,
    >>::Type;
}

// SliceView, all labels, String / bytes / message types
impl<'slice, L, V> FieldTypeGen<(L, (tags::wire::LengthDelimited, V))> for tags::SliceView<'slice>
where
    V: tags::LengthDelimitedTypeTag,
{
    type Type = crate::SliceViewField<'slice>;
}

// SliceView, all labels, fixed32 / 64 types
impl<'slice, L, V> FieldTypeGen<(L, (tags::wire::Bits32, V))> for tags::SliceView<'slice>
where
    V: tags::Bits32TypeTag,
    <V as tags::Bits32TypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::SliceView<'slice>>,
{
    type Type = <<V as tags::Bits32TypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::SliceView<'slice>,
    >>::Type;
}
impl<'slice, L, V> FieldTypeGen<(L, (tags::wire::Bits64, V))> for tags::SliceView<'slice>
where
    V: tags::Bits64TypeTag,
    <V as tags::Bits64TypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::SliceView<'slice>>,
{
    type Type = <<V as tags::Bits64TypeTag>::NativeType as TypicalWrapperTypeFor<
        L,
        tags::SliceView<'slice>,
    >>::Type;
}

// A helper trait for simplifying the code.

pub trait TypicalWrapperTypeFor<LabelTag, ImplTag> {
    type Type;
}
impl<T> TypicalWrapperTypeFor<tags::Required, tags::SimpleStruct> for T {
    type Type = T;
}
impl<T> TypicalWrapperTypeFor<tags::Optional2, tags::SimpleStruct> for T {
    type Type = Option<T>;
}
impl<T> TypicalWrapperTypeFor<tags::Optional3, tags::SimpleStruct> for T {
    type Type = T;
}
impl<T> TypicalWrapperTypeFor<tags::Repeated, tags::SimpleStruct> for T {
    type Type = Vec<T>;
}
impl<'bump, T> TypicalWrapperTypeFor<tags::Required, tags::Bumpalo<'bump>> for T {
    type Type = T;
}
impl<'bump, T> TypicalWrapperTypeFor<tags::Optional2, tags::Bumpalo<'bump>> for T {
    type Type = Option<T>;
}
impl<'bump, T> TypicalWrapperTypeFor<tags::Optional3, tags::Bumpalo<'bump>> for T {
    type Type = T;
}
impl<'bump, T> TypicalWrapperTypeFor<tags::Repeated, tags::Bumpalo<'bump>> for T
where
    T: 'bump,
{
    type Type = crate::bumpalo::collections::Vec<'bump, T>;
}
impl<'slice, T> TypicalWrapperTypeFor<tags::Required, tags::SliceView<'slice>> for T {
    type Type = T;
}
impl<'slice, T> TypicalWrapperTypeFor<tags::Optional2, tags::SliceView<'slice>> for T {
    type Type = Option<T>;
}
impl<'slice, T> TypicalWrapperTypeFor<tags::Optional3, tags::SliceView<'slice>> for T {
    type Type = T;
}
impl<'slice, T> TypicalWrapperTypeFor<tags::Repeated, tags::SliceView<'slice>> for T {
    type Type = crate::SliceViewField<'slice>;
}
