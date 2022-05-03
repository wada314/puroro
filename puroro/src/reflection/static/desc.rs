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

use crate::tags;

pub trait MessageDescriptor {
    type Fields;
}

pub trait FieldDescriptor {
    const NUMBER: i32;
    type Number: I32Trait;
    type FieldType: tags::FieldTypeTag;
}

struct MD;
struct FD1;
struct FD2;
struct FD3;
impl MessageDescriptor for MD {
    type Fields = (FD1, (FD2, (FD3, ())));
}
impl FieldDescriptor for FD1 {
    const NUMBER: i32 = 1;
    type Number = I32<1>;
    type FieldType = tags::String;
}
impl FieldDescriptor for FD2 {
    const NUMBER: i32 = 2;
    type Number = I32<2>;
    type FieldType = tags::UInt32;
}
impl FieldDescriptor for FD3 {
    const NUMBER: i32 = 3;
    type Number = I32<3>;
    type FieldType = tags::Message<MD>;
}

struct I32<const VALUE: i32>;
trait I32Trait {
    const VALUE: i32;
}
impl<const VALUE: i32> I32Trait for I32<VALUE> {
    const VALUE: i32 = VALUE;
}

trait GetTypeFromTuple {
    type Type;
}
struct FindNumberFromTuple<T, N: I32Trait>(std::marker::PhantomData<(T, N)>);
impl<N: I32Trait> GetTypeFromTuple for FindNumberFromTuple<(), N> {
    type Type = ();
}
impl<T: FieldDescriptor, U> GetTypeFromTuple for FindNumberFromTuple<(T, U), T::Number> {
    type Type = T;
}

trait GetFieldTrait {
    type Type;
}
struct GetField<const NUMBER: i32>;
impl<const NUMBER: i32> GetFieldTrait for GetField<NUMBER> {
    type Type = Type;
}
