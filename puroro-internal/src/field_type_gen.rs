use crate::variant;
use crate::{GetBumpaloStructImplFor, GetSimpleStructImplFor, GetSliceViewStructImplFor};
use ::puroro::tags;

// MAPS???

pub trait FieldTypeGen<WireAndValueTag> {
    type Type;
}

// SimpleStruct, all labels, variant types
impl<L, V> FieldTypeGen<(L, (tags::wire::Variant, V))> for tags::SimpleStruct
where
    V: variant::VariantTypeTag,
    <V as variant::VariantTypeTag>::NativeType: TypicalWrapperTypeFor<L, tags::SimpleStruct>,
{
    type Type = <<V as variant::VariantTypeTag>::NativeType as TypicalWrapperTypeFor<
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

impl<'slice, M> FieldTypeGen<(tags::Required, tags::Message<M>)> for tags::SliceView<'slice>
where
    M: GetSliceViewStructImplFor<'slice>,
    <M as GetSliceViewStructImplFor<'slice>>::Type: ::puroro::Message,
{
    type Type = crate::SliceViewField<'slice>;
}
impl<'slice, M> FieldTypeGen<(tags::Optional2, tags::Message<M>)> for tags::SliceView<'slice>
where
    M: GetSliceViewStructImplFor<'slice>,
    <M as GetSliceViewStructImplFor<'slice>>::Type: ::puroro::Message,
{
    type Type = crate::SliceViewField<'slice>;
}
impl<'slice, M> FieldTypeGen<(tags::Optional3, tags::Message<M>)> for tags::SliceView<'slice>
where
    M: GetSliceViewStructImplFor<'slice>,
    <M as GetSliceViewStructImplFor<'slice>>::Type: ::puroro::Message,
{
    type Type = crate::SliceViewField<'slice>;
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
