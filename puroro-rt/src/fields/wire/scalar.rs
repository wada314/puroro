//! Unified wire/storage semantics for singular scalar fields (varint + LEN).
//!
//! Each implementor is a **protobuf type marker** (`ProtoInt32`, `ProtoBool`, …).
//! The physical field slot is the associated [`Slot`](ScalarProtoType::Slot)
//! (`Self` for addressable wrappers; `()` for bit-packed [`ProtoBool`]).
//!
//! Logical getter payloads implement [`ScalarRef`] (`is_empty`). Wire
//! `encoded_len` / `encode` stay on [`ScalarProtoType`] because multiple markers
//! can share the same `Ref` type (e.g. [`ProtoInt32`] and [`ProtoSint32`] both
//! use `i32`).
//!
//! [`VarintProtoType`](super::varint::VarintProtoType) and
//! [`LenProtoType`](super::len::LenProtoType) remain for **repeated** fields,
//! which keep storing the inner [`VarintProtoType::Value`] /
//! [`LenProtoType::Storage`] so public slices stay `&[i32]` / `&[UnmanagedString]`.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::ops::{Deref, DerefMut};

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::decode;
use crate::encode;
use crate::fields::shared::{
    MessageCommon, PresenceBits,
    slot_init::SlotInitMut,
    value_slot::{AddressableSlot, ValueSlot, ValueSlotMutAccess},
};

use super::len::{LenProtoType, ProtoBytes, ProtoString};
use super::varint::{ProtoBool, ProtoEnumStorage, VarintProtoType};

/// Logical getter payload for a singular scalar (`i32`, `bool`, `&str`, …).
///
/// Emptiness for IMPLICIT omit lives here. Wire encode stays on
/// [`ScalarProtoType`] because distinct markers may share the same `Ref` type.
pub trait ScalarRef {
    /// `true` when this value is protobuf empty / type-zero.
    fn is_empty(self) -> bool;
}

macro_rules! impl_scalar_ref_zero {
    ($ty:ty, $zero:expr) => {
        impl ScalarRef for $ty {
            #[inline]
            fn is_empty(self) -> bool {
                self == $zero
            }
        }
    };
}

impl_scalar_ref_zero!(i32, 0);
impl_scalar_ref_zero!(i64, 0);
impl_scalar_ref_zero!(u32, 0);
impl_scalar_ref_zero!(u64, 0);

impl ScalarRef for bool {
    #[inline]
    fn is_empty(self) -> bool {
        !self
    }
}

impl ScalarRef for &str {
    #[inline]
    fn is_empty(self) -> bool {
        self.is_empty()
    }
}

impl ScalarRef for &[u8] {
    #[inline]
    fn is_empty(self) -> bool {
        self.is_empty()
    }
}

impl<E: ProtoEnumStorage> ScalarRef for E {
    #[inline]
    fn is_empty(self) -> bool {
        self == E::proto_zero()
    }
}

/// Wire + accessor semantics for a singular scalar protobuf type.
///
/// The implementor is the **type marker** (not necessarily what sits in the
/// field struct). Physical storage is [`Slot`](Self::Slot):
/// - addressable wrappers: `Slot = Self` (thin payload in the field)
/// - [`ProtoBool`]: `Slot = ()` (logical `bool` in [`MessageCommon`] bitvec)
///
/// Interim: [`ProtoBool`] still carries `VALUE_BIT` as a const generic for
/// codegen stability; a future cleanup should move that index to the field /
/// layout side so the type marker is bit-index-free.
pub trait ScalarProtoType: Sized {
    /// Physical value stored in the singular field slot (excluding
    /// [`MessageCommon`] bits).
    type Slot: AddressableSlot;

    /// Borrowed / by-value view returned by getters (`i32`, `&str`, `bool`, …).
    type Ref<'a>: ScalarRef
    where
        Self: 'a;

    /// Mutable handle returned by `_mut` accessors (`&mut i32`, `StringGuard`,
    /// bit handle, …).
    type Mut<'a, A: Allocator + 'a>: DerefMut
    where
        Self: 'a;

