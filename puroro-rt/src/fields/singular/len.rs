//! Singular LEN field wrapper — generic over payload type and presence policy.
//!
//! Storage uses the same [`ValueSlot`](crate::fields::shared::value_slot::ValueSlot)
//! model as varint fields (`T` for Implicit/Oneof, `MaybeUninit<T>` for Explicit /
//! LegacyRequired). Heap payloads are wrapped in [`ManuallyDrop`] so message /
//! oneof `Drop` can release them through [`deallocate`](SingularLenField::deallocate)
//! without an implicit panic from `UnmanagedString` / `UnmanagedVec`.

use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use crate::defaults::ProtoDefault;
use crate::encode;
use ::puroro::DecodeError;
use ::puroro::WireType;
use ::puroro::{HasDefault, Optional};

use crate::fields::shared::{
    BindableMut, MessageCommon, PresenceBits,
    field_presence::{FieldPresence, LegacyRequired, Oneof, RequiredFieldPresence},
    slot_init::AlwaysInitialized,
    value_slot::ValueSlot,
};
use crate::fields::wire::len::{self, LenProtoType};

/// Singular LEN field — parametrised by [`LenProtoType`] `T`, presence policy `P`,
/// proto field number `FIELD`, message allocator `A`, and compile-time default marker `D`.
pub struct SingularLenField<
    T: LenProtoType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator,
    D = ProtoDefault,
> where
    P::ValueSlot<T::Storage>: ValueSlot<T::Storage>,
{
    value: ManuallyDrop<P::ValueSlot<T::Storage>>,
    _marker: PhantomData<(P, A, D)>,
}

impl<T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, P, FIELD, A, D>
where
    P::ValueSlot<T::Storage>: ValueSlot<T::Storage>,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: ManuallyDrop::new(ValueSlot::new_in(alloc)),
            _marker: PhantomData,
        }
    }

    pub fn encoded_len<Pb>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || P::payload_is_empty(&self.value)) {
            let init = P::slot_init_view(common);
            let storage = self
                .value
                .as_ref(&init)
                .expect("should_emit implies initialized slot");
            encode::encoded_len_len_field(FIELD, T::as_bytes(storage).len())
        } else {
            0
        }
    }

    pub fn encode_raw<Pb, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        Pb: PresenceBits,
    {
        if P::should_emit(common, || P::payload_is_empty(&self.value)) {
            let init = P::slot_init_view(common);
            let storage = self
                .value
                .as_ref(&init)
                .expect("should_emit implies initialized slot");
            encode::encode_len_field(FIELD, T::as_bytes(storage), buf);
        }
    }

    /// Releases the payload through `common`'s allocator. Must be called from the
    /// owning message's `Drop` (or equivalent) before the field itself is dropped.
    pub fn deallocate<Pb>(&mut self, common: &MessageCommon<Pb, A>)
    where
        Pb: PresenceBits,
        A: Clone,
    {
        let init = P::slot_init_view(common);
        let alloc = common.alloc.clone();
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(&init, alloc);
    }

    #[inline]
    pub fn has<Pb>(&self, common: &MessageCommon<Pb, A>) -> bool
    where
        Pb: PresenceBits,
    {
        P::is_set(common, || P::payload_is_empty(&self.value))
    }
}

impl<T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, P, FIELD, A, D>
where
    P::ValueSlot<T::Storage>: ValueSlot<T::Storage>,
    for<'a> T::Ref<'a>: Copy,
    D: for<'a> HasDefault<T::Ref<'a>>,
{
    pub fn optional<'a, Pb>(&'a self, common: &MessageCommon<Pb, A>) -> Optional<T::Ref<'a>, D>
    where
        Pb: PresenceBits,
    {
        let init = P::slot_init_view(common);
        let v = if P::is_set(common, || P::payload_is_empty(&self.value)) {
            Some(T::borrow(
                self.value
                    .as_ref(&init)
                    .expect("is_set implies initialized slot"),
            ))
        } else {
            None
        };
        Optional::new(v)
    }
}

impl<T: LenProtoType, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, crate::fields::shared::field_presence::Implicit, FIELD, A, D>
{
    #[inline]
    pub fn value(&self) -> T::Ref<'_> {
        T::borrow(
            self.value
                .as_ref(&AlwaysInitialized)
                .expect("always-initialized slot"),
        )
    }
}

impl<T: LenProtoType, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, Oneof, FIELD, A, D>
{
    #[inline]
    pub fn value(&self) -> T::Ref<'_> {
        T::borrow(
            self.value
                .as_ref(&AlwaysInitialized)
                .expect("always-initialized slot"),
        )
    }

    /// Growable handle for a oneof LEN variant (slot is always initialized).
    pub fn value_mut(&mut self, alloc: A) -> T::Mut<'_, A> {
        T::with_alloc(self.value.get_mut(), alloc)
    }

    /// Releases the always-present payload. Used from [`OneofDeallocate`].
    pub fn deallocate_in(&mut self, alloc: A) {
        let slot = unsafe { ManuallyDrop::take(&mut self.value) };
        slot.deallocate_in(&AlwaysInitialized, alloc);
    }
}

