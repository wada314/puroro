//! Compact unknown-field store for [`crate::MessageCommon`].
//!
//! Empty is one word (`None`). The first append or [`UnknownFields::child_mut`]
//! allocates a node: this message’s wire blob plus field-number → child stores
//! (inlined nested messages, later).

use ::allocator_api2::alloc::Allocator;
use ::core::fmt;
use ::core::ops::Deref;
use ::std::vec::Vec;
use ::unmanaged::{CloneIn, DeallocateIn, DefaultIn, UnmanagedBox, UnmanagedVec};

/// Preserve-policy unknown fields: self blob + optional inlined-child subtrees.
///
/// [`Deref`]s to the **self** wire blob so generated encode (`.len()`, `&store`)
/// stays unchanged. Child subtrees are not part of that slice.
pub struct UnknownFields<A: Allocator> {
    inner: Option<UnmanagedBox<UnknownNode<A>, A>>,
}

struct UnknownNode<A: Allocator> {
    self_blob: UnmanagedVec<u8, A>,
    /// Sorted by proto field number.
    children: UnmanagedVec<ChildEntry<A>, A>,
}

struct ChildEntry<A: Allocator> {
    number: u32,
    store: UnknownFields<A>,
}

impl<A: Allocator> UnknownFields<A> {
    /// Empty store (no heap).
    #[inline]
    pub const fn new() -> Self {
        Self { inner: None }
    }

    /// Whether this node and every child subtree are empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match &self.inner {
            None => true,
            Some(node) => node.self_blob.is_empty() && node.children.is_empty(),
        }
    }

    /// This message’s unknown trailer (not child subtrees).
    #[inline]
    pub fn self_blob(&self) -> &[u8] {
        match &self.inner {
            None => &[],
            Some(node) => &node.self_blob,
        }
    }

    /// Child store for inlined field `number`, if an entry exists.
    #[inline]
    pub fn child(&self, number: u32) -> Option<&Self> {
        let node = self.inner.as_ref()?;
        let i = node
            .children
            .binary_search_by(|e| e.number.cmp(&number))
            .ok()?;
        Some(&node.children[i].store)
    }

    /// Child store for inlined field `number`, creating an empty entry if needed.
    pub fn child_mut(&mut self, number: u32, alloc: A) -> &mut Self
    where
        A: Clone,
    {
        let node = self.ensure_node(alloc.clone());
        match node.children.binary_search_by(|e| e.number.cmp(&number)) {
            Ok(i) => &mut node.children[i].store,
            Err(i) => {
                // SAFETY: `alloc` owns `children`.
                let mut g = unsafe { node.children.with_alloc(alloc) };
                g.insert(
                    i,
                    ChildEntry {
                        number,
                        store: Self::new(),
                    },
                );
                drop(g);
                &mut node.children[i].store
            }
        }
    }

    /// Drops the subtree for inlined field `number`, if present.
    pub fn remove_child(&mut self, number: u32, alloc: &A)
    where
        A: Clone,
    {
        let Some(node) = self.inner.as_mut() else {
            return;
        };
        let Ok(i) = node.children.binary_search_by(|e| e.number.cmp(&number)) else {
            return;
        };
        // SAFETY: `alloc.clone()` is interchangeable with the owner of `children`.
        let mut g = unsafe { node.children.with_alloc(alloc.clone()) };
        let child = g.remove(i);
        drop(g);
        // SAFETY: same allocator owns the child tree.
        unsafe { child.store.deallocate_in(alloc) };
        self.compact_if_empty(alloc);
    }

    /// Self-blob + child map (field number order) for `PartialEq`.
    pub fn eq_tree(&self, other: &Self) -> bool {
        if self.self_blob() != other.self_blob() {
            return false;
        }
        let empty: &[ChildEntry<A>] = &[];
        let a = self
            .inner
            .as_ref()
            .map(|n| n.children.as_ref())
            .unwrap_or(empty);
        let b = other
            .inner
            .as_ref()
            .map(|n| n.children.as_ref())
            .unwrap_or(empty);
        if a.len() != b.len() {
            return false;
        }
        a.iter()
            .zip(b.iter())
            .all(|(x, y)| x.number == y.number && x.store.eq_tree(&y.store))
    }

    /// Deep-copies this tree into `alloc`.
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

    /// Releases the tree. Must be called instead of implicit [`Drop`] when
    /// `inner` is `Some`.
    ///
    /// # Safety
    ///
    /// `alloc` must own every buffer in this tree.
    pub unsafe fn deallocate(self, alloc: &A) {
        unsafe { self.deallocate_in(alloc) };
    }

    fn ensure_node(&mut self, alloc: A) -> &mut UnknownNode<A>
    where
        A: Clone,
    {
        &mut *self.inner.get_or_insert_with(|| {
            UnmanagedBox::new_in(
                UnknownNode {
                    self_blob: UnmanagedVec::new(alloc.clone()),
                    children: UnmanagedVec::new(alloc.clone()),
                },
                alloc,
            )
        })
    }

    fn compact_if_empty(&mut self, alloc: &A) {
        let Some(node) = self.inner.as_ref() else {
            return;
        };
        if !node.self_blob.is_empty() || !node.children.is_empty() {
            return;
        }
        let boxed = self.inner.take().expect("node");
        // SAFETY: `alloc` owns the empty node.
        unsafe { boxed.deallocate(alloc) };
    }

    /// Self blob vec after ensuring a node exists (decode append).
    pub(crate) fn self_blob_vec_mut(&mut self, alloc: A) -> &mut UnmanagedVec<u8, A>
    where
        A: Clone,
    {
        &mut self.ensure_node(alloc).self_blob
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
            // SAFETY: caller owns the tree.
            unsafe { boxed.deallocate(alloc) };
        }
    }
}

