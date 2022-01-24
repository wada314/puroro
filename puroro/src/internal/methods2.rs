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

use crate::internal::impls::option::{OptionFields, OptionShared};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::MessageImpl;

pub trait GetOptFieldMethod2<const NUMBER: i32> {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_opt(&self) -> Self::GetterType<'_>;
}

pub trait GetOptFieldMethodImpl2<
    ImplTag,
    LabelTag,
    TypeTag,
    FieldType,
    SharedType,
    const NUMBER: i32,
>
{
    type GetterType<'a>
    where
        Self: 'a;
    fn get_opt(&self) -> Self::GetterType<'_>;
}

impl<MP, ImplTag, LabelTag, TypeTag, Fields, Shared, const NUMBER: i32> GetOptFieldMethod2<NUMBER>
    for MessageImpl<MP, ImplTag, Fields, Shared>
where
    Self: GetOptFieldMethodImpl2<ImplTag, LabelTag, TypeTag, Fields, Shared, NUMBER>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
{
    type GetterType<'a>
    where
        Self: 'a = <Self as GetOptFieldMethodImpl2<ImplTag, LabelTag, TypeTag, Fields, Shared, NUMBER>>::GetterType<'a>;
    fn get_opt(&self) -> Self::GetterType<'_> {
        <Self as GetOptFieldMethodImpl2<ImplTag, LabelTag, TypeTag, Fields, Shared, NUMBER>>::get_opt(self)
    }
}
