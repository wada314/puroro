//! Oneof group storage — the exception to per-field independence.
//!
//! Protobuf oneof variants are mutually exclusive: setting or decoding one
//! variant clears any other. [`OneofSlot`] centralises that coupling so
//! individual variant field types are not used on the message struct.
//!
//! Mutation flows through [`OneofSlotMut`], obtained via
//! [`BindableMut::bind_mut`](crate::fields::shared::BindableMut::bind_mut); read
//! binding uses [`OneofSlotRef`] via [`Bindable::bind`](crate::fields::shared::Bindable::bind).
//! Both mirror the bound-view idiom used by the singular / repeated field
//! families: the slot is bound to the message [`MessageCommon`] (for the
//! allocator on the mut path), and the previously-active variant is released
//! through [`OneofDeallocate::deallocate`] before the slot is overwritten.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::puroro::{HasDefault, Optional};

use crate::fields::enum_variant::EnumVariant;
use crate::fields::shared::{
    Bindable, BindableMut, FieldDeallocate, MessageCommon, PresenceBits,
    field_presence::{FieldPresence, Oneof},
    value_slot::ValueSlot,
};
use crate::fields::singular::field::SingularField;
use crate::fields::singular::message::NestedMessageField;
use crate::fields::wire::scalar::ScalarProtoType;

/// Explicit release of a generated `oneof` storage enum.
///
/// The enum owns allocator-less field wrappers in its variants. [`OneofSlotMut`]
/// takes the active variant and calls this before overwriting the slot, passing
/// the same [`MessageCommon`] message `Drop` uses so each arm can call
/// [`FieldDeallocate::deallocate`](crate::FieldDeallocate::deallocate).
///
/// `Pb` and `A` are trait parameters because the storage enum pins the message's
/// presence newtype and allocator type.
pub trait OneofDeallocate<Pb: PresenceBits, A: Allocator> {
    /// Drops the active variant and frees its storage through `common`.
    ///
    /// # Safety
    ///
    /// `common.alloc` must be the allocator that owns the variant's buffers.
    unsafe fn deallocate(self, common: &MessageCommon<Pb, A>);
}

/// Wire encode behaviour for a generated oneof storage enum variant.
pub trait OneofEncodable<A: Allocator> {
    /// Wire byte length of this active variant.
    fn encoded_len<Pb: PresenceBits>(&self, common: &MessageCommon<Pb, A>) -> usize;

    /// Encodes this active variant.
    fn encode_raw<Pb: PresenceBits, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B);
}

/// Storage for a protobuf `oneof` group.
///
/// `E` is the generated `enum` for the group: one unit-or-tuple variant per
/// oneof member, each holding that member's **field wrapper** — the same type a
/// singular field of that kind would use (`SingularLenField<_, _, A>`,
/// `SingularVarintField<_, _>`, a nested-message field, …), minus presence
/// (which the slot itself tracks). At most one variant is active, so the slot is
/// just an `Option<E>`. For the allocator-owning cases, `E` is expected to
/// implement [`OneofDeallocate`] so a previously-active variant can be released
/// explicitly before it is overwritten; that bound is required by the mutating
/// view ([`OneofSlotMut`]) rather than by the slot itself, so plain read/encode
/// paths stay free of it.
///
/// The stored variant is replaced whenever another one is set or decoded (last
/// wins on the wire). All mutation goes through [`BindableMut::bind_mut`](crate::fields::shared::BindableMut::bind_mut); the
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

    /// Wire byte length when a variant is active.
    pub fn encoded_len<Pb, A>(&self, common: &MessageCommon<Pb, A>) -> usize
    where
        E: OneofEncodable<A>,
        Pb: PresenceBits,
        A: Allocator,
    {
        self.as_ref().map(|v| v.encoded_len(common)).unwrap_or(0)
    }

    /// Encodes the active variant.
    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        E: OneofEncodable<A>,
        Pb: PresenceBits,
        A: Allocator,
    {
        if let Some(v) = self.as_ref() {
            v.encode_raw(common, buf);
        }
    }
}

