//! Proto field → catalog [`FieldKind`] IR.
//!
//! [`plan::plan_fields`] walks a resolved [`Message`](crate::resolved::Message),
//! assigns common bit indices (ascending field number, one pass),
//! and produces a [`MessageFieldPlan`] ready for catalog emission.
//!
//! Mapping table: [IMPLEMENTATION.md §8](../../IMPLEMENTATION.md#8-proto-field--catalog-mapping).

mod plan;
mod storage;
mod tarjan;

pub use plan::{MessageFieldPlan, MessageMember, PlannedField, PlannedOneof, plan_fields};
pub use storage::{MessageStoragePlan, plan_message_storage};

use crate::case::to_upper_snake;
use crate::default_value::CustomDefault;
use crate::descriptor::features::{EnumType, RepeatedFieldEncoding, Utf8Validation};
use crate::resolved::{Enum, Field, Message, SingularPresence, TypeRef};

/// Catalog shape for one generated field (struct member or oneof variant).
#[derive(Debug, Clone)]
pub enum FieldKind<'a> {
    Singular {
        wire: WireTypeKind<'a>,
        presence: PlannedPresence,
        layout: PlannedLayout,
        /// Non-type-zero `[default = …]`; `None` keeps `ProtoDefault`.
        custom_default: Option<CustomDefault>,
    },
    Repeated {
        wire: WireTypeKind<'a>,
        encoding: RepeatedEncodingKind,
    },
    /// `map<K, V>` — key/value wires from the synthetic entry message.
    Map {
        key: WireTypeKind<'a>,
        value: WireTypeKind<'a>,
    },
}

/// Thin `puroro_rt` wire / element marker corresponding to a protobuf type.
#[derive(Debug, Clone, Copy)]
pub enum WireTypeKind<'a> {
    Double,
    Float,
    Int64,
    UInt64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String {
        utf8: Utf8Validation,
    },
    Bytes {
        utf8: Utf8Validation,
    },
    UInt32,
    SFixed32,
    SFixed64,
    SInt32,
    SInt64,
    Message(&'a Message<'a>),
    Enum {
        ty: &'a Enum<'a>,
        openness: EnumType,
    },
}

/// Catalog marker for a protobuf `string` field.
///
/// Plan decides the marker; emit only quotes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCatalogMarker {
    /// `::puroro_rt::ProtoString` (`utf8_validation=VERIFY`).
    ProtoString,
    /// `::puroro_rt::ProtoStringUnchecked` (`utf8_validation=NONE`).
    /// Bytes views (`&[u8]`); still a valid map key (unlike `bytes`).
    ProtoStringUnchecked,
}

impl StringCatalogMarker {
    pub fn from_utf8(utf8: Utf8Validation) -> Self {
        match utf8 {
            Utf8Validation::Verify => Self::ProtoString,
            Utf8Validation::None => Self::ProtoStringUnchecked,
        }
    }
}

/// `FieldPresence` marker baked into `SingularField<…, P, …>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannedPresence {
    Implicit,
    Explicit {
        bit: usize,
        /// e.g. `BIT_TITLE`
        bit_const: String,
    },
    LegacyRequired {
        bit: usize,
        bit_const: String,
    },
    /// Variant inside a [`PlannedOneof`] / `OneofSlot`.
    Oneof,
    /// Singular message — pointer presence (`Message` marker).
    Message,
}

/// `ValueLayout` on `SingularField` (`Inline` default vs `Boxed` / `BitPacked` / SSO).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannedLayout {
    Inline,
    /// Nested message behind `UnmanagedBox` (`SingularField<…, Boxed>`).
    Boxed,
    BitPacked {
        value_bit: usize,
        /// e.g. `BIT_DONE_VALUE`
        bit_const: String,
    },
    /// Singular `string` / `bytes` — heap/inline bit for [`InlineOrHeap`] (`true` = heap).
    InlineOrHeap {
        /// Index of the `MessageCommon` bit (`1` = heap arm, `0` = inline).
        heap_bit: usize,
        /// e.g. `BIT_TITLE_SSO`
        bit_const: String,
    },
}

/// `RepeatedField` encoding parameter (`Packed` / `Expanded`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatedEncodingKind {
    Packed,
    Expanded,
}

