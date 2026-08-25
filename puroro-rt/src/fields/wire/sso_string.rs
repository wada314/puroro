//! Small-string (SSO) slot for singular `string` fields.
//!
//! Physical storage is the shared [`SsoBuf`](super::sso_buf::SsoBuf) union
//! ([`SsoString`] = heap [`UnmanagedString`]). Short payloads live inline;
//! longer ones use the heap arm. Which arm is live is recorded **only** in a
//! [`MessageCommon`](crate::fields::shared::MessageCommon) heap bit via
//! [`InlineOrHeap`](crate::fields::shared::value_layout::InlineOrHeap)
//! (`true` = heap, `false` = inline).
//!
//! Packing (inline vs heap) is done by crate-private helpers; callers of
//! [`SsoStringMut`] / [`InlineOrHeap`] pass logical UTF-8 and do not see arms.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::core::ops::Deref;
use ::core::str;
use ::unmanaged::{String as AllocString, UnmanagedString};

use super::sso_buf::{
    SsoBuf, pack_heap as pack_heap_buf, pack_inline, pack_written as pack_written_buf,
};

pub use super::sso_buf::INLINE_CAP;

/// Untagged 24-byte singular `string` slot (inline buffer or heap [`UnmanagedString`]).
///
/// Arm selection requires an external `is_heap` flag ([`InlineOrHeap`] heap bit).
///
/// [`InlineOrHeap`]: crate::fields::shared::value_layout::InlineOrHeap
pub type SsoString<A> = SsoBuf<UnmanagedString<A>>;

/// Packs `s` into a slot, using inline storage when it fits.
#[inline]
pub(crate) fn pack_str<A: Allocator + Clone>(s: &str, alloc: A) -> (SsoString<A>, bool) {
    if s.len() <= INLINE_CAP {
        (pack_inline_utf8(s.as_bytes()), false)
    } else {
        (
            pack_heap(UnmanagedString::from_string(AllocString::from_str_in(
                s, alloc,
            ))),
            true,
        )
    }
}

/// Inline arm from already-validated UTF-8 bytes (`len <= INLINE_CAP`).
#[inline]
pub(crate) fn pack_inline_utf8<A: Allocator>(bytes: &[u8]) -> SsoString<A> {
    pack_inline(bytes)
}

/// Heap arm from an [`UnmanagedString`] (any length, including short/empty).
#[inline]
pub(crate) fn pack_heap<A: Allocator>(heap: UnmanagedString<A>) -> SsoString<A> {
    pack_heap_buf(heap)
}

/// Packs a written [`UnmanagedString`], demoting to inline when short.
///
/// On demote, `value` is deallocated after copying bytes into the inline arm.
pub(crate) fn pack_written<A: Allocator + Clone>(
    value: UnmanagedString<A>,
    alloc: A,
) -> (SsoString<A>, bool) {
    pack_written_buf(value, alloc)
}

impl<A: Allocator> SsoBuf<UnmanagedString<A>> {
    pub fn as_str(&self, is_heap: bool) -> &str {
        // SAFETY: constructors / mutators keep the live arm as UTF-8.
        unsafe { str::from_utf8_unchecked(self.as_bytes::<A>(is_heap)) }
    }
}

/// Mutable handle for a singular SSO string field (`_mut` accessors).
///
/// Edits stay inline while the result fits in [`INLINE_CAP`]; overflow promotes
/// to a heap [`UnmanagedString`]. The [`BitRef`] heap bit
/// ([`SSO_HEAP`](crate::fields::shared::value_layout::SSO_HEAP) /
/// [`SSO_INLINE`](crate::fields::shared::value_layout::SSO_INLINE)) is the sole
/// arm discriminant.
pub struct SsoStringMut<'a, A: Allocator> {
    slot: &'a mut SsoString<A>,
    /// `MessageCommon` heap bit (`true` = [`SSO_HEAP`](crate::fields::shared::value_layout::SSO_HEAP)).
    tag: BitRef<'a, Mut, u8, Lsb0>,
    alloc: A,
}

