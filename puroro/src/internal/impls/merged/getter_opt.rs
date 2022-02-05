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
use crate::internal::methods::{GetFieldMethod, GetOptFieldMethod, GetOptFieldMethodImpl};
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
    GetterType,
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
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<GetterType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<GetterType>>,
{
    type GetterType = Option<GetterType>;
    fn get_opt(&'a self) -> Self::GetterType {
        let (left, right) = (&self.shared.left, &self.shared.right);
        let right_opt = <RightMessage as GetOptFieldMethod<NUMBER>>::get_opt(
            <RightMessageRef as AsMessageImplRef>::as_message_impl_ref(&right),
        );
        right_opt.or_else(|| {
            <LeftMessage as GetOptFieldMethod<NUMBER>>::get_opt(
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
    LeftGetterType,
    RightGetterType,
    FinalGetterType,
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
    LeftMessage: 'a + GetFieldMethod<'a, NUMBER, GetterType = LeftGetterType>,
    RightMessage: 'a + GetFieldMethod<'a, NUMBER, GetterType = RightGetterType>,
    (LeftGetterType, RightGetterType): IntoMergedMessage<MergedMessage = FinalGetterType>,
{
    type GetterType = Option<FinalGetterType>;
    fn get_opt(&'a self) -> Self::GetterType {
        let (left, right) = (&self.shared.left, &self.shared.right);
        let right_opt = <RightMessage as GetFieldMethod<NUMBER>>::get(
            <RightMessageRef as AsMessageImplRef>::as_message_impl_ref(&right),
        );
        let left_opt = <LeftMessage as GetFieldMethod<NUMBER>>::get(
            <LeftMessageRef as AsMessageImplRef>::as_message_impl_ref(&left),
        );
        // Maybe need to return None if the both fields are None?
        Some(IntoMergedMessage::into_message((left_opt, right_opt)))
    }
}
