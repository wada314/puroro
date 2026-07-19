//! Map key marker over [`RepeatedElement`].
//!
//! Protobuf map keys are a subset of scalar types. Values use
//! [`RepeatedElement`] directly (almost any element type except another map).

use super::fixed::{ProtoFixed32, ProtoFixed64, ProtoSFixed32, ProtoSFixed64};
use super::len::ProtoString;
use super::repeated_element::RepeatedElement;
use super::varint::{
    ProtoBool, ProtoInt32, ProtoInt64, ProtoSint32, ProtoSint64, ProtoUInt32, ProtoUInt64,
};

/// Marker: valid protobuf map **key**.
///
/// Spec: integral types, `bool`, or `string` — not floating-point, `bytes`,
/// enum, or message. Storage is [`RepeatedElement::Element`].
pub trait MapKey: RepeatedElement {}

impl MapKey for ProtoInt32 {}
impl MapKey for ProtoInt64 {}
impl MapKey for ProtoUInt32 {}
impl MapKey for ProtoUInt64 {}
impl MapKey for ProtoSint32 {}
impl MapKey for ProtoSint64 {}
impl MapKey for ProtoBool {}
impl MapKey for ProtoFixed32 {}
impl MapKey for ProtoFixed64 {}
impl MapKey for ProtoSFixed32 {}
impl MapKey for ProtoSFixed64 {}
impl MapKey for ProtoString {}
