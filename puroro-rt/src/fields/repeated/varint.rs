//! Repeated varint field wrapper — generic over wire type and encode policy.
//!
//! Elements are stored in an allocator-less [`UnmanagedVec`] wrapped in
//! [`ManuallyDrop`]. Growth and release borrow the message allocator.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::unmanaged::UnmanagedVec;
use ::unmanaged::vec::VecGuard;

use crate::decode;
use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::fields::shared::{Bindable, BindableMut, FieldDeallocate, MessageCommon, PresenceBits};
use crate::fields::wire::varint::{self, VarintProtoType};

use super::encoding::RepeatedVarintEncoding;

/// Repeated field whose elements share a varint wire representation.
///
/// Param `E` is [`Packed`](super::encoding::Packed) or
/// [`Expanded`](super::encoding::Expanded) — affects **encode only**.
/// [`RepeatedVarintFieldMut::merge`] accepts both packed and expanded wire forms.
pub struct RepeatedVarintField<
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    A: Allocator,
> {
    values: ManuallyDrop<UnmanagedVec<T::Value>>,
    _marker: PhantomData<(E, A)>,
}

impl<T: VarintProtoType, E: RepeatedVarintEncoding, const FIELD: u32, A: Allocator>
    RepeatedVarintField<T, E, FIELD, A>
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            values: ManuallyDrop::new(UnmanagedVec::new(alloc)),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T::Value] {
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
            E::encoded_len::<T>(FIELD, self.as_slice())
        }
    }

    pub fn encode_raw<Pb, B: BufMut>(&self, _common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
    {
        if !self.values.is_empty() {
            E::encode::<B, T>(FIELD, self.as_slice(), buf);
        }
    }

    /// Releases the backing buffer through `common.alloc`. Terminal; call once
    /// from the owning message's `Drop`.
    pub fn deallocate<Pb: PresenceBits>(&mut self, common: &MessageCommon<Pb, A>)
    where
        A: Clone,
    {
        // SAFETY: called once; an owned clone of the message allocator owns the
        // buffer. Elements are `Copy` scalars with no per-element cleanup.
        let v = unsafe { ManuallyDrop::take(&mut self.values) };
        unsafe { v.deallocate(common.alloc.clone()) };
    }
}

impl<
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    Pb: PresenceBits,
    A: Allocator + Clone,
> FieldDeallocate<Pb, A> for RepeatedVarintField<T, E, FIELD, A>
{
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
        RepeatedVarintField::deallocate(self, common);
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a repeated varint field to its message common
/// state, produced by [`Bindable::bind`](crate::fields::shared::Bindable::bind).
///
/// Mirrors [`RepeatedVarintFieldMut`] for the read path. Generated getters always
/// go through this view — `field.bind(&common).as_slice()` — even though the
/// accessor does not consult `common`.
pub struct RepeatedVarintFieldRef<
    'a,
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    Pb: PresenceBits,
    A: Allocator,
> {
    field: &'a RepeatedVarintField<T, E, FIELD, A>,
    /// Bound for symmetry with [`RepeatedVarintFieldMut`]; unused by current getters.
    #[allow(dead_code)]
    common: &'a MessageCommon<Pb, A>,
}

impl<
    'a,
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    Pb: PresenceBits,
    A: Allocator,
> RepeatedVarintFieldRef<'a, T, E, FIELD, Pb, A>
{
    #[inline]
    fn new(
        field: &'a RepeatedVarintField<T, E, FIELD, A>,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    #[inline]
    pub fn as_slice(self) -> &'a [T::Value] {
        self.field.as_slice()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.field.is_empty()
    }
}

impl<
    'a,
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    A: Allocator + Clone,
    Pb: PresenceBits,
> Bindable<&'a MessageCommon<Pb, A>> for &'a RepeatedVarintField<T, E, FIELD, A>
{
    type Bound = RepeatedVarintFieldRef<'a, T, E, FIELD, Pb, A>;

    #[inline]
    fn bind(self, common: &'a MessageCommon<Pb, A>) -> Self::Bound {
        RepeatedVarintFieldRef::new(self, common)
    }
}

impl<
    'f,
    'c,
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    A: Allocator + Clone,
    Pb: PresenceBits,
> BindableMut<&'c mut MessageCommon<Pb, A>> for &'f mut RepeatedVarintField<T, E, FIELD, A>
{
    type BoundMut = RepeatedVarintFieldMut<'f, 'c, T, E, FIELD, Pb, A>;

    #[inline]
    fn bind_mut(self, common: &'c mut MessageCommon<Pb, A>) -> Self::BoundMut {
        RepeatedVarintFieldMut::new(self, common)
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a repeated varint field to its message common state,
/// produced by [`BindableMut::bind_mut`](crate::fields::shared::BindableMut::bind_mut).
///
/// Bundles the element buffer with the allocator context so that generated code
/// can mutate through a single call. Repeated fields have no presence bit, so
/// the view carries only `common` (for the allocator). Every method consumes
/// the view, so a fresh `bind_mut` precedes each mutation.
pub struct RepeatedVarintFieldMut<
    'f,
    'c,
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    Pb: PresenceBits,
    A: Allocator,
> {
    field: &'f mut RepeatedVarintField<T, E, FIELD, A>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<
    'f,
    'c,
    T: VarintProtoType,
    E: RepeatedVarintEncoding,
    const FIELD: u32,
    Pb: PresenceBits,
    A: Allocator,
> RepeatedVarintFieldMut<'f, 'c, T, E, FIELD, Pb, A>
{
    #[inline]
    fn new(
        field: &'f mut RepeatedVarintField<T, E, FIELD, A>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns a growable handle over the elements, backed by an owned clone of
    /// the message allocator (the guard owns it).
    pub fn values_mut(self) -> VecGuard<'f, T::Value, A>
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        unsafe { self.field.values.with_alloc(alloc) }
    }

    /// Empties the vector (keeps the buffer capacity).
    pub fn clear(self)
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc) };
        g.clear();
    }

    /// Merges one packed (LEN) or expanded (VARINT) occurrence — appends element(s).
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        // SAFETY: an owned clone of the message allocator owns this vector's
        // buffer.
        let mut g = unsafe { self.field.values.with_alloc(alloc) };
        match wire_type {
            WireType::Len => {
                let len = decode::decode_varint(buf)? as usize;
                if buf.remaining() < len {
                    return Err(DecodeError::TruncatedMessage);
                }
                let mut sub = buf.take(len);
                while sub.has_remaining() {
                    let raw = decode::decode_varint(&mut sub)?;
                    g.push(T::decode_wire(raw)?);
                }
            }
            WireType::Varint => {
                let raw = decode::decode_varint(buf)?;
                g.push(T::decode_wire(raw)?);
            }
            _ => return Err(DecodeError::InvalidTag),
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type RepeatedVarint<T, E, const FIELD: u32, A> = RepeatedVarintField<T, E, FIELD, A>;

pub type RepeatedPackedVarintField<T, const FIELD: u32, A> =
    RepeatedVarintField<T, super::encoding::Packed, FIELD, A>;
pub type RepeatedExpandedVarintField<T, const FIELD: u32, A> =
    RepeatedVarintField<T, super::encoding::Expanded, FIELD, A>;

pub type RepeatedPackedInt32<const FIELD: u32, A> =
    RepeatedPackedVarintField<varint::ProtoInt32, FIELD, A>;
pub type RepeatedExpandedInt32<const FIELD: u32, A> =
    RepeatedExpandedVarintField<varint::ProtoInt32, FIELD, A>;
