//! Oneof group storage — the exception to per-field independence.
//!
//! Protobuf oneof variants are mutually exclusive: setting or decoding one
//! variant clears any other. [`OneofSlot`] centralises that coupling so
//! individual variant field types are not used on the message struct.
//!
//! Mutation flows through [`OneofSlotMut`], obtained via
//! [`OneofSlot::bind_mut`]; read binding uses [`OneofSlotRef`] via
//! [`OneofSlot::bind`]. Both mirror the bound-view idiom used by the singular /
//! repeated field families: the slot is bound to the message [`MessageCommon`]
//! (for the allocator on the mut path), and the previously-active variant is
//! released through [`OneofDeallocate::deallocate`] before the slot is
//! overwritten.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::core::fmt::{self, Debug, Formatter, Result as FmtResult};
use ::puroro::{
    HasDefault, OneofView as OneofViewTrait, OneofViewMut as OneofViewMutTrait, Optional,
};
use ::unmanaged::UnmanagedBox;

use crate::fields::oneof_variant::OneofVariant;
use crate::fields::shared::field_inspect::{FieldCloneIn, FieldDebug, FieldEncode, FieldPartialEq};
use crate::fields::shared::{
    DefaultIn, FieldDeallocate, MessageCommon, MessageCommonAlloc, MessageCommonBits, ValueLayout,
    field_presence::{FieldPresence, Oneof},
    value_slot::{AddressableSlot, ValueSlot},
};
use crate::fields::singular::field::SingularField;
use crate::fields::wire::proto_message::ProtoMessage;
use crate::fields::wire::singular_type::SingularType;
use crate::message_encode::{EncodeCtx, MessageEncode};
use crate::message_merge::MessageMerge;

/// Explicit release of a generated `oneof` storage enum.
///
/// The enum owns allocator-less field wrappers in its variants. [`OneofSlotMut`]
/// takes the active variant and calls this before overwriting the slot, passing
/// the same [`MessageCommon`] message `Drop` uses so each arm can call
/// [`FieldDeallocate::deallocate`](crate::FieldDeallocate::deallocate).
///
/// `C` is the message common context (typically [`MessageCommon`]). Impls that
/// free heap payloads bound [`MessageCommonAlloc`].
pub trait OneofDeallocate<C> {
    /// Drops the active variant and frees its storage through `common`.
    ///
    /// # Safety
    ///
    /// `common` must be the parent message's [`MessageCommon`] — the same
    /// instance message `Drop` uses. Its allocator must own the variant's
    /// buffers; a different message's common is unsound.
    unsafe fn deallocate(self, common: &C);
}

/// Wire encode behaviour for a generated oneof storage enum variant.
///
/// Methods take [`MessageCommon`] so arms can call [`FieldEncode`] on singular
/// wrappers. A [`MessageCommonBits`] bound is required because a variant may be
/// a bit-packed `bool` whose value lives in the message bitfield.
pub trait OneofEncodable<A: Allocator> {
    /// Wire byte length of this active variant.
    fn encoded_len<P>(&self, common: &MessageCommon<P, A>, ctx: &mut EncodeCtx) -> usize
    where
        MessageCommon<P, A>: MessageCommonBits + MessageCommonAlloc<Alloc = A>;

    /// Encodes this active variant.
    fn encode_raw<P, B: BufMut>(
        &self,
        common: &MessageCommon<P, A>,
        ctx: &mut EncodeCtx,
        buf: &mut B,
    ) where
        MessageCommon<P, A>: MessageCommonBits + MessageCommonAlloc<Alloc = A>;
}

