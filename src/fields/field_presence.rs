//! Field presence policy markers (`Implicit` / `Explicit`).
//!
//! Composed with wire-encoding markers ([`VarintProtoType`](super::varint::VarintProtoType),
//! [`LenProtoType`](super::len::LenProtoType)) in singular field wrappers.

use ::allocator_api2::alloc::Allocator;

use super::common::MessageCommon;
use super::presence::PresenceBits;

/// Encode / merge / clear behaviour for singular field presence.
pub trait FieldPresence: Copy {
    /// `true` when this field should be written on the wire.
    fn should_emit<P, A>(
        common: &MessageCommon<P, A>,
        bit: usize,
        payload_empty: bool,
    ) -> bool
    where
        P: PresenceBits,
        A: Allocator;

    /// Called after a successful merge or setter (marks EXPLICIT fields present).
    fn on_set<P, A>(common: &mut MessageCommon<P, A>, bit: usize)
    where
        P: PresenceBits,
        A: Allocator;

    /// Called when an EXPLICIT field is cleared.
    fn on_clear<P, A>(common: &mut MessageCommon<P, A>, bit: usize)
    where
        P: PresenceBits,
        A: Allocator;
}

/// Marker for IMPLICIT presence — omit on wire when payload is empty / type-zero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Implicit;

impl FieldPresence for Implicit {
    fn should_emit<P, A>(_: &MessageCommon<P, A>, _: usize, payload_empty: bool) -> bool
    where
        P: PresenceBits,
        A: Allocator,
    {
        !payload_empty
    }

    fn on_set<P, A>(_: &mut MessageCommon<P, A>, _: usize)
    where
        P: PresenceBits,
        A: Allocator,
    {
    }

    fn on_clear<P, A>(_: &mut MessageCommon<P, A>, _: usize)
    where
        P: PresenceBits,
        A: Allocator,
    {
    }
}

/// Marker for EXPLICIT presence — tracked in the message bitfield.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Explicit;

impl FieldPresence for Explicit {
    fn should_emit<P, A>(common: &MessageCommon<P, A>, bit: usize, _: bool) -> bool
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.is_present(bit)
    }

    fn on_set<P, A>(common: &mut MessageCommon<P, A>, bit: usize)
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.set_presence(bit, true);
    }

    fn on_clear<P, A>(common: &mut MessageCommon<P, A>, bit: usize)
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.set_presence(bit, false);
    }
}

/// Sub-trait for EXPLICIT-only accessors (`optional`, `has`, `clear`).
pub trait ExplicitFieldPresence: FieldPresence {}

impl ExplicitFieldPresence for Explicit {}
