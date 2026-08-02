//! Length-scoped [`Buf`] wrapper for nested-message decode.
//!
//! Nested LEN frames push an absolute end offset onto a dynamic stack so the
//! same concrete buffer type is reused at every recursion depth (no
//! `Take<Take<…>>` monomorphization, no `copy_to_bytes` of the payload).

use ::bytes::Buf;
use ::core::ops::{Deref, DerefMut};

use crate::error::DecodeError;

/// Buffer used by nested decode: a [`Buf`] that can push / pop LEN scopes.
///
/// The only implementor is [`ScopedBuf`]. [`Message::merge_from`](crate::Message::merge_from)
/// wraps an arbitrary `Buf` into a [`ScopedBuf`] before calling
/// [`merge_from_with_depth`](crate::Message::merge_from_with_depth).
pub trait DecodeBuf: Buf {
    /// Restrict subsequent reads to the next `len` bytes of the current scope.
    ///
    /// Returns [`DecodeError::TruncatedMessage`] when fewer than `len` bytes
    /// remain in the current scope.
    fn push_limit(&mut self, len: usize) -> Result<(), DecodeError>;

    /// End the innermost scope, skipping any unread bytes up to its end.
    ///
    /// Panics if the limit stack is empty.
    fn pop_limit(&mut self);

    /// [`push_limit`](Self::push_limit) plus a guard that pops on drop.
    #[inline]
    fn push_limit_guard(&mut self, len: usize) -> Result<LimitGuard<'_, Self>, DecodeError> {
        self.push_limit(len)?;
        Ok(LimitGuard { buf: self })
    }
}

/// [`Buf`] adapter that tracks nested LEN frames via a stack of absolute ends.
///
/// Each [`push_limit`](DecodeBuf::push_limit) records `consumed + len` in `ends`.
/// [`remaining`](Buf::remaining) is the minimum of the inner buffer's remaining
/// bytes and the distance to the innermost end.
pub struct ScopedBuf<'a, B: Buf> {
    inner: &'a mut B,
    /// Total bytes advanced through this wrapper.
    consumed: usize,
    /// Absolute `consumed` values at which each nested frame ends (innermost last).
    ends: Vec<usize>,
}

impl<'a, B: Buf> ScopedBuf<'a, B> {
    /// Wrap `inner` with an empty limit stack (no length restriction).
    #[inline]
    pub fn new(inner: &'a mut B) -> Self {
        Self {
            inner,
            consumed: 0,
            ends: Vec::new(),
        }
    }

    /// Bytes still allowed by the innermost limit, or `None` if unrestricted.
    #[inline]
    fn frame_remaining(&self) -> Option<usize> {
        self.ends
            .last()
            .map(|&end| end.saturating_sub(self.consumed))
    }
}

impl<B: Buf> Buf for ScopedBuf<'_, B> {
    #[inline]
    fn remaining(&self) -> usize {
        match self.frame_remaining() {
            Some(frame) => self.inner.remaining().min(frame),
            None => self.inner.remaining(),
        }
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        let rem = self.remaining();
        let chunk = self.inner.chunk();
        if chunk.len() > rem {
            &chunk[..rem]
        } else {
            chunk
        }
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        let rem = self.remaining();
        assert!(
            cnt <= rem,
            "cannot advance past scoped remaining: cnt={cnt}, remaining={rem}"
        );
        self.inner.advance(cnt);
        self.consumed += cnt;
    }
}

impl<B: Buf> DecodeBuf for ScopedBuf<'_, B> {
    #[inline]
    fn push_limit(&mut self, len: usize) -> Result<(), DecodeError> {
        if self.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        self.ends.push(self.consumed + len);
        Ok(())
    }

    #[inline]
    fn pop_limit(&mut self) {
        let end = self
            .ends
            .pop()
            .expect("pop_limit called with empty limit stack");
        if self.consumed < end {
            let skip = end - self.consumed;
            // Skip is within the popped frame; advance the inner buffer directly
            // so the parent frame's remaining is not consulted mid-skip.
            assert!(
                skip <= self.inner.remaining(),
                "scoped frame end exceeds inner buffer"
            );
            self.inner.advance(skip);
            self.consumed = end;
        }
    }
}