/// Storage for a protobuf `oneof` group.
///
/// `E` is the generated `enum` for the group: one unit-or-tuple variant per
/// oneof member, each holding that member's **field wrapper** — the same type a
/// singular field of that kind would use (`SingularField<ProtoString, …>`,
/// `SingularField<ProtoInt32, …>`, `SingularField<ProtoMessage<…>, …>`, …), minus presence
/// (which the slot itself tracks). At most one variant is active, so the slot is
/// just an `Option<E>`. For the allocator-owning cases, `E` is expected to
/// implement [`OneofDeallocate`] so a previously-active variant can be released
/// explicitly before it is overwritten; that bound is required by the mutating
/// view ([`OneofSlotMut`]) rather than by the slot itself, so plain read/encode
/// paths stay free of it.
///
/// The stored variant is replaced whenever another one is set or decoded (last
/// wins on the wire). All mutation goes through [`OneofSlot::bind_mut`]; the
/// inherent [`set`](Self::set) / [`take`](Self::take) / [`clear`](Self::clear)
/// are low-level primitives used by the view and do **not** release the
/// previous variant on their own.
pub struct OneofSlot<E> {
    value: Option<E>,
}

impl<E> OneofSlot<E> {
    /// Creates an empty oneof slot, ignoring `alloc` (codegen uses `new_in` uniformly).
    #[inline]
    pub fn new_in<A: Allocator>(_alloc: A) -> Self {
        Self { value: None }
    }

    /// Builds a slot from an already-constructed active variant (or `None`).
    #[inline]
    pub fn from_option(value: Option<E>) -> Self {
        Self { value }
    }

    /// Returns the active variant, if any.
    #[inline]
    pub fn as_ref(&self) -> Option<&E> {
        self.value.as_ref()
    }

    /// Returns a mutable reference to the active variant, if any.
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut E> {
        self.value.as_mut()
    }

    /// Installs `value` as the active variant.
    ///
    /// Note: assigning over an existing variant drops it. When `E` owns
    /// allocator-less storage, callers must [`take`](Self::take) and release the
    /// previous variant explicitly before calling `set`. To leave the slot empty,
    /// use [`clear`](Self::clear).
    #[inline]
    pub fn set(&mut self, value: E) {
        self.value = Some(value);
    }

    /// Removes and returns the active variant, leaving the slot empty.
    #[inline]
    pub fn take(&mut self) -> Option<E> {
        self.value.take()
    }

    /// Clears whichever variant was active without freeing it.
    ///
    /// Prefer [`OneofSlotMut::clear`](OneofSlotMut::clear) / [`FieldDeallocate`]
    /// so the active variant is released through the message allocator.
    #[inline]
    pub fn clear(&mut self) {
        self.value = None;
    }

    /// Binds this slot to `common` for read access.
    #[inline]
    pub fn bind<'a, Pb, A: Allocator + Clone>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> OneofSlotRef<'a, E, Pb, A> {
        OneofSlotRef::new(self, common)
    }

    /// Binds this slot to `common` for mutation.
    #[inline]
    pub fn bind_mut<'f, 'c, Pb, A: Allocator + Clone>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> OneofSlotMut<'f, 'c, E, Pb, A> {
        OneofSlotMut::new(self, common)
    }
}

impl<E, P, A: Allocator> FieldDeallocate<MessageCommon<P, A>> for OneofSlot<E>
where
    E: OneofDeallocate<MessageCommon<P, A>>,
{
    /// Releases the active variant through `common` (no-op when unset).
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<P, A>) {
        if let Some(old) = self.take() {
            // SAFETY: `common.alloc` owns the active variant's buffers.
            unsafe { old.deallocate(common) };
        }
    }
}

