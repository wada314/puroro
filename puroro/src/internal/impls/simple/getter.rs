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
use crate::internal::HasField;
use crate::tags;
use crate::Message;

// repeated field
impl<'a, MP, FieldsType, SharedType, TypeTag, const NUMBER: i32>
    GetFieldMethodImpl<
        'a,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        tags::SimpleImpl,
        tags::Repeated,
        TypeTag,
        NUMBER,
    > for Message<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    <FieldsType as HasField<NUMBER>>::Type: 'a,
{
    type GetterType = &'a <FieldsType as HasField<NUMBER>>::Type;
    fn get(&'a self) -> Self::GetterType {
        <FieldsType as HasField<NUMBER>>::get(&self.fields)
    }
}
