//! Unified singular scalar field — varint and LEN share one wrapper.
//!
//! **Singular** here means **non-repeated**: both presence-tracked fields
//! (`EXPLICIT` / “optional”) and non-presence-tracked fields (`IMPLICIT`) use
//! this type. Cardinality (singular vs repeated) is separate from presence
//! ([`FieldPresence`](crate::fields::shared::field_presence::FieldPresence)).
//!
//! Parametrised by protobuf type marker `T: SingularType`, [`FieldPresence`],
//! proto field number `FIELD`, allocator `A`, value [`ValueLayout`] `L`
//! (`Inline`, or [`BitPacked`] for packed bool), and compile-time default marker `D`.
//! Physical storage is `P::ValueSlot<L::Slot>`. Heap payloads are wrapped in
//! [`ManuallyDrop`] so message / oneof `Drop` can release them through
//! [`deallocate`](SingularField::deallocate) without an implicit panic from
//! `UnmanagedString` / `UnmanagedVec` / [`SsoString`](crate::fields::wire::sso_string::SsoString) / [`SsoBytes`](crate::fields::wire::sso_bytes::SsoBytes).
//! Copy scalars / ZST bool slots use the same layout; their layout teardown is a no-op.

use ::core::fmt::{self, Debug, Formatter, Result as FmtResult};
use ::core::marker::PhantomData;
use ::core::mem::ManuallyDrop;
use ::core::ops::Deref;

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;

use crate::defaults::ProtoDefault;
use ::puroro::{DecodeBuf, DecodeError, HasDefault, Optional, WireType};

use crate::decode::WireSpan;
use crate::encode::field_number_const;
use crate::fields::shared::FieldDeallocate;
use crate::fields::shared::field_inspect::{FieldCloneIn, FieldDebug, FieldEncode, FieldPartialEq};
use crate::fields::shared::{
    DefaultIn, MessageBindingMut, MessageCommon, MessageCommonAlloc, MessageCommonSharedBits,
    field_presence::{
        Explicit, FieldPresence, Implicit, LegacyRequired, Message, Oneof, RequiredFieldPresence,
    },
    slot_init::{AlwaysInitialized, SlotInitView},
    value_layout::{Inline, ValueLayout, ValueLayoutClone, ValueLayoutMerge, ValueLayoutMut},
    value_slot::{
        AddressableSlot, ValueSlot, ValueSlotMutAccess, ValueSlotNew, ValueSlotRefAccess,
    },
};
use crate::fields::wire::encode_type::{encode_field, encoded_len_field};
use crate::fields::wire::len::{ProtoBytes, ProtoString};
use crate::fields::wire::proto_ref_ops::{ProtoRefDebug, ProtoRefEq};
use crate::fields::wire::singular_type::SingularType;
use crate::fields::wire::wire_or_sso::{WireOrSso, WireOrSsoKind, WireOrSsoStore};
use crate::message_encode::EncodeCtx;

/// Type-level projection of a [`SingularField`]'s `_mut` payload.
///
/// [`Mut`](Self::Mut) is the type of [`SingularFieldMut::value_mut`] /
/// oneof [`SingularField::value_mut`](SingularField::value_mut). Generated
/// oneof `OneofGroup::Mut` names this from the per-variant field alias so
/// layout / marker details stay on that alias.
pub trait SingularFieldAccess {
    type Mut<'a>: Deref
    where
        Self: 'a;
}

/// Singular (non-repeated) scalar field — varint or LEN, selected by type marker `T`.
///
/// Parameter order: `T`, `P`, `FIELD`, `A`, `L = Inline`, `D = ProtoDefault`.
pub struct SingularField<
    T: SingularType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator,
    L: ValueLayout<T, A> = Inline,
    D = ProtoDefault,
> where
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    value: ManuallyDrop<P::ValueSlot<L::Slot>>,
    _marker: PhantomData<(T, P, A, L, D)>,
}

impl<T, P, const FIELD: u32, A, L, D> SingularFieldAccess for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayoutMut<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    type Mut<'a>
        = L::Mut<'a>
    where
        Self: 'a;
}

