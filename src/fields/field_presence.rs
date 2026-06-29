//! Field presence policy markers (`Implicit` / `Explicit` / `LegacyRequired`).
//!
//! Composed with wire-encoding markers ([`VarintProtoType`](super::varint::VarintProtoType),
//! [`LenProtoType`](super::len::LenProtoType)) in singular field wrappers.

use ::allocator_api2::alloc::Allocator;

use crate::error::DecodeError;

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

/// Marker for LEGACY_REQUIRED — wire/encode/merge identical to [`Explicit`];
/// message `validate()` must check the presence bit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LegacyRequired;

impl FieldPresence for LegacyRequired {
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
impl ExplicitFieldPresence for LegacyRequired {}

/// Sub-trait for LEGACY_REQUIRED fields — adds presence validation for `validate()`.
pub trait RequiredFieldPresence: ExplicitFieldPresence {
    /// Returns `MissingRequiredField` when the bit is unset.
    fn validate_present<P, A>(
        common: &MessageCommon<P, A>,
        bit: usize,
        field_number: u32,
    ) -> Result<(), DecodeError>
    where
        P: PresenceBits,
        A: Allocator;
}

impl RequiredFieldPresence for LegacyRequired {
    fn validate_present<P, A>(
        common: &MessageCommon<P, A>,
        bit: usize,
        field_number: u32,
    ) -> Result<(), DecodeError>
    where
        P: PresenceBits,
        A: Allocator,
    {
        if common.is_present(bit) {
            Ok(())
        } else {
            Err(DecodeError::MissingRequiredField { field_number })
        }
    }
}
