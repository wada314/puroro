//! Parsed wire views over `descriptor.proto` / `plugin.proto` message bytes.
//!
//! [`parse()`](CodeGeneratorRequest::parse) walks each message body once, stores
//! the fields this plugin needs, then getters return those values. Nested LEN
//! messages are parsed recursively. This is not `puroro-types` generated code
//! (bootstrap cycle).
//!
//! Singular fields are last-wins. Wrong wire types for a requested field are
//! errors; other fields are ignored.

use crate::error::{Error, Result};
use ::protobuf_core::{AsRefExtProtobuf, Field, FieldValue, ReadExtVarint};
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

fn record_extension_enum(out: &mut Vec<(u32, i32)>, number: u32, value: i32) {
    if let Some((_, slot)) = out.iter_mut().rev().find(|(n, _)| *n == number) {
        *slot = value;
    } else {
        out.push((number, value));
    }
}

fn extension_enum_get(stored: &[(u32, i32)], field_number: u32) -> Option<i32> {
    stored
        .iter()
        .rev()
        .find(|(n, _)| *n == field_number)
        .map(|(_, v)| *v)
}

/// `google.protobuf.FileDescriptorSet` wire numbers (encode helper).
#[derive(Debug, Clone, Copy)]
pub struct FileDescriptorSet;

impl FileDescriptorSet {
    /// `repeated FileDescriptorProto file = 1;`
    pub(crate) const FILE: u32 = 1;
}

/// `google.protobuf.compiler.CodeGeneratorRequest` parsed view.
#[derive(Debug, Clone)]
pub struct CodeGeneratorRequest<'a> {
    file_to_generate: Vec<&'a str>,
    parameter: Option<&'a str>,
    proto_file: Vec<FileDescriptorProto<'a>>,
}

impl<'a> CodeGeneratorRequest<'a> {
    pub(crate) const FILE_TO_GENERATE: u32 = 1;
    pub(crate) const PARAMETER: u32 = 2;
    pub(crate) const PROTO_FILE: u32 = 15;

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("CodeGeneratorRequest", || {
            let mut file_to_generate = Vec::new();
            let mut parameter = None;
            let mut proto_file = Vec::new();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::FILE_TO_GENERATE => file_to_generate.push(expect_str(&field)?),
                    Self::PARAMETER => parameter = Some(expect_str(&field)?),
                    Self::PROTO_FILE => {
                        proto_file.push(FileDescriptorProto::parse(expect_len(&field)?)?)
                    }
                    _ => {}
                }
            }
            Ok(Self {
                file_to_generate,
                parameter,
                proto_file,
            })
        })
    }

    pub fn file_to_generate(&self) -> &[&'a str] {
        &self.file_to_generate
    }

    pub fn parameter(&self) -> Option<&'a str> {
        self.parameter
    }

    pub fn proto_file(&self) -> &[FileDescriptorProto<'a>] {
        &self.proto_file
    }
}

/// `google.protobuf.FileDescriptorProto` parsed view.
#[derive(Debug, Clone)]
pub struct FileDescriptorProto<'a> {
    bytes: &'a [u8],
    name: Option<&'a str>,
    package: Option<&'a str>,
    dependency: Vec<&'a str>,
    message_type: Vec<DescriptorProto<'a>>,
    enum_type: Vec<EnumDescriptorProto<'a>>,
    options: Option<FileOptions>,
    source_code_info: Option<SourceCodeInfo<'a>>,
    syntax: Option<&'a str>,
    edition: Option<i32>,
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

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("FileDescriptorProto", || {
            let mut name = None;
            let mut package = None;
            let mut dependency = Vec::new();
            let mut message_type = Vec::new();
            let mut enum_type = Vec::new();
            let mut options = None;
            let mut source_code_info = None;
            let mut syntax = None;
            let mut edition = None;
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::NAME => name = Some(expect_str(&field)?),
                    Self::PACKAGE => package = Some(expect_str(&field)?),
                    Self::DEPENDENCY => dependency.push(expect_str(&field)?),
                    Self::MESSAGE_TYPE => {
                        message_type.push(DescriptorProto::parse(expect_len(&field)?)?)
                    }
                    Self::ENUM_TYPE => {
                        enum_type.push(EnumDescriptorProto::parse(expect_len(&field)?)?)
                    }
                    Self::OPTIONS => options = Some(FileOptions::parse(expect_len(&field)?)?),
                    Self::SOURCE_CODE_INFO => {
                        source_code_info = Some(SourceCodeInfo::parse(expect_len(&field)?)?)
                    }
                    Self::SYNTAX => syntax = Some(expect_str(&field)?),
                    Self::EDITION => edition = Some(expect_i32(&field)?),
                    _ => {}
                }
            }
            Ok(Self {
                bytes,
                name,
                package,
                dependency,
                message_type,
                enum_type,
                options,
                source_code_info,
                syntax,
                edition,
            })
        })
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn package(&self) -> Option<&'a str> {
        self.package
    }

    pub fn dependency(&self) -> &[&'a str] {
        &self.dependency
    }

    pub fn message_type(&self) -> &[DescriptorProto<'a>] {
        &self.message_type
    }

    pub fn enum_type(&self) -> &[EnumDescriptorProto<'a>] {
        &self.enum_type
    }

    pub fn options(&self) -> Option<&FileOptions> {
        self.options.as_ref()
    }

    pub fn source_code_info(&self) -> Option<&SourceCodeInfo<'a>> {
        self.source_code_info.as_ref()
    }

    pub fn syntax(&self) -> Option<&'a str> {
        self.syntax
    }

    pub fn edition(&self) -> Option<i32> {
        self.edition
    }
}

