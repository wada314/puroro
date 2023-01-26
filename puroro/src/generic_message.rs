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

use crate::internal::{FieldType, SharedItems};
use crate::typenum::Comparable;
use crate::{PuroroError, Result};
use ::std::iter;
use ::std::slice;

pub trait GenericMessage {
    type FieldType<'a, N: 'a + Comparable>: GenericField<'a>
    where
        Self: 'a;
    fn field<'a, N: 'a + Comparable>(&'a self) -> Self::FieldType<'a, N>;
}
pub trait GenericField<'a> {
    fn try_get_i32(&self) -> Result<Option<i32>> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
    fn try_get_string(&self) -> Result<Option<&str>> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
    type MessageType: GenericMessage;
    fn try_get_message(&self) -> Result<Option<Self::MessageType>> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
    type NumIteratorType<T: 'a + Clone>: Iterator<Item = T>;
    fn try_get_repeated_i32(&self) -> Result<Self::NumIteratorType<i32>> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
    type UnsizedIteratorType<T: 'a + ?Sized>: Iterator<Item = &'a T>;
    fn try_get_repeated_string(&self) -> Result<Self::UnsizedIteratorType<str>> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
    type MessageIteratorType: Iterator<Item = Self::MessageType>;
    fn try_get_repeated_message(&self) -> Result<Self::MessageIteratorType> {
        Err(PuroroError::UnavailableGenericFieldType)?
    }
}
pub trait FieldsTrait {
    type Type<N: Comparable>: FieldType;
    fn field<'a, N: 'a + Comparable>(&'a self) -> &'a Self::Type<N>;
}

impl GenericMessage for () {
    type FieldType<'a, N: 'a + Comparable> = () where Self: 'a;
    fn field<'a, N: 'a + Comparable>(&'a self) -> Self::FieldType<'a, N> {
        ()
    }
}
impl<'a, T: GenericMessage> GenericMessage for &'a T {
    type FieldType<'b, N: 'b + Comparable> = T::FieldType<'b, N>
    where
        Self: 'b;
    fn field<'b, N: 'b + Comparable>(&'b self) -> Self::FieldType<'b, N> {
        T::field::<N>(self)
    }
}
impl<'a, T: GenericMessage> GenericMessage for &'a mut T {
    type FieldType<'b, N: 'b + Comparable> = T::FieldType<'b, N>
    where
        Self: 'b;
    fn field<'b, N: 'b + Comparable>(&'b self) -> Self::FieldType<'b, N> {
        T::field::<N>(self)
    }
}

impl<'a> GenericField<'a> for () {
    type MessageType = ();
    type NumIteratorType<T: 'a + Clone> = iter::Empty<T>;
    type UnsizedIteratorType<T: 'a + ?Sized> = iter::Empty<&'a T>;
    type MessageIteratorType = iter::Empty<()>;
}

pub struct GenericFieldImpl<'a, F, S>(pub &'a F, pub &'a S);
impl<'a, F, S> GenericField<'a> for GenericFieldImpl<'a, F, S>
where
    F: FieldType,
    S: SharedItems,
{
    fn try_get_i32(&self) -> Result<Option<i32>> {
        FieldType::try_get_i32(self.0)
    }
    type MessageType = F::MessageType<'a>;
    fn try_get_message(&self) -> Result<Option<Self::MessageType>> {
        FieldType::try_get_message(self.0)
    }
    type NumIteratorType<T: 'a + Clone> = iter::Cloned<slice::Iter<'a, T>>;
    fn try_get_repeated_i32(&self) -> Result<Self::NumIteratorType<i32>> {
        FieldType::try_get_repeated_i32(self.0).map(|slice| slice.iter().cloned())
    }
    type UnsizedIteratorType<T: 'a + ?Sized> = F::RepeatedUnsizedType<'a, T>;
    fn try_get_repeated_string(&self) -> Result<Self::UnsizedIteratorType<str>> {
        FieldType::try_get_repeated_string(self.0)
    }
    type MessageIteratorType = F::RepeatedMessageType<'a>;
    fn try_get_repeated_message(&self) -> Result<Self::MessageIteratorType> {
        FieldType::try_get_repeated_message(self.0)
    }
}