impl<E, Pb: PresenceBits, A: Allocator> FieldDeallocate<Pb, A> for OneofSlot<E>
where
    E: OneofDeallocate<Pb, A>,
{
    /// Releases the active variant through `common` (no-op when unset).
    #[inline]
    fn deallocate(&mut self, common: &MessageCommon<Pb, A>) {
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

impl<'a, E, A: Allocator + Clone, Pb: PresenceBits> Bindable<&'a MessageCommon<Pb, A>>
    for &'a OneofSlot<E>
{
    type Bound = OneofSlotRef<'a, E, Pb, A>;

    #[inline]
    fn bind(self, common: &'a MessageCommon<Pb, A>) -> Self::Bound {
        OneofSlotRef::new(self, common)
    }
}

impl<'f, 'c, E, A: Allocator + Clone, Pb: PresenceBits> BindableMut<&'c mut MessageCommon<Pb, A>>
    for &'f mut OneofSlot<E>
{
    type BoundMut = OneofSlotMut<'f, 'c, E, Pb, A>;

    #[inline]
    fn bind_mut(self, common: &'c mut MessageCommon<Pb, A>) -> Self::BoundMut {
        OneofSlotMut::new(self, common)
    }
}

// ---------------------------------------------------------------------------
// Read view
// ---------------------------------------------------------------------------

/// Short-lived shared binding of a oneof slot to its message common state,
/// produced by [`Bindable::bind`](crate::fields::shared::Bindable::bind).
///
/// Mirrors [`OneofSlotMut`] for the read path. Generated group views wrap this
/// (or hold the same pair of references) so `notification()`-style accessors
/// always return a handle, including when the group is unset. Per-variant
/// getters go through [`variant_of`](Self::variant_of).
pub struct OneofSlotRef<'a, E, Pb: PresenceBits, A: Allocator> {
    slot: &'a OneofSlot<E>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, E, Pb: PresenceBits, A: Allocator> OneofSlotRef<'a, E, Pb, A> {
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
    /// `V` is the zero-sized marker type for which the storage enum implements
    /// [`EnumVariant`]. Generated message getters use this so oneof members
    /// mirror ordinary fields:
    /// `self.notification.bind(&common).variant_of::<EmailAddress>().optional()`.
    #[inline]
    pub fn variant_of<V>(self) -> OneofVariantRef<'a, <E as EnumVariant<V>>::Value, Pb, A>
    where
        E: EnumVariant<V>,
    {
        OneofVariantRef::new(
            self.slot
                .as_ref()
                .and_then(|e| <E as EnumVariant<V>>::variant_ref(e)),
            self.common,
        )
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a oneof slot to its message common state,
/// [`OneofSlotMut`](crate::fields::oneof::OneofSlotMut), produced by
/// [`BindableMut::bind_mut`](crate::fields::shared::BindableMut::bind_mut).
///
/// Bundles the slot with the allocator context so that generated code can
/// mutate through a single call while the previously-active variant is released
/// consistently (via [`OneofDeallocate`]). A oneof has no presence bit, so the view
/// carries only `common` (for the allocator). Every method consumes the view, so
/// a fresh `bind_mut` precedes each mutation.
pub struct OneofSlotMut<'f, 'c, E, Pb: PresenceBits, A: Allocator> {
    slot: &'f mut OneofSlot<E>,
    common: &'c mut MessageCommon<Pb, A>,
}

impl<'f, 'c, E, Pb: PresenceBits, A: Allocator> OneofSlotMut<'f, 'c, E, Pb, A> {
    #[inline]
    fn new(slot: &'f mut OneofSlot<E>, common: &'c mut MessageCommon<Pb, A>) -> Self {
        Self { slot, common }
    }

    /// Replaces the whole oneof with `value`, freeing the previously-active
    /// variant first (last wins on the wire). Backs the decode arms.
    pub fn set(self, value: E)
    where
        E: OneofDeallocate<Pb, A>,
        A: Clone,
    {
        if let Some(old) = self.slot.take() {
            // SAFETY: `common.alloc` owns the previous variant's buffers.
            unsafe { old.deallocate(self.common) };
        }
        self.slot.set(value);
    }

    /// Ensures the active variant is `V`; otherwise frees any existing variant and
    /// installs a fresh one via [`EnumVariant::new_value`]. Returns the field's
    /// bound mutation view (so callers do not re-bind `common`).
    ///
    /// Because `E` may own non-droppable storage, switching variants always goes
    /// through this method (or [`set`](Self::set) / [`clear`](Self::clear)) so the
    /// previous variant is released with the message allocator before the slot is
    /// overwritten.
    ///
    /// Generated accessors and decode arms use
    /// `slot.bind_mut(common).variant_mut::<V>().value_mut()` /
    /// `.merge(…)`.
    pub fn variant_mut<V>(
        self,
    ) -> <&'f mut <E as EnumVariant<V>>::Value as BindableMut<&'c mut MessageCommon<Pb, A>>>::BoundMut
    where
        E: EnumVariant<V, Alloc = A> + OneofDeallocate<Pb, A>,
        &'f mut <E as EnumVariant<V>>::Value: BindableMut<&'c mut MessageCommon<Pb, A>>,
        A: Clone,
    {
        let slot = self.slot;
        let common = self.common;

        let needs_install = !matches!(
            slot.as_ref(),
            Some(e) if <E as EnumVariant<V>>::variant_ref(e).is_some()
        );

        if needs_install {
            if let Some(old) = slot.take() {
                // SAFETY: `common.alloc` owns the previous variant's buffers.
                unsafe { old.deallocate(common) };
            }
            slot.set(<E as EnumVariant<V>>::from_variant(
                <E as EnumVariant<V>>::new_value(common.alloc.clone()),
            ));
        }

        let field = <E as EnumVariant<V>>::variant_mut(slot.as_mut().unwrap())
            .expect("from_variant must construct the variant that V selects");
        field.bind_mut(common)
    }

    /// Frees the active variant (if any), leaving the slot empty.
    pub fn clear(self)
    where
        E: OneofDeallocate<Pb, A>,
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

impl<E: ::core::fmt::Debug> ::core::fmt::Debug for OneofSlot<E> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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

// ---------------------------------------------------------------------------
// Per-variant read handles
// ---------------------------------------------------------------------------

/// Borrowed read handle for one member of an active [`OneofSlot`].
///
/// `None` when the slot is unset or holds a different variant. Produced by
/// [`OneofSlotRef::variant_of`] so message getters can mirror ordinary fields
/// (`self.notification.bind(&common).variant_of::<WebhookId>().optional()`).
///
/// [`optional`](Self::optional) threads the field wrapper's default marker `D`:
/// when this handle is empty, the returned [`Optional`](::puroro::Optional) is
/// unset and `get()` yields `D::DEFAULT` (proto custom default or type zero)
/// without activating the variant — matching official const oneof getters.
pub struct OneofVariantRef<'a, F, Pb: PresenceBits, A: Allocator> {
    field: Option<&'a F>,
    common: &'a MessageCommon<Pb, A>,
}

impl<'a, F, Pb: PresenceBits, A: Allocator> OneofVariantRef<'a, F, Pb, A> {
    #[inline]
    pub fn new(field: Option<&'a F>, common: &'a MessageCommon<Pb, A>) -> Self {
        Self { field, common }
    }
}

impl<'a, T: ScalarProtoType, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    OneofVariantRef<'a, SingularField<T, Oneof, FIELD, D>, Pb, A>
where
    for<'b> T::Ref<'b>: Copy,
    D: for<'b> HasDefault<T::Ref<'b>>,
    <Oneof as FieldPresence>::ValueSlot<T>: ValueSlot<T>,
{
    pub fn optional(self) -> Optional<T::Ref<'a>, D>
    where
        A: Clone,
    {
        match self.field {
            Some(f) => f.bind(self.common).optional(),
            None => Optional::new(None),
        }
    }
}

impl<'a, const VALUE_BIT: usize, const FIELD: u32, D, Pb: PresenceBits, A: Allocator>
    OneofVariantRef<'a, crate::fields::singular::BoolField<Oneof, VALUE_BIT, FIELD, D>, Pb, A>
where
    D: HasDefault<bool>,
{
    pub fn optional(self) -> Optional<bool, D>
    where
        A: Clone,
    {
        match self.field {
            Some(f) => f.bind(self.common).optional(),
            None => Optional::new(None),
        }
    }
}

impl<'a, M, const FIELD: u32, A: Allocator, Pb: PresenceBits>
    OneofVariantRef<'a, NestedMessageField<M, Oneof, FIELD, A>, Pb, A>
{
    pub fn get(self) -> Option<&'a M>
    where
        A: Clone,
    {
        self.field.map(|f| f.bind(self.common).value())
    }
}

// ---------------------------------------------------------------------------
// Group identity + public bound views
// ---------------------------------------------------------------------------

/// Generated oneof storage identity: case / projected Ref / Mut, plus the
/// message presence and allocator types.
///
/// Implemented on the crate-internal storage alias (e.g. `NotificationStorage`).
/// [`OneofView`] / [`OneofViewMut`] are parametrised by this trait so message
/// accessors need not expose the storage type in their signatures (RPIT).
pub trait OneofGroup {
    /// Payload-less discriminant of the active variant.
    type Case: Copy;

    /// Projected shared view of the active variant.
    type Ref<'a>
    where
        Self: 'a;

    /// Projected mutable view of the active variant.
    type Mut<'a>
    where
        Self: 'a;

    /// Per-message presence bitfield type.
    type Presence: PresenceBits;

    /// Message allocator type.
    type Alloc: Allocator + Clone;

    /// Owned storage enum type held in [`OneofSlot`]. Usually `Self`.
    type Storage: OneofDeallocate<Self::Presence, Self::Alloc>;

    /// Discriminant for an active storage value.
    fn case(storage: &Self::Storage) -> Self::Case;

    /// Project an active storage value to the shared view.
    fn to_ref<'a>(
        storage: &'a Self::Storage,
        common: &'a MessageCommon<Self::Presence, Self::Alloc>,
    ) -> Self::Ref<'a>;

    /// Project an active storage value to the mutable view.
    fn to_mut<'a>(
        storage: &'a mut Self::Storage,
        common: &'a mut MessageCommon<Self::Presence, Self::Alloc>,
    ) -> Self::Mut<'a>;
}

/// Shared bound view of a oneof group (slot + [`MessageCommon`]).
///
/// Returned by generated `notification()`-style accessors even when unset.
pub struct OneofView<'a, G: OneofGroup> {
    slot: &'a OneofSlot<G::Storage>,
    common: &'a MessageCommon<G::Presence, G::Alloc>,
}

impl<'a, G: OneofGroup> OneofView<'a, G> {
    /// Creates a shared group view from a slot and message common state.
    #[inline]
    pub fn new(
        slot: &'a OneofSlot<G::Storage>,
        common: &'a MessageCommon<G::Presence, G::Alloc>,
    ) -> Self {
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

/// Mutable bound view of a oneof group (slot + [`MessageCommon`]).
///
/// Returned by generated `notification_mut()`-style accessors even when unset.
pub struct OneofViewMut<'a, G: OneofGroup> {
    slot: &'a mut OneofSlot<G::Storage>,
    common: &'a mut MessageCommon<G::Presence, G::Alloc>,
}

impl<'a, G: OneofGroup> OneofViewMut<'a, G> {
    /// Creates a mutable group view from a slot and message common state.
    #[inline]
    pub fn new(
        slot: &'a mut OneofSlot<G::Storage>,
        common: &'a mut MessageCommon<G::Presence, G::Alloc>,
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