/// `google.protobuf.DescriptorProto` parsed view.
#[derive(Debug, Clone)]
pub struct DescriptorProto<'a> {
    name: Option<&'a str>,
    field: Vec<FieldDescriptorProto<'a>>,
    nested_type: Vec<DescriptorProto<'a>>,
    enum_type: Vec<EnumDescriptorProto<'a>>,
    options: Option<MessageOptions>,
    oneof_decl: Vec<OneofDescriptorProto<'a>>,
}

impl<'a> DescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const FIELD: u32 = 2;
    pub(crate) const NESTED_TYPE: u32 = 3;
    pub(crate) const ENUM_TYPE: u32 = 4;
    pub(crate) const OPTIONS: u32 = 7;
    pub(crate) const ONEOF_DECL: u32 = 8;

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("DescriptorProto", || {
            let mut name = None;
            let mut field = Vec::new();
            let mut nested_type = Vec::new();
            let mut enum_type = Vec::new();
            let mut options = None;
            let mut oneof_decl = Vec::new();
            for f in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let f = f?;
                match f.field_number.as_u32() {
                    Self::NAME => name = Some(expect_str(&f)?),
                    Self::FIELD => field.push(FieldDescriptorProto::parse(expect_len(&f)?)?),
                    Self::NESTED_TYPE => nested_type.push(DescriptorProto::parse(expect_len(&f)?)?),
                    Self::ENUM_TYPE => enum_type.push(EnumDescriptorProto::parse(expect_len(&f)?)?),
                    Self::OPTIONS => options = Some(MessageOptions::parse(expect_len(&f)?)?),
                    Self::ONEOF_DECL => {
                        oneof_decl.push(OneofDescriptorProto::parse(expect_len(&f)?)?)
                    }
                    _ => {}
                }
            }
            Ok(Self {
                name,
                field,
                nested_type,
                enum_type,
                options,
                oneof_decl,
            })
        })
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn field(&self) -> &[FieldDescriptorProto<'a>] {
        &self.field
    }

    pub fn nested_type(&self) -> &[DescriptorProto<'a>] {
        &self.nested_type
    }

    pub fn enum_type(&self) -> &[EnumDescriptorProto<'a>] {
        &self.enum_type
    }

    pub fn options(&self) -> Option<&MessageOptions> {
        self.options.as_ref()
    }

    pub fn oneof_decl(&self) -> &[OneofDescriptorProto<'a>] {
        &self.oneof_decl
    }
}

