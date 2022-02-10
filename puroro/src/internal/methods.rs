// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::MessageImpl;

use super::impls::option::IntoOptionMessage;

//################ Methods interfaces ################

pub trait GetFieldMethod<'a, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

pub trait GetOptFieldMethod<'a, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

pub trait GetSliceFieldMethod<'a, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

pub trait HasFieldMethod<'a, const NUMBER: i32> {
    fn has(&'a self) -> bool;
}

pub trait GetMutFieldMethod<'a, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a mut self) -> Self::ReturnType;
}

//################ Methods Impl traits, for specialization ################

pub trait GetFieldMethodImpl<'a, ImplTag, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

pub trait GetOptFieldMethodImpl<'a, ImplTag, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

pub trait GetSliceFieldMethodImpl<'a, ImplTag, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

pub trait GetMutFieldMethodImpl<'a, ImplTag, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a mut self) -> Self::ReturnType;
}

pub trait GetFieldMethodImplImpl<
    'a,
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetFieldMethodImplImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<NUMBER>,
{
    type ReturnType = <Self as GetFieldMethodImplImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >>::ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        <Self as GetFieldMethodImplImpl<
            'a,
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            NUMBER,
        >>::invoke(self)
    }
}

impl<'a, MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetOptFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetOptFieldMethodImpl<'a, ImplTag, NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<NUMBER>,
{
    type ReturnType = <Self as GetOptFieldMethodImpl<'a, ImplTag, NUMBER>>::ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        <Self as GetOptFieldMethodImpl<ImplTag, NUMBER>>::invoke(self)
    }
}

pub trait GetSliceFieldMethodImplImpl<
    'a,
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetSliceFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetSliceFieldMethodImplImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<NUMBER>,
{
    type ReturnType = <Self as GetSliceFieldMethodImplImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >>::ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        <Self as GetSliceFieldMethodImplImpl<
            'a,
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            NUMBER,
        >>::invoke(self)
    }
}

pub trait GetMutFieldMethodImplImpl<
    'a,
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type ReturnType;
    fn invoke(&'a mut self) -> Self::ReturnType;
}

impl<'a, MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetMutFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetMutFieldMethodImplImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<NUMBER>,
{
    type ReturnType = <Self as GetMutFieldMethodImplImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >>::ReturnType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        <Self as GetMutFieldMethodImplImpl<
            'a,
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            NUMBER,
        >>::invoke(self)
    }
}

//################ Blanket impls for get() methods ################

// (optional|required|[unlabeled]) non-ld field
// Call invoke method, and returns a default value if it's `None`.
impl<'a, MP, ImplTag, FieldsType, SharedType, ReturnType, _1, _2, const NUMBER: i32>
    GetFieldMethodImplImpl<
        'a,
        ImplTag,
        tags::NonRepeatedLabel<_1>,
        tags::NonLdType<_2>,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties<TypeTag = tags::NonLdType<_2>>,
    tags::NonLdType<_2>: tags::FieldTypeTag,
    <tags::NonLdType<_2> as tags::FieldTypeTag>::DefaultValueType: Clone + Into<ReturnType>,
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
{
    type ReturnType = ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        let value_opt = <Self as GetOptFieldMethod<'a, NUMBER>>::invoke(&self);
        value_opt.unwrap_or(Into::<ReturnType>::into(Clone::clone(
            &<<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )))
    }
}

// (optional|required|[unlabeled]) (string|bytes) field
// Call invoke method, and returns a default value if it's `None`.
impl<'a, MP, ImplTag, FieldsType, SharedType, BorrowedType, _1, _2, const NUMBER: i32>
    GetFieldMethodImplImpl<
        'a,
        ImplTag,
        tags::NonRepeatedLabel<_1>,
        tags::StringOrBytesType<_2>,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<TypeTag = tags::StringOrBytesType<_2>>,
    tags::StringOrBytesType<_2>: tags::FieldTypeTag<DefaultValueType = &'static BorrowedType>,
    tags::StringOrBytesType<_2>: tags::StringOrBytesTypeTag<BorrowedType = BorrowedType>,
    BorrowedType: 'static + ?Sized,
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = Option<&'a BorrowedType>>,
{
    type ReturnType = &'a BorrowedType;
    fn invoke(&'a self) -> Self::ReturnType {
        let value_opt = <Self as GetOptFieldMethod<'a, NUMBER>>::invoke(self);
        value_opt.unwrap_or(
            <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )
    }
}

// (optional|required|[unlabeled]) message field
impl<'a, MP, ImplTag, FieldMP, ReturnType, FieldsType, SharedType, _1, const NUMBER: i32>
    GetFieldMethodImplImpl<
        'a,
        ImplTag,
        tags::NonRepeatedLabel<_1>,
        tags::Message<FieldMP>,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::Message<FieldMP>>,
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = ReturnType>,
    ReturnType: IntoOptionMessage<FieldMP>,
{
    type ReturnType = <ReturnType as IntoOptionMessage<FieldMP>>::OptionMessage;
    fn invoke(&'a self) -> Self::ReturnType {
        let msg_opt = <Self as GetOptFieldMethod<'a, NUMBER>>::invoke(self);
        <ReturnType as IntoOptionMessage<FieldMP>>::into_message(msg_opt)
    }
}

//################ Blanket impls for has() methods ################

impl<'a, MP, ImplTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    HasFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
{
    fn has(&'a self) -> bool {
        <Self as GetOptFieldMethod<NUMBER>>::invoke(self).is_some()
    }
}
