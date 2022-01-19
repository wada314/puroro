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

use crate::internal::methods::{GetFieldMethodImpl, GetOptFieldMethod};
use crate::internal::FieldProperties;
use crate::internal::{FieldAndSharedRef, SharedObjects};
use crate::tags;

// [optional|required|(unlabeled)] non-message field
// assuming that getter_opt method is implemented, and the `FieldPropetry::DEFAULT_VALUE`
// type is the same as the getter's return type.
impl<'a, _1, _2, _3, FP, FieldType, Shared, GetterType>
    GetFieldMethodImpl<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NonRepeatedLabel<_1>,
        tags::NonMessageType<_2, _3>,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    GetterType: Default,
    FP: FieldProperties<
        LabelTag = tags::NonRepeatedLabel<_1>,
        TypeTag = tags::NonMessageType<_2, _3>,
    >,
    tags::NonMessageType<_2, _3>: tags::FieldTypeTag, /*<DefaultValueType = GetterType>*/
    //<tags::NonMessageType<_2, _3> as tags::FieldTypeTag>::DefaultValueType: Into<GetterType>,
    Self: GetOptFieldMethod<'a, FP, tags::SimpleImpl, GetterType = Option<GetterType>>,
{
    type GetterTypeImpl = GetterType;
    fn get_impl(&self) -> Self::GetterTypeImpl {
        let opt_value = <Self as GetOptFieldMethod<FP, tags::SimpleImpl>>::get_opt(self);
        opt_value.unwrap_or(/*Into::into(FP::DEFAULT_VALUE)*/ Default::default())
    }
}

// [optional|required|(unlabeled)] message field
// assuming that getter_opt method is implemented.
impl<'a, _1, FP, MP, FieldType, Shared>
    GetFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::NonRepeatedLabel<_1>, tags::Message<MP>>
    for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::Message<MP>>,
    Shared: SharedObjects,
    Self: GetOptFieldMethod<'a, FP, tags::SimpleImpl>,
{
    type GetterTypeImpl = <Self as GetOptFieldMethod<'a, FP, tags::SimpleImpl>>::GetterType;
    fn get_impl(&self) -> Self::GetterTypeImpl {
        <Self as GetOptFieldMethod<FP, tags::SimpleImpl>>::get_opt(self)
    }
}

// repeated field
impl<'a, FP, ScalarType, Shared, TypeTag>
    GetFieldMethodImpl<'a, FP, tags::SimpleImpl, tags::Repeated, TypeTag>
    for FieldAndSharedRef<'a, Vec<ScalarType>, Shared>
where
    FP: FieldProperties<LabelTag = tags::Repeated, TypeTag = TypeTag>,
    Shared: SharedObjects,
{
    type GetterTypeImpl = &'a [ScalarType];
    fn get_impl(&self) -> Self::GetterTypeImpl {
        self.field
    }
}