/// `google.protobuf.FieldDescriptorProto` parsed view.
#[derive(Debug, Clone)]
pub struct FieldDescriptorProto<'a> {
    name: Option<&'a str>,
    number: Option<i32>,
    label: Option<i32>,
    type_: Option<i32>,
    type_name: Option<&'a str>,
    default_value: Option<&'a str>,
    options: Option<FieldOptions>,
    oneof_index: Option<i32>,
    proto3_optional: Option<bool>,
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

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("FieldDescriptorProto", || {
            let mut name = None;
            let mut number = None;
            let mut label = None;
            let mut type_ = None;
            let mut type_name = None;
            let mut default_value = None;
            let mut options = None;
            let mut oneof_index = None;
            let mut proto3_optional = None;
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::NAME => name = Some(expect_str(&field)?),
                    Self::NUMBER => number = Some(expect_i32(&field)?),
                    Self::LABEL => label = Some(expect_i32(&field)?),
                    Self::TYPE => type_ = Some(expect_i32(&field)?),
                    Self::TYPE_NAME => type_name = Some(expect_str(&field)?),
                    Self::DEFAULT_VALUE => default_value = Some(expect_str(&field)?),
                    Self::OPTIONS => options = Some(FieldOptions::parse(expect_len(&field)?)?),
                    Self::ONEOF_INDEX => oneof_index = Some(expect_i32(&field)?),
                    Self::PROTO3_OPTIONAL => proto3_optional = Some(expect_bool(&field)?),
                    _ => {}
                }
            }
            Ok(Self {
                name,
                number,
                label,
                type_,
                type_name,
                default_value,
                options,
                oneof_index,
                proto3_optional,
            })
        })
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn number(&self) -> Option<i32> {
        self.number
    }

    pub fn label(&self) -> Option<i32> {
        self.label
    }

    pub fn type_(&self) -> Option<i32> {
        self.type_
    }

    pub fn type_name(&self) -> Option<&'a str> {
        self.type_name
    }

    pub fn default_value(&self) -> Option<&'a str> {
        self.default_value
    }

    pub fn options(&self) -> Option<&FieldOptions> {
        self.options.as_ref()
    }

    pub fn oneof_index(&self) -> Option<i32> {
        self.oneof_index
    }

    pub fn proto3_optional(&self) -> Option<bool> {
        self.proto3_optional
    }
}

/// `google.protobuf.OneofDescriptorProto` parsed view.
#[derive(Debug, Clone)]
pub struct OneofDescriptorProto<'a> {
    name: Option<&'a str>,
}

impl<'a> OneofDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("OneofDescriptorProto", || {
            let mut name = None;
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                if field.field_number.as_u32() == Self::NAME {
                    name = Some(expect_str(&field)?);
                }
            }
            Ok(Self { name })
        })
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }
}

/// `google.protobuf.EnumDescriptorProto` parsed view.
#[derive(Debug, Clone)]
pub struct EnumDescriptorProto<'a> {
    name: Option<&'a str>,
    value: Vec<EnumValueDescriptorProto<'a>>,
    options: Option<EnumOptions>,
}

impl<'a> EnumDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const VALUE: u32 = 2;
    pub(crate) const OPTIONS: u32 = 3;

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("EnumDescriptorProto", || {
            let mut name = None;
            let mut value = Vec::new();
            let mut options = None;
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::NAME => name = Some(expect_str(&field)?),
                    Self::VALUE => {
                        value.push(EnumValueDescriptorProto::parse(expect_len(&field)?)?)
                    }
                    Self::OPTIONS => options = Some(EnumOptions::parse(expect_len(&field)?)?),
                    _ => {}
                }
            }
            Ok(Self {
                name,
                value,
                options,
            })
        })
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn value(&self) -> &[EnumValueDescriptorProto<'a>] {
        &self.value
    }

    pub fn options(&self) -> Option<&EnumOptions> {
        self.options.as_ref()
    }
}

/// `google.protobuf.EnumValueDescriptorProto` parsed view.
#[derive(Debug, Clone)]
pub struct EnumValueDescriptorProto<'a> {
    name: Option<&'a str>,
    number: Option<i32>,
}

impl<'a> EnumValueDescriptorProto<'a> {
    pub(crate) const NAME: u32 = 1;
    pub(crate) const NUMBER: u32 = 2;

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("EnumValueDescriptorProto", || {
            let mut name = None;
            let mut number = None;
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::NAME => name = Some(expect_str(&field)?),
                    Self::NUMBER => number = Some(expect_i32(&field)?),
                    _ => {}
                }
            }
            Ok(Self { name, number })
        })
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn number(&self) -> Option<i32> {
        self.number
    }
}

/// `google.protobuf.FileOptions` parsed view (subset).
#[derive(Debug, Clone)]
pub struct FileOptions {
    features: Option<FeatureSet>,
}

impl FileOptions {
    pub(crate) const FEATURES: u32 = 50;

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("FileOptions", || {
            let mut features = None;
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                if field.field_number.as_u32() == Self::FEATURES {
                    features = Some(FeatureSet::parse(expect_len(&field)?)?);
                }
            }
            Ok(Self { features })
        })
    }

    pub fn features(&self) -> Option<&FeatureSet> {
        self.features.as_ref()
    }
}

