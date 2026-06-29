//! Singular LEN field wrapper — generic over payload type and presence policy.

use ::core::marker::PhantomData;

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;

use crate::decode;
use crate::encode;
use crate::error::DecodeError;
use crate::optional::{HasDefault, Optional};
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::field_presence::{ExplicitFieldPresence, FieldPresence};
use super::len::{self, LenProtoType};
use super::presence::PresenceBits;

/// Singular LEN field (`string`, `bytes`, …) — parametrised by [`LenProtoType`] `T`
/// and presence policy `P`.
pub struct SingularLenField<T: LenProtoType, P: FieldPresence, A: Allocator> {
    value: T::Storage<A>,
    _presence: PhantomData<P>,
}

impl<T: LenProtoType, P: FieldPresence, A: Allocator + Clone> SingularLenField<T, P, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: T::new_empty(alloc),
            _presence: PhantomData,
        }
    }

    /// Borrowed payload (IMPLICIT public getters).
    #[inline]
    pub fn borrow(&self) -> T::Ref<'_, A> {
        T::borrow(&self.value)
    }

    pub fn set_str<Pb, const BIT: usize>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        v: &str,
    ) where
        Pb: PresenceBits,
        T: LenProtoType<Storage<A> = ::allocator_api2::boxed::Box<str, A>>,
    {
        P::on_set(common, BIT);
        self.value = decode::str_to_box_in(v, common.alloc.clone());
    }

    pub fn set_from_slice<Pb, const BIT: usize>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        v: &[u8],
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
    {
        P::on_set(common, BIT);
        self.value = T::store_from_slice(v, common.alloc.clone())?;
        Ok(())
    }

    pub fn clear_value(&mut self, alloc: A) {
        self.value = T::new_empty(alloc);
    }

    pub fn encoded_len<Pb, const FIELD: u32, const BIT: usize>(
        &self,
        common: &MessageCommon<Pb, A>,
    ) -> usize
    where
        Pb: PresenceBits,
    {
        let empty = T::is_empty(&self.value);
        if P::should_emit(common, BIT, empty) {
            encode::encoded_len_len_field(FIELD, T::as_bytes(&self.value).len())
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, B: BufMut, const FIELD: u32, const BIT: usize>(
        &self,
        common: &MessageCommon<Pb, A>,
        buf: &mut B,
    ) where
        Pb: PresenceBits,
    {
        let empty = T::is_empty(&self.value);
        if P::should_emit(common, BIT, empty) {
            encode::encode_len_field(FIELD, T::as_bytes(&self.value), buf);
        }
    }

    pub fn merge<Pb, B: Buf, const BIT: usize>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        self.value = T::decode(buf, common.alloc.clone())?;
        P::on_set(common, BIT);
        Ok(())
    }
}

impl<T: LenProtoType, P: ExplicitFieldPresence, A: Allocator + Clone> SingularLenField<T, P, A> {
    #[inline]
    pub fn has<Pb, const BIT: usize>(&self, common: &MessageCommon<Pb, A>) -> bool
    where
        Pb: PresenceBits,
    {
        common.is_present(BIT)
    }

    pub fn optional<'a, Pb, D, const BIT: usize>(
        &'a self,
        common: &MessageCommon<Pb, A>,
        default: D,
    ) -> Optional<T::Ref<'a, A>, D>
    where
        Pb: PresenceBits,
        D: HasDefault<T::Ref<'a, A>>,
        T::Ref<'a, A>: Copy,
    {
        let v = if common.is_present(BIT) {
            Some(T::borrow(&self.value))
        } else {
            None
        };
        Optional::new(v, default)
    }

    pub fn clear<Pb, const BIT: usize>(&mut self, common: &mut MessageCommon<Pb, A>)
    where
        Pb: PresenceBits,
    {
        P::on_clear(common, BIT);
        self.value = T::new_empty(common.alloc.clone());
    }
}

impl<T: LenProtoType, P: FieldPresence, A: Allocator + Clone> Clone for SingularLenField<T, P, A>
where
    T::Storage<A>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _presence: PhantomData,
        }
    }
}

impl<T: LenProtoType, P: FieldPresence, A: Allocator + Clone> PartialEq
    for SingularLenField<T, P, A>
where
    T::Storage<A>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: LenProtoType, P: FieldPresence, A: Allocator + Clone> Eq for SingularLenField<T, P, A> where
    T::Storage<A>: Eq
{
}

impl<T: LenProtoType, P: FieldPresence, A: Allocator + Clone> ::core::fmt::Debug
    for SingularLenField<T, P, A>
where
    T::Storage<A>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SingularLenField")
            .field("value", &self.value)
            .finish()
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
