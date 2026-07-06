//! Oneof group storage — the exception to per-field independence.
//!
//! Protobuf oneof variants are mutually exclusive: setting or decoding one
//! variant clears any other. [`OneofSlot`] centralises that coupling so
//! individual variant field types are not used on the message struct.
//!
//! Mutation flows through [`OneofSlotMut`], obtained via
//! [`BindableMut::bind_mut`](crate::fields::shared::BindableMut::bind_mut), mirroring the bound-view idiom used by the singular /
//! repeated field families: the slot is bound to the message
//! [`MessageCommon`] (for the allocator), and the previously-active variant is
//! released through [`OneofDeallocate::deallocate`] before the slot is overwritten.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;

use crate::fields::shared::{BindableMut, MessageCommon, PresenceBits};

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

    /// Projects the active storage enum onto one variant's field wrapper.
    ///
    /// Generated code passes the storage enum's per-variant accessor (for example
    /// `NotificationStorage::webhook_id`) so message getters can mirror ordinary
    /// fields: `self.notification.variant_of(Storage::webhook_id).optional(common)`.
    #[inline]
    pub fn variant_of<'a, F>(&'a self, select: fn(&E) -> Option<&F>) -> OneofVariantRef<'a, F> {
        OneofVariantRef::new(self.as_ref().and_then(select))
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

impl<E, A: Allocator + Clone, Pb: PresenceBits> BindableMut<MessageCommon<Pb, A>> for OneofSlot<E> {
    type BoundMut<'f, 'c> = OneofSlotMut<'f, 'c, E, Pb, A>
    where
        Self: 'f,
        MessageCommon<Pb, A>: 'c;

    fn bind_mut<'f, 'c>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> OneofSlotMut<'f, 'c, E, Pb, A> {
        OneofSlotMut::new(self, common)
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

    /// Ensures the active variant is the one `project` extracts; otherwise frees any
    /// existing variant and installs a fresh one built by `install` (which receives
    /// an owned allocator clone). Returns a mutable reference to the variant's
    /// inner payload, borrowing only the slot (`'f`), so `common` is free once this
    /// returns.
    ///
    /// `is_active` inspects the stored enum immutably; `project` extracts the inner
    /// field wrapper when that variant is active (`None` for other variants). Because `E` may own
    /// non-droppable storage, switching variants always goes through this method
    /// (or [`set`](Self::set) / [`clear`](Self::clear)) so the previous variant is
    /// released with the message allocator before the slot is overwritten.
    ///
    /// Backs the per-variant `_mut` accessors. Generated `merge_from` selects the
    /// variant through the storage enum's `bind_<variant>_mut` helper (which
    /// calls this method internally), then merges the wire occurrence through
    /// the returned bound field view — reusing each field's merge machinery
    /// instead of a bespoke `merge_<variant>` helper on the enum.
    pub fn variant_mut<V>(
        self,
        is_active: impl FnOnce(&E) -> bool,
        project: impl FnOnce(&mut E) -> Option<&mut V>,
        install: impl FnOnce(A) -> E,
    ) -> &'f mut V
    where
        E: OneofDeallocate<A>,
        A: Clone,
    {
        let slot = self.slot;
        let common = self.common;

        if !matches!(slot.as_ref(), Some(e) if is_active(e)) {
            if let Some(old) = slot.take() {
                // SAFETY: an owned clone of the message allocator owns the
                // previous variant's buffers.
                unsafe { old.deallocate(common.alloc.clone()) };
            }
            slot.set(install(common.alloc.clone()));
        }
        project(slot.as_mut().unwrap()).expect(
            "install closure must construct the variant that project extracts",
        )
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
/// `None` when the slot is unset or holds a different variant. Generated
/// `OneofSlot` impls expose one accessor per variant; each returns this type so
/// message getters can mirror ordinary fields
/// (`self.notification.webhook_id().optional(&self._common)`).
pub struct OneofVariantRef<'a, F> {
    field: Option<&'a F>,
}

impl<'a, F> OneofVariantRef<'a, F> {
    #[inline]
    pub fn new(field: Option<&'a F>) -> Self {
        Self { field }
    }
}

impl<
        'a,
        T: crate::fields::wire::len::LenProtoType,
        const FIELD: u32,
        A: Allocator,
        D,
    > OneofVariantRef<'a, crate::fields::singular::len::SingularLenField<T, crate::fields::shared::field_presence::Oneof, FIELD, A, D>>
where
    for<'b> T::Ref<'b>: Copy,
    D: for<'b> ::puroro::HasDefault<T::Ref<'b>>,
{
    pub fn optional<Pb: PresenceBits>(
        self,
        common: &MessageCommon<Pb, A>,
    ) -> ::puroro::Optional<T::Ref<'a>, D> {
        match self.field {
            Some(f) => f.optional(common),
            None => ::puroro::Optional::new(None),
        }
    }
}

impl<
        'a,
        T: crate::fields::wire::varint::VarintProtoType,
        const FIELD: u32,
        D,
    > OneofVariantRef<'a, crate::fields::singular::varint::SingularVarintField<T, crate::fields::shared::field_presence::Oneof, FIELD, D>>
where
    T::Value: Copy,
    D: ::puroro::HasDefault<T::Value>,
    <crate::fields::shared::field_presence::Oneof as crate::fields::shared::field_presence::FieldPresence>::ValueSlot<T::Value>:
        crate::fields::shared::value_slot::ValueSlot<T::Value>,
{
    pub fn optional<Pb: PresenceBits, A: Allocator>(
        self,
        common: &MessageCommon<Pb, A>,
    ) -> ::puroro::Optional<T::Value, D> {
        match self.field {
            Some(f) => f.optional(common),
            None => ::puroro::Optional::new(None),
        }
    }
}

impl<
        'a,
        M,
        const FIELD: u32,
        A: Allocator,
    > OneofVariantRef<'a, crate::fields::singular::message::NestedMessageField<M, crate::fields::shared::field_presence::Oneof, FIELD, A>>
{
    pub fn get(self) -> Option<&'a M> {
        self.field.map(crate::fields::singular::message::NestedMessageField::value)
    }
}
