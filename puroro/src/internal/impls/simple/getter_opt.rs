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
use crate::internal::methods::GetOptFieldMethodImpl;
use crate::internal::{Bitfield, SharedBitfield};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::tags;
use crate::AsMessageRef;
use crate::MessageImpl;

trait MethodImpl<'a, IsLd, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetOptFieldMethodImpl<'a, tags::SimpleImpl, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    Self: MethodImpl<'a, TypeTag::IsLd, TypeTag::IsMessage, NUMBER, ReturnType = ReturnType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::FieldTypeTag,
{
    type ReturnType = ReturnType;
    fn invoke(&'a self) -> Self::ReturnType {
        MethodImpl::invoke(self)
    }
}

// (optional|required) numeric field
impl<'a, MP, TypeTag, FieldsType, SharedType, FieldType, NumType, const NUMBER: i32>
    MethodImpl<'a, False, False, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = FieldType>,
    FieldType: Clone + Into<NumType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::NumericalTypeTag<NativeType = NumType>,
    SharedType: SharedBitfield,
{
    type ReturnType = Option<NumType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let opt_bit_index = MP::Fields::<NUMBER>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            let field = HasField::<NUMBER>::get(&self.fields);
            Some(field.clone().into())
        } else {
            None
        }
    }
}

// (optional|required) (string|bytes) field
impl<'a, MP, TypeTag, FieldsType, SharedType, FieldType, BorrowedType, const NUMBER: i32>
    MethodImpl<'a, True, False, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = FieldType>,
    FieldType: 'a + AsRef<BorrowedType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<TypeTag = TypeTag>,
    TypeTag: tags::StringOrBytesTypeTag<BorrowedType = BorrowedType>,
    SharedType: SharedBitfield,
    BorrowedType: 'a + ?Sized,
{
    type ReturnType = Option<&'a BorrowedType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let opt_bit_index = MP::Fields::<NUMBER>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            let field = HasField::<NUMBER>::get(&self.fields);
            Some(field.as_ref())
        } else {
            None
        }
    }
}

// (optional|required|[unlabeled]) message field
// Typically the field type is `Option<Box<M>>`.
impl<
    'a,
    MP,
    FieldsType,
    SharedType,
    FieldMessageAsRefType, // typically `Box<M>`
    FieldMessageType,      // `M`
    const NUMBER: i32,
> MethodImpl<'a, True, True, NUMBER> for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = Option<FieldMessageAsRefType>>,
    FieldMessageAsRefType: 'a + AsMessageRef<MessageType = FieldMessageType>,
    FieldMessageType: 'a,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
{
    type ReturnType = Option<&'a FieldMessageType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let field = HasField::<NUMBER>::get(&self.fields);
        field.as_ref().map(|ref_msg| ref_msg.as_message_ref())
    }
}
