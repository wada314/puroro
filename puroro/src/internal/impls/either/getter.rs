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

use super::{EitherRepeatedField, EitherShared};
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImplImpl};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef};

// repeated string | bytes field
// Assuming the internal type's getter types are `IntoIterator`
impl<
    'a,
    MP,
    _1,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    const NUMBER: i32,
>
    GetFieldMethodImplImpl<
        'a,
        tags::EitherImpl,
        tags::Repeated,
        tags::StringOrBytesType<_1>,
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
        FieldProperties<LabelTag = tags::Repeated, TypeTag = tags::StringOrBytesType<_1>>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = LeftReturnType>,
    RightMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = RightReturnType>,
    LeftReturnType: IntoIterator,
    RightReturnType: IntoIterator,
{
    type ReturnType = EitherRepeatedField<LeftReturnType, RightReturnType>;
    fn invoke(&'a self) -> Self::ReturnType {
        EitherRepeatedField(
            self.shared
                .either
                .as_ref()
                .map_left(|left| {
                    <LeftMessage as GetFieldMethod<NUMBER>>::invoke(left.as_message_impl_ref())
                })
                .map_right(|right| {
                    <RightMessage as GetFieldMethod<NUMBER>>::invoke(right.as_message_impl_ref())
                }),
        )
    }
}
