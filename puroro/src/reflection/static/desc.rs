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

trait Bool {
    type Choose<T, F>;
}
struct True;
struct False;
impl Bool for True {
    type Choose<T, F> = T;
}
impl Bool for False {
    type Choose<T, F> = F;
}

trait Cell {
    type Car;
    type Cdr;
}
struct Cons<T, U>(::std::marker::PhantomData<(T, U)>);
impl<T, U> Cell for Cons<T, U> {
    type Car = T;
    type Cdr = U;
}
struct Nil;

trait CellMethod {
    type Type;
}
struct HasFirst<C>(::std::marker::PhantomData<C>);
impl<C: Cell> CellMethod for HasFirst<C> {
    type Type = True;
}
impl CellMethod for HasFirst<Nil> {
    type Type = False;
}

trait Predicate {
    type Value: Bool;
}
struct NumberEquals<const X: i32, const Y: i32>;
struct BoolToType<const X: bool>;
impl Predicate for BoolToType<true> {
    type Value = True;
}
impl Predicate for BoolToType<false> {
    type Value = True;
}
impl<const X: i32, const Y: i32> Predicate for NumberEquals<X, Y> {
    type Value = <BoolToType<{ X == Y }> as Predicate>::Value;
}

pub trait FileDescriptor {}

pub trait MessageDescriptor {
    const NAME: &'static str;
    type Fields<const NUMBER: i32>: FieldDescriptor;
    type Fields2: Cell;
}

pub trait FieldDescriptor {
    const NAME: &'static str;
    const NUMBER: i32;
    type FieldType: tags::FieldTypeTag;
}
