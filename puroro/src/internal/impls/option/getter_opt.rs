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

use super::{OptionFields, OptionShared};
use crate::internal::methods::{GetOptFieldMethod, GetOptFieldMethodImpl};
use crate::internal::{FieldProperties, HasField, MessageProperties};
use crate::MessageImpl;
use crate::{tags, AsMessageRef};

// non-repeated field
// If the inner `Option` is `Some` then delegate to the inner type.
// If it's `None`, then just return `None`.
impl<MP, TypeTag, InnerMessageRef, InnerMessage, InnerGetterType, _1, const NUMBER: i32>
    GetOptFieldMethodImpl<
        tags::OptionImpl,
        tags::NonRepeatedLabel<_1>,
        TypeTag,
        <OptionFields as HasField<NUMBER>>::Type,
        OptionShared<InnerMessageRef>,
        NUMBER,
    > for MessageImpl<MP, tags::OptionImpl, OptionFields, OptionShared<InnerMessageRef>>
where
    MP: MessageProperties,
    <MP as MessageProperties>::Fields<NUMBER>: FieldProperties,
    InnerMessageRef: AsMessageRef<MessageType = InnerMessage>,
    InnerMessage: 'static,
    for<'a> InnerMessage: GetOptFieldMethod<NUMBER, GetterType<'a> = Option<InnerGetterType>>,
{
    type GetterType<'a>
    where
        Self: 'a,
    = Option<InnerGetterType>;
    fn get_opt(&self) -> Self::GetterType<'_> {
        self.shared.option.as_ref().and_then(|msg| {
            <InnerMessage as GetOptFieldMethod<NUMBER>>::get_opt(
                <InnerMessageRef as AsMessageRef>::as_message_ref(&msg),
            )
        })
    }
}
