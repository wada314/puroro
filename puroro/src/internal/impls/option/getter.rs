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
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImpl};
use crate::internal::{EmptyFields, FieldProperties, HasField, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageRef};

// repeated field
// Assuming the internal type's getter type is `&[T]` or whatever `Default` type
impl<'a, MP, TypeTag, InnerMessageRef, InnerMessage, InnerGetterType, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        tags::OptionImpl,
        tags::Repeated,
        TypeTag,
        <EmptyFields as HasField<NUMBER>>::Type,
        OptionShared<InnerMessageRef>,
        NUMBER,
    > for MessageImpl<MP, tags::OptionImpl, EmptyFields, OptionShared<InnerMessageRef>>
where
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::Repeated, TypeTag = TypeTag>,
    InnerMessageRef: AsMessageRef<MessageType = InnerMessage>,
    InnerMessage: 'a + GetFieldMethod<'a, NUMBER, GetterType = InnerGetterType>,
    InnerGetterType: Default,
{
    type GetterType = InnerGetterType;
    fn get(&'a self) -> Self::GetterType {
        self.shared
            .option
            .as_ref()
            .map(|msg| {
                <InnerMessage as GetFieldMethod<NUMBER>>::get(
                    <InnerMessageRef as AsMessageRef>::as_message_ref(&msg),
                )
            })
            .unwrap_or_default()
    }
}
