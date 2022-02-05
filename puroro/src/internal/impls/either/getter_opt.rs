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

use super::{EitherShared, IntoEitherMessage};
use crate::internal::impls::option::IntoOptionMessage;
use crate::internal::methods::{GetOptFieldMethod, GetOptFieldMethodImpl};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::{AsMessageImplRef, Either, MessageImpl};

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
        tags::EitherImpl,
        tags::NonRepeatedLabel<_1>,
        tags::NonMessageType<_2, _3, _4>,
        <EmptyFields as HasField<NUMBER>>::Type,
        EitherShared<LeftMessageRef, RightMessageRef>,
        NUMBER,
    >
    for MessageImpl<
        MP,
        tags::EitherImpl,
        EmptyFields,
        EitherShared<LeftMessageRef, RightMessageRef>,
    >
where
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties<
        LabelTag = tags::NonRepeatedLabel<_1>,
        TypeTag = tags::NonMessageType<_2, _3, _4>,
    >,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = GetterType>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = GetterType>,
{
    type GetterType = GetterType;
    fn get_opt(&'a self) -> Self::GetterType {
        self.shared.either.as_ref().either(
            |msg| {
                <LeftMessage as GetOptFieldMethod<NUMBER>>::get_opt(
                    <LeftMessageRef as AsMessageImplRef>::as_message_impl_ref(&msg),
                )
            },
            |msg| {
                <RightMessage as GetOptFieldMethod<NUMBER>>::get_opt(
                    <RightMessageRef as AsMessageImplRef>::as_message_impl_ref(&msg),
                )
            },
        )
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
        tags::EitherImpl,
        tags::NonRepeatedLabel<_1>,
        tags::Message<InnerMP>,
        <EmptyFields as HasField<NUMBER>>::Type,
        EitherShared<LeftMessageRef, RightMessageRef>,
        NUMBER,
    >
    for MessageImpl<
        MP,
        tags::EitherImpl,
        EmptyFields,
        EitherShared<LeftMessageRef, RightMessageRef>,
    >
where
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::Message<InnerMP>>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<LeftGetterType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<RightGetterType>>,
    Either<LeftGetterType, RightGetterType>:
        IntoEitherMessage<InnerMP, EitherMessage = FinalGetterType>,
{
    type GetterType = Option<FinalGetterType>;
    fn get_opt(&'a self) -> Self::GetterType {
        self.shared
            .either
            .as_ref()
            .either(
                |msg| {
                    <LeftMessage as GetOptFieldMethod<NUMBER>>::get_opt(
                        <LeftMessageRef as AsMessageImplRef>::as_message_impl_ref(&msg),
                    )
                    .map(|l| Either::Left(l))
                },
                |msg| {
                    <RightMessage as GetOptFieldMethod<NUMBER>>::get_opt(
                        <RightMessageRef as AsMessageImplRef>::as_message_impl_ref(&msg),
                    )
                    .map(|r| Either::Right(r))
                },
            ) // Option<Either<LeftGetterType, RightGetterType>
            .map(|e| e.into_message())
    }
}