impl<T: LenProtoType, const BIT: usize, const FIELD: u32, A: Allocator, D>
    SingularLenField<T, LegacyRequired<BIT>, FIELD, A, D>
where
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<T::Storage>: ValueSlot<T::Storage>,
{
    pub fn validate_required<Pb>(&self, common: &MessageCommon<Pb, A>) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
    {
        LegacyRequired::<BIT>::validate_present(common, FIELD, || {
            LegacyRequired::<BIT>::payload_is_empty(&self.value)
        })
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

pub struct SingularLenFieldMut<
    'f,
    'c,
    T: LenProtoType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator,
    D,
    Pb: PresenceBits,
> where
    P::ValueSlot<T::Storage>: ValueSlot<T::Storage>,
{
    field: &'f mut SingularLenField<T, P, FIELD, A, D>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator, D, Pb: PresenceBits>
    SingularLenFieldMut<'f, 'c, T, P, FIELD, A, D, Pb>
where
    P::ValueSlot<T::Storage>: ValueSlot<T::Storage>,
{
    #[inline]
    fn new(
        field: &'f mut SingularLenField<T, P, FIELD, A, D>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> Self {
        Self { field, common }
    }

    /// Returns a growable guard. Ensures the slot is initialized, then releases
    /// the `common` borrow so the guard only ties up the field (`'f`).
    #[inline]
    pub fn value_mut(self) -> T::Mut<'f, A>
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        {
            let mut init = P::slot_init_mut(self.common);
            self.field.value.ensure_init(&mut init, alloc.clone());
        }
        T::with_alloc(self.field.value.get_mut(), alloc)
    }

    pub fn merge<B: Buf>(self, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != len::WIRE_TYPE {
            return Err(DecodeError::InvalidTag);
        }
        let new = T::decode(buf, self.common.alloc.clone())?;
        let alloc = self.common.alloc.clone();
        let mut init = P::slot_init_mut(self.common);
        self.field.value.set(&mut init, alloc, new);
        Ok(())
    }

    /// Resets payload to empty / type-zero and clears explicit presence when applicable.
    ///
    /// For [`Implicit`](crate::fields::shared::field_presence::Implicit) fields this omits the field on
    /// the wire.
    pub fn clear(self)
    where
        A: Clone,
    {
        let alloc = self.common.alloc.clone();
        let mut init = P::slot_init_mut(self.common);
        self.field.value.clear(&mut init, alloc);
    }
}

impl<T: LenProtoType, P: FieldPresence, const FIELD: u32, A: Allocator + Clone, D, Pb: PresenceBits>
    BindableMut<MessageCommon<Pb, A>> for SingularLenField<T, P, FIELD, A, D>
where
    P::ValueSlot<T::Storage>: ValueSlot<T::Storage>,
{
    type BoundMut<'f, 'c> = SingularLenFieldMut<'f, 'c, T, P, FIELD, A, D, Pb>
    where
        Self: 'f,
        MessageCommon<Pb, A>: 'c;

    fn bind_mut<'f, 'c>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularLenFieldMut<'f, 'c, T, P, FIELD, A, D, Pb> {
        SingularLenFieldMut::new(self, common)
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

pub type SingularLen<T, P, const FIELD: u32, A, D = ProtoDefault> =
    SingularLenField<T, P, FIELD, A, D>;

pub type ImplicitLenField<T, const FIELD: u32, A> =
    SingularLenField<T, crate::fields::shared::field_presence::Implicit, FIELD, A>;
pub type OneofLenField<T, const FIELD: u32, A> =
    SingularLenField<T, crate::fields::shared::field_presence::Oneof, FIELD, A>;
pub type ExplicitLenField<T, const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    SingularLenField<T, crate::fields::shared::field_presence::Explicit<BIT>, FIELD, A, D>;
pub type LegacyRequiredLenField<T, const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    SingularLenField<T, crate::fields::shared::field_presence::LegacyRequired<BIT>, FIELD, A, D>;

pub type ImplicitString<const FIELD: u32, A> = ImplicitLenField<len::ProtoString, FIELD, A>;
pub type ExplicitString<const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    ExplicitLenField<len::ProtoString, BIT, FIELD, A, D>;
pub type ImplicitBytes<const FIELD: u32, A> = ImplicitLenField<len::ProtoBytes, FIELD, A>;
pub type ExplicitBytes<const BIT: usize, const FIELD: u32, A, D = ProtoDefault> =
    ExplicitLenField<len::ProtoBytes, BIT, FIELD, A, D>;
