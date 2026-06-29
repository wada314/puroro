//! Presence bitfield access for explicit-presence singular fields.
//!
//! Generated messages implement this trait on their `BitArray<…>` presence type
//! via a thin `impl PresenceBits for TaskPresence { … }` that forwards to
//! const bit indices. Field types call `PresenceBits` methods only — they never
//! name the concrete `BitArray` type.

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
