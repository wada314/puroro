//! Repeated LEN field wrapper (`repeated string`, `repeated bytes`, …).
//!
//! Elements are stored in an allocator-less [`UnmanagedVec`] wrapped in
//! [`ManuallyDrop`]; each element is itself allocator-less
//! ([`LenProtoType::Storage`]). Releasing therefore drains and frees every
//! element first, then frees the buffer.

use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::unmanaged::UnmanagedVec;

use crate::encode;
use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::fields::shared::{FieldDeallocate, MessageCommon, PresenceBits};
use crate::fields::wire::len::{self, LenProtoType};

/// Repeated field whose elements are length-delimited records (one tag per element).
pub struct RepeatedLenField<T: LenProtoType<Alloc = A>, const FIELD: u32, A: Allocator> {
    values: ManuallyDrop<UnmanagedVec<T::Storage, A>>,
}

impl<T: LenProtoType<Alloc = A>, const FIELD: u32, A: Allocator> RepeatedLenField<T, FIELD, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            values: ManuallyDrop::new(UnmanagedVec::new(alloc)),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T::Storage] {
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
        self.values
            .iter()
            .map(|v| encode::encoded_len_len_field(FIELD, T::as_bytes(v).len()))
            .sum()
    }

    pub fn encode_raw<Pb, B: BufMut>(&self, _common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
    {
        for v in self.values.iter() {
            encode::encode_len_field(FIELD, T::as_bytes(v), buf);
        }
    }

    /// Binds this field to `common` for read access.
    #[inline]
    pub fn bind<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> RepeatedLenFieldRef<'a, T, FIELD, Pb, A>
    where
        A: Clone,
    {
        RepeatedLenFieldRef::new(self, common)
    }

    /// Binds this field to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> RepeatedLenFieldMut<'f, 'c, T, FIELD, Pb, A>
    where
        A: Clone,
    {
        RepeatedLenFieldMut::new(self, common)
    }
}

impl<T: LenProtoType<Alloc = A>, const FIELD: u32, Pb: PresenceBits, A: Allocator + Clone>
    FieldDeallocate<Pb, A> for RepeatedLenField<T, FIELD, A>
{
    /// Releases every element and the backing buffer through `common.alloc`.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        // SAFETY: called once; owned clones of the message allocator own the
        // buffer and every element.
        let alloc = common.alloc.clone();
        let mut v = unsafe { ManuallyDrop::take(&mut self.values) };
        {
            let mut g = unsafe { v.with_alloc(alloc.clone()) };
            while let Some(elem) = g.pop() {
                unsafe { T::deallocate(elem, alloc.clone()) };
            }
        }
        unsafe { v.deallocate(alloc) };
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a repeated LEN field to its message common
/// state, produced by [`RepeatedLenField::bind`].
///
/// Mirrors [`RepeatedLenFieldMut`] for the read path. Generated getters always
/// go through this view — `field.bind(&common).as_slice()` — even though the
/// accessor does not consult `common`.
pub struct RepeatedLenFieldRef<
    'a,
    T: LenProtoType<Alloc = A>,
    const FIELD: u32,
    Pb: PresenceBits,
    A: Allocator,
> {
    field: &'a RepeatedLenField<T, FIELD, A>,
    /// Bound for symmetry with [`RepeatedLenFieldMut`]; unused by current getters.
    #[allow(dead_code)]
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, T: LenProtoType<Alloc = A>, const FIELD: u32, Pb: PresenceBits, A: Allocator>
    RepeatedLenFieldRef<'a, T, FIELD, Pb, A>
{
    #[inline]
    fn new(field: &'a RepeatedLenField<T, FIELD, A>, common: &'a MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }

    #[inline]
    pub fn as_slice(self) -> &'a [T::Storage] {
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

/// Short-lived binding of a repeated LEN field to its message common state,
/// produced by [`RepeatedLenField::bind_mut`].
///
/// Bundles the element buffer with the allocator context so that generated code
/// can mutate through a single call. Repeated fields have no presence bit, so
/// the view carries only `common` (for the allocator). Every method consumes
/// the view, so a fresh `bind_mut` precedes each mutation.
pub struct RepeatedLenFieldMut<
    'f,
    'c,
    T: LenProtoType<Alloc = A>,
    const FIELD: u32,
    Pb: PresenceBits,
    A: Allocator,
> {
    field: &'f mut RepeatedLenField<T, FIELD, A>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, T: LenProtoType<Alloc = A>, const FIELD: u32, Pb: PresenceBits, A: Allocator>
    RepeatedLenFieldMut<'f, 'c, T, FIELD, Pb, A>
{
    #[inline]
    fn new(
        field: &'f mut RepeatedLenField<T, FIELD, A>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Appends an element built from a payload slice, allocating through owned
    /// clones of the message allocator.
    pub fn push_in(self, v: impl AsRef<[u8]>) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        let stored = T::store_from_slice(v.as_ref(), alloc.clone())?;
        // SAFETY: owned clones of the message allocator are interchangeable and
        // own this vector's buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc) };
        g.push(stored);
        Ok(())
    }

    /// Merges one LEN element: decodes and appends it.
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let alloc = self.common.alloc.clone();
        let stored = T::decode(buf, alloc.clone())?;
        // SAFETY: owned clones of the message allocator own this vector's buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc) };
        g.push(stored);
        Ok(())
    }

    /// Drops every element and empties the vector (keeps the buffer capacity).
    pub fn clear(self)
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: owned clones of the message allocator own this vector's buffer
        // and every element.
        let mut g = unsafe { self.field.values.with_alloc(alloc.clone()) };
        while let Some(elem) = g.pop() {
            unsafe { T::deallocate(elem, alloc.clone()) };
        }
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type RepeatedLen<T, const FIELD: u32, A> = RepeatedLenField<T, FIELD, A>;
pub type RepeatedString<const FIELD: u32, A> = RepeatedLenField<len::ProtoString<A>, FIELD, A>;
pub type RepeatedBytes<const FIELD: u32, A> = RepeatedLenField<len::ProtoBytes<A>, FIELD, A>;
