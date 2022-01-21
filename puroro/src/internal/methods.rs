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
use crate::Message;

pub trait GetFieldMethodImpl<FieldType, SharedType, ImplTag, LabelTag, TypeTag, const NUMBER: i32> {
    type GetterType<'a>
    where
        Self: 'a;
    fn get(&self) -> Self::GetterType<'_>;
}
pub trait GetFieldMethod<const NUMBER: i32> {
    type GetterType<'a>
    where
        Self: 'a;
    fn get(&self) -> Self::GetterType<'_>;
}
impl<MP, ImplTag, FieldsType, SharedType, const NUMBER: i32> GetFieldMethod<NUMBER>
    for Message<MP, ImplTag, FieldsType, SharedType>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    FieldsType: HasField<NUMBER>,
    Self: GetFieldMethodImpl<
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >,
{
    type GetterType<'a>
    where
        Self: 'a,
    = <Self as GetFieldMethodImpl<
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >>::GetterType<'a>;
    fn get(&self) -> Self::GetterType<'_> {
        <Self as GetFieldMethodImpl<
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            ImplTag,
            <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
            <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
            NUMBER,
        >>::get(&self)
    }
}

pub trait GetOptFieldMethodImpl<
    FieldType,
    SharedType,
    ImplTag,
    LabelTag,
    TypeTag,
    const NUMBER: i32,
>
{
    type GetterType<'a>
    where
        Self: 'a;
    fn get_opt(&self) -> Self::GetterType<'_>;
}
pub trait GetOptFieldMethod<const NUMBER: i32> {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_opt(&self) -> Self::GetterType<'_>;
}
impl<MP, ImplTag, FieldsType, SharedType, const NUMBER: i32> GetOptFieldMethod<NUMBER>
    for Message<MP, ImplTag, FieldsType, SharedType>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    FieldsType: HasField<NUMBER>,
    Self: GetOptFieldMethodImpl<
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >,
{
    type GetterType<'a>
    where
        Self: 'a,
    = <Self as GetOptFieldMethodImpl<
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >>::GetterType<'a>;
    fn get_opt(&self) -> Self::GetterType<'_> {
        <Self as GetOptFieldMethodImpl<
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            ImplTag,
            <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
            <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
            NUMBER,
        >>::get_opt(&self)
    }
}
