//! Shared island-root wire buffer.
//!
//! The first implementation stores bytes in [`Bytes`] (global allocator).
//! [`SharedWire`] still carries `A` so a later `AllocVec` + refcount swap
//! keeps the same call sites.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bytes::{Bytes, BytesMut};
use ::core::fmt::{self, Debug, Formatter};
use ::core::mem;

use super::record::WireSpan;

/// Reference-counted wire blob plus the message allocator (unused for data).
pub struct SharedWire<A: Allocator = Global> {
    data: Bytes,
    alloc: A,
}

impl<A: Allocator + Clone> Clone for SharedWire<A> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            alloc: self.alloc.clone(),
        }
    }
}

impl<A: Allocator> SharedWire<A> {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<A: Allocator + Clone> SharedWire<A> {
    pub fn empty(alloc: A) -> Self {
        Self {
            data: Bytes::new(),
            alloc,
        }
    }

    pub fn from_slice(bytes: &[u8], alloc: A) -> Self {
        Self {
            data: Bytes::copy_from_slice(bytes),
            alloc,
        }
    }

    /// Cheap handle to the same allocation (same as [`Clone`]).
    pub fn clone_handle(&self) -> Self {
        self.clone()
    }

    /// Same bytes, destination allocator stored on the handle.
    pub fn clone_in(&self, alloc: A) -> Self {
        Self {
            data: self.data.clone(),
            alloc,
        }
    }

    /// Sub-handle of `span`. Offsets in the result are 0-based in the slice.
    pub fn slice(&self, span: WireSpan) -> Result<Self, ::puroro::DecodeError> {
        let _ = span.slice(self.as_bytes())?;
        let end = span.offset + span.len;
        Ok(Self {
            data: self.data.slice(span.offset..end),
            alloc: self.alloc.clone(),
        })
    }

    /// Append `more`. If other handles exist, copies then appends (COW).
    pub fn append(&mut self, more: &[u8]) {
        if more.is_empty() {
            return;
        }
        let mut buf = BytesMut::from(mem::take(&mut self.data));
        buf.extend_from_slice(more);
        self.data = buf.freeze();
    }
}

impl<A: Allocator> Debug for SharedWire<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedWire")
            .field("len", &self.data.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::allocator_api2::alloc::Global;

    #[test]
    fn clone_shares_allocation() {
        let a = SharedWire::from_slice(b"hello", Global);
        let b = a.clone_handle();
        assert_eq!(a.as_bytes().as_ptr(), b.as_bytes().as_ptr());
        assert_eq!(a.as_bytes(), b"hello");
    }

    #[test]
    fn slice_is_subrange_without_copy() {
        let root = SharedWire::from_slice(b"xxHELPyy", Global);
        let part = root.slice(WireSpan { offset: 2, len: 4 }).unwrap();
        assert_eq!(part.as_bytes(), b"HELP");
        assert_eq!(part.as_bytes().as_ptr(), root.as_bytes()[2..].as_ptr());
    }

    #[test]
    fn append_unique_keeps_going() {
        let mut w = SharedWire::from_slice(b"ab", Global);
        w.append(b"cd");
        assert_eq!(w.as_bytes(), b"abcd");
    }

    #[test]
    fn append_after_clone_does_not_mutate_peer() {
        let mut parent = SharedWire::from_slice(b"ab", Global);
        let child = parent.clone_handle();
        parent.append(b"cd");
        assert_eq!(parent.as_bytes(), b"abcd");
        assert_eq!(child.as_bytes(), b"ab");
    }
}
