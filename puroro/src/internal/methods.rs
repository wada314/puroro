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

pub trait GetOptFieldMethodImpl<
    'a,
    FieldType,
    SharedType,
    ImplTag,
    LabelTag,
    TypeTag,
    const NUMBER: i32,
>
{
    type GetterType;
    fn get_opt(&'a self) -> Self::GetterType;
}
pub trait GetOptFieldMethod<'a, const NUMBER: i32> {
    type GetterType;
    fn get_opt(&'a self) -> Self::GetterType;
}
impl<'a, MP, ImplTag, FieldsType, SharedType, const NUMBER: i32> GetOptFieldMethod<'a, NUMBER>
    for Message<MP, ImplTag, FieldsType, SharedType>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    FieldsType: HasField<NUMBER>,
    Self: GetOptFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >,
{
    type GetterType = <Self as GetOptFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >>::GetterType;
    fn get_opt(&'a self) -> Self::GetterType {
        <Self as GetOptFieldMethodImpl<
            'a,
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            ImplTag,
            <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
            <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
            NUMBER,
        >>::get_opt(&self)
    }
}
