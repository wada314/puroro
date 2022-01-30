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
use crate::AsMessageRef;
use crate::MessageImpl;

// (optional|required) numeric field
impl<'a, MP, FieldsType, SharedType, NumType, _1, _2, const NUMBER: i32>
    GetOptFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1>,
        tags::NonLdType<_2>,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    tags::NonLdType<_2>: tags::NumericalTypeTag<NativeType = NumType>,
    <FieldsType as HasField<NUMBER>>::Type: Clone + Into<NumType>,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    SharedType: SharedBitfield,
{
    type GetterType = Option<NumType>;
    fn get_opt(&'a self) -> Self::GetterType {
        let opt_bit_index = <<MP as MessageProperties>::Fields<NUMBER> as FieldProperties>::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            let field = <FieldsType as HasField<NUMBER>>::get(&self.fields);
            Some(field.clone().into())
        } else {
            None
        }
    }
}

// (optional|required) (string|bytes) field
impl<'a, MP, FieldsType, SharedType, BorrowedType, _1, _2, const NUMBER: i32>
    GetOptFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1>,
        tags::StringOrBytesType<_2>,
        <FieldsType as HasField<NUMBER>>::Type,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER>,
    <FieldsType as HasField<NUMBER>>::Type: AsRef<BorrowedType>,
    tags::StringOrBytesType<_2>: tags::StringOrBytesTypeTag<BorrowedType = BorrowedType>,
    BorrowedType: 'a + ?Sized,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    SharedType: SharedBitfield,
{
    type GetterType = Option<&'a BorrowedType>;
    fn get_opt(&'a self) -> Self::GetterType {
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
    FieldsType,
    SharedType,
    FieldMP,
    FieldMessageAsRefType,
    FieldMessageType, // `M`
    _1,
    const NUMBER: i32,
>
    GetOptFieldMethodImpl<
        'a,
        tags::SimpleImpl,
        tags::NonRepeatedLabel<_1>,
        tags::Message<FieldMP>,
        Option<FieldMessageAsRefType>,
        SharedType,
        NUMBER,
    > for MessageImpl<MP, tags::SimpleImpl, FieldsType, SharedType>
where
    FieldsType: HasField<NUMBER, Type = Option<FieldMessageAsRefType>>,
    FieldMessageAsRefType: 'a + AsMessageRef<MessageType = FieldMessageType>,
    FieldMessageType: 'a,
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
{
    type GetterType = Option<&'a FieldMessageType>;
    fn get_opt(&'a self) -> Self::GetterType {
        let field = <FieldsType as HasField<NUMBER>>::get(&self.fields);
        field.as_ref().map(|ref_msg| ref_msg.as_message_ref())
    }
}
