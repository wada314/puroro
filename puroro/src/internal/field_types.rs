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

pub struct SingularNumericField<RustType, ProtoType>(RustType, PhantomData<ProtoType>);
pub struct OptionalNumericField<RustType, ProtoType, const BITFIELD_INDEX: usize>(
    RustType,
    PhantomData<ProtoType>,
);

pub struct SingularStringField(String);
pub struct OptionalStringField<const BITFIELD_INDEX: usize>(String);

pub struct SingularHeapMessageField<M>(Option<Box<M>>);