/// `google.protobuf.MessageOptions` parsed view (subset + custom enum extensions).
#[derive(Debug, Clone)]
pub struct MessageOptions {
    map_entry: Option<bool>,
    features: Option<FeatureSet>,
    extension_enums: Vec<(u32, i32)>,
}

impl MessageOptions {
    pub(crate) const MAP_ENTRY: u32 = 7;
    pub(crate) const FEATURES: u32 = 12;

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("MessageOptions", || {
            let mut map_entry = None;
            let mut features = None;
            let mut extension_enums = Vec::new();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::MAP_ENTRY => map_entry = Some(expect_bool(&field)?),
                    Self::FEATURES => features = Some(FeatureSet::parse(expect_len(&field)?)?),
                    n => {
                        if let FieldValue::Varint(_) = &field.value {
                            record_extension_enum(&mut extension_enums, n, expect_i32(&field)?);
                        }
                    }
                }
            }
            Ok(Self {
                map_entry,
                features,
                extension_enums,
            })
        })
    }

    pub fn map_entry(&self) -> Option<bool> {
        self.map_entry
    }

    pub fn features(&self) -> Option<&FeatureSet> {
        self.features.as_ref()
    }

    pub fn extension_enum(&self, field_number: u32) -> Option<i32> {
        extension_enum_get(&self.extension_enums, field_number)
    }
}

/// `google.protobuf.FieldOptions` parsed view (subset + custom enum extensions).
#[derive(Debug, Clone)]
pub struct FieldOptions {
    packed: Option<bool>,
    features: Option<FeatureSet>,
    extension_enums: Vec<(u32, i32)>,
}

impl FieldOptions {
    pub(crate) const PACKED: u32 = 2;
    pub(crate) const FEATURES: u32 = 21;

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("FieldOptions", || {
            let mut packed = None;
            let mut features = None;
            let mut extension_enums = Vec::new();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::PACKED => packed = Some(expect_bool(&field)?),
                    Self::FEATURES => features = Some(FeatureSet::parse(expect_len(&field)?)?),
                    n => {
                        if let FieldValue::Varint(_) = &field.value {
                            record_extension_enum(&mut extension_enums, n, expect_i32(&field)?);
                        }
                    }
                }
            }
            Ok(Self {
                packed,
                features,
                extension_enums,
            })
        })
    }

    pub fn packed(&self) -> Option<bool> {
        self.packed
    }

    pub fn features(&self) -> Option<&FeatureSet> {
        self.features.as_ref()
    }

    pub fn extension_enum(&self, field_number: u32) -> Option<i32> {
        extension_enum_get(&self.extension_enums, field_number)
    }
}

/// `google.protobuf.EnumOptions` parsed view (subset + custom enum extensions).
#[derive(Debug, Clone)]
pub struct EnumOptions {
    features: Option<FeatureSet>,
    extension_enums: Vec<(u32, i32)>,
}

impl EnumOptions {
    pub(crate) const FEATURES: u32 = 7;

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("EnumOptions", || {
            let mut features = None;
            let mut extension_enums = Vec::new();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::FEATURES => features = Some(FeatureSet::parse(expect_len(&field)?)?),
                    n => {
                        if let FieldValue::Varint(_) = &field.value {
                            record_extension_enum(&mut extension_enums, n, expect_i32(&field)?);
                        }
                    }
                }
            }
            Ok(Self {
                features,
                extension_enums,
            })
        })
    }

    pub fn features(&self) -> Option<&FeatureSet> {
        self.features.as_ref()
    }

    pub fn extension_enum(&self, field_number: u32) -> Option<i32> {
        extension_enum_get(&self.extension_enums, field_number)
    }
}

/// `google.protobuf.FeatureSet` parsed view (owned scalars; no borrow).
#[derive(Debug, Clone, Default)]
pub struct FeatureSet {
    field_presence: Option<i32>,
    enum_type: Option<i32>,
    repeated_field_encoding: Option<i32>,
    utf8_validation: Option<i32>,
    message_encoding: Option<i32>,
    json_format: Option<i32>,
    enforce_naming_style: Option<i32>,
    default_symbol_visibility: Option<i32>,
}

impl FeatureSet {
    pub(crate) const FIELD_PRESENCE: u32 = 1;
    pub(crate) const ENUM_TYPE: u32 = 2;
    pub(crate) const REPEATED_FIELD_ENCODING: u32 = 3;
    pub(crate) const UTF8_VALIDATION: u32 = 4;
    pub(crate) const MESSAGE_ENCODING: u32 = 5;
    pub(crate) const JSON_FORMAT: u32 = 6;
    pub(crate) const ENFORCE_NAMING_STYLE: u32 = 7;
    pub(crate) const DEFAULT_SYMBOL_VISIBILITY: u32 = 8;

