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

mod base_trait;
mod non_repeated;
mod repeated;

pub use base_trait::FieldType;
pub use non_repeated::NonRepeatedFieldType;
pub use repeated::RepeatedFieldType;

use ::std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct SingularNumericalField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);
#[derive(Default, Clone)]
pub struct OptionalNumericalField<RustType, ProtoType, const BITFIELD_INDEX: usize>(
    RustType,
    PhantomData<ProtoType>,
);
#[derive(Default, Clone)]
pub struct RepeatedNumericalField<RustType, ProtoType>(Vec<RustType>, PhantomData<ProtoType>);

#[derive(Default, Clone)]
pub struct SingularStringField(String);
#[derive(Default, Clone)]
pub struct OptionalStringField<const BITFIELD_INDEX: usize>(String);
#[derive(Default, Clone)]
pub struct RepeatedStringField(Vec<String>);

#[derive(Default, Clone)]
pub struct SingularBytesField(Vec<u8>);
#[derive(Default, Clone)]
pub struct OptionalBytesField<const BITFIELD_INDEX: usize>(Vec<u8>);
#[derive(Default, Clone)]
pub struct RepeatedBytesField(Vec<Vec<u8>>);

#[derive(Default, Clone)]
pub struct SingularHeapMessageField<M>(Option<Box<M>>);
#[derive(Default, Clone)]
pub struct RepeatedMessageField<M>(Vec<M>);
