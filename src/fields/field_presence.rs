//! Field presence policy markers (`Implicit` / `Explicit<BIT>` / `LegacyRequired<BIT>` / [`Oneof`]).
//!
//! Composed with wire-encoding markers ([`VarintProtoType`](super::varint::VarintProtoType),
//! [`LenProtoType`](super::len::LenProtoType)) in singular field wrappers.
//!
//! Only [`Explicit`] and [`LegacyRequired`] carry a presence bit index; [`Implicit`] and
//! [`Oneof`] have none.

use ::allocator_api2::alloc::Allocator;

use crate::error::DecodeError;

use super::common::MessageCommon;
use super::presence::PresenceBits;

/// Encode / merge / clear behaviour for singular field presence.
pub trait FieldPresence: Copy {
    /// `true` when this field should be written on the wire.
    fn should_emit<P, A>(
        common: &MessageCommon<P, A>,
        payload_empty: bool,
    ) -> bool
    where
        P: PresenceBits,
        A: Allocator;

    /// Called after a successful merge or setter (marks EXPLICIT fields present).
    fn on_set<P, A>(common: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator;

    /// Called when an EXPLICIT field is cleared.
    fn on_clear<P, A>(common: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator;
}

/// Marker for IMPLICIT presence — omit on wire when payload is empty / type-zero.
/// Carries no presence bit index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Implicit;

impl FieldPresence for Implicit {
    fn should_emit<P, A>(_: &MessageCommon<P, A>, payload_empty: bool) -> bool
    where
        P: PresenceBits,
        A: Allocator,
    {
        !payload_empty
    }

    fn on_set<P, A>(_: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
    }

    fn on_clear<P, A>(_: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
    }
}

/// Marker for a **oneof variant** field — presence is tracked by the enclosing
/// [`OneofSlot`](super::oneof::OneofSlot), not by this wrapper. Always emits on
/// the wire when the variant is active (even when the payload is empty / type-zero).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Oneof;

impl FieldPresence for Oneof {
    fn should_emit<P, A>(_: &MessageCommon<P, A>, _: bool) -> bool
    where
        P: PresenceBits,
        A: Allocator,
    {
        true
    }

    fn on_set<P, A>(_: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
    }

    fn on_clear<P, A>(_: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
    }
}

/// Marker for EXPLICIT presence — tracked in the message bitfield at `BIT`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Explicit<const BIT: usize>;

impl<const BIT: usize> Default for Explicit<BIT> {
    fn default() -> Self {
        Self
    }
}

impl<const BIT: usize> FieldPresence for Explicit<BIT> {
    fn should_emit<P, A>(common: &MessageCommon<P, A>, _: bool) -> bool
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.is_present(BIT)
    }

    fn on_set<P, A>(common: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.set_presence(BIT, true);
    }

    fn on_clear<P, A>(common: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.set_presence(BIT, false);
    }
}

/// Marker for LEGACY_REQUIRED — wire/encode/merge identical to [`Explicit`];
/// message `validate()` must check the presence bit at `BIT`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LegacyRequired<const BIT: usize>;

impl<const BIT: usize> Default for LegacyRequired<BIT> {
    fn default() -> Self {
        Self
    }
}

impl<const BIT: usize> FieldPresence for LegacyRequired<BIT> {
    fn should_emit<P, A>(common: &MessageCommon<P, A>, _: bool) -> bool
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.is_present(BIT)
    }

    fn on_set<P, A>(common: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.set_presence(BIT, true);
    }

    fn on_clear<P, A>(common: &mut MessageCommon<P, A>)
    where
        P: PresenceBits,
        A: Allocator,
    {
        common.set_presence(BIT, false);
    }
}

/// Sub-trait for EXPLICIT-only accessors (`optional`, `has`, `clear`).
pub trait ExplicitFieldPresence: FieldPresence {
    /// Presence bit index in the message bitfield.
    const BIT: usize;
}

impl<const BIT: usize> ExplicitFieldPresence for Explicit<BIT> {
    const BIT: usize = BIT;
}

impl<const BIT: usize> ExplicitFieldPresence for LegacyRequired<BIT> {
    const BIT: usize = BIT;
}

/// Sub-trait for LEGACY_REQUIRED fields — adds presence validation for `validate()`.
pub trait RequiredFieldPresence: ExplicitFieldPresence {
    /// Returns `MissingRequiredField` when the bit is unset.
    fn validate_present<P, A>(
        common: &MessageCommon<P, A>,
        field_number: u32,
    ) -> Result<(), DecodeError>
    where
        P: PresenceBits,
        A: Allocator,
    {
        if common.is_present(Self::BIT) {
            Ok(())
        } else {
            Err(DecodeError::MissingRequiredField { field_number })
        }
    }
}

impl<const BIT: usize> RequiredFieldPresence for LegacyRequired<BIT> {}
