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
use ::bytes::Buf;

use ::puroro::DecodeError;

use super::{
    MessageCommon, PresenceBits,
    slot_init::SlotInitMut,
    value_slot::{ValueSlot, ValueSlotMutAccess},
};
use crate::decode;
use crate::fields::wire::proto_type::{PayloadAccess, ProtoType};
use crate::fields::wire::varint::ProtoBool;

/// Where a singular field's logical value is stored.
pub trait ValueLayout<T: ProtoType>: Copy {
    fn is_proto_empty<Pb: PresenceBits>(
        slot: &T::Slot,
        common: &MessageCommon<Pb, T::Alloc>,
    ) -> bool;

    fn get<'a, Pb: PresenceBits>(
        slot: &'a T::Slot,
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> T::Ref<'a>;

    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, T::Alloc>,
    ) -> T::Mut<'a>
    where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        T: 'a;

    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, T::Alloc>,
        value: T::Written,
    ) where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits;

    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, T::Alloc>)
    where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits;

    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, T::Alloc>,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: u32,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf;
}

/// Value lives in the field slot payload (`T::Slot`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Inline;

impl<T: PayloadAccess> ValueLayout<T> for Inline {
    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(
        slot: &T::Slot,
        common: &MessageCommon<Pb, T::Alloc>,
    ) -> bool {
        T::is_proto_empty(slot, common)
    }

    #[inline]
    fn get<'a, Pb: PresenceBits>(
        slot: &'a T::Slot,
        common: &'a MessageCommon<Pb, T::Alloc>,
    ) -> T::Ref<'a> {
        T::get(slot, common)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, T::Alloc>,
    ) -> T::Mut<'a>
    where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        T: 'a,
    {
        T::with_mut(slot, init, common)
    }

    #[inline]
    fn write<VS, I, Pb>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, T::Alloc>,
        value: T::Written,
    ) where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        T::write(slot, init, common, value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, T::Alloc>)
    where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        T::clear(slot, init, common);
    }

    #[inline]
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, T::Alloc>,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: u32,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<T::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf,
    {
        T::merge(slot, init, common, wire_type, buf, field)
    }
}

/// Logical `bool` packed at `VALUE_BIT` in [`MessageCommon`]'s bitvec.
///
/// Used only with [`ProtoBool`]. The field slot remains a ZST for presence/init.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BitPacked<const VALUE_BIT: usize>;

impl<A: Allocator + Clone, const VALUE_BIT: usize> ValueLayout<ProtoBool<A>>
    for BitPacked<VALUE_BIT>
{
    #[inline]
    fn is_proto_empty<Pb: PresenceBits>(
        _slot: &ProtoBool<A>,
        common: &MessageCommon<Pb, A>,
    ) -> bool {
        !common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn get<'a, Pb: PresenceBits>(
        _slot: &'a ProtoBool<A>,
        common: &'a MessageCommon<Pb, A>,
    ) -> bool {
        common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> BitRef<'a, Mut, u8, Lsb0>
    where
        VS: ValueSlot<ProtoBool<A>>,
        I: SlotInitMut,
        Pb: PresenceBits,
        ProtoBool<A>: 'a,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.bit_mut(VALUE_BIT)
    }

    #[inline]
    fn write<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>, value: bool)
    where
        VS: ValueSlot<ProtoBool<A>>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.set_bit(VALUE_BIT, value);
    }

    #[inline]
    fn clear<VS, I, Pb>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<ProtoBool<A>>,
        I: SlotInitMut,
        Pb: PresenceBits,
    {
        common.set_bit(VALUE_BIT, false);
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn merge<VS, I, Pb, B>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        wire_type: ::puroro::WireType,
        buf: &mut B,
        field: u32,
    ) -> Result<(), DecodeError>
    where
        VS: ValueSlot<ProtoBool<A>>,
        I: SlotInitMut,
        Pb: PresenceBits,
        B: Buf,
    {
        match ProtoBool::<A>::decode(wire_type, buf, common.alloc.clone()) {
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
