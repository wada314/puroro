//! Value-storage layout for singular fields (`Inline` vs bit-packed bool).
//!
//! Orthogonal to [`FieldPresence`](super::field_presence::FieldPresence):
//! presence decides *whether* a field is set; layout decides *where the value
//! bytes / bits live*.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::{
    order::Lsb0,
    ptr::{BitRef, Mut},
};
use ::puroro::{DecodeBuf, DecodeError, WireType};

use ::unmanaged::DeallocateIn;

use super::{
    DefaultIn, MessageCommon, MessageCommonBits,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};
use crate::decode;
use crate::fields::wire::numerical::ProtoBool;
use crate::fields::wire::numerical::{BoolCodec, NumericalType};
use crate::fields::wire::singular_type::{PayloadAccess, SingularType};
use crate::fields::wire::wire_payload::{CopyWirePayload, VarintPayload};

/// Where a singular field's logical value is stored.
pub trait ValueLayout<T: SingularType, A: Allocator + Clone>: Copy
where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    fn is_proto_empty<Pb>(slot: &T::Slot<A>, common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    fn get<'a, Pb>(slot: &'a T::Slot<A>, common: &'a MessageCommon<Pb, A>) -> T::View<'a, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits;

    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> T::Mut<'a, A>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        T: 'a,
        A: 'a;

    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: T::Written<A>,
    ) where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits;

    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits;

    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: u32,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf;
}

/// Value lives in the field slot payload (`T::Slot<A>`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Inline;

impl<T: PayloadAccess, A: Allocator + Clone> ValueLayout<T, A> for Inline
where
    T::Slot<A>: AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    #[inline]
    fn is_proto_empty<Pb>(slot: &T::Slot<A>, common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        T::is_proto_empty(slot, common)
    }

    #[inline]
    fn get<'a, Pb>(slot: &'a T::Slot<A>, common: &'a MessageCommon<Pb, A>) -> T::View<'a, A>
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        T::get(slot, common)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> T::Mut<'a, A>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        T: 'a,
        A: 'a,
    {
        T::with_mut(slot, init, common)
    }

    #[inline]
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: T::Written<A>,
    ) where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        T::write(slot, init, common, value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        T::clear(slot, init, common);
    }

    #[inline]
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: u32,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<T::Slot<A>, A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        T::merge(slot, init, common, wire_type, buf, field, depth)
    }
}

/// Logical `bool` packed at `VALUE_BIT` in [`MessageCommon`]'s bitvec.
///
/// Used only with [`ProtoBool`]. The field slot remains a ZST for presence/init.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BitPacked<const VALUE_BIT: usize>;

impl<A: Allocator + Clone, const VALUE_BIT: usize> ValueLayout<ProtoBool, A>
    for BitPacked<VALUE_BIT>
where
    (): AddressableSlot + DefaultIn<A> + DeallocateIn<A>,
{
    #[inline]
    fn is_proto_empty<Pb>(_slot: &(), common: &MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        !common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn get<'a, Pb>(_slot: &'a (), common: &'a MessageCommon<Pb, A>) -> bool
    where
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> BitRef<'a, Mut, u8, Lsb0>
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        ProtoBool: 'a,
        A: 'a,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.bit_mut(VALUE_BIT)
    }

    #[inline]
    fn write<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>, value: bool)
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.set_bit(VALUE_BIT, value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
    {
        common.set_bit(VALUE_BIT, false);
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: WireType,
        buf: &mut B,
        field: u32,
        _depth: usize,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<(), A>,
        I: SlotInitMut,
        MessageCommon<Pb, A>: MessageCommonBits,
        B: DecodeBuf,
    {
        match BoolCodec::from_wire_body(VarintPayload::decode(wire_type, buf)?) {
            Ok(new) => {
                Self::write(slot, init, common, new);
                Ok(())
            }
            Err(DecodeError::UnknownClosedEnum { raw }) => {
                decode::save_unknown_varint_field(
                    field,
                    raw,
                    &mut common.unknown_fields,
                    common.alloc.clone(),
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
