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

#[derive(Default, Clone, PartialEq)]
pub struct Dummy;

#[derive(Default, Clone, PartialEq)]
pub struct NumericalField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);

#[derive(Default, Clone, PartialEq)]
pub struct BytesField(Vec<u8>);

#[derive(Default, Clone, PartialEq)]
pub struct StringField(String);

#[derive(Default, Clone, PartialEq)]
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

impl OneofFieldType for Dummy {
    type GetterType<'a> = ()
    where
        Self: 'a;

    fn get_field(&self) -> Self::GetterType<'_> {
        ()
    }
    type MutGetterType<'a> = ()
    where
        Self: 'a;
    fn mut_field(&mut self) -> Self::MutGetterType<'_> {
        ()
    }
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

impl<M> OneofFieldType for HeapMessageField<M> {
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