    /// Value accepted by `set` / produced by [`decode`](Self::decode).
    type Written;

    /// Expected wire type for a singular occurrence of this field.
    const WIRE_TYPE: WireType;

    /// Reads the logical getter view from the slot and/or `common`.
    fn get<'a, Pb: PresenceBits, A: Allocator>(
        slot: &'a Self::Slot,
        common: &'a MessageCommon<Pb, A>,
    ) -> Self::Ref<'a>;

    /// Ensures the slot is present and returns a mutable accessor handle.
    ///
    /// Heap-backed mutators clone the allocator from `common.alloc`.
    fn with_mut<'a, VS, I, Pb, A>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a, A>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone + 'a;

    /// Writes `value`, ensuring slot presence when applicable.
    fn write<VS, I, Pb, A>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone;

    /// Clears the logical value and slot presence / payload.
    fn clear<VS, I, Pb, A>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone;

    /// Wire byte length of one tagged occurrence for `value`.
    ///
    /// Kept on the type marker (not [`ScalarRef`]) so int32 vs sint32 can share
    /// `Ref = i32` with different wire encodings.
    fn encoded_len(value: Self::Ref<'_>, field: u32) -> usize;

    /// Encodes one tagged occurrence for `value`.
    fn encode<B: BufMut>(value: Self::Ref<'_>, field: u32, buf: &mut B);

    /// Decodes one occurrence after the tag has been read (`wire_type` checked here).
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Written, DecodeError>;
}

// ---------------------------------------------------------------------------
// Addressable varint wrappers (Slot = Self): numerics + ProtoEnum
// ---------------------------------------------------------------------------

impl<T> ScalarProtoType for T
where
    T: VarintProtoType + AddressableSlot + From<T::Value> + 'static,
    T: Deref<Target = T::Value> + DerefMut,
    T::Value: ScalarRef,
{
    type Slot = Self;
    type Ref<'a>
        = T::Value
    where
        Self: 'a;
    type Mut<'a, A: Allocator + 'a>
        = &'a mut T::Value
    where
        Self: 'a;
    type Written = Self;
    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn get<'a, Pb: PresenceBits, A: Allocator>(
        slot: &'a Self::Slot,
        _common: &'a MessageCommon<Pb, A>,
    ) -> Self::Ref<'a> {
        *Deref::deref(slot)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb, A>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a, A>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone + 'a,
    {
        DerefMut::deref_mut(ValueSlot::with_mut(slot, init, common).get_mut())
    }

    #[inline]
    fn write<VS, I, Pb, A>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<VS, I, Pb, A>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len(value: Self::Ref<'_>, field: u32) -> usize {
        encode::encoded_len_varint_field(field, T::encode_wire(value))
    }

    #[inline]
    fn encode<B: BufMut>(value: Self::Ref<'_>, field: u32, buf: &mut B) {
        encode::encode_varint_field(field, T::encode_wire(value), buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: A,
    ) -> Result<Self::Written, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        Ok(Self::from(T::decode_wire(raw)?))
    }
}

// ---------------------------------------------------------------------------
// LEN wrappers (Slot = Self)
// ---------------------------------------------------------------------------

impl ScalarProtoType for ProtoString {
    type Slot = Self;
    type Ref<'a> = &'a str;
    type Mut<'a, A: Allocator + 'a> = <Self as LenProtoType>::Mut<'a, A>;
    type Written = Self;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn get<'a, Pb: PresenceBits, A: Allocator>(
        slot: &'a Self::Slot,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a str {
        &slot.0
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb, A>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a, A>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone + 'a,
    {
        let alloc = common.alloc.clone();
        <Self as LenProtoType>::with_alloc(
            &mut ValueSlot::with_mut(slot, init, common).get_mut().0,
            alloc,
        )
    }

    #[inline]
    fn write<VS, I, Pb, A>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<VS, I, Pb, A>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len(value: Self::Ref<'_>, field: u32) -> usize {
        encode::encoded_len_len_field(field, value.as_bytes().len())
    }

