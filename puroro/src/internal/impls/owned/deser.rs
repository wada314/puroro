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

use crate::deser::DeserOptions;
use crate::internal::bool::False;
use crate::internal::methods::GetMutFieldMethod;
use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::{
    methods, FieldHandlerBase, FieldHandlerMut, FieldProperties, MessageProperties, SharedBitfield,
};
use crate::tags;
use crate::MessageImpl;
use crate::{ErrorKind, Result};
use ::std::io::Result as IoResult;
use ::std::marker::PhantomData;

pub struct DeserOwnedFieldHandler<MP, FieldsType, SharedType, Iter> {
    pub(crate) bytes: Iter,
    pub(crate) wire_type: WireType,
    pub(crate) recursion_level: usize,
    pub(crate) options: DeserOptions,
    pub(crate) _phantom: PhantomData<(MP, FieldsType, SharedType)>,
}

trait DeserOwnedFieldImpl<LabelTag, TypeTag, MessageImplType, IsRepeated, const NUMBER: i32> {
    fn deser_field(&mut self, message: &mut MessageImplType) -> Result<()>;
}

impl<MP, FieldsType, SharedType, Iter> FieldHandlerBase
    for DeserOwnedFieldHandler<MP, FieldsType, SharedType, Iter>
{
    type ReturnType = ();
}

impl<MP, LabelTag, TypeTag, FieldsType, SharedType, Iter, const NUMBER: i32> FieldHandlerMut<NUMBER>
    for DeserOwnedFieldHandler<MP, FieldsType, SharedType, Iter>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    Self: DeserOwnedFieldImpl<
        LabelTag,
        TypeTag,
        MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>,
        LabelTag::IsRepeated,
        NUMBER,
    >,
{
    type MessageType = MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>;
    fn handle_mut(&mut self, message: &mut Self::MessageType) -> Result<Self::ReturnType> {
        if let Some(recursion_limit) = self.options.recursion_limit {
            if self.recursion_level >= recursion_limit {
                Err(ErrorKind::DeserRecursionOverflow())?
            }
        }
        Ok(self.deser_field(message)?)
    }
}

// numerical non-repeated fields
impl<MP, LabelTag, VariantTypeTag, FieldType, FieldsType, SharedType, Iter, const NUMBER: i32>
    DeserOwnedFieldImpl<
        LabelTag,
        tags::Variant<VariantTypeTag>,
        MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>,
        False, /* IsRepeated */
        NUMBER,
    > for DeserOwnedFieldHandler<MP, FieldsType, SharedType, Iter>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    Iter: Iterator<Item = IoResult<u8>>,
    tags::Variant<VariantTypeTag>:
        tags::NumericalTypeTag<NativeType = FieldType> + crate::internal::variant::VariantTypeTag,
    LabelTag: tags::FieldLabelTag,
    for<'a> MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>:
        methods::GetMutFieldMethod<'a, NUMBER, ReturnType = &'a mut FieldType>,
{
    fn deser_field(
        &mut self,
        message: &mut MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>,
    ) -> Result<()> {
        match self.wire_type {
            WireType::Variant => {
                let variant = Variant::decode_bytes(&mut self.bytes)?;

                if LabelTag::DO_DEFAULT_CHECK && variant.is_zero() {
                    // If the field is proto3 unlabaled type, then do not touch
                    // the field value.
                    return Ok(());
                }

                *message.invoke_get_mut() = variant.to_native::<tags::Variant<VariantTypeTag>>()?;
                Ok(())
            }
            WireType::LengthDelimited => {
                todo!()
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
    }
}

// `string` proto field where the rust's field type is `std::string::String`.
impl<MP, LabelTag, FieldsType, SharedType, Iter, const NUMBER: i32>
    DeserOwnedFieldImpl<
        LabelTag,
        tags::String,
        MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>,
        False, /* IsRepeated */
        NUMBER,
    > for DeserOwnedFieldHandler<MP, FieldsType, SharedType, Iter>
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties,
    Iter: Iterator<Item = IoResult<u8>>,
    SharedType: SharedBitfield,
    LabelTag: tags::FieldLabelTag,

    for<'a> MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>:
        methods::GetMutFieldMethod<'a, NUMBER, ReturnType = &'a mut String>,
{
    fn deser_field(
        &mut self,
        message: &mut MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>,
    ) -> Result<()> {
        if let WireType::LengthDelimited = self.wire_type {
            let length: usize = Variant::decode_bytes(&mut self.bytes)?
                .to_u32()?
                .try_into()?;

            if LabelTag::DO_DEFAULT_CHECK && length == 0 {
                // If the field is proto3 unlabaled type, then do not touch
                // the field value.
                return Ok(());
            }

            let field: &mut String = message.invoke_get_mut();
            if field.capacity() < length {
                field.reserve(length - field.capacity());
            }
            let mut inner_vec = ::std::mem::take(field).into_bytes();
            for byte in self.bytes.by_ref().take(length) {
                inner_vec.push(byte?);
            }
            if self.options.do_utf8_check {
                *field = String::from_utf8(inner_vec).map_err(|e| Into::<ErrorKind>::into(e))?;
            } else {
                unsafe {
                    *field = String::from_utf8_unchecked(inner_vec);
                }
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}
