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

use crate::generic_message::{FieldsTrait, GenericField, GenericFieldImpl, GenericMessage};
use crate::internal as int;
use crate::internal::SharedItemsImpl;
use crate::typenum::{Bool, Comparable};
use ::typenum::{U1, U2, U3, U4};

#[repr(transparent)]
pub struct Person<T>(T);

#[derive(Default)]
struct PersonMessage {
    fields: PersonMessageFields,
    shared: SharedItemsImpl<1>,
}
#[derive(Default)]
struct PersonMessageFields {
    partner: int::SingularHeapMessageField<PersonMessage>,
    age: int::SingularNumericalField<i32, int::tags::Int32>,
}

impl<T: GenericMessage> Person<T> {
    pub fn partner(&self) -> Option<Person<impl '_ + GenericMessage>> {
        self.0
            .field::<U1>()
            .try_get_message()
            .unwrap()
            .map(|m| Person(m))
    }
    pub fn age(&self) -> i32 {
        self.0
            .field::<U2>()
            .try_get_i32()
            .unwrap()
            .unwrap_or_default()
    }
    pub fn children(&self) -> impl '_ + Iterator<Item = Person<impl '_ + GenericMessage>> {
        self.0
            .field::<U3>()
            .try_get_repeated_message()
            .unwrap()
            .map(|m| Person(m))
    }
    pub fn ipv4_address(&self) -> impl '_ + Iterator<Item = i32> {
        self.0.field::<U4>().try_get_repeated_i32().unwrap()
    }
}

impl crate::Message for PersonMessage {
    fn from_bytes_iter<I: Iterator<Item = std::io::Result<u8>>>(iter: I) -> crate::Result<Self> {
        todo!()
    }
    fn merge_from_bytes_iter<I: Iterator<Item = std::io::Result<u8>>>(
        &mut self,
        iter: I,
    ) -> crate::Result<()> {
        todo!()
    }
    fn to_bytes<W: std::io::Write>(&self, out: &mut W) -> crate::Result<()> {
        todo!()
    }
}
impl GenericMessage for PersonMessage {
    type FieldType<'a, N: 'a + Comparable> =
        GenericFieldImpl<'a, <PersonMessageFields as FieldsTrait>::Type<N>, SharedItemsImpl<1>> where Self: 'a;
    fn field<'a, N: 'a + Comparable>(&'a self) -> Self::FieldType<'a, N> {
        GenericFieldImpl(self.fields.field::<N>(), &self.shared)
    }
}
impl FieldsTrait for PersonMessageFields {
    type Type<N: Comparable> = <N::Eq<U1> as Bool>::IfF<
        int::SingularHeapMessageField<PersonMessage>,
        <N::Eq<U2> as Bool>::IfF<int::SingularNumericalField<i32, int::tags::Int32>, ()>,
    >;
    fn field<'a, N: 'a + Comparable>(&'a self) -> &'a Self::Type<N> {
        <N::Eq<U1> as Bool>::if_f(&self.partner, <N::Eq<U2> as Bool>::if_f(&self.age, &()))
    }
}
