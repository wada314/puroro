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

use crate::internal::methods::GetFieldMethodImpl;
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::MessageImpl;

// repeated field
impl<'a, MP, FieldsType, SharedType, ItemType, TypeTag, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::Repeated,
        TypeTag,
        Vec<ItemType>,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = Vec<ItemType>>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::Repeated, TypeTag = TypeTag>,
    ItemType: 'a,
{
    type GetterType = &'a [ItemType];
    fn get(&'a self) -> Self::GetterType {
        <FieldsType as HasField<NUMBER>>::get(&self.fields).as_slice()
    }
}
