//! Map key marker over [`RepeatedElement`].
//!
//! Protobuf map keys are a subset of scalar types. Values use any
//! [`RepeatedElement`] (except another map). Shared key / value views are
//! [`RepeatedElement::RefView`].
//!
//! Borrowed keys become stored [`RepeatedElement::Element`] values via
//! [`ToOwnedIn`](::unmanaged::ToOwnedIn) on `RefView` (copy scalars / `str`).

use super::fixed::{ProtoFixed32, ProtoFixed64, ProtoSFixed32, ProtoSFixed64};
use super::len::ProtoString;
use super::repeated_element::RepeatedElement;
use super::varint::{
    ProtoBool, ProtoInt32, ProtoInt64, ProtoSInt32, ProtoSInt64, ProtoUInt32, ProtoUInt64,
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
