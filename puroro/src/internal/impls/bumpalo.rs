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

//! # Bumpalo message structs
//!
//! **The implementation is highly experimental and the interface will change
//! in very soon!!**
//!

pub mod de;

use crate::bumpalo::collections::{String, Vec};
use crate::bumpalo::Bump;
use ::std::borrow::Borrow;
use ::std::mem;
use ::std::mem::ManuallyDrop;
use ::std::ops::{Deref, DerefMut, Index};
use ::std::ptr;
use ::std::ptr::NonNull;
use ::std::slice;
use ::std::str::Utf8Error;

pub trait BumpDefault<'bump> {
    fn default_in(bump: &'bump Bump) -> Self;
}
impl<'bump> BumpDefault<'bump> for NoAllocString {
    fn default_in(bump: &'bump Bump) -> Self {
        NoAllocString::new_in(bump)
    }
}
impl<'bump, T> BumpDefault<'bump> for NoAllocVec<T> {
    fn default_in(bump: &'bump Bump) -> Self {
        NoAllocVec::new_in(bump)
    }
}
impl<'bump, T> BumpDefault<'bump> for Option<T> {
    fn default_in(_: &'bump Bump) -> Self {
        ::std::default::Default::default()
    }
}
macro_rules! impl_bumpalo_default {
    ($ty:ty) => {
        impl<'bump> BumpDefault<'bump> for $ty {
            fn default_in(_: &'bump Bump) -> Self {
                Default::default()
            }
        }
    };
}
impl_bumpalo_default!(i32);
impl_bumpalo_default!(u32);
impl_bumpalo_default!(f32);
impl_bumpalo_default!(i64);
impl_bumpalo_default!(u64);
impl_bumpalo_default!(f64);
impl_bumpalo_default!(bool);

/// A box for proto message internal usage.
/// DO NOT USE THIS TYPE IN NORMAL PLACES, IT'S NOT SAFE!
///
/// Unlike other [`NoAllocString`] and [`NoAllocVec`] types, this type is
/// just a slightly modified version of [`bumpalo::boxed::Box`] where
/// replaced it's internal data type from `&'bump mut T` to [`NonNull<T>`].
/// The main purpose of this type is to make sure the `Drop` is called
/// when the box itself is dropped, and of course as a box storing a value
/// into heap memory.
pub struct NoAllocBox<T>(NonNull<T>);
impl<T> NoAllocBox<T> {
    pub fn new_in(x: T, bump: &Bump) -> Self {
        Self(unsafe { NonNull::new_unchecked(bump.alloc(x)) })
    }
}
impl<T> Deref for NoAllocBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T> DerefMut for NoAllocBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl<T> Borrow<T> for NoAllocBox<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}
impl<T> AsRef<T> for NoAllocBox<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}
impl<T> Drop for NoAllocBox<T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.0.as_ptr()) }
    }
}

pub trait AddBump {
    type AddToRef<'bump, 'this>
    where
        Self: 'bump + 'this;
    fn add_bump<'bump, 'this>(&'this self, bump: &'bump Bump) -> Self::AddToRef<'bump, 'this>;
    type AddToMutRef<'bump, 'this>
    where
        Self: 'bump + 'this;
    fn add_bump_mut<'bump, 'this>(
        &'this mut self,
        bump: &'bump Bump,
    ) -> Self::AddToMutRef<'bump, 'this>;
}

pub trait RemoveBump {
    type Removed;
    fn remove_bump(self) -> Self::Removed;
}

