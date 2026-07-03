//! Singular LEN field wrapper — generic over payload type and presence policy.
//!
//! The payload is stored allocator-less ([`LenProtoType::Storage`]) wrapped in
//! [`ManuallyDrop`], so it never frees itself implicitly. The owning message
//! releases it via [`deallocate`](SingularLenField::deallocate) in its `Drop`,
//! and mutation flows through [`value_mut`](SingularLenField::value_mut).

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;

use crate::encode;
use crate::error::DecodeError;
use crate::optional::{HasDefault, Optional};
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::field_presence::{ExplicitFieldPresence, FieldPresence, RequiredFieldPresence};
use super::len::{self, LenProtoType};
use super::presence::PresenceBits;

/// Singular LEN field (`string`, `bytes`, …) — parametrised by [`LenProtoType`] `T`
/// and presence policy `P`. `A` is the message allocator; it is not stored inline.
pub struct SingularLenField<T: LenProtoType, P: FieldPresence, A: Allocator> {
    value: ManuallyDrop<T::Storage>,
    _marker: PhantomData<(P, A)>,
}

impl<T: LenProtoType, P: FieldPresence, A: Allocator> SingularLenField<T, P, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: ManuallyDrop::new(T::new_empty(alloc)),
            _marker: PhantomData,
        }
    }

    /// Wraps already-decoded [`LenProtoType::Storage`] into a field.
    ///
    /// Presence-agnostic (does not touch `MessageCommon`): the payload is simply
    /// adopted, so the caller owns responsibility for eventual
    /// [`deallocate`](Self::deallocate). Used by oneof variants, whose presence is
    /// tracked by the enclosing `OneofSlot` rather than a presence bit.
    pub fn from_storage(value: T::Storage) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            _marker: PhantomData,
        }
    }

    /// Borrowed payload (IMPLICIT public getters).
    #[inline]
    pub fn value(&self) -> T::Ref<'_> {
        T::borrow(&self.value)
    }

    /// Returns a growable handle over the payload, backed by the owned `alloc`.
    ///
    /// The allocator is taken by value (callers pass an `alloc.clone()`): the
    /// returned guard owns it, so growth and the eventual free all use the same
    /// allocator type `A` rather than a borrow.
    ///
    /// Presence is the caller's responsibility: generated `_mut` accessors set
    /// the presence bit before calling this.
    pub fn value_mut(&mut self, alloc: A) -> T::Mut<'_, A> {
        T::with_alloc(&mut self.value, alloc)
    }

    /// Binds this field to its message `common` state (presence + allocator),
    /// producing a short-lived [`SingularLenFieldMut`] view that carries the
    /// whole mutation context. `bit` is the presence index for policy `P`.
    ///
    /// This is the entry point for every mutation (`value_mut` / `merge` /
    /// `clear`): generated accessors call `field.bind(&mut common, bit).…()`
    /// instead of threading `common` through each method.
    #[inline]
    pub fn bind<'f, 'c, Pb: PresenceBits>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
        bit: usize,
    ) -> SingularLenFieldMut<'f, 'c, T, P, Pb, A> {
        SingularLenFieldMut::new(self, common, bit)
    }

    pub fn encoded_len<Pb>(&self, common: &MessageCommon<Pb, A>, field: u32, bit: usize) -> usize
    where
        Pb: PresenceBits,
    {
        let empty = T::is_empty(&self.value);
        if P::should_emit(common, bit, empty) {
            encode::encoded_len_len_field(field, T::as_bytes(&self.value).len())
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, B: BufMut>(
        &self,
        common: &MessageCommon<Pb, A>,
        field: u32,
        bit: usize,
        buf: &mut B,
    ) where
        Pb: PresenceBits,
    {
        let empty = T::is_empty(&self.value);
        if P::should_emit(common, bit, empty) {
            encode::encode_len_field(field, T::as_bytes(&self.value), buf);
        }
    }

    /// Releases the payload through the owned `alloc`. Terminal; call once from
    /// the owning message's `Drop`, after which `self` must not be used.
    pub fn deallocate(&mut self, alloc: A) {
        // SAFETY: called once; an owned clone of the message allocator owns the
        // payload's buffer.
        let old = unsafe { ManuallyDrop::take(&mut self.value) };
        unsafe { T::deallocate(old, alloc) };
    }
}

impl<T: LenProtoType, P: ExplicitFieldPresence, A: Allocator> SingularLenField<T, P, A> {
    #[inline]
    pub fn has<Pb>(&self, common: &MessageCommon<Pb, A>, bit: usize) -> bool
    where
        Pb: PresenceBits,
    {
        common.is_present(bit)
    }

    pub fn optional<'a, Pb, D>(
        &'a self,
        common: &MessageCommon<Pb, A>,
        bit: usize,
        default: D,
    ) -> Optional<T::Ref<'a>, D>
    where
        Pb: PresenceBits,
        D: HasDefault<T::Ref<'a>>,
        T::Ref<'a>: Copy,
    {
        let v = if common.is_present(bit) {
            Some(T::borrow(&self.value))
        } else {
            None
        };
        Optional::new(v, default)
    }
}

