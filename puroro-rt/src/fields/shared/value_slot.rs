//! Value slot storage for singular fields — always-initialized (`T`) or
//! presence-tracked ([`MaybeUninit<T>`]), selected by [`FieldPresence::ValueSlot`].
//!
//! [`ValueSlot`] is implemented for raw `T` and [`MaybeUninit<T>`]. The trait's
//! type parameter `T` disambiguates the two blanket impls so they do not overlap.
//!
//! Construction, replacement, and clearing thread an allocator (via
//! [`DefaultIn`](super::DefaultIn) / [`DeallocateIn`](super::DeallocateIn)) so a
//! heap-backed payload (`UnmanagedString`, `UnmanagedVec`) can be built and
//! released through the message allocator; allocator-less scalars ignore it.
//!
//! Mutation takes a [`SlotInitMut`](super::slot_init::SlotInitMut) handle so init-state
//! branching is centralized here rather than in [`FieldPresence`](super::field_presence::FieldPresence).

use ::core::mem::MaybeUninit;
use ::core::ops::DerefMut;

use ::allocator_api2::alloc::Allocator;

use super::{
    slot_init::{SlotInitMut, SlotInitView},
    DefaultIn, DeallocateIn,
};

/// Storage operations for a singular field value slot.
pub trait ValueSlot<T: DefaultIn + DeallocateIn> {
    /// Creates value storage when the parent message is constructed.
    ///
    /// For raw `T`, returns [`DefaultIn::default_in`] — the slot is always
    /// initialized. For [`MaybeUninit`], returns [`MaybeUninit::uninit`] and
    /// ignores `alloc` — the slot starts absent.
    fn new_in<A: Allocator>(alloc: A) -> Self;

    /// Assigns `value`, releasing any previously stored payload through `alloc`
    /// and updating init state when the slot was uninitialized.
    ///
    /// Unlike [`as_mut`](Self::as_mut), the slot does **not** need to be initialized
    /// first — for [`MaybeUninit`] this uses [`MaybeUninit::write`] on first assign.
    fn set<A: Allocator>(&mut self, init: &mut impl SlotInitMut, alloc: A, value: T);

    /// Drops an initialized payload (through `alloc`) and clears init state; the
    /// always-initialized variant reinstalls an empty value, hence `A: Clone`.
    fn clear<A: Allocator + Clone>(&mut self, init: &mut impl SlotInitMut, alloc: A);

    /// Returns a mutable guard to the value, lazy-initializing with
    /// [`DefaultIn::default_in`] (using `alloc`) when the slot is uninitialized.
    ///
    /// `init` is taken **by value** so its `common` borrow can be threaded into
    /// the returned guard: the opaque return therefore binds *both* the field
    /// borrow and `init`'s `common` borrow. That keeps room for a slot whose
    /// value lives inside `common` (e.g. a bool packed into the presence
    /// bitvec) to hand back a guard that borrows `common` rather than the field.
    ///
    /// The returned guard dereferences to `T`; for the current slots it is just
    /// `&mut T`, but the opaque return also keeps room for heap payloads to hand
    /// back a write-back guard instead.
    fn as_mut<A: Allocator>(
        &mut self,
        init: impl SlotInitMut,
        alloc: A,
    ) -> impl DerefMut<Target = T>;

    /// Ensures the slot holds a value, writing [`DefaultIn::default_in`] (using
    /// `alloc`) and marking init state when it was previously uninitialized.
    ///
    /// Call this before [`get_mut`](Self::get_mut) when the caller needs a bare
    /// `&mut T` that borrows only the field (e.g. LEN growable guards that wrap
    /// storage with an allocator and must release `common` first).
    fn ensure_init<A: Allocator>(&mut self, init: &mut impl SlotInitMut, alloc: A);

    /// Returns `&mut T`.
    ///
    /// # Precondition
    ///
    /// The slot must be initialized — call [`ensure_init`](Self::ensure_init)
    /// first (always-initialized slots satisfy this trivially).
    fn get_mut(&mut self) -> &mut T;

    /// Consumes the slot and frees any live payload through `alloc`.
    ///
    /// Used from message / oneof `Drop` paths. For [`MaybeUninit`] the init view
    /// decides whether a payload exists; always-initialized slots always free.
    fn deallocate_in<A: Allocator>(self, init: &impl SlotInitView, alloc: A);

