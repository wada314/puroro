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

use crate::internal::methods::GetFieldMethodImpl;
use crate::internal::FieldProperties;
use crate::internal::{FieldAndSharedRef, SharedObjects};
use crate::tags;

// for repeated message type
impl<'a, FP, MP, MessageType, Shared>
    GetFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::Repeated, tags::Message<MP>>
    for FieldAndSharedRef<'a, Vec<MessageType>, Shared>
where
    FP: FieldProperties<LabelTag = tags::Repeated, TypeTag = tags::Message<MP>>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = &'a [MessageType];
    fn get_impl(&self) -> Self::GetterTypeImpl {
        self.field
    }
}
