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

use crate::internal::methods::{GetFieldMethodImpl2, GetOptFieldMethod};
use crate::internal::FieldProperties;
use crate::internal::{FieldAndSharedRef, SharedObjects};
use crate::tags;

// [optional|required|(unlabeled)] numeric field
// assuming that getter_opt method is implemented, and the `FieldPropetry::DEFAULT_VALUE`
// type is the same as the getter's return type.
impl<'a, _1, _2, FP, FieldType, Shared, GetterType>
    GetFieldMethodImpl2<'a, FP, tags::SimpleImpl, tags::NonRepeatedLabel<_1>, tags::NonLdType<_2>>
    for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<LabelTag = tags::NonRepeatedLabel<_1>, TypeTag = tags::NonLdType<_2>>,
    tags::NonLdType<_2>: tags::FieldTypeTag<DefaultValueType = GetterType>,
    Self: GetOptFieldMethod<'a, FP, tags::SimpleImpl, GetterType = Option<GetterType>>,
{
    type GetterTypeImpl = GetterType;
    fn get_impl(&self) -> Self::GetterTypeImpl {
        let opt_value = <Self as GetOptFieldMethod<FP, tags::SimpleImpl>>::get_opt(self);
        opt_value.unwrap_or(FP::DEFAULT_VALUE)
    }
}

// [optional|required|(unlabeled)] (string|bytes) field
// assuming that getter_opt method is implemented, and the `FieldPropetry::DEFAULT_VALUE`
// type is the *subtype* of the getter's return type.
// (e.g. `&'static str` for the DEFAULT_VALUE, and `&'a str` for the getter's return type.)
// Because of this difference, I needed to split the impl code with other numerical types.
// Maybe theer's a good solution for this, but I'm not sure...

// i.e. `str` or `[u8]`
type Borrowed<_2> = <tags::StringOrBytesType<_2> as tags::StringOrBytesTypeTag>::BorrowedType;

impl<'a, 'b: 'a, _1, _2, FP, FieldType, Shared>
    GetFieldMethodImpl2<
        'a,
        FP,
        tags::SimpleImpl,
        tags::NonRepeatedLabel<_1>,
        tags::StringOrBytesType<_2>,
    > for FieldAndSharedRef<'a, FieldType, Shared>
where
    FP: FieldProperties<
        LabelTag = tags::NonRepeatedLabel<_1>,
        TypeTag = tags::StringOrBytesType<_2>,
    >,
    tags::StringOrBytesType<_2>:
        tags::FieldTypeTag<DefaultValueType = &'b Borrowed<_2>> + tags::StringOrBytesTypeTag,
    Borrowed<_2>: 'a + 'b,
    Self: GetOptFieldMethod<'a, FP, tags::SimpleImpl, GetterType = Option<&'a Borrowed<_2>>>,
{
    type GetterTypeImpl = &'a Borrowed<_2>;
    fn get_impl(&self) -> Self::GetterTypeImpl {
        let opt_value = <Self as GetOptFieldMethod<FP, tags::SimpleImpl>>::get_opt(self);
        opt_value.unwrap_or(FP::DEFAULT_VALUE)
    }
}

// [optional|required|(unlabeled)] message field
// assuming that getter_opt method is implemented.
impl<'a, _1, FP, MP, FieldType, Shared>
    GetFieldMethodImpl2<'a, FP, tags::SimpleImpl, tags::NonRepeatedLabel<_1>, tags::Message<MP>>
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
    GetFieldMethodImpl2<'a, FP, tags::SimpleImpl, tags::Repeated, TypeTag>
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
