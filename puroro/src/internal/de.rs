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

pub mod from_iter;

use self::from_iter::ScopedIter;
use crate::internal::types::FieldData;
use crate::{EmptyMessageWrapper, Merged, Result};
use ::std::ops::DerefMut;

pub trait DeserMessageFromBytesIter {
    fn deser_field<I>(
        &mut self,
        field_number: i32,
        data: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>;
}
macro_rules! impl_deser_field {
    ($lambda:expr) => {
        fn deser_field<I>(
            &mut self,
            field_number: i32,
            data: FieldData<&mut ScopedIter<I>>,
        ) -> Result<()>
        where
            I: Iterator<Item = std::io::Result<u8>>,
        {
            ($lambda)(self).deser_field(field_number, data)
        }
    };
}
impl<T> DeserMessageFromBytesIter for &'_ mut T
where
    T: DeserMessageFromBytesIter,
{
    impl_deser_field!(DerefMut::deref_mut);
}
impl<T> DeserMessageFromBytesIter for Box<T>
where
    T: DeserMessageFromBytesIter,
{
    impl_deser_field!(DerefMut::deref_mut);
}
impl<'bump, T> DeserMessageFromBytesIter for crate::bumpalo::boxed::Box<'bump, T>
where
    T: DeserMessageFromBytesIter,
{
    impl_deser_field!(DerefMut::deref_mut);
}
impl<T> DeserMessageFromBytesIter for Option<T>
where
    T: DeserMessageFromBytesIter + Default,
{
    impl_deser_field!(|x| Option::get_or_insert_with(x, Default::default));
}
impl<T> DeserMessageFromBytesIter for Merged<T, ()>
where
    T: DeserMessageFromBytesIter,
{
    impl_deser_field!(Merged::first_mut);
}
impl<T> DeserMessageFromBytesIter for Merged<(), T>
where
    T: DeserMessageFromBytesIter,
{
    impl_deser_field!(Merged::last_mut);
}
impl<T, U> DeserMessageFromBytesIter for Merged<T, EmptyMessageWrapper<U>>
where
    T: DeserMessageFromBytesIter,
{
    impl_deser_field!(Merged::first_mut);
}
impl<T, U> DeserMessageFromBytesIter for Merged<EmptyMessageWrapper<T>, U>
where
    U: DeserMessageFromBytesIter,
{
    impl_deser_field!(Merged::last_mut);
}