impl<E> Default for OneofSlot<E> {
    fn default() -> Self {
        Self { value: None }
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a oneof slot to its message common state,
/// produced by [`OneofSlot::bind`].
///
/// Mirrors [`OneofSlotMut`] for the read path. Generated group views wrap this
/// (or hold the same pair of references) so `notification()`-style accessors
/// always return a handle, including when the group is unset. Per-variant
/// getters go through [`variant_of`](Self::variant_of).
pub struct OneofSlotRef<'a, E, Pb, A: Allocator> {
    slot: &'a OneofSlot<E>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, E, Pb, A: Allocator> OneofSlotRef<'a, E, Pb, A> {
    #[inline]
    fn new(slot: &'a OneofSlot<E>, common: &'a MessageCommon<Pb, A>) -> Self {
        Self { slot, common }
    }

    /// Returns the active storage variant, if any.
    #[inline]
    pub fn as_ref(&self) -> Option<&'a E> {
        self.slot.as_ref()
    }

    /// Projects the active storage enum onto one variant's field wrapper.
    ///
    /// `FIELD` is the protobuf field number for which the storage enum implements
    /// [`OneofVariant`]. Generated message getters use this so oneof members
    /// mirror ordinary fields:
    /// `self.notification.bind(&common).variant_of::<FIELD_EMAIL_ADDRESS>().optional()`.
    #[inline]
    pub fn variant_of<const FIELD: u32>(
        self,
    ) -> OneofVariantRef<'a, <E as OneofVariant<FIELD>>::Value, Pb, A>
    where
        E: OneofVariant<FIELD>,
    {
        OneofVariantRef::new(
            self.slot
                .as_ref()
                .and_then(|e| <E as OneofVariant<FIELD>>::variant_ref(e)),
            self.common,
        )
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a oneof slot to its message common state,
/// produced by [`OneofSlot::bind_mut`].
///
/// Bundles the slot with the allocator context so that generated code can
/// mutate through a single call while the previously-active variant is released
/// consistently (via [`OneofDeallocate`]). A oneof has no presence bit, so the view
/// carries only `common` (for the allocator). Every method consumes the view, so
/// a fresh `bind_mut` precedes each mutation.
pub struct OneofSlotMut<'f, 'c, E, Pb, A: Allocator> {
    slot: &'f mut OneofSlot<E>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, E, Pb, A: Allocator> OneofSlotMut<'f, 'c, E, Pb, A> {
    #[inline]
    fn new(slot: &'f mut OneofSlot<E>, common: &'c mut MessageCommon<Pb, A>) -> Self {
        Self { slot, common }
    }

    /// Replaces the whole oneof with `value`, freeing the previously-active
    /// variant first (last wins on the wire). Backs the decode arms.
    pub fn set(self, value: E)
    where
        E: OneofDeallocate<MessageCommon<Pb, A>>,
        A: Clone,
    {
        if let Some(old) = self.slot.take() {
            // SAFETY: `common.alloc` owns the previous variant's buffers.
            unsafe { old.deallocate(self.common) };
        }
        self.slot.set(value);
    }

    /// Ensures the active variant is `FIELD`; otherwise frees any existing
    /// variant and installs a fresh one via [`DefaultIn`] on
    /// [`OneofVariant::Value`]. Returns a mutable reference to the variant's
    /// field wrapper.
    ///
    /// Because `E` may own non-droppable storage, switching variants always goes
    /// through this method (or [`set`](Self::set) / [`clear`](Self::clear)) so the
    /// previous variant is released with the message allocator before the slot is
    /// overwritten.
    ///
    /// Consumes this view so `common` can be re-borrowed; generated accessors and
    /// decode arms then bind the field themselves:
    /// `slot.bind_mut(common).variant_mut::<FIELD_EMAIL_ADDRESS>().bind_mut(common).value_mut()` /
    /// `.merge(…)`.
    pub fn variant_mut<const FIELD: u32>(self) -> &'f mut <E as OneofVariant<FIELD>>::Value
    where
        E: OneofVariant<FIELD> + OneofDeallocate<MessageCommon<Pb, A>>,
        <E as OneofVariant<FIELD>>::Value: DefaultIn<A>,
        A: Clone,
    {
        let slot = self.slot;
        let common = self.common;

        let needs_install = !matches!(
            slot.as_ref(),
            Some(e) if <E as OneofVariant<FIELD>>::variant_ref(e).is_some()
        );

        if needs_install {
            if let Some(old) = slot.take() {
                // SAFETY: `common.alloc` owns the previous variant's buffers.
                unsafe { old.deallocate(common) };
            }
            slot.set(<E as OneofVariant<FIELD>>::from_variant(
                DefaultIn::default_in(common.alloc.clone()),
            ));
        }

        <E as OneofVariant<FIELD>>::variant_mut(slot.as_mut().unwrap())
            .expect("from_variant must construct the variant that FIELD selects")
    }

    /// Frees the active variant (if any), leaving the slot empty.
    pub fn clear(self)
    where
        E: OneofDeallocate<MessageCommon<Pb, A>>,
        A: Clone,
    {
        if let Some(old) = self.slot.take() {
            // SAFETY: `common.alloc` owns the active variant's buffers.
            unsafe { old.deallocate(self.common) };
        }
    }
}

impl<E: Clone> Clone for OneofSlot<E> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<E: Debug> fmt::Debug for OneofSlot<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("OneofSlot")
            .field("value", &self.value)
            .finish()
    }
}

impl<E: PartialEq> PartialEq for OneofSlot<E> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<E, Pb, A> FieldPartialEq<MessageCommon<Pb, A>> for OneofSlot<E>
where
    E: OneofGroup<Bits = Pb, Alloc = A>,
    A: Allocator + Clone,
{
    #[inline]
    fn field_eq(
        &self,
        common: &MessageCommon<Pb, A>,
        other: &Self,
        other_common: &MessageCommon<Pb, A>,
    ) -> bool {
        match (self.as_ref(), other.as_ref()) {
            (None, None) => true,
            (Some(a), Some(b)) => E::to_ref(a, common) == E::to_ref(b, other_common),
            _ => false,
        }
    }
}

impl<E, Pb, A> FieldDebug<MessageCommon<Pb, A>> for OneofSlot<E>
where
    E: OneofGroup<Bits = Pb, Alloc = A>,
    A: Allocator + Clone,
    E::Case: Debug,
{
    #[inline]
    fn fmt_debug(&self, _common: &MessageCommon<Pb, A>, f: &mut Formatter<'_>) -> FmtResult {
        // Discriminant only — matches `notification().case()` in Debug.
        Debug::fmt(&self.as_ref().map(E::case), f)
    }
}

impl<E, Pb, A> FieldEncode<MessageCommon<Pb, A>> for OneofSlot<E>
where
    E: OneofEncodable<A> + OneofGroup<Bits = Pb, Alloc = A>,
    MessageCommon<Pb, A>: MessageCommonBits + MessageCommonAlloc<Alloc = A>,
    A: Allocator + Clone,
{
    fn encoded_len(&self, common: &MessageCommon<Pb, A>, ctx: &mut EncodeCtx) -> usize {
        self.as_ref()
            .map(|v| v.encoded_len(common, ctx))
            .unwrap_or(0)
    }

    fn encode_raw<B: BufMut>(
        &self,
        common: &MessageCommon<Pb, A>,
        ctx: &mut EncodeCtx,
        buf: &mut B,
    ) {
        if let Some(v) = self.as_ref() {
            v.encode_raw(common, ctx, buf);
        }
    }
}

impl<E, Pb, A> FieldCloneIn<MessageCommon<Pb, A>> for OneofSlot<E>
where
    E: OneofGroup<Bits = Pb, Alloc = A>,
    A: Allocator + Clone,
{
    fn clone_field(&self, common: &MessageCommon<Pb, A>, alloc: A) -> Self {
        Self::from_option(self.as_ref().map(|s| E::clone_storage_in(s, common, alloc)))
    }
}

// ---------------------------------------------------------------------------
// Per-variant read handles
// ---------------------------------------------------------------------------

/// Borrowed read handle for one member of an active [`OneofSlot`].
///
/// `None` when the slot is unset or holds a different variant. Produced by
/// [`OneofSlotRef::variant_of`] so message getters can mirror ordinary fields
/// (`self.notification.bind(&common).variant_of::<FIELD_WEBHOOK_ID>().optional()`).
///
/// [`optional`](Self::optional) threads the field wrapper's default marker `D`:
/// when this handle is empty, the returned [`Optional`](::puroro::Optional) is
/// unset and `get()` yields `D::DEFAULT` (proto custom default or type zero)
/// without activating the variant — matching official const oneof getters.
pub struct OneofVariantRef<'a, F, Pb, A: Allocator> {
    field: Option<&'a F>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, F, Pb, A: Allocator> OneofVariantRef<'a, F, Pb, A> {
    #[inline]
    pub fn new(field: Option<&'a F>, common: &'a MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }
}

impl<'a, T: SingularType, const FIELD: u32, A: Allocator + Clone, L: ValueLayout<T, A>, D, Pb>
    OneofVariantRef<'a, SingularField<T, Oneof, FIELD, A, L, D>, Pb, A>
where
    T::View<'a, A>: Copy,
    D: HasDefault<T::View<'a, A>>,
    MessageCommon<Pb, A>: MessageCommonBits,
    L::Slot: AddressableSlot + DefaultIn<A>,
    <Oneof as FieldPresence>::ValueSlot<L::Slot>: ValueSlot<L::Slot, A>,
{
    pub fn optional(self) -> Optional<T::View<'a, A>, D>
    where
        A: Clone,
    {
        match self.field {
            Some(f) => f.bind(self.common).optional(),
            None => Optional::new(None),
        }
    }
}

impl<'a, M, const FIELD: u32, A: Allocator + Clone, Pb>
    OneofVariantRef<'a, SingularField<ProtoMessage<M>, Oneof, FIELD, A>, Pb, A>
where
    M: ::puroro::Message<Alloc = A> + MessageEncode + MessageMerge + ::unmanaged::DeallocateIn<A>,
    MessageCommon<Pb, A>: MessageCommonBits,
    <Oneof as FieldPresence>::ValueSlot<UnmanagedBox<M, A>>: ValueSlot<UnmanagedBox<M, A>, A>,
{
    /// Returns the child when this message variant is active.
    pub fn get(self) -> Option<&'a M> {
        self.field.map(|f| f.bind(self.common).value())
    }
}

