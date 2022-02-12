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

use super::OptionShared;
use crate::internal::methods::{GetOptFieldMethod, GetOptFieldMethodImpl};
use crate::MessageImpl;
use crate::{tags, AsMessageImplRef};

// non-repeated field
// If the inner `Option` is `Some` then delegate to the inner type.
// If it's `None`, then just return `None`.
impl<'a, MP, FieldsType, InnerMessageRef, InnerMessage, InnerReturnType, const NUMBER: i32>
    GetOptFieldMethodImpl<'a, tags::OptionImpl, NUMBER>
    for MessageImpl<MP, tags::OptionImpl, FieldsType, OptionShared<InnerMessageRef>>
where
    InnerMessageRef: AsMessageImplRef<MessageImplType = InnerMessage>,
    InnerMessage: 'a + GetOptFieldMethod<'a, NUMBER, ReturnType = Option<InnerReturnType>>,
{
    type ReturnType = Option<InnerReturnType>;
    fn invoke_get_opt_impl(&'a self) -> Self::ReturnType {
        self.shared
            .option
            .as_ref()
            .and_then(|msg| msg.as_message_impl_ref().invoke_get_opt())
    }
}
