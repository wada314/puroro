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

use crate::internal::bool::{False, True};
use crate::internal::methods::{GetFieldMethodImpl, GetFieldMethodImplImpl};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::MessageImpl;
use ::itertools::structs::MapInto;
use ::itertools::Itertools;
use ::std::iter::Cloned;

trait MethodImpl<'a, LabelTag, TypeTag, FieldType, SharedType, IsLd, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, LabelTag, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetFieldMethodImpl<'a, tags::SimpleImpl, False, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    Self: MethodImpl<
        'a,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        <TypeTag as tags::FieldTypeTag>::IsLd,
        NUMBER,
        ReturnType = ReturnType,
    >,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    FieldsType: HasField<NUMBER>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        MethodImpl::invoke(self)
    }
}

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
    GetFieldMethodImplImpl<
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
    type ReturnType = MapInto<Cloned<InnerIterType>, NumType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let slice = <FieldsType as HasField<NUMBER>>::get(&self.fields);
        slice.into_iter().cloned().map_into::<NumType>()
    }
}

// repeated ld field
// Just return the `into_iter()` value as-is
impl<'a, MP, FieldsType, SharedType, LdType, FieldType, InnerIterType, const NUMBER: i32>
    GetFieldMethodImplImpl<
        'a,
        tags::SimpleImpl,
        tags::Repeated,
        tags::LengthDelimited<LdType>,
        FieldType,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = FieldType>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<LabelTag = tags::Repeated, TypeTag = tags::LengthDelimited<LdType>>,
    FieldType: 'a,
    &'a FieldType: IntoIterator<IntoIter = InnerIterType>,
{
    type ReturnType = InnerIterType;
    fn invoke(&'a self) -> Self::ReturnType {
        let slice = <FieldsType as HasField<NUMBER>>::get(&self.fields);
        slice.into_iter()
    }
}