impl<T, P, const FIELD: u32, A, L, D> Clone for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A> + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, P, const FIELD: u32, A, L, D> Copy for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A> + Copy,
{
}

impl<T, P, const FIELD: u32, A, L, D> fmt::Debug for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SingularField")
            .field("value", &*self.value)
            .finish()
    }
}

impl<T, P, const FIELD: u32, A, L, D> SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    /// Creates a field with an empty value slot.
    #[inline]
    pub fn new_in(alloc: A) -> Self
    where
        P::ValueSlot<L::Slot>: ValueSlotNew<L::Slot, A>,
    {
        Self {
            value: ManuallyDrop::new(ValueSlotNew::new_in(alloc)),
            _marker: PhantomData,
        }
    }

    /// Binds this field to `common` for read access.
    #[inline]
    pub fn bind<'a, Cx: MessageBindingMut<A>>(
        &'a self,
        common: &'a Cx,
    ) -> SingularFieldRef<'a, T, P, FIELD, A, L, D, Cx> {
        SingularFieldRef::new(self, common)
    }

    /// Binds this field to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Cx: MessageBindingMut<A>>(
        &'f mut self,
        common: &'c mut Cx,
    ) -> SingularFieldMut<'f, 'c, T, P, FIELD, A, L, D, Cx> {
        SingularFieldMut::new(self, common)
    }
}

/// Always-initialized presence policies do not read the bitfield.
macro_rules! impl_singular_deallocate_always {
    ($presence:ty) => {
        impl<T, const FIELD: u32, A, L, D, Cx> FieldDeallocate<Cx>
            for SingularField<T, $presence, FIELD, A, L, D>
        where
            T: SingularType,
            A: Allocator,
            L: ValueLayout<T, A>,
            Cx: MessageBindingMut<A>,
            L::Slot: AddressableSlot,
            <$presence as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
        {
            #[inline]
            fn deallocate(&mut self, common: &Cx) {
                let slot = unsafe { ManuallyDrop::take(&mut self.value) };
                L::deallocate_slot(slot, true, common);
            }
        }
    };
}

impl_singular_deallocate_always!(Implicit);
impl_singular_deallocate_always!(Oneof);
impl_singular_deallocate_always!(Message);

/// Bit-tracked presence policies consult the init bit via a probe.
macro_rules! impl_singular_deallocate_bit {
    ($presence:ty) => {
        impl<T, const BIT: usize, const FIELD: u32, A, L, D, Cx> FieldDeallocate<Cx>
            for SingularField<T, $presence, FIELD, A, L, D>
        where
            T: SingularType,
            A: Allocator,
            L: ValueLayout<T, A>,
            Cx: MessageBindingMut<A>,
            L::Slot: AddressableSlot,
            <$presence as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
        {
            #[inline]
            fn deallocate(&mut self, common: &Cx) {
                let init = <$presence as FieldPresence>::slot_init_view();
                let initialized = init.is_initialized(|b| common.is_bit_set(b));
                let slot = unsafe { ManuallyDrop::take(&mut self.value) };
                L::deallocate_slot(slot, initialized, common);
            }
        }
    };
}

impl_singular_deallocate_bit!(Explicit<BIT>);
impl_singular_deallocate_bit!(LegacyRequired<BIT>);

impl<T, const FIELD: u32, A, L, D> SingularField<T, Implicit, FIELD, A, L, D>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    /// Low-level borrow of the always-initialized slot's logical value.
    #[inline]
    pub fn value<'a, Cx: MessageBindingMut<A>>(&'a self, common: &'a Cx) -> T::View<'a, A> {
        let slot = (*self.value)
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot");
        L::get(slot, common)
    }
}

impl<T, const FIELD: u32, A, L, D> SingularField<T, Oneof, FIELD, A, L, D>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    /// Low-level borrow of the always-initialized oneof-variant slot's logical value.
    #[inline]
    pub fn value<'a, Cx: MessageBindingMut<A>>(&'a self, common: &'a Cx) -> T::View<'a, A> {
        let slot = (*self.value)
            .with(AlwaysInitialized, common)
            .get()
            .expect("always-initialized slot");
        L::get(slot, common)
    }

    /// Mutable accessor for a oneof variant (slot is always initialized).
    pub fn value_mut<'a, Cx: MessageBindingMut<A>>(&'a mut self, common: &'a mut Cx) -> L::Mut<'a>
    where
        A: Clone,
        L: ValueLayoutMut<T, A>,
        L::Slot: DefaultIn<A>,
    {
        L::with_mut(&mut *self.value, AlwaysInitialized, common)
    }
}

