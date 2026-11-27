//! Parsed wire views over `descriptor.proto` / `plugin.proto` message bytes.
//!
//! [`parse()`](CodeGeneratorRequest::parse) walks each message body once and
//! mutates a default-initialized open struct. Nested LEN messages are parsed
//! recursively. Wire field numbers are written as literals next to the field
//! they fill. This is not `puroro-types` generated code (bootstrap cycle).
//!
//! Singular fields are last-wins. Wrong wire types for a requested field are
//! errors; other fields are ignored.

use crate::error::{Error, Result};
use ::protobuf_core::{AsRefExtProtobuf, Field, FieldNumber, FieldValue, ReadExtVarint};
use ::std::str;

fn while_decoding<T>(message: &'static str, f: impl FnOnce() -> Result<T>) -> Result<T> {
    f().map_err(|e| e.while_decoding(message))
}

fn expect_len<'a>(field: &Field<&'a [u8]>) -> Result<&'a [u8]> {
    match &field.value {
        FieldValue::Len(bytes) => Ok(*bytes),
        _ => Err(Error::unexpected_field(field.field_number.as_u32())),
    }
}

fn expect_str<'a>(field: &Field<&'a [u8]>) -> Result<&'a str> {
    Ok(str::from_utf8(expect_len(field)?)?)
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

fn extend_i32s(out: &mut Vec<i32>, field: &Field<&[u8]>) -> Result<()> {
    match &field.value {
        FieldValue::Len(payload) => {
            let mut slice: &[u8] = payload;
            while !slice.is_empty() {
                let varint = slice.read_varint()?;
                out.push(varint.try_to_int32()?);
            }
            Ok(())
        }
        FieldValue::Varint(v) => {
            out.push(v.try_to_int32()?);
            Ok(())
        }
        _ => Err(Error::unexpected_field(field.field_number.as_u32())),
    }
}

fn record_extension_enum(out: &mut Vec<(FieldNumber, i32)>, number: FieldNumber, value: i32) {
    if let Some((_, slot)) = out.iter_mut().rev().find(|(n, _)| *n == number) {
        *slot = value;
    } else {
        out.push((number, value));
    }
}

/// `google.protobuf.compiler.CodeGeneratorRequest` parsed view.
#[derive(Debug, Clone, Default)]
pub struct CodeGeneratorRequest<'a> {
    pub file_to_generate: Vec<&'a str>,
    pub parameter: Option<&'a str>,
    pub proto_file: Vec<FileDescriptorProto<'a>>,
}

impl<'a> CodeGeneratorRequest<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("CodeGeneratorRequest", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    1 => out.file_to_generate.push(expect_str(&field)?),
                    2 => out.parameter = Some(expect_str(&field)?),
                    15 => out
                        .proto_file
                        .push(FileDescriptorProto::parse(expect_len(&field)?)?),
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.FileDescriptorProto` parsed view.
#[derive(Debug, Clone, Default)]
pub struct FileDescriptorProto<'a> {
    /// Original message body bytes (for `FileDescriptorSet` re-encode).
    pub bytes: &'a [u8],
    pub name: Option<&'a str>,
    pub package: Option<&'a str>,
    pub dependency: Vec<&'a str>,
    pub message_type: Vec<DescriptorProto<'a>>,
    pub enum_type: Vec<EnumDescriptorProto<'a>>,
    pub options: Option<FileOptions>,
    pub source_code_info: Option<SourceCodeInfo<'a>>,
    pub syntax: Option<&'a str>,
    pub edition: Option<i32>,
}

impl<'a> FileDescriptorProto<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("FileDescriptorProto", || {
            let mut out = Self {
                bytes,
                ..Self::default()
            };
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    1 => out.name = Some(expect_str(&field)?),
                    2 => out.package = Some(expect_str(&field)?),
                    3 => out.dependency.push(expect_str(&field)?),
                    4 => out
                        .message_type
                        .push(DescriptorProto::parse(expect_len(&field)?)?),
                    5 => out
                        .enum_type
                        .push(EnumDescriptorProto::parse(expect_len(&field)?)?),
                    8 => out.options = Some(FileOptions::parse(expect_len(&field)?)?),
                    9 => out.source_code_info = Some(SourceCodeInfo::parse(expect_len(&field)?)?),
                    12 => out.syntax = Some(expect_str(&field)?),
                    14 => out.edition = Some(expect_i32(&field)?),
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.DescriptorProto` parsed view.
#[derive(Debug, Clone, Default)]
pub struct DescriptorProto<'a> {
    pub name: Option<&'a str>,
    pub field: Vec<FieldDescriptorProto<'a>>,
    pub nested_type: Vec<DescriptorProto<'a>>,
    pub enum_type: Vec<EnumDescriptorProto<'a>>,
    pub options: Option<MessageOptions>,
    pub oneof_decl: Vec<OneofDescriptorProto<'a>>,
}

