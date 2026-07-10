//! Field presence policy markers (`Implicit` / `Explicit<BIT>` / `LegacyRequired<BIT>` / [`Oneof`]).
//!
//! Composed with wire-encoding markers ([`VarintProtoType`](crate::fields::wire::varint::VarintProtoType),
//! [`LenProtoType`](crate::fields::wire::len::LenProtoType)) in singular field wrappers.
//!
//! Only [`Explicit`] and [`LegacyRequired`] carry a presence bit index; [`Implicit`] and
//! [`Oneof`] have none.

use ::allocator_api2::alloc::Allocator;
use ::core::mem::MaybeUninit;

use ::puroro::DecodeError;

use super::{
    slot_init::{AlwaysInitialized, BitInitMut, BitInitView, SlotInitMut, SlotInitView},
    value_slot::ValueSlot, DefaultIn, DeallocateIn, MessageCommon, PresenceBits, ProtoEmpty,
};

/// Encode / merge / clear behaviour for singular field presence.
pub trait FieldPresence: Copy {
    /// Stored value layout for singular scalar fields.
    ///
    /// [`Implicit`] and [`Oneof`] use always-initialized `T`; [`Explicit`] and
    /// [`LegacyRequired`] use [`MaybeUninit<T>`].
    type ValueSlot<T: DefaultIn + DeallocateIn>: ValueSlot<T>;

    /// Mutable init-state handle for value-slot mutation.
    type SlotInitMut<'a, P: PresenceBits + 'a, A: Allocator + 'a>: SlotInitMut;

    /// Read-only init-state view for value-slot reads.
    type SlotInitView<'a, P: PresenceBits + 'a, A: Allocator + 'a>: SlotInitView;

    /// Returns a mutable init-state handle bound to `common`.
    fn slot_init_mut<'a, Pb, A>(
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::SlotInitMut<'a, Pb, A>
    where
        Pb: PresenceBits,
        A: Allocator;

    /// Returns a read-only init-state view bound to `common`.
    fn slot_init_view<'a, Pb, A>(common: &'a MessageCommon<Pb, A>) -> Self::SlotInitView<'a, Pb, A>
    where
        Pb: PresenceBits,
        A: Allocator;

    /// `true` when the stored payload equals the protobuf empty / type-zero.
    ///
    /// Only [`Implicit`] consults the slot; bitfield-backed policies never call this.
    fn payload_is_empty<T: DefaultIn + DeallocateIn + ProtoEmpty>(slot: &Self::ValueSlot<T>) -> bool;

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
}

/// Marker for IMPLICIT presence — omit on wire when payload is empty / type-zero.
/// Carries no presence bit index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Implicit;

impl FieldPresence for Implicit {
    type ValueSlot<T: DefaultIn + DeallocateIn> = T;
    type SlotInitMut<'a, P: PresenceBits + 'a, A: Allocator + 'a> = AlwaysInitialized;
    type SlotInitView<'a, P: PresenceBits + 'a, A: Allocator + 'a> = AlwaysInitialized;

    fn slot_init_mut<'a, Pb, A>(_: &'a mut MessageCommon<Pb, A>) -> AlwaysInitialized
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        AlwaysInitialized
    }

    fn slot_init_view<'a, Pb, A>(_: &'a MessageCommon<Pb, A>) -> AlwaysInitialized
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        AlwaysInitialized
    }

    fn payload_is_empty<T: DefaultIn + DeallocateIn + ProtoEmpty>(slot: &T) -> bool {
        slot.is_proto_empty()
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
}

/// Marker for a **oneof variant** field — presence is tracked by the enclosing
/// [`OneofSlot`](super::oneof::OneofSlot), not by this wrapper. Always emits on
/// the wire when the variant is active (even when the payload is empty / type-zero).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Oneof;

impl FieldPresence for Oneof {
    type ValueSlot<T: DefaultIn + DeallocateIn> = T;
    type SlotInitMut<'a, P: PresenceBits + 'a, A: Allocator + 'a> = AlwaysInitialized;
    type SlotInitView<'a, P: PresenceBits + 'a, A: Allocator + 'a> = AlwaysInitialized;

    fn slot_init_mut<'a, Pb, A>(_: &'a mut MessageCommon<Pb, A>) -> AlwaysInitialized
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        AlwaysInitialized
    }

    fn slot_init_view<'a, Pb, A>(_: &'a MessageCommon<Pb, A>) -> AlwaysInitialized
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        AlwaysInitialized
    }

    fn payload_is_empty<T: DefaultIn + DeallocateIn + ProtoEmpty>(_slot: &T) -> bool {
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
    type ValueSlot<T: DefaultIn + DeallocateIn> = MaybeUninit<T>;
    type SlotInitMut<'a, P: PresenceBits + 'a, A: Allocator + 'a> = BitInitMut<'a, BIT, P, A>;
    type SlotInitView<'a, P: PresenceBits + 'a, A: Allocator + 'a> = BitInitView<'a, BIT, P, A>;

    fn slot_init_mut<'a, Pb, A>(
        common: &'a mut MessageCommon<Pb, A>,
    ) -> BitInitMut<'a, BIT, Pb, A>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        BitInitMut::new(common)
    }

    fn slot_init_view<'a, Pb, A>(
        common: &'a MessageCommon<Pb, A>,
    ) -> BitInitView<'a, BIT, Pb, A>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        BitInitView::new(common)
    }

    fn payload_is_empty<T: DefaultIn + DeallocateIn + ProtoEmpty>(_slot: &MaybeUninit<T>) -> bool {
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
    type ValueSlot<T: DefaultIn + DeallocateIn> = MaybeUninit<T>;
    type SlotInitMut<'a, P: PresenceBits + 'a, A: Allocator + 'a> = BitInitMut<'a, BIT, P, A>;
    type SlotInitView<'a, P: PresenceBits + 'a, A: Allocator + 'a> = BitInitView<'a, BIT, P, A>;

    fn slot_init_mut<'a, Pb, A>(
        common: &'a mut MessageCommon<Pb, A>,
    ) -> BitInitMut<'a, BIT, Pb, A>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        BitInitMut::new(common)
    }

    fn slot_init_view<'a, Pb, A>(
        common: &'a MessageCommon<Pb, A>,
    ) -> BitInitView<'a, BIT, Pb, A>
    where
        Pb: PresenceBits,
        A: Allocator,
    {
        BitInitView::new(common)
    }

    fn payload_is_empty<T: DefaultIn + DeallocateIn + ProtoEmpty>(_slot: &MaybeUninit<T>) -> bool {
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
