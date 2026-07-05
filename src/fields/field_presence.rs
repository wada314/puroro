//! Field presence policy markers (`Implicit` / `Explicit<BIT>` / `LegacyRequired<BIT>` / [`Oneof`]).
//!
//! Composed with wire-encoding markers ([`VarintProtoType`](super::varint::VarintProtoType),
//! [`LenProtoType`](super::len::LenProtoType)) in singular field wrappers.
//!
//! Only [`Explicit`] and [`LegacyRequired`] carry a presence bit index; [`Implicit`] and
//! [`Oneof`] have none.

use ::allocator_api2::alloc::Allocator;
use ::core::mem::MaybeUninit;

use crate::error::DecodeError;

use super::common::MessageCommon;
use super::presence::PresenceBits;
use super::proto_zero::ProtoZero;
use super::value_slot::ValueSlot;

/// Encode / merge / clear behaviour for singular field presence.
pub trait FieldPresence: Copy {
    /// Stored value layout for singular scalar fields.
    ///
    /// [`Implicit`] and [`Oneof`] use always-initialized `T`; [`Explicit`] and
    /// [`LegacyRequired`] use [`MaybeUninit<T>`].
    type ValueSlot<T: ProtoZero>: ValueSlot<T>;

    /// `true` when the stored payload equals the protobuf type-zero.
    ///
    /// Only [`Implicit`] consults the slot; bitfield-backed policies never call this.
    fn payload_is_empty<T: ProtoZero>(slot: &Self::ValueSlot<T>) -> bool;

    /// `true` when this field should be written on the wire.
    ///
    /// `is_payload_empty` is evaluated only when the policy depends on the stored
    /// value (e.g. [`Implicit`]); bitfield-backed policies ignore it.
    fn should_emit<P, A, F>(
        common: &MessageCommon<P, A>,
        is_payload_empty: F,
    ) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool;

    /// `true` when the field is considered present for accessor APIs (`has_*`, `Optional`).
    ///
    /// `is_payload_empty` is evaluated only when the policy depends on the stored
    /// value (e.g. [`Implicit`]); bitfield-backed policies ignore it.
    fn is_set<P, A, F>(common: &MessageCommon<P, A>, is_payload_empty: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool;

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

    /// Writes `value` into the slot (first write vs overwrite decided from `common`).
    fn write_slot<P, A, T>(
        common: &MessageCommon<P, A>,
        slot: &mut Self::ValueSlot<T>,
        value: T,
    ) where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero;

    /// Clears the payload slot using presence state in `common` (call before [`on_clear`](Self::on_clear)).
    fn clear_slot<P, A, T>(
        common: &MessageCommon<P, A>,
        slot: &mut Self::ValueSlot<T>,
    ) where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero;

    /// Marks the field present and returns a mutable reference to the initialized slot.
    fn prepare_mut_slot<'slot, P, A, T>(
        common: &mut MessageCommon<P, A>,
        slot: &'slot mut Self::ValueSlot<T>,
    ) -> &'slot mut T
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero;
}

/// Marker for IMPLICIT presence — omit on wire when payload is empty / type-zero.
/// Carries no presence bit index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Implicit;

impl FieldPresence for Implicit {
    type ValueSlot<T: ProtoZero> = T;

    fn payload_is_empty<T: ProtoZero>(slot: &T) -> bool {
        T::is_proto_zero(slot)
    }

    fn should_emit<P, A, F>(_: &MessageCommon<P, A>, is_payload_empty: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
    {
        !is_payload_empty()
    }

    fn is_set<P, A, F>(_: &MessageCommon<P, A>, is_payload_empty: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
    {
        !is_payload_empty()
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

    fn write_slot<P, A, T>(_: &MessageCommon<P, A>, slot: &mut T, value: T)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        unsafe { slot.write_dropping_previous(value) };
    }

    fn clear_slot<P, A, T>(_: &MessageCommon<P, A>, slot: &mut T)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        T::set_proto_zero(slot);
    }

    fn prepare_mut_slot<'slot, P, A, T>(
        _: &mut MessageCommon<P, A>,
        slot: &'slot mut T,
    ) -> &'slot mut T
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        slot
    }
}

/// Marker for a **oneof variant** field — presence is tracked by the enclosing
/// [`OneofSlot`](super::oneof::OneofSlot), not by this wrapper. Always emits on
/// the wire when the variant is active (even when the payload is empty / type-zero).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Oneof;

impl FieldPresence for Oneof {
    type ValueSlot<T: ProtoZero> = T;

