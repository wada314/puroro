//! The [`Optional`] type and [`HasDefault`] trait for explicit-presence fields.

use ::core::marker::PhantomData;

/// Implemented by zero-sized types that carry a compile-time default value.
///
/// The implementing type is a private struct defined inside the accessor
/// method body — it never appears in the public API.
///
/// For string fields, implement with a lifetime parameter so the same
/// marker type works for any borrow lifetime:
///
/// ```rust,ignore
/// struct TitleDefault;
/// impl<'a> HasDefault<&'a str> for TitleDefault {
///     const DEFAULT: &'a str = "N/A";  // &'static str coerces to &'a str
/// }
/// ```
pub trait HasDefault<T: Copy> {
    const DEFAULT: T;
}

/// An `Option`-like return value for explicit-presence fields.
///
/// `T` is the value type — either a `Copy` scalar (`i32`, `bool`, …) or a
/// reference (`&'a str`, `&'a [u8]`).  `D` is a zero-sized type providing
/// the proto-declared default as a compile-time constant via [`HasDefault<T>`].
///
/// The concrete `D` is always a private struct defined locally inside the
/// accessor method.  The return type in generated code reads:
/// `Optional<i32, impl HasDefault<i32>>` (or the string / bytes variant).
///
/// # Usage in generated code
///
/// ```rust,ignore
/// // For: optional int32 max_retries = 3 [default = 3];
/// pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> {
///     struct Default3;
///     impl HasDefault<i32> for Default3 { const DEFAULT: i32 = 3; }
///     Optional::new(self.max_retries, Default3)
/// }
///
/// // For: string title = 1 [default = "N/A"];
/// pub fn title<'s>(&'s self) -> Optional<&'s str, impl HasDefault<&'s str>> {
///     struct DefaultNA;
///     impl<'a> HasDefault<&'a str> for DefaultNA { const DEFAULT: &'a str = "N/A"; }
///     Optional::new(self.title.as_deref(), DefaultNA)
/// }
///
/// // Both chain directly — no let binding required:
/// let n: i32 = task.max_retries().get();
/// let s: &str = task.title().get();
/// ```
pub struct Optional<T: Copy, D: HasDefault<T>> {
    value: Option<T>,
    _phantom: PhantomData<D>,
}

impl<T: Copy, D: HasDefault<T>> Optional<T, D> {
    /// Creates an `Optional` from the raw `Option<T>`.
    ///
    /// The `_tag` argument is a zero-sized `D` value used solely to let
    /// the compiler infer the `D` type parameter from the call site.
    pub fn new(value: Option<T>, _tag: D) -> Self {
        Self { value, _phantom: PhantomData }
    }

    /// Returns the field's value, or the proto-declared default when not set.
    ///
    /// This is the primary accessor.  The returned value is always meaningful —
    /// callers never receive a "nothing" result.  Use [`is_set`](Self::is_set)
    /// separately if you need to distinguish "explicitly set" from "default".
    pub fn get(&self) -> T { self.value.unwrap_or(D::DEFAULT) }

    /// Returns `true` if the field was explicitly set on the wire.
    ///
    /// # Note on `Option` conversion
    ///
    /// `Optional<T, D>` intentionally provides no method that returns
    /// `Option<T>`.  Proto explicit-presence fields always carry a meaningful
    /// value (either the set value or the declared default); mapping to
    /// `Option` would conflate "not set" with "no value", making the
    /// default-value semantics impossible to enforce at the type level.
    ///
    /// # Lazy implementations (`TaskLazy`)
    ///
    /// `merge_from` stores wire bytes only.  Getters wire-scan and decode on
    /// demand; `Optional` is constructed only after a successful semantic decode.
    /// `has_*()` may wire-scan without semantic decode.  See `DESIGN.md` §8.
    pub fn is_set(&self) -> bool { self.value.is_some() }
}
