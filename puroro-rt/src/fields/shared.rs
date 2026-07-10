//! Shared infrastructure for generated message fields.
//!
//! [`MessageCommon`] and [`PresenceBits`] are per-message state. [`ProtoZero`],
//! [`ValueSlot`](value_slot::ValueSlot), [`SlotInitView`](slot_init::SlotInitView) /
//! [`SlotInitMut`](slot_init::SlotInitMut), and [`FieldPresence`](field_presence::FieldPresence)
//! govern singular scalar storage and init state.

pub mod bindable;
pub mod field_presence;
pub mod slot_init;
pub mod value_slot;

pub use bindable::{Bindable, BindableMut};

use ::core::mem::ManuallyDrop;

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedVec;

// ---------------------------------------------------------------------------
// Presence bitfield (`PresenceBits`)
// ---------------------------------------------------------------------------

/// Read/write interface to a message's presence bitfield.
///
/// Each EXPLICIT / LEGACY_REQUIRED singular field has a stable `bit` index
/// assigned at codegen time. Field runtime types take `bit` as a `const` generic
/// or method parameter and use this trait to query/update presence.
pub trait PresenceBits {
    /// Returns whether the field at `bit` is explicitly present.
    fn is_set(&self, bit: usize) -> bool;

    /// Sets or clears the presence bit at `bit`.
    fn set(&mut self, bit: usize, present: bool);

    /// Clears the presence bit at `bit`.
    #[inline]
    fn clear(&mut self, bit: usize) {
        self.set(bit, false);
    }
}

// ---------------------------------------------------------------------------
// Per-message common state (`MessageCommon`)
// ---------------------------------------------------------------------------

/// Infrastructure fields shared by every field in a generated message.
///
/// Field types take `&Self` or `&mut Self` rather than a back-pointer to the
/// parent message struct.
///
/// The allocator `alloc` is the single canonical copy for the whole message:
/// unmanaged field payloads borrow it (`&alloc`) for every operation that
/// (de)allocates. `unknown_fields` is an allocator-less [`UnmanagedVec`] wrapped
/// in [`ManuallyDrop`], so it never frees itself implicitly; the owning message
/// releases it via [`deallocate`](Self::deallocate) in its `Drop`.
pub struct MessageCommon<P, A: Allocator> {
    pub presence: P,
    pub unknown_fields: ManuallyDrop<UnmanagedVec<u8>>,
    pub alloc: A,
}

impl<P, A: Allocator> MessageCommon<P, A> {
    /// Creates common state with the given presence bitfield and allocator.
    pub fn new_in(presence: P, alloc: A) -> Self {
        // `UnmanagedVec::new` does not allocate; it only decomposes an empty
        // `Vec`, so the borrow here never establishes buffer ownership (this is
        // the documented no-op use of `UnmanagedVec::new(&alloc)`). Once the
        // buffer actually grows it is owned by an owned-`A` clone, and it is
        // freed with an owned-`A` clone in `deallocate`.
        let unknown_fields = ManuallyDrop::new(UnmanagedVec::new(&alloc));
        Self {
            presence,
            unknown_fields,
            alloc,
        }
    }
}

impl<P, A: Allocator + Clone> MessageCommon<P, A> {
    /// Releases the unknown-field buffer. Must be called exactly once from the
    /// owning message's `Drop`; afterwards `self` must not be used.
    pub fn deallocate(&mut self) {
        // SAFETY: called once from the message `Drop`; `unknown_fields` is not
        // touched again, and an owned clone of `self.alloc` is interchangeable
        // with the clones that grew the buffer (`Allocator + Clone` contract).
        let uf = unsafe { ManuallyDrop::take(&mut self.unknown_fields) };
        unsafe { uf.deallocate(self.alloc.clone()) };
    }
}

impl<P: PresenceBits, A: Allocator> MessageCommon<P, A> {
    /// Returns whether a presence bit is set.
    #[inline]
    pub fn is_present(&self, bit: usize) -> bool {
        self.presence.is_set(bit)
    }

    /// Sets or clears a presence bit.
    #[inline]
    pub fn set_presence(&mut self, bit: usize, present: bool) {
        self.presence.set(bit, present);
    }
}

// ---------------------------------------------------------------------------
// Protobuf type-zero (`ProtoZero`)
// ---------------------------------------------------------------------------

