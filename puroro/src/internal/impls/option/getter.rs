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

use super::{OptionFields, OptionShared};
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImpl};
use crate::internal::{FieldProperties, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageRef};

// repeated field
// Assuming the internal type's getter type is `&[T]` or whatever `Default` type
impl<
    'a,
    MP,
    MessageRef,
    InnerImplTag,
    InnerFieldsType,
    InnerSharedType,
    GetterType,
    const NUMBER: i32,
>
    GetFieldMethodImpl<
        'a,
        (),
        OptionShared<MessageRef>,
        tags::OptionImpl,
        tags::Repeated,
        <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    > for MessageImpl<MP, tags::OptionImpl, OptionFields, OptionShared<MessageRef>>
where
    MessageImpl<MP, InnerImplTag, InnerFieldsType, InnerSharedType>:
        'a + GetFieldMethod<'a, NUMBER, GetterType = GetterType>,
    GetterType: Default,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    MessageRef:
        AsMessageRef<MessageType = MessageImpl<MP, InnerImplTag, InnerFieldsType, InnerSharedType>>,
{
    type GetterType = GetterType;
    fn get(&'a self) -> Self::GetterType {
        self.shared
            .option
            .as_ref()
            .map(|msg| msg.as_message_ref().get())
            .unwrap_or_default()
    }
}
