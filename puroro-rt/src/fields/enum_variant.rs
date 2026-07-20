//! Const-generic dispatch on enum variants by proto field number.
//!
//! Generated oneof storage enums implement [`EnumVariant`] once per variant,
//! keyed by that variant's field number. This lets
//! [`OneofSlot`](super::oneof::OneofSlot) and
//! [`OneofSlotMut`](super::oneof::OneofSlotMut) select a variant without
//! per-call-site closure boilerplate or zero-sized marker types.
//!
//! Empty payload construction uses [`DefaultIn`](super::shared::DefaultIn) on
//! [`Value`](Self::Value) (e.g. [`SingularField::default_in`](super::singular::SingularField)),
//! not a per-variant `new_value` hook.

/// Field-number access to one variant of an enum `Self`.
///
/// `FIELD` is the protobuf field number of the variant. Generated code
/// implements this trait on the storage enum for each member.
pub trait EnumVariant<const FIELD: u32> {
    /// The payload type stored in the variant numbered `FIELD`.
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

    /// Wraps `value` as the variant numbered `FIELD` of `Self`.
    fn from_variant(value: Self::Value) -> Self;
}
