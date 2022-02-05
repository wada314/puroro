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
use ::itertools::structs::MapInto;
use ::itertools::Itertools;
use ::std::iter::Cloned;
use ::std::ops::Deref;
use ::std::slice;

// repeated non-ld field
// `Clone` and then `Into` the iter value
impl<
    'a,
    MP,
    FieldsType,
    SharedType,
    FieldType,
    InnerIterType,
    ItemType,
    NumType,
    _1,
    const NUMBER: i32,
>
    GetFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::Repeated,
        tags::NonLdType<_1>,
        FieldType,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = FieldType>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::Repeated, TypeTag = tags::NonLdType<_1>>,
    tags::NonLdType<_1>: tags::NumericalTypeTag<NativeType = NumType>,
    FieldType: 'a,
    &'a FieldType: IntoIterator<Item = &'a ItemType, IntoIter = InnerIterType>,
    InnerIterType: Iterator<Item = &'a ItemType>,
    ItemType: 'a + Clone + Into<NumType>,
{
    type GetterType = MapInto<Cloned<InnerIterType>, NumType>;
    fn get(&'a self) -> Self::GetterType {
        let slice = <FieldsType as HasField<NUMBER>>::get(&self.fields);
        slice.into_iter().cloned().map_into::<NumType>()
    }
}
