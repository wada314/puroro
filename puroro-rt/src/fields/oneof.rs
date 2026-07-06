//! Oneof group storage — the exception to per-field independence.
//!
//! Protobuf oneof variants are mutually exclusive: setting or decoding one
//! variant clears any other. [`OneofSlot`] centralises that coupling so
//! individual variant field types are not used on the message struct.
//!
//! Mutation flows through [`OneofSlotMut`], obtained via
//! [`OneofSlot::bind`], mirroring the bound-view idiom used by the singular /
//! repeated field families: the slot is bound to the message
//! [`MessageCommon`] (for the allocator), and the previously-active variant is
//! released through [`OneofDeallocate::deallocate`] before the slot is overwritten.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::fields::shared::{MessageCommon, PresenceBits};

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

/// Wire decode behaviour for a generated oneof storage enum.
///
/// Implemented by the crate-internal enum held in [`OneofSlot`]. Lets the parent
/// message use the same `encoded_len` / `encode_raw` / merge shape as other
/// fields without storage-specific static helpers.
pub trait OneofGroup<A: Allocator + Clone>: OneofDeallocate<A> + OneofEncodable<A> {
    /// Merges a wire occurrence into the matching variant, selecting it first.
    fn merge_wire<Pb, B>(
        slot: &mut OneofSlot<Self>,
        common: &mut MessageCommon<Pb, A>,
        field_number: u32,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        B: Buf,
        Self: Sized;
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
/// wins on the wire). All mutation goes through [`bind`](Self::bind); the
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
    pub fn get(&self) -> Option<&E> {
        self.value.as_ref()
    }

    /// Returns a mutable reference to the active variant, if any.
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut E> {
        self.value.as_mut()
    }

    /// Binds this slot to its message `common` state (for the allocator),
    /// producing a short-lived [`OneofSlotMut`] view.
    ///
    /// This is the entry point for every mutation (`set` / `variant_mut` /
    /// `clear`): generated accessors call `slot.bind(&mut common).…()` instead
    /// of releasing the old variant and rewriting the slot by hand. A oneof
    /// carries no presence bit, so the view needs only `common`.
    #[inline]
    pub fn bind<'f, 'c, Pb: PresenceBits, A: Allocator>(
        &'f mut self,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> OneofSlotMut<'f, 'c, E, Pb, A> {
        OneofSlotMut::new(self, common)
    }

    /// Replaces the whole oneof (clears any previous variant).
    ///
    /// Note: assigning over an existing variant drops it. When `E` owns
    /// allocator-less storage, callers must [`take`](Self::take) and release the
    /// previous variant explicitly before calling `set`.
    #[inline]
    pub fn set(&mut self, value: Option<E>) {
        self.value = value;
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
        self.get().map(|v| v.encoded_len(common)).unwrap_or(0)
    }

    /// Encodes the active variant.
    pub fn encode_raw<Pb, A, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B)
    where
        E: OneofEncodable<A>,
        Pb: PresenceBits,
        A: Allocator,
    {
        if let Some(v) = self.get() {
            v.encode_raw(common, buf);
        }
    }
}

impl<E> OneofSlot<E> {
    /// Merges a wire occurrence for one member of this oneof group.
    pub fn merge_wire<Pb, A, B>(
        &mut self,
        common: &mut MessageCommon<Pb, A>,
        field_number: u32,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        E: OneofGroup<A>,
        A: Allocator + Clone,
        Pb: PresenceBits,
        B: Buf,
    {
        E::merge_wire(self, common, field_number, wire_type, buf)
    }
}

impl<E> Default for OneofSlot<E> {
    fn default() -> Self {
        Self { value: None }
    }
}

// ---------------------------------------------------------------------------
// Mutation view
// ---------------------------------------------------------------------------

/// Short-lived binding of a oneof slot to its message common state, produced by
/// [`OneofSlot::bind`].
///
/// Bundles the slot with the allocator context so that generated code can
/// mutate through a single call while the previously-active variant is released
/// consistently (via [`OneofDeallocate`]). A oneof has no presence bit, so the view
/// carries only `common` (for the allocator). Every method consumes the view, so
/// a fresh `bind` precedes each mutation.
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
        self.slot.set(Some(value));
    }

    /// Ensures the active variant satisfies `is_match`; otherwise frees any
    /// existing variant and installs a fresh one built by `make` (which receives
    /// an owned allocator clone). Returns a mutable reference to the now-active
    /// variant, borrowing only the slot (`'f`), so `common` is free once this
    /// returns.
    ///
    /// Backs both the per-variant `_mut` accessors and the decode arms: after
    /// selecting the variant, generated `merge_from` merges the wire occurrence
    /// into the returned field through that field's own bind idiom
    /// (`field.bind(common).merge(…)` for every variant kind) — so the oneof
    /// reuses each field's merge machinery instead of a bespoke `merge_<variant>`
    /// helper on the enum.
    pub fn variant_mut(
        self,
        is_match: impl FnOnce(&E) -> bool,
        make: impl FnOnce(A) -> E,
    ) -> &'f mut E
    where
        E: OneofDeallocate<A>,
        A: Clone,
    {
        let slot = self.slot;
        let common = self.common;
        let active = matches!(slot.get(), Some(e) if is_match(e));
        if !active {
            if let Some(old) = slot.take() {
                // SAFETY: an owned clone of the message allocator owns the
                // previous variant's buffers.
                unsafe { old.deallocate(common.alloc.clone()) };
            }
            slot.set(Some(make(common.alloc.clone())));
        }
        // The branch above guarantees the slot is now occupied.
        slot.get_mut().unwrap()
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
        f.debug_struct("OneofSlot").field("value", &self.value).finish()
    }
}

impl<E: PartialEq> PartialEq for OneofSlot<E> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
