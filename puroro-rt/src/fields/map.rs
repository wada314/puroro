//! Protobuf `map<K, V>` field catalog.
//!
//! | Submodule | Contents |
//! |---|---|
//! | [`entry`] | map-entry wire encode / decode (`key=1`, `value=2`) |
//! | [`field`] | [`MapField<K, V, FIELD, A>`] — `K: MapKey`, `V: RepeatedElement` |
//!
//! Borrowed keys become stored [`RepeatedElement::Element`] values via
//! [`ToOwnedIn`](::unmanaged::ToOwnedIn) on `RefView` (copy scalars / `str`).

pub(crate) mod entry;
pub(crate) mod field;

pub use field::{MapField, MapFieldMut, MapFieldRef};

use crate::fields::wire::{
    ProtoBool, ProtoFixed32, ProtoFixed64, ProtoInt32, ProtoInt64, ProtoSFixed32, ProtoSFixed64,
    ProtoSInt32, ProtoSInt64, ProtoString, ProtoUInt32, ProtoUInt64, RepeatedElement,
};

/// Marker: valid protobuf map **key**.
///
/// Spec: integral types, `bool`, or `string` — not floating-point, `bytes`,
/// enum, or message. Storage is [`RepeatedElement::Element`]; the user-facing
/// key type is [`RepeatedElement::RefView`] (`i32`, `str`, …).
///
/// Implementors must ensure:
/// - `RefView: Hash + Eq` (enforced at map lookup / `entry_mut` sites)
/// - `Element<A>: Eq + Hash + Borrow<RefView>` so lookups work with hashbrown
///   `Equivalent`
/// - `RefView: ToOwnedIn<A, Owned = Element<A>>` so vacant `entry` paths can
///   materialize a stored key
pub trait MapKey: RepeatedElement {}

impl MapKey for ProtoInt32 {}
impl MapKey for ProtoInt64 {}
impl MapKey for ProtoUInt32 {}
impl MapKey for ProtoUInt64 {}
impl MapKey for ProtoSInt32 {}
impl MapKey for ProtoSInt64 {}
impl MapKey for ProtoBool {}
impl MapKey for ProtoFixed32 {}
impl MapKey for ProtoFixed64 {}
impl MapKey for ProtoSFixed32 {}
impl MapKey for ProtoSFixed64 {}
impl MapKey for ProtoString {}
