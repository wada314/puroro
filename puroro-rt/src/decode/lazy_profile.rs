//! Manifest for a lazy message's island buffer and singular LEN read policy.
//!
//! [`Owned`] keeps a [`Bytes`](bytes::Bytes) island and promotes string / bytes
//! on get ([`WireOrSso`](crate::fields::wire::wire_or_sso::WireOrSso) cache).
//! [`Borrowed`] stores `&[u8]` and peeks without promoting. Returned string /
//! bytes views are tied to the `&self` borrow of the message (`&'msg str`);
//! for [`Borrowed`], that shortens the input-buffer lifetime when the message
//! borrow is shorter. Slot layout is still `WireOrSso` for both in this phase;
//! a span-only store for no-cache profiles is a follow-up.

use ::allocator_api2::alloc::Allocator;
use ::bytes::Bytes;
use ::core::marker::PhantomData;
use ::puroro::DecodeError;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use super::wire_buf::WireBuf;
use crate::fields::wire::{WireOrSsoKind, WireOrSsoSlot};

/// Island buffer plus singular string / bytes get policy for a lazy message.
pub trait LazyProfile: Clone {
    /// Contiguous wire handle stored on [`super::LazyScan`].
    type Buf: WireBuf;

    /// Read a present `WireOrSso` string slot.
    ///
    /// The returned `&'msg str` is tied to the message / wire borrow. For
    /// [`Borrowed`], the input buffer must outlive `'msg` (true while the
    /// message still holds that buffer).
    fn get_str<'msg, A: Allocator + Clone>(
        slot: &'msg WireOrSsoSlot<UnmanagedString<A>>,
        arm: WireOrSsoKind,
        wire: &'msg Self::Buf,
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'msg str, DecodeError>
    where
        Self: 'msg;

    /// Read a present `WireOrSso` bytes slot.
    fn get_bytes<'msg, A: Allocator + Clone>(
        slot: &'msg WireOrSsoSlot<UnmanagedVec<u8, A>>,
        arm: WireOrSsoKind,
        wire: &'msg Self::Buf,
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'msg [u8], DecodeError>
    where
        Self: 'msg;
}

/// Owned island (`Bytes`) with UTF-8 / bytes promote on get.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Owned;

impl LazyProfile for Owned {
    type Buf = Bytes;

    #[inline]
    fn get_str<'msg, A: Allocator + Clone>(
        slot: &'msg WireOrSsoSlot<UnmanagedString<A>>,
        arm: WireOrSsoKind,
        wire: &'msg Bytes,
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'msg str, DecodeError>
    where
        Self: 'msg,
    {
        slot.get_str(arm, wire.as_bytes(), alloc, set_arm)
    }

    #[inline]
    fn get_bytes<'msg, A: Allocator + Clone>(
        slot: &'msg WireOrSsoSlot<UnmanagedVec<u8, A>>,
        arm: WireOrSsoKind,
        wire: &'msg Bytes,
        alloc: A,
        set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'msg [u8], DecodeError>
    where
        Self: 'msg,
    {
        slot.get_bytes(arm, wire.as_bytes(), alloc, set_arm)
    }
}

/// Borrowed island (`&[u8]`) with peek-on-get (no promote).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Borrowed<'buf> {
    _buf: PhantomData<&'buf [u8]>,
}

impl<'buf> LazyProfile for Borrowed<'buf> {
    type Buf = &'buf [u8];

    #[inline]
    fn get_str<'msg, A: Allocator + Clone>(
        slot: &'msg WireOrSsoSlot<UnmanagedString<A>>,
        arm: WireOrSsoKind,
        wire: &'msg &'buf [u8],
        _alloc: A,
        _set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'msg str, DecodeError>
    where
        Self: 'msg,
    {
        // `'buf: 'msg` from `Self: 'msg`; shorten the peek to the message borrow.
        slot.peek_str(arm, wire)
    }

    #[inline]
    fn get_bytes<'msg, A: Allocator + Clone>(
        slot: &'msg WireOrSsoSlot<UnmanagedVec<u8, A>>,
        arm: WireOrSsoKind,
        wire: &'msg &'buf [u8],
        _alloc: A,
        _set_arm: impl FnOnce(WireOrSsoKind),
    ) -> Result<&'msg [u8], DecodeError>
    where
        Self: 'msg,
    {
        slot.peek_bytes(arm, wire)
    }
}
