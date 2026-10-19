//! Field presence policy markers (`Implicit` / `Explicit<BIT>` / `LegacyRequired<BIT>` /
//! [`Message`] / [`Oneof`]).
//!
//! Composed with [`SingularType`](crate::fields::wire::singular_type::SingularType) markers in
//! singular field wrappers.
//!
//! Only [`Explicit`] and [`LegacyRequired`] carry a presence bit index; [`Implicit`],
//! [`Message`], and [`Oneof`] have none. [`Message`] uses pointer presence
//! (`Option` via [`ValueSlot`](super::value_slot::ValueSlot)).

use ::core::mem::MaybeUninit;

use ::puroro::DecodeError;

use super::{
    MessageCommonBits,
    slot_init::{AlwaysInitialized, BitInit, SlotInitMut, SlotInitView},
    value_slot::AddressableSlot,
};

/// Encode / merge / clear behaviour for singular field presence.
pub trait FieldPresence: Copy {
    /// Stored value layout for singular scalar fields.
    ///
    /// [`Implicit`] and [`Oneof`] use always-initialized `T`; [`Explicit`] and
    /// [`LegacyRequired`] use [`MaybeUninit<T>`].
    ///
    /// The `ValueSlot<T>` bound is enforced at use sites ([`SingularField`]).
    /// `T` here is the **slot** payload ([`ValueLayout::Slot`](crate::ValueLayout)),
    /// not the protobuf type marker.
    type ValueSlot<T: AddressableSlot>;

    /// Borrow-free mutable init-state marker.
    type SlotInitMut: SlotInitMut;

    /// Borrow-free read-only init-state marker.
    type SlotInitView: SlotInitView;

    /// Returns a mutable init-state marker.
    fn slot_init_mut() -> Self::SlotInitMut;

    /// Returns a read-only init-state marker.
    fn slot_init_view() -> Self::SlotInitView;

    /// `true` when this field should be written on the wire.
    ///
    /// `is_payload_empty` is evaluated only when the policy depends on the stored
    /// value (e.g. [`Implicit`]); bitfield-backed policies ignore it.
    /// Callers supply emptiness via [`ValueLayout::is_proto_empty`](super::value_layout::ValueLayout).
    fn should_emit<C: MessageCommonBits, F>(common: &C, is_payload_empty: F) -> bool
    where
        F: FnOnce() -> bool;

    /// `true` when the field is considered present for accessor APIs (`has_*`, `Optional`).
    ///
    /// `is_payload_empty` is evaluated only when the policy depends on the stored
    /// value (e.g. [`Implicit`]); bitfield-backed policies ignore it.
    /// Callers supply emptiness via [`ValueLayout::is_proto_empty`](super::value_layout::ValueLayout).
    fn is_set<C: MessageCommonBits, F>(common: &C, is_payload_empty: F) -> bool
    where
        F: FnOnce() -> bool;
}

/// Marker for IMPLICIT presence — omit on wire when payload is empty / type-zero.
/// Carries no presence bit index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Implicit;

impl FieldPresence for Implicit {
    type ValueSlot<T: AddressableSlot> = T;
    type SlotInitMut = AlwaysInitialized;
    type SlotInitView = AlwaysInitialized;

    fn slot_init_mut() -> AlwaysInitialized {
        AlwaysInitialized
    }

    fn slot_init_view() -> AlwaysInitialized {
        AlwaysInitialized
    }

    fn should_emit<C: MessageCommonBits, F>(_: &C, is_payload_empty: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        !is_payload_empty()
    }

    fn is_set<C: MessageCommonBits, F>(_: &C, is_payload_empty: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        !is_payload_empty()
    }
}

/// Marker for a singular nested-message field — presence is the slot itself
/// ([`Option`](core::option::Option) via [`ValueSlot`](super::value_slot::ValueSlot)),
/// not a message bitfield. Emit / `is_set` consult the caller callback (absent =
/// empty). Used for nested messages via [`ProtoMessage`](crate::ProtoMessage).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Message;

impl FieldPresence for Message {
    type ValueSlot<T: AddressableSlot> = Option<T>;
    type SlotInitMut = AlwaysInitialized;
    type SlotInitView = AlwaysInitialized;

    fn slot_init_mut() -> AlwaysInitialized {
        AlwaysInitialized
    }

    fn slot_init_view() -> AlwaysInitialized {
        AlwaysInitialized
    }

    fn should_emit<C: MessageCommonBits, F>(_: &C, is_payload_empty: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        !is_payload_empty()
    }

    fn is_set<C: MessageCommonBits, F>(_: &C, is_payload_empty: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        !is_payload_empty()
    }
}

/// Marker for a **oneof variant** field — presence is tracked by the enclosing
/// [`OneofSlot`](super::oneof::OneofSlot), not by this wrapper. Always emits on
/// the wire when the variant is active (even when the payload is empty / type-zero).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Oneof;

impl FieldPresence for Oneof {
    type ValueSlot<T: AddressableSlot> = T;
    type SlotInitMut = AlwaysInitialized;
    type SlotInitView = AlwaysInitialized;

    fn slot_init_mut() -> AlwaysInitialized {
        AlwaysInitialized
    }

    fn slot_init_view() -> AlwaysInitialized {
        AlwaysInitialized
    }

    fn should_emit<C: MessageCommonBits, F>(_: &C, _: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        true
    }

    fn is_set<C: MessageCommonBits, F>(_: &C, _: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        true
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
    type ValueSlot<T: AddressableSlot> = MaybeUninit<T>;
    type SlotInitMut = BitInit<BIT>;
    type SlotInitView = BitInit<BIT>;

    fn slot_init_mut() -> BitInit<BIT> {
        BitInit
    }

    fn slot_init_view() -> BitInit<BIT> {
        BitInit
    }

    fn should_emit<C: MessageCommonBits, F>(common: &C, _: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        common.is_bit_set(BIT)
    }

    fn is_set<C: MessageCommonBits, F>(common: &C, _: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        common.is_bit_set(BIT)
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
    type ValueSlot<T: AddressableSlot> = MaybeUninit<T>;
    type SlotInitMut = BitInit<BIT>;
    type SlotInitView = BitInit<BIT>;

    fn slot_init_mut() -> BitInit<BIT> {
        BitInit
    }

    fn slot_init_view() -> BitInit<BIT> {
        BitInit
    }

    fn should_emit<C: MessageCommonBits, F>(common: &C, _: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        common.is_bit_set(BIT)
    }

    fn is_set<C: MessageCommonBits, F>(common: &C, _: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        common.is_bit_set(BIT)
    }
}

/// Sub-trait for LEGACY_REQUIRED fields — adds presence validation for `validate()`.
pub(crate) trait RequiredFieldPresence: FieldPresence {
    /// Returns `MissingRequiredField` when the field is not present.
    fn validate_present<C: MessageCommonBits, F>(
        common: &C,
        field_number: u32,
        is_payload_empty: F,
    ) -> Result<(), DecodeError>
    where
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
