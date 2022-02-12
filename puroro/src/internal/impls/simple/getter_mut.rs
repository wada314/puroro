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
use crate::internal::methods::GetMutFieldMethodImpl;
use crate::internal::{Bitfield, SharedAllocator, SharedBitfield};
use crate::internal::{FieldProperties, HasField, HasMutField, MessageProperties};
use crate::{tags, AsMessageMut, AsMessageRef};
use crate::{DefaultIn, MessageImpl};

trait MethodImpl<'a, IsLd, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a mut self) -> Self::ReturnType;
}

impl<'a, MP, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetMutFieldMethodImpl<'a, tags::SimpleImpl, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    Self: MethodImpl<'a, TypeTag::IsLd, TypeTag::IsMessage, NUMBER, ReturnType = ReturnType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        MethodImpl::invoke(self)
    }
}

// (optional|required) non-ld field
impl<'a, MP, FieldsType, SharedType, FieldType, const NUMBER: i32>
    MethodImpl<'a, False, False, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasMutField<NUMBER, Type = FieldType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    FieldType: 'a + Default,
    SharedType: SharedBitfield,
{
    type ReturnType = &'a mut FieldType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        let opt_bit_index = MP::Fields::<NUMBER>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if !self.shared.bitfield().get(opt_bit_index) {
            self.shared.bitfield_mut().set(opt_bit_index, true);
            // initailize the field by `Default` value
            *HasMutField::<NUMBER>::get_field_mut(&mut self.fields) = Default::default();
        }
        HasMutField::<NUMBER>::get_field_mut(&mut self.fields)
    }
}

// (optional|required) string|bytes field
impl<'a, MP, FieldsType, SharedType, FieldType, Alloc, const NUMBER: i32>
    MethodImpl<'a, True, False, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasMutField<NUMBER, Type = FieldType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    FieldType: 'a + DefaultIn<AllocatorType = Alloc>,
    SharedType: SharedBitfield + SharedAllocator<AllocatorType = Alloc>,
    Alloc: Clone,
{
    type ReturnType = &'a mut FieldType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        let opt_bit_index = MP::Fields::<NUMBER>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if !self.shared.bitfield().get(opt_bit_index) {
            self.shared.bitfield_mut().set(opt_bit_index, true);
            // initailize the field by `Default` value
            *<FieldsType as HasMutField<NUMBER>>::get_field_mut(&mut self.fields) =
                DefaultIn::default_in(self.shared.alloc().clone());
        }
        <FieldsType as HasMutField<NUMBER>>::get_field_mut(&mut self.fields)
    }
}

// non-repeated message field
impl<
    'a,
    MP,
    FieldsType,
    SharedType,
    FieldMessageAsRefType, // typically `Box<M>`
    FieldMessageType,      // `M`
    Alloc,
    const NUMBER: i32,
> MethodImpl<'a, True, True, NUMBER> for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = Option<FieldMessageAsRefType>> + HasMutField<NUMBER>,
    FieldMessageAsRefType: 'a
        + DefaultIn<AllocatorType = Alloc>
        + AsMessageRef<MessageType = FieldMessageType>
        + AsMessageMut,
    FieldMessageType: 'a,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    SharedType: SharedAllocator<AllocatorType = Alloc>,
    Alloc: Clone,
{
    type ReturnType = &'a mut FieldMessageType;
    fn invoke(&'a mut self) -> Self::ReturnType {
        let field_opt = HasMutField::<NUMBER>::get_field_mut(&mut self.fields);
        field_opt
            .get_or_insert_with(|| FieldMessageAsRefType::default_in(self.shared.alloc().clone()))
            .as_message_mut()
    }
}
