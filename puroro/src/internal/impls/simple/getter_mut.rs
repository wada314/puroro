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
use crate::internal::{Bitfield, SharedAllocator, SharedBitfield};
use crate::internal::{FieldProperties, HasField, HasMutField, MessageProperties};
use crate::{tags, AsMessageMut, AsMessageRef};
use crate::{DefaultIn, MessageImpl};

// (optional|required) non-message field
impl<'a, MP, FieldsType, SharedType, FieldType, _1, _2, const NUMBER: i32>
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
    FieldType: 'a + Default,
    SharedType: SharedBitfield,
{
    type ReturnType = &'a mut FieldType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        let opt_bit_index = <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if !self.shared.bitfield().get(opt_bit_index) {
            self.shared.bitfield_mut().set(opt_bit_index, true);
            // initailize the field by `Default` value
            *<FieldsType as HasMutField<NUMBER>>::get_mut(&mut self.fields) = Default::default();
        }
        <FieldsType as HasMutField<NUMBER>>::get_mut(&mut self.fields)
    }
}

// (optional|required) string|bytes field
impl<'a, MP, FieldsType, SharedType, FieldType, Alloc, _1, _2, const NUMBER: i32>
    GetMutFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1>,
        tags::StringOrBytesType<_2>,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasMutField<NUMBER, Type = FieldType>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>:
        FieldProperties<TypeTag = tags::StringOrBytesType<_2>>,
    FieldType: 'a + DefaultIn<AllocatorType = Alloc>,
    SharedType: SharedBitfield + SharedAllocator<AllocatorType = Alloc>,
    Alloc: Clone,
{
    type ReturnType = &'a mut FieldType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        let opt_bit_index = <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if !self.shared.bitfield().get(opt_bit_index) {
            self.shared.bitfield_mut().set(opt_bit_index, true);
            // initailize the field by `Default` value
            *<FieldsType as HasMutField<NUMBER>>::get_mut(&mut self.fields) =
                DefaultIn::default_in(self.shared.alloc().clone());
        }
        <FieldsType as HasMutField<NUMBER>>::get_mut(&mut self.fields)
    }
}

// non-repeated message field
impl<
    'a,
    MP,
    FieldsType,
    SharedType,
    FieldMP,
    FieldMessageAsRefType, // typically `Box<M>`
    FieldMessageType,      // `M`
    Alloc,
    _1,
    const NUMBER: i32,
>
    GetMutFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::NonRepeatedLabel<_1>,
        tags::Message<FieldMP>,
        Option<FieldMessageAsRefType>,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = Option<FieldMessageAsRefType>> + HasMutField<NUMBER>,
    FieldMessageAsRefType: 'a
        + DefaultIn<AllocatorType = Alloc>
        + AsMessageRef<MessageType = FieldMessageType>
        + AsMessageMut,
    FieldMessageType: 'a,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    SharedType: SharedAllocator<AllocatorType = Alloc>,
    Alloc: Clone,
{
    type ReturnType = &'a mut FieldMessageType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        let field_opt = <FieldsType as HasMutField<NUMBER>>::get_mut(&mut self.fields);
        field_opt
            .get_or_insert_with(|| DefaultIn::default_in(self.shared.alloc().clone()))
            .as_message_mut()
    }
}
