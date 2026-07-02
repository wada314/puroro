//! Oneof group storage — the exception to per-field independence.
//!
//! Protobuf oneof variants are mutually exclusive: setting or decoding one
//! variant clears any other. [`OneofSlot`] centralises that coupling so
//! individual variant field types are not used on the message struct.

/// Storage for a protobuf `oneof` group.
///
/// Decode arms for each variant field number call [`set`](Self::set) with the
/// decoded enum value; each call replaces the entire slot (last wins on wire).
/// Per-variant convenience setters on the parent message delegate here.
pub struct OneofSlot<E> {
    value: Option<E>,
}

impl<E> OneofSlot<E> {
    /// Creates an empty oneof slot.
    pub fn new() -> Self {
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
}

impl<E> Default for OneofSlot<E> {
    fn default() -> Self {
        Self::new()
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