impl<'a, A: Allocator + Clone> SsoStringMut<'a, A> {
    #[inline]
    pub(crate) fn new(
        slot: &'a mut SsoString<A>,
        tag: BitRef<'a, Mut, u8, Lsb0>,
        alloc: A,
    ) -> Self {
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

    /// Replaces the contents by copying from `s`.
    pub fn set(&mut self, s: &str) {
        let alloc = self.alloc.clone();
        let (new, new_is_heap) = pack_str(s, alloc.clone());
        let old_is_heap = self.is_heap();
        // SAFETY: message allocator owns any previous heap buffer; tag matches arm.
        unsafe {
            self.slot
                .replace_packed(new, new_is_heap, old_is_heap, &alloc)
        };
        self.set_heap(new_is_heap);
    }

    /// Replaces the contents by taking ownership of `s`.
    ///
    /// Long values keep `s`'s heap buffer; short values may be stored inline.
    pub fn set_string(&mut self, s: AllocString<A>) {
        let alloc = self.alloc.clone();
        let (new, new_is_heap) = pack_written(UnmanagedString::from_string(s), alloc.clone());
        let old_is_heap = self.is_heap();
        // SAFETY: message allocator owns any previous heap buffer; tag matches arm.
        unsafe {
            self.slot
                .replace_packed(new, new_is_heap, old_is_heap, &alloc)
        };
        self.set_heap(new_is_heap);
    }

    /// Clears to an empty inline string.
    pub fn clear(&mut self) {
        use crate::fields::shared::value_layout::SSO_INLINE;

        let alloc = self.alloc.clone();
        let old_is_heap = self.is_heap();
        // SAFETY: message allocator owns any previous heap buffer; tag matches arm.
        unsafe {
            self.slot
                .replace_packed(SsoString::empty_inline(), SSO_INLINE, old_is_heap, &alloc)
        };
        self.set_heap(SSO_INLINE);
    }

    /// Appends `s`, promoting to heap when the result would exceed [`INLINE_CAP`].
    pub fn push_str(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        if !self.is_heap() {
            let cur_len = self.slot.len::<A>(false);
            if cur_len + s.len() <= INLINE_CAP {
                // SAFETY: tag says inline arm is live.
                let inline = unsafe { &mut self.slot.inline };
                inline.data[cur_len..cur_len + s.len()].copy_from_slice(s.as_bytes());
                inline.len = (cur_len + s.len()) as u8;
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
            guard.push_str(s);
        }
    }

    /// Appends a Unicode scalar value.
    pub fn push(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        self.push_str(ch.encode_utf8(&mut buf));
    }

    /// Shortens to `new_len` bytes (must be on a char boundary).
    pub fn truncate(&mut self, new_len: usize) {
        assert!(
            self.slot.as_str(self.is_heap()).is_char_boundary(new_len),
            "new_len does not lie on a char boundary"
        );
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

impl<A: Allocator> Deref for SsoStringMut<'_, A> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.slot.as_str(*self.tag)
    }
}

impl<A: Allocator + Clone> ::puroro::StringMut<A> for SsoStringMut<'_, A> {
    #[inline]
    fn set(&mut self, s: &str) {
        SsoStringMut::set(self, s);
    }

    #[inline]
    fn set_string(&mut self, s: AllocString<A>) {
        SsoStringMut::set_string(self, s);
    }

    #[inline]
    fn clear(&mut self) {
        SsoStringMut::clear(self);
    }

    #[inline]
    fn push_str(&mut self, s: &str) {
        SsoStringMut::push_str(self, s);
    }

    #[inline]
    fn push(&mut self, ch: char) {
        SsoStringMut::push(self, ch);
    }

    #[inline]
    fn truncate(&mut self, new_len: usize) {
        SsoStringMut::truncate(self, new_len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::allocator_api2::alloc::Global;

    #[test]
    fn empty_and_short_are_inline() {
        let (s, is_heap) = pack_str::<Global>("", Global);
        assert!(!is_heap);
        assert_eq!(s.as_str(false), "");
        unsafe { s.deallocate(false, &Global) };

        let (s, is_heap) = pack_str("hi", Global);
        assert!(!is_heap);
        assert_eq!(s.as_str(false), "hi");
        unsafe { s.deallocate(false, &Global) };
    }

    #[test]
    fn boundary_lengths() {
        let max_inline = "a".repeat(INLINE_CAP);
        let (s, is_heap) = pack_str(&max_inline, Global);
        assert!(!is_heap);
        assert_eq!(s.as_str(false).len(), INLINE_CAP);
        unsafe { s.deallocate(false, &Global) };

        let needs_heap = "a".repeat(INLINE_CAP + 1);
        let (s, is_heap) = pack_str(&needs_heap, Global);
        assert!(is_heap);
        assert_eq!(s.as_str(true), needs_heap);
        unsafe { s.deallocate(true, &Global) };
    }

    #[test]
    fn short_heap_is_allowed() {
        let heap = UnmanagedString::from_string(AllocString::from_str_in("xy", Global));
        let s = pack_heap(heap);
        assert_eq!(s.as_str(true), "xy");
        unsafe { s.deallocate(true, &Global) };
    }

    #[test]
    fn pack_written_demotes_short() {
        let heap = UnmanagedString::from_string(AllocString::from_str_in("xy", Global));
        let (s, is_heap) = pack_written(heap, Global);
        assert!(!is_heap);
        assert_eq!(s.as_str(false), "xy");
        unsafe { s.deallocate(false, &Global) };
    }

    #[test]
    fn clone_packed_preserves_arm() {
        let (inline, _) = pack_str("ab", Global);
        let (c, is_heap) = inline.clone_packed(false, Global);
        assert!(!is_heap);
        assert_eq!(c.as_str(false), "ab");
        unsafe {
            inline.deallocate(false, &Global);
            c.deallocate(false, &Global);
        }

        let (heap, _) = pack_str(&"z".repeat(INLINE_CAP + 2), Global);
        let (c, is_heap) = heap.clone_packed(true, Global);
        assert!(is_heap);
        assert_eq!(c.as_str(true), heap.as_str(true));
        unsafe {
            heap.deallocate(true, &Global);
            c.deallocate(true, &Global);
        }
    }
}
