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
use crate::internal::{Bitfield, FieldAndSharedMut, SharedObjects};
use crate::tags;

// [unlabeled] non-message field.
// Just return a mutable reference to the field.
impl<'a, _1, _2, FP, FieldType, Shared>
    GetMutFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::Unlabeled, tags::NonMessageType<_1, _2>>
    for FieldAndSharedMut<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::Unlabeled, TypeTag = tags::NonMessageType<_1, _2>>,
{
    type GetterTypeImpl = &'a mut FieldType;
    fn get_mut_impl(self) -> Self::GetterTypeImpl {
        self.field
    }
}

// (optional|required) non-message field,
// Check the optional bit, set if it's not set,
// initialize the field if the optional bit was not set, and then return it.
impl<'a, _1, _2, _3, FP, FieldType, Shared>
    GetMutFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1>,
        tags::NonMessageType<_2, _3>,
    > for FieldAndSharedMut<'a, FieldType, Shared>
where
    FP: FieldProperties<
        LabelTag = tags::NeedOptionalBitLabel<_1>,
        TypeTag = tags::NonMessageType<_2, _3>,
    >,
    tags::NonMessageType<_2, _3>: tags::FieldTypeTag,
    <tags::NonMessageType<_2, _3> as tags::FieldTypeTag>::DefaultValueType: Clone + Into<FieldType>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = &'a mut FieldType;
    fn get_mut_impl(self) -> Self::GetterTypeImpl {
        let bitfield_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX;
        if !self.shared.bitfield().get(bitfield_index) {
            // need to initialize
            self.shared.bitfield_mut().set(bitfield_index, true);
            *self.field = Into::into(Clone::clone(&FP::DEFAULT_VALUE));
        }
        self.field
    }
}
