//! Repeated LEN field wrapper (`repeated string`, `repeated bytes`, …).

use ::bytes::{Buf, BufMut};
use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AVec;

use crate::decode;
use crate::encode;
use crate::error::DecodeError;
use crate::wire_type::WireType;

use super::common::MessageCommon;
use super::len::{self, LenProtoType};
use super::presence::PresenceBits;

/// Repeated field whose elements are length-delimited records (one tag per element).
pub struct RepeatedLenField<T: LenProtoType, A: Allocator> {
    values: AVec<T::Storage<A>, A>,
}

impl<T: LenProtoType, A: Allocator + Clone> RepeatedLenField<T, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            values: AVec::new_in(alloc),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T::Storage<A>] {
        &self.values
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn push_str<P>(
        &mut self,
        common: &MessageCommon<P, A>,
        v: &str,
    ) where
        P: PresenceBits,
        T: LenProtoType<Storage<A> = ::allocator_api2::boxed::Box<str, A>>,
    {
        self.values
            .push(decode::str_to_box_in(v, common.alloc.clone()));
    }

    pub fn push_from_slice<P>(
        &mut self,
        common: &MessageCommon<P, A>,
        v: &[u8],
    ) -> Result<(), DecodeError>
    where
        P: PresenceBits,
    {
        let stored = T::store_from_slice(v, common.alloc.clone())?;
        self.values.push(stored);
        Ok(())
    }

    pub fn encoded_len(&self, field: u32) -> usize {
        self.values
            .iter()
            .map(|v| encode::encoded_len_len_field(field, T::as_bytes(v).len()))
            .sum()
    }

    pub fn encode_raw<B: BufMut>(&self, field: u32, buf: &mut B) {
        for v in &self.values {
            encode::encode_len_field(field, T::as_bytes(v), buf);
        }
    }

    pub fn merge<P, B: Buf>(
        &mut self,
        common: &MessageCommon<P, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        P: PresenceBits,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let stored = T::decode(buf, common.alloc.clone())?;
        self.values.push(stored);
        Ok(())
    }
}

impl<T: LenProtoType, A: Allocator + Clone> Clone for RepeatedLenField<T, A>
where
    T::Storage<A>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }
}

impl<T: LenProtoType, A: Allocator + Clone> PartialEq for RepeatedLenField<T, A>
where
    T::Storage<A>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<T: LenProtoType, A: Allocator + Clone> Eq for RepeatedLenField<T, A> where T::Storage<A>: Eq {}

impl<T: LenProtoType, A: Allocator + Clone> ::core::fmt::Debug for RepeatedLenField<T, A>
where
    T::Storage<A>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepeatedLenField")
            .field(&self.values)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type RepeatedLen<T, A> = RepeatedLenField<T, A>;
pub type RepeatedString<A> = RepeatedLenField<len::ProtoString, A>;
pub type RepeatedBytes<A> = RepeatedLenField<len::ProtoBytes, A>;
