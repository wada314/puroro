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

use ::std::fmt::Debug;
use ::std::io::Write;
use ::std::ops::{Deref, DerefMut};
use ::std::ptr::NonNull;
use ::std::slice;

/// An optimized `Vec<u8>` for protobuf `bytes` field type.
///
/// The length of `bytes` is shorter than 2^32, so the length
/// and the capacity of the buffer is `u32` instead of `usize`.
///
/// Also, it does not allocate heap memory until its length become longer than
/// the `size_of::<*mut u8>() + size_of::<u32>()` value.
#[repr(C, align(8))]
pub struct Bytes {
    maybe_ptr_cap: MaybePtrCap,
    length: u32,
}

#[derive(Clone, Copy)]
#[repr(C, packed(4))]
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
    PtrCap(
        // #[repr(packed)] struct cannot make a reference!
        PtrCap,
    ),
    Bytes(&'a [u8; PTR_CAP_SIZE]),
}
enum CaseMut<'a> {
    PtrCap(
        // #[repr(packed)] struct cannot make a reference!
        PtrCap,
    ),
    Bytes(&'a mut [u8; PTR_CAP_SIZE]),
}

impl Clone for Bytes {
    fn clone(&self) -> Self {
        match self.case() {
            Case::PtrCap(_) => unsafe {
                let vec = self.assume_is_vec();
                let cloned_vec = vec.clone();
                ::std::mem::forget(vec);
                let mut cloned = Self::new();
                cloned.write_back_vec(cloned_vec);
                cloned
            },
            Case::Bytes(&bytes) => Self {
                maybe_ptr_cap: MaybePtrCap { bytes },
                length: self.length,
            },
        }
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self.case() {
            Case::PtrCap(PtrCap { ptr, .. }) => unsafe {
                slice::from_raw_parts_mut(ptr.as_ptr(), self.length as usize)
            },
            Case::Bytes(bytes) => &bytes[..(self.length as usize)],
        }
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let length = self.length as usize;
        match self.case_mut() {
            CaseMut::PtrCap(PtrCap { ptr, .. }) => unsafe {
                slice::from_raw_parts_mut(ptr.as_ptr(), length)
            },
            CaseMut::Bytes(bytes) => &mut bytes[..(length)],
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

impl From<&Vec<u8>> for Bytes {
    fn from(value: &Vec<u8>) -> Self {
        value.as_slice().into()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        let mut bytes = Bytes::new();
        if value.len() <= PTR_CAP_SIZE {
            bytes.length = value.len().try_into().unwrap();
            unsafe { &mut bytes.maybe_ptr_cap.bytes[..(value.len())] }.copy_from_slice(&value);
        } else {
            unsafe { bytes.write_back_vec(value) };
        }
        bytes
    }
}

impl Extend<u8> for Bytes {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, into_iter: T) {
        let mut iter = into_iter.into_iter().fuse();

        // while shorter than the threshold
        while (self.length as usize) < PTR_CAP_SIZE {
            if let Some(b) = iter.next() {
                let bytes_array = unsafe { &mut self.maybe_ptr_cap.bytes };
                bytes_array[self.length as usize] = b;
                self.length += 1;
            } else {
                break;
            }
        }

        // when stepping over the threshold
        if (self.length as usize) == PTR_CAP_SIZE {
            if let Some(b) = iter.next() {
                let mut vec = unsafe { self.maybe_ptr_cap.bytes }.to_vec();
                vec.push(b);
                unsafe { self.write_back_vec(vec) };
            }
        }

        if (self.length as usize) > PTR_CAP_SIZE {
            // the bytes contents is now guaranteed to be on the heap memory.
            let mut vec = unsafe { self.assume_is_vec() };
            vec.extend(iter);
            unsafe { self.write_back_vec(vec) };
        }
    }
}

impl FromIterator<u8> for Bytes {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut bytes = Self::new();
        bytes.extend(iter);
        bytes
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        <[u8] as PartialEq>::eq(self, other)
    }
}

impl PartialEq<[u8]> for Bytes {
    fn eq(&self, other: &[u8]) -> bool {
        <[u8] as PartialEq>::eq(self, other)
    }
}

impl Eq for Bytes {}

impl Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <[u8] as Debug>::fmt(&self, f)
    }
}

