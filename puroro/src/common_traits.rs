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

use ::std::convert::TryFrom;

pub trait Enum2:
    'static + PartialEq + Clone + Default + TryFrom<i32, Error = i32> + Into<i32>
{
}
pub trait Enum3: 'static + PartialEq + Clone + Default + From<i32> + Into<i32> {}

pub trait DefaultIn {
    type AllocatorType;
    fn default_in(alloc: Self::AllocatorType) -> Self;
}

pub trait AsMessageRef {
    type MessageType;
    fn as_message_ref(&self) -> &Self::MessageType;
}
impl<'a, T> AsMessageRef for &'a T
where
    T: AsMessageRef,
{
    type MessageType = T::MessageType;
    fn as_message_ref(&self) -> &Self::MessageType {
        <T as AsMessageRef>::as_message_ref(*self)
    }
}
