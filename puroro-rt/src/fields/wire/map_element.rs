//! Map key marker over [`RepeatedElement`].
//!
//! Protobuf map keys are a subset of scalar types. Values use any
//! [`RepeatedElement`] (except another map). Shared key / value views are
//! [`RepeatedElement::RefView`].

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedString;

use super::fixed::{ProtoFixed32, ProtoFixed64, ProtoSFixed32, ProtoSFixed64};
use super::len::ProtoString;
use super::repeated_element::RepeatedElement;
use super::varint::{
    ProtoBool, ProtoInt32, ProtoInt64, ProtoSInt32, ProtoSInt64, ProtoUInt32, ProtoUInt64,
};
use crate::decode;

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
pub trait MapKey: RepeatedElement {
    /// Build a stored key from a [`RefView`](RepeatedElement::RefView)
    /// (`entry_mut` path).
    fn key_from_view<A: Allocator + Clone>(view: &Self::RefView, alloc: A) -> Self::Element<A>;
}

// ---------------------------------------------------------------------------
// MapKey
// ---------------------------------------------------------------------------

macro_rules! impl_copy_map_key {
    ($marker:ty, $view:ty) => {
        impl MapKey for $marker {
            #[inline]
            fn key_from_view<A: Allocator + Clone>(view: &$view, _alloc: A) -> $view {
                *view
            }
        }
    };
}

impl_copy_map_key!(ProtoInt32, i32);
impl_copy_map_key!(ProtoInt64, i64);
impl_copy_map_key!(ProtoUInt32, u32);
impl_copy_map_key!(ProtoUInt64, u64);
impl_copy_map_key!(ProtoSInt32, i32);
impl_copy_map_key!(ProtoSInt64, i64);
impl_copy_map_key!(ProtoBool, bool);
impl_copy_map_key!(ProtoFixed32, u32);
impl_copy_map_key!(ProtoFixed64, u64);
impl_copy_map_key!(ProtoSFixed32, i32);
impl_copy_map_key!(ProtoSFixed64, i64);

impl MapKey for ProtoString {
    #[inline]
    fn key_from_view<A: Allocator + Clone>(view: &str, alloc: A) -> UnmanagedString<A> {
        decode::str_to_unmanaged_in(view, alloc)
    }
}
