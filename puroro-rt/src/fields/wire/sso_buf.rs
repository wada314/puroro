//! Shared untagged 3-word SSO slot (inline buffer or heap arm).
//!
//! [`SsoString`](super::sso_string::SsoString) and
//! [`SsoBytes`](super::sso_bytes::SsoBytes) are instantiations of this union.
//! Which arm is live is recorded **only** in a
//! [`MessageCommon`](crate::fields::shared::MessageCommon) heap bit via
//! [`InlineOrHeap`](crate::fields::shared::value_layout::InlineOrHeap)
//! (`true` = heap, `false` = inline).

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::mem::{self, ManuallyDrop};
use ::core::str;
use ::unmanaged::{CloneIn, DefaultIn, String as AllocString, UnmanagedString, UnmanagedVec};

/// Max inline byte length (one byte of the 3-word slot is the length).
pub const INLINE_CAP: usize = mem::size_of::<usize>() * 3 - 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct InlineBuf {
    pub(crate) data: [u8; INLINE_CAP],
    /// Inline length (`0..=INLINE_CAP`). Not a heap/inline tag.
    pub(crate) len: u8,
}

const _: () = assert!(mem::size_of::<InlineBuf>() == mem::size_of::<usize>() * 3);

impl InlineBuf {
    pub(crate) fn empty() -> Self {
        Self {
            data: [0u8; INLINE_CAP],
            len: 0,
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() <= INLINE_CAP);
        let mut data = [0u8; INLINE_CAP];
        data[..bytes.len()].copy_from_slice(bytes);
        Self {
            data,
            len: bytes.len() as u8,
        }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        let len = self.len as usize;
        debug_assert!(len <= INLINE_CAP);
        &self.data[..len]
    }
}

/// Heap-arm operations required by [`SsoBuf`].
pub(crate) trait SsoHeap<A: Allocator>: Sized {
    fn as_slice(&self) -> &[u8];

    /// # Safety
    ///
    /// `alloc` must own this value's buffer.
    unsafe fn deallocate_heap(self, alloc: &A);

    fn clone_heap(&self, alloc: A) -> Self
    where
        A: Clone;

    fn from_slice(bytes: &[u8], alloc: A) -> Self
    where
        A: Clone;
}

impl<A: Allocator> SsoHeap<A> for UnmanagedString<A> {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.as_bytes()
    }

    #[inline]
    unsafe fn deallocate_heap(self, alloc: &A) {
        // SAFETY: forwarded to the caller's `alloc` obligation.
        unsafe { self.deallocate(alloc) };
    }

    #[inline]
    fn clone_heap(&self, alloc: A) -> Self
    where
        A: Clone,
    {
        CloneIn::clone_in(self, alloc)
    }

    #[inline]
    fn from_slice(bytes: &[u8], alloc: A) -> Self
    where
        A: Clone,
    {
        // SAFETY: string SSO only stores UTF-8 in the inline arm.
        let s = unsafe { str::from_utf8_unchecked(bytes) };
        UnmanagedString::from_string(AllocString::from_str_in(s, alloc))
    }
}

impl<A: Allocator> SsoHeap<A> for UnmanagedVec<u8, A> {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self
    }

    #[inline]
    unsafe fn deallocate_heap(self, alloc: &A) {
        // SAFETY: forwarded to the caller's `alloc` obligation.
        unsafe { self.deallocate(alloc) };
    }

    #[inline]
    fn clone_heap(&self, alloc: A) -> Self
    where
        A: Clone,
    {
        self.clone_in(alloc)
    }

    #[inline]
    fn from_slice(bytes: &[u8], alloc: A) -> Self
    where
        A: Clone,
    {
        let mut vec = AllocVec::with_capacity_in(bytes.len(), alloc);
        vec.extend_from_slice(bytes);
        UnmanagedVec::from_vec(vec)
    }
}

/// Untagged 24-byte singular LEN slot (inline buffer or heap `H`).
///
/// Arm selection requires an external `is_heap` flag ([`InlineOrHeap`] heap bit).
///
/// [`InlineOrHeap`]: crate::fields::shared::value_layout::InlineOrHeap
#[must_use = "an SSO slot must be released via `deallocate`; dropping it panics"]
#[repr(C)]
pub union SsoBuf<H> {
    pub(crate) heap: ManuallyDrop<H>,
    pub(crate) inline: InlineBuf,
}

impl<H> Drop for SsoBuf<H> {
    fn drop(&mut self) {
        panic!("SSO slot must be released via `deallocate`, not dropped implicitly");
    }
}

