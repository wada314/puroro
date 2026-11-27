//! Thin wire views over `descriptor.proto` / `plugin.proto` message bytes.
//!
//! Each type wraps a message LEN body (`&[u8]`) and exposes named getters that
//! look up a field number via [`protobuf_core::AsRefExtProtobuf`]. Field numbers
//! are `pub(crate)` associated constants on the same type (for example
//! `FileDescriptorProto::NAME`). This is not `puroro-types` generated code
//! (bootstrap cycle); it is a hand-written subset of the official field layout.
//!
//! Singular getters are last-wins. Wrong wire types for a requested field are
//! errors; other fields are ignored.

use crate::error::{Error, Result};
use ::protobuf_core::{AsRefExtProtobuf, Field, FieldValue, ReadExtVarint};
use ::std::str;

/// Shared helpers for last-wins / repeated field lookups on a message body.
mod raw {
    use super::*;

    pub(super) fn last_len(bytes: &[u8], number: u32) -> Result<Option<&[u8]>> {
        let mut found = None;
        for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
            let field = field?;
            if field.field_number.as_u32() == number {
                found = Some(expect_len(&field)?);
            }
        }
        Ok(found)
    }

    pub(super) fn last_string(bytes: &[u8], number: u32) -> Result<Option<&str>> {
        match last_len(bytes, number)? {
            None => Ok(None),
            Some(payload) => Ok(Some(str::from_utf8(payload)?)),
        }
    }

    pub(super) fn last_i32(bytes: &[u8], number: u32) -> Result<Option<i32>> {
        let mut found = None;
        for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
            let field = field?;
            if field.field_number.as_u32() == number {
                found = Some(expect_i32(&field)?);
            }
        }
        Ok(found)
    }

    pub(super) fn last_bool(bytes: &[u8], number: u32) -> Result<Option<bool>> {
        let mut found = None;
        for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
            let field = field?;
            if field.field_number.as_u32() == number {
                found = Some(expect_bool(&field)?);
            }
        }
        Ok(found)
    }

    pub(super) fn repeated_strings(bytes: &[u8], number: u32) -> Result<Vec<&str>> {
        let mut out = Vec::new();
        for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
            let field = field?;
            if field.field_number.as_u32() == number {
                out.push(str::from_utf8(expect_len(&field)?)?);
            }
        }
        Ok(out)
    }

    pub(super) fn repeated_lens(bytes: &[u8], number: u32) -> Result<Vec<&[u8]>> {
        let mut out = Vec::new();
        for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
            let field = field?;
            if field.field_number.as_u32() == number {
                out.push(expect_len(&field)?);
            }
        }
        Ok(out)
    }

    /// Packed or unpacked `repeated int32`.
    pub(super) fn repeated_i32s(bytes: &[u8], number: u32) -> Result<Vec<i32>> {
        let mut out = Vec::new();
        for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
            let field = field?;
            if field.field_number.as_u32() != number {
                continue;
            }
            match &field.value {
                FieldValue::Len(payload) => {
                    let mut slice: &[u8] = payload;
                    while !slice.is_empty() {
                        let varint = slice.read_varint()?;
                        out.push(varint.try_to_int32()?);
                    }
                }
                FieldValue::Varint(v) => out.push(v.try_to_int32()?),
                _ => return Err(Error::unexpected_field(field.field_number.as_u32())),
            }
        }
        Ok(out)
    }

    fn expect_len<'a>(field: &Field<&'a [u8]>) -> Result<&'a [u8]> {
        match &field.value {
            FieldValue::Len(bytes) => Ok(*bytes),
            _ => Err(Error::unexpected_field(field.field_number.as_u32())),
        }
    }

    fn expect_i32(field: &Field<&[u8]>) -> Result<i32> {
        match &field.value {
            FieldValue::Varint(v) => Ok(v.try_to_int32()?),
            _ => Err(Error::unexpected_field(field.field_number.as_u32())),
        }
    }

    fn expect_bool(field: &Field<&[u8]>) -> Result<bool> {
        match &field.value {
            FieldValue::Varint(v) => Ok(v.to_bool()),
            _ => Err(Error::unexpected_field(field.field_number.as_u32())),
        }
    }
}

/// `google.protobuf.FileDescriptorSet` wire numbers (encode helper).
#[derive(Debug, Clone, Copy)]
pub struct FileDescriptorSet;

impl FileDescriptorSet {
    /// `repeated FileDescriptorProto file = 1;`
    pub(crate) const FILE: u32 = 1;
}