    #[inline]
    fn encode<B: BufMut>(value: Self::Ref<'_>, field: u32, buf: &mut B) {
        encode::encode_len_field(field, value.as_bytes(), buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Written, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        Ok(Self(<Self as LenProtoType>::decode(buf, alloc)?))
    }
}

impl ScalarProtoType for ProtoBytes {
    type Slot = Self;
    type Ref<'a> = &'a [u8];
    type Mut<'a, A: Allocator + 'a> = <Self as LenProtoType>::Mut<'a, A>;
    type Written = Self;
    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn get<'a, Pb: PresenceBits, A: Allocator>(
        slot: &'a Self::Slot,
        _common: &'a MessageCommon<Pb, A>,
    ) -> &'a [u8] {
        &slot.0
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb, A>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a, A>
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone + 'a,
    {
        let alloc = common.alloc.clone();
        <Self as LenProtoType>::with_alloc(
            &mut ValueSlot::with_mut(slot, init, common).get_mut().0,
            alloc,
        )
    }

    #[inline]
    fn write<VS, I, Pb, A>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: Self::Written,
    ) where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        ValueSlot::with_mut(slot, init, common).set(value);
    }

    #[inline]
    fn clear<VS, I, Pb, A>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<Self::Slot>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len(value: Self::Ref<'_>, field: u32) -> usize {
        encode::encoded_len_len_field(field, value.len())
    }

    #[inline]
    fn encode<B: BufMut>(value: Self::Ref<'_>, field: u32, buf: &mut B) {
        encode::encode_len_field(field, value, buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Written, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        Ok(Self(<Self as LenProtoType>::decode(buf, alloc)?))
    }
}

// ---------------------------------------------------------------------------
// Bit-packed bool (Slot = ())
// ---------------------------------------------------------------------------

impl<const VALUE_BIT: usize> ScalarProtoType for ProtoBool<VALUE_BIT> {
    /// Unit slot: presence/init only. Logical `bool` lives at `VALUE_BIT` in
    /// [`MessageCommon`].
    type Slot = ();
    type Ref<'a> = bool;
    type Mut<'a, A: Allocator + 'a> =
        ::bitvec::ptr::BitRef<'a, ::bitvec::ptr::Mut, u8, ::bitvec::order::Lsb0>;
    type Written = bool;
    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn get<'a, Pb: PresenceBits, A: Allocator>(
        _slot: &'a (),
        common: &'a MessageCommon<Pb, A>,
    ) -> bool {
        common.is_bit_set(VALUE_BIT)
    }

    #[inline]
    fn with_mut<'a, VS, I, Pb, A>(
        slot: &'a mut VS,
        init: I,
        common: &'a mut MessageCommon<Pb, A>,
    ) -> Self::Mut<'a, A>
    where
        VS: ValueSlot<()>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone + 'a,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.bit_mut(VALUE_BIT)
    }

    #[inline]
    fn write<VS, I, Pb, A>(
        slot: &mut VS,
        init: I,
        common: &mut MessageCommon<Pb, A>,
        value: bool,
    ) where
        VS: ValueSlot<()>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        let _ = ValueSlot::with_mut(slot, init, common).get_mut();
        common.set_bit(VALUE_BIT, value);
    }

    #[inline]
    fn clear<VS, I, Pb, A>(slot: &mut VS, init: I, common: &mut MessageCommon<Pb, A>)
    where
        VS: ValueSlot<()>,
        I: SlotInitMut,
        Pb: PresenceBits,
        A: Allocator + Clone,
    {
        common.set_bit(VALUE_BIT, false);
        ValueSlot::with_mut(slot, init, common).clear();
    }

    #[inline]
    fn encoded_len(value: bool, field: u32) -> usize {
        encode::encoded_len_varint_field(field, Self::encode_wire(value))
    }

    #[inline]
    fn encode<B: BufMut>(value: bool, field: u32, buf: &mut B) {
        encode::encode_varint_field(field, Self::encode_wire(value), buf);
    }

    #[inline]
    fn decode<B: Buf, A: Allocator>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: A,
    ) -> Result<bool, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        Self::decode_wire(raw)
    }
}
