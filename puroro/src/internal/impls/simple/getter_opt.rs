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

// [optional|required] numeric field
impl<'a, _1, _2, FP, FieldType, Shared>
    GetOptFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NeedOptionalBitLabel<_1>,
        tags::NonLdType<_2>,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NeedOptionalBitLabel<_1>, TypeTag = tags::NonLdType<_2>>,
    tags::NonLdType<_2>: tags::NumericalTypeTag,
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

// [optional|required] string field
impl<'a, _1, FP, FieldType, Shared>
    GetOptFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::NeedOptionalBitLabel<_1>, tags::String>
    for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NeedOptionalBitLabel<_1>, TypeTag = tags::String>,
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

// [optional|required] bytes field
impl<'a, _1, FP, FieldType, Shared>
    GetOptFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::NeedOptionalBitLabel<_1>, tags::Bytes>
    for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NeedOptionalBitLabel<_1>, TypeTag = tags::Bytes>,
    FieldType: AsRef<[u8]>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<&'a [u8]>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let opt_bit_index = FP::OPTIONAL_FIELD_BITFIELD_INDEX;
        if self.shared.bitfield().get(opt_bit_index) {
            Some(AsRef::as_ref(self.field))
        } else {
            None
        }
    }
}

// (unlabeled) numeric field
// Returns `Some(...)` when the field value is non-default value,
// and returns `None` if it is default value.
impl<'a, _1, FP, FieldType, Shared>
    GetOptFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::Unlabeled, tags::NonLdType<_1>>
    for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::Unlabeled, TypeTag = tags::NonLdType<_1>>,
    tags::NonLdType<_1>: tags::NumericalTypeTag,
    FP::TypeTag: tags::NumericalTypeTag,
    FieldType: Clone + Into<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<<FP::TypeTag as tags::NumericalTypeTag>::NativeType>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        let inner_value = Into::into(Clone::clone(self.field));
        if inner_value == Default::default() {
            None
        } else {
            Some(inner_value)
        }
    }
}

// [optional|required|(unlabeled)] message field
// The field value type is `Option<Box<M>>`, where the return type should be
// `Option<&M>`.
impl<'a, _1, MP, FP, MessageType, Shared>
    GetOptFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::NonRepeatedLabel<_1>, tags::Message<MP>>
    for FieldAndSharedRef<'a, Option<Box<MessageType>>, Shared>
where
    FP: FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::Message<MP>>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = Option<&'a MessageType>;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl {
        self.field.as_deref()
    }
}