impl<T: LenProtoType, P: RequiredFieldPresence, A: Allocator> SingularLenField<T, P, A> {
    /// Checks the presence bit for a LEGACY_REQUIRED field.
    pub fn validate_required<Pb>(
        &self,
        common: &MessageCommon<Pb, A>,
        bit: usize,
        field_number: u32,
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
    {
        P::validate_present(common, bit, field_number)
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a singular LEN field to its message common state,
/// produced by [`SingularLenField::bind`].
///
/// It bundles the field storage with the presence/allocator context so that a
/// generated accessor can express a whole mutation as a single call, instead of
/// poking `_common` and the field separately (mirrors the `unmanaged` guard
/// idiom of temporarily reuniting split state). Two lifetimes keep the returned
/// guard tied to the field storage only (`'f`); the `common` borrow (`'c`) is
/// released as soon as the method returns. Every method consumes the view, so a
/// fresh `bind` precedes each mutation.
pub struct SingularLenFieldMut<'f, 'c, T: LenProtoType, P: FieldPresence, Pb: PresenceBits, A: Allocator>
{
    field: &'f mut SingularLenField<T, P, A>,
    common: &'c mut MessageCommon<Pb, A>,
    bit: usize,
}

impl<'f, 'c, T: LenProtoType, P: FieldPresence, Pb: PresenceBits, A: Allocator>
    SingularLenFieldMut<'f, 'c, T, P, Pb, A>
{
    #[inline]
    fn new(
        field: &'f mut SingularLenField<T, P, A>,
        common: &'c mut MessageCommon<Pb, A>,
        bit: usize,
    ) -> Self {
        Self { field, common, bit }
    }

    /// Marks presence (per `P`) and returns a growable guard over the payload.
    ///
    /// The guard implements `DerefMut<Target = String<A>>` / `Vec<u8, A>` and
    /// writes the payload back into the allocator-less storage on drop. It
    /// borrows only the field (`'f`), so `common` is free again once this
    /// returns.
    #[inline]
    pub fn value_mut(self) -> T::Mut<'f, A>
    where
        A: Clone,
    {
        P::on_set(self.common, self.bit);
        let alloc = self.common.alloc.clone();
        self.field.value_mut(alloc)
    }

    /// Merges one LEN occurrence: decodes a fresh payload, frees the old one,
    /// and marks presence (per `P`).
    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        // Decode first so a failure leaves the old value intact. Each `unmanaged`
        // op gets its own owned `alloc` clone (never a borrow).
        let new = T::decode(buf, self.common.alloc.clone())?;
        let old = unsafe { ManuallyDrop::take(&mut self.field.value) };
        // SAFETY: an owned clone of `common.alloc` is interchangeable with the
        // allocator that owns the old payload's buffer.
        unsafe { T::deallocate(old, self.common.alloc.clone()) };
        self.field.value = ManuallyDrop::new(new);
        P::on_set(self.common, self.bit);
        Ok(())
    }
}

impl<'f, 'c, T: LenProtoType, P: ExplicitFieldPresence, Pb: PresenceBits, A: Allocator>
    SingularLenFieldMut<'f, 'c, T, P, Pb, A>
{
    /// Clears presence and resets the payload to empty, freeing the old buffer.
    pub fn clear(self)
    where
        A: Clone,
    {
        P::on_clear(self.common, self.bit);
        let old = unsafe { ManuallyDrop::take(&mut self.field.value) };
        // SAFETY: an owned clone of `common.alloc` is interchangeable with the
        // allocator that owns the old payload's buffer.
        unsafe { T::deallocate(old, self.common.alloc.clone()) };
        self.field.value = ManuallyDrop::new(T::new_empty(self.common.alloc.clone()));
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type SingularLen<T, P, A> = SingularLenField<T, P, A>;

pub type ImplicitLenField<T, A> = SingularLenField<T, super::field_presence::Implicit, A>;
pub type ExplicitLenField<T, A> = SingularLenField<T, super::field_presence::Explicit, A>;

pub type ImplicitString<A> = ImplicitLenField<len::ProtoString, A>;
pub type ExplicitString<A> = ExplicitLenField<len::ProtoString, A>;
pub type ImplicitBytes<A> = ImplicitLenField<len::ProtoBytes, A>;
pub type ExplicitBytes<A> = ExplicitLenField<len::ProtoBytes, A>;
