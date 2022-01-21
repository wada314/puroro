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
use crate::internal::methods::GetOptFieldMethodImpl;
use crate::internal::FieldProperties;
use crate::tags;
/*
impl<'a, FP, T> GetOptFieldMethodImpl<'a, FP, tags::OptionImpl, FP::LabelTag, FP::TypeTag>
    for FieldAndSharedRef<'a, (), OptionShared<T>>
where
    FP: FieldProperties,
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
*/
