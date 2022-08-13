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

use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::Result;
use ::std::borrow::Borrow;
use ::std::io::Result as IoResult;
use ::std::marker::PhantomData;

pub fn wire_and_number_from_bytes_iter<I: Iterator<Item = IoResult<u8>>>(
    mut iter: I,
) -> Result<(WireType, i32)> {
    let var_i32 = Variant::decode_bytes(&mut iter)?.get_i32()?;
    Ok(((var_i32 & 0x7).try_into()?, (var_i32 >> 3)))
}

pub struct BorrowedIter<B: ?Sized, I>(I, PhantomData<B>);
impl<B: ?Sized, I> BorrowedIter<B, I> {
    pub fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}
impl<'a, B, I, T> Iterator for BorrowedIter<B, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a + Borrow<B>,
    B: 'a + ?Sized,
{
    type Item = &'a B;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|owned| owned.borrow())
    }
}