// ---------------------------------------------------------------------------
// Group identity + public bound views
// ---------------------------------------------------------------------------

/// Generated oneof storage: case / projected Ref / Mut, plus the message
/// common-bits and allocator types.
///
/// Implemented on the crate-internal storage alias (e.g. `NotificationStorage`),
/// which is also the type held in [`OneofSlot`]. [`OneofView`] /
/// [`OneofViewMut`] are parametrised by this trait so message accessors need
/// not expose the storage type in their signatures (RPIT).
pub trait OneofGroup
where
    Self: OneofDeallocate<MessageCommon<Self::Bits, Self::Alloc>>,
{
    /// Payload-less discriminant of the active variant.
    type Case: Copy;

    /// Projected shared view of the active variant.
    ///
    /// [`PartialEq`] is required so message equality can compare projected
    /// views without a generated `ref_eq` hook. The bound is gated on
    /// `Self: 'a` (not a blanket `for<'a>`), so reference allocators such as
    /// `A = &Bump` remain usable.
    type Ref<'a>: PartialEq
    where
        Self: 'a;

    /// Projected mutable view of the active variant.
    type Mut<'a>
    where
        Self: 'a;

    /// Per-message common-bits type (may be unused by the group).
    type Bits;

    /// Message allocator type.
    type Alloc: Allocator + Clone;

    /// Discriminant for an active storage value.
    fn case(storage: &Self) -> Self::Case;

    /// Project an active storage value to the shared view.
    fn to_ref<'a>(
        storage: &'a Self,
        common: &'a MessageCommon<Self::Bits, Self::Alloc>,
    ) -> Self::Ref<'a>;

    /// Project an active storage value to the mutable view.
    fn to_mut<'a>(
        storage: &'a mut Self,
        common: &'a mut MessageCommon<Self::Bits, Self::Alloc>,
    ) -> Self::Mut<'a>;

    /// Deep-copies an active storage value into `alloc`.
    fn clone_storage_in(
        storage: &Self,
        common: &MessageCommon<Self::Bits, Self::Alloc>,
        alloc: Self::Alloc,
    ) -> Self;
}

