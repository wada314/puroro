//! The [`Optional`] type and [`HasDefault`] trait for explicit-presence fields.

use ::core::marker::PhantomData;

/// Zero-sized type carrying a compile-time default for explicit-presence fields.
///
/// Generated code defines a message-local ZST in `{message}/defaults.rs` when the
/// proto field has `[default = …]`. Otherwise the field type's default `D`
/// parameter is [`puroro_rt::ProtoDefault`].
pub trait HasDefault<T: Copy> {
    /// Proto default for this field (`[default = …]` or the type default).
    const DEFAULT: T;
}

/// Return type for explicit-presence field accessors.
///
/// Same shape as `Option<T>`: [`is_set`](Self::is_set) reports whether the
/// field was set, and [`get`](Self::get) returns that value. An absent field
/// still has a value. [`get`](Self::get) returns the default from the `.proto`
/// file. Editions and proto2 can set that default to a value other than the
/// zero value of `T`, with `[default = "foo"]` or `[default = 3]`. Proto3
/// rejects that option, so the default stays the type zero (`""`, `0`,
/// `false`). `D`
/// carries the default ([`HasDefault::DEFAULT`]).
///
/// The type does not convert to `Option<T>`. Use [`is_set`](Self::is_set) for
/// presence and [`get`](Self::get) for the value (explicit or default).
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

    /// Set value, or the `.proto` default when the field is absent.
    ///
    /// The default is [`HasDefault::DEFAULT`]: the type zero, or another
    /// value such as `[default = "foo"]` / `[default = 3]` when the schema
    /// declares one.
    pub fn get(&self) -> T {
        self.value.unwrap_or(D::DEFAULT)
    }

    /// `true` if explicitly set on the wire.
    pub fn is_set(&self) -> bool {
        self.value.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::{HasDefault, Optional};

    struct TitleDefault;
    impl HasDefault<&'static str> for TitleDefault {
        const DEFAULT: &'static str = "foo";
    }

    struct MaxRetriesDefault;
    impl HasDefault<i32> for MaxRetriesDefault {
        const DEFAULT: i32 = 3;
    }

    #[test]
    fn absent_get_returns_declared_default() {
        let title = Optional::<&str, TitleDefault>::new(None);
        assert!(!title.is_set());
        assert_eq!(title.get(), "foo");

        let retries = Optional::<i32, MaxRetriesDefault>::new(None);
        assert!(!retries.is_set());
        assert_eq!(retries.get(), 3);
    }

    #[test]
    fn set_value_hides_default() {
        let title = Optional::<&str, TitleDefault>::new(Some("Shore"));
        assert!(title.is_set());
        assert_eq!(title.get(), "Shore");
    }
}
