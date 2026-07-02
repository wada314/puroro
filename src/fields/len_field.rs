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

    pub fn merge<Pb, B: Buf>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        bit: usize,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        A: Clone,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        // Decode first so a failure leaves the old value intact. Each `unmanaged`
        // op gets its own owned `alloc` clone (never a borrow).
        let new = T::decode(buf, common.alloc.clone())?;
        let old = unsafe { ManuallyDrop::take(&mut self.value) };
        // SAFETY: an owned clone of `common.alloc` is interchangeable with the
        // allocator that owns the old payload's buffer.
        unsafe { T::deallocate(old, common.alloc.clone()) };
        self.value = ManuallyDrop::new(new);
        P::on_set(common, bit);
        Ok(())
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

    pub fn clear<Pb>(&mut self, common: &mut MessageCommon<Pb, A>, bit: usize)
    where
        Pb: PresenceBits,
        A: Clone,
    {
        P::on_clear(common, bit);
        let old = unsafe { ManuallyDrop::take(&mut self.value) };
        // SAFETY: an owned clone of `common.alloc` is interchangeable with the
        // allocator that owns the old payload's buffer.
        unsafe { T::deallocate(old, common.alloc.clone()) };
        self.value = ManuallyDrop::new(T::new_empty(common.alloc.clone()));
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
// Type aliases
// ---------------------------------------------------------------------------

pub type SingularLen<T, P, A> = SingularLenField<T, P, A>;

pub type ImplicitLenField<T, A> = SingularLenField<T, super::field_presence::Implicit, A>;
pub type ExplicitLenField<T, A> = SingularLenField<T, super::field_presence::Explicit, A>;

pub type ImplicitString<A> = ImplicitLenField<len::ProtoString, A>;
pub type ExplicitString<A> = ExplicitLenField<len::ProtoString, A>;
pub type ImplicitBytes<A> = ImplicitLenField<len::ProtoBytes, A>;
pub type ExplicitBytes<A> = ExplicitLenField<len::ProtoBytes, A>;