/// RAII guard that pops the innermost limit when dropped.
///
/// Obtained from [`DecodeBuf::push_limit_guard`]. Derefs to the underlying
/// [`DecodeBuf`] while the scope is active.
pub struct LimitGuard<'a, B: DecodeBuf + ?Sized> {
    buf: &'a mut B,
}

impl<B: DecodeBuf + ?Sized> Deref for LimitGuard<'_, B> {
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.buf
    }
}

impl<B: DecodeBuf + ?Sized> DerefMut for LimitGuard<'_, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buf
    }
}

impl<B: DecodeBuf + ?Sized> Drop for LimitGuard<'_, B> {
    fn drop(&mut self) {
        self.buf.pop_limit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unrestricted_reads_match_inner() {
        let mut data: &[u8] = &[1, 2, 3, 4, 5];
        let mut scoped = ScopedBuf::new(&mut data);
        assert_eq!(scoped.remaining(), 5);
        assert_eq!(scoped.chunk(), &[1, 2, 3, 4, 5]);
        scoped.advance(2);
        assert_eq!(scoped.remaining(), 3);
        assert_eq!(data, &[3, 4, 5]);
    }

    #[test]
    fn push_limit_restricts_remaining() {
        let mut data: &[u8] = &[10, 20, 30, 40, 50];
        let mut scoped = ScopedBuf::new(&mut data);
        scoped.push_limit(3).unwrap();
        assert_eq!(scoped.remaining(), 3);
        assert_eq!(scoped.chunk(), &[10, 20, 30]);
        scoped.advance(3);
        assert_eq!(scoped.remaining(), 0);
        scoped.pop_limit();
        assert_eq!(scoped.remaining(), 2);
        assert_eq!(scoped.chunk(), &[40, 50]);
    }

    #[test]
    fn pop_limit_skips_unread() {
        let mut data: &[u8] = &[1, 2, 3, 4, 5, 6];
        let mut scoped = ScopedBuf::new(&mut data);
        scoped.push_limit(4).unwrap();
        scoped.advance(1);
        scoped.pop_limit(); // skip 2,3,4
        assert_eq!(scoped.remaining(), 2);
        assert_eq!(scoped.chunk(), &[5, 6]);
        assert_eq!(data, &[5, 6]);
    }

    #[test]
    fn nested_limits() {
        let mut data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
        let mut scoped = ScopedBuf::new(&mut data);
        scoped.push_limit(6).unwrap();
        scoped.push_limit(3).unwrap();
        assert_eq!(scoped.remaining(), 3);
        scoped.advance(3);
        scoped.pop_limit();
        assert_eq!(scoped.remaining(), 3); // bytes 4,5,6 in outer frame
        scoped.pop_limit();
        assert_eq!(scoped.remaining(), 2); // 7,8
    }

    #[test]
    fn push_limit_truncated() {
        let mut data: &[u8] = &[1, 2];
        let mut scoped = ScopedBuf::new(&mut data);
        assert_eq!(scoped.push_limit(3), Err(DecodeError::TruncatedMessage));
    }

    #[test]
    fn limit_guard_pops_on_drop() {
        let mut data: &[u8] = &[1, 2, 3, 4];
        let mut scoped = ScopedBuf::new(&mut data);
        {
            let mut guard = scoped.push_limit_guard(2).unwrap();
            assert_eq!(guard.remaining(), 2);
            guard.advance(1);
        }
        assert_eq!(scoped.remaining(), 2);
        assert_eq!(scoped.chunk(), &[3, 4]);
    }

    #[test]
    fn limit_guard_skips_unread_on_drop() {
        let mut data: &[u8] = &[1, 2, 3, 4, 5];
        let mut scoped = ScopedBuf::new(&mut data);
        {
            let _guard = scoped.push_limit_guard(3).unwrap();
            // consume nothing
        }
        assert_eq!(scoped.chunk(), &[4, 5]);
    }
}