    pub fn parse(bytes: &[u8]) -> Result<Self> {
        while_decoding("FeatureSet", || {
            let mut out = Self::default();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::FIELD_PRESENCE => out.field_presence = Some(expect_i32(&field)?),
                    Self::ENUM_TYPE => out.enum_type = Some(expect_i32(&field)?),
                    Self::REPEATED_FIELD_ENCODING => {
                        out.repeated_field_encoding = Some(expect_i32(&field)?)
                    }
                    Self::UTF8_VALIDATION => out.utf8_validation = Some(expect_i32(&field)?),
                    Self::MESSAGE_ENCODING => out.message_encoding = Some(expect_i32(&field)?),
                    Self::JSON_FORMAT => out.json_format = Some(expect_i32(&field)?),
                    Self::ENFORCE_NAMING_STYLE => {
                        out.enforce_naming_style = Some(expect_i32(&field)?)
                    }
                    Self::DEFAULT_SYMBOL_VISIBILITY => {
                        out.default_symbol_visibility = Some(expect_i32(&field)?)
                    }
                    // Language-specific FeatureSet extensions (e.g. 1000+) are LEN.
                    _ => {}
                }
            }
            Ok(out)
        })
    }

    pub fn field_presence(&self) -> Option<i32> {
        self.field_presence
    }

    pub fn enum_type(&self) -> Option<i32> {
        self.enum_type
    }

    pub fn repeated_field_encoding(&self) -> Option<i32> {
        self.repeated_field_encoding
    }

    pub fn utf8_validation(&self) -> Option<i32> {
        self.utf8_validation
    }

    pub fn message_encoding(&self) -> Option<i32> {
        self.message_encoding
    }

    pub fn json_format(&self) -> Option<i32> {
        self.json_format
    }

    pub fn enforce_naming_style(&self) -> Option<i32> {
        self.enforce_naming_style
    }

    pub fn default_symbol_visibility(&self) -> Option<i32> {
        self.default_symbol_visibility
    }
}

/// `google.protobuf.SourceCodeInfo` parsed view.
#[derive(Debug, Clone)]
pub struct SourceCodeInfo<'a> {
    location: Vec<SourceCodeInfoLocation<'a>>,
}

impl<'a> SourceCodeInfo<'a> {
    pub(crate) const LOCATION: u32 = 1;

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("SourceCodeInfo", || {
            let mut location = Vec::new();
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                if field.field_number.as_u32() == Self::LOCATION {
                    location.push(SourceCodeInfoLocation::parse(expect_len(&field)?)?);
                }
            }
            Ok(Self { location })
        })
    }

    pub fn location(&self) -> &[SourceCodeInfoLocation<'a>] {
        &self.location
    }
}

/// `google.protobuf.SourceCodeInfo.Location` parsed view.
#[derive(Debug, Clone)]
pub struct SourceCodeInfoLocation<'a> {
    path: Vec<i32>,
    leading_comments: Option<&'a str>,
    trailing_comments: Option<&'a str>,
}

impl<'a> SourceCodeInfoLocation<'a> {
    pub(crate) const PATH: u32 = 1;
    pub(crate) const LEADING_COMMENTS: u32 = 3;
    pub(crate) const TRAILING_COMMENTS: u32 = 4;

    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        while_decoding("SourceCodeInfo.Location", || {
            let mut path = Vec::new();
            let mut leading_comments = None;
            let mut trailing_comments = None;
            for field in AsRefExtProtobuf::read_protobuf_fields(bytes) {
                let field = field?;
                match field.field_number.as_u32() {
                    Self::PATH => extend_i32s(&mut path, &field)?,
                    Self::LEADING_COMMENTS => leading_comments = Some(expect_str(&field)?),
                    Self::TRAILING_COMMENTS => trailing_comments = Some(expect_str(&field)?),
                    _ => {}
                }
            }
            Ok(Self {
                path,
                leading_comments,
                trailing_comments,
            })
        })
    }

    pub fn path(&self) -> &[i32] {
        &self.path
    }

    pub fn leading_comments(&self) -> Option<&'a str> {
        self.leading_comments
    }

    pub fn trailing_comments(&self) -> Option<&'a str> {
        self.trailing_comments
    }
}
