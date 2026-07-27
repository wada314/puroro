//! Unified repeated field wrapper — generic over element marker and encode policy.
//!
//! Elements are stored in an allocator-less [`UnmanagedVec`] wrapped in
//! [`ManuallyDrop`]. Growth and release borrow the message allocator.

use ::core::fmt::{Debug, Formatter, Result as FmtResult};
use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::unmanaged::CloneIn;
use ::unmanaged::UnmanagedVec;
use ::unmanaged::vec::VecGuard;

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::fields::shared::field_inspect::{FieldCloneIn, FieldDebug, FieldEncode, FieldPartialEq};
use crate::fields::shared::{FieldDeallocate, MessageCommon, PresenceBits};
use crate::fields::wire::repeated_element::{
    RepeatedElement, RepeatedElementMerge, RepeatedElementMut, RepeatedVecMut,
};

use super::container::RepeatedElementsMut;
use super::encoding::RepeatedEncoding;

/// Repeated field parametrised by type marker `T`, encode policy `E`, and allocator `A`.
///
/// Parameter order: `T`, `E`, `FIELD`, `A`.
pub struct RepeatedField<T, E, const FIELD: u32, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
{
    values: ManuallyDrop<UnmanagedVec<T::Element<A>, A>>,
    _encoding: PhantomData<E>,
}

impl<T, E, const FIELD: u32, A> RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            values: ManuallyDrop::new(UnmanagedVec::new(alloc)),
            _encoding: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T::Element<A>] {
        &self.values
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn encoded_len<Pb>(&self, _common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
    {
        if self.values.is_empty() {
            0
        } else {
            E::encoded_len(FIELD, self.as_slice())
        }
    }

    pub fn encode_raw<Pb, B: BufMut>(&self, _common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
    {
        if !self.values.is_empty() {
            E::encode(FIELD, self.as_slice(), buf);
        }
    }

    /// Binds this field to `common` for read access.
    #[inline]
    pub fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> RepeatedFieldRef<'a, T, E, FIELD, A, Pb> {
        RepeatedFieldRef::new(self, common)
    }

    /// Binds this field to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> RepeatedFieldMut<'f, 'c, T, E, FIELD, A, Pb> {
        RepeatedFieldMut::new(self, common)
    }

    /// Deep-copies elements into `alloc`. `common` is unused (kept for symmetry
    /// with singular / oneof `clone_in`).
    #[inline]
    pub fn clone_in<Pb>(&self, _common: &MessageCommon<Pb, A>, alloc: A) -> Self
    where
        Pb: PresenceBits,
        T::Element<A>: CloneIn<A>,
    {
        Self {
            values: ManuallyDrop::new(self.values.clone_in(alloc)),
            _encoding: PhantomData,
        }
    }
}

impl<T, E, const FIELD: u32, A, Pb> FieldDeallocate<Pb, A> for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    /// Releases every element (when heap-backed) and the backing buffer.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        // SAFETY: called once; owned clones of the message allocator own the
        // buffer and every element.
        let alloc = common.alloc.clone();
        let mut v = unsafe { ManuallyDrop::take(&mut self.values) };
        {
            let mut g = unsafe { v.with_alloc(alloc.clone()) };
            while let Some(elem) = g.pop() {
                unsafe { T::deallocate_element(elem, alloc.clone()) };
            }
        }
        // Elements were already released above; free the empty buffer only.
        unsafe { v.deallocate_buffer(alloc) };
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a repeated field to its message common state.
pub struct RepeatedFieldRef<
    'a,
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    const FIELD: u32,
    A: Allocator + Clone,
    Pb: PresenceBits,
> {
    field: &'a RepeatedField<T, E, FIELD, A>,
    /// Bound for symmetry with [`RepeatedFieldMut`]; unused by current getters.
    #[allow(dead_code)]
    common: &'a MessageCommon<Pb, A>,
    _encoding: PhantomData<E>,
}

