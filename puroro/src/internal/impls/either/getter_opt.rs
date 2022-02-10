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
use crate::internal::bool::{False, True};
use crate::internal::methods::{GetOptFieldMethod, GetOptFieldMethodImpl};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::{AsMessageImplRef, Either, MessageImpl};

trait MethodImpl<'a, LabelTag, TypeTag, FieldType, SharedType, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, LabelTag, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetOptFieldMethodImpl<'a, tags::EitherImpl, NUMBER>
    for MessageImpl<MP, tags::EitherImpl, FieldsType, SharedType>
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
        EitherShared<LeftMessageRef, RightMessageRef>,
        False,
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
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = ReturnType>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = ReturnType>,
{
    type ReturnType = ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        self.shared.either.as_ref().either(
            |msg| GetOptFieldMethod::<NUMBER>::invoke(msg.as_message_impl_ref()),
            |msg| GetOptFieldMethod::<NUMBER>::invoke(msg.as_message_impl_ref()),
        )
    }
}

// non-repeated message field
// Return `Either` of the both return types
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
        EitherShared<LeftMessageRef, RightMessageRef>,
        True,
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
        FieldProperties<LabelTag = LabelTag, TypeTag = tags::Message<InnerMP>>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<LeftReturnType>>,
    RightMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<RightReturnType>>,
    Either<LeftReturnType, RightReturnType>:
        IntoEitherMessage<InnerMP, EitherMessage = FinalReturnType>,
{
    type ReturnType = Option<FinalReturnType>;
    fn invoke(&'a self) -> Self::ReturnType {
        self.shared
            .either
            .as_ref()
            .either(
                |msg| {
                    GetOptFieldMethod::<NUMBER>::invoke(msg.as_message_impl_ref())
                        .map(|l| Either::Left(l))
                },
                |msg| {
                    GetOptFieldMethod::<NUMBER>::invoke(msg.as_message_impl_ref())
                        .map(|r| Either::Right(r))
                },
            ) // Option<Either<LeftReturnType, RightReturnType>>
            .map(|e| e.into_message())
    }
}
