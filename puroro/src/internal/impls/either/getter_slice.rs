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
use crate::internal::methods::{GetSliceFieldMethod, GetSliceFieldMethodImpl};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef};

// repeated field
// Assuming the both internal type's getter types are same `&[T]` type
impl<
    'a,
    MP,
    FieldsType,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    SliceType,
    const NUMBER: i32,
> GetSliceFieldMethodImpl<'a, tags::EitherImpl, NUMBER>
    for MessageImpl<MP, tags::EitherImpl, FieldsType, EitherShared<LeftMessageRef, RightMessageRef>>
where
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetSliceFieldMethod<'a, NUMBER, ReturnType = SliceType>,
    RightMessage: 'a + GetSliceFieldMethod<'a, NUMBER, ReturnType = SliceType>,
{
    type ReturnType = SliceType;
    fn invoke(&'a self) -> Self::ReturnType {
        self.shared.either.as_ref().either(
            |left| GetSliceFieldMethod::<NUMBER>::invoke(left.as_message_impl_ref()),
            |right| GetSliceFieldMethod::<NUMBER>::invoke(right.as_message_impl_ref()),
        )
    }
}