    fn payload_is_empty<T: ProtoZero>(_slot: &T) -> bool {
        false
    }

    fn should_emit<P, A, F>(_: &MessageCommon<P, A>, _: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
    {
        true
    }

    fn is_set<P, A, F>(_: &MessageCommon<P, A>, _: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
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

    fn write_slot<P, A, T>(_: &MessageCommon<P, A>, slot: &mut T, value: T)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        unsafe { slot.write_dropping_previous(value) };
    }

    fn clear_slot<P, A, T>(_: &MessageCommon<P, A>, slot: &mut T)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        T::set_proto_zero(slot);
    }

    fn prepare_mut_slot<'slot, P, A, T>(
        _: &mut MessageCommon<P, A>,
        slot: &'slot mut T,
    ) -> &'slot mut T
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        slot
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
    type ValueSlot<T: ProtoZero> = MaybeUninit<T>;

    fn payload_is_empty<T: ProtoZero>(_slot: &MaybeUninit<T>) -> bool {
        false
    }

    fn should_emit<P, A, F>(common: &MessageCommon<P, A>, _: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
    {
        common.is_present(BIT)
    }

    fn is_set<P, A, F>(common: &MessageCommon<P, A>, _: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
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

    fn write_slot<P, A, T>(common: &MessageCommon<P, A>, slot: &mut MaybeUninit<T>, value: T)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        if common.is_present(BIT) {
            unsafe { slot.write_dropping_previous(value) };
        } else {
            unsafe { slot.write_without_drop(value) };
        }
    }

    fn clear_slot<P, A, T>(common: &MessageCommon<P, A>, slot: &mut MaybeUninit<T>)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        if common.is_present(BIT) {
            unsafe { slot.assume_init_drop() };
        }
    }

    fn prepare_mut_slot<'slot, P, A, T>(
        common: &mut MessageCommon<P, A>,
        slot: &'slot mut MaybeUninit<T>,
    ) -> &'slot mut T
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        let was_present = common.is_present(BIT);
        common.set_presence(BIT, true);
        if was_present {
            unsafe { slot.assume_init_mut() }
        } else {
            unsafe {
                T::set_proto_zero(&mut *slot.as_mut_ptr());
                slot.assume_init_mut()
            }
        }
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
    type ValueSlot<T: ProtoZero> = MaybeUninit<T>;

    fn payload_is_empty<T: ProtoZero>(_slot: &MaybeUninit<T>) -> bool {
        false
    }

    fn should_emit<P, A, F>(common: &MessageCommon<P, A>, _: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
    {
        common.is_present(BIT)
    }

    fn is_set<P, A, F>(common: &MessageCommon<P, A>, _: F) -> bool
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
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

    fn write_slot<P, A, T>(common: &MessageCommon<P, A>, slot: &mut MaybeUninit<T>, value: T)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        if common.is_present(BIT) {
            unsafe { slot.write_dropping_previous(value) };
        } else {
            unsafe { slot.write_without_drop(value) };
        }
    }

    fn clear_slot<P, A, T>(common: &MessageCommon<P, A>, slot: &mut MaybeUninit<T>)
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        if common.is_present(BIT) {
            unsafe { slot.assume_init_drop() };
        }
    }

    fn prepare_mut_slot<'slot, P, A, T>(
        common: &mut MessageCommon<P, A>,
        slot: &'slot mut MaybeUninit<T>,
    ) -> &'slot mut T
    where
        P: PresenceBits,
        A: Allocator,
        T: ProtoZero,
    {
        let was_present = common.is_present(BIT);
        common.set_presence(BIT, true);
        if was_present {
            unsafe { slot.assume_init_mut() }
        } else {
            unsafe {
                T::set_proto_zero(&mut *slot.as_mut_ptr());
                slot.assume_init_mut()
            }
        }
    }
}

/// Sub-trait for LEGACY_REQUIRED fields — adds presence validation for `validate()`.
pub trait RequiredFieldPresence: FieldPresence {
    /// Returns `MissingRequiredField` when the field is not present.
    fn validate_present<P, A, F>(
        common: &MessageCommon<P, A>,
        field_number: u32,
        is_payload_empty: F,
    ) -> Result<(), DecodeError>
    where
        P: PresenceBits,
        A: Allocator,
        F: FnOnce() -> bool,
    {
        if Self::is_set(common, is_payload_empty) {
            Ok(())
        } else {
            Err(DecodeError::MissingRequiredField { field_number })
        }
    }
}

impl<const BIT: usize> RequiredFieldPresence for LegacyRequired<BIT> {}
