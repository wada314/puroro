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
use crate::MessageImpl;

pub trait GetFieldMethod<const NUMBER: i32> {
    type GetterType<'a>
    where
        Self: 'a;
    fn get(&self) -> Self::GetterType<'_>;
}

pub trait GetFieldMethodImpl<ImplTag, LabelTag, TypeTag, FieldType, SharedType, const NUMBER: i32> {
    type GetterType<'a>
    where
        Self: 'a;
    fn get(&self) -> Self::GetterType<'_>;
}

impl<MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetFieldMethod<NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetFieldMethodImpl<
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<1>>::Type,
        SharedType,
        NUMBER,
    >,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<1>,
{
    type GetterType<'a>
    where
        Self: 'a,
    = <Self as GetFieldMethodImpl<
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<1>>::Type,
        SharedType,
        NUMBER,
    >>::GetterType<'a>;
    fn get(&self) -> Self::GetterType<'_> {
        <Self as GetFieldMethodImpl<
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<1>>::Type,
            SharedType,
            NUMBER,
        >>::get(self)
    }
}
pub trait GetOptFieldMethod<const NUMBER: i32> {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_opt(&self) -> Self::GetterType<'_>;
}

pub trait GetOptFieldMethodImpl<
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type GetterType<'a>
    where
        Self: 'a;
    fn get_opt(&self) -> Self::GetterType<'_>;
}

impl<MP, ImplTag, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    GetOptFieldMethod<NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: GetOptFieldMethodImpl<
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<1>>::Type,
        SharedType,
        NUMBER,
    >,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<1>,
{
    type GetterType<'a>
    where
        Self: 'a,
    = <Self as GetOptFieldMethodImpl<
        ImplTag,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<1>>::Type,
        SharedType,
        NUMBER,
    >>::GetterType<'a>;
    fn get_opt(&self) -> Self::GetterType<'_> {
        <Self as GetOptFieldMethodImpl<
            ImplTag,
            LabelTag,
            TypeTag,
            <FieldsType as HasField<1>>::Type,
            SharedType,
            NUMBER,
        >>::get_opt(self)
    }
}

//################ Blanket impls ################

// (optional|required|[unlabeled]) non-ld field
// Call get_opt method, and returns a default value if it's `None`.
impl<MP, ImplTag, FieldsType, SharedType, GetterType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
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
    Self: 'static,
    for<'a> Self: GetOptFieldMethod<NUMBER, GetterType<'a> = Option<GetterType>>,
{
    type GetterType<'a>
    where
        Self: 'a,
    = GetterType;
    fn get(&self) -> Self::GetterType<'_> {
        let value_opt = <Self as GetOptFieldMethod<NUMBER>>::get_opt(&self);
        value_opt.unwrap_or(Into::<GetterType>::into(Clone::clone(
            &<<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )))
    }
}

// (optional|required|[unlabeled]) (string|bytes) field
// Call get_opt method, and returns a default value if it's `None`.
impl<MP, ImplTag, FieldsType, SharedType, BorrowedType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
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
    Self: 'static,
    for<'a> Self: GetOptFieldMethod<NUMBER, GetterType<'a> = Option<&'a BorrowedType>>,
{
    type GetterType<'a>
    where
        Self: 'a,
    = &'a BorrowedType;
    fn get(&self) -> Self::GetterType<'_> {
        let value_opt = <Self as GetOptFieldMethod<NUMBER>>::get_opt(self);
        value_opt.unwrap_or(
            <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )
    }
}

// (optional|required|[unlabeled]) message field
impl<MP, ImplTag, FieldMP, FieldMessageType, FieldsType, SharedType, _1, const NUMBER: i32>
    GetFieldMethodImpl<
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
    FieldMessageType: 'static,
    Self: 'static,
    for<'a> Self: GetOptFieldMethod<NUMBER, GetterType<'a> = Option<&'a FieldMessageType>>,
{
    type GetterType<'a>
    where
        Self: 'a,
    = <Self as GetOptFieldMethod<NUMBER>>::GetterType<'a>;
    fn get(&self) -> Self::GetterType<'_> {
        let value_opt = <Self as GetOptFieldMethod<NUMBER>>::get_opt(self);
        Into::into(value_opt)
    }
}
