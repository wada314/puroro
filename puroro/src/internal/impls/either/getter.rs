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

use super::{EitherIter, EitherShared, IntoEitherMessage};
use crate::internal::bool::{False, True};
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImpl};
use crate::internal::{FieldProperties, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef, Either};

trait MethodImpl<'a, IsLd, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetFieldMethodImpl<'a, tags::EitherImpl, True, NUMBER>
    for MessageImpl<MP, tags::EitherImpl, FieldsType, SharedType>
where
    Self: MethodImpl<'a, TypeTag::IsLd, TypeTag::IsMessage, NUMBER, ReturnType = ReturnType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke_get_impl(&'a self) -> Self::ReturnType {
        self.invoke()
    }
}

// repeated non-ld field
// (Implicitly ) assuming the internal type's getter types `T` and `U` are:
// ```
// T: IntoIterator,
// U: IntoIterator<Item = <T as IntoIterator>::Item>,
// ```
// The both `Item` types must to be the same.
// And then returns an `Either<T, U>`.
// As long as the requirements above are met, `Either<T, U>` become an
// `IntoIterator<Item = T::Item>`.
impl<
    'a,
    MP,
    FieldsType,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    const NUMBER: i32,
> MethodImpl<'a, False, False, NUMBER>
    for MessageImpl<MP, tags::EitherImpl, FieldsType, EitherShared<LeftMessageRef, RightMessageRef>>
where
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = LeftReturnType>,
    RightMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = RightReturnType>,
{
    type ReturnType = Either<LeftReturnType, RightReturnType>;
    fn invoke(&'a self) -> Self::ReturnType {
        self.shared
            .either
            .as_ref()
            .map_left(|left| left.as_message_impl_ref().invoke_get())
            .map_right(|right| right.as_message_impl_ref().invoke_get())
    }
}

// repeated string | bytes field
// Assuming the internal type's getter types `T` and `U` are:
// ```
// T: IntoIterator,
// <T as IntoIterator>::Item: AsRef<str>, // or [u8]
// U: IntoIterator,
// <U as IntoIterator>::Item: AsRef<str>, // or [u8]
// ```
// And then returns an `IntoIterator<Item = Either<T::Item, U::Item>>`.
// The both item types no need to be the same.
// `Either` of 2 `AsRef<str>` types is `AsRef<str>` by itself too.
impl<
    'a,
    MP,
    FieldsType,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    const NUMBER: i32,
> MethodImpl<'a, True, False, NUMBER>
    for MessageImpl<MP, tags::EitherImpl, FieldsType, EitherShared<LeftMessageRef, RightMessageRef>>
where
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = LeftReturnType>,
    RightMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = RightReturnType>,
    LeftReturnType: IntoIterator,
    RightReturnType: IntoIterator,
{
    type ReturnType = EitherIter<LeftReturnType::IntoIter, RightReturnType::IntoIter>;
    fn invoke(&'a self) -> Self::ReturnType {
        EitherIter(
            self.shared
                .either
                .as_ref()
                .map_left(|left| left.as_message_impl_ref().invoke_get().into_iter())
                .map_right(|right| right.as_message_impl_ref().invoke_get().into_iter()),
        )
    }
}

// repeated message field
// Assuming the internal type's getter types `T` and `U` are:
// ```
// T: IntoIterator,
// U: IntoIterator,
// Either<T::Item, U::Item>: IntoEitherMessage<MP>,
// ```
// And then returns an `IntoIterator<Item = Either<T::Item, U::Item>::EitherMessage>`.
// The both item types no need to be the same.
impl<
    'a,
    MP,
    InnerMP,
    FieldsType,
    LeftMessageRef,
    RightMessageRef,
    LeftMessage,
    RightMessage,
    LeftReturnType,
    RightReturnType,
    LeftItemType,
    RightItemType,
    const NUMBER: i32,
> MethodImpl<'a, True, True, NUMBER>
    for MessageImpl<MP, tags::EitherImpl, FieldsType, EitherShared<LeftMessageRef, RightMessageRef>>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = tags::Message<InnerMP>>,
    LeftMessageRef: AsMessageImplRef<MessageImplType = LeftMessage>,
    RightMessageRef: AsMessageImplRef<MessageImplType = RightMessage>,
    LeftMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = LeftReturnType>,
    RightMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = RightReturnType>,
    LeftReturnType: IntoIterator<Item = LeftItemType>,
    RightReturnType: IntoIterator<Item = RightItemType>,
    Either<LeftItemType, RightItemType>: IntoEitherMessage<InnerMP>,
{
    type ReturnType = impl IntoIterator<
        Item = <Either<LeftItemType, RightItemType> as IntoEitherMessage<InnerMP>>::EitherMessage,
    >;
    fn invoke(&'a self) -> Self::ReturnType {
        EitherIter(
            self.shared
                .either
                .as_ref()
                .map_left(|left| left.as_message_impl_ref().invoke_get().into_iter())
                .map_right(|right| right.as_message_impl_ref().invoke_get().into_iter()),
        )
        .map(|either| either.into_message())
    }
}
