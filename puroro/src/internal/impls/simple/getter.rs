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

use crate::internal::methods::{GetFieldMethodImpl, GetOptFieldMethod};
use crate::internal::{Bitfield, SharedBitfield};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::Message;

type BorrowedType<_1> = <tags::StringOrBytesType<_1> as tags::StringOrBytesTypeTag>::BorrowedType;

// (optional|required|[unlabeled]) non-ld field
impl<'a, MP, FieldsType, SharedType, GetterType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        tags::SimpleImpl,
        tags::NonRepeatedLabel<_1>,
        tags::NonLdType<_2>,
        NUMBER,
    > for Message<MP, tags::SimpleImpl, FieldsType, SharedType>
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

impl<'a, MP, FieldsType, SharedType, _1, _2, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        tags::SimpleImpl,
        tags::NonRepeatedLabel<_1>,
        tags::StringOrBytesType<_2>,
        NUMBER,
    > for Message<MP, tags::SimpleImpl, FieldsType, SharedType>
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

impl<'a, MP, FieldsType, SharedType, TypeTag, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        tags::SimpleImpl,
        tags::Repeated,
        TypeTag,
        NUMBER,
    > for Message<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    <FieldsType as HasField<NUMBER>>::Type: 'a,
{
    type GetterType = &'a <FieldsType as HasField<NUMBER>>::Type;
    fn get(&'a self) -> Self::GetterType {
        <FieldsType as HasField<NUMBER>>::get(&self.fields)
    }
}