impl<T, const FIELD: u32, A, L, D> SingularField<T, Message, FIELD, A, L, D>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    <Message as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    /// Returns whether the pointer-present slot holds a value.
    #[inline]
    pub fn is_present(&self) -> bool {
        (*self.value).is_some()
    }

    /// proto2 / editions `required` message — pointer presence, no bit.
    pub fn validate_required<Pb>(&self, _common: &MessageCommon<Pb, A>) -> Result<(), DecodeError> {
        if self.is_present() {
            Ok(())
        } else {
            Err(DecodeError::MissingRequiredField {
                field_number: FIELD,
            })
        }
    }
}

impl<T, P, const FIELD: u32, A, L, D> DefaultIn<A> for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlotNew<L::Slot, A>,
{
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<T, const BIT: usize, const FIELD: u32, A, L, D>
    SingularField<T, LegacyRequired<BIT>, FIELD, A, L, D>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    pub fn validate_required<Cx: MessageBindingMut<A>>(
        &self,
        common: &Cx,
    ) -> Result<(), DecodeError> {
        LegacyRequired::<BIT>::validate_present(common, FIELD, || {
            let init = <LegacyRequired<BIT> as FieldPresence>::slot_init_view();
            match (*self.value).with(init, common).get() {
                Some(slot) => L::is_proto_empty(slot, common),
                None => true,
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a singular field to its message common state,
/// produced by [`SingularField::bind`].
pub struct SingularFieldRef<
    'a,
    T: SingularType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator,
    L: ValueLayout<T, A>,
    D,
    Cx,
> where
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    field: &'a SingularField<T, P, FIELD, A, L, D>,
    common: &'a Cx,
}

impl<'a, T, P, const FIELD: u32, A, L, D, Cx> SingularFieldRef<'a, T, P, FIELD, A, L, D, Cx>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    pub(crate) fn new(field: &'a SingularField<T, P, FIELD, A, L, D>, common: &'a Cx) -> Self {
        Self { field, common }
    }
}

impl<'a, T, P, const FIELD: u32, A, L, D, Cx> SingularFieldRef<'a, T, P, FIELD, A, L, D, Cx>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    /// Returns the logical value when the field is present.
    pub fn get(self) -> Option<T::View<'a, A>> {
        if P::is_set(self.common, || {
            match (*self.field.value)
                .with(P::slot_init_view(), self.common)
                .get()
            {
                Some(slot) => L::is_proto_empty(slot, self.common),
                None => true,
            }
        }) {
            let slot = (*self.field.value)
                .with(P::slot_init_view(), self.common)
                .get()
                .expect("is_set implies initialized slot");
            Some(L::get(slot, self.common))
        } else {
            None
        }
    }
}

impl<'a, T, P, const FIELD: u32, A, L, D, Cx> SingularFieldRef<'a, T, P, FIELD, A, L, D, Cx>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
    T::View<'a, A>: Copy,
    D: HasDefault<T::View<'a, A>>,
{
    pub fn optional(self) -> Optional<T::View<'a, A>, D> {
        Optional::new(self.get())
    }
}

impl<'a, T, const FIELD: u32, A, L, D, Cx> SingularFieldRef<'a, T, Implicit, FIELD, A, L, D, Cx>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    pub fn value(self) -> T::View<'a, A> {
        self.field.value(self.common)
    }
}

impl<'a, T, const FIELD: u32, A, L, D, Cx> SingularFieldRef<'a, T, Oneof, FIELD, A, L, D, Cx>
where
    T: SingularType,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    pub fn value(self) -> T::View<'a, A> {
        self.field.value(self.common)
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a singular field to its message common state,
/// produced by [`SingularField::bind_mut`].
pub struct SingularFieldMut<
    'f,
    'c,
    T: SingularType,
    P: FieldPresence,
    const FIELD: u32,
    A: Allocator,
    L: ValueLayout<T, A>,
    D,
    Cx,