/// `google.protobuf.compiler.CodeGeneratorRequest` wire view.
#[derive(Debug, Clone, Copy)]
pub struct CodeGeneratorRequest<'a> {
    bytes: &'a [u8],
}

impl<'a> CodeGeneratorRequest<'a> {
    pub(crate) const FILE_TO_GENERATE: u32 = 1;
    pub(crate) const PARAMETER: u32 = 2;
    pub(crate) const PROTO_FILE: u32 = 15;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }

    /// `repeated string file_to_generate = 1;`
    pub fn file_to_generate(self) -> Result<Vec<&'a str>> {
        raw::repeated_strings(self.bytes, Self::FILE_TO_GENERATE)
    }

    /// `optional string parameter = 2;`
    pub fn parameter(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::PARAMETER)
    }

    /// `repeated FileDescriptorProto proto_file = 15;`
    pub fn proto_file(self) -> Result<Vec<FileDescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::PROTO_FILE)
            .map(|bodies| bodies.into_iter().map(FileDescriptorProto::new).collect())
    }
}

/// `google.protobuf.FileDescriptorProto` wire view.
#[derive(Debug, Clone, Copy)]
pub struct FileDescriptorProto<'a> {
    bytes: &'a [u8],
}

impl<'a> FileDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const PACKAGE: u32 = 2;
    pub(crate) const DEPENDENCY: u32 = 3;
    pub(crate) const MESSAGE_TYPE: u32 = 4;
    pub(crate) const ENUM_TYPE: u32 = 5;
    pub(crate) const OPTIONS: u32 = 8;
    pub(crate) const SOURCE_CODE_INFO: u32 = 9;
    pub(crate) const SYNTAX: u32 = 12;
    pub(crate) const EDITION: u32 = 14;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }

    /// `optional string name = 1;`
    pub fn name(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::NAME)
    }

    /// `optional string package = 2;`
    pub fn package(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::PACKAGE)
    }

    /// `repeated string dependency = 3;`
    pub fn dependency(self) -> Result<Vec<&'a str>> {
        raw::repeated_strings(self.bytes, Self::DEPENDENCY)
    }

    /// `repeated DescriptorProto message_type = 4;`
    pub fn message_type(self) -> Result<Vec<DescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::MESSAGE_TYPE)
            .map(|bodies| bodies.into_iter().map(DescriptorProto::new).collect())
    }

    /// `repeated EnumDescriptorProto enum_type = 5;`
    pub fn enum_type(self) -> Result<Vec<EnumDescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::ENUM_TYPE)
            .map(|bodies| bodies.into_iter().map(EnumDescriptorProto::new).collect())
    }

    /// `optional FileOptions options = 8;`
    pub fn options(self) -> Result<Option<FileOptions<'a>>> {
        Ok(raw::last_len(self.bytes, Self::OPTIONS)?.map(FileOptions::new))
    }

    /// `optional SourceCodeInfo source_code_info = 9;`
    pub fn source_code_info(self) -> Result<Option<SourceCodeInfo<'a>>> {
        Ok(raw::last_len(self.bytes, Self::SOURCE_CODE_INFO)?.map(SourceCodeInfo::new))
    }

    /// `optional string syntax = 12;`
    pub fn syntax(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::SYNTAX)
    }

    /// `optional Edition edition = 14;`
    pub fn edition(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::EDITION)
    }
}

/// `google.protobuf.DescriptorProto` wire view.
#[derive(Debug, Clone, Copy)]
pub struct DescriptorProto<'a> {
    bytes: &'a [u8],
}

impl<'a> DescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const FIELD: u32 = 2;
    pub(crate) const NESTED_TYPE: u32 = 3;
    pub(crate) const ENUM_TYPE: u32 = 4;
    pub(crate) const OPTIONS: u32 = 7;
    pub(crate) const ONEOF_DECL: u32 = 8;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }

    pub fn name(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::NAME)
    }

    pub fn field(self) -> Result<Vec<FieldDescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::FIELD)
            .map(|bodies| bodies.into_iter().map(FieldDescriptorProto::new).collect())
    }

    pub fn nested_type(self) -> Result<Vec<DescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::NESTED_TYPE)
            .map(|bodies| bodies.into_iter().map(DescriptorProto::new).collect())
    }

    pub fn enum_type(self) -> Result<Vec<EnumDescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::ENUM_TYPE)
            .map(|bodies| bodies.into_iter().map(EnumDescriptorProto::new).collect())
    }

    pub fn options(self) -> Result<Option<MessageOptions<'a>>> {
        Ok(raw::last_len(self.bytes, Self::OPTIONS)?.map(MessageOptions::new))
    }

    pub fn oneof_decl(self) -> Result<Vec<OneofDescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::ONEOF_DECL)
            .map(|bodies| bodies.into_iter().map(OneofDescriptorProto::new).collect())
    }
}

