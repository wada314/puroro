//! Small-bytes (SSO) slot for singular `bytes` fields.
//!
//! Physical storage is the shared [`SsoBuf`](super::sso_buf::SsoBuf) union
//! ([`SsoBytes`] = heap [`UnmanagedVec<u8>`]). Short payloads live inline;
//! longer ones use the heap arm. Which arm is live is recorded **only** in a
//! [`MessageCommon`](crate::fields::shared::MessageCommon) heap bit via
//! [`InlineOrHeap`](crate::fields::shared::value_layout::InlineOrHeap)
//! (`true` = heap, `false` = inline).

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::core::ops::Deref;
use ::unmanaged::UnmanagedVec;

use super::sso_buf::{
    SsoBuf, pack_heap as pack_heap_buf, pack_inline, pack_written as pack_written_buf,
};
use super::sso_string::INLINE_CAP;

/// Untagged 24-byte singular `bytes` slot (inline buffer or heap [`UnmanagedVec`]).
///
/// Arm selection requires an external `is_heap` flag ([`InlineOrHeap`] heap bit).
///
/// [`InlineOrHeap`]: crate::fields::shared::value_layout::InlineOrHeap
pub type SsoBytes<A> = SsoBuf<UnmanagedVec<u8, A>>;

/// Packs `bytes` into a slot, using inline storage when it fits.
#[inline]
pub(crate) fn pack_bytes<A: Allocator + Clone>(bytes: &[u8], alloc: A) -> (SsoBytes<A>, bool) {
    if bytes.len() <= INLINE_CAP {
        (pack_inline(bytes), false)
    } else {
        let mut vec = AllocVec::with_capacity_in(bytes.len(), alloc);
        vec.extend_from_slice(bytes);
        (pack_heap(UnmanagedVec::from_vec(vec)), true)
    }
}

/// Heap arm from an [`UnmanagedVec`] (any length, including short/empty).
#[inline]
pub(crate) fn pack_heap<A: Allocator>(heap: UnmanagedVec<u8, A>) -> SsoBytes<A> {
    pack_heap_buf(heap)
}

/// Packs a written [`UnmanagedVec`], demoting to inline when short.
pub(crate) fn pack_written<A: Allocator + Clone>(
    value: UnmanagedVec<u8, A>,
    alloc: A,
) -> (SsoBytes<A>, bool) {
    pack_written_buf(value, alloc)
}

/// Mutable handle for a singular SSO bytes field (`_mut` accessors).
///
/// Edits stay inline while the result fits in [`INLINE_CAP`]; overflow promotes
/// to a heap [`UnmanagedVec`]. The [`BitRef`] heap bit
/// ([`SSO_HEAP`](crate::fields::shared::value_layout::SSO_HEAP) /
/// [`SSO_INLINE`](crate::fields::shared::value_layout::SSO_INLINE)) is the sole
/// arm discriminant.
pub struct SsoBytesMut<'a, A: Allocator> {
    slot: &'a mut SsoBytes<A>,
    /// `MessageCommon` heap bit (`true` = [`SSO_HEAP`](crate::fields::shared::value_layout::SSO_HEAP)).
    tag: BitRef<'a, Mut, u8, Lsb0>,
    alloc: A,
}

impl<'a, A: Allocator + Clone> SsoBytesMut<'a, A> {
    #[inline]
    pub(crate) fn new(slot: &'a mut SsoBytes<A>, tag: BitRef<'a, Mut, u8, Lsb0>, alloc: A) -> Self {
        Self { slot, tag, alloc }
    }

    #[inline]
    fn is_heap(&self) -> bool {
        *self.tag
    }

    #[inline]
    fn set_heap(&mut self, is_heap: bool) {
        *self.tag = is_heap;
    }

    /// Replaces the contents by copying from `bytes`.
    pub fn set(&mut self, bytes: &[u8]) {
        let alloc = self.alloc.clone();
        let (new, new_is_heap) = pack_bytes(bytes, alloc.clone());
        let old_is_heap = self.is_heap();
        // SAFETY: message allocator owns any previous heap buffer; tag matches arm.
        unsafe {
            self.slot
                .replace_packed(new, new_is_heap, old_is_heap, &alloc)
        };
        self.set_heap(new_is_heap);
    }

    /// Replaces the contents by taking ownership of `v`.
    ///
    /// Long values keep `v`'s heap buffer; short values may be stored inline.
    pub fn set_vec(&mut self, v: AllocVec<u8, A>) {
        let alloc = self.alloc.clone();
        let (new, new_is_heap) = pack_written(UnmanagedVec::from_vec(v), alloc.clone());
        let old_is_heap = self.is_heap();
        // SAFETY: message allocator owns any previous heap buffer; tag matches arm.
        unsafe {
            self.slot
                .replace_packed(new, new_is_heap, old_is_heap, &alloc)
        };
        self.set_heap(new_is_heap);
    }

    /// Clears to empty inline bytes.
    pub fn clear(&mut self) {
        use crate::fields::shared::value_layout::SSO_INLINE;

        let alloc = self.alloc.clone();
        let old_is_heap = self.is_heap();
        // SAFETY: message allocator owns any previous heap buffer; tag matches arm.
        unsafe {
            self.slot
                .replace_packed(SsoBytes::empty_inline(), SSO_INLINE, old_is_heap, &alloc)
        };
        self.set_heap(SSO_INLINE);
    }

