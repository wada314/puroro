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

use ::std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct NumericalField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);

#[derive(Default, Clone)]
pub struct BytesField(Vec<u8>);

#[derive(Default, Clone)]
pub struct StringField(String);

#[derive(Default, Clone)]
pub struct HeapMessageField<M>(Box<M>);

pub trait OneofFieldType {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_>;
    type MutGetterType<'a>
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_>;
}

pub trait OneofFieldTypeOpt<'a> {
    type OptGetterType;
    type GetterType;
    type DefaultValueType;
    fn get_field_opt(self) -> Self::OptGetterType;
    fn get_field<D: FnOnce() -> Self::DefaultValueType>(self, default: D) -> Self::GetterType;
}

impl<RustType, ProtoType> OneofFieldType for NumericalField<RustType, ProtoType>
where
    RustType: Clone,
{
    type GetterType<'a> = RustType
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        self.0.clone()
    }
    type MutGetterType<'a> = &'a mut RustType
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }
}

impl OneofFieldType for BytesField {
    type GetterType<'a> = &'a [u8]
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        self.0.as_ref()
    }
    type MutGetterType<'a> = &'a mut Vec<u8>
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }
}

impl OneofFieldType for StringField {
    type GetterType<'a> = &'a str
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        self.0.as_str()
    }
    type MutGetterType<'a> = &'a mut String
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }
}

impl<M: Default> OneofFieldType for HeapMessageField<M> {
    type GetterType<'a> = &'a M
    where
        Self: 'a;
    fn get_field(&self) -> Self::GetterType<'_> {
        &self.0
    }
    type MutGetterType<'a> = &'a mut M
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        &mut self.0
    }
}

pub trait OneofFieldTypeOptForNonMessageType {
    type GetterType;
    fn get_field_opt(self) -> Option<Self::GetterType>;
}

impl<'a, T: OneofFieldTypeOptForNonMessageType> OneofFieldTypeOpt<'a> for T {
    type OptGetterType = Option<<T as OneofFieldTypeOptForNonMessageType>::GetterType>;
    type GetterType = <T as OneofFieldTypeOptForNonMessageType>::GetterType;
    type DefaultValueType = <T as OneofFieldTypeOptForNonMessageType>::GetterType;
    fn get_field_opt(self) -> Self::OptGetterType {
        <T as OneofFieldTypeOptForNonMessageType>::get_field_opt(self)
    }
    fn get_field<D: FnOnce() -> Self::DefaultValueType>(self, default: D) -> Self::GetterType {
        <T as OneofFieldTypeOptForNonMessageType>::get_field_opt(self).unwrap_or_else(default)
    }
}

impl<'a, RustType, ProtoType> OneofFieldTypeOptForNonMessageType
    for Option<&'a NumericalField<RustType, ProtoType>>
where
    RustType: Clone,
{
    type GetterType = RustType;
    fn get_field_opt(self) -> Option<Self::GetterType> {
        self.map(|f| f.get_field())
    }
}

impl<'a> OneofFieldTypeOptForNonMessageType for Option<&'a BytesField> {
    type GetterType = &'a [u8];
    fn get_field_opt(self) -> Option<Self::GetterType> {
        self.map(|f| f.get_field())
    }
}

impl<'a> OneofFieldTypeOptForNonMessageType for Option<&'a StringField> {
    type GetterType = &'a str;
    fn get_field_opt(self) -> Option<Self::GetterType> {
        self.map(|f| f.get_field())
    }
}

impl<'a, M: Default> OneofFieldTypeOpt<'a> for Option<&'a HeapMessageField<M>> {
    type OptGetterType = Option<&'a M>;
    type GetterType = Option<&'a M>;
    type DefaultValueType = ();
    fn get_field_opt(self) -> Self::OptGetterType {
        self.map(|f| f.get_field())
    }
    fn get_field<D: FnOnce() -> Self::DefaultValueType>(self, _default: D) -> Self::GetterType {
        self.get_field_opt()
    }
}