/// `google.protobuf.FieldDescriptorProto` wire view.
#[derive(Debug, Clone, Copy)]
pub struct FieldDescriptorProto<'a> {
    bytes: &'a [u8],
}

impl<'a> FieldDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const NUMBER: u32 = 3;
    pub(crate) const LABEL: u32 = 4;
    pub(crate) const TYPE: u32 = 5;
    pub(crate) const TYPE_NAME: u32 = 6;
    pub(crate) const DEFAULT_VALUE: u32 = 7;
    pub(crate) const OPTIONS: u32 = 8;
    pub(crate) const ONEOF_INDEX: u32 = 9;
    pub(crate) const PROTO3_OPTIONAL: u32 = 17;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn name(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::NAME)
    }

    pub fn number(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::NUMBER)
    }

    pub fn label(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::LABEL)
    }

    pub fn type_(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::TYPE)
    }

    pub fn type_name(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::TYPE_NAME)
    }

    pub fn default_value(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::DEFAULT_VALUE)
    }

    pub fn options(self) -> Result<Option<FieldOptions<'a>>> {
        Ok(raw::last_len(self.bytes, Self::OPTIONS)?.map(FieldOptions::new))
    }

    pub fn oneof_index(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::ONEOF_INDEX)
    }

    pub fn proto3_optional(self) -> Result<Option<bool>> {
        raw::last_bool(self.bytes, Self::PROTO3_OPTIONAL)
    }
}

/// `google.protobuf.OneofDescriptorProto` wire view.
#[derive(Debug, Clone, Copy)]
pub struct OneofDescriptorProto<'a> {
    bytes: &'a [u8],
}

impl<'a> OneofDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn name(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::NAME)
    }
}

/// `google.protobuf.EnumDescriptorProto` wire view.
#[derive(Debug, Clone, Copy)]
pub struct EnumDescriptorProto<'a> {
    bytes: &'a [u8],
}

impl<'a> EnumDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const VALUE: u32 = 2;
    pub(crate) const OPTIONS: u32 = 3;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn name(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::NAME)
    }

    pub fn value(self) -> Result<Vec<EnumValueDescriptorProto<'a>>> {
        raw::repeated_lens(self.bytes, Self::VALUE).map(|bodies| {
            bodies
                .into_iter()
                .map(EnumValueDescriptorProto::new)
                .collect()
        })
    }

    pub fn options(self) -> Result<Option<EnumOptions<'a>>> {
        Ok(raw::last_len(self.bytes, Self::OPTIONS)?.map(EnumOptions::new))
    }
}

/// `google.protobuf.EnumValueDescriptorProto` wire view.
#[derive(Debug, Clone, Copy)]
pub struct EnumValueDescriptorProto<'a> {
    bytes: &'a [u8],
}

impl<'a> EnumValueDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const NUMBER: u32 = 2;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn name(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::NAME)
    }

    pub fn number(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::NUMBER)
    }
}

/// `google.protobuf.FileOptions` wire view (subset).
#[derive(Debug, Clone, Copy)]
pub struct FileOptions<'a> {
    bytes: &'a [u8],
}

impl<'a> FileOptions<'a> {
    pub(crate) const FEATURES: u32 = 50;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn features(self) -> Result<Option<FeatureSet<'a>>> {
        Ok(raw::last_len(self.bytes, Self::FEATURES)?.map(FeatureSet::new))
    }
}

/// `google.protobuf.MessageOptions` wire view (subset + puroro extensions).
#[derive(Debug, Clone, Copy)]
pub struct MessageOptions<'a> {
    bytes: &'a [u8],
}

impl<'a> MessageOptions<'a> {
    pub(crate) const MAP_ENTRY: u32 = 7;
    pub(crate) const FEATURES: u32 = 12;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn map_entry(self) -> Result<Option<bool>> {
        raw::last_bool(self.bytes, Self::MAP_ENTRY)
    }

    pub fn features(self) -> Result<Option<FeatureSet<'a>>> {
        Ok(raw::last_len(self.bytes, Self::FEATURES)?.map(FeatureSet::new))
    }

    /// Custom option enum at `field_number` (last-wins).
    pub fn extension_enum(self, field_number: u32) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, field_number)
    }
}