impl<'a> DescriptorProto<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("DescriptorProto", || {
            let mut out = Self::default();
            for f in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let f = f?;
                match f.field_number.as_u32() {
                    1 => out.name = Some(expect_str(&f)?),
                    2 => out
                        .field
                        .push(FieldDescriptorProto::parse(expect_len(&f)?)?),
                    3 => out
                        .nested_type
                        .push(DescriptorProto::parse(expect_len(&f)?)?),
                    4 => out
                        .enum_type
                        .push(EnumDescriptorProto::parse(expect_len(&f)?)?),
                    7 => out.options = Some(MessageOptions::parse(expect_len(&f)?)?),
                    8 => out
                        .oneof_decl
                        .push(OneofDescriptorProto::parse(expect_len(&f)?)?),
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.FieldDescriptorProto` parsed view.
#[derive(Debug, Clone, Default)]
pub struct FieldDescriptorProto<'a> {
    pub name: Option<&'a str>,
    pub number: Option<i32>,
    pub label: Option<i32>,
    pub type_: Option<i32>,
    pub type_name: Option<&'a str>,
    pub default_value: Option<&'a str>,
    pub options: Option<FieldOptions>,
    pub oneof_index: Option<i32>,
    pub proto3_optional: Option<bool>,
}

impl<'a> FieldDescriptorProto<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("FieldDescriptorProto", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    1 => out.name = Some(expect_str(&field)?),
                    3 => out.number = Some(expect_i32(&field)?),
                    4 => out.label = Some(expect_i32(&field)?),
                    5 => out.type_ = Some(expect_i32(&field)?),
                    6 => out.type_name = Some(expect_str(&field)?),
                    7 => out.default_value = Some(expect_str(&field)?),
                    8 => out.options = Some(FieldOptions::parse(expect_len(&field)?)?),
                    9 => out.oneof_index = Some(expect_i32(&field)?),
                    17 => out.proto3_optional = Some(expect_bool(&field)?),
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.OneofDescriptorProto` parsed view.
#[derive(Debug, Clone, Default)]
pub struct OneofDescriptorProto<'a> {
    pub name: Option<&'a str>,
}

impl<'a> OneofDescriptorProto<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("OneofDescriptorProto", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                if field.field_number.as_u32() == 1 {
                    out.name = Some(expect_str(&field)?);
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.EnumDescriptorProto` parsed view.
#[derive(Debug, Clone, Default)]
pub struct EnumDescriptorProto<'a> {
    pub name: Option<&'a str>,
    pub value: Vec<EnumValueDescriptorProto<'a>>,
    pub options: Option<EnumOptions>,
}

impl<'a> EnumDescriptorProto<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("EnumDescriptorProto", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    1 => out.name = Some(expect_str(&field)?),
                    2 => out
                        .value
                        .push(EnumValueDescriptorProto::parse(expect_len(&field)?)?),
                    3 => out.options = Some(EnumOptions::parse(expect_len(&field)?)?),
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.EnumValueDescriptorProto` parsed view.
#[derive(Debug, Clone, Default)]
pub struct EnumValueDescriptorProto<'a> {
    pub name: Option<&'a str>,
    pub number: Option<i32>,
}

impl<'a> EnumValueDescriptorProto<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("EnumValueDescriptorProto", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    1 => out.name = Some(expect_str(&field)?),
                    2 => out.number = Some(expect_i32(&field)?),
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.FileOptions` parsed view (subset).
#[derive(Debug, Clone, Default)]
pub struct FileOptions {
    pub features: Option<FeatureSet>,
}

impl FileOptions {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("FileOptions", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                if field.field_number.as_u32() == 50 {
                    out.features = Some(FeatureSet::parse(expect_len(&field)?)?);
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.MessageOptions` parsed view (subset + custom enum extensions).
#[derive(Debug, Clone, Default)]
pub struct MessageOptions {
    pub map_entry: Option<bool>,
    pub features: Option<FeatureSet>,
    /// Last-wins varint extensions keyed by field number.
    pub extension_enums: Vec<(FieldNumber, i32)>,
}

impl MessageOptions {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("MessageOptions", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    7 => out.map_entry = Some(expect_bool(&field)?),
                    12 => out.features = Some(FeatureSet::parse(expect_len(&field)?)?),
                    _ => {
                        if let FieldValue::Varint(_) = &field.value {
                            record_extension_enum(
                                &mut out.extension_enums,
                                field.field_number,
                                expect_i32(&field)?,
                            );
                        }
                    }
                }
            }
            Ok(out)
        })
    }

    pub fn extension_enum(&self, field_number: u32) -> Option<i32> {
        self.extension_enums
            .iter()
            .rev()
            .find(|(n, _)| n.as_u32() == field_number)
            .map(|(_, v)| *v)
    }
}

