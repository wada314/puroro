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
use crate::internal::methods::{
    GetFieldMethod, GetOptFieldMethod, GetOptFieldMethodImpl, HasFieldMethod,
};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::{AsMessageImplRef, MessageImpl};

// non-repeated non-message field
// Assuming that the both message types returns the same type.
impl<
    'a,
    MP,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    ReturnType,
    _1,
    _2,
    _3,
    _4,
    const NUMBER: i32,
>
    GetOptFieldMethodImpl<
        'a,
        tags::MergedImpl,
        tags::NonRepeatedLabel<_1>,
        tags::NonMessageType<_2, _3, _4>,
        <EmptyFields as HasField<NUMBER>>::Type,
        MergedShared<LeftMessageRef, RightMessageRef>,
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
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties<
        LabelTag = tags::NonRepeatedLabel<_1>,
        TypeTag = tags::NonMessageType<_2, _3, _4>,
    >,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<ReturnType>>,
{
    type ReturnType = Option<ReturnType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let (left, right) = (&self.shared.left, &self.shared.right);
        let right_opt = <RightMessage as GetOptFieldMethod<NUMBER>>::invoke(
            <RightMessageRef as AsMessageImplRef>::as_message_impl_ref(&right),
        );
        right_opt.or_else(|| {
            <LeftMessage as GetOptFieldMethod<NUMBER>>::invoke(
                <LeftMessageRef as AsMessageImplRef>::as_message_impl_ref(&left),
            )
        })
    }
}

// non-repeated message field
impl<
    'a,
    MP,
    InnerMP,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    FinalReturnType,
    _1,
    const NUMBER: i32,
>
    GetOptFieldMethodImpl<
        'a,
        tags::MergedImpl,
        tags::NonRepeatedLabel<_1>,
        tags::Message<InnerMP>,
        <EmptyFields as HasField<NUMBER>>::Type,
        MergedShared<LeftMessageRef, RightMessageRef>,
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
        FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::Message<InnerMP>>,
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
        let left_message_impl_ref =
            <LeftMessageRef as AsMessageImplRef>::as_message_impl_ref(&left);
        let right_message_impl_ref =
            <RightMessageRef as AsMessageImplRef>::as_message_impl_ref(&right);

        let has_left = <LeftMessage as HasFieldMethod<NUMBER>>::has(left_message_impl_ref);
        let has_right = <RightMessage as HasFieldMethod<NUMBER>>::has(right_message_impl_ref);
        if has_left || has_right {
            let left_field = <LeftMessage as GetFieldMethod<NUMBER>>::invoke(left_message_impl_ref);
            let right_field =
                <RightMessage as GetFieldMethod<NUMBER>>::invoke(right_message_impl_ref);
            Some(IntoMergedMessage::into_message((left_field, right_field)))
        } else {
            None
        }
    }
}
