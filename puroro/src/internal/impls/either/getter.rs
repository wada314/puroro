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
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImpl};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef};

// repeated string | bytes field
// Assuming the internal type's getter type is `&[T]` or whatever `IntoIter<Item: AsRef<str>>` type
impl<
    'a,
    MP,
    _1,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftGetterType,
    RightGetterType,
    const NUMBER: i32,
>
    GetFieldMethodImpl<
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
    LeftMessage: 'a + GetFieldMethod<'a, NUMBER, GetterType = LeftGetterType>,
    RightMessage: 'a + GetFieldMethod<'a, NUMBER, GetterType = RightGetterType>,
    LeftGetterType: Default,
    RightGetterType: Default,
{
    type GetterType = LeftGetterType;
    fn get(&'a self) -> Self::GetterType {
        todo!()
    }
}
