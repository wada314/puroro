//! The [`Message`] trait shared by every generated protobuf message.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::error::DecodeError;
use crate::unknown::UnknownField;

/// Maximum nesting depth for nested-message decode.
///
/// Root messages start at depth `0`. Entering a child increments depth; when
/// `depth >= RECURSION_LIMIT`, decode returns
/// [`DecodeError::RecursionLimitExceeded`]. Matches the common protobuf C++
/// default of 100.
///
/// Enforced on nested-message decode paths; application code uses
/// [`Message::merge_from`] / [`Message::decode`].
pub const RECURSION_LIMIT: usize = 100;

/// Implemented by every generated message type (C++ `MessageLite`-like surface).
///
/// Field accessors stay as inherent methods on the generated struct so that
/// proto field names do not collide with these helpers. If a proto field is
/// named `validate`, `unknown_fields`, `encode_to_vec`, etc., the inherent
/// getter wins method resolution; call the trait method via UFCS
/// (e.g. [`Message::validate`]).
///
/// Wire body sizing / writing (`encoded_len` / `encode_raw` with an encode
/// context) lives on [`puroro_rt::MessageEncode`] — not on this trait. Depth-aware
/// merge lives on [`puroro_rt::MessageMerge`]. Generated code implements
/// [`encode`](Self::encode) / [`merge_from`](Self::merge_from) as thin wrappers;
/// [`encode_to_vec`](Self::encode_to_vec) / [`encode_to_bytes`](Self::encode_to_bytes)
/// / [`decode`](Self::decode) are convenience defaults on top of that.
///
/// The associated [`Alloc`](Self::Alloc) is the message's single allocator type
/// parameter. Nested fields construct children with
/// [`new_in`](Self::new_in) using the parent allocator
/// (`M: Message<Alloc = A> + unmanaged::DeallocateIn<A>` at catalog use sites).
/// Each concrete message must implement [`unmanaged::DeallocateIn`] for its
/// `Alloc` (orphan rules forbid a blanket impl on this trait). An inherent
/// `new()` for `Global` may still be provided on the concrete type.
pub trait Message: Sized {
    /// Allocator that owns this message's heap allocations.
    type Alloc: Allocator + Clone;

    /// Creates an empty message with the given allocator.
    fn new_in(alloc: Self::Alloc) -> Self;

    // -- codec --------------------------------------------------------------

    /// Appends the encoded message body to `buf`.
    ///
    /// The caller owns the destination buffer (and thus its allocator /
    /// capacity strategy). Generated impls forward to
    /// [`puroro_rt::encode_message`].
    fn encode<B: BufMut>(&self, buf: &mut B);

    /// Convenience: encodes into a new `Vec<u8>` via [`encode`](Self::encode).
    fn encode_to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        self.encode(&mut v);
        v
    }

    /// Convenience: encodes into [`bytes::Bytes`] via [`encode_to_vec`](Self::encode_to_vec).
    fn encode_to_bytes(&self) -> ::bytes::Bytes {
        ::bytes::Bytes::from(self.encode_to_vec())
    }

    /// Reads fields from `buf` and merges them into `self`.
    ///
    /// Generated impls forward to [`puroro_rt::merge_message`] (depth `0`).
    ///
    /// Merge semantics (identical across proto2, proto3, editions):
    /// - Singular scalar: last value seen wins.
    /// - Singular message: recursively merged.
    /// - Repeated: each occurrence appends to the list.
    /// - Unknown fields: accumulated for round-trip preservation.
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    /// Decodes a complete message. Provided; requires `Self: Default`.
    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError>
    where
        Self: Default,
    {
        let mut msg = Self::default();
        msg.merge_from(&mut buf)?;
        Ok(msg)
    }

    // -- infrastructure -----------------------------------------------------

    /// Iterates preserved unknown fields (and closed-enum unknowns diverted
    /// into the unknown set).
    fn unknown_fields(&self) -> impl Iterator<Item = UnknownField<'_>> + '_;

    /// Checks `LEGACY_REQUIRED` fields.
    ///
    /// Messages with no such fields return [`Ok`].
    fn validate(&self) -> Result<(), DecodeError>;
}
