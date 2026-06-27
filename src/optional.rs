//! The [`Optional`] trait — an `Option`-like view over an explicit-presence field.

/// An `Option`-like accessor for a protobuf field that has **explicit presence**
/// and a proto-declared default value.
///
/// The code generator emits this trait as the return type of every
/// explicit-presence field accessor.  The concrete implementing type is
/// created inside the accessor's method body and is not visible in the
/// public API.
///
/// # Accessor naming convention
///
/// For a field `optional int32 max_retries = 3 [default = 3]`:
///
/// | Method | Return type | Semantics |
/// |---|---|---|
/// | `max_retries()` | `impl Optional` | Rich Option-like view |
/// | `max_retries_raw()` | `i32` | Value with default applied; no wrapper |
/// | `has_max_retries()` | `bool` | Presence check only |
///
/// # RPIT and lifetime notes
///
/// The generated accessor for a **scalar** field captures no lifetimes
/// (`+ use<>` in edition 2024 terms) because the concrete View copies the
/// scalar value at construction time.  The common pattern works directly:
///
/// ```rust,ignore
/// let n: i32 = task.max_retries().get();
/// ```
///
/// The generated accessor for a **string / bytes** field captures the
/// message's borrow lifetime (`+ use<'s>`).  Due to Rust's drop-check rules
/// for opaque return types, calling `.get()` on the view in a chained
/// expression does not compile — use a `let` binding, or use `foo_raw()`
/// which directly returns `&str` / `&[u8]` without the Optional wrapper.
pub trait Optional {
    /// The value type produced by this accessor.
    ///
    /// - For scalar types (`i32`, `bool`, `f32`, …) this is the scalar
    ///   itself, with no lifetime parameter.
    /// - For `string` fields this is `&'a str`.
    /// - For `bytes` fields this is `&'a [u8]`.
    type Value<'a>
    where
        Self: 'a;

    /// Returns the field's value, substituting the proto-declared default
    /// when the field is not set.
    fn get(&self) -> Self::Value<'_>;

    /// Returns `Some(value)` if the field was explicitly set, `None` if not.
    ///
    /// Unlike [`get`](Self::get), this does **not** substitute the default.
    fn get_opt(&self) -> Option<Self::Value<'_>>;

    /// Returns `true` if the field was explicitly set.
    fn is_set(&self) -> bool;
}
