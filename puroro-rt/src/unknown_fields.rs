//! Unknown-field store for [`crate::MessageCommon`].
//!
//! [`UnknownFields`] preserves unrecognized wire bytes (empty is one word).
//! [`DiscardUnknowns`] is a ZST that skips the same payloads.
//! [`RejectUnknowns`] is a ZST that fails decode on an unrecognized tag.
//! Catalog code talks to any of them through [`UnknownStore`].

use ::allocator_api2::alloc::Allocator;
use ::core::fmt;
use ::core::ops::Deref;
use ::unmanaged::{CloneIn, DeallocateIn, DefaultIn, UnmanagedBox, UnmanagedVec};

/// Message-level unknown-field policy (preserve, discard, or reject).
///
/// Decode helpers append through [`append_blob_mut`](Self::append_blob_mut).
/// Discard and reject return [`None`] there. Unrecognized tags also consult
/// [`on_unrecognized_tag`](Self::on_unrecognized_tag): reject fails, preserve
/// and discard succeed. Closed-enum diversion uses only `append_blob_mut`,
/// so a ZST store drops that value instead of failing decode.
pub trait UnknownStore<A: Allocator>: Sized {
    /// Empty store.
    fn new() -> Self;

    /// Whether this store has no unknown bytes.
    fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }

    /// Preserved trailer, or empty when discarding.
    fn as_bytes(&self) -> &[u8];

    /// Deep-copies this store into `alloc`.
    fn clone_in(&self, alloc: A) -> Self
    where
        A: Clone;

    /// Releases any heap. Must be called instead of implicit [`Drop`] when
    /// the store owns a buffer.
    ///
    /// # Safety
    ///
    /// `alloc` must own this buffer.
    unsafe fn deallocate(self, alloc: &A);

    /// Blob to append unknown wire bytes, or [`None`] to drop them.
    fn append_blob_mut(&mut self, alloc: A) -> Option<&mut UnmanagedVec<u8, A>>
    where
        A: Clone;

    /// Unrecognized field number (not a closed-enum divert).
    ///
    /// Preserve and discard return [`Ok`]. Reject returns
    /// [`DecodeError::UnrecognizedField`](::puroro::DecodeError::UnrecognizedField).
    fn on_unrecognized_tag(&self, field_number: u32) -> Result<(), ::puroro::DecodeError> {
        let _ = field_number;
        Ok(())
    }
}

/// Preserve-policy unknown fields: this message’s wire trailer.
///
/// [`Deref`]s to that blob so generated encode (`.len()`, `&store`) stays a
/// byte slice. Each message owns its own store; nested messages do not share it.
pub struct UnknownFields<A: Allocator> {
    inner: Option<UnmanagedBox<UnmanagedVec<u8, A>, A>>,
}

impl<A: Allocator> UnknownFields<A> {
    /// Empty store (no heap). Trait [`UnknownStore::new`] cannot be `const`.
    #[inline]
    const fn new() -> Self {
        Self { inner: None }
    }

    #[inline]
    fn blob(&self) -> &[u8] {
        match &self.inner {
            None => &[],
            Some(blob) => blob.as_ref(),
        }
    }

    /// Blob vec after ensuring a heap node exists (decode append).
    fn blob_vec_mut(&mut self, alloc: A) -> &mut UnmanagedVec<u8, A>
    where
        A: Clone,
    {
        self.inner
            .get_or_insert_with(|| UnmanagedBox::new_in(UnmanagedVec::new(alloc.clone()), alloc))
    }
}

impl<A: Allocator> UnknownStore<A> for UnknownFields<A> {
    #[inline]
    fn new() -> Self {
        Self::new()
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.blob()
    }

    #[inline]
    fn clone_in(&self, alloc: A) -> Self
    where
        A: Clone,
    {
        CloneIn::clone_in(self, alloc)
    }

    #[inline]
    unsafe fn deallocate(self, alloc: &A) {
        unsafe { DeallocateIn::deallocate_in(self, alloc) };
    }

    #[inline]
    fn append_blob_mut(&mut self, alloc: A) -> Option<&mut UnmanagedVec<u8, A>>
    where
        A: Clone,
    {
        Some(self.blob_vec_mut(alloc))
    }
}

/// Discard-policy unknown fields: no buffer, no round-trip.
///
/// Decode still consumes unrecognized payloads; encode / the public iterator
/// see nothing. Empty is a ZST (no word in [`crate::MessageCommon`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DiscardUnknowns;

impl<A: Allocator> UnknownStore<A> for DiscardUnknowns {
    #[inline]
    fn new() -> Self {
        Self
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &[]
    }

    #[inline]
    fn clone_in(&self, _alloc: A) -> Self
    where
        A: Clone,
    {
        Self
    }

    #[inline]
    unsafe fn deallocate(self, _alloc: &A) {}

    #[inline]
    fn append_blob_mut(&mut self, _alloc: A) -> Option<&mut UnmanagedVec<u8, A>>
    where
        A: Clone,
    {
        None
    }
}

impl Deref for DiscardUnknowns {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &[]
    }
}