/// A vec for proto message internal usage.
/// DO NOT USE THIS TYPE IN NORMAL PLACES, IT'S NOT SAFE!
///
/// This type is, essentially, [`bumpalo::collections::Vec`] minus `bump: &Bump` field.
/// In our usage, the bumpalo pointer is always stored in the message structure,
/// so each vec or string in the message does not need to store the pointer to bump.
/// This will make the message struct's self-referential paradox easier in future development.
/// Instead, we need to make sure for every mutation operation pass the correct
/// bump pointer and construct a [`bumpalo::collections::Vec`], and when the operation
/// is done we need to write back the modification to the [`NoAllocVec`] type.
pub struct NoAllocVec<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize,
}
impl<T> NoAllocVec<T> {
    pub fn new() -> Self {
        // Actually bump ptr is not needed when allocating.
        // Maybe we somehow make a dangling bump ptr here to skip
        // Bump::new() invocation...
        let fake_bump = Bump::new();
        let vec = Vec::new_in(&fake_bump);
        unsafe { Self::from_vec(vec) }
    }
    pub fn new_in(bump: &Bump) -> Self {
        let vec = Vec::new_in(bump);
        unsafe { Self::from_vec(vec) }
    }
    pub fn from_vec<'bump>(mut vec: Vec<'bump, T>) -> Self {
        let result = Self {
            ptr: vec.as_mut_ptr(),
            length: vec.len(),
            capacity: vec.capacity(),
        };
        mem::forget(vec);
        result
    }

    /// Returns a unsafely casted vector. Super duper unsafe!
    /// TODO: Maybe add little more restriction for safety.
    /// e.g. Require the item to implement some sort of marker trait.
    pub unsafe fn cast_item_unchecked<U>(&self) -> &NoAllocVec<U> {
        mem::transmute(self)
    }

    /// Construct an immutable [`Vec`](bumpalo::collections::Vec) by adding bump ptr.
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    /// Also, if the `ManuallyDrop` inner value has been taken then it's also unsafe.
    unsafe fn as_vec_in<'bump>(&self, bump: &'bump Bump) -> ManuallyDrop<Vec<'bump, T>> {
        ManuallyDrop::new(Vec::from_raw_parts_in(
            self.ptr,
            self.length,
            self.capacity,
            bump,
        ))
    }

    /// Construct a mutable [`Vec`](bumpalo::collections::Vec) wrapped by [`MutRefVec`].
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn as_mut_vec_in<'bump>(&mut self, bump: &'bump Bump) -> RefMutVec<'bump, '_, T> {
        RefMutVec {
            temp_vec: self.as_vec_in(bump),
            ref_vec: self,
        }
    }
}
impl<T> AddBump for NoAllocVec<T> {
    type AddToRef<'bump, 'this>
    where
        Self: 'bump + 'this,
    = ManuallyDrop<Vec<'bump, T>>;
    fn add_bump<'bump, 'this>(&'this self, bump: &'bump Bump) -> Self::AddToRef<'bump, 'this> {
        unsafe { self.as_vec_in(bump) }
    }
    type AddToMutRef<'bump, 'this>
    where
        Self: 'bump + 'this,
    = RefMutVec<'bump, 'this, T>;
    fn add_bump_mut<'bump, 'this>(
        &'this mut self,
        bump: &'bump Bump,
    ) -> Self::AddToMutRef<'bump, 'this> {
        unsafe { self.as_mut_vec_in(bump) }
    }
}
impl<'bump, T> RemoveBump for Vec<'bump, T> {
    type Removed = NoAllocVec<T>;
    fn remove_bump(self) -> Self::Removed {
        NoAllocVec::from_vec(self)
    }
}
impl<T> Deref for NoAllocVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::slice::from_raw_parts(self.ptr, self.length) }
    }
}
impl<T> DerefMut for NoAllocVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { ::std::slice::from_raw_parts_mut(self.ptr, self.length) }
    }
}
impl<T> Borrow<[T]> for NoAllocVec<T> {
    fn borrow(&self) -> &[T] {
        self.deref()
    }
}
impl<T> AsRef<[T]> for NoAllocVec<T> {
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}
impl<T> Drop for NoAllocVec<T> {
    fn drop(&mut self) {
        // bumpalo's Vec does not drop items so manually dropping it
        // https://github.com/fitzgen/bumpalo/issues/133
        unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, self.length)) }
    }
}
impl<T> Default for NoAllocVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a, T> IntoIterator for &'a NoAllocVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.deref().into_iter()
    }
}

pub struct RefMutVec<'bump, 'vec, T> {
    temp_vec: ManuallyDrop<Vec<'bump, T>>,
    ref_vec: &'vec mut NoAllocVec<T>,
}
impl<'bump, 'vec, T: Default> RefMutVec<'bump, 'vec, T> {
    pub fn push_default(&mut self) -> &mut T {
        self.temp_vec.push(T::default());
        self.last_mut().unwrap()
    }
}

impl<'bump, 'vec, T: 'bump> Deref for RefMutVec<'bump, 'vec, T> {
    type Target = Vec<'bump, T>;
    fn deref(&self) -> &Self::Target {
        &self.temp_vec
    }
}
impl<'bump, 'vec, T: 'bump> DerefMut for RefMutVec<'bump, 'vec, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.temp_vec
    }
}
impl<'bump, 'vec, T> Drop for RefMutVec<'bump, 'vec, T> {
    fn drop(&mut self) {
        // We can drop without a bump ptr, though we cannot get benefit of
        // bump memory reusing when the deallocated memory block is the last block
        // allocated by the bump instance.
        // It's not much a deal if we are creating a complex data structure like
        // protobuf I guess though...
        unsafe {
            *self.ref_vec = NoAllocVec::from_vec(ManuallyDrop::take(&mut self.temp_vec));
        }
    }
}

/// A string for proto message internal usage.
/// DO NOT USE THIS TYPE IN NORMAL PLACES, IT'S NOT SAFE!
///
/// This type is, essentially, [`bumpalo::collections::String`] minus `bump: &Bump` field.
/// In our usage, the bumpalo pointer is always stored in the message structure,
/// so each vec or string in the message does not need to store the pointer to bump.
/// This will make the message struct's self-referential paradox easier in future development.
/// Instead, we need to make sure for every mutation operation pass the correct
/// bump pointer and construct a [`bumpalo::collections::String`], and when the operation
/// is done we need to write back the modification to the [`NoAllocString`] type.
pub struct NoAllocString {
    vec: NoAllocVec<u8>,
}
impl NoAllocString {
    pub fn new() -> Self {
        Self {
            vec: NoAllocVec::new(),
        }
    }

    pub fn new_in(bump: &Bump) -> Self {
        Self {
            vec: NoAllocVec::new_in(bump),
        }
    }

