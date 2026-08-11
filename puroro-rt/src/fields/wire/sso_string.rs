//! Small-string (SSO) slot for singular `string` fields.
//!
//! The physical slot is 3 words (same as [`UnmanagedString`]): short payloads
//! live inline; longer ones use a heap [`UnmanagedString`]. Which arm is live is
//! recorded **only** in a [`MessageCommon`](crate::fields::shared::MessageCommon)
//! heap bit via [`InlineOrHeap`](crate::fields::shared::value_layout::InlineOrHeap)
//! (`true` = heap, `false` = inline) — the slot itself is an untagged union and
//! does not inspect [`UnmanagedString`]'s memory layout.
//!
//! Packing (inline vs heap) is done by crate-private helpers; callers of
//! [`SsoStringMut`] / [`InlineOrHeap`] pass logical UTF-8 and do not see arms.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::core::mem::{self, ManuallyDrop};
use ::core::ops::Deref;
use ::core::str;
use ::unmanaged::{CloneIn, DeallocateIn, DefaultIn, String as AllocString, UnmanagedString};

/// Max inline UTF-8 byte length (one byte of the 3-word slot is the length).
pub const INLINE_CAP: usize = mem::size_of::<usize>() * 3 - 1;

#[repr(C)]
#[derive(Clone, Copy)]
struct InlineBuf {
    data: [u8; INLINE_CAP],
    /// Inline UTF-8 length (`0..=INLINE_CAP`). Not a heap/inline tag.
    len: u8,
}

/// Untagged 24-byte singular `string` slot (inline buffer or heap [`UnmanagedString`]).
///
/// Arm selection requires an external `is_heap` flag ([`InlineOrHeap`] heap bit).
#[must_use = "an SsoString must be released via `deallocate`; dropping it panics"]
#[repr(C)]
pub union SsoString<A: Allocator> {
    heap: ManuallyDrop<UnmanagedString<A>>,
    inline: InlineBuf,
}

impl<A: Allocator> Drop for SsoString<A> {
    fn drop(&mut self) {
        panic!("SsoString must be released via `deallocate`, not dropped implicitly");
    }
}

const _: () = assert!(mem::size_of::<InlineBuf>() == mem::size_of::<usize>() * 3);

impl InlineBuf {
    fn empty() -> Self {
        Self {
            data: [0u8; INLINE_CAP],
            len: 0,
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() <= INLINE_CAP);
        let mut data = [0u8; INLINE_CAP];
        data[..bytes.len()].copy_from_slice(bytes);
        Self {
            data,
            len: bytes.len() as u8,
        }
    }

    fn as_str(&self) -> &str {
        let len = self.len as usize;
        debug_assert!(len <= INLINE_CAP);
        // SAFETY: constructors / mutators keep `data[..len]` as UTF-8.
        unsafe { str::from_utf8_unchecked(&self.data[..len]) }
    }
}

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
    SsoString {
        inline: InlineBuf::from_bytes(bytes),
    }
}

/// Heap arm from an [`UnmanagedString`] (any length, including short/empty).
#[inline]
pub(crate) fn pack_heap<A: Allocator>(heap: UnmanagedString<A>) -> SsoString<A> {
    SsoString {
        heap: ManuallyDrop::new(heap),
    }
}

/// Packs a written [`UnmanagedString`], demoting to inline when short.
///
/// On demote, `value` is deallocated after copying bytes into the inline arm.
pub(crate) fn pack_written<A: Allocator + Clone>(
    value: UnmanagedString<A>,
    alloc: A,
) -> (SsoString<A>, bool) {
    if value.len() <= INLINE_CAP {
        let packed = pack_inline_utf8::<A>(value.as_bytes());
        // SAFETY: `alloc` owns `value`'s buffer.
        unsafe { value.deallocate(alloc) };
        (packed, false)
    } else {
        (pack_heap(value), true)
    }
}

impl<A: Allocator> SsoString<A> {
    /// Empty inline string slot (caller must keep the heap bit clear / inline).
    #[inline]
    pub fn empty_inline() -> Self {
        Self {
            inline: InlineBuf::empty(),
        }
    }

    pub fn as_str(&self, is_heap: bool) -> &str {
        if is_heap {
            // SAFETY: caller affirms the heap arm is live.
            unsafe { &self.heap }
        } else {
            // SAFETY: caller affirms the inline arm is live.
            let inline = unsafe { &self.inline };
            inline.as_str()
        }
    }

    #[inline]
    pub fn len(&self, is_heap: bool) -> usize {
        self.as_str(is_heap).len()
    }

    #[inline]
    pub fn is_empty(&self, is_heap: bool) -> bool {
        self.len(is_heap) == 0
    }

    /// Clones this slot, preserving the arm selected by `is_heap`.
    pub fn clone_packed(&self, is_heap: bool, alloc: A) -> (Self, bool)
    where
        A: Clone,
    {
        if is_heap {
            // SAFETY: caller affirms the heap arm is live.
            let heap = unsafe { &*self.heap };
            (pack_heap(CloneIn::clone_in(heap, alloc)), true)
        } else {
            (pack_inline_utf8(self.as_str(false).as_bytes()), false)
        }
    }

    /// # Safety
    ///
    /// `is_heap` must match the live arm. `alloc` must own any live heap buffer.
    pub unsafe fn deallocate(mut self, is_heap: bool, alloc: A)
    where
        A: Clone,
    {
        if is_heap {
            // SAFETY: heap arm live; `ManuallyDrop::take` prevents double-free.
            let heap = unsafe { ManuallyDrop::take(&mut self.heap) };
            mem::forget(self);
            unsafe { heap.deallocate(alloc) };
        } else {
            mem::forget(self);
        }
    }

