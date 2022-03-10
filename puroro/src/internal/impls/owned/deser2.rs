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

use crate::deser::{DeserContext, DeserFromBytesImpl, ScopedIterator};
use crate::deser2::DeserFromSlice;
use crate::internal::bool::{False, True};
use crate::internal::methods::GetMutFieldMethod;
use crate::internal::types::WireType;
use crate::internal::variant;
use crate::internal::variant::Variant;
use crate::internal::{
    methods, FieldHandlerBase, FieldHandlerMut, FieldProperties, MessageProperties, SharedBitfield,
};
use crate::{tags, AsMessageImplMut};
use crate::{AsMessageImplRef, MessageImpl};
use crate::{ErrorKind, Result};
use ::std::cell::RefCell;
use ::std::io::Result as IoResult;
use ::std::rc::Rc;

pub struct GetMaybeLastMutMessageHandler {}

impl FieldHandlerBase for GetMaybeLastMutMessageHandler {
    type ReturnType<'a> = Option<&'a mut dyn DeserFromSlice>;
}

impl<MP, LabelTag, TypeTag, FieldsType, SharedType, const NUMBER: i32>
    FieldHandlerMut<MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>, NUMBER>
    for GetMaybeLastMutMessageHandler
where
    MP: MessageProperties,
    MP::Fields<NUMBER>: FieldProperties<LabelTag = LabelTag, TypeTag = TypeTag>,
    LabelTag: tags::FieldLabelTag,
    TypeTag: tags::FieldTypeTag,
    Self: HandlerImpl<
        TypeTag,
        MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>,
        LabelTag::IsRepeated,
        TypeTag::IsMessage,
        NUMBER,
    >,
{
    fn handle_mut<'a>(
        self,
        message: &'a mut MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>,
    ) -> Result<Self::ReturnType<'a>> {
        Ok(self.handle_mut_impl(message)?)
    }
}

pub trait HandlerImpl<TypeTag, MsgImplType, IsRepeated, IsMessage, const NUMBER: i32> {
    fn handle_mut_impl<'a>(
        self,
        message: &'a mut MsgImplType,
    ) -> Result<Option<&'a mut dyn DeserFromSlice>>;
}

impl<FieldMsgType, MsgImplType, const NUMBER: i32>
    HandlerImpl<tags::Message<FieldMsgType>, MsgImplType, False, True, NUMBER>
    for GetMaybeLastMutMessageHandler
where
    for<'a> MsgImplType: methods::GetMutFieldMethod<'a, NUMBER, ReturnType = &'a mut FieldMsgType>,
    FieldMsgType: 'static + AsMessageImplMut,
    FieldMsgType::MessageImplType: 'static + DeserFromSlice,
{
    fn handle_mut_impl<'a>(
        self,
        message: &'a mut MsgImplType,
    ) -> Result<Option<&'a mut dyn DeserFromSlice>> {
        Ok(Some(message.invoke_get_mut().as_message_impl_mut()))
    }
}

impl<TypeTag, MsgImplType, IsRepeated, const NUMBER: i32>
    HandlerImpl<TypeTag, MsgImplType, IsRepeated, False, NUMBER> for GetMaybeLastMutMessageHandler
{
    fn handle_mut_impl<'a>(
        self,
        message: &'a mut MsgImplType,
    ) -> Result<Option<&'a mut dyn DeserFromSlice>> {
        Ok(None)
    }
}