    pub fn from_utf8(vec: NoAllocVec<u8>) -> ::std::result::Result<Self, Utf8Error> {
        if let Err(error) = ::std::str::from_utf8(&vec) {
            Err(error)
        } else {
            Ok(Self { vec })
        }
    }

    pub fn from_utf8_unchecked(vec: NoAllocVec<u8>) -> Self {
        Self { vec }
    }

    /// Construct an immutable [`String`](bumpalo::collections::String) by adding bump ptr.
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    /// Also, if the `ManuallyDrop` inner value has been taken then it's also unsafe.
    unsafe fn as_string_in<'bump>(&self, bump: &'bump Bump) -> ManuallyDrop<String<'bump>> {
        ManuallyDrop::new(String::from_utf8_unchecked(ManuallyDrop::into_inner(
            self.vec.as_vec_in(bump),
        )))
    }

    /// Construct a mutable [`String`](bumpalo::collections::String) by adding bump ptr.
    /// This function must take a same bump ref with the one given in `new_in` method.
    ///
    /// # Safety
    /// This function is unsafe because there are no guarantee that the
    /// given `bump` is the same instance with the one given at construction time.
    pub unsafe fn as_mut_string_in<'bump, 'string>(
        &'string mut self,
        bump: &'bump Bump,
    ) -> RefMutString<'bump, 'string> {
        RefMutString {
            temp_string: self.as_string_in(bump),
            ref_string: self,
        }
    }
}
impl AddBump for NoAllocString {
    type AddToRef<'bump, 'this>
    where
        Self: 'bump + 'this,
    = ManuallyDrop<String<'bump>>;
    fn add_bump<'bump, 'this>(&'this self, bump: &'bump Bump) -> Self::AddToRef<'bump, 'this> {
        unsafe { self.as_string_in(bump) }
    }
    type AddToMutRef<'bump, 'this>
    where
        Self: 'bump + 'this,
    = RefMutString<'bump, 'this>;
    fn add_bump_mut<'bump, 'this>(
        &'this mut self,
        bump: &'bump Bump,
    ) -> Self::AddToMutRef<'bump, 'this> {
        unsafe { self.as_mut_string_in(bump) }
    }
}
impl<'bump> RemoveBump for String<'bump> {
    type Removed = NoAllocString;
    fn remove_bump(self) -> Self::Removed {
        NoAllocString::from_utf8_unchecked(NoAllocVec::from_vec(self.into_bytes()))
    }
}

impl Deref for NoAllocString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { ::std::str::from_utf8_unchecked(&self.vec) }
    }
}
impl Borrow<str> for NoAllocString {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
impl AsRef<str> for NoAllocString {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}
impl Default for NoAllocString {
    fn default() -> Self {
        Self {
            vec: Default::default(),
        }
    }
}

pub struct RefMutString<'bump, 'string> {
    temp_string: ManuallyDrop<String<'bump>>,
    ref_string: &'string mut NoAllocString,
}

impl<'bump, 'string> Deref for RefMutString<'bump, 'string> {
    type Target = String<'bump>;
    fn deref(&self) -> &Self::Target {
        &self.temp_string
    }
}
impl<'bump, 'string> DerefMut for RefMutString<'bump, 'string> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.temp_string
    }
}
impl<'bump, 'string> Drop for RefMutString<'bump, 'string> {
    fn drop(&mut self) {
        unsafe {
            let vec = ManuallyDrop::take(&mut self.temp_string).into_bytes();
            self.ref_string.vec = NoAllocVec::from_vec(vec);
        }
    }
}

pub struct AddBumpVecView<'bump, 'vec, T> {
    vec: &'vec mut Vec<'bump, T>,
    bump: &'bump Bump,
}
impl<'bump, 'vec, T> AddBumpVecView<'bump, 'vec, T> {
    fn new(vec: &'vec mut Vec<'bump, T>, bump: &'bump Bump) -> Self {
        Self { vec, bump }
    }

    pub fn push<U>(&mut self, val: U)
    where
        U: RemoveBump<Removed = T>,
    {
        self.vec.push(<U as RemoveBump>::remove_bump(val));
    }
}
impl<'bump, 'vec, T> AddBumpVecView<'bump, 'vec, T>
where
    T: AddBump,
{
    pub fn get(&self, index: usize) -> Option<<T as AddBump>::AddToRef<'bump, '_>> {
        self.vec
            .get(index)
            .map(|v| <T as AddBump>::add_bump(v, self.bump))
    }
    pub fn get_mut(&mut self, index: usize) -> Option<<T as AddBump>::AddToMutRef<'bump, '_>> {
        self.vec
            .get_mut(index)
            .map(|v| <T as AddBump>::add_bump_mut(v, self.bump))
    }
}
impl<'bump, 'vec, T> AddBumpVecView<'bump, 'vec, T>
where
    T: AddBump + Default,
{
    pub fn push_default(&mut self) -> <T as AddBump>::AddToMutRef<'bump, '_> {
        self.vec.push(Default::default());
        <T as AddBump>::add_bump_mut(self.vec.last_mut().unwrap(), self.bump)
    }
}
