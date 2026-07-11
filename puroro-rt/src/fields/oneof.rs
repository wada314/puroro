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
    Bindable, BindableMut, MessageCommon, PresenceBits,
    field_presence::{FieldPresence, Oneof},
    value_slot::ValueSlot,
};
use crate::fields::singular::field::SingularField;
use crate::fields::singular::message::NestedMessageField;
use crate::fields::wire::scalar::ScalarProtoType;

/// Explicit, allocator-driven release of a generated `oneof` enum over allocator `A`.
///
/// A oneof enum owns allocator-less storage in its variants (field wrappers such
/// as `SingularLenField<_, _, A>`, nested messages, …), which cannot free
/// themselves. The enum implements this trait so [`OneofSlotMut`] can release the
/// active variant — handing it the message allocator — before overwriting the
/// slot. This mirrors the `deallocate(self, alloc)` contract of the `unmanaged`
/// types: the name stresses that dropping is *not* implicit; the caller must pass
/// the owning allocator. Variants that hold only inline scalars make `deallocate`
/// a no-op.
///
/// `A` is a trait parameter (rather than a generic method parameter) because a
/// storage enum that owns field wrappers pins their allocator type to its own
/// `A`; the freed allocator must match that type exactly.
pub trait OneofDeallocate<A: Allocator> {
    /// Drops the active variant and frees its storage through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns the variant's buffers.
    unsafe fn deallocate(self, alloc: A);
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

    /// Clears whichever variant was active.
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
        E: OneofDeallocate<A>,
        A: Clone,
    {
        if let Some(old) = self.slot.take() {
            // SAFETY: an owned clone of the message allocator owns the previous
            // variant's buffers.
            unsafe { old.deallocate(self.common.alloc.clone()) };
        }
        self.slot.set(value);
    }

    /// Ensures the active variant is `V`; otherwise frees any existing variant and
    /// installs a fresh one built by `make` (which receives an owned allocator
    /// clone). Returns a mutable reference to the variant's inner payload,
    /// borrowing only the slot (`'f`), so `common` is free once this returns.
    ///
    /// Because `E` may own non-droppable storage, switching variants always goes
    /// through this method (or [`set`](Self::set) / [`clear`](Self::clear)) so the
    /// previous variant is released with the message allocator before the slot is
    /// overwritten.
    ///
    /// Backs the per-variant `bind_<variant>_mut` helpers on generated storage
    /// enums. `merge_from` selects the variant through those helpers, then merges
    /// the wire occurrence through the returned bound field view.
    pub fn variant_mut<V>(
        self,
        make: impl FnOnce(A) -> <E as EnumVariant<V>>::Value,
    ) -> &'f mut <E as EnumVariant<V>>::Value
    where
        E: EnumVariant<V> + OneofDeallocate<A>,
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
                // SAFETY: an owned clone of the message allocator owns the
                // previous variant's buffers.
                unsafe { old.deallocate(common.alloc.clone()) };
            }
            slot.set(<E as EnumVariant<V>>::from_variant(make(
                common.alloc.clone(),
            )));
        }

        <E as EnumVariant<V>>::variant_mut(slot.as_mut().unwrap())
            .expect("from_variant must construct the variant that V selects")
    }

    /// Frees the active variant (if any), leaving the slot empty.
    pub fn clear(self)
    where
        E: OneofDeallocate<A>,
        A: Clone,
    {
        if let Some(old) = self.slot.take() {
            // SAFETY: an owned clone of the message allocator owns the active
            // variant's buffers.
            unsafe { old.deallocate(self.common.alloc.clone()) };
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