impl<A: Allocator + Clone> CloneIn<A> for UnknownNode<A> {
    fn clone_in(&self, alloc: A) -> Self {
        Self {
            self_blob: self.self_blob.clone_in(alloc.clone()),
            children: self.children.clone_in(alloc),
        }
    }
}

impl<A: Allocator> DeallocateIn<A> for UnknownNode<A> {
    unsafe fn deallocate_in(self, alloc: &A) {
        let UnknownNode {
            self_blob,
            children,
        } = self;
        // SAFETY: caller owns both buffers and nested stores.
        unsafe {
            self_blob.deallocate(alloc);
            children.deallocate(alloc);
        }
    }
}

impl<A: Allocator> DeallocateIn<A> for ChildEntry<A> {
    #[inline]
    unsafe fn deallocate_in(self, alloc: &A) {
        unsafe { self.store.deallocate_in(alloc) };
    }
}

impl<A: Allocator + Clone> CloneIn<A> for ChildEntry<A> {
    #[inline]
    fn clone_in(&self, alloc: A) -> Self {
        Self {
            number: self.number,
            store: self.store.clone_in(alloc),
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
        self.eq_tree(other)
    }
}

impl<A: Allocator> Eq for UnknownFields<A> {}

impl<A: Allocator> fmt::Debug for UnknownFields<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children: Vec<u32> = self
            .inner
            .as_ref()
            .map(|n| n.children.iter().map(|c| c.number).collect())
            .unwrap_or_default();
        f.debug_struct("UnknownFields")
            .field("self_blob", &self.self_blob())
            .field("children", &children)
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

    #[test]
    fn child_unknowns_are_not_in_parent_iterator() {
        let mut parent = UnknownFields::new();
        save_unknown_varint_field(field(1), 1, &mut parent, Global);
        save_unknown_varint_field(field(9), 9, parent.child_mut(22, Global), Global);
        assert_eq!(iter_unknown_fields(&parent).count(), 1);
        assert_eq!(iter_unknown_fields(parent.child(22).unwrap()).count(), 1);
        let other = {
            let mut p = UnknownFields::new();
            save_unknown_varint_field(field(1), 1, &mut p, Global);
            p
        };
        assert_ne!(parent, other);
        unsafe {
            other.deallocate_in(&Global);
            parent.deallocate_in(&Global);
        }
    }

    #[test]
    fn remove_child_compacts_to_empty() {
        let mut parent = UnknownFields::new();
        save_unknown_varint_field(field(2), 2, parent.child_mut(5, Global), Global);
        assert!(!parent.is_empty());
        parent.remove_child(5, &Global);
        assert!(parent.is_empty());
    }

    #[test]
    fn nested_child_path() {
        let mut root = UnknownFields::new();
        save_unknown_varint_field(
            field(8),
            8,
            root.child_mut(1, Global).child_mut(2, Global),
            Global,
        );
        assert!(root.child(1).unwrap().child(2).is_some());
        assert_eq!(iter_unknown_fields(&root).count(), 0);
        let cloned = root.clone_in(Global);
        assert_eq!(cloned, root);
        unsafe {
            cloned.deallocate_in(&Global);
            root.deallocate_in(&Global);
        }
    }
}
