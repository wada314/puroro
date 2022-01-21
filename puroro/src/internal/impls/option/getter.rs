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

use crate::internal::fake_message_impl;
use crate::internal::impls::option::{OptionFields, OptionShared};
use crate::internal::methods::{GetFieldMethod, GetFieldMethodImpl};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::Message;

// (optional|required|[unlabeled]) field
// redirect to simple
impl<'a, MP, FieldsType, SharedType, _1, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        tags::OptionImpl,
        tags::NonRepeatedLabel<_1>,
        <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::TypeTag,
        NUMBER,
    > for Message<MP, tags::OptionImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>>,
    Message<MP, tags::SimpleImpl, FieldsType, SharedType>: GetFieldMethod<'a, NUMBER>,
{
    type GetterType = <Message<MP, tags::SimpleImpl, FieldsType, SharedType> as GetFieldMethod<
        'a,
        NUMBER,
    >>::GetterType;
    fn get(&'a self) -> Self::GetterType {
        <Message<MP, tags::SimpleImpl, FieldsType, SharedType> as GetFieldMethod<'a, NUMBER>>::get(
            fake_message_impl(self),
        )
    }
}