/// `google.protobuf.FieldOptions` wire view (subset + puroro extensions).
#[derive(Debug, Clone, Copy)]
pub struct FieldOptions<'a> {
    bytes: &'a [u8],
}

impl<'a> FieldOptions<'a> {
    pub(crate) const PACKED: u32 = 2;
    pub(crate) const FEATURES: u32 = 21;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn packed(self) -> Result<Option<bool>> {
        raw::last_bool(self.bytes, Self::PACKED)
    }

    pub fn features(self) -> Result<Option<FeatureSet<'a>>> {
        Ok(raw::last_len(self.bytes, Self::FEATURES)?.map(FeatureSet::new))
    }

    /// Custom option enum at `field_number` (last-wins).
    pub fn extension_enum(self, field_number: u32) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, field_number)
    }
}

/// `google.protobuf.EnumOptions` wire view (subset + puroro extensions).
#[derive(Debug, Clone, Copy)]
pub struct EnumOptions<'a> {
    bytes: &'a [u8],
}

impl<'a> EnumOptions<'a> {
    pub(crate) const FEATURES: u32 = 7;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn features(self) -> Result<Option<FeatureSet<'a>>> {
        Ok(raw::last_len(self.bytes, Self::FEATURES)?.map(FeatureSet::new))
    }

    /// Custom option enum at `field_number` (last-wins).
    pub fn extension_enum(self, field_number: u32) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, field_number)
    }
}

/// `google.protobuf.FeatureSet` wire view.
#[derive(Debug, Clone, Copy)]
pub struct FeatureSet<'a> {
    bytes: &'a [u8],
}

impl<'a> FeatureSet<'a> {
    pub(crate) const FIELD_PRESENCE: u32 = 1;
    pub(crate) const ENUM_TYPE: u32 = 2;
    pub(crate) const REPEATED_FIELD_ENCODING: u32 = 3;
    pub(crate) const UTF8_VALIDATION: u32 = 4;
    pub(crate) const MESSAGE_ENCODING: u32 = 5;
    pub(crate) const JSON_FORMAT: u32 = 6;
    pub(crate) const ENFORCE_NAMING_STYLE: u32 = 7;
    pub(crate) const DEFAULT_SYMBOL_VISIBILITY: u32 = 8;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn field_presence(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::FIELD_PRESENCE)
    }

    pub fn enum_type(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::ENUM_TYPE)
    }

    pub fn repeated_field_encoding(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::REPEATED_FIELD_ENCODING)
    }

    pub fn utf8_validation(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::UTF8_VALIDATION)
    }

    pub fn message_encoding(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::MESSAGE_ENCODING)
    }

    pub fn json_format(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::JSON_FORMAT)
    }

    pub fn enforce_naming_style(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::ENFORCE_NAMING_STYLE)
    }

    pub fn default_symbol_visibility(self) -> Result<Option<i32>> {
        raw::last_i32(self.bytes, Self::DEFAULT_SYMBOL_VISIBILITY)
    }
}

/// `google.protobuf.SourceCodeInfo` wire view.
#[derive(Debug, Clone, Copy)]
pub struct SourceCodeInfo<'a> {
    bytes: &'a [u8],
}

impl<'a> SourceCodeInfo<'a> {
    pub(crate) const LOCATION: u32 = 1;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn location(self) -> Result<Vec<SourceCodeInfoLocation<'a>>> {
        raw::repeated_lens(self.bytes, Self::LOCATION).map(|bodies| {
            bodies
                .into_iter()
                .map(SourceCodeInfoLocation::new)
                .collect()
        })
    }
}

/// `google.protobuf.SourceCodeInfo.Location` wire view.
#[derive(Debug, Clone, Copy)]
pub struct SourceCodeInfoLocation<'a> {
    bytes: &'a [u8],
}

impl<'a> SourceCodeInfoLocation<'a> {
    pub(crate) const PATH: u32 = 1;
    pub(crate) const LEADING_COMMENTS: u32 = 3;
    pub(crate) const TRAILING_COMMENTS: u32 = 4;

    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub fn path(self) -> Result<Vec<i32>> {
        raw::repeated_i32s(self.bytes, Self::PATH)
    }

    pub fn leading_comments(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::LEADING_COMMENTS)
    }

    pub fn trailing_comments(self) -> Result<Option<&'a str>> {
        raw::last_string(self.bytes, Self::TRAILING_COMMENTS)
    }
}