/// `google.protobuf.FieldOptions` parsed view (subset + custom enum extensions).
#[derive(Debug, Clone, Default)]
pub struct FieldOptions {
    pub packed: Option<bool>,
    pub features: Option<FeatureSet>,
    /// Last-wins varint extensions keyed by field number.
    pub extension_enums: Vec<(FieldNumber, i32)>,
}

impl FieldOptions {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("FieldOptions", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    2 => out.packed = Some(expect_bool(&field)?),
                    21 => out.features = Some(FeatureSet::parse(expect_len(&field)?)?),
                    _ => {
                        if let FieldValue::Varint(_) = &field.value {
                            record_extension_enum(
                                &mut out.extension_enums,
                                field.field_number,
                                expect_i32(&field)?,
                            );
                        }
                    }
                }
            }
            Ok(out)
        })
    }

    pub fn extension_enum(&self, field_number: u32) -> Option<i32> {
        self.extension_enums
            .iter()
            .rev()
            .find(|(n, _)| n.as_u32() == field_number)
            .map(|(_, v)| *v)
    }
}

/// `google.protobuf.EnumOptions` parsed view (subset + custom enum extensions).
#[derive(Debug, Clone, Default)]
pub struct EnumOptions {
    pub features: Option<FeatureSet>,
    /// Last-wins varint extensions keyed by field number.
    pub extension_enums: Vec<(FieldNumber, i32)>,
}

impl EnumOptions {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("EnumOptions", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    7 => out.features = Some(FeatureSet::parse(expect_len(&field)?)?),
                    _ => {
                        if let FieldValue::Varint(_) = &field.value {
                            record_extension_enum(
                                &mut out.extension_enums,
                                field.field_number,
                                expect_i32(&field)?,
                            );
                        }
                    }
                }
            }
            Ok(out)
        })
    }

    pub fn extension_enum(&self, field_number: u32) -> Option<i32> {
        self.extension_enums
            .iter()
            .rev()
            .find(|(n, _)| n.as_u32() == field_number)
            .map(|(_, v)| *v)
    }
}

/// `google.protobuf.FeatureSet` parsed view (owned scalars; no borrow).
#[derive(Debug, Clone, Default)]
pub struct FeatureSet {
    pub field_presence: Option<i32>,
    pub enum_type: Option<i32>,
    pub repeated_field_encoding: Option<i32>,
    pub utf8_validation: Option<i32>,
    pub message_encoding: Option<i32>,
    pub json_format: Option<i32>,
    pub enforce_naming_style: Option<i32>,
    pub default_symbol_visibility: Option<i32>,
}

impl FeatureSet {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("FeatureSet", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    1 => out.field_presence = Some(expect_i32(&field)?),
                    2 => out.enum_type = Some(expect_i32(&field)?),
                    3 => out.repeated_field_encoding = Some(expect_i32(&field)?),
                    4 => out.utf8_validation = Some(expect_i32(&field)?),
                    5 => out.message_encoding = Some(expect_i32(&field)?),
                    6 => out.json_format = Some(expect_i32(&field)?),
                    7 => out.enforce_naming_style = Some(expect_i32(&field)?),
                    8 => out.default_symbol_visibility = Some(expect_i32(&field)?),
                    // Language-specific FeatureSet extensions (e.g. 1000+) are LEN.
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.SourceCodeInfo` parsed view.
#[derive(Debug, Clone, Default)]
pub struct SourceCodeInfo<'a> {
    pub location: Vec<SourceCodeInfoLocation<'a>>,
}

impl<'a> SourceCodeInfo<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("SourceCodeInfo", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                if field.field_number.as_u32() == 1 {
                    out.location
                        .push(SourceCodeInfoLocation::parse(expect_len(&field)?)?);
                }
            }
            Ok(out)
        })
    }
}

/// `google.protobuf.SourceCodeInfo.Location` parsed view.
#[derive(Debug, Clone, Default)]
pub struct SourceCodeInfoLocation<'a> {
    pub path: Vec<i32>,
    pub leading_comments: Option<&'a str>,
    pub trailing_comments: Option<&'a str>,
}

impl<'a> SourceCodeInfoLocation<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("SourceCodeInfo.Location", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    1 => extend_i32s(&mut out.path, &field)?,
                    3 => out.leading_comments = Some(expect_str(&field)?),
                    4 => out.trailing_comments = Some(expect_str(&field)?),
                    _ => {}
                }
            }
            Ok(out)
        })
    }
}
