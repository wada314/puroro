//! Per-marker [`PartialEq`] / [`Debug`] for [`ProtoType::Ref`] without
//! `for<'a> Ref<'a, A>: …` HRTBs that force `A: 'static` on nested messages.

use ::allocator_api2::alloc::Allocator;
use ::core::fmt::{Debug, Formatter, Result as FmtResult};

use ::puroro::Message;

use super::fixed::{
    ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoSFixed32, ProtoSFixed64,
};
use super::len::{ProtoBytes, ProtoString};
use super::proto_message::ProtoMessage;
use super::proto_type::ProtoType;
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoEnumStorage, ProtoInt32,
    ProtoInt64, ProtoSint32, ProtoSint64, ProtoUInt32, ProtoUInt64,
};

/// Equality for [`ProtoType::Ref`] views (used by catalog [`FieldPartialEq`](crate::FieldPartialEq)).
pub trait ProtoRefEq<A: Allocator + Clone>: ProtoType {
    fn option_eq<'a>(lhs: Option<Self::Ref<'a, A>>, rhs: Option<Self::Ref<'a, A>>) -> bool
    where
        A: 'a;
}

/// [`Debug`] for [`ProtoType::Ref`] views (used by catalog [`FieldDebug`](crate::FieldDebug)).
pub trait ProtoRefDebug<A: Allocator + Clone>: ProtoType {
    fn fmt_ref<'a>(value: &Self::Ref<'a, A>, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a;

    #[inline]
    fn fmt_option<'a>(value: Option<Self::Ref<'a, A>>, f: &mut Formatter<'_>) -> FmtResult
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

macro_rules! impl_proto_ref_ops_copy {
    ($($marker:ty),+ $(,)?) => {$(
        impl<A: Allocator + Clone> ProtoRefEq<A> for $marker {
            #[inline]
            fn option_eq<'a>(
                lhs: Option<Self::Ref<'a, A>>,
                rhs: Option<Self::Ref<'a, A>>,
            ) -> bool
            where
                A: 'a,
            {
                lhs == rhs
            }
        }

        impl<A: Allocator + Clone> ProtoRefDebug<A> for $marker {
            #[inline]
            fn fmt_ref<'a>(value: &Self::Ref<'a, A>, f: &mut Formatter<'_>) -> FmtResult
            where
                A: 'a,
            {
                Debug::fmt(value, f)
            }
        }
    )+};
}

impl_proto_ref_ops_copy!(
    ProtoInt32,
    ProtoInt64,
    ProtoUInt32,
    ProtoUInt64,
    ProtoSint32,
    ProtoSint64,
    ProtoBool,
    ProtoFixed32,
    ProtoFixed64,
    ProtoSFixed32,
    ProtoSFixed64,
    ProtoFloat,
    ProtoDouble,
    ProtoString,
    ProtoBytes,
);

impl<A: Allocator + Clone, E: ProtoEnumStorage + OpenEnum + PartialEq> ProtoRefEq<A>
    for ProtoEnum<E, Open>
{
    #[inline]
    fn option_eq<'a>(lhs: Option<Self::Ref<'a, A>>, rhs: Option<Self::Ref<'a, A>>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<A: Allocator + Clone, E: ProtoEnumStorage + ClosedEnum + PartialEq> ProtoRefEq<A>
    for ProtoEnum<E, Closed>
{
    #[inline]
    fn option_eq<'a>(lhs: Option<Self::Ref<'a, A>>, rhs: Option<Self::Ref<'a, A>>) -> bool
    where
        A: 'a,
    {
        lhs == rhs
    }
}

impl<A: Allocator + Clone, E: ProtoEnumStorage + OpenEnum + Debug> ProtoRefDebug<A>
    for ProtoEnum<E, Open>
{
    #[inline]
    fn fmt_ref<'a>(value: &Self::Ref<'a, A>, f: &mut Formatter<'_>) -> FmtResult
    where
        A: 'a,
    {
        Debug::fmt(value, f)
    }
}

impl<A: Allocator + Clone, E: ProtoEnumStorage + ClosedEnum + Debug> ProtoRefDebug<A>
    for ProtoEnum<E, Closed>
{
    #[inline]
    fn fmt_ref<'a>(value: &Self::Ref<'a, A>, f: &mut Formatter<'_>) -> FmtResult
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