/// Reject-policy unknown fields: no buffer, unrecognized tags fail decode.
///
/// A well-formed unrecognized tag becomes
/// [`DecodeError::UnrecognizedField`](::puroro::DecodeError::UnrecognizedField)
/// after its payload is consumed. Closed-enum diversion still calls
/// [`append_blob_mut`](UnknownStore::append_blob_mut), which drops the value
/// and does not fail. Empty is a ZST (no word in [`crate::MessageCommon`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RejectUnknowns;

impl<A: Allocator> UnknownStore<A> for RejectUnknowns {
    #[inline]
    fn new() -> Self {
        Self
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &[]
    }

    #[inline]
    fn clone_in(&self, _alloc: A) -> Self
    where
        A: Clone,
    {
        Self
    }

    #[inline]
    unsafe fn deallocate(self, _alloc: &A) {}

    #[inline]
    fn append_blob_mut(&mut self, _alloc: A) -> Option<&mut UnmanagedVec<u8, A>>
    where
        A: Clone,
    {
        None
    }

    #[inline]
    fn on_unrecognized_tag(&self, field_number: u32) -> Result<(), ::puroro::DecodeError> {
        Err(::puroro::DecodeError::UnrecognizedField { field_number })
    }
}

impl Deref for RejectUnknowns {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &[]
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
        self.blob()
    }
}

impl<A: Allocator> PartialEq for UnknownFields<A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.blob() == other.blob()
    }
}

impl<A: Allocator> Eq for UnknownFields<A> {}

impl<A: Allocator> fmt::Debug for UnknownFields<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnknownFields").field(&self.blob()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{DiscardUnknowns, RejectUnknowns, UnknownFields, UnknownStore};
    use crate::decode::{
        iter_unknown_fields, save_unknown_varint_field, skip_field_and_save, skip_field_and_save_in,
    };
    use ::allocator_api2::alloc::Global;
    use ::bytes::Buf;
    use ::core::mem::size_of;
    use ::protobuf_core::FieldNumber;
    use ::puroro::{DecodeError, WireType};
    use ::std::vec::Vec;
    use ::unmanaged::{CloneIn, DeallocateIn};

    fn field(n: u32) -> FieldNumber {
        FieldNumber::try_new(n).expect("field number")
    }

    #[test]
    fn empty_is_one_word() {
        assert_eq!(size_of::<UnknownFields<Global>>(), size_of::<usize>());
        let s = UnknownFields::<Global>::new();
        assert!(UnknownStore::<Global>::is_empty(&s));
        assert!(s.as_bytes().is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn discard_is_zst() {
        assert_eq!(size_of::<DiscardUnknowns>(), 0);
        let mut s = DiscardUnknowns;
        save_unknown_varint_field(field(7), 42, &mut s, Global);
        assert!(UnknownStore::<Global>::is_empty(&s));
        assert_eq!(&*s, &[] as &[u8]);
        assert_eq!(iter_unknown_fields(&s).count(), 0);
    }

    #[test]
    fn discard_skip_advances_input() {
        let mut wire: &[u8] = &[1];
        let mut s = DiscardUnknowns;
        skip_field_and_save_in(field(3), WireType::Varint, &mut wire, &mut s, Global).unwrap();
        assert!(!wire.has_remaining());
        assert_eq!(iter_unknown_fields(&s).count(), 0);
    }

    #[test]
    fn append_round_trips_blob() {
        let mut s = UnknownFields::new();
        save_unknown_varint_field(field(7), 42, &mut s, Global);
        assert_eq!(s.len(), s.as_bytes().len());
        let items: Vec<_> = iter_unknown_fields(&s).collect();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].number(), 7);
        let copy = CloneIn::clone_in(&s, Global);
        assert_eq!(copy, s);
        unsafe {
            copy.deallocate_in(&Global);
            s.deallocate_in(&Global);
        }
    }

    #[test]
    fn reject_is_zst_and_fails_unrecognized_tag() {
        assert_eq!(size_of::<RejectUnknowns>(), 0);
        let mut s = RejectUnknowns;
        let mut wire: &[u8] = &[1];
        let err = skip_field_and_save_in(field(3), WireType::Varint, &mut wire, &mut s, Global)
            .unwrap_err();
        assert_eq!(err, DecodeError::UnrecognizedField { field_number: 3 });
        assert!(!wire.has_remaining());
        assert_eq!(iter_unknown_fields(&s).count(), 0);
    }

    #[test]
    fn reject_truncated_payload_is_wire_error() {
        let mut wire: &[u8] = &[];
        let mut s = RejectUnknowns;
        let err = skip_field_and_save_in(field(3), WireType::Varint, &mut wire, &mut s, Global)
            .unwrap_err();
        assert_eq!(err, DecodeError::UnexpectedEof);
    }

    #[test]
    fn reject_closed_enum_divert_drops_without_error() {
        let mut s = RejectUnknowns;
        save_unknown_varint_field(field(1), 99, &mut s, Global);
        assert_eq!(iter_unknown_fields(&s).count(), 0);
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