    /// Borrows `&T` when initialized.
    ///
    /// Like [`Option::as_ref`](Option::as_ref) — `None` when the slot is not initialized.
    fn as_ref<'a>(&'a self, init: &impl SlotInitView) -> Option<&'a T>;
}

impl<T: DefaultIn + DeallocateIn> ValueSlot<T> for T {
    fn new_in<A: Allocator>(alloc: A) -> Self {
        T::default_in(alloc)
    }

    fn set<A: Allocator>(&mut self, _: &mut impl SlotInitMut, alloc: A, value: T) {
        let old = ::core::mem::replace(self, value);
        // SAFETY: `alloc` is the message allocator that owns `old`'s buffer.
        unsafe { old.deallocate_in(alloc) };
    }

    fn clear<A: Allocator + Clone>(&mut self, _: &mut impl SlotInitMut, alloc: A) {
        let old = ::core::mem::replace(self, T::default_in(alloc.clone()));
        // SAFETY: `alloc` owns `old`'s buffer.
        unsafe { old.deallocate_in(alloc) };
    }

    fn as_mut<A: Allocator>(
        &mut self,
        _init: impl SlotInitMut,
        _alloc: A,
    ) -> impl DerefMut<Target = T> {
        self
    }

    fn ensure_init<A: Allocator>(&mut self, _: &mut impl SlotInitMut, _alloc: A) {}

    fn get_mut(&mut self) -> &mut T {
        self
    }

    fn deallocate_in<A: Allocator>(self, _: &impl SlotInitView, alloc: A) {
        // SAFETY: `alloc` owns this value's buffer.
        unsafe { DeallocateIn::deallocate_in(self, alloc) };
    }

    fn as_ref<'a>(&'a self, _: &impl SlotInitView) -> Option<&'a T> {
        Some(self)
    }
}

impl<T: DefaultIn + DeallocateIn> ValueSlot<T> for MaybeUninit<T> {
    fn new_in<A: Allocator>(_alloc: A) -> Self {
        MaybeUninit::uninit()
    }

    fn set<A: Allocator>(&mut self, init: &mut impl SlotInitMut, alloc: A, value: T) {
        if init.is_initialized() {
            // SAFETY: init bit set implies a live payload.
            let old = ::core::mem::replace(unsafe { self.assume_init_mut() }, value);
            // SAFETY: `alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(alloc) };
        } else {
            self.write(value);
            init.set_initialized(true);
        }
    }

    fn clear<A: Allocator + Clone>(&mut self, init: &mut impl SlotInitMut, alloc: A) {
        if init.is_initialized() {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            let old = unsafe { self.assume_init_read() };
            // SAFETY: `alloc` owns `old`'s buffer.
            unsafe { old.deallocate_in(alloc) };
            init.set_initialized(false);
        }
    }

    fn as_mut<A: Allocator>(
        &mut self,
        mut init: impl SlotInitMut,
        alloc: A,
    ) -> impl DerefMut<Target = T> {
        if !init.is_initialized() {
            self.write(T::default_in(alloc));
            init.set_initialized(true);
        }
        // SAFETY: just ensured the slot is initialized.
        unsafe { self.assume_init_mut() }
    }

    fn ensure_init<A: Allocator>(&mut self, init: &mut impl SlotInitMut, alloc: A) {
        if !init.is_initialized() {
            self.write(T::default_in(alloc));
            init.set_initialized(true);
        }
    }

    fn get_mut(&mut self) -> &mut T {
        // SAFETY: the `ensure_init` / `as_mut` precondition guarantees a live payload.
        unsafe { self.assume_init_mut() }
    }

    fn deallocate_in<A: Allocator>(self, init: &impl SlotInitView, alloc: A) {
        if init.is_initialized() {
            // SAFETY: init bit set implies a live payload we now take ownership of.
            let value = unsafe { self.assume_init() };
            // SAFETY: `alloc` owns `value`'s buffer.
            unsafe { DeallocateIn::deallocate_in(value, alloc) };
        }
    }

    fn as_ref<'a>(&'a self, init: &impl SlotInitView) -> Option<&'a T> {
        if init.is_initialized() {
            Some(unsafe { self.assume_init_ref() })
        } else {
            None
        }
    }
}