/// Shared bound view of a oneof group (slot + [`MessageCommon`]).
///
/// Returned by generated `notification()`-style accessors even when unset.
pub struct OneofView<'a, G: OneofGroup>
where
    G: OneofDeallocate<MessageCommon<G::Bits, G::Alloc>>,
{
    slot: &'a OneofSlot<G>,
    common: &'a MessageCommon<G::Bits, G::Alloc>,
}

impl<'a, G: OneofGroup> OneofView<'a, G>
where
    G: OneofDeallocate<MessageCommon<G::Bits, G::Alloc>>,
{
    /// Creates a shared group view from a slot and message common state.
    #[inline]
    pub fn new(slot: &'a OneofSlot<G>, common: &'a MessageCommon<G::Bits, G::Alloc>) -> Self {
        Self { slot, common }
    }

    /// Which variant is set (`None` when the group is unset).
    #[inline]
    pub fn case(&self) -> Option<G::Case> {
        self.slot.as_ref().map(G::case)
    }

    /// Projected read view of the active variant, if any.
    #[inline]
    pub fn as_ref(&self) -> Option<G::Ref<'a>> {
        self.slot.as_ref().map(|s| G::to_ref(s, self.common))
    }
}

impl<'a, G: OneofGroup> OneofViewTrait for OneofView<'a, G>
where
    G: OneofDeallocate<MessageCommon<G::Bits, G::Alloc>>,
{
    type Case = G::Case;
    type Ref = G::Ref<'a>;

    #[inline]
    fn case(&self) -> Option<Self::Case> {
        OneofView::case(self)
    }

    #[inline]
    fn as_ref(&self) -> Option<Self::Ref> {
        OneofView::as_ref(self)
    }
}