/// Protobuf type-zero for a singular scalar storage type.
///
/// [`proto_zero`](Self::proto_zero) constructs the type-zero value (message ctor,
/// lazy-init placeholder). [`is_proto_zero`](Self::is_proto_zero) drives implicit
/// presence omit-on-wire checks. [`set_proto_zero`](Self::set_proto_zero) writes
/// the type-zero into an existing storage slot (clear).
pub trait ProtoZero: Copy {
    /// Returns the protobuf type-zero (`0`, `false`, …).
    fn proto_zero() -> Self;

    /// Returns `true` when `value` equals the protobuf type-zero.
    fn is_proto_zero(value: &Self) -> bool;

    /// Writes the protobuf type-zero into `value`.
    fn set_proto_zero(value: &mut Self) {
        *value = Self::proto_zero();
    }
}

impl ProtoZero for i32 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for i64 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for u32 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for u64 {
    fn proto_zero() -> Self {
        0
    }

    fn is_proto_zero(value: &Self) -> bool {
        *value == 0
    }
}

impl ProtoZero for bool {
    fn proto_zero() -> Self {
        false
    }

    fn is_proto_zero(value: &Self) -> bool {
        !*value
    }
}

// ---------------------------------------------------------------------------
// Allocator-aware construction / release (`DefaultIn` / `DeallocateIn`)
// ---------------------------------------------------------------------------

/// Allocator-aware construction of the protobuf type-zero / empty value.
///
/// Generalizes [`Default`] for payloads whose empty form needs an allocator
/// (`UnmanagedString`, `UnmanagedVec`). Allocator-less scalars ignore `alloc`.
/// This lets [`ValueSlot`](value_slot::ValueSlot) construct any stored value
/// uniformly, whether or not it is heap-backed.
pub trait DefaultIn {
    /// Builds the empty / type-zero value, using `alloc` when heap-backed.
    fn default_in<A: Allocator>(alloc: A) -> Self;
}

/// Allocator-aware release of a stored value.
///
/// Pairs with [`DefaultIn`] so [`ValueSlot`](value_slot::ValueSlot) can replace
/// or clear a payload without leaking. Allocator-less scalars are a no-op.
pub trait DeallocateIn {
    /// Drops the value and frees its backing allocation through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns this value's buffer.
    unsafe fn deallocate_in<A: Allocator>(self, alloc: A);
}

/// Empty / type-zero predicate for IMPLICIT omit-on-encode.
///
/// Scalars use [`ProtoZero::is_proto_zero`]; LEN payloads use `is_empty`.
pub trait ProtoEmpty {
    /// `true` when the value equals the protobuf empty / type-zero.
    fn is_proto_empty(&self) -> bool;
}

// Scalars (and generated enum newtypes) reach these traits through `ProtoZero`.
// `UnmanagedString` / `UnmanagedVec` are not `ProtoZero`, so they get dedicated
// impls below without overlapping the blankets.
impl<T: ProtoZero> DefaultIn for T {
    #[inline]
    fn default_in<A: Allocator>(_alloc: A) -> Self {
        T::proto_zero()
    }
}

impl<T: ProtoZero> DeallocateIn for T {
    #[inline]
    unsafe fn deallocate_in<A: Allocator>(self, _alloc: A) {}
}

impl<T: ProtoZero> ProtoEmpty for T {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        T::is_proto_zero(self)
    }
}

impl DefaultIn for ::unmanaged::UnmanagedString {
    #[inline]
    fn default_in<A: Allocator>(alloc: A) -> Self {
        ::unmanaged::UnmanagedString::new(alloc)
    }
}

impl DeallocateIn for ::unmanaged::UnmanagedString {
    #[inline]
    unsafe fn deallocate_in<A: Allocator>(self, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { self.deallocate(alloc) };
    }
}

impl ProtoEmpty for ::unmanaged::UnmanagedString {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.is_empty()
    }
}

impl DefaultIn for ::unmanaged::UnmanagedVec<u8> {
    #[inline]
    fn default_in<A: Allocator>(alloc: A) -> Self {
        ::unmanaged::UnmanagedVec::new(alloc)
    }
}

impl DeallocateIn for ::unmanaged::UnmanagedVec<u8> {
    #[inline]
    unsafe fn deallocate_in<A: Allocator>(self, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { self.deallocate(alloc) };
    }
}

impl ProtoEmpty for ::unmanaged::UnmanagedVec<u8> {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        self.is_empty()
    }
}