impl<'a, T, E, const FIELD: u32, A, Pb> RepeatedFieldRef<'a, T, E, FIELD, A, Pb>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn new(field: &'a RepeatedField<T, E, FIELD, A>, common: &'a MessageCommon<Pb, A>) -> Self {
        Self {
            field,
            common,
            _encoding: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(self) -> &'a [T::Element<A>] {
        self.field.as_slice()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.field.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a repeated field to its message common state.
pub struct RepeatedFieldMut<
    'f,
    'c,
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    const FIELD: u32,
    A: Allocator + Clone,
    Pb: PresenceBits,
> {
    field: &'f mut RepeatedField<T, E, FIELD, A>,
    common: &'c mut MessageCommon<Pb, A>,
    _encoding: PhantomData<E>,
}

impl<'f, 'c, T, E, const FIELD: u32, A, Pb> RepeatedFieldMut<'f, 'c, T, E, FIELD, A, Pb>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn new(
        field: &'f mut RepeatedField<T, E, FIELD, A>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self {
            field,
            common,
            _encoding: PhantomData,
        }
    }

    /// Growable handle over vec-mutable elements (`repeated int32`, message, …).
    pub fn values_mut(self) -> VecGuard<'f, T::Element<A>, A>
    where
        T: RepeatedVecMut,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        unsafe { self.field.values.with_alloc(alloc) }
    }

    /// Minimal container mutator ([`RepeatedContainerMut`](super::container::RepeatedContainerMut)).
    ///
    /// Works for string / bytes as well as scalars and messages: [`push`](super::container::RepeatedContainerMut::push)
    /// appends a default element and returns a singular-style mut handle.
    pub fn container_mut(self) -> RepeatedElementsMut<'f, T, A>
    where
        T: RepeatedElementMut + RepeatedElementMerge<A>,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        RepeatedElementsMut::new(unsafe { self.field.values.with_alloc(alloc) })
    }

    /// Empties the vector (keeps capacity). Heap elements are freed first.
    pub fn clear(self) {
        let alloc = self.common.alloc.clone();
        // SAFETY: owned clones of the message allocator own this vector's buffer
        // and every element.
        let mut g = unsafe { self.field.values.with_alloc(alloc.clone()) };
        while let Some(elem) = g.pop() {
            unsafe { T::deallocate_element(elem, alloc.clone()) };
        }
    }

    /// Merges one wire occurrence — appends element(s).
    pub fn merge<B: Buf>(
        self,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        T: RepeatedElementMerge<A>,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc.clone()) };
        T::merge_occurrence(wire_type, buf, alloc, depth, |elem| {
            g.push(elem);
        })
    }
}

impl<T, E, const FIELD: u32, A, Pb> FieldPartialEq<Pb, A> for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
    T::Element<A>: PartialEq,
{
    #[inline]
    fn field_eq(
        &self,
        _common: &MessageCommon<Pb, A>,
        other: &Self,
        _other_common: &MessageCommon<Pb, A>,
    ) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T, E, const FIELD: u32, A, Pb> FieldDebug<Pb, A> for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
    T::Element<A>: Debug,
{
    #[inline]
    fn fmt_debug(&self, _common: &MessageCommon<Pb, A>, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self.as_slice(), f)
    }
}

impl<T, E, const FIELD: u32, A, Pb> FieldEncode<Pb, A> for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
{
    #[inline]
    fn wire_encoded_len(&self, common: &MessageCommon<Pb, A>) -> usize {
        self.encoded_len(common)
    }

    #[inline]
    fn wire_encode_raw<B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B) {
        self.encode_raw(common, buf);
    }
}

impl<T, E, const FIELD: u32, A, Pb> FieldCloneIn<Pb, A> for RepeatedField<T, E, FIELD, A>
where
    T: RepeatedElement,
    E: RepeatedEncoding<T, A>,
    A: Allocator + Clone,
    Pb: PresenceBits,
    T::Element<A>: CloneIn<A>,
{
    #[inline]
    fn clone_field(&self, common: &MessageCommon<Pb, A>, alloc: A) -> Self {
        self.clone_in(common, alloc)
    }
}
