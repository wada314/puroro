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
use crate::internal::bool::{False, True};
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImpl};
use crate::internal::{EmptyFields, FieldProperties, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef};

trait MethodImpl<'a, IsLd, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetFieldMethodImpl<'a, tags::EitherImpl, True, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    Self: MethodImpl<'a, TypeTag::IsLd, TypeTag::IsMessage, NUMBER, ReturnType = ReturnType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke_get_impl(&'a self) -> Self::ReturnType {
        MethodImpl::invoke(self)
    }
}

// repeated string | bytes field
// Assuming the internal type's getter types are `IntoIterator`
impl<
    'a,
    MP,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    const NUMBER: i32,
> MethodImpl<'a, True, False, NUMBER>
    for MessageImpl<
        MP,
        tags::EitherImpl,
        EmptyFields,
        EitherShared<LeftMessageRef, RightMessageRef>,
    >
where
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
                .map_left(|left| GetFieldMethod::<NUMBER>::invoke_get(left.as_message_impl_ref()))
                .map_right(|right| GetFieldMethod::<NUMBER>::invoke_get(right.as_message_impl_ref())),
        )
    }
}
