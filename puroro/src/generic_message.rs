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

use crate::internal as int;
use crate::internal::{FieldType, SharedItems, SharedItemsImpl};
use crate::typenum::{Bool, Cmp, Comparable};
use crate::Message;
use ::typenum::U1;

trait GenericMessage {
    type FieldType<'a, N: 'a + Comparable>: GenericField
    where
        Self: 'a;
}
trait GenericField {}
trait FieldsTrait {
    type Type<N: Comparable>: FieldType;
}

impl GenericMessage for () {
    type FieldType<'a, N: 'a + Comparable> = () where Self: 'a;
}
impl GenericField for () {}

#[derive(Default)]
struct PersonMessage {
    fields: PersonMessageFields,
    shared: SharedItemsImpl<1>,
}
#[derive(Default)]
struct PersonMessageFields {
    partner: int::SingularHeapMessageField<PersonMessage>,
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
impl FieldsTrait for PersonMessageFields {
    type Type<N: Comparable> =
        <Cmp<U1, N> as Bool>::IfF<int::SingularHeapMessageField<PersonMessage>, ()>;
}

struct FieldRef<'a, F, S>(&'a F, &'a S);
impl<'a, F, S> GenericField for FieldRef<'a, F, S>
where
    F: FieldType,
    S: SharedItems,
{
}

impl GenericMessage for PersonMessage {
    type FieldType<'a, N: 'a + Comparable> =
        FieldRef<'a, <PersonMessageFields as FieldsTrait>::Type<N>, SharedItemsImpl<1>> where Self: 'a;
}
