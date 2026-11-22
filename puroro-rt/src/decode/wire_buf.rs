//! Wire buffers held by a lazy message.
//!
//! Owned lazy messages use [`Bytes`]. Zero-copy decode uses `&[u8]`. See
//! `.cursor/plans/wire-buf.md`.

use ::bytes::{Bytes, BytesMut};
use ::core::mem;
use ::puroro::DecodeError;

use super::record::WireSpan;

/// Contiguous wire bytes that can yield a subrange without borrowing `self`.
///
/// `Slice::Slice` is `Slice`, so nested subranges stop changing type.
pub trait WireBuf: Clone {
    /// One contiguous subrange. Does not borrow `self`.
    type Slice: WireBuf<Slice = Self::Slice>;

    fn as_bytes(&self) -> &[u8];

    fn subslice(&self, span: WireSpan) -> Result<Self::Slice, DecodeError>;

    /// Empty placeholder for a nested slot before [`super::LazyScan::adopt`].
    ///
    /// For `&[u8]` this is `&[]` (coerces to any `'buf`). For [`Bytes`] it is
    /// an empty owned buffer. Roots that decode from a buffer use `open` /
    /// `parse_in` instead of this constructor.
    fn empty() -> Self;
}

/// Buffers that can grow. `&[u8]` does not implement this.
pub trait WireAppend: WireBuf {
    /// Append `chunk`. Shared buffers copy then append (COW).
    fn append(&mut self, chunk: &[u8]);
}

impl<'buf> WireBuf for &'buf [u8] {
    type Slice = &'buf [u8];

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self
    }

    #[inline]
    fn subslice(&self, span: WireSpan) -> Result<&'buf [u8], DecodeError> {
        // `self` is `&&'buf [u8]`. Passing it to `slice` can tie the result to
        // the method borrow. Dereference so the result is `'buf`.
        #[allow(clippy::explicit_auto_deref)]
        {
            span.slice(*self)
        }
    }

    #[inline]
    fn empty() -> Self {
        &[]
    }
}

impl WireBuf for Bytes {
    type Slice = Bytes;

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self
    }

    #[inline]
    fn subslice(&self, span: WireSpan) -> Result<Bytes, DecodeError> {
        let _ = span.slice(self.as_bytes())?;
        let end = span.offset + span.len;
        Ok(self.slice(span.offset..end))
    }

    #[inline]
    fn empty() -> Self {
        Bytes::new()
    }
}

impl WireAppend for Bytes {
    fn append(&mut self, chunk: &[u8]) {
        if chunk.is_empty() {
            return;
        }
        let mut buf = BytesMut::from(mem::take(self));
        buf.extend_from_slice(chunk);
        *self = buf.freeze();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_subslice_keeps_buf_lifetime() {
        let root: &[u8] = b"xxHELPyy";
        let part = take_subslice(root);
        assert_eq!(part, b"HELP");
        assert_eq!(part.as_ptr(), root[2..].as_ptr());
    }

    /// Compiles only when `subslice` returns the buffer lifetime, not `&self`.
    fn take_subslice(root: &[u8]) -> &[u8] {
        WireBuf::subslice(&root, WireSpan { offset: 2, len: 4 }).unwrap()
    }

    #[test]
    fn slice_subslice_out_of_range() {
        let root: &[u8] = b"ab";
        assert!(matches!(
            WireBuf::subslice(&root, WireSpan { offset: 1, len: 2 }),
            Err(DecodeError::TruncatedMessage)
        ));
    }

    #[test]
    fn bytes_subslice_shares_allocation() {
        let root = Bytes::copy_from_slice(b"xxHELPyy");
        let part = root.subslice(WireSpan { offset: 2, len: 4 }).unwrap();
        assert_eq!(part.as_bytes(), b"HELP");
        assert_eq!(part.as_ptr(), root.as_bytes()[2..].as_ptr());
    }

    #[test]
    fn bytes_append_unique_keeps_going() {
        let mut w = Bytes::copy_from_slice(b"ab");
        w.append(b"cd");
        assert_eq!(w.as_bytes(), b"abcd");
    }

    #[test]
    fn bytes_append_after_clone_does_not_mutate_peer() {
        let mut parent = Bytes::copy_from_slice(b"ab");
        let child = parent.clone();
        parent.append(b"cd");
        assert_eq!(parent.as_bytes(), b"abcd");
        assert_eq!(child.as_bytes(), b"ab");
    }

    #[test]
    fn bytes_empty_is_empty() {
        assert!(Bytes::empty().is_empty());
    }

    #[test]
    fn slice_empty_is_empty() {
        let e: &[u8] = <&[u8] as WireBuf>::empty();
        assert!(e.is_empty());
    }
}
