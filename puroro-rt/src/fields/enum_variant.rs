//! Type-parameter dispatch on enum variants.
//!
//! Generated oneof storage enums implement [`EnumVariant`] once per variant,
//! using a zero-sized marker type as `V`. This lets [`OneofSlot`](super::oneof::OneofSlot)
//! and [`OneofSlotMut`](super::oneof::OneofSlotMut) select a variant without
//! per-call-site closure boilerplate.
//!
//! Empty payload construction uses [`DefaultIn`](super::shared::DefaultIn) on
//! [`Value`](Self::Value) (e.g. [`SingularField::default_in`](super::singular::SingularField)),
//! not a per-variant `new_value` hook.

/// Marker-typed access to one variant `V` of an enum `Self`.
///
/// Generated code provides a zero-sized marker type per variant and implements
/// this trait on the storage enum for each marker.
pub trait EnumVariant<V> {
    /// The payload type stored in variant `V`.
    ///
    /// Installing a fresh variant uses [`DefaultIn`](super::shared::DefaultIn)
    /// on this type at the [`OneofSlotMut::variant_mut`](super::oneof::OneofSlotMut::variant_mut)
    /// call site.
    type Value;

    /// Borrowed reference to the variant's payload, or `None` if a different
    /// variant is active.
    fn variant_ref(&self) -> Option<&Self::Value>;

    /// Mutable reference ditto.
    fn variant_mut(&mut self) -> Option<&mut Self::Value>;

    /// Wraps `value` as variant `V` of `Self`.
    fn from_variant(value: Self::Value) -> Self;
}
