//! Length-delimited scalar markers ([`ProtoString`] / [`ProtoBytes`] /
//! [`ProtoStringUnchecked`]).
//!
//! Markers are [`LenScalar`] wrappers around a [`LenCodec`] (Method 1 style):
//! blankets go on `LenScalar<C>`, so they do not collide with
//! [`Numerical`](super::numerical::Numerical)`<C>` blankets.
//!
//! Repeated / map storage uses [`UnmanagedString`](::unmanaged::UnmanagedString)
//! / [`UnmanagedVec`](::unmanaged::UnmanagedVec). Singular `string` / `bytes` use
//! SSO via [`InlineOrHeap`](crate::fields::shared::value_layout::InlineOrHeap)
//! (`ProtoString` → [`SsoString`](crate::fields::wire::sso_string::SsoString);
//! `ProtoBytes` / `ProtoStringUnchecked` → [`SsoBytes`](crate::fields::wire::sso_bytes::SsoBytes).
//! [`Inline`](crate::fields::shared::value_layout::Inline) keeps the heap
//! [`UnmanagedString`] / [`UnmanagedVec`] slot).

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::cmp::PartialEq;
use ::core::fmt::Debug;
use ::core::marker::PhantomData;
use ::core::ops::{Deref, DerefMut};
use ::unmanaged::string::StringGuard;
use ::unmanaged::vec::VecGuard;
use ::unmanaged::{DeallocateIn, DefaultIn, UnmanagedString, UnmanagedVec};

use ::puroro::{DecodeBuf, DecodeError};

use crate::decode;
use crate::fields::shared::value_slot::AddressableSlot;

/// Codec differences between LEN scalars (`string` vs `bytes`).
///
/// `pub` because it appears in bounds of public trait impls on [`LenScalar`]
/// (same reason [`NumericalType`](super::numerical::NumericalType) is public).
/// Not re-exported from the crate root — generated code uses [`ProtoString`] /
/// [`ProtoBytes`] only.
pub trait LenCodec: Sized {
    /// Unsized borrowed view (`str` / `[u8]`).
    type RefView: ?Sized + Debug + PartialEq;

    /// Singular / repeated physical slot.
    type Slot<A: Allocator>: AddressableSlot
        + DefaultIn<A>
        + DeallocateIn<A>
        + Deref<Target = Self::RefView>;

    /// Target of [`Mut`](Self::Mut) / repeated element mut
    /// ([`unmanaged::String`](::unmanaged::String) / `Vec<u8, A>`).
    type MutTarget<A: Allocator>: ?Sized;

    /// Singular / repeated mutable handle (`StringGuard` / `VecGuard`).
    type Mut<'a, A: Allocator>: DerefMut<Target = Self::MutTarget<A>>
    where
        Self: 'a,
        A: 'a;

    fn new_slot<A: Allocator + Clone>(alloc: A) -> Self::Slot<A>;

    fn decode_in<B: DecodeBuf, A: Allocator + Clone>(
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Slot<A>, DecodeError>;

    /// # Safety
    ///
    /// `alloc` must own `slot`'s buffer.
    unsafe fn slot_with_alloc<'a, A: Allocator + Clone + 'a>(
        slot: &'a mut Self::Slot<A>,
        alloc: A,
    ) -> Self::Mut<'a, A>
    where
        Self: 'a;

    /// Wire payload bytes for a borrowed view (UTF-8 bytes for `string`).
    fn as_wire_bytes(view: &Self::RefView) -> &[u8];
}

/// Length-delimited scalar protobuf type marker, parametrised by [`LenCodec`].
///
/// Public only so [`ProtoString`] / [`ProtoBytes`] aliases can be crate-root
/// re-exports; prefer those aliases in generated code. Not re-exported from
/// the crate root.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LenScalar<C>(PhantomData<C>);

/// Protobuf `string` codec (`utf8_validation=VERIFY`; see [`LenScalar`] note).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct StringCodec;

/// Protobuf `string` codec (`utf8_validation=NONE`; bytes view, map-key capable).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UncheckedStringCodec;

/// Protobuf `bytes` codec (see [`LenScalar`] visibility note).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BytesCodec;

/// LEN codecs whose borrowed view is `[u8]` (`bytes` and `utf8_validation=NONE` strings).
///
/// `pub` because it appears in bounds of public [`LenScalar`] impls. Not
/// re-exported from the crate root.
pub trait BytesLikeLenCodec: LenCodec<RefView = [u8]> {}

impl BytesLikeLenCodec for BytesCodec {}
impl BytesLikeLenCodec for UncheckedStringCodec {}

/// Protobuf `string` type marker (`utf8_validation=VERIFY`).
pub type ProtoString = LenScalar<StringCodec>;

/// Protobuf `string` type marker (`utf8_validation=NONE`).
///
/// Decode copies wire bytes without UTF-8 checks. Accessors yield `&[u8]`
/// (same views as [`ProtoBytes`]) so invalid UTF-8 is never presented as
/// `&str`. Distinct from [`ProtoBytes`] because `string` may be a map key.
pub type ProtoStringUnchecked = LenScalar<UncheckedStringCodec>;

/// Protobuf `bytes` type marker.
pub type ProtoBytes = LenScalar<BytesCodec>;

impl LenCodec for StringCodec {
    type RefView = str;
    type Slot<A: Allocator> = UnmanagedString<A>;
    type MutTarget<A: Allocator> = ::unmanaged::String<A>;
    type Mut<'a, A: Allocator>
        = StringGuard<'a, A>
    where
        Self: 'a,
        A: 'a;