/// Inline arm from bytes (`len <= INLINE_CAP`).
#[inline]
pub(crate) fn pack_inline<H>(bytes: &[u8]) -> SsoBuf<H> {
    SsoBuf {
        inline: InlineBuf::from_bytes(bytes),
    }
}

/// Heap arm from an already-built heap value (any length, including short/empty).
#[inline]
pub(crate) fn pack_heap<H>(heap: H) -> SsoBuf<H> {
    SsoBuf {
        heap: ManuallyDrop::new(heap),
    }
}

/// Packs a written heap value, demoting to inline when short.
///
/// On demote, `value` is deallocated after copying bytes into the inline arm.
pub(crate) fn pack_written<H, A: Allocator + Clone>(value: H, alloc: A) -> (SsoBuf<H>, bool)
where
    H: SsoHeap<A>,
{
    if value.as_slice().len() <= INLINE_CAP {
        let packed = pack_inline::<H>(value.as_slice());
        // SAFETY: `alloc` owns `value`'s buffer.
        unsafe { value.deallocate_heap(&alloc) };
        (packed, false)
    } else {
        (pack_heap(value), true)
    }
}

impl<H> SsoBuf<H> {
    /// Empty inline slot (caller must keep the heap bit clear / inline).
    #[inline]
    pub fn empty_inline() -> Self {
        Self {
            inline: InlineBuf::empty(),
        }
    }

    pub(crate) fn as_bytes<A: Allocator>(&self, is_heap: bool) -> &[u8]
    where
        H: SsoHeap<A>,
    {
        if is_heap {
            // SAFETY: caller affirms the heap arm is live.
            unsafe { self.heap.as_slice() }
        } else {
            // SAFETY: caller affirms the inline arm is live.
            unsafe { self.inline.as_bytes() }
        }
    }

    #[inline]
    pub(crate) fn len<A: Allocator>(&self, is_heap: bool) -> usize
    where
        H: SsoHeap<A>,
    {
        self.as_bytes::<A>(is_heap).len()
    }

    #[inline]
    pub(crate) fn is_empty<A: Allocator>(&self, is_heap: bool) -> bool
    where
        H: SsoHeap<A>,
    {
        self.len::<A>(is_heap) == 0
    }

    /// Clones this slot, preserving the arm selected by `is_heap`.
    pub(crate) fn clone_packed<A: Allocator + Clone>(&self, is_heap: bool, alloc: A) -> (Self, bool)
    where
        H: SsoHeap<A>,
    {
        if is_heap {
            // SAFETY: caller affirms the heap arm is live.
            let heap = unsafe { &*self.heap };
            (pack_heap(heap.clone_heap(alloc)), true)
        } else {
            (pack_inline(self.as_bytes::<A>(false)), false)
        }
    }

    /// # Safety
    ///
    /// `is_heap` must match the live arm. `alloc` must own any live heap buffer.
    pub(crate) unsafe fn deallocate<A: Allocator>(mut self, is_heap: bool, alloc: &A)
    where
        H: SsoHeap<A>,
    {
        if is_heap {
            // SAFETY: heap arm live; `ManuallyDrop::take` prevents double-free.
            let heap = unsafe { ManuallyDrop::take(&mut self.heap) };
            mem::forget(self);
            unsafe { heap.deallocate_heap(alloc) };
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
    pub(crate) unsafe fn replace_packed<A: Allocator>(
        &mut self,
        new: Self,
        _new_is_heap: bool,
        old_is_heap: bool,
        alloc: &A,
    ) where
        H: SsoHeap<A>,
    {
        let old = mem::replace(self, new);
        unsafe { old.deallocate(old_is_heap, alloc) };
    }

    /// # Safety
    ///
    /// Heap arm must be live.
    pub(crate) unsafe fn heap_mut(&mut self) -> &mut H {
        unsafe { &mut self.heap }
    }

    /// Promotes an inline slot to heap. No-op if already heap.
    ///
    /// # Safety
    ///
    /// `is_heap` must match the live arm before the call.
    pub(crate) unsafe fn promote_to_heap<A: Allocator + Clone>(&mut self, is_heap: bool, alloc: A)
    where
        H: SsoHeap<A>,
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
        let new_slot = pack_heap(H::from_slice(&data[..len], alloc));
        let prev = mem::replace(self, new_slot);
        mem::forget(prev);
    }
}

impl<H, A: Allocator> DefaultIn<A> for SsoBuf<H> {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        Self::empty_inline()
    }
}
