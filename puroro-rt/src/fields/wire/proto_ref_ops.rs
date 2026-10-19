//! Per-marker [`PartialEq`] / [`Debug`] for [`EncodeType::View`](super::encode_type::EncodeType::View)
//! (e.g. `i32`, `&str`, `&M`) without `for<'a> View<'a, A>: …` HRTBs that force
//! `A: 'static` on nested messages.

use ::allocator_api2::alloc::Allocator;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};

use super::len::{BytesLikeLenCodec, LenScalar, StringCodec};
use super::numerical::{Numerical, NumericalType};
use super::proto_message::ProtoMessage;
use super::singular_type::SingularType;
use crate::fields::shared::value_slot::AddressableSlot;
use crate::message_encode::MessageEncode;

/// Equality for [`EncodeType::View`](super::encode_type::EncodeType::View)
/// (used by catalog [`FieldPartialEq`](crate::fields::shared::field_inspect::FieldPartialEq)).
pub(crate) trait ProtoRefEq<A: Allocator>: SingularType {
    fn option_eq<'a>(lhs: Option<Self::View<'a, A>>, rhs: Option<Self::View<'a, A>>) -> bool
    where
        A: 'a;
}

/// [`Debug`] for [`EncodeType::View`](super::encode_type::EncodeType::View)
/// (used by catalog [`FieldDebug`](crate::fields::shared::field_inspect::FieldDebug)).
pub(crate) trait ProtoRefDebug<A: Allocator>: SingularType {
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

// Addressable numerical markers (`Ref = NativeType`): int / fixed / float / bool / enum.
impl<C, A> ProtoRefEq<A> for Numerical<C>
where
    C: NumericalType,
    C::NativeType: PartialEq + AddressableSlot,
    A: Allocator,
{
    #[inline]
    fn option_eq<'a>(lhs: Option<C::NativeType>, rhs: Option<C::NativeType>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<C, A> ProtoRefDebug<A> for Numerical<C>
where
    C: NumericalType,
    C::NativeType: Debug + AddressableSlot,
    A: Allocator,
{
    #[inline]
    fn fmt_ref<'a>(value: &C::NativeType, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(value, f)
    }
}

impl<A: Allocator> ProtoRefEq<A> for LenScalar<StringCodec> {
    #[inline]
    fn option_eq<'a>(lhs: Option<&'a str>, rhs: Option<&'a str>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<A: Allocator> ProtoRefDebug<A> for LenScalar<StringCodec> {
    #[inline]
    fn fmt_ref<'a>(value: &&'a str, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(value, f)
    }
}

impl<A: Allocator, C: BytesLikeLenCodec> ProtoRefEq<A> for LenScalar<C> {
    #[inline]
    fn option_eq<'a>(lhs: Option<&'a [u8]>, rhs: Option<&'a [u8]>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<A: Allocator, C: BytesLikeLenCodec> ProtoRefDebug<A> for LenScalar<C> {
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
    M: MessageEncode + PartialEq,
    A: Allocator,
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
    M: MessageEncode + Debug,
    A: Allocator,
{
    #[inline]
    fn fmt_ref<'a>(value: &&'a M, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(*value, f)
    }
}
