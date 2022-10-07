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
use ::std::mem::ManuallyDrop;

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
    fn clear(&mut self);
}

pub trait OneofFieldTypeOpt {
    type OptGetterType<'a>
    where
        Self: 'a;
    type GetterType<'a>
    where
        Self: 'a;
    type DefaultValueType<'a>
    where
        Self: 'a;
    fn get_field_opt(&self) -> Self::OptGetterType<'_>;
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        default: D,
    ) -> Self::GetterType<'_>;
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
    fn clear(&mut self) {}
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
    fn clear(&mut self) {
        self.0.clear()
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
    fn clear(&mut self) {
        self.0.clear()
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
    fn clear(&mut self) {
        *self.0 = M::default();
    }
}

pub trait OneofFieldTypeOptForNonMessageType {
    type GetterType<'a>
    where
        Self: 'a;
    fn get_field_opt(&self) -> Option<Self::GetterType<'_>>;
}

impl<T: OneofFieldTypeOptForNonMessageType> OneofFieldTypeOpt for T {
    type OptGetterType<'a> = Option<<T as OneofFieldTypeOptForNonMessageType>::GetterType<'a>>
    where
        Self: 'a;
    type GetterType<'a> = <T as OneofFieldTypeOptForNonMessageType>::GetterType<'a>
    where
        Self: 'a;
    type DefaultValueType<'a> = <T as OneofFieldTypeOptForNonMessageType>::GetterType<'a>
    where
        Self: 'a;
    fn get_field_opt(&self) -> Self::OptGetterType<'_> {
        <T as OneofFieldTypeOptForNonMessageType>::get_field_opt(self)
    }
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        default: D,
    ) -> Self::GetterType<'_> {
        <T as OneofFieldTypeOptForNonMessageType>::get_field_opt(self).unwrap_or_else(default)
    }
}

impl<RustType, ProtoType> OneofFieldTypeOptForNonMessageType
    for Option<&NumericalField<RustType, ProtoType>>
where
    RustType: Clone,
{
    type GetterType<'a> = RustType
    where
        Self: 'a;
    fn get_field_opt(&self) -> Option<Self::GetterType<'_>> {
        self.map(|f| f.get_field())
    }
}

impl OneofFieldTypeOptForNonMessageType for Option<&BytesField> {
    type GetterType<'a> = &'a [u8]
    where
        Self: 'a;
    fn get_field_opt(&self) -> Option<Self::GetterType<'_>> {
        self.map(|f| f.get_field())
    }
}

impl OneofFieldTypeOptForNonMessageType for Option<&StringField> {
    type GetterType<'a> = &'a str
    where
        Self: 'a;
    fn get_field_opt(&self) -> Option<Self::GetterType<'_>> {
        self.map(|f| f.get_field())
    }
}

impl<M: Default> OneofFieldTypeOpt for Option<&HeapMessageField<M>> {
    type OptGetterType<'a> = Option<&'a M>
    where
        Self: 'a;
    type GetterType<'a> = Option<&'a M>
    where
        Self: 'a;
    type DefaultValueType<'a> = ()
    where
        Self: 'a;
    fn get_field_opt(&self) -> Self::OptGetterType<'_> {
        self.map(|f| f.get_field())
    }
    fn get_field<'a, D: FnOnce() -> Self::DefaultValueType<'a>>(
        &'a self,
        _default: D,
    ) -> Self::GetterType<'_> {
        self.get_field_opt()
    }
}