    #[inline]
    fn new_slot<A: Allocator + Clone>(alloc: A) -> UnmanagedString<A> {
        UnmanagedString::new(alloc)
    }

    #[inline]
    fn decode_in<B: DecodeBuf, A: Allocator + Clone>(
        buf: &mut B,
        alloc: A,
    ) -> Result<UnmanagedString<A>, DecodeError> {
        decode::decode_string_in(buf, alloc)
    }

    #[inline]
    unsafe fn slot_with_alloc<'a, A: Allocator + Clone + 'a>(
        slot: &'a mut UnmanagedString<A>,
        alloc: A,
    ) -> StringGuard<'a, A>
    where
        Self: 'a,
    {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { slot.with_alloc(alloc) }
    }

    #[inline]
    fn as_wire_bytes(view: &str) -> &[u8] {
        view.as_bytes()
    }
}

impl LenCodec for UncheckedStringCodec {
    type RefView = [u8];
    type Slot<A: Allocator> = UnmanagedVec<u8, A>;
    type MutTarget<A: Allocator> = AllocVec<u8, A>;
    type Mut<'a, A: Allocator>
        = VecGuard<'a, u8, A>
    where
        Self: 'a,
        A: 'a;

    #[inline]
    fn new_slot<A: Allocator + Clone>(alloc: A) -> UnmanagedVec<u8, A> {
        BytesCodec::new_slot(alloc)
    }

    #[inline]
    fn decode_in<B: DecodeBuf, A: Allocator + Clone>(
        buf: &mut B,
        alloc: A,
    ) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        BytesCodec::decode_in(buf, alloc)
    }

    #[inline]
    unsafe fn slot_with_alloc<'a, A: Allocator + Clone + 'a>(
        slot: &'a mut UnmanagedVec<u8, A>,
        alloc: A,
    ) -> VecGuard<'a, u8, A>
    where
        Self: 'a,
    {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { BytesCodec::slot_with_alloc(slot, alloc) }
    }

    #[inline]
    fn as_wire_bytes(view: &[u8]) -> &[u8] {
        BytesCodec::as_wire_bytes(view)
    }
}

impl LenCodec for BytesCodec {
    type RefView = [u8];
    type Slot<A: Allocator> = UnmanagedVec<u8, A>;
    type MutTarget<A: Allocator> = AllocVec<u8, A>;
    type Mut<'a, A: Allocator>
        = VecGuard<'a, u8, A>
    where
        Self: 'a,
        A: 'a;

    #[inline]
    fn new_slot<A: Allocator + Clone>(alloc: A) -> UnmanagedVec<u8, A> {
        UnmanagedVec::new(alloc)
    }

    #[inline]
    fn decode_in<B: DecodeBuf, A: Allocator + Clone>(
        buf: &mut B,
        alloc: A,
    ) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        decode::decode_bytes_in(buf, alloc)
    }

    #[inline]
    unsafe fn slot_with_alloc<'a, A: Allocator + Clone + 'a>(
        slot: &'a mut UnmanagedVec<u8, A>,
        alloc: A,
    ) -> VecGuard<'a, u8, A>
    where
        Self: 'a,
    {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { slot.with_alloc(alloc) }
    }

    #[inline]
    fn as_wire_bytes(view: &[u8]) -> &[u8] {
        view
    }
}
