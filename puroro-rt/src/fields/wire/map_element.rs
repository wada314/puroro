//! Map key / value view projection over [`RepeatedElement`].
//!
//! Protobuf map keys are a subset of scalar types. Values use
//! [`MapValueView`] (almost any element type except another map).

use ::core::hash::Hash;
use ::core::ops::Deref;

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use super::fixed::{
    ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat, ProtoSFixed32, ProtoSFixed64,
};
use super::len::{ProtoBytes, ProtoString};
use super::proto_message::ProtoMessage;
use super::repeated_element::RepeatedElement;
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSint32,
    ProtoSint64, ProtoUInt32, ProtoUInt64,
};
use crate::decode;
use ::puroro::Message;

/// Marker: valid protobuf map **key**.
///
/// Spec: integral types, `bool`, or `string` — not floating-point, `bytes`,
/// enum, or message. Storage is [`RepeatedElement::Element`].
///
/// [`KeyView`](Self::KeyView) is the user-facing key type (`i32`, `str`, …).
/// Implementors must ensure `Element<A>: Eq + Hash + Borrow<KeyView>` so lookups
/// via [`KeyView`] work with hashbrown `Equivalent`.
pub trait MapKey: RepeatedElement {
    /// Shared key view for [`MapRef`](::puroro::MapRef) / [`MapMut`](::puroro::MapMut).
    type KeyView: ?Sized + Hash + Eq;

    /// Build a stored key from a [`KeyView`](Self::KeyView) (`entry_mut` path).
    fn key_from_view<A: Allocator + Clone>(view: &Self::KeyView, alloc: A) -> Self::Element<A>;
}

/// Shared map-value view projection (`Element` → user-facing [`View`](Self::View)).
///
/// Distinct from [`ProtoType::Ref`](super::proto_type::ProtoType::Ref): scalars use
/// by-value `Ref = i32`, while maps need `&View` with `View = i32`.
pub trait MapValueView: RepeatedElement {
    type View: ?Sized;

    fn as_view<A: Allocator + Clone>(elem: &Self::Element<A>) -> &Self::View;
}

// ---------------------------------------------------------------------------
// MapKey
// ---------------------------------------------------------------------------

macro_rules! impl_copy_map_key {
    ($marker:ty, $view:ty) => {
        impl MapKey for $marker {
            type KeyView = $view;

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
impl_copy_map_key!(ProtoSint32, i32);
impl_copy_map_key!(ProtoSint64, i64);
impl_copy_map_key!(ProtoBool, bool);
impl_copy_map_key!(ProtoFixed32, u32);
impl_copy_map_key!(ProtoFixed64, u64);
impl_copy_map_key!(ProtoSFixed32, i32);
impl_copy_map_key!(ProtoSFixed64, i64);

impl MapKey for ProtoString {
    type KeyView = str;

    #[inline]
    fn key_from_view<A: Allocator + Clone>(view: &str, alloc: A) -> UnmanagedString<A> {
        decode::str_to_unmanaged_in(view, alloc)
    }
}

// ---------------------------------------------------------------------------
// MapValueView
// ---------------------------------------------------------------------------

macro_rules! impl_identity_map_value_view {
    ($marker:ty, $view:ty) => {
        impl MapValueView for $marker {
            type View = $view;

            #[inline]
            fn as_view<A: Allocator + Clone>(elem: &$view) -> &$view {
                elem
            }
        }
    };
}

impl_identity_map_value_view!(ProtoDouble, f64);
impl_identity_map_value_view!(ProtoFloat, f32);
impl_identity_map_value_view!(ProtoInt64, i64);
impl_identity_map_value_view!(ProtoUInt64, u64);
impl_identity_map_value_view!(ProtoInt32, i32);
impl_identity_map_value_view!(ProtoFixed64, u64);
impl_identity_map_value_view!(ProtoFixed32, u32);
impl_identity_map_value_view!(ProtoBool, bool);
impl_identity_map_value_view!(ProtoUInt32, u32);
impl_identity_map_value_view!(ProtoSFixed32, i32);
impl_identity_map_value_view!(ProtoSFixed64, i64);
impl_identity_map_value_view!(ProtoSint32, i32);
impl_identity_map_value_view!(ProtoSint64, i64);

macro_rules! impl_enum_map_value_view {
    ($kind:ty, $bound:ident) => {
        impl<E: $bound> MapValueView for ProtoEnum<E, $kind> {
            type View = E;

            #[inline]
            fn as_view<A: Allocator + Clone>(elem: &E) -> &E {
                elem
            }
        }
    };
}

impl_enum_map_value_view!(Open, OpenEnum);
impl_enum_map_value_view!(Closed, ClosedEnum);

impl MapValueView for ProtoString {
    type View = str;

    #[inline]
    fn as_view<A: Allocator + Clone>(elem: &UnmanagedString<A>) -> &str {
        elem.deref()
    }
}

impl MapValueView for ProtoBytes {
    type View = [u8];

    #[inline]
    fn as_view<A: Allocator + Clone>(elem: &UnmanagedVec<u8, A>) -> &[u8] {
        elem.deref()
    }
}

impl<M: Message> MapValueView for ProtoMessage<M> {
    type View = M;

    #[inline]
    fn as_view<A: Allocator + Clone>(elem: &M) -> &M {
        elem
    }
}
