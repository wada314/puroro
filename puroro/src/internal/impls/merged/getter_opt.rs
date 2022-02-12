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
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, Merged express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{IntoMergedMessage, MergedShared};
use crate::internal::bool::{False, True};
use crate::internal::methods::{
    GetFieldMethod, GetOptFieldMethod, GetOptFieldMethodImpl, HasFieldMethod,
};
use crate::internal::{FieldProperties, MessageProperties};
use crate::tags;
use crate::{AsMessageImplRef, MessageImpl};

trait MethodImpl<'a, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetOptFieldMethodImpl<'a, tags::MergedImpl, NUMBER>
    for MessageImpl<MP, tags::MergedImpl, FieldsType, SharedType>
where
    Self: MethodImpl<'a, TypeTag::IsMessage, NUMBER, ReturnType = ReturnType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke_get_opt_impl(&'a self) -> Self::ReturnType {
        self.invoke()
    }
}

// non-repeated non-message field
// Assuming that the both message types returns the same type.
impl<
    'a,
    MP,
    FieldsType,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    ReturnType,
    const NUMBER: i32,
> MethodImpl<'a, False, NUMBER>
    for MessageImpl<MP, tags::MergedImpl, FieldsType, MergedShared<LeftMessageRef, RightMessageRef>>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
{
    type ReturnType = Option<ReturnType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let (left, right) = (&self.shared.left, &self.shared.right);
        let right_opt = right.as_message_impl_ref().invoke_get_opt();
        right_opt.or_else(|| left.as_message_impl_ref().invoke_get_opt())
    }
}

// non-repeated message field
impl<
    'a,
    MP,
    InnerMP,
    FieldsType,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    FinalReturnType,
    const NUMBER: i32,
> MethodImpl<'a, True, NUMBER>
    for MessageImpl<MP, tags::MergedImpl, FieldsType, MergedShared<LeftMessageRef, RightMessageRef>>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = tags::Message<InnerMP>>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage:
        'a + GetFieldMethod<'a, NUMBER, ReturnType = LeftReturnType> + HasFieldMethod<'a, NUMBER>,
    RightMessage:
        'a + GetFieldMethod<'a, NUMBER, ReturnType = RightReturnType> + HasFieldMethod<'a, NUMBER>,
    (LeftReturnType, RightReturnType): IntoMergedMessage<InnerMP, MergedMessage = FinalReturnType>,
{
    type ReturnType = Option<FinalReturnType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let (left, right) = (&self.shared.left, &self.shared.right);
        let left_message_impl_ref = left.as_message_impl_ref();
        let right_message_impl_ref = right.as_message_impl_ref();

        let has_left = left_message_impl_ref.invoke_has();
        let has_right = right_message_impl_ref.invoke_has();
        if has_left || has_right {
            let left_field = left_message_impl_ref.invoke_get();
            let right_field = right_message_impl_ref.invoke_get();
            Some((left_field, right_field).into_message())
        } else {
            None
        }
    }
}
