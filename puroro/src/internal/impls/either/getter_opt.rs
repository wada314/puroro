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

use super::EitherShared;
use crate::internal::methods::{GetOptFieldMethod, GetOptFieldMethodImpl};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::{AsMessageImplRef, Either, MessageImpl};

// non-repeated non-ld field
// Assuming that the both message types returns the same type.
impl<
    'a,
    MP,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    NumType,
    _1,
    _2,
    const NUMBER: i32,
>
    GetOptFieldMethodImpl<
        'a,
        tags::EitherImpl,
        tags::NonRepeatedLabel<_1>,
        tags::NonLdType<_2>,
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
        FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::NonLdType<_2>>,
    tags::NonLdType<_2>: tags::NumericalTypeTag<NativeType = NumType>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<NumType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<NumType>>,
{
    type GetterType = Option<NumType>;
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

// non-repeated string|bytes field
// Assuming that the both message types returns `AsRef<str>` or `AsRef<[u8]>` type.
impl<
    'a,
    MP,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftGetterType,
    RightGetterType,
    _1,
    _2,
    const NUMBER: i32,
>
    GetOptFieldMethodImpl<
        'a,
        tags::EitherImpl,
        tags::NonRepeatedLabel<_1>,
        tags::StringOrBytesType<_2>,
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
        TypeTag = tags::StringOrBytesType<_2>,
    >,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<LeftGetterType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<RightGetterType>>,
{
    type GetterType = Option<Either<LeftGetterType, RightGetterType>>;
    fn get_opt(&'a self) -> Self::GetterType {
        self.shared
            .either
            .as_ref()
            .map_left(|msg| {
                <LeftMessage as GetOptFieldMethod<NUMBER>>::get_opt(
                    <LeftMessageRef as AsMessageImplRef>::as_message_impl_ref(&msg),
                )
            })
            .map_right(|msg| {
                <RightMessage as GetOptFieldMethod<NUMBER>>::get_opt(
                    <RightMessageRef as AsMessageImplRef>::as_message_impl_ref(&msg),
                )
            })
            .either(
                |l_opt| l_opt.map(|l| Either::Left(l)),
                |r_opt| r_opt.map(|r| Either::Right(r)),
            )
    }
}
