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

use crate::bumpalo::boxed::Box as BBox;
use crate::bumpalo::collections::{String as BString, Vec as BVec};
use crate::bumpalo::Bump;
use crate::internal::NoAllocBumpBox;
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
impl<T> DefaultIn for Box<T>
where
    T: DefaultIn,
{
    type AllocatorType = T::AllocatorType;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        Box::new(T::default_in(alloc))
    }
}
impl<'a, T> DefaultIn for NoAllocBumpBox<T>
where
    T: DefaultIn<AllocatorType = &'a Bump>,
{
    type AllocatorType = &'a Bump;
    fn default_in(bump: Self::AllocatorType) -> Self {
        NoAllocBumpBox::new_in(T::default_in(bump), bump)
    }
}
impl<'bump, T> DefaultIn for BVec<'bump, T> {
    type AllocatorType = &'bump Bump;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        BVec::new_in(alloc)
    }
}
impl<'bump> DefaultIn for BString<'bump> {
    type AllocatorType = &'bump Bump;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        BString::new_in(alloc)
    }
}
impl<'bump, T> DefaultIn for BBox<'bump, T>
where
    T: DefaultIn<AllocatorType = &'bump Bump>,
{
    type AllocatorType = &'bump Bump;
    fn default_in(alloc: Self::AllocatorType) -> Self {
        BBox::new_in(DefaultIn::default_in(alloc), alloc)
    }
}

pub trait AsMessageImplRef {
    type MessageImplType;
    fn as_message_impl_ref(&self) -> &Self::MessageImplType;
}
macro_rules! impl_as_message_impl_ref {
    (impl<$($lt:lifetime, )* T> $ty:ty) => {
        impl<$($lt, )* T> AsMessageImplRef for $ty
        where
            T: AsMessageImplRef,
        {
            type MessageImplType = T::MessageImplType;
            fn as_message_impl_ref(&self) -> &Self::MessageImplType {
                <T as AsMessageImplRef>::as_message_impl_ref(self)
            }
        }
    };
}
impl_as_message_impl_ref!(impl<'a, T> &'a T);
impl_as_message_impl_ref!(impl<'a, T> &'a mut T);
impl_as_message_impl_ref!(impl<T> Box<T>);
impl_as_message_impl_ref!(impl<T> crate::internal::NoAllocBumpBox<T>);

pub trait AsMessageImplMut: AsMessageImplRef {
    fn as_message_impl_mut(&mut self) -> &mut <Self as AsMessageImplRef>::MessageImplType;
}
macro_rules! impl_as_message_impl_mut {
    (impl<$($lt:lifetime, )* T> $ty:ty) => {
        impl<$($lt, )* T> AsMessageImplMut for $ty
        where
            T: AsMessageImplMut,
        {
            fn as_message_impl_mut(&mut self) -> &mut Self::MessageImplType {
                <T as AsMessageImplMut>::as_message_impl_mut(self)
            }
        }
    };
}
impl_as_message_impl_mut!(impl<'a, T> &'a mut T);
impl_as_message_impl_mut!(impl<T> Box<T>);
impl_as_message_impl_mut!(impl<T> crate::internal::NoAllocBumpBox<T>);

pub trait AsMessageRef {
    type MessageType;
    fn as_message_ref(&self) -> &Self::MessageType;
}
macro_rules! impl_as_message_ref {
    (impl<$($lt:lifetime, )* T> $ty:ty) => {
        impl<$($lt, )* T> AsMessageRef for $ty
        where
            T: AsMessageRef,
        {
            type MessageType = T::MessageType;
            fn as_message_ref(&self) -> &Self::MessageType {
                <T as AsMessageRef>::as_message_ref(self)
            }
        }
    };
}
impl_as_message_ref!(impl<'a, T> &'a T);
impl_as_message_ref!(impl<'a, T> &'a mut T);
impl_as_message_ref!(impl<T> Box<T>);
impl_as_message_ref!(impl<T> crate::internal::NoAllocBumpBox<T>);

pub trait AsMessageMut: AsMessageRef {
    fn as_message_mut(&mut self) -> &mut <Self as AsMessageRef>::MessageType;
}
macro_rules! impl_as_message_mut {
    (impl<$($lt:lifetime, )* T> $ty:ty) => {
        impl<$($lt, )* T> AsMessageMut for $ty
        where
            T: AsMessageMut,
        {
            fn as_message_mut(&mut self) -> &mut Self::MessageType {
                <T as AsMessageMut>::as_message_mut(self)
            }
        }
    };
}
impl_as_message_mut!(impl<'a, T> &'a mut T);
impl_as_message_mut!(impl<T> Box<T>);
impl_as_message_mut!(impl<T> crate::internal::NoAllocBumpBox<T>);
