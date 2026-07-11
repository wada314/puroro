//! The [`Optional`] type and [`HasDefault`] trait for explicit-presence fields.
//!
//! See `DESIGN.md` §3 for semantics and §8 for lazy (`TaskLazy`) behaviour.

use ::core::marker::PhantomData;

/// Zero-sized type carrying a compile-time default for explicit-presence fields.
///
/// Generated code defines a message-local ZST in `{message}/defaults.rs` when the
/// proto field has `[default = …]`. Otherwise the field type's default `D`
/// parameter is [`puroro_rt::ProtoDefault`].
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
    /// Creates an `Optional` using the default marker `D` from the field type.
    pub fn new(value: Option<T>) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Value or proto-declared default when not set.
    pub fn get(&self) -> T {
        self.value.unwrap_or(D::DEFAULT)
    }

    /// `true` if explicitly set on the wire.
    pub fn is_set(&self) -> bool {
        self.value.is_some()
    }
}