> where
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    field: &'f mut SingularField<T, P, FIELD, A, L, D>,
    common: &'c mut Cx,
}

impl<'f, 'c, T, P, const FIELD: u32, A, L, D, Cx> SingularFieldMut<'f, 'c, T, P, FIELD, A, L, D, Cx>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    pub(crate) fn new(
        field: &'f mut SingularField<T, P, FIELD, A, L, D>,
        common: &'c mut Cx,
    ) -> Self {
        Self { field, common }
    }
}

impl<'f, 'c, T, P, const FIELD: u32, A, L, D, Cx> SingularFieldMut<'f, 'c, T, P, FIELD, A, L, D, Cx>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    /// Returns a mutable accessor, lazy-initializing the slot when needed.
    #[inline]
    pub fn value_mut(self) -> L::Mut<'f>
    where
        'c: 'f,
        A: Clone,
        L: ValueLayoutMut<T, A>,
        L::Slot: DefaultIn<A>,
    {
        L::with_mut(&mut *self.field.value, P::slot_init_mut(), self.common)
    }

    /// Alias of [`value_mut`](Self::value_mut) for nested-message call sites.
    #[inline]
    pub fn get_mut(self) -> L::Mut<'f>
    where
        'c: 'f,
        A: Clone,
        L: ValueLayoutMut<T, A>,
        L::Slot: DefaultIn<A>,
    {
        self.value_mut()
    }

    pub fn merge<B: DecodeBuf>(
        self,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
        L: ValueLayoutMerge<T, A>,
        L::Slot: DefaultIn<A>,
    {
        L::merge(
            &mut *self.field.value,
            P::slot_init_mut(),
            self.common,
            wire_type,
            buf,
            field_number_const::<FIELD>(),
            depth,
        )
    }

    /// Resets the value slot and clears explicit presence when applicable.
    pub fn clear(self)
    where
        A: Clone,
        L::Slot: DefaultIn<A>,
    {
        L::clear(&mut *self.field.value, P::slot_init_mut(), self.common);
    }
}

impl<T, P, const FIELD: u32, A, const KIND: usize, D>
    SingularField<T, P, FIELD, A, WireOrSso<KIND>, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator + Clone,
    WireOrSso<KIND>: ValueLayout<T, A>,
    <WireOrSso<KIND> as ValueLayout<T, A>>::Slot:
        AddressableSlot + DefaultIn<A> + WireOrSsoStore<A>,
    P::ValueSlot<<WireOrSso<KIND> as ValueLayout<T, A>>::Slot>:
        ValueSlot<<WireOrSso<KIND> as ValueLayout<T, A>>::Slot, A>,
{
    /// Record a LEN payload that already lives in the island buffer.
    pub fn store_len_span<Cx: MessageBindingMut<A>>(&mut self, span: WireSpan, common: &mut Cx) {
        let init = P::slot_init_mut();
        let old = WireOrSso::<KIND>::kind(common);
        let alloc = common.clone_alloc();
        {
            let slot = ValueSlot::with_mut(&mut *self.value, init, common).get_mut();
            slot.store_wire(span, old, &alloc);
        }
        WireOrSso::<KIND>::set_kind(common, WireOrSsoKind::Wire);
    }
}

impl<P, const FIELD: u32, A, const KIND: usize, D>
    SingularField<ProtoString, P, FIELD, A, WireOrSso<KIND>, D>
