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
use crate::{PuroroError, Result};
use ::typenum::U1;

pub trait GenericMessage {
    type FieldType<'a, N: 'a + Comparable>: GenericField
    where
        Self: 'a;
}
pub trait GenericField {
    fn try_get_i32(&self) -> Result<Option<i32>>;
    type MessageType<'a>: GenericMessage
    where
        Self: 'a;
    fn try_get_message(&self) -> Result<Option<Self::MessageType<'_>>>;
}
pub trait FieldsTrait {
    type Type<N: Comparable>: FieldType;
}

impl GenericMessage for () {
    type FieldType<'a, N: 'a + Comparable> = () where Self: 'a;
}
impl<'a, T: GenericMessage> GenericMessage for &'a T {
    type FieldType<'b, N: 'b + Comparable> = T::FieldType<'b, N>
    where
        Self: 'b;
}
impl<'a, T: GenericMessage> GenericMessage for &'a mut T {
    type FieldType<'b, N: 'b + Comparable> = T::FieldType<'b, N>
    where
        Self: 'b;
}

impl GenericField for () {
    fn try_get_i32(&self) -> Result<Option<i32>> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
    type MessageType<'a> = ();
    fn try_get_message(&self) -> Result<Option<Self::MessageType<'_>>> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
}

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

struct GenericFieldImpl<'a, F, S>(&'a F, &'a S);
impl<'a, F, S> GenericField for GenericFieldImpl<'a, F, S>
where
    F: FieldType,
    S: SharedItems,
{
    fn try_get_i32(&self) -> Result<Option<i32>> {
        FieldType::try_get_i32(self.0)
    }
    type MessageType<'b> = F::MessageType<'b> where Self: 'b;
    fn try_get_message(&self) -> Result<Option<Self::MessageType<'_>>> {
        FieldType::try_get_message(self.0)
    }
}

impl GenericMessage for PersonMessage {
    type FieldType<'a, N: 'a + Comparable> =
        GenericFieldImpl<'a, <PersonMessageFields as FieldsTrait>::Type<N>, SharedItemsImpl<1>> where Self: 'a;
}
