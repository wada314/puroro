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

use crate::internal::methods::GetOptFieldMethodImpl;
use crate::internal::{Bitfield, SharedBitfield};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::Message;

type NumType<_1> = <tags::NonLdType<_1> as tags::NumericalTypeTag>::NativeType;

// [optional|required] numeric field
impl<MP, FieldsType, SharedType, _1, _2, const NUMBER: i32>
    GetOptFieldMethodImpl<
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1>,
        tags::NonLdType<_2>,
        NUMBER,
    > for Message<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    tags::NonLdType<_2>: tags::NumericalTypeTag,
    <FieldsType as HasField<NUMBER>>::Type: Clone + Into<NumType<_2>>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    SharedType: SharedBitfield,
{
    type GetterType = Option<NumType<_2>>;
    fn get_opt(&self) -> Self::GetterType {
        let opt_bit_index = <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            let field = <FieldsType as HasField<NUMBER>>::get(&self.fields);
            Some(field.clone().into())
        } else {
            None
        }
    }
}
