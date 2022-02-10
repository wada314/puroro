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

trait MethodImpl<'a, LabelTag, TypeTag, FieldType, SharedType, IsLd, IsMessage, const NUMBER: i32> {
    type ReturnType;
    fn invoke(&'a self) -> Self::ReturnType;
}

impl<'a, MP, LabelTag, TypeTag, FieldsType, SharedType, ReturnType, const NUMBER: i32>
    GetOptFieldMethodImpl<'a, tags::SimpleImpl, NUMBER>
    for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    Self: MethodImpl<
        'a,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        <TypeTag as tags::FieldTypeTag>::IsLd,
        <TypeTag as tags::FieldTypeTag>::IsMessage,
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

// (optional|required) numeric field
impl<'a, MP, LabelTag, TypeTag, FieldsType, SharedType, NumType, const NUMBER: i32>
    MethodImpl<
        'a,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        False,
        False,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    TypeTag: tags::NumericalTypeTag<NativeType = NumType>,
    <FieldsType as HasField<NUMBER>>::Type: Clone + Into<NumType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    SharedType: SharedBitfield,
{
    type ReturnType = Option<NumType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let opt_bit_index = <MP::Fields<NUMBER> as FieldProperties>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            let field = <FieldsType as HasField<NUMBER>>::get(&self.fields);
            Some(field.clone().into())
        } else {
            None
        }
    }
}

// (optional|required) (string|bytes) field
impl<'a, MP, LabelTag, TypeTag, FieldsType, SharedType, BorrowedType, const NUMBER: i32>
    MethodImpl<
        'a,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        True,
        False,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    TypeTag: tags::StringOrBytesTypeTag<BorrowedType = BorrowedType>,
    <FieldsType as HasField<NUMBER>>::Type: AsRef<BorrowedType>,
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    SharedType: SharedBitfield,
    BorrowedType: 'a + ?Sized,
{
    type ReturnType = Option<&'a BorrowedType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let opt_bit_index = <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            let field = <FieldsType as HasField<NUMBER>>::get(&self.fields);
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
    LabelTag,
    TypeTag,
    FieldsType,
    SharedType,
    FieldMessageAsRefType, // typically `Box<M>`
    FieldMessageType,      // `M`
    const NUMBER: i32,
>
    MethodImpl<
        'a,
        LabelTag,
        TypeTag,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        True,
        True,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = Option<FieldMessageAsRefType>>,
    FieldMessageAsRefType: 'a + AsMessageRef<MessageType = FieldMessageType>,
    FieldMessageType: 'a,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
{
    type ReturnType = Option<&'a FieldMessageType>;
    fn invoke(&'a self) -> Self::ReturnType {
        let field = <FieldsType as HasField<NUMBER>>::get(&self.fields);
        field.as_ref().map(|ref_msg| ref_msg.as_message_ref())
    }
}
