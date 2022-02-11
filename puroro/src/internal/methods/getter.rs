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

use crate::internal::impls::option::IntoOptionMessage;
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImplImpl, GetOptFieldMethod};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef};

trait MethodImpl<'a, LabelTag, TypeTag, FieldType, SharedType, IsLd, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
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
