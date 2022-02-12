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

use super::OptionShared;
use crate::internal::bool::True;
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImpl};
use crate::internal::{EmptyFields, FieldProperties, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef};
use ::std::iter;
use ::std::option;

// repeated field
// Wrap the internal message's iterator by `std::option::IntoIter`, and then flatten it.
impl<'a, MP, InnerMessageRef, InnerMessage, InnerReturnType, const NUMBER: i32>
    GetFieldMethodImpl<'a, tags::OptionImpl, True, NUMBER>
    for MessageImpl<MP, tags::OptionImpl, EmptyFields, OptionShared<InnerMessageRef>>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    InnerMessageRef: AsMessageImplRef<MessageImplType = InnerMessage>,
    InnerMessage: 'a + GetFieldMethod<'a, NUMBER, ReturnType = InnerReturnType>,
    InnerReturnType: Iterator,
{
    type ReturnType = iter::Flatten<option::IntoIter<InnerReturnType>>;
    fn invoke(&'a self) -> Self::ReturnType {
        self.shared
            .option
            .as_ref()
            .map(|msg| GetFieldMethod::<NUMBER>::invoke(msg.as_message_impl_ref()))
            .into_iter()
            .flatten()
    }
}