where
    P: FieldPresence,
    A: Allocator + Clone,
    <WireOrSso<KIND> as ValueLayout<ProtoString, A>>::Slot: AddressableSlot,
    P::ValueSlot<<WireOrSso<KIND> as ValueLayout<ProtoString, A>>::Slot>:
        ValueSlot<<WireOrSso<KIND> as ValueLayout<ProtoString, A>>::Slot, A>,
{
    /// Decode / promote a UTF-8 string. Failed is sticky.
    pub fn try_str<'a, Cx>(
        &'a self,
        common: &'a Cx,
        wire: &'a [u8],
    ) -> Result<Optional<&'a str, D>, DecodeError>
    where
        Cx: MessageBindingMut<A> + MessageCommonSharedBits,
        D: HasDefault<&'a str>,
    {
        if !P::is_set(common, || {
            match (*self.value).with(P::slot_init_view(), common).get() {
                Some(slot) => {
                    <WireOrSso<KIND> as ValueLayout<ProtoString, A>>::is_proto_empty(slot, common)
                }
                None => true,
            }
        }) {
            return Ok(Optional::new(None));
        }
        let slot = (*self.value)
            .with(P::slot_init_view(), common)
            .get()
            .expect("is_set implies initialized slot");
        let arm = WireOrSso::<KIND>::kind(common);
        let s = slot.get_str(arm, wire, common.clone_alloc(), |new| {
            WireOrSso::<KIND>::set_kind_shared(common, new);
        })?;
        Ok(Optional::new(Some(s)))
    }
}

impl<P, const FIELD: u32, A, const KIND: usize, D>
    SingularField<ProtoBytes, P, FIELD, A, WireOrSso<KIND>, D>
where
    P: FieldPresence,
    A: Allocator + Clone,
    <WireOrSso<KIND> as ValueLayout<ProtoBytes, A>>::Slot: AddressableSlot,
    P::ValueSlot<<WireOrSso<KIND> as ValueLayout<ProtoBytes, A>>::Slot>:
        ValueSlot<<WireOrSso<KIND> as ValueLayout<ProtoBytes, A>>::Slot, A>,
{
    /// Copy a bytes payload into Inline / Heap.
    pub fn try_bytes<'a, Cx>(
        &'a self,
        common: &'a Cx,
        wire: &'a [u8],
    ) -> Result<Optional<&'a [u8], D>, DecodeError>
    where
        Cx: MessageBindingMut<A> + MessageCommonSharedBits,
        D: HasDefault<&'a [u8]>,
    {
        if !P::is_set(common, || {
            match (*self.value).with(P::slot_init_view(), common).get() {
                Some(slot) => {
                    <WireOrSso<KIND> as ValueLayout<ProtoBytes, A>>::is_proto_empty(slot, common)
                }
                None => true,
            }
        }) {
            return Ok(Optional::new(None));
        }
        let slot = (*self.value)
            .with(P::slot_init_view(), common)
            .get()
            .expect("is_set implies initialized slot");
        let arm = WireOrSso::<KIND>::kind(common);
        let bytes = slot.get_bytes(arm, wire, common.clone_alloc(), |new| {
            WireOrSso::<KIND>::set_kind_shared(common, new);
        })?;
        Ok(Optional::new(Some(bytes)))
    }
}

// ---------------------------------------------------------------------------
// FieldPartialEq / FieldDebug / FieldEncode / FieldCloneIn (message field visitors)
// ---------------------------------------------------------------------------

impl<T, P, const FIELD: u32, A, L, D, Cx> FieldPartialEq<Cx> for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType + ProtoRefEq<A>,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn field_eq(&self, common: &Cx, other: &Self, other_common: &Cx) -> bool {
        // Option equality matches getter semantics (IMPLICIT zero ≡ unset).
        T::option_eq(self.bind(common).get(), other.bind(other_common).get())
    }
}

impl<T, P, const FIELD: u32, A, L, D, Cx> FieldEncode<Cx> for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    fn encoded_len(&self, common: &Cx, ctx: &mut EncodeCtx) -> usize {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            match (*self.value).with(init, common).get() {
                Some(slot) => L::is_proto_empty(slot, common),
                None => true,
            }
        }) {
            let init = P::slot_init_view();
            let slot = (*self.value)
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            encoded_len_field::<T, A>(L::get(slot, common), field_number_const::<FIELD>(), ctx)
        } else {
            0
        }
    }

    fn encode_raw<B: BufMut>(&self, common: &Cx, ctx: &mut EncodeCtx, buf: &mut B) {
        if P::should_emit(common, || {
            let init = P::slot_init_view();
            match (*self.value).with(init, common).get() {
                Some(slot) => L::is_proto_empty(slot, common),
                None => true,
            }
        }) {
            let init = P::slot_init_view();
            let slot = (*self.value)
                .with(init, common)
                .get()
                .expect("should_emit implies initialized slot");
            encode_field::<T, A, B>(
                L::get(slot, common),
                field_number_const::<FIELD>(),
                ctx,
                buf,
            );
        }
    }
}

