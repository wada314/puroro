//! Allocator-aware UTF-8 owned string type.

use crate::traits::{CloneIn, DefaultIn, ToOwnedIn};
use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec;
use core::borrow::Borrow;
use core::fmt;
use core::ops::{Deref, DerefMut};
use core::str;

/// A UTF-8 string that stores data inside an allocator-aware `Vec<u8, A>`.
pub struct OwnedString<A: Allocator> {
    bytes: Vec<u8, A>,
}

impl<A: Allocator> OwnedString<A> {
    /// Creates an empty `OwnedString` using the provided allocator.
    #[inline]
    pub fn new_in(alloc: A) -> Self {
        Self {
            bytes: Vec::new_in(alloc),
        }
    }

    /// Creates an `OwnedString` with the specified capacity in bytes.
    #[inline]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self {
            bytes: Vec::with_capacity_in(capacity, alloc),
        }
    }

    /// Converts the given UTF-8 byte buffer into an `OwnedString`.
    pub fn from_utf8_in(bytes: Vec<u8, A>) -> Result<Self, FromUtf8Error<A>> {
        match str::from_utf8(&bytes) {
            Ok(_) => Ok(Self { bytes }),
            Err(error) => Err(FromUtf8Error::from_std(error, bytes)),
        }
    }

    /// Copies the given `&str` into a newly allocated `OwnedString`.
    pub fn from_str_in(source: &str, alloc: A) -> Self {
        let mut bytes = Vec::with_capacity_in(source.len(), alloc);
        bytes.extend_from_slice(source.as_bytes());
        Self { bytes }
    }

    /// Returns the underlying allocator-aware byte buffer.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8, A> {
        self.bytes
    }

    /// Borrows the string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        // Safety: `OwnedString` maintains the UTF-8 invariant for `bytes`.
        unsafe { str::from_utf8_unchecked(&self.bytes) }
    }

    /// Borrows the underlying UTF-8 bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Mutable access to the string slice.
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        // Safety: `OwnedString` maintains the UTF-8 invariant for `bytes`.
        unsafe { str::from_utf8_unchecked_mut(&mut self.bytes) }
    }

    /// Returns the number of bytes used by the string.
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns `true` if the string has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Ensures that the string has at least `additional` bytes of capacity.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.bytes.reserve(additional);
    }

    /// Returns the current capacity in bytes.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    /// Shrinks the capacity of the string as much as possible.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.bytes.shrink_to_fit();
    }

    /// Clears the contents of the string, preserving capacity.
    #[inline]
    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    /// Appends a string slice to the end of this `OwnedString`.
    #[inline]
    pub fn push_str(&mut self, suffix: &str) {
        self.bytes.extend_from_slice(suffix.as_bytes());
    }

    /// Appends a single UTF-8 scalar value to the end of this `OwnedString`.
    pub fn push(&mut self, ch: char) {
        let mut buffer = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buffer);
        self.bytes.extend_from_slice(encoded.as_bytes());
    }
}

impl OwnedString<Global> {
    /// Creates an empty `OwnedString` using the global allocator.
    #[inline]
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    /// Creates a global `OwnedString` with the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, Global)
    }
}

impl<A: Allocator> Deref for OwnedString<A> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<A: Allocator> DerefMut for OwnedString<A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}

impl<A: Allocator> Borrow<str> for OwnedString<A> {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<A: Allocator> AsRef<str> for OwnedString<A> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<A: Allocator> fmt::Debug for OwnedString<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<A: Allocator> fmt::Display for OwnedString<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<A: Allocator, B: Allocator> PartialEq<OwnedString<B>> for OwnedString<A> {
    #[inline]
    fn eq(&self, other: &OwnedString<B>) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<A: Allocator> PartialEq<str> for OwnedString<A> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<A: Allocator> PartialEq<&str> for OwnedString<A> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<A: Allocator> Eq for OwnedString<A> {}

impl<A: Allocator> DefaultIn<A> for OwnedString<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator> CloneIn<A> for OwnedString<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut bytes = Vec::with_capacity_in(self.len(), alloc);
        bytes.extend_from_slice(self.as_bytes());
        Self { bytes }
    }
}

impl<A: Allocator + Default> Default for OwnedString<A> {
    #[inline]
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> Clone for OwnedString<A> {
    fn clone(&self) -> Self {
        let allocator = self.bytes.allocator().clone();
        let mut bytes = Vec::with_capacity_in(self.len(), allocator);
        bytes.extend_from_slice(self.as_bytes());
        Self { bytes }
    }
}

impl<'a, A: Allocator> ToOwnedIn<A> for str {
    type Owned = OwnedString<A>;

    #[inline]
    fn to_owned_in(&self, alloc: A) -> Self::Owned {
        OwnedString::from_str_in(self, alloc)
    }
}

impl From<&str> for OwnedStringGlobal {
    #[inline]
    fn from(source: &str) -> Self {
        OwnedString::from_str_in(source, Global)
    }
}

/// Error returned when attempting to convert invalid UTF-8 bytes.
pub struct FromUtf8Error<A: Allocator> {
    error: str::Utf8Error,
    bytes: Vec<u8, A>,
}

impl<A: Allocator> FromUtf8Error<A> {
    fn from_std(error: str::Utf8Error, bytes: Vec<u8, A>) -> Self {
        Self { error, bytes }
    }

    /// Returns the underlying `Utf8Error`.
    #[inline]
    pub fn utf8_error(&self) -> &str::Utf8Error {
        &self.error
    }

    /// Consumes the error and returns the owned bytes.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8, A> {
        self.bytes
    }
}

impl<A: Allocator> fmt::Debug for FromUtf8Error<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FromUtf8Error")
            .field("error", &self.error)
            .finish()
    }
}

/// Convenience alias for the global allocator variant.
pub type OwnedStringGlobal = OwnedString<Global>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CloneIn, ToOwnedIn, util};
    use ::allocator_api2::alloc::Global;

    #[test]
    fn new_in_produces_empty_string() {
        let string = OwnedString::new_in(Global);
        assert!(string.is_empty());
    }

    #[test]
    fn from_str_in_copies_utf8() {
        let text = "hello";
        let string = OwnedString::from_str_in(text, Global);
        assert_eq!(string.as_str(), text);
    }

    #[test]
    fn push_str_appends_content() {
        let mut string = OwnedString::from_str_in("foo", Global);
        string.push_str("bar");
        assert_eq!(string.as_str(), "foobar");
    }

    #[test]
    fn clone_in_reallocates() {
        let original = OwnedString::from_str_in("value", Global);
        let cloned = original.clone_in(Global);
        assert_eq!(cloned, original);
    }

    #[test]
    fn clone_uses_same_allocator_when_available() {
        let original = OwnedString::from_str_in("abc", Global);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn to_owned_in_from_str() {
        let text = "owned";
        let string = text.to_owned_in(Global);
        assert_eq!(string.as_str(), text);
    }

    #[test]
    fn invalid_utf8_is_detected() {
        let bytes = util::vec_from_slice_in(&[0xf0, 0x28, 0x8c, 0xbc], Global);
        let result = OwnedString::from_utf8_in(bytes);
        assert!(result.is_err());
    }
}
