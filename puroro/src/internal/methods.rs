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

mod getter;

use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::MessageImpl;

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
    fn invoke(&'a self) -> bool;
}

pub trait GetMutFieldMethod<'a, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a mut self) -> Self::ReturnType;
}

//################ Methods Impl traits, for specialization ################

pub trait GetFieldMethodImpl<'a, ImplTag, IsRepeated, const NUMBER: i32> {
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

//################ Delegating to methods Impl traits ################

impl<'a, MP, ImplTag, LabelTag, FieldsType, SharedType, const NUMBER: i32>
    GetFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetFieldMethodImpl<'a, ImplTag, LabelTag::IsRepeated, NUMBER>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<LabelTag = LabelTag>,
    LabelTag: tags::FieldLabelTag,
{
    type ReturnType =
        <Self as GetFieldMethodImpl<'a, ImplTag, LabelTag::IsRepeated, NUMBER>>::ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        GetFieldMethodImpl::invoke(self)
    }
}

impl<'a, MP, ImplTag, FieldsType, SharedType, const NUMBER: i32> GetOptFieldMethod<'a, NUMBER>
    for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetOptFieldMethodImpl<'a, ImplTag, NUMBER>,
{
    type ReturnType = <Self as GetOptFieldMethodImpl<'a, ImplTag, NUMBER>>::ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        GetOptFieldMethodImpl::invoke(self)
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

//################ Blanket impls for has() methods ################

impl<'a, MP, ImplTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    HasFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
{
    fn invoke(&'a self) -> bool {
        GetOptFieldMethod::invoke(self).is_some()
    }
}
