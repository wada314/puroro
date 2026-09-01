//! Bound-view wrappers for `repeated message` when `M: `[`NestedMessage`].
//!
//! Storage stays `Element = M` (each element owns its `MessageCommon`).
//! Getters project [`NestedMessage::View`] / [`NestedMessage::Mut`].

use ::allocator_api2::alloc::Allocator;
use ::puroro::{Message, RepeatedMessageMut, RepeatedRef};
use ::unmanaged::vec::VecGuard;

use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::repeated_element::{RepeatedElement, RepeatedElementMerge};
use crate::fields::wire::shared_message::NestedMessage;
use crate::message_encode::MessageEncode;
use crate::message_merge::MessageMerge;

/// Shared projection of a repeated nested-message field.
pub struct RepeatedMessageRef<'a, M: NestedMessage> {
    elems: &'a [M],
}

impl<'a, M: NestedMessage> RepeatedMessageRef<'a, M> {
    #[inline]
    pub(crate) fn new(elems: &'a [M]) -> Self {
        Self { elems }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.elems.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<M::View<'a>> {
        self.elems.get(index).map(NestedMessage::as_view)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = M::View<'a>> + 'a {
        self.elems.iter().map(NestedMessage::as_view)
    }
}

impl<'a, M: NestedMessage> RepeatedRef<M::View<'a>> for RepeatedMessageRef<'a, M> {
    #[inline]
    fn len(&self) -> usize {
        RepeatedMessageRef::len(self)
    }

    #[inline]
    fn get(&self, index: usize) -> Option<M::View<'a>> {
        RepeatedMessageRef::get(self, index)
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = M::View<'a>> + '_ {
        RepeatedMessageRef::iter(self)
    }
}

/// Growable mutator over repeated nested-message elements (`M: NestedMessage`).
///
/// [`push`](Self::push) / [`get_mut`](Self::get_mut) return [`NestedMessage::Mut`],
/// not `&mut M`.
pub struct RepeatedMessagesMut<'a, M, A>
where
    M: NestedMessage<Alloc = A>,
    A: Allocator,
{
    values: VecGuard<'a, M, A>,
}

impl<'a, M, A> RepeatedMessagesMut<'a, M, A>
where
    M: NestedMessage<Alloc = A>,
    A: Allocator,
{
    #[inline]
    pub(crate) fn new(values: VecGuard<'a, M, A>) -> Self {
        Self { values }
    }
}

impl<'a, M, A> RepeatedMessagesMut<'a, M, A>
where
    M: NestedMessage<Alloc = A>
        + Message<Alloc = A>
        + MessageEncode
        + MessageMerge
        + ::unmanaged::DeallocateIn<A>,
    A: Allocator + Clone,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends a default message and returns a mutator for it.
    #[inline]
    pub fn push(&mut self) -> M::Mut<'_> {
        let alloc = self.values.allocator().clone();
        self.values
            .push(ProtoMessage::<M>::default_element(alloc.clone()));
        let elem = self.values.last_mut().expect("element present after push");
        NestedMessage::as_mut(elem)
    }

    /// Mutable handle for the element at `index`, or `None` if out of range.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<M::Mut<'_>> {
        self.values.get_mut(index).map(NestedMessage::as_mut)
    }

    /// Removes all elements (heap payloads are released).
    #[inline]
    pub fn clear(&mut self) {
        let alloc = self.values.allocator().clone();
        while let Some(elem) = self.values.pop() {
            // SAFETY: same allocator ownership as `RepeatedFieldMut::clear`.
            unsafe { ProtoMessage::<M>::deallocate_element(elem, &alloc) };
        }
    }

    /// Removes the last element (and releases it). Returns whether one existed.
    #[inline]
    pub fn pop(&mut self) -> bool {
        let alloc = self.values.allocator().clone();
        match self.values.pop() {
            Some(elem) => {
                // SAFETY: same allocator ownership as `clear`.
                unsafe { ProtoMessage::<M>::deallocate_element(elem, &alloc) };
                true
            }
            None => false,
        }
    }
}

impl<'a, M, A> RepeatedMessageMut for RepeatedMessagesMut<'a, M, A>
where
    M: NestedMessage<Alloc = A>
        + Message<Alloc = A>
        + MessageEncode
        + MessageMerge
        + ::unmanaged::DeallocateIn<A>,
    A: Allocator + Clone,
{
    type Mut<'m>
        = M::Mut<'m>
    where
        Self: 'm;

    #[inline]
    fn len(&self) -> usize {
        RepeatedMessagesMut::len(self)
    }

    #[inline]
    fn push(&mut self) -> M::Mut<'_> {
        RepeatedMessagesMut::push(self)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<M::Mut<'_>> {
        RepeatedMessagesMut::get_mut(self, index)
    }

    #[inline]
    fn clear(&mut self) {
        RepeatedMessagesMut::clear(self);
    }

    #[inline]
    fn pop(&mut self) -> bool {
        RepeatedMessagesMut::pop(self)
    }
}
