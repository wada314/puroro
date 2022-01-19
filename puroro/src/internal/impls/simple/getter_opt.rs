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
use crate::internal::Bitfield;
use crate::internal::FieldProperties;
use crate::internal::{FieldAndSharedRef, SharedObjects};
use crate::tags;

// for [optional|required] numeric types
impl<'a, _1, _2, _3, _4, _5, FP, FieldType, Shared>
    GetOptFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1, _2>,
        tags::NonLdType<_3, _4, _5>,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<
        LabelTag = tags::NeedOptionalBitLabel<_1, _2>,
        TypeTag = tags::NonLdType<_3, _4, _5>,
    >,
    tags::NonLdType<_3, _4, _5>: tags::NumericalTypeTag,
    FP::TypeTag: tags::NumericalTypeTag,
    FieldType: Clone + Into<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let opt_bit_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            Some(self.field.clone().into())
        } else {
            None
        }
    }
}

// for [optional|required] string type
impl<'a, _1, _2, FP, FieldType, Shared>
    GetOptFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1, _2>,
        tags::String,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NeedOptionalBitLabel<_1, _2>, TypeTag = tags::String>,
    FieldType: AsRef<str>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<&'a str>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let opt_bit_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            Some(AsRef::as_ref(self.field))
        } else {
            None
        }
    }
}
