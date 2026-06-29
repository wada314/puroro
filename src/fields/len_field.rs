//! Singular LEN field wrappers (`string`, `bytes`) generic over [`LenProtoType`].
//!
//! Nested messages use [`super::message::NestedMessageField`] instead.

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;

use crate::decode;
use crate::encode;
use crate::error::DecodeError;
use crate::optional::{HasDefault, Optional};
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::len::{self, LenProtoType};
use super::presence::PresenceBits;

// ---------------------------------------------------------------------------
// IMPLICIT presence
// ---------------------------------------------------------------------------

/// Singular LEN field with IMPLICIT presence (`string`, `bytes`, …).
pub struct ImplicitLenField<T: LenProtoType, A: Allocator> {
    value: T::Storage<A>,
}

impl<T: LenProtoType, A: Allocator + Clone> ImplicitLenField<T, A> {
    /// Creates a field holding the type-empty payload.
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: T::new_empty(alloc),
        }
    }

    /// Returns a borrowed view of the stored value.
    #[inline]
    pub fn get(&self) -> T::Ref<'_, A> {
        T::borrow(&self.value)
    }

    /// Copies `v` into the field (validates UTF-8 for [`len::ProtoString`]).
    pub fn set_from_slice(
        &mut self,
        common: &MessageCommon<impl PresenceBits, A>,
        v: &[u8],
    ) -> Result<(), DecodeError> {
        self.value = T::store_from_slice(v, common.alloc.clone())?;
        Ok(())
    }

    /// Sets a string field from `&str`.
    pub fn set_str(&mut self, common: &MessageCommon<impl PresenceBits, A>, v: &str)
    where
        T: LenProtoType<Storage<A> = ::allocator_api2::boxed::Box<str, A>>,
    {
        self.value = decode::str_to_box_in(v, common.alloc.clone());
    }

    /// Wire byte length, or `0` when omitted (empty payload).
    pub fn encoded_len<const FIELD: u32>(&self) -> usize {
        if T::is_empty(&self.value) {
            0
        } else {
            encode::encoded_len_len_field(FIELD, T::as_bytes(&self.value).len())
        }
    }

    /// Encodes when non-empty.
    pub fn encode_raw<const FIELD: u32, B: BufMut>(&self, buf: &mut B) {
        if !T::is_empty(&self.value) {
            encode::encode_len_field(FIELD, T::as_bytes(&self.value), buf);
        }
    }

    /// Merges one wire occurrence (last value wins — replaces payload).
    pub fn merge<B: Buf>(
        &mut self,
        common: &MessageCommon<impl PresenceBits, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError> {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        self.value = T::decode(buf, common.alloc.clone())?;
        Ok(())
    }
}

impl<T: LenProtoType, A: Allocator + Clone> Clone for ImplicitLenField<T, A>
where
    T::Storage<A>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T: LenProtoType, A: Allocator + Clone> PartialEq for ImplicitLenField<T, A>
where
    T::Storage<A>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: LenProtoType, A: Allocator + Clone> Eq for ImplicitLenField<T, A> where T::Storage<A>: Eq {}

impl<T: LenProtoType, A: Allocator + Clone> ::core::fmt::Debug for ImplicitLenField<T, A>
where
    T::Storage<A>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ImplicitLenField")
            .field("value", &self.value)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// EXPLICIT presence
// ---------------------------------------------------------------------------

/// Singular LEN field with EXPLICIT presence (bit + value slot).
pub struct ExplicitLenField<T: LenProtoType, A: Allocator> {
    value: T::Storage<A>,
}

impl<T: LenProtoType, A: Allocator + Clone> ExplicitLenField<T, A> {
    /// Creates an unset field (bit clear, empty payload).
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: T::new_empty(alloc),
        }
    }

    #[inline]
    pub fn has<P: PresenceBits, const BIT: usize>(
        &self,
        common: &MessageCommon<P, A>,
    ) -> bool {
        common.is_present(BIT)
    }

    pub fn get<'a, P: PresenceBits, D, const BIT: usize>(
        &'a self,
        common: &MessageCommon<P, A>,
        default: D,
    ) -> Optional<T::Ref<'a, A>, D>
    where
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

    pub fn set_str<P: PresenceBits, const BIT: usize>(
        &mut self,
        common: &mut MessageCommon<P, A>,
        v: &str,
    ) where
        T: LenProtoType<Storage<A> = ::allocator_api2::boxed::Box<str, A>>,
    {
        common.set_presence(BIT, true);
        self.value = decode::str_to_box_in(v, common.alloc.clone());
    }

    pub fn set_from_slice<P: PresenceBits, const BIT: usize>(
        &mut self,
        common: &mut MessageCommon<P, A>,
        v: &[u8],
    ) -> Result<(), DecodeError> {
        common.set_presence(BIT, true);
        self.value = T::store_from_slice(v, common.alloc.clone())?;
        Ok(())
    }

    pub fn clear<P: PresenceBits, const BIT: usize>(
        &mut self,
        common: &mut MessageCommon<P, A>,
    ) {
        common.set_presence(BIT, false);
        self.value = T::new_empty(common.alloc.clone());
    }

    pub fn encoded_len<P: PresenceBits, const FIELD: u32, const BIT: usize>(
        &self,
        common: &MessageCommon<P, A>,
    ) -> usize {
        if common.is_present(BIT) {
            encode::encoded_len_len_field(FIELD, T::as_bytes(&self.value).len())
        } else {
            0
        }
    }

    pub fn encode_raw<P: PresenceBits, B: BufMut, const FIELD: u32, const BIT: usize>(
        &self,
        common: &MessageCommon<P, A>,
        buf: &mut B,
    ) {
        if common.is_present(BIT) {
            encode::encode_len_field(FIELD, T::as_bytes(&self.value), buf);
        }
    }

    pub fn merge<P: PresenceBits, B: Buf, const BIT: usize>(
        &mut self,
        common: &mut MessageCommon<P, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError> {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        self.value = T::decode(buf, common.alloc.clone())?;
        common.set_presence(BIT, true);
        Ok(())
    }
}

impl<T: LenProtoType, A: Allocator + Clone> Clone for ExplicitLenField<T, A>
where
    T::Storage<A>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T: LenProtoType, A: Allocator + Clone> PartialEq for ExplicitLenField<T, A>
where
    T::Storage<A>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: LenProtoType, A: Allocator + Clone> Eq for ExplicitLenField<T, A> where T::Storage<A>: Eq {}

impl<T: LenProtoType, A: Allocator + Clone> ::core::fmt::Debug for ExplicitLenField<T, A>
where
    T::Storage<A>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ExplicitLenField")
            .field("value", &self.value)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type ImplicitString<A> = ImplicitLenField<len::ProtoString, A>;
pub type ExplicitString<A> = ExplicitLenField<len::ProtoString, A>;
pub type ImplicitBytes<A> = ImplicitLenField<len::ProtoBytes, A>;
pub type ExplicitBytes<A> = ExplicitLenField<len::ProtoBytes, A>;
