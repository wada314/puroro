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

use crate::internal::methods::{GetFieldMethodImpl, GetOptFieldMethod};
use crate::internal::{Bitfield, SharedBitfield};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::Message;

// (optional|required|[unlabeled]) non-message field
impl<MP, FieldsType, SharedType, GetterType, _1, _2, _3, const NUMBER: i32>
    GetFieldMethodImpl<
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        tags::SimpleImpl,
        tags::NonRepeatedLabel<_1>,
        tags::NonMessageType<_2, _3>,
        NUMBER,
    > for Message<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    tags::NonMessageType<_2, _3>: tags::FieldTypeTag,
    <tags::NonMessageType<_2, _3> as tags::FieldTypeTag>::DefaultValueType:
        Clone + Into<GetterType>,
    for<'a> Self: 'a + GetOptFieldMethod<NUMBER, GetterType<'a> = Option<GetterType>>,
{
    type GetterType<'a>
    where
        Self: 'a,
    = GetterType;
    fn get(&self) -> Self::GetterType<'_> {
        let value_opt = <Self as GetOptFieldMethod<NUMBER>>::get_opt(self);
        value_opt.unwrap_or(
            <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
        )
    }
}
