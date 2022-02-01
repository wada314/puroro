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

use crate::internal::methods::GetMutFieldMethodImpl;
use crate::internal::{Bitfield, SharedBitfield};
use crate::internal::{FieldProperties, HasField, HasMutField, MessageProperties};
use crate::tags;
use crate::AsMessageMut;
use crate::MessageImpl;

// (optional|required) numeric field
impl<'a, MP, FieldsType, SharedType, NumType, FieldType, _1, _2, const NUMBER: i32>
    GetMutFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1>,
        tags::NonLdType<_2>,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasMutField<NUMBER, Type = FieldType>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties<TypeTag = tags::NonLdType<_2>>,
    tags::NonLdType<_2>: tags::FieldTypeTag + tags::NumericalTypeTag<NativeType = NumType>,
    <tags::NonLdType<_2> as tags::FieldTypeTag>::DefaultValueType: Clone + Into<FieldType>,
    FieldType: 'a,
    SharedType: SharedBitfield,
{
    type GetterType = &'a mut FieldType;
    fn get_mut(&'a mut self) -> Self::GetterType {
        let opt_bit_index = <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if !self.shared.bitfield().get(opt_bit_index) {
            self.shared.bitfield_mut().set(opt_bit_index, true);
            // initailize the field by default value
            *<FieldsType as HasMutField<NUMBER>>::get_mut(&mut self.fields) =
                Into::into(Clone::clone(
                    &<<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::DEFAULT_VALUE,
                ));
        }
        <FieldsType as HasMutField<NUMBER>>::get_mut(&mut self.fields)
    }
}
