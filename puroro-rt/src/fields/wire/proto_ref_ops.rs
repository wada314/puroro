//! Per-marker [`PartialEq`] / [`Debug`] for [`EncodeType::View`](super::encode_type::EncodeType::View)
//! (e.g. `i32`, `&str`, `&M`) without `for<'a> View<'a, A>: …` HRTBs that force
//! `A: 'static` on nested messages.

use ::allocator_api2::alloc::Allocator;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};

use ::puroro::Message;

use super::len::{ProtoBytes, ProtoString};
use super::numerical::NumericalType;
use super::proto_message::ProtoMessage;
use super::singular_type::SingularType;
use super::varint::ProtoBool;
use crate::fields::shared::value_slot::AddressableSlot;

/// Equality for [`EncodeType::View`](super::encode_type::EncodeType::View)
/// (used by catalog [`FieldPartialEq`](crate::FieldPartialEq)).
pub trait ProtoRefEq<A: Allocator + Clone>: SingularType {
    fn option_eq<'a>(lhs: Option<Self::View<'a, A>>, rhs: Option<Self::View<'a, A>>) -> bool
    where
        A: 'a;
}

/// [`Debug`] for [`EncodeType::View`](super::encode_type::EncodeType::View)
/// (used by catalog [`FieldDebug`](crate::FieldDebug)).
pub trait ProtoRefDebug<A: Allocator + Clone>: SingularType {
    fn fmt_ref<'a>(value: &Self::View<'a, A>, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a;

    #[inline]
    fn fmt_option<'a>(value: Option<Self::View<'a, A>>, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        match value {
            None => f.write_str("None"),
            Some(v) => {
                f.write_str("Some(")?;
                Self::fmt_ref(&v, f)?;
                f.write_str(")")
            }
        }
    }
}

// Addressable numerical markers (`Ref = Value`): int / fixed / float / enum.
// `ProtoBool` is separate (logical `bool`, singular `Slot = ()`).
impl<T, A> ProtoRefEq<A> for T
where
    T: NumericalType,
    T::Value: PartialEq + AddressableSlot,
    A: Allocator + Clone,
{
    #[inline]
    fn option_eq<'a>(lhs: Option<T::Value>, rhs: Option<T::Value>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<T, A> ProtoRefDebug<A> for T
where
    T: NumericalType,
    T::Value: Debug + AddressableSlot,
    A: Allocator + Clone,
{
    #[inline]
    fn fmt_ref<'a>(value: &T::Value, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(value, f)
    }
}

impl<A: Allocator + Clone> ProtoRefEq<A> for ProtoBool {
    #[inline]
    fn option_eq<'a>(lhs: Option<bool>, rhs: Option<bool>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<A: Allocator + Clone> ProtoRefDebug<A> for ProtoBool {
    #[inline]
    fn fmt_ref<'a>(value: &bool, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(value, f)
    }
}

impl<A: Allocator + Clone> ProtoRefEq<A> for ProtoString {
    #[inline]
    fn option_eq<'a>(lhs: Option<&'a str>, rhs: Option<&'a str>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<A: Allocator + Clone> ProtoRefDebug<A> for ProtoString {
    #[inline]
    fn fmt_ref<'a>(value: &&'a str, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(value, f)
    }
}

impl<A: Allocator + Clone> ProtoRefEq<A> for ProtoBytes {
    #[inline]
    fn option_eq<'a>(lhs: Option<&'a [u8]>, rhs: Option<&'a [u8]>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<A: Allocator + Clone> ProtoRefDebug<A> for ProtoBytes {
    #[inline]
    fn fmt_ref<'a>(value: &&'a [u8], f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(value, f)
    }
}

/// Nested messages: bound is `M: PartialEq` / `Debug` (no `A: 'static` HRTB).
impl<M, A> ProtoRefEq<A> for ProtoMessage<M>
where
    M: Message<Alloc = A> + PartialEq,
    A: Allocator + Clone,
{
    #[inline]
    fn option_eq<'a>(lhs: Option<&'a M>, rhs: Option<&'a M>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<M, A> ProtoRefDebug<A> for ProtoMessage<M>
where
    M: Message<Alloc = A> + Debug,
    A: Allocator + Clone,
{
    #[inline]
    fn fmt_ref<'a>(value: &&'a M, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(*value, f)
    }
}
