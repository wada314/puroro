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

//! A customized version of `std::vec::Vec` for use in protobuf field type
//! `bytes`.
//! The `bytes` type in protobuf can only have max 2^32 - 1 length, and
//! the item type is limited to `u8` type, so we can make a special
//! optimized version for it.

use ::std::alloc;
use ::std::alloc::Layout;
use ::std::mem;
use ::std::ops::Deref;
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

impl Bytes {
    fn maybe_ptr_cap(&self) -> Option<&PtrCap> {
        (self.length as usize > PTR_CAP_SIZE).then(|| unsafe { &self.maybe_ptr_cap.ptr_cap })
    }
    unsafe fn maybe_drop_nocleanup(&self) {
        if let Some(PtrCap { ptr, capacity }) = self.maybe_ptr_cap().cloned() {
            alloc::dealloc(
                ptr.as_ptr(),
                Layout::from_size_align_unchecked(capacity as usize, mem::align_of::<u8>()),
            )
        }
    }

    pub const fn new() -> Self {
        Self {
            maybe_ptr_cap: MaybePtrCap {
                bytes: [0; PTR_CAP_SIZE],
            },
            length: 0,
        }
    }
}
