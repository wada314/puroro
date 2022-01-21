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

use crate::internal::impls::option::{OptionFields, OptionShared};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::Message;

pub trait GetFieldMethodImpl<
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
    fn get(&'a self) -> Self::GetterType;
}
pub trait GetFieldMethod<'a, const NUMBER: i32> {
    type GetterType;
    fn get(&'a self) -> Self::GetterType;
}
impl<'a, MP, ImplTag, FieldsType, SharedType, const NUMBER: i32> GetFieldMethod<'a, NUMBER>
    for Message<MP, ImplTag, FieldsType, SharedType>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    FieldsType: HasField<NUMBER>,
    Self: GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >,
{
    type GetterType = <Self as GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
        <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    >>::GetterType;
    fn get(&'a self) -> Self::GetterType {
        <Self as GetFieldMethodImpl<
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            ImplTag,
            <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
            <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
            NUMBER,
        >>::get(self)
    }
}

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
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            ImplTag,
            <MP::Fields<NUMBER> as FieldProperties>::LabelTag,
            <MP::Fields<NUMBER> as FieldProperties>::TypeTag,
            NUMBER,
        >>::get_opt(self)
    }
}

//################ Blanket impls ################

type BorrowedType<_1> = <tags::StringOrBytesType<_1> as tags::StringOrBytesTypeTag>::BorrowedType;

// (optional|required|[unlabeled]) non-ld field
impl<'a, MP, ImplTag, FieldsType, SharedType, GetterType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        tags::NonRepeatedLabel<_1>,
        tags::NonLdType<_2>,
        NUMBER,
    > for Message<MP, ImplTag, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::NonLdType<_2>>,
    tags::NonLdType<_2>: tags::FieldTypeTag,
    <tags::NonLdType<_2> as tags::FieldTypeTag>::DefaultValueType: Clone + Into<GetterType>,
    Self: GetOptFieldMethod<'a, NUMBER, GetterType = Option<GetterType>>,
{
    type GetterType = GetterType;
    fn get(&'a self) -> Self::GetterType {
        let value_opt = <Self as GetOptFieldMethod<NUMBER>>::get_opt(self);
        value_opt.unwrap_or(Into::<GetterType>::into(Clone::clone(
            &<<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )))
    }
}

// (optional|required|[unlabeled]) (string|bytes) field
impl<'a, MP, ImplTag, FieldsType, SharedType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        tags::NonRepeatedLabel<_1>,
        tags::StringOrBytesType<_2>,
        NUMBER,
    > for Message<MP, ImplTag, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties<
        LabelTag = tags::NonRepeatedLabel<_1>,
        TypeTag = tags::StringOrBytesType<_2>,
    >,
    tags::StringOrBytesType<_2>: tags::StringOrBytesTypeTag,
    tags::StringOrBytesType<_2>: tags::FieldTypeTag<DefaultValueType = &'static BorrowedType<_2>>,
    BorrowedType<_2>: 'a + 'static,
    Self: GetOptFieldMethod<'a, NUMBER, GetterType = Option<&'a BorrowedType<_2>>>,
{
    type GetterType = &'a BorrowedType<_2>;
    fn get(&'a self) -> Self::GetterType {
        let value_opt = <Self as GetOptFieldMethod<NUMBER>>::get_opt(self);
        value_opt.unwrap_or(
            <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )
    }
}

// (optional|required|[unlabeled]) message field
// returns a `OptionalImpl`-nized message
impl<'a, MP, ImplTag, FieldMP, FieldMessageType, FieldsType, SharedType, _1, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        ImplTag,
        tags::NonRepeatedLabel<_1>,
        tags::Message<FieldMP>,
        NUMBER,
    > for Message<MP, ImplTag, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::Message<FieldMP>>,
    FieldMessageType: 'a,
    Self: GetOptFieldMethod<'a, NUMBER, GetterType = Option<&'a FieldMessageType>>,
{
    type GetterType =
        Message<FieldMP, tags::OptionImpl, OptionFields, OptionShared<&'a FieldMessageType>>;
    fn get(&'a self) -> Self::GetterType {
        let value_opt = <Self as GetOptFieldMethod<NUMBER>>::get_opt(self);
        Into::into(value_opt)
    }
}