impl RepeatedEncodingKind {
    pub fn from_resolved(encoding: RepeatedFieldEncoding) -> Self {
        match encoding {
            RepeatedFieldEncoding::Packed => Self::Packed,
            RepeatedFieldEncoding::Expanded => Self::Expanded,
        }
    }
}

impl<'a> WireTypeKind<'a> {
    pub fn from_field(field: &Field<'a>) -> Self {
        let utf8 = field.utf8_validation();
        match field.type_ref() {
            TypeRef::Double => Self::Double,
            TypeRef::Float => Self::Float,
            TypeRef::Int64 => Self::Int64,
            TypeRef::UInt64 => Self::UInt64,
            TypeRef::Int32 => Self::Int32,
            TypeRef::Fixed64 => Self::Fixed64,
            TypeRef::Fixed32 => Self::Fixed32,
            TypeRef::Bool => Self::Bool,
            TypeRef::String => Self::String {
                utf8: utf8.unwrap_or(Utf8Validation::Verify),
            },
            TypeRef::Bytes => Self::Bytes {
                utf8: utf8.unwrap_or(Utf8Validation::Verify),
            },
            TypeRef::UInt32 => Self::UInt32,
            TypeRef::SFixed32 => Self::SFixed32,
            TypeRef::SFixed64 => Self::SFixed64,
            TypeRef::SInt32 => Self::SInt32,
            TypeRef::SInt64 => Self::SInt64,
            TypeRef::Message(m) => Self::Message(m),
            TypeRef::Enum(e) => Self::Enum {
                ty: e,
                openness: e.openness(),
            },
        }
    }

    /// Whether the type may use packed repeated wire encoding.
    pub fn is_packable(self) -> bool {
        match self {
            Self::String { .. } | Self::Bytes { .. } | Self::Message(_) => false,
            Self::Double
            | Self::Float
            | Self::Int64
            | Self::UInt64
            | Self::Int32
            | Self::Fixed64
            | Self::Fixed32
            | Self::Bool
            | Self::UInt32
            | Self::SFixed32
            | Self::SFixed64
            | Self::SInt32
            | Self::SInt64
            | Self::Enum { .. } => true,
        }
    }
}

impl PlannedPresence {
    pub fn from_singular(
        presence: SingularPresence,
        proto_name: &str,
        next_bit: &mut usize,
    ) -> Self {
        match presence {
            SingularPresence::Implicit => Self::Implicit,
            SingularPresence::Explicit => {
                let bit = *next_bit;
                *next_bit += 1;
                Self::Explicit {
                    bit,
                    bit_const: presence_bit_const(proto_name),
                }
            }
            SingularPresence::LegacyRequired => {
                let bit = *next_bit;
                *next_bit += 1;
                Self::LegacyRequired {
                    bit,
                    bit_const: presence_bit_const(proto_name),
                }
            }
            SingularPresence::Oneof => Self::Oneof,
            SingularPresence::Message => Self::Message,
        }
    }
}

/// `FIELD_TITLE` from proto field name `title`.
pub fn field_number_const(proto_name: &str) -> String {
    format!("FIELD_{}", to_upper_snake(proto_name))
}

/// `BIT_TITLE` — presence bit for EXPLICIT / LEGACY_REQUIRED.
pub fn presence_bit_const(proto_name: &str) -> String {
    format!("BIT_{}", to_upper_snake(proto_name))
}

/// `BIT_DONE_VALUE` — bool value bit in `BitPacked`.
pub fn value_bit_const(proto_name: &str) -> String {
    format!("BIT_{}_VALUE", to_upper_snake(proto_name))
}

/// `BIT_TITLE_SSO` — singular string / bytes heap bit in `InlineOrHeap` (`1` = heap).
pub fn sso_bit_const(proto_name: &str) -> String {
    format!("BIT_{}_SSO", to_upper_snake(proto_name))
}

/// Byte length of `BitArray<[u8; N], Lsb0>` for `bit_count` bits (`N == 0` allowed).
pub fn bit_array_byte_len(bit_count: usize) -> usize {
    bit_count.div_ceil(8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_catalog_marker_follows_utf8_validation() {
        assert_eq!(
            StringCatalogMarker::from_utf8(Utf8Validation::Verify),
            StringCatalogMarker::ProtoString
        );
        assert_eq!(
            StringCatalogMarker::from_utf8(Utf8Validation::None),
            StringCatalogMarker::ProtoStringUnchecked
        );
    }
}
