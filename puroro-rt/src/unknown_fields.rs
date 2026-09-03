//! Compact unknown-field store for [`crate::MessageCommon`].
//!
//! Empty is one word (`None`). The first append allocates a heap blob of
//! unrecognized wire bytes for this message only.

use ::allocator_api2::alloc::Allocator;
use ::core::fmt;
use ::core::ops::Deref;
use ::unmanaged::{CloneIn, DeallocateIn, DefaultIn, UnmanagedBox, UnmanagedVec};

/// Preserve-policy unknown fields: this message’s wire trailer.
///
/// [`Deref`]s to that blob so generated encode (`.len()`, `&store`) stays a
/// byte slice. Each message owns its own store; nested messages do not share it.
pub struct UnknownFields<A: Allocator> {
    inner: Option<UnmanagedBox<UnmanagedVec<u8, A>, A>>,
}

impl<A: Allocator> UnknownFields<A> {
    /// Empty store (no heap).
    #[inline]
    pub const fn new() -> Self {
        Self { inner: None }
    }

    /// Whether this store has no unknown bytes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match &self.inner {
            None => true,
            Some(blob) => blob.is_empty(),
        }
    }

    /// This message’s unknown trailer.
    #[inline]
    pub fn self_blob(&self) -> &[u8] {
        match &self.inner {
            None => &[],
            Some(blob) => blob.as_ref(),
        }
    }

    /// Deep-copies this blob into `alloc`.
    pub fn clone_in(&self, alloc: A) -> Self
    where
        A: Clone,
    {
        let Some(boxed) = &self.inner else {
            return Self::new();
        };
        Self {
            inner: Some(UnmanagedBox::new_in(
                (**boxed).clone_in(alloc.clone()),
                alloc,
            )),
        }
    }

    /// Releases the blob. Must be called instead of implicit [`Drop`] when
    /// `inner` is `Some`.
    ///
    /// # Safety
    ///
    /// `alloc` must own this buffer.
    pub unsafe fn deallocate(self, alloc: &A) {
        unsafe { self.deallocate_in(alloc) };
    }

    /// Blob vec after ensuring a heap node exists (decode append).
    pub(crate) fn self_blob_vec_mut(&mut self, alloc: A) -> &mut UnmanagedVec<u8, A>
    where
        A: Clone,
    {
        self.inner
            .get_or_insert_with(|| UnmanagedBox::new_in(UnmanagedVec::new(alloc.clone()), alloc))
    }
}

impl<A: Allocator> Default for UnknownFields<A> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Allocator> DefaultIn<A> for UnknownFields<A> {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        Self::new()
    }
}

impl<A: Allocator + Clone> CloneIn<A> for UnknownFields<A> {
    #[inline]
    fn clone_in(&self, alloc: A) -> Self {
        UnknownFields::clone_in(self, alloc)
    }
}

impl<A: Allocator> DeallocateIn<A> for UnknownFields<A> {
    #[inline]
    unsafe fn deallocate_in(self, alloc: &A) {
        let UnknownFields { inner } = self;
        if let Some(boxed) = inner {
            // SAFETY: caller owns the blob.
            unsafe { boxed.deallocate(alloc) };
        }
    }
}

impl<A: Allocator> Deref for UnknownFields<A> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.self_blob()
    }
}

impl<A: Allocator> PartialEq for UnknownFields<A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.self_blob() == other.self_blob()
    }
}

impl<A: Allocator> Eq for UnknownFields<A> {}

impl<A: Allocator> fmt::Debug for UnknownFields<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnknownFields")
            .field(&self.self_blob())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::UnknownFields;
    use crate::decode::{iter_unknown_fields, save_unknown_varint_field, skip_field_and_save};
    use ::allocator_api2::alloc::Global;
    use ::bytes::Buf;
    use ::core::mem::size_of;
    use ::protobuf_core::FieldNumber;
    use ::puroro::WireType;
    use ::std::vec::Vec;
    use ::unmanaged::DeallocateIn;

    fn field(n: u32) -> FieldNumber {
        FieldNumber::try_new(n).expect("field number")
    }

    #[test]
    fn empty_is_one_word() {
        assert_eq!(size_of::<UnknownFields<Global>>(), size_of::<usize>());
        let s = UnknownFields::<Global>::new();
        assert!(s.is_empty());
        assert!(s.self_blob().is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn append_round_trips_self_blob() {
        let mut s = UnknownFields::new();
        save_unknown_varint_field(field(7), 42, &mut s, Global);
        assert_eq!(s.len(), s.self_blob().len());
        let items: Vec<_> = iter_unknown_fields(&s).collect();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].number(), 7);
        let copy = s.clone_in(Global);
        assert_eq!(copy, s);
        unsafe {
            copy.deallocate_in(&Global);
            s.deallocate_in(&Global);
        }
    }

    #[test]
    fn skip_field_and_save_varint() {
        let mut wire: &[u8] = &[1];
        let mut s = UnknownFields::new();
        skip_field_and_save(field(3), WireType::Varint, &mut wire, &mut s, Global).unwrap();
        assert!(!wire.has_remaining());
        assert_eq!(iter_unknown_fields(&s).count(), 1);
        unsafe { s.deallocate_in(&Global) };
    }
}
