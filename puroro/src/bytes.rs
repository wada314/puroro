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

//! A customized version of `std::vec::Vec<u8>`
//! for use in protobuf field type `bytes`.
//! The `bytes` type in protobuf can only have max 2^32 - 1 length, and
//! the item type is limited to `u8` type, so we can make a special
//! optimized version for it.

use ::std::alloc;
use ::std::alloc::Layout;
use ::std::io::Write;
use ::std::mem;
use ::std::ops::Deref;
use ::std::ptr;
use ::std::ptr::NonNull;
use ::std::slice;

/// An optimized `Vec<u8>` for protobuf `bytes` field type.
///
/// The length of `bytes` is shorter than 2^32, so the length
/// and the capacity of the buffer is `u32` instead of `usize`.
///
/// Also, it does not allocate heap memory until its length become longer than
/// the `sizeof::<*mut u8>() + sizeof::<u32>()` value.
#[repr(C)]
pub struct Bytes {
    maybe_ptr_cap: MaybePtrCap,
    length: u32,
}

#[derive(Clone, Copy)]
struct PtrCap {
    ptr: NonNull<u8>,
    capacity: u32,
}
const PTR_CAP_SIZE: usize = ::std::mem::size_of::<PtrCap>();

union MaybePtrCap {
    ptr_cap: PtrCap,
    bytes: [u8; PTR_CAP_SIZE],
}

enum Case<'a> {
    PtrCap(Vec<u8>),
    Bytes(&'a [u8], &'a [u8]),
}
enum CaseMut<'a> {
    PtrCap(Vec<u8>),
    Bytes(&'a mut [u8], &'a mut [u8]),
}

impl Default for Bytes {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        if self.length as usize <= PTR_CAP_SIZE {
            unsafe { &self.maybe_ptr_cap.bytes }
        } else {
            unsafe {
                slice::from_raw_parts(
                    self.maybe_ptr_cap.ptr_cap.ptr.as_ptr(),
                    self.length as usize,
                )
            }
        }
    }
}

impl Drop for Bytes {
    fn drop(&mut self) {
        unsafe { self.maybe_drop_nocleanup() }
    }
}

impl Write for Bytes {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        let mut bytes = Bytes::new();
        bytes.extend_from_slice(value);
        bytes
    }
}

impl Bytes {
    fn maybe_ptr_cap(&self) -> Option<&PtrCap> {
        (self.length as usize > PTR_CAP_SIZE).then(|| unsafe { &self.maybe_ptr_cap.ptr_cap })
    }
    fn case(&self) -> Case<'_> {
        let length = self.length as usize;
        if length <= PTR_CAP_SIZE {
            unsafe {
                let (first, last) = self.maybe_ptr_cap.bytes.split_at(length);
                Case::Bytes(first, last)
            }
        } else {
            Case::PtrCap(unsafe {
                Vec::from_raw_parts(
                    self.maybe_ptr_cap.ptr_cap.ptr.as_ptr(),
                    length,
                    self.maybe_ptr_cap.ptr_cap.capacity as usize,
                )
            })
        }
    }
    fn case_mut(&mut self) -> CaseMut<'_> {
        let length = self.length as usize;
        if length <= PTR_CAP_SIZE {
            unsafe {
                let (first, last) = self.maybe_ptr_cap.bytes.split_at_mut(length);
                CaseMut::Bytes(first, last)
            }
        } else {
            CaseMut::PtrCap(unsafe {
                Vec::from_raw_parts(
                    self.maybe_ptr_cap.ptr_cap.ptr.as_ptr(),
                    length,
                    self.maybe_ptr_cap.ptr_cap.capacity as usize,
                )
            })
        }
    }
    unsafe fn maybe_drop_nocleanup(&self) {
        if let Some(PtrCap { ptr, capacity }) = self.maybe_ptr_cap().cloned() {
            alloc::dealloc(
                ptr.as_ptr(),
                Layout::from_size_align_unchecked(capacity as usize, mem::align_of::<u8>()),
            )
        }
    }
    unsafe fn write_back_vec(&mut self, vec: Vec<u8>) {
        let (ptr, length_usize, capacity_usize) = vec.into_raw_parts();
        let length: u32 = length_usize.try_into().unwrap();
        let capacity: u32 = capacity_usize.try_into().unwrap();
        if length_usize <= PTR_CAP_SIZE {
            ::std::ptr::copy_nonoverlapping(
                ptr,
                self.maybe_ptr_cap.bytes.as_mut_ptr(),
                length_usize,
            );
        } else {
            self.maybe_ptr_cap = MaybePtrCap {
                ptr_cap: PtrCap {
                    ptr: NonNull::new_unchecked(ptr),
                    capacity,
                },
            }
        }
        self.length = length;
    }

    pub const fn new() -> Self {
        Self {
            maybe_ptr_cap: MaybePtrCap {
                bytes: [0; PTR_CAP_SIZE],
            },
            length: 0,
        }
    }

    pub fn extend_from_slice(&mut self, other: &[u8]) {
        match self.case_mut() {
            CaseMut::PtrCap(mut tmp_vec) => {
                tmp_vec.extend_from_slice(other);
                unsafe { self.write_back_vec(tmp_vec) };
            }
            CaseMut::Bytes(first, last) => {
                if other.len() <= last.len() {
                    unsafe {
                        ptr::copy_nonoverlapping(other.as_ptr(), last.as_mut_ptr(), other.len())
                    }
                } else {
                    let mut new_vec = Vec::with_capacity(first.len() + other.len());
                    new_vec.extend_from_slice(first);
                    new_vec.extend_from_slice(other);
                    unsafe { self.write_back_vec(new_vec) }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.case(); // drop vec
        self.length = 0;
    }
}