    /// Replaces `self` with a packed slot, freeing the previous arm when needed.
    ///
    /// # Safety
    ///
    /// `old_is_heap` must match the live arm of `self`. `alloc` must own any
    /// live heap buffer in `self`.
    pub unsafe fn replace_packed(
        &mut self,
        new: Self,
        _new_is_heap: bool,
        old_is_heap: bool,
        alloc: A,
    ) where
        A: Clone,
    {
        let old = mem::replace(self, new);
        unsafe { old.deallocate(old_is_heap, alloc) };
    }

    /// # Safety
    ///
    /// Heap arm must be live.
    unsafe fn heap_mut(&mut self) -> &mut UnmanagedString<A> {
        unsafe { &mut self.heap }
    }

    /// Promotes an inline slot to heap. No-op if already heap.
    ///
    /// # Safety
    ///
    /// `is_heap` must match the live arm before the call.
    unsafe fn promote_to_heap(&mut self, is_heap: bool, alloc: A)
    where
        A: Clone,
    {
        if is_heap {
            return;
        }
        // Copy inline bytes before replacing the union arm.
        let (len, data) = {
            // SAFETY: caller affirms inline arm is live.
            let inline = unsafe { &self.inline };
            (inline.len as usize, inline.data)
        };
        debug_assert!(len <= INLINE_CAP);
        // SAFETY: inline payload is UTF-8.
        let s = unsafe { str::from_utf8_unchecked(&data[..len]) };
        let new_slot = pack_heap(UnmanagedString::from_string(AllocString::from_str_in(
            s, alloc,
        )));
        let prev = mem::replace(self, new_slot);
        mem::forget(prev);
    }
}

impl<A: Allocator> DefaultIn<A> for SsoString<A> {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        Self::empty_inline()
    }
}

/// Blind `DeallocateIn` / `CloneIn` are not meaningful for an untagged SSO slot.
///
/// Message Drop / field clone must go through [`InlineOrHeap`] via
/// [`SsoString::deallocate`] / [`SsoString::clone_packed`] with the heap bit.
/// These impls exist only to satisfy [`ValueSlot`](crate::fields::shared::value_slot::ValueSlot)
/// bounds used by mut views (SSO mutators bypass `DeallocateIn` on the slot).
impl<A: Allocator + Clone> DeallocateIn<A> for SsoString<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        panic!("SsoString must be deallocated with is_heap from MessageCommon HEAP_BIT");
    }
}

impl<A: Allocator + Clone> CloneIn<A> for SsoString<A> {
    #[inline]
    fn clone_in(&self, _alloc: A) -> Self {
        panic!("SsoString must be cloned with is_heap from MessageCommon HEAP_BIT");
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
                .replace_packed(new, new_is_heap, old_is_heap, alloc)
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
                .replace_packed(new, new_is_heap, old_is_heap, alloc)
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
                .replace_packed(SsoString::empty_inline(), SSO_INLINE, old_is_heap, alloc)
        };
        self.set_heap(SSO_INLINE);
    }

    /// Appends `s`, promoting to heap when the result would exceed [`INLINE_CAP`].
    pub fn push_str(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        if !self.is_heap() {
            let cur_len = self.slot.len(false);
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
        unsafe { s.deallocate(false, Global) };

        let (s, is_heap) = pack_str("hi", Global);
        assert!(!is_heap);
        assert_eq!(s.as_str(false), "hi");
        unsafe { s.deallocate(false, Global) };
    }

    #[test]
    fn boundary_lengths() {
        let max_inline = "a".repeat(INLINE_CAP);
        let (s, is_heap) = pack_str(&max_inline, Global);
        assert!(!is_heap);
        assert_eq!(s.as_str(false).len(), INLINE_CAP);
        unsafe { s.deallocate(false, Global) };

        let needs_heap = "a".repeat(INLINE_CAP + 1);
        let (s, is_heap) = pack_str(&needs_heap, Global);
        assert!(is_heap);
        assert_eq!(s.as_str(true), needs_heap);
        unsafe { s.deallocate(true, Global) };
    }

    #[test]
    fn short_heap_is_allowed() {
        let heap = UnmanagedString::from_string(AllocString::from_str_in("xy", Global));
        let s = pack_heap(heap);
        assert_eq!(s.as_str(true), "xy");
        unsafe { s.deallocate(true, Global) };
    }

    #[test]
    fn pack_written_demotes_short() {
        let heap = UnmanagedString::from_string(AllocString::from_str_in("xy", Global));
        let (s, is_heap) = pack_written(heap, Global);
        assert!(!is_heap);
        assert_eq!(s.as_str(false), "xy");
        unsafe { s.deallocate(false, Global) };
    }

    #[test]
    fn clone_packed_preserves_arm() {
        let (inline, _) = pack_str("ab", Global);
        let (c, is_heap) = inline.clone_packed(false, Global);
        assert!(!is_heap);
        assert_eq!(c.as_str(false), "ab");
        unsafe {
            inline.deallocate(false, Global);
            c.deallocate(false, Global);
        }

        let (heap, _) = pack_str(&"z".repeat(INLINE_CAP + 2), Global);
        let (c, is_heap) = heap.clone_packed(true, Global);
        assert!(is_heap);
        assert_eq!(c.as_str(true), heap.as_str(true));
        unsafe {
            heap.deallocate(true, Global);
            c.deallocate(true, Global);
        }
    }
}