    /// Appends `bytes`, promoting to heap when the result would exceed [`INLINE_CAP`].
    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }
        if !self.is_heap() {
            let cur_len = self.slot.len::<A>(false);
            if cur_len + bytes.len() <= INLINE_CAP {
                // SAFETY: tag says inline arm is live.
                let inline = unsafe { &mut self.slot.inline };
                inline.data[cur_len..cur_len + bytes.len()].copy_from_slice(bytes);
                inline.len = (cur_len + bytes.len()) as u8;
                return;
            }
            // SAFETY: heap bit says inline; promote then mark heap.
            use crate::fields::shared::value_layout::{SSO_HEAP, SSO_INLINE};
            unsafe { self.slot.promote_to_heap(SSO_INLINE, self.alloc.clone()) };
            self.set_heap(SSO_HEAP);
        }
        {
            // SAFETY: tag says heap arm is live.
            let heap = unsafe { self.slot.heap_mut() };
            // SAFETY: message allocator owns the heap buffer.
            let mut guard = unsafe { heap.with_alloc(self.alloc.clone()) };
            guard.extend_from_slice(bytes);
        }
    }

    /// Appends a single byte.
    pub fn push(&mut self, b: u8) {
        self.extend_from_slice(&[b]);
    }

    /// Shortens to `new_len` bytes.
    pub fn truncate(&mut self, new_len: usize) {
        if !self.is_heap() {
            // SAFETY: tag says inline arm is live.
            let inline = unsafe { &mut self.slot.inline };
            let len = inline.len as usize;
            if new_len < len {
                inline.len = new_len as u8;
            }
            return;
        }
        {
            // SAFETY: tag says heap arm is live.
            let heap = unsafe { self.slot.heap_mut() };
            // SAFETY: message allocator owns the heap buffer.
            let mut guard = unsafe { heap.with_alloc(self.alloc.clone()) };
            guard.truncate(new_len);
        }
    }
}

impl<A: Allocator> Deref for SsoBytesMut<'_, A> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.slot.as_bytes::<A>(*self.tag)
    }
}

impl<A: Allocator + Clone> ::puroro::BytesMut<A> for SsoBytesMut<'_, A> {
    #[inline]
    fn set(&mut self, bytes: &[u8]) {
        SsoBytesMut::set(self, bytes);
    }

    #[inline]
    fn set_vec(&mut self, v: AllocVec<u8, A>) {
        SsoBytesMut::set_vec(self, v);
    }

    #[inline]
    fn clear(&mut self) {
        SsoBytesMut::clear(self);
    }

    #[inline]
    fn extend_from_slice(&mut self, bytes: &[u8]) {
        SsoBytesMut::extend_from_slice(self, bytes);
    }

    #[inline]
    fn push(&mut self, b: u8) {
        SsoBytesMut::push(self, b);
    }

    #[inline]
    fn truncate(&mut self, new_len: usize) {
        SsoBytesMut::truncate(self, new_len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::allocator_api2::alloc::Global;

    #[test]
    fn empty_and_short_are_inline() {
        let (s, is_heap) = pack_bytes::<Global>(b"", Global);
        assert!(!is_heap);
        assert_eq!(s.as_bytes::<Global>(false), b"");
        unsafe { s.deallocate(false, &Global) };

        let (s, is_heap) = pack_bytes(b"hi", Global);
        assert!(!is_heap);
        assert_eq!(s.as_bytes::<Global>(false), b"hi");
        unsafe { s.deallocate(false, &Global) };
    }

    #[test]
    fn boundary_lengths() {
        let max_inline = vec![b'a'; INLINE_CAP];
        let (s, is_heap) = pack_bytes(&max_inline, Global);
        assert!(!is_heap);
        assert_eq!(s.as_bytes::<Global>(false).len(), INLINE_CAP);
        unsafe { s.deallocate(false, &Global) };

        let needs_heap = vec![b'a'; INLINE_CAP + 1];
        let (s, is_heap) = pack_bytes(&needs_heap, Global);
        assert!(is_heap);
        assert_eq!(s.as_bytes::<Global>(true), needs_heap);
        unsafe { s.deallocate(true, &Global) };
    }

    #[test]
    fn pack_written_demotes_short() {
        let mut vec = AllocVec::new_in(Global);
        vec.extend_from_slice(b"xy");
        let (s, is_heap) = pack_written(UnmanagedVec::from_vec(vec), Global);
        assert!(!is_heap);
        assert_eq!(s.as_bytes::<Global>(false), b"xy");
        unsafe { s.deallocate(false, &Global) };
    }

    #[test]
    fn clone_packed_preserves_arm() {
        let (inline, _) = pack_bytes(b"ab", Global);
        let (c, is_heap) = inline.clone_packed(false, Global);
        assert!(!is_heap);
        assert_eq!(c.as_bytes::<Global>(false), b"ab");
        unsafe {
            inline.deallocate(false, &Global);
            c.deallocate(false, &Global);
        }

        let long = vec![b'z'; INLINE_CAP + 2];
        let (heap, _) = pack_bytes(&long, Global);
        let (c, is_heap) = heap.clone_packed(true, Global);
        assert!(is_heap);
        assert_eq!(c.as_bytes::<Global>(true), heap.as_bytes::<Global>(true));
        unsafe {
            heap.deallocate(true, &Global);
            c.deallocate(true, &Global);
        }
    }
}
