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

use super::impls::option::MessageInOptionTrait;

pub trait GetFieldMethod<'a, const NUMBER: i32> {
    type GetterType;
    fn get(&'a self) -> Self::GetterType;
}

pub trait GetFieldMethodImpl<
    'a,
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type GetterType;
    fn get(&'a self) -> Self::GetterType;
}

impl<'a, MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetFieldMethodImpl<
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
    type GetterType = <Self as GetFieldMethodImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >>::GetterType;
    fn get(&'a self) -> Self::GetterType {
        <Self as GetFieldMethodImpl<
            'a,
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            NUMBER,
        >>::get(self)
    }
}

pub trait GetOptFieldMethod<'a, const NUMBER: i32> {
    type GetterType;
    fn get_opt(&'a self) -> Self::GetterType;
}

pub trait GetOptFieldMethodImpl<
    'a,
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type GetterType;
    fn get_opt(&'a self) -> Self::GetterType;
}

impl<'a, MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetOptFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetOptFieldMethodImpl<
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
    type GetterType = <Self as GetOptFieldMethodImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >>::GetterType;
    fn get_opt(&'a self) -> Self::GetterType {
        <Self as GetOptFieldMethodImpl<
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            NUMBER,
        >>::get_opt(self)
    }
}

pub trait GetMutFieldMethod<'a, const NUMBER: i32> {
    type GetterType;
    fn get_mut(&'a mut self) -> Self::GetterType;
}

pub trait GetMutFieldMethodImpl<
    'a,
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type GetterType;
    fn get_mut(&'a mut self) -> Self::GetterType;
}

impl<'a, MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetMutFieldMethod<'a, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetMutFieldMethodImpl<
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
    type GetterType = <Self as GetMutFieldMethodImpl<
        'a,
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    >>::GetterType;
    fn get_mut(&'a mut self) -> Self::GetterType {
        <Self as GetMutFieldMethodImpl<
            'a,
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<NUMBER>>::Type,
            SharedType,
            NUMBER,
        >>::get_mut(self)
    }
}

//################ Blanket impls ################

// (optional|required|[unlabeled]) non-ld field
// Call get_opt method, and returns a default value if it's `None`.
impl<'a, MP, ImplTag, FieldsType, SharedType, GetterType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
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
    <tags::NonLdType<_2> as tags::FieldTypeTag>::DefaultValueType: Clone + Into<GetterType>,
    Self: GetOptFieldMethod<'a, NUMBER, GetterType = Option<GetterType>>,
{
    type GetterType = GetterType;
    fn get(&'a self) -> Self::GetterType {
        let value_opt = <Self as GetOptFieldMethod<'a, NUMBER>>::get_opt(&self);
        value_opt.unwrap_or(Into::<GetterType>::into(Clone::clone(
            &<<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )))
    }
}

// (optional|required|[unlabeled]) (string|bytes) field
// Call get_opt method, and returns a default value if it's `None`.
impl<'a, MP, ImplTag, FieldsType, SharedType, BorrowedType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
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
    Self: GetOptFieldMethod<'a, NUMBER, GetterType = Option<&'a BorrowedType>>,
{
    type GetterType = &'a BorrowedType;
    fn get(&'a self) -> Self::GetterType {
        let value_opt = <Self as GetOptFieldMethod<'a, NUMBER>>::get_opt(self);
        value_opt.unwrap_or(
            <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )
    }
}

// (optional|required|[unlabeled]) message field
impl<'a, MP, ImplTag, FieldMP, FieldMessageType, FieldsType, SharedType, _1, const NUMBER: i32>
    GetFieldMethodImpl<
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
    FieldMessageType: 'a,
    Self: GetOptFieldMethod<'a, NUMBER, GetterType = Option<&'a FieldMessageType>>,
    Option<&'a FieldMessageType>: MessageInOptionTrait<FieldMP>,
{
    type GetterType =
        <Option<&'a FieldMessageType> as MessageInOptionTrait<FieldMP>>::WrappedOptionMessage;
    fn get(&'a self) -> Self::GetterType {
        <Option<&FieldMessageType> as MessageInOptionTrait<FieldMP>>::into_message(
            <Self as GetOptFieldMethod<'a, NUMBER>>::get_opt(self),
        )
    }
}
