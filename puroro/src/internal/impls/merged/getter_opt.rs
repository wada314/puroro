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
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::{AsMessageImplRef, MessageImpl};

trait MethodImpl<'a, LabelTag, TypeTag, FieldType, SharedType, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, LabelTag, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetOptFieldMethodImpl<'a, tags::MergedImpl, NUMBER>
    for MessageImpl<MP, tags::MergedImpl, FieldsType, SharedType>
where
    Self: MethodImpl<
        'a,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        <TypeTag as tags::FieldTypeTag>::IsMessage,
        NUMBER,
        ReturnType = ReturnType,
    >,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<NUMBER>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        MethodImpl::invoke(self)
    }
}

// non-repeated non-message field
// Assuming that the both message types returns the same type.
impl<
    'a,
    LabelTag,
    TypeTag,
    MP,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    ReturnType,
    const NUMBER: i32,
>
    MethodImpl<
        'a,
        LabelTag,
        TypeTag,
        <EmptyFields as HasField<NUMBER>>::Type,
        MergedShared<LeftMessageRef, RightMessageRef>,
        False,
        NUMBER,
    >
    for MessageImpl<
        MP,
        tags::MergedImpl,
        EmptyFields,
        MergedShared<LeftMessageRef, RightMessageRef>,
    >
where
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
{
    type ReturnType = Option<ReturnType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let (left, right) = (&self.shared.left, &self.shared.right);
        let right_opt = GetOptFieldMethod::<NUMBER>::invoke(right.as_message_impl_ref());
        right_opt.or_else(|| GetOptFieldMethod::<NUMBER>::invoke(left.as_message_impl_ref()))
    }
}

// non-repeated message field
impl<
    'a,
    LabelTag,
    MP,
    InnerMP,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    FinalReturnType,
    const NUMBER: i32,
>
    MethodImpl<
        'a,
        LabelTag,
        tags::Message<InnerMP>,
        <EmptyFields as HasField<NUMBER>>::Type,
        MergedShared<LeftMessageRef, RightMessageRef>,
        True,
        NUMBER,
    >
    for MessageImpl<
        MP,
        tags::MergedImpl,
        EmptyFields,
        MergedShared<LeftMessageRef, RightMessageRef>,
    >
where
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = tags::Message<InnerMP>>,
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

        let has_left = HasFieldMethod::<NUMBER>::has(left_message_impl_ref);
        let has_right = HasFieldMethod::<NUMBER>::has(right_message_impl_ref);
        if has_left || has_right {
            let left_field = GetFieldMethod::<NUMBER>::invoke(left_message_impl_ref);
            let right_field = GetFieldMethod::<NUMBER>::invoke(right_message_impl_ref);
            Some(IntoMergedMessage::into_message((left_field, right_field)))
        } else {
            None
        }
    }
}
