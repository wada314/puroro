//! The [`Optional`] type and [`HasDefault`] trait for explicit-presence fields.
//!
//! See `DESIGN.md` §3 for semantics and §8 for lazy (`TaskLazy`) behaviour.

use ::core::marker::PhantomData;

/// Zero-sized type carrying a compile-time default for explicit-presence fields.
///
/// Generated code defines a private struct inside each accessor and implements
/// this trait with the proto `[default = …]` value. For strings, use a
/// lifetime-generic impl so `const DEFAULT: &'static str` coerces to any `&'a str`.
pub trait HasDefault<T: Copy> {
    const DEFAULT: T;
}

/// Return type for explicit-presence field accessors.
///
/// Wraps a fully decoded `Option<T>`: `None` = absent on wire; `Some(t)` = set.
/// Provides [`get`](Self::get) (value or proto default) and
/// [`is_set`](Self::is_set) (explicit presence). No conversion to `Option<T>`.
///
/// On `TaskLazy`, the getter returns `Err` before constructing `Optional` if
/// semantic decode fails; use `has_*()` for wire presence without validation.
pub struct Optional<T: Copy, D: HasDefault<T>> {
    value: Option<T>,
    _phantom: PhantomData<D>,
}

impl<T: Copy, D: HasDefault<T>> Optional<T, D> {
    /// Creates an `Optional`. `_tag` is a zero-sized `D` for type inference.
    pub fn new(value: Option<T>, _tag: D) -> Self {
        Self { value, _phantom: PhantomData }
    }

    /// Value or proto-declared default when not set.
    pub fn get(&self) -> T { self.value.unwrap_or(D::DEFAULT) }

    /// `true` if explicitly set on the wire.
    pub fn is_set(&self) -> bool { self.value.is_some() }
}