impl<T, P, const FIELD: u32, A, L, D, Cx> FieldCloneIn<Cx> for SingularField<T, P, FIELD, A, L, D>
where
    T: SingularType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayoutClone<T, A>,
    Cx: MessageBindingMut<A> + MessageCommonAlloc<Alloc = A>,
    L::Slot: AddressableSlot,
    P::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    fn clone_field(&self, common: &Cx, alloc: A) -> Self {
        let init = P::slot_init_view();
        let initialized = init.is_initialized(|b| common.is_bit_set(b));
        Self {
            value: ManuallyDrop::new(L::clone_slot(&*self.value, initialized, common, alloc)),
            _marker: PhantomData,
        }
    }
}

impl<T, const FIELD: u32, A, L, D, Cx> FieldDebug<Cx> for SingularField<T, Implicit, FIELD, A, L, D>
where
    T: SingularType + ProtoRefDebug<A>,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <Implicit as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn fmt_debug(&self, common: &Cx, f: &mut Formatter<'_>) -> FmtResult {
        T::fmt_ref(&self.value(common), f)
    }
}

impl<T, const BIT: usize, const FIELD: u32, A, L, D, Cx> FieldDebug<Cx>
    for SingularField<T, Explicit<BIT>, FIELD, A, L, D>
where
    T: SingularType + ProtoRefDebug<A>,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <Explicit<BIT> as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn fmt_debug(&self, common: &Cx, f: &mut Formatter<'_>) -> FmtResult {
        T::fmt_option(self.bind(common).get(), f)
    }
}

impl<T, const BIT: usize, const FIELD: u32, A, L, D, Cx> FieldDebug<Cx>
    for SingularField<T, LegacyRequired<BIT>, FIELD, A, L, D>
where
    T: SingularType + ProtoRefDebug<A>,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <LegacyRequired<BIT> as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn fmt_debug(&self, common: &Cx, f: &mut Formatter<'_>) -> FmtResult {
        T::fmt_option(self.bind(common).get(), f)
    }
}