/// Mutable bound view of a oneof group (slot + [`MessageCommon`]).
///
/// Returned by generated `notification_mut()`-style accessors even when unset.
pub struct OneofViewMut<'a, G: OneofGroup>
where
    G: OneofDeallocate<MessageCommon<G::Bits, G::Alloc>>,
{
    slot: &'a mut OneofSlot<G>,
    common: &'a mut MessageCommon<G::Bits, G::Alloc>,
}

impl<'a, G: OneofGroup> OneofViewMut<'a, G>
where
    G: OneofDeallocate<MessageCommon<G::Bits, G::Alloc>>,
{
    /// Creates a mutable group view from a slot and message common state.
    #[inline]
    pub fn new(
        slot: &'a mut OneofSlot<G>,
        common: &'a mut MessageCommon<G::Bits, G::Alloc>,
    ) -> Self {
        Self { slot, common }
    }

    /// Reborrow as a shared bound view (for `case` / `as_ref` while mutating).
    #[inline]
    pub fn as_view<'b>(&'b self) -> OneofView<'b, G> {
        OneofView {
            slot: &*self.slot,
            common: &*self.common,
        }
    }

    /// Which variant is set (`None` when the group is unset).
    #[inline]
    pub fn case(&self) -> Option<G::Case> {
        self.as_view().case()
    }

    /// Projected read view of the active variant, if any (reborrows `self`).
    #[inline]
    pub fn as_ref(&self) -> Option<G::Ref<'_>> {
        self.as_view().as_ref()
    }

    /// Projected mutable view of the *currently active* variant (no switch).
    ///
    /// Consumes this bound view. Returns `None` when the group is unset.
    #[inline]
    pub fn as_mut(self) -> Option<G::Mut<'a>> {
        self.slot.as_mut().map(|s| G::to_mut(s, self.common))
    }

    /// Clears whichever variant is active (freeing it through the message allocator).
    #[inline]
    pub fn clear(self) {
        self.slot.bind_mut(self.common).clear();
    }
}

impl<'a, G: OneofGroup> OneofViewMutTrait for OneofViewMut<'a, G>
where
    G: OneofDeallocate<MessageCommon<G::Bits, G::Alloc>>,
{
    type Case = G::Case;

    type Ref<'b>
        = G::Ref<'b>
    where
        Self: 'b;

    type Mut = G::Mut<'a>;

    type Shared<'b>
        = OneofView<'b, G>
    where
        Self: 'b;

    #[inline]
    fn as_view(&self) -> Self::Shared<'_> {
        OneofViewMut::as_view(self)
    }

    #[inline]
    fn case(&self) -> Option<Self::Case> {
        OneofViewMut::case(self)
    }

    #[inline]
    fn as_mut(self) -> Option<Self::Mut> {
        OneofViewMut::as_mut(self)
    }

    #[inline]
    fn clear(self) {
        OneofViewMut::clear(self);
    }
}