impl Bytes {
    fn case(&self) -> Case<'_> {
        let length = self.length as usize;
        if length <= PTR_CAP_SIZE {
            unsafe { Case::Bytes(&self.maybe_ptr_cap.bytes) }
        } else {
            unsafe { Case::PtrCap(self.maybe_ptr_cap.ptr_cap) }
        }
    }
    fn case_mut(&mut self) -> CaseMut<'_> {
        let length = self.length as usize;
        if length <= PTR_CAP_SIZE {
            unsafe { CaseMut::Bytes(&mut self.maybe_ptr_cap.bytes) }
        } else {
            unsafe { CaseMut::PtrCap(self.maybe_ptr_cap.ptr_cap) }
        }
    }
    unsafe fn maybe_drop_nocleanup(&self) {
        if let Case::PtrCap(PtrCap { ptr, capacity }) = self.case() {
            unsafe {
                Vec::from_raw_parts(ptr.as_ptr(), self.length as usize, capacity as usize);
            }
        }
    }
    unsafe fn assume_is_vec(&self) -> Vec<u8> {
        let PtrCap { ptr, capacity } = self.maybe_ptr_cap.ptr_cap;
        Vec::from_raw_parts(ptr.as_ptr(), self.length as usize, capacity as usize)
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
            // Reconstruct the given vec to release the buffer.
            Vec::from_raw_parts(ptr, length_usize, capacity_usize);
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
        let length = self.length as usize;
        match self.case_mut() {
            CaseMut::PtrCap(PtrCap { ptr, capacity }) => {
                let mut vec =
                    unsafe { Vec::from_raw_parts(ptr.as_ptr(), length, capacity as usize) };
                vec.extend_from_slice(other);
                unsafe { self.write_back_vec(vec) };
            }
            CaseMut::Bytes(bytes) => {
                let new_len = length + other.len();
                if new_len > PTR_CAP_SIZE {
                    let mut vec = Vec::with_capacity(new_len);
                    vec.extend_from_slice(&bytes[..length]);
                    vec.extend_from_slice(other);
                    unsafe {
                        self.write_back_vec(vec);
                    }
                } else {
                    bytes[length..new_len].copy_from_slice(other);
                    self.length = new_len.try_into().unwrap();
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.case(); // drop vec
        self.length = 0;
    }
}

#[cfg(test)]
mod test {
    use super::Bytes;

    #[test]
    fn test_new() {
        let new_bytes = Bytes::new();
        assert_eq!(&new_bytes, &[] as &[u8]);
    }

    #[test]
    fn test_extend_slice() {
        let mut bytes = Bytes::new();
        assert_eq!(&bytes, [].as_slice());

        bytes.extend_from_slice(&[0]);
        assert_eq!(&bytes, [0].as_slice());

        bytes.extend_from_slice(&[1, 2]);
        assert_eq!(&bytes, [0, 1, 2].as_slice());
    }

    #[test]
    fn test_extend_slice_11() {
        let mut bytes: Bytes = b"0123456789".as_slice().into();
        bytes.extend_from_slice(b"a");
        assert_eq!(&bytes, b"0123456789a".as_slice());
    }

    #[test]
    fn test_extend_slice_12() {
        let mut bytes: Bytes = b"0123456789".as_slice().into();
        bytes.extend_from_slice(b"ab");
        assert_eq!(&bytes, b"0123456789ab".as_slice());
    }

    #[test]
    fn test_extend_slice_13() {
        let mut bytes: Bytes = b"0123456789".as_slice().into();
        bytes.extend_from_slice(b"abc");
        assert_eq!(&bytes, b"0123456789abc".as_slice());
    }

    #[test]
    fn test_layout() {
        assert_eq!(16, ::std::mem::size_of::<Bytes>());
        assert_eq!(8, ::std::mem::align_of::<Bytes>());
    }
}
