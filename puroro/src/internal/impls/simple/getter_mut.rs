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
use crate::internal::FieldProperties;
use crate::internal::{FieldAndSharedMut, SharedObjects};
use crate::tags;

// [unlabeled] numeric field
impl<'a, _1, FP, FieldType, Shared>
    GetMutFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::Unlabeled, tags::NonLdType<_1>>
    for FieldAndSharedMut<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::Unlabeled, TypeTag = tags::NonLdType<_1>>,
{
    type GetterTypeImpl = &'a mut FieldType;
    fn get_mut_impl(self) -> Self::GetterTypeImpl {
        // Just return the mutable reference.
        self.field
    }
}