impl<T, const FIELD: u32, A, L, D, Cx> FieldDebug<Cx> for SingularField<T, Message, FIELD, A, L, D>
where
    T: SingularType + ProtoRefDebug<A>,
    A: Allocator,
    L: ValueLayout<T, A>,
    Cx: MessageBindingMut<A>,
    L::Slot: AddressableSlot,
    <Message as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    #[inline]
    fn fmt_debug(&self, common: &Cx, f: &mut Formatter<'_>) -> FmtResult {
        T::fmt_option(self.bind(common).get(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::shared::FieldDeallocate;
    use crate::fields::shared::MessageCommon;
    use crate::fields::shared::field_presence::{Explicit, Implicit};
    use crate::fields::shared::value_layout::{BitPacked, Inline, InlineOrHeap};
    use crate::fields::wire::len::{ProtoBytes, ProtoString, ProtoStringUnchecked};
    use crate::fields::wire::numerical::ProtoBool;
    use ::allocator_api2::alloc::Global;
    use ::bitvec::array::BitArray;
    use ::bitvec::order::Lsb0;
    use ::puroro::{DecodeError, ScopedBuf, WireType};

    type Bits1 = BitArray<[u8; 1], Lsb0>;
    type TestCommon = MessageCommon<Bits1, Global>;

    #[test]
    fn proto_bool_inline_implicit_set_and_omit() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field = SingularField::<ProtoBool, Implicit, 1, Global, Inline>::new_in(Global);
        assert_eq!(field.bind(&common).get(), None);
        *field.bind_mut(&mut common).value_mut() = true;
        assert_eq!(field.bind(&common).get(), Some(true));
        field.bind_mut(&mut common).clear();
        assert_eq!(field.bind(&common).get(), None);
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_bool_inline_explicit_can_store_false() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field = SingularField::<ProtoBool, Explicit<0>, 1, Global, Inline>::new_in(Global);
        assert_eq!(field.bind(&common).get(), None);
        *field.bind_mut(&mut common).value_mut() = false;
        assert_eq!(field.bind(&common).get(), Some(false));
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_bool_inline_merges_varint_true() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field = SingularField::<ProtoBool, Implicit, 1, Global, Inline>::new_in(Global);
        let mut data: &[u8] = &[1];
        let mut buf = ScopedBuf::new(&mut data);
        field
            .bind_mut(&mut common)
            .merge(WireType::Varint, &mut buf, 0)
            .expect("merge true");
        assert_eq!(field.bind(&common).get(), Some(true));
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_bool_bitpacked_still_uses_value_bit() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoBool, Implicit, 1, Global, BitPacked<0>>::new_in(Global);
        assert_eq!(field.bind(&common).get(), None);
        *field.bind_mut(&mut common).value_mut() = true;
        assert!(common.is_bit_set(0));
        assert_eq!(field.bind(&common).get(), Some(true));
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_bytes_sso_set_and_promote() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoBytes, Explicit<0>, 1, Global, InlineOrHeap<1>>::new_in(Global);
        assert_eq!(field.bind(&common).get(), None);
        field.bind_mut(&mut common).value_mut().set(b"hi");
        assert_eq!(field.bind(&common).get(), Some(&b"hi"[..]));
        assert!(!common.is_bit_set(1));

        let long = vec![b'x'; crate::INLINE_CAP + 1];
        field.bind_mut(&mut common).value_mut().set(&long);
        assert_eq!(field.bind(&common).get(), Some(long.as_slice()));
        assert!(common.is_bit_set(1));

        field.bind_mut(&mut common).clear();
        assert_eq!(field.bind(&common).get(), None);
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_string_sso_rejects_invalid_utf8() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoString, Explicit<0>, 1, Global, InlineOrHeap<1>>::new_in(Global);
        let mut data: &[u8] = &[1, 0xff];
        let mut buf = ScopedBuf::new(&mut data);
        let err = field
            .bind_mut(&mut common)
            .merge(WireType::Len, &mut buf, 0)
            .unwrap_err();
        assert_eq!(err, DecodeError::InvalidUtf8);
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_string_unchecked_sso_accepts_invalid_utf8() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoStringUnchecked, Explicit<0>, 1, Global, InlineOrHeap<1>>::new_in(
                Global,
            );
        let mut data: &[u8] = &[1, 0xff];
        let mut buf = ScopedBuf::new(&mut data);
        field
            .bind_mut(&mut common)
            .merge(WireType::Len, &mut buf, 0)
            .expect("NONE accepts invalid UTF-8");
        let got = field.bind(&common).get().expect("set");
        assert_eq!(got, &[0xff]);
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_string_unchecked_inline_accepts_invalid_utf8() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoStringUnchecked, Explicit<0>, 1, Global, Inline>::new_in(Global);
        let mut data: &[u8] = &[1, 0xff];
        let mut buf = ScopedBuf::new(&mut data);
        field
            .bind_mut(&mut common)
            .merge(WireType::Len, &mut buf, 0)
            .expect("NONE heap layout");
        let got = field.bind(&common).get().expect("set");
        assert_eq!(got, &[0xff]);
        field.deallocate(&common);
        common.deallocate();
    }

    #[test]
    fn proto_string_unchecked_sso_heap_arm_accepts_invalid_utf8() {
        let mut common = TestCommon::new_in(Bits1::ZERO, Global);
        let mut field =
            SingularField::<ProtoStringUnchecked, Explicit<0>, 1, Global, InlineOrHeap<1>>::new_in(
                Global,
            );
        let n = crate::INLINE_CAP + 1;
        let mut payload = vec![n as u8];
        payload.extend(vec![0xff; n]);
        let mut data: &[u8] = payload.as_slice();
        let mut buf = ScopedBuf::new(&mut data);
        field
            .bind_mut(&mut common)
            .merge(WireType::Len, &mut buf, 0)
            .expect("NONE heap-arm SSO");
        let got = field.bind(&common).get().expect("set");
        assert_eq!(got.len(), n);
        assert!(got.iter().all(|&b| b == 0xff));
        assert!(common.is_bit_set(1));
        field.deallocate(&common);
        common.deallocate();
    }
}
