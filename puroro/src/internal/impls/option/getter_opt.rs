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
use crate::internal::methods::{GetOptFieldMethod, GetOptFieldMethodImpl};
use crate::internal::{FieldProperties, MessageProperties};
use crate::Message;
use crate::{tags, AsMessageRef};

impl<
    'a,
    MP,
    MessageRef,
    InnerImplTag,
    InnerFieldsType,
    InnerSharedType,
    FinalInnerGetterType,
    const NUMBER: i32,
>
    GetOptFieldMethodImpl<
        'a,
        (),
        OptionShared<MessageRef>,
        tags::OptionImpl,
        <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::LabelTag,
        <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    > for Message<MP, tags::OptionImpl, OptionFields, OptionShared<MessageRef>>
where
    Message<MP, InnerImplTag, InnerFieldsType, InnerSharedType>:
        'a + GetOptFieldMethod<'a, NUMBER, GetterType = Option<FinalInnerGetterType>>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    MessageRef: AsMessageRef<
        MessageProperties = MP,
        ImplTag = InnerImplTag,
        FieldsType = InnerFieldsType,
        SharedType = InnerSharedType,
    >,
{
    type GetterType = Option<FinalInnerGetterType>;
    fn get_opt(&'a self) -> Self::GetterType {
        self.shared
            .option
            .as_ref()
            .and_then(|msg| msg.as_message_ref().get_opt())
    }
}
