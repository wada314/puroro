//! Unified repeated field wrapper — generic over element marker and encode policy.
//!
//! Elements are stored in an allocator-less [`UnmanagedVec`] wrapped in
//! [`ManuallyDrop`]. Growth and release borrow the message allocator.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::bytes::{Buf, BufMut};
use ::unmanaged::UnmanagedVec;
use ::unmanaged::vec::VecGuard;

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::fields::shared::{FieldDeallocate, MessageCommon, PresenceBits};
use crate::fields::wire::len::{ProtoBytes, ProtoString};
use crate::fields::wire::repeated_items::{RepeatedItems, RepeatedSlicePush};
use crate::fields::wire::varint;

use super::encoding::{Expanded, Packed, RepeatedEncoding};

/// Repeated field parametrised by type marker `T` and encode policy `E`.
///
/// [`Packed`] is only available when `T: PackableRepeatedItems`. [`Expanded`]
/// covers packable and non-packable markers (string / bytes / future message).
/// [`RepeatedFieldMut::merge`] accepts both packed and expanded wire forms for
/// packable `T`.
pub struct RepeatedField<T: RepeatedItems, E: RepeatedEncoding<T>, const FIELD: u32> {
    values: ManuallyDrop<UnmanagedVec<T::Element, T::Alloc>>,
    _encoding: PhantomData<E>,
}

impl<T: RepeatedItems, E: RepeatedEncoding<T>, const FIELD: u32> RepeatedField<T, E, FIELD> {
    pub fn new_in(alloc: T::Alloc) -> Self {
        Self {
            values: ManuallyDrop::new(UnmanagedVec::new(alloc)),
            _encoding: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T::Element] {
        &self.values
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn encoded_len<Pb>(&self, _common: &MessageCommon<Pb, T::Alloc>) -> usize
    where
        Pb: PresenceBits,
    {
        if self.values.is_empty() {
            0
        } else {
            E::encoded_len(FIELD, self.as_slice())
        }
    }

    pub fn encode_raw<Pb, B: BufMut>(&self, _common: &MessageCommon<Pb, T::Alloc>, buf: &mut B)
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
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> RepeatedFieldRef<'a, T, E, FIELD, Pb>
    where
        T::Alloc: Clone,
    {
        RepeatedFieldRef::new(self, common)
    }

    /// Binds this field to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, T::Alloc>,
    ) -> RepeatedFieldMut<'f, 'c, T, E, FIELD, Pb>
    where
        T::Alloc: Clone,
    {
        RepeatedFieldMut::new(self, common)
    }
}

impl<T, E, const FIELD: u32, Pb> FieldDeallocate<Pb, T::Alloc> for RepeatedField<T, E, FIELD>
where
    T: RepeatedItems,
    E: RepeatedEncoding<T>,
    Pb: PresenceBits,
    T::Alloc: Clone,
{
    /// Releases every element (when heap-backed) and the backing buffer.
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, T::Alloc>) {
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
        unsafe { v.deallocate(alloc) };
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a repeated field to its message common state.
pub struct RepeatedFieldRef<
    'a,
    T: RepeatedItems,
    E: RepeatedEncoding<T>,
    const FIELD: u32,
    Pb: PresenceBits,
> {
    field: &'a RepeatedField<T, E, FIELD>,
    /// Bound for symmetry with [`RepeatedFieldMut`]; unused by current getters.
    #[allow(dead_code)]
    common: &'a MessageCommon<Pb, T::Alloc>,
    _encoding: PhantomData<E>,
}

impl<'a, T: RepeatedItems, E: RepeatedEncoding<T>, const FIELD: u32, Pb: PresenceBits>
    RepeatedFieldRef<'a, T, E, FIELD, Pb>
{
    #[inline]
    fn new(field: &'a RepeatedField<T, E, FIELD>, common: &'a MessageCommon<Pb, T::Alloc>) -> Self {
        Self {
            field,
            common,
            _encoding: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(self) -> &'a [T::Element] {
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
///
/// Repeated fields have no presence bit; the view carries `common` for the
/// allocator. Every method consumes the view.
pub struct RepeatedFieldMut<
    'f,
    'c,
    T: RepeatedItems,
    E: RepeatedEncoding<T>,
    const FIELD: u32,
    Pb: PresenceBits,
> {
    field: &'f mut RepeatedField<T, E, FIELD>,
    common: &'c mut MessageCommon<Pb, T::Alloc>,
    _encoding: PhantomData<E>,
}

impl<'f, 'c, T: RepeatedItems, E: RepeatedEncoding<T>, const FIELD: u32, Pb: PresenceBits>
    RepeatedFieldMut<'f, 'c, T, E, FIELD, Pb>
{
    #[inline]
    fn new(
        field: &'f mut RepeatedField<T, E, FIELD>,
        common: &'c mut MessageCommon<Pb, T::Alloc>,
    ) -> Self {
        Self {
            field,
            common,
            _encoding: PhantomData,
        }
    }

    /// Growable handle over copy elements (`repeated int32`, …).
    pub fn values_mut(self) -> VecGuard<'f, T::Element, T::Alloc>
    where
        T::Alloc: Clone,
        T::Element: Copy,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        unsafe { self.field.values.with_alloc(alloc) }
    }

    /// Appends an element built from a payload slice (`repeated string` / `bytes`).
    pub fn push_in(self, v: impl AsRef<[u8]>) -> Result<(), DecodeError>
    where
        T: RepeatedSlicePush,
        T::Alloc: Clone,
    {
        let alloc = self.common.alloc.clone();
        let stored = T::element_from_slice(v.as_ref(), alloc.clone())?;
        // SAFETY: owned clones of the message allocator own this vector's buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc) };
        g.push(stored);
        Ok(())
    }

    /// Empties the vector (keeps capacity). Heap elements are freed first.
    pub fn clear(self)
    where
        T::Alloc: Clone,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: owned clones of the message allocator own this vector's buffer
        // and every element.
        let mut g = unsafe { self.field.values.with_alloc(alloc.clone()) };
        while let Some(elem) = g.pop() {
            unsafe { T::deallocate_element(elem, alloc.clone()) };
        }
    }

    /// Merges one wire occurrence — appends element(s).
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        T::Alloc: Clone,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc.clone()) };
        T::merge_occurrence(wire_type, buf, alloc, |elem| {
            g.push(elem);
        })
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type RepeatedPackedVarintField<T, const FIELD: u32> = RepeatedField<T, Packed, FIELD>;
pub type RepeatedExpandedVarintField<T, const FIELD: u32> = RepeatedField<T, Expanded, FIELD>;
pub type RepeatedLenField<T, const FIELD: u32> = RepeatedField<T, Expanded, FIELD>;

pub type RepeatedPackedInt32<const FIELD: u32, A> =
    RepeatedPackedVarintField<varint::ProtoInt32<A>, FIELD>;
pub type RepeatedExpandedInt32<const FIELD: u32, A> =
    RepeatedExpandedVarintField<varint::ProtoInt32<A>, FIELD>;
pub type RepeatedString<const FIELD: u32, A> = RepeatedLenField<ProtoString<A>, FIELD>;
pub type RepeatedBytes<const FIELD: u32, A> = RepeatedLenField<ProtoBytes<A>, FIELD>;
