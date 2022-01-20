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

use crate::internal::FieldProperties;

pub trait GetFieldMethodImpl<'a, FP, ImplTag, LabelTag, TypeTag> {
    type GetterTypeImpl;
    fn get_impl(&self) -> Self::GetterTypeImpl;
}
pub trait GetFieldMethod<'a, FP, ImplTag> {
    type GetterType;
    fn get(&self) -> Self::GetterType;
}
impl<'a, FP, ImplTag, T> GetFieldMethod<'a, FP, ImplTag> for T
where
    FP: FieldProperties,
    T: GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>,
{
    type GetterType =
        <Self as GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::GetterTypeImpl;
    fn get(&self) -> Self::GetterType {
        <Self as GetFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::get_impl(self)
    }
}

pub trait GetOptFieldMethodImpl<'a, FP, ImplTag, LabelTag, TypeTag> {
    type GetterTypeImpl;
    fn get_opt_impl(&self) -> Self::GetterTypeImpl;
}
pub trait GetOptFieldMethod<'a, FP, ImplTag> {
    type GetterType;
    fn get_opt(&self) -> Self::GetterType;
}
impl<'a, FP, ImplTag, T> GetOptFieldMethod<'a, FP, ImplTag> for T
where
    FP: FieldProperties,
    T: GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>,
{
    type GetterType =
        <Self as GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::GetterTypeImpl;
    fn get_opt(&self) -> Self::GetterType {
        <Self as GetOptFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::get_opt_impl(
            &self,
        )
    }
}

pub trait GetMutFieldMethodImpl<'a, FP, ImplTag, LabelTag, TypeTag> {
    type GetterTypeImpl;
    fn get_mut_impl(self) -> Self::GetterTypeImpl;
}
pub trait GetMutFieldMethod<'a, FP, ImplTag> {
    type GetterType;
    fn get_mut(self) -> Self::GetterType;
}
impl<'a, FP, ImplTag, T> GetMutFieldMethod<'a, FP, ImplTag> for T
where
    FP: FieldProperties,
    T: GetMutFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>,
{
    type GetterType =
        <Self as GetMutFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::GetterTypeImpl;
    fn get_mut(self) -> Self::GetterType {
        <Self as GetMutFieldMethodImpl<'a, FP, ImplTag, FP::LabelTag, FP::TypeTag>>::get_mut_impl(
            self,
        )
    }
}
