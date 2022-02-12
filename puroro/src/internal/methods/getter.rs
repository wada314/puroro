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

//! Blanket impls for scalar `get()` methods.
//! It just redirects to `get_opt()` method and unwrap or return a default value.

use crate::internal::bool::{False, True};
use crate::internal::impls::option::IntoOptionMessage;
use crate::internal::methods::{GetFieldMethodImpl, GetOptFieldMethod};
use crate::internal::{FieldProperties, MessageProperties};
use crate::tags;
use crate::MessageImpl;

trait MethodImpl<'a, IsLd, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, ImplTag, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetFieldMethodImpl<'a, ImplTag, False, NUMBER>
    for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    Self: MethodImpl<'a, TypeTag::IsLd, TypeTag::IsMessage, NUMBER, ReturnType = ReturnType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        MethodImpl::invoke(self)
    }
}

// (optional|required|[unlabeled]) non-ld field
// Call `get_opt()` method, and returns a default value if it's `None`.
impl<'a, MP, ImplTag, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    MethodImpl<'a, False, False, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
    TypeTag::DefaultValueType: Clone + Into<ReturnType>,
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
{
    type ReturnType = ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        let value_opt = GetOptFieldMethod::<NUMBER>::invoke(self);
        value_opt.unwrap_or(Into::<ReturnType>::into(Clone::clone(
            &MP::Fields::<NUMBER>::DEFAULT_VALUE,
        )))
    }
}

// (optional|required|[unlabeled]) (string|bytes) field
// Call invoke method, and returns a default value if it's `None`.
impl<'a, MP, ImplTag, TypeTag, FieldsType, SharedType, BorrowedType, const NUMBER: i32>
    MethodImpl<'a, True, False, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag<DefaultValueType = &'static BorrowedType>,
    TypeTag: tags::StringOrBytesTypeTag<BorrowedType = BorrowedType>,
    BorrowedType: 'static + ?Sized,
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = Option<&'a BorrowedType>>,
{
    type ReturnType = &'a BorrowedType;
    fn invoke(&'a self) -> Self::ReturnType {
        let value_opt = GetOptFieldMethod::<NUMBER>::invoke(self);
        value_opt.unwrap_or(MP::Fields::<NUMBER>::DEFAULT_VALUE)
    }
}

// (optional|required|[unlabeled]) message field
impl<'a, MP, ImplTag, FieldMP, ReturnType, FieldsType, SharedType, const NUMBER: i32>
    MethodImpl<'a, True, True, NUMBER> for MessageImpl<MP, ImplTag, FieldsType, SharedType>
where
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties<TypeTag = tags::Message<FieldMP>>,
    Self: GetOptFieldMethod<'a, NUMBER, ReturnType = ReturnType>,
    ReturnType: IntoOptionMessage<FieldMP>,
{
    type ReturnType = ReturnType::OptionMessage;
    fn invoke(&'a self) -> Self::ReturnType {
        let msg_opt = GetOptFieldMethod::<NUMBER>::invoke(self);
        IntoOptionMessage::into_message(msg_opt)
    }
}
