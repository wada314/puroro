//! Minimal wire codec for `plugin.proto` / `descriptor.proto` subsets.
//!
//! Decodes only the fields needed to build [`crate::descriptor::CodegenRequest`],
//! and encodes a minimal [`CodeGeneratorResponse`].
//!
//! Decode walks wire bytes through [`views`]: each message is [`parse`](views::FileDescriptorProto::parse)d
//! once into an open struct with public fields. Field numbers live as associated
//! constants on those types (and on the encode types below). The structured IR
//! is [`crate::descriptor`]; this module does not use `puroro-types` generated
//! messages (bootstrap cycle).

pub mod views;

use self::views::{
    CodeGeneratorRequest as CodeGeneratorRequestView, DescriptorProto, EnumDescriptorProto,
    EnumValueDescriptorProto, FieldDescriptorProto, FileDescriptorProto, FileDescriptorSet,
    OneofDescriptorProto, SourceCodeInfo,
};
use crate::descriptor::features::FeatureSet;
use crate::descriptor::{
    BYTES_LAYOUT_OPTION_NUMBER, BytesLayout, CHILD_PARSE_OPTION_NUMBER, ChildParse, CodegenMeta,
    CodegenRequest, Comments, ENUM_REPR_OPTION_NUMBER, Edition, EnumDesc, EnumRepr, EnumValueDesc,
    FieldDesc, FieldLabel, FieldType, MESSAGE_LAYOUT_OPTION_NUMBER, MessageDesc, MessageLayout,
    OneofDesc, ProtoFile, ProtoFqn, STRING_LAYOUT_OPTION_NUMBER, StringLayout, Syntax,
    UNKNOWN_FIELDS_OPTION_NUMBER, UnknownFieldsPolicy,
};
use crate::error::{Error, Result};
use ::protobuf_core::{Field, FieldNumber, FieldValue, ProtobufError, WriteExtProtobuf};
use ::std::convert::TryFrom;

/// `CodeGeneratorResponse.FEATURE_PROTO3_OPTIONAL`
pub const FEATURE_PROTO3_OPTIONAL: u64 = 1;
/// `CodeGeneratorResponse.FEATURE_SUPPORTS_EDITIONS`
pub const FEATURE_SUPPORTS_EDITIONS: u64 = 2;

/// Features this plugin always advertises to `protoc`.
pub const SUPPORTED_FEATURES: u64 = FEATURE_PROTO3_OPTIONAL | FEATURE_SUPPORTS_EDITIONS;

/// `google.protobuf.Edition.EDITION_PROTO2` — inclusive lower bound we accept.
pub const MINIMUM_EDITION: i32 = 998;
/// `google.protobuf.Edition.EDITION_2024` — inclusive upper bound we resolve today.
pub const MAXIMUM_EDITION: i32 = 1001;

/// One generated file entry for `CodeGeneratorResponse.File`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseFile {
    pub name: String,
    pub content: String,
}

impl ResponseFile {
    /// `optional string name = 1;`
    pub(crate) const NAME: u32 = 1;
    /// `optional string content = 15;`
    pub(crate) const CONTENT: u32 = 15;
}

/// Plugin response before wire encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGeneratorResponse {
    pub error: Option<String>,
    pub supported_features: u64,
    pub minimum_edition: Option<i32>,
    pub maximum_edition: Option<i32>,
    pub files: Vec<ResponseFile>,
}

impl CodeGeneratorResponse {
    /// `optional string error = 1;`
    pub(crate) const ERROR: u32 = 1;
    /// `optional uint64 supported_features = 2;`
    pub(crate) const SUPPORTED_FEATURES_FIELD: u32 = 2;
    /// `optional int32 minimum_edition = 3;`
    pub(crate) const MINIMUM_EDITION_FIELD: u32 = 3;
    /// `optional int32 maximum_edition = 4;`
    pub(crate) const MAXIMUM_EDITION_FIELD: u32 = 4;
    /// `repeated File file = 15;`
    pub(crate) const FILE: u32 = 15;

    fn advertised(error: Option<String>, files: Vec<ResponseFile>) -> Self {
        Self {
            error,
            supported_features: SUPPORTED_FEATURES,
            minimum_edition: Some(MINIMUM_EDITION),
            maximum_edition: Some(MAXIMUM_EDITION),
            files,
        }
    }

    pub fn from_files(files: Vec<ResponseFile>) -> Self {
        Self::advertised(None, files)
    }

    pub fn from_error(message: impl Into<String>) -> Self {
        Self::advertised(Some(message.into()), Vec::new())
    }

    /// Encode this response to `CodeGeneratorResponse` wire bytes.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();

        if let Some(error) = &self.error {
            write_string_field(&mut out, Self::ERROR, error)?;
        }
        if self.supported_features != 0 {
            write_uint64_field(
                &mut out,
                Self::SUPPORTED_FEATURES_FIELD,
                self.supported_features,
            )?;
        }
        if let Some(edition) = self.minimum_edition {
            write_int32_field(&mut out, Self::MINIMUM_EDITION_FIELD, edition)?;
        }
        if let Some(edition) = self.maximum_edition {
            write_int32_field(&mut out, Self::MAXIMUM_EDITION_FIELD, edition)?;
        }
        for file in &self.files {
            let mut nested = Vec::new();
            write_string_field(&mut nested, ResponseFile::NAME, &file.name)?;
            write_string_field(&mut nested, ResponseFile::CONTENT, &file.content)?;
            write_len_field(&mut out, Self::FILE, &nested)?;
        }

        Ok(out)
    }
}

/// Decode `CodeGeneratorRequest` bytes into descriptor + metadata.
pub fn decode_request(bytes: &[u8]) -> Result<CodegenRequest> {
    with_decoding_context("CodeGeneratorRequest", || {
        let request = CodeGeneratorRequestView::parse(bytes)?;
        let file_to_generate = request
            .file_to_generate
            .iter()
            .copied()
            .map(str::to_owned)
            .collect();
        let parameter = request.parameter.map(str::to_owned);
        let mut proto_files = Vec::new();
        let mut proto_file_raw = Vec::new();
        for file in &request.proto_file {
            proto_file_raw.push(file.bytes.to_vec());
            proto_files.push(decode_file_descriptor(file)?);
        }
        Ok(CodegenRequest {
            meta: CodegenMeta {
                file_to_generate,
                parameter,
            },
            proto_files,
            proto_file_raw,
        })
    })
}

/// Encode raw `FileDescriptorProto` bodies as a `FileDescriptorSet` wire message.
///
/// `google.protobuf.FileDescriptorSet.file` is field 1 (length-delimited). Bodies
/// are written in the given order without re-parsing.
pub fn encode_file_descriptor_set(proto_file_raw: &[Vec<u8>]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    for body in proto_file_raw {
        write_len_field(&mut out, FileDescriptorSet::FILE, body)?;
    }
    Ok(out)
}

fn decode_file_descriptor(file: &FileDescriptorProto<'_>) -> Result<ProtoFile> {
    with_decoding_context("FileDescriptorProto", || {
        let name = file.name.unwrap_or("").to_owned();
        let package = file.package.unwrap_or("").to_owned();
        let dependency = file.dependency.iter().copied().map(str::to_owned).collect();
        let mut messages = Vec::new();
        for message in &file.message_type {
            messages.push(decode_descriptor(message)?);
        }
        let mut enums = Vec::new();
        for enumeration in &file.enum_type {
            enums.push(decode_enum(enumeration)?);
        }
        let features = match &file.options {
            Some(options) => with_decoding_context("FileOptions", || {
                Ok(match &options.features {
                    Some(fs) => decode_feature_set(fs)?,
                    None => FeatureSet::default(),
                })
            })?,
            None => FeatureSet::default(),
        };
        let source_locations = match &file.source_code_info {
            Some(info) => decode_source_code_info(info)?,
            None => Vec::new(),
        };
        let syntax = resolve_syntax(file.syntax, file.edition)?;

        let mut out = ProtoFile {
            name,
            package,
            syntax,
            features,
            dependency,
            messages,
            enums,
        };
        attach_source_comments(&mut out, &source_locations);
        Ok(out)
    })
}

fn resolve_syntax(syntax_raw: Option<&str>, edition_raw: Option<i32>) -> Result<Syntax> {
    match syntax_raw {
        None | Some("") | Some("proto2") => Ok(Syntax::Proto2),
        Some("proto3") => Ok(Syntax::Proto3),
        Some("editions") => {
            let raw = edition_raw.ok_or_else(|| {
                Error::Codegen(
                    "FileDescriptorProto.syntax is \"editions\" but edition is missing".into(),
                )
            })?;
            let edition = Edition::try_from(raw).map_err(|_| {
                Error::Codegen(format!("unsupported protobuf edition value `{raw}`"))
            })?;
            Ok(Syntax::Editions(edition))
        }
        Some(other) => Err(Error::Codegen(format!(
            "unsupported FileDescriptorProto.syntax `{other}`"
        ))),
    }
}

fn decode_descriptor(message: &DescriptorProto<'_>) -> Result<MessageDesc> {
    with_decoding_context("DescriptorProto", || {
        let name = message.name.unwrap_or("").to_owned();
        let mut fields = Vec::new();
        for field in &message.field {
            fields.push(decode_field(field)?);
        }
        let mut nested_messages = Vec::new();
        for nested in &message.nested_type {
            nested_messages.push(decode_descriptor(nested)?);
        }
        let mut nested_enums = Vec::new();
        for enumeration in &message.enum_type {
            nested_enums.push(decode_enum(enumeration)?);
        }
        let mut oneofs = Vec::new();
        for oneof in &message.oneof_decl {
            oneofs.push(decode_oneof(oneof)?);
        }
        let decoded = match &message.options {
            Some(options) => decode_message_options(options)?,
            None => DecodedMessageOptions::default(),
        };

        Ok(MessageDesc {
            name,
            fields,
            nested_messages,
            nested_enums,
            oneofs,
            map_entry: decoded.map_entry,
            unknown_fields: decoded.unknown_fields,
            child_parse: decoded.child_parse,
            features: decoded.features,
            comments: Comments::default(),
        })
    })
}

#[derive(Default)]
struct DecodedMessageOptions {
    map_entry: bool,
    unknown_fields: UnknownFieldsPolicy,
    child_parse: ChildParse,
    features: FeatureSet,
}

/// `MessageOptions`: `map_entry`, `features`, and puroro extensions.
fn decode_message_options(options: &views::MessageOptions) -> Result<DecodedMessageOptions> {
    with_decoding_context("MessageOptions", || {
        Ok(DecodedMessageOptions {
            map_entry: options.map_entry.unwrap_or(false),
            unknown_fields: enum_from_i32_opt(
                options.extension_enum(UNKNOWN_FIELDS_OPTION_NUMBER),
                UNKNOWN_FIELDS_OPTION_NUMBER,
            )?
            .unwrap_or_default(),
            child_parse: enum_from_i32_opt(
                options.extension_enum(CHILD_PARSE_OPTION_NUMBER),
                CHILD_PARSE_OPTION_NUMBER,
            )?
            .unwrap_or_default(),
            features: match &options.features {
                Some(fs) => decode_feature_set(fs)?,
                None => FeatureSet::default(),
            },
        })
    })
}

fn decode_field(field: &FieldDescriptorProto<'_>) -> Result<FieldDesc> {
    with_decoding_context("FieldDescriptorProto", || {
        let name = field.name.unwrap_or("").to_owned();
        let number = field.number.unwrap_or(0);
        let label = enum_from_i32_opt(field.label, FieldDescriptorProto::LABEL)?
            .unwrap_or(FieldLabel::Optional);
        let type_ =
            enum_from_i32_opt(field.type_, FieldDescriptorProto::TYPE)?.unwrap_or(FieldType::Int32);
        let type_name = field.type_name.map(ProtoFqn::parse);
        let default_value = field.default_value.map(str::to_owned);
        let oneof_index = field.oneof_index;
        let proto3_optional = field.proto3_optional.unwrap_or(false);
        let decoded = match &field.options {
            Some(options) => decode_field_options(options)?,
            None => DecodedFieldOptions::default(),
        };

        Ok(FieldDesc {
            name,
            number,
            label,
            type_,
            type_name,
            oneof_index,
            proto3_optional,
            default_value,
            packed: decoded.packed,
            string_layout: decoded.string_layout,
            bytes_layout: decoded.bytes_layout,
            message_layout: decoded.message_layout,
            child_parse: decoded.child_parse,
            features: decoded.features,
            comments: Comments::default(),
        })
    })
}

#[derive(Default)]
struct DecodedFieldOptions {
    features: FeatureSet,
    packed: Option<bool>,
    string_layout: Option<StringLayout>,
    bytes_layout: Option<BytesLayout>,
    message_layout: Option<MessageLayout>,
    child_parse: Option<ChildParse>,
}

/// `FieldOptions`: `packed`, `features`, and puroro layout extensions.
fn decode_field_options(options: &views::FieldOptions) -> Result<DecodedFieldOptions> {
    with_decoding_context("FieldOptions", || {
        Ok(DecodedFieldOptions {
            features: match &options.features {
                Some(fs) => decode_feature_set(fs)?,
                None => FeatureSet::default(),
            },
            packed: options.packed,
            string_layout: enum_from_i32_opt(
                options.extension_enum(STRING_LAYOUT_OPTION_NUMBER),
                STRING_LAYOUT_OPTION_NUMBER,
            )?,
            bytes_layout: enum_from_i32_opt(
                options.extension_enum(BYTES_LAYOUT_OPTION_NUMBER),
                BYTES_LAYOUT_OPTION_NUMBER,
            )?,
            message_layout: enum_from_i32_opt(
                options.extension_enum(MESSAGE_LAYOUT_OPTION_NUMBER),
                MESSAGE_LAYOUT_OPTION_NUMBER,
            )?,
            child_parse: enum_from_i32_opt(
                options.extension_enum(CHILD_PARSE_OPTION_NUMBER),
                CHILD_PARSE_OPTION_NUMBER,
            )?,
        })
    })
}

fn decode_feature_set(features: &views::FeatureSet) -> Result<FeatureSet> {
    with_decoding_context("FeatureSet", || {
        Ok(FeatureSet {
            field_presence: enum_from_i32_opt(
                features.field_presence,
                views::FeatureSet::FIELD_PRESENCE,
            )?,
            enum_type: enum_from_i32_opt(features.enum_type, views::FeatureSet::ENUM_TYPE)?,
            repeated_field_encoding: enum_from_i32_opt(
                features.repeated_field_encoding,
                views::FeatureSet::REPEATED_FIELD_ENCODING,
            )?,
            utf8_validation: enum_from_i32_opt(
                features.utf8_validation,
                views::FeatureSet::UTF8_VALIDATION,
            )?,
            message_encoding: enum_from_i32_opt(
                features.message_encoding,
                views::FeatureSet::MESSAGE_ENCODING,
            )?,
            json_format: enum_from_i32_opt(features.json_format, views::FeatureSet::JSON_FORMAT)?,
            enforce_naming_style: enum_from_i32_opt(
                features.enforce_naming_style,
                views::FeatureSet::ENFORCE_NAMING_STYLE,
            )?,
            default_symbol_visibility: enum_from_i32_opt(
                features.default_symbol_visibility,
                views::FeatureSet::DEFAULT_SYMBOL_VISIBILITY,
            )?,
        })
    })
}

fn decode_oneof(oneof: &OneofDescriptorProto<'_>) -> Result<OneofDesc> {
    with_decoding_context("OneofDescriptorProto", || {
        Ok(OneofDesc {
            name: oneof.name.unwrap_or("").to_owned(),
            comments: Comments::default(),
        })
    })
}

fn decode_enum(enumeration: &EnumDescriptorProto<'_>) -> Result<EnumDesc> {
    with_decoding_context("EnumDescriptorProto", || {
        let name = enumeration.name.unwrap_or("").to_owned();
        let mut values = Vec::new();
        for value in &enumeration.value {
            values.push(decode_enum_value(value)?);
        }
        let decoded = match &enumeration.options {
            Some(options) => decode_enum_options(options)?,
            None => DecodedEnumOptions::default(),
        };
        Ok(EnumDesc {
            name,
            values,
            features: decoded.features,
            enum_repr: decoded.enum_repr,
            comments: Comments::default(),
        })
    })
}

#[derive(Default)]
struct DecodedEnumOptions {
    features: FeatureSet,
    enum_repr: EnumRepr,
}

/// `EnumOptions`: `features` and `(puroro.enum_repr)`.
fn decode_enum_options(options: &views::EnumOptions) -> Result<DecodedEnumOptions> {
    with_decoding_context("EnumOptions", || {
        Ok(DecodedEnumOptions {
            features: match &options.features {
                Some(fs) => decode_feature_set(fs)?,
                None => FeatureSet::default(),
            },
            enum_repr: enum_from_i32_opt(
                options.extension_enum(ENUM_REPR_OPTION_NUMBER),
                ENUM_REPR_OPTION_NUMBER,
            )?
            .unwrap_or_default(),
        })
    })
}

fn decode_enum_value(value: &EnumValueDescriptorProto<'_>) -> Result<EnumValueDesc> {
    with_decoding_context("EnumValueDescriptorProto", || {
        Ok(EnumValueDesc {
            name: value.name.unwrap_or("").to_owned(),
            number: value.number.unwrap_or(0),
            comments: Comments::default(),
        })
    })
}

/// One `SourceCodeInfo.Location` with comments we care about for rustdoc.
struct SourceLocation {
    path: Vec<i32>,
    comments: Comments,
}

fn decode_source_code_info(info: &SourceCodeInfo<'_>) -> Result<Vec<SourceLocation>> {
    with_decoding_context("SourceCodeInfo", || {
        let mut locations = Vec::new();
        for loc in &info.location {
            if let Some(decoded) = decode_source_location(loc)? {
                locations.push(decoded);
            }
        }
        Ok(locations)
    })
}

fn decode_source_location(
    loc: &views::SourceCodeInfoLocation<'_>,
) -> Result<Option<SourceLocation>> {
    with_decoding_context("SourceCodeInfo.Location", || {
        let comments = Comments {
            leading: loc.leading_comments.map(str::to_owned),
            trailing: loc.trailing_comments.map(str::to_owned),
        };
        if comments.is_empty() {
            Ok(None)
        } else {
            Ok(Some(SourceLocation {
                path: loc.path.clone(),
                comments,
            }))
        }
    })
}

fn enum_from_i32_opt<T: TryFrom<i32>>(raw: Option<i32>, field_number: u32) -> Result<Option<T>> {
    match raw {
        None => Ok(None),
        Some(value) => T::try_from(value)
            .map(Some)
            .map_err(|_| Error::unexpected_field(field_number)),
    }
}

/// Attach `SourceCodeInfo` comments onto declaration descriptors by path.
///
/// Only whole-declaration paths are used (message / field / enum / value /
/// oneof). Deeper paths (name, type, number, …) are ignored.
fn attach_source_comments(file: &mut ProtoFile, locations: &[SourceLocation]) {
    // SourceCodeInfo paths store descriptor field numbers as `int32`.
    let message_type = FileDescriptorProto::MESSAGE_TYPE as i32;
    let enum_type = FileDescriptorProto::ENUM_TYPE as i32;
    for loc in locations {
        match loc.path.as_slice() {
            [m, mi] if *m == message_type => {
                if let Some(message) = index_mut(&mut file.messages, *mi) {
                    message.comments.merge(loc.comments.clone());
                }
            }
            [m, mi, rest @ ..] if *m == message_type => {
                if let Some(message) = index_mut(&mut file.messages, *mi) {
                    attach_message_comments(message, rest, &loc.comments);
                }
            }
            [e, ei] if *e == enum_type => {
                if let Some(enumeration) = index_mut(&mut file.enums, *ei) {
                    enumeration.comments.merge(loc.comments.clone());
                }
            }
            [e, ei, rest @ ..] if *e == enum_type => {
                if let Some(enumeration) = index_mut(&mut file.enums, *ei) {
                    attach_enum_comments(enumeration, rest, &loc.comments);
                }
            }
            _ => {}
        }
    }
}

fn attach_message_comments(message: &mut MessageDesc, path: &[i32], comments: &Comments) {
    let field_path = DescriptorProto::FIELD as i32;
    let nested_type_path = DescriptorProto::NESTED_TYPE as i32;
    let enum_type_path = DescriptorProto::ENUM_TYPE as i32;
    let oneof_decl_path = DescriptorProto::ONEOF_DECL as i32;
    match path {
        [] => message.comments.merge(comments.clone()),
        [f, fi] if *f == field_path => {
            if let Some(desc) = index_mut(&mut message.fields, *fi) {
                desc.comments.merge(comments.clone());
            }
        }
        [n, ni] if *n == nested_type_path => {
            if let Some(nested) = index_mut(&mut message.nested_messages, *ni) {
                nested.comments.merge(comments.clone());
            }
        }
        [n, ni, rest @ ..] if *n == nested_type_path => {
            if let Some(nested) = index_mut(&mut message.nested_messages, *ni) {
                attach_message_comments(nested, rest, comments);
            }
        }
        [e, ei] if *e == enum_type_path => {
            if let Some(enumeration) = index_mut(&mut message.nested_enums, *ei) {
                enumeration.comments.merge(comments.clone());
            }
        }
        [e, ei, rest @ ..] if *e == enum_type_path => {
            if let Some(enumeration) = index_mut(&mut message.nested_enums, *ei) {
                attach_enum_comments(enumeration, rest, comments);
            }
        }
        [o, oi] if *o == oneof_decl_path => {
            if let Some(oneof) = index_mut(&mut message.oneofs, *oi) {
                oneof.comments.merge(comments.clone());
            }
        }
        _ => {}
    }
}

fn attach_enum_comments(enumeration: &mut EnumDesc, path: &[i32], comments: &Comments) {
    let value_path = EnumDescriptorProto::VALUE as i32;
    match path {
        [] => enumeration.comments.merge(comments.clone()),
        [v, vi] if *v == value_path => {
            if let Some(desc) = index_mut(&mut enumeration.values, *vi) {
                desc.comments.merge(comments.clone());
            }
        }
        _ => {}
    }
}

fn index_mut<T>(items: &mut [T], index: i32) -> Option<&mut T> {
    usize::try_from(index).ok().and_then(|i| items.get_mut(i))
}

fn with_decoding_context<T>(message: &'static str, f: impl FnOnce() -> Result<T>) -> Result<T> {
    f().map_err(|e| e.while_decoding(message))
}

/// Maps [`FieldNumber::try_new`]'s bare `u32` error into [`Error`].
trait FieldNumberExt: Sized {
    fn try_from_u32(value: u32) -> Result<Self>;
}

impl FieldNumberExt for FieldNumber {
    fn try_from_u32(value: u32) -> Result<Self> {
        Self::try_new(value).map_err(|value| {
            ProtobufError::FieldNumberOutOfRange {
                value: i64::from(value),
            }
            .into()
        })
    }
}

fn write_len_field(out: &mut Vec<u8>, number: u32, value: &[u8]) -> Result<()> {
    let field = Field::new(FieldNumber::try_from_u32(number)?, FieldValue::Len(value));
    out.write_protobuf_field(&field)?;
    Ok(())
}

fn write_string_field(out: &mut Vec<u8>, number: u32, value: &str) -> Result<()> {
    write_len_field(out, number, value.as_bytes())
}

fn write_int32_field(out: &mut Vec<u8>, number: u32, value: i32) -> Result<()> {
    let field: Field<&[u8]> = Field::new(
        FieldNumber::try_from_u32(number)?,
        FieldValue::from_int32(value),
    );
    out.write_protobuf_field(&field)?;
    Ok(())
}

fn write_uint64_field(out: &mut Vec<u8>, number: u32, value: u64) -> Result<()> {
    let field: Field<&[u8]> = Field::new(
        FieldNumber::try_from_u32(number)?,
        FieldValue::from_uint64(value),
    );
    out.write_protobuf_field(&field)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::features::{EnumType, FieldPresence};
    use crate::descriptor::{FieldLabel, FieldType};
    use ::protobuf_core::AsRefExtProtobuf;

    fn expect_len<'a>(field: &Field<&'a [u8]>) -> Result<&'a [u8]> {
        match &field.value {
            FieldValue::Len(bytes) => Ok(*bytes),
            _ => Err(Error::unexpected_field(field.field_number.as_u32())),
        }
    }

    fn expect_string(field: &Field<&[u8]>) -> Result<String> {
        let bytes = expect_len(field)?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    fn encode_string_field(field_number: u32, value: &str) -> Vec<u8> {
        let mut out = Vec::new();
        write_string_field(&mut out, field_number, value).unwrap();
        out
    }

    fn encode_message_field(field_number: u32, nested: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        write_len_field(&mut out, field_number, nested).unwrap();
        out
    }

    fn encode_varint_field(field_number: u32, value: i32) -> Vec<u8> {
        let mut out = Vec::new();
        write_int32_field(&mut out, field_number, value).unwrap();
        out
    }

    fn encode_packed_int32s(values: &[i32]) -> Vec<u8> {
        use ::protobuf_core::WriteExtVarint;
        let mut bytes = Vec::new();
        for value in values {
            let varint = ::protobuf_core::Varint::from_int32(*value);
            bytes.write_varint(&varint).unwrap();
        }
        bytes
    }

    fn encode_source_location(path: &[i32], leading: &str) -> Vec<u8> {
        let mut loc = Vec::new();
        // repeated int32 path = 1 [packed = true];
        write_len_field(&mut loc, 1, &encode_packed_int32s(path)).unwrap();
        // optional string leading_comments = 3;
        loc.extend(encode_string_field(3, leading));
        loc
    }

    #[test]
    fn source_code_info_attaches_comments_by_path() {
        // FieldDescriptorProto { name=title, number=1, label=OPTIONAL, type=STRING }
        let mut field = Vec::new();
        field.extend(encode_string_field(1, "title"));
        field.extend(encode_varint_field(3, 1));
        field.extend(encode_varint_field(4, FieldLabel::Optional as i32));
        field.extend(encode_varint_field(5, FieldType::String as i32));

        // DescriptorProto { name=Task, field=... }
        let mut message = Vec::new();
        message.extend(encode_string_field(1, "Task"));
        message.extend(encode_message_field(2, &field));

        let mut source_info = Vec::new();
        // FileDescriptorProto.message_type[0]
        source_info.extend(encode_message_field(
            1,
            &encode_source_location(&[4, 0], "A task message.\n"),
        ));
        // FileDescriptorProto.message_type[0].field[0]
        source_info.extend(encode_message_field(
            1,
            &encode_source_location(&[4, 0, 2, 0], "Title text.\n"),
        ));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "task.proto"));
        file.extend(encode_string_field(12, "proto3"));
        file.extend(encode_message_field(4, &message));
        file.extend(encode_message_field(9, &source_info));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "task.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        let msg = &decoded.proto_files[0].messages[0];
        assert_eq!(
            msg.comments.doc_text().map(str::trim),
            Some("A task message."),
            "{:?}",
            msg.comments
        );
        assert_eq!(
            msg.fields[0].comments.doc_text().map(str::trim),
            Some("Title text."),
            "{:?}",
            msg.fields[0].comments
        );
    }

    #[test]
    fn decode_minimal_request() {
        // FieldDescriptorProto { name=title, number=1, label=OPTIONAL, type=STRING }
        let mut field = Vec::new();
        field.extend(encode_string_field(1, "title"));
        field.extend(encode_varint_field(3, 1));
        field.extend(encode_varint_field(4, FieldLabel::Optional as i32));
        field.extend(encode_varint_field(5, FieldType::String as i32));

        // DescriptorProto { name=Task, field=... }
        let mut message = Vec::new();
        message.extend(encode_string_field(1, "Task"));
        message.extend(encode_message_field(2, &field));

        // FileDescriptorProto { name=example.proto, package=example, message_type=Task }
        let mut file = Vec::new();
        file.extend(encode_string_field(1, "example.proto"));
        file.extend(encode_string_field(2, "example"));
        file.extend(encode_message_field(4, &message));

        // CodeGeneratorRequest
        let mut request = Vec::new();
        request.extend(encode_string_field(1, "example.proto"));
        request.extend(encode_string_field(2, "rename=foo"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(decoded.meta.file_to_generate, vec!["example.proto"]);
        assert_eq!(decoded.meta.parameter.as_deref(), Some("rename=foo"));
        assert_eq!(decoded.proto_files.len(), 1);
        assert_eq!(decoded.proto_file_raw, vec![file]);
        assert_eq!(decoded.proto_files[0].name, "example.proto");
        assert_eq!(decoded.proto_files[0].package, "example");
        // syntax field omitted → proto2 default
        assert_eq!(decoded.proto_files[0].syntax, Syntax::Proto2);
        assert_eq!(decoded.proto_files[0].features, FeatureSet::default());
        assert_eq!(decoded.proto_files[0].messages.len(), 1);
        assert_eq!(decoded.proto_files[0].messages[0].name, "Task");
        assert_eq!(decoded.proto_files[0].messages[0].fields.len(), 1);
        let f = &decoded.proto_files[0].messages[0].fields[0];
        assert_eq!(f.name, "title");
        assert_eq!(f.number, 1);
        assert_eq!(f.label, FieldLabel::Optional);
        assert_eq!(f.type_, FieldType::String);
    }

    #[test]
    fn decode_editions_file_with_field_presence_feature() {
        // FeatureSet { field_presence: IMPLICIT }
        let feature_set = encode_varint_field(1, FieldPresence::Implicit as i32);
        // FileOptions { features: FeatureSet }
        let file_options = encode_message_field(50, &feature_set);

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "ed.proto"));
        file.extend(encode_string_field(12, "editions"));
        file.extend(encode_varint_field(14, Edition::Edition2023 as i32));
        file.extend(encode_message_field(8, &file_options));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "ed.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].syntax,
            Syntax::Editions(Edition::Edition2023)
        );
        assert_eq!(
            decoded.proto_files[0].features.field_presence,
            Some(FieldPresence::Implicit)
        );
    }

    #[test]
    fn file_options_field_7_is_not_features() {
        // EnumOptions.features = 7; FileOptions.features = 50. A FeatureSet
        // sitting on FileOptions field 7 must not be treated as file features.
        let feature_set = encode_varint_field(1, FieldPresence::Implicit as i32);
        let file_options = encode_message_field(7, &feature_set);

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "ed.proto"));
        file.extend(encode_string_field(12, "editions"));
        file.extend(encode_varint_field(14, Edition::Edition2023 as i32));
        file.extend(encode_message_field(8, &file_options));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "ed.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert!(decoded.proto_files[0].features.field_presence.is_none());
    }

    #[test]
    fn decode_feature_set_skips_unknown_len_extension() {
        // FeatureSet { field_presence: IMPLICIT, (unknown LEN extension) = 1000 }
        let mut feature_set = encode_varint_field(1, FieldPresence::Implicit as i32);
        feature_set.extend(encode_message_field(1000, b"lang-ext"));
        let file_options = encode_message_field(50, &feature_set);

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "ed.proto"));
        file.extend(encode_string_field(12, "editions"));
        file.extend(encode_varint_field(14, Edition::Edition2023 as i32));
        file.extend(encode_message_field(8, &file_options));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "ed.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].features.field_presence,
            Some(FieldPresence::Implicit)
        );
    }

    #[test]
    fn decode_field_options_features_field_number_21() {
        use crate::descriptor::features::RepeatedFieldEncoding;

        // FeatureSet { repeated_field_encoding: EXPANDED }
        let feature_set = encode_varint_field(3, RepeatedFieldEncoding::Expanded as i32);
        // FieldOptions { features = 21 }
        let field_options = encode_message_field(21, &feature_set);

        let mut field = Vec::new();
        field.extend(encode_string_field(1, "ids"));
        field.extend(encode_varint_field(3, 1));
        field.extend(encode_varint_field(4, FieldLabel::Repeated as i32));
        field.extend(encode_varint_field(5, FieldType::Int32 as i32));
        field.extend(encode_message_field(8, &field_options));

        let mut message = Vec::new();
        message.extend(encode_string_field(1, "M"));
        message.extend(encode_message_field(2, &field));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "ed.proto"));
        file.extend(encode_string_field(12, "editions"));
        file.extend(encode_varint_field(14, Edition::Edition2023 as i32));
        file.extend(encode_message_field(4, &message));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "ed.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].messages[0].fields[0]
                .features
                .repeated_field_encoding,
            Some(RepeatedFieldEncoding::Expanded)
        );
    }

    #[test]
    fn decode_field_options_string_layout_extension() {
        use crate::descriptor::{STRING_LAYOUT_OPTION_NUMBER, StringLayout};

        // FieldOptions { (puroro.string_layout) = STRING_LAYOUT_HEAP }
        let field_options =
            encode_varint_field(STRING_LAYOUT_OPTION_NUMBER, StringLayout::Heap as i32);

        let mut field = Vec::new();
        field.extend(encode_string_field(1, "body"));
        field.extend(encode_varint_field(3, 1));
        field.extend(encode_varint_field(4, FieldLabel::Optional as i32));
        field.extend(encode_varint_field(5, FieldType::String as i32));
        field.extend(encode_message_field(8, &field_options));

        let mut message = Vec::new();
        message.extend(encode_string_field(1, "M"));
        message.extend(encode_message_field(2, &field));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "t.proto"));
        file.extend(encode_message_field(4, &message));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "t.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].messages[0].fields[0].string_layout,
            Some(StringLayout::Heap)
        );
    }

    #[test]
    fn decode_field_options_bytes_layout_extension() {
        use crate::descriptor::{BYTES_LAYOUT_OPTION_NUMBER, BytesLayout};

        // FieldOptions { (puroro.bytes_layout) = BYTES_LAYOUT_HEAP }
        let field_options =
            encode_varint_field(BYTES_LAYOUT_OPTION_NUMBER, BytesLayout::Heap as i32);

        let mut field = Vec::new();
        field.extend(encode_string_field(1, "body"));
        field.extend(encode_varint_field(3, 1));
        field.extend(encode_varint_field(4, FieldLabel::Optional as i32));
        field.extend(encode_varint_field(5, FieldType::Bytes as i32));
        field.extend(encode_message_field(8, &field_options));

        let mut message = Vec::new();
        message.extend(encode_string_field(1, "M"));
        message.extend(encode_message_field(2, &field));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "t.proto"));
        file.extend(encode_message_field(4, &message));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "t.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].messages[0].fields[0].bytes_layout,
            Some(BytesLayout::Heap)
        );
    }

    #[test]
    fn decode_field_options_message_layout_extension() {
        use crate::descriptor::{MESSAGE_LAYOUT_OPTION_NUMBER, MessageLayout};

        // FieldOptions { (puroro.message_layout) = MESSAGE_LAYOUT_INLINE }
        let field_options =
            encode_varint_field(MESSAGE_LAYOUT_OPTION_NUMBER, MessageLayout::Inline as i32);

        let mut field = Vec::new();
        field.extend(encode_string_field(1, "child"));
        field.extend(encode_varint_field(3, 1));
        field.extend(encode_varint_field(4, FieldLabel::Optional as i32));
        field.extend(encode_varint_field(5, FieldType::Message as i32));
        field.extend(encode_string_field(6, ".Point"));
        field.extend(encode_message_field(8, &field_options));

        let mut message = Vec::new();
        message.extend(encode_string_field(1, "M"));
        message.extend(encode_message_field(2, &field));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "t.proto"));
        file.extend(encode_message_field(4, &message));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "t.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].messages[0].fields[0].message_layout,
            Some(MessageLayout::Inline)
        );
    }

    #[test]
    fn decode_message_options_unknown_fields_extension() {
        use crate::descriptor::{UNKNOWN_FIELDS_OPTION_NUMBER, UnknownFieldsPolicy};

        // MessageOptions { (puroro.unknown_fields) = UNKNOWN_FIELDS_DISCARD }
        let options = encode_varint_field(
            UNKNOWN_FIELDS_OPTION_NUMBER,
            UnknownFieldsPolicy::Discard as i32,
        );

        let mut message = Vec::new();
        message.extend(encode_string_field(1, "M"));
        message.extend(encode_message_field(7, &options));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "t.proto"));
        file.extend(encode_message_field(4, &message));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "t.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].messages[0].unknown_fields,
            UnknownFieldsPolicy::Discard
        );

        let reject = encode_varint_field(
            UNKNOWN_FIELDS_OPTION_NUMBER,
            UnknownFieldsPolicy::Reject as i32,
        );
        let mut reject_message = Vec::new();
        reject_message.extend(encode_string_field(1, "R"));
        reject_message.extend(encode_message_field(7, &reject));
        let mut reject_file = Vec::new();
        reject_file.extend(encode_string_field(1, "t.proto"));
        reject_file.extend(encode_message_field(4, &reject_message));
        let mut reject_request = Vec::new();
        reject_request.extend(encode_string_field(1, "t.proto"));
        reject_request.extend(encode_message_field(15, &reject_file));
        let decoded = decode_request(&reject_request).unwrap();
        assert_eq!(
            decoded.proto_files[0].messages[0].unknown_fields,
            UnknownFieldsPolicy::Reject
        );
    }

    #[test]
    fn decode_message_options_json_format() {
        use crate::descriptor::features::JsonFormat;

        // FeatureSet { json_format: LEGACY_BEST_EFFORT }
        // MessageOptions.features is field 12 (runtime retention, message target).
        let feature_set = encode_varint_field(6, JsonFormat::LegacyBestEffort as i32);
        let options = encode_message_field(12, &feature_set);

        let mut message = Vec::new();
        message.extend(encode_string_field(1, "M"));
        message.extend(encode_message_field(7, &options));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "t.proto"));
        file.extend(encode_message_field(4, &message));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "t.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].messages[0].features.json_format,
            Some(JsonFormat::LegacyBestEffort)
        );
    }

    #[test]
    fn decode_enum_options_features_field_number_7() {
        // FeatureSet { enum_type: CLOSED }
        let feature_set = encode_varint_field(2, EnumType::Closed as i32);
        // EnumOptions { features = 7 }
        let enum_options = encode_message_field(7, &feature_set);
        let mut enum_desc = Vec::new();
        enum_desc.extend(encode_string_field(1, "Priority"));
        enum_desc.extend(encode_message_field(3, &enum_options));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "ed.proto"));
        file.extend(encode_string_field(12, "editions"));
        file.extend(encode_varint_field(14, Edition::Edition2023 as i32));
        file.extend(encode_message_field(5, &enum_desc));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "ed.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].enums[0].features.enum_type,
            Some(EnumType::Closed)
        );
    }

    #[test]
    fn decode_enum_options_enum_repr_extension() {
        use crate::descriptor::{ENUM_REPR_OPTION_NUMBER, EnumRepr};

        // EnumOptions { (puroro.enum_repr) = ENUM_REPR_RUST_ENUM }
        let enum_options = encode_varint_field(ENUM_REPR_OPTION_NUMBER, EnumRepr::RustEnum as i32);
        let mut enum_desc = Vec::new();
        enum_desc.extend(encode_string_field(1, "Status"));
        enum_desc.extend(encode_message_field(3, &enum_options));

        let mut file = Vec::new();
        file.extend(encode_string_field(1, "t.proto"));
        file.extend(encode_message_field(5, &enum_desc));

        let mut request = Vec::new();
        request.extend(encode_string_field(1, "t.proto"));
        request.extend(encode_message_field(15, &file));

        let decoded = decode_request(&request).unwrap();
        assert_eq!(
            decoded.proto_files[0].enums[0].enum_repr,
            EnumRepr::RustEnum
        );
    }

    #[test]
    fn encode_response_round_trips_files() {
        let response = CodeGeneratorResponse::from_files(vec![ResponseFile {
            name: "example.rs".into(),
            content: "// hello\n".into(),
        }]);
        let bytes = response.encode().unwrap();

        let mut names = Vec::new();
        let mut contents = Vec::new();
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field.unwrap();
            if field.field_number.as_u32() == CodeGeneratorResponse::FILE {
                let nested = expect_len(&field).unwrap();
                for inner in AsRefExtProtobuf::read_protobuf_fields(&nested) {
                    let inner = inner.unwrap();
                    match inner.field_number.as_u32() {
                        ResponseFile::NAME => names.push(expect_string(&inner).unwrap()),
                        ResponseFile::CONTENT => contents.push(expect_string(&inner).unwrap()),
                        _ => {}
                    }
                }
            }
        }
        assert_eq!(names, vec!["example.rs"]);
        assert_eq!(contents, vec!["// hello\n"]);
    }

    #[test]
    fn bad_field_gets_message_context() {
        // Field number 1 on CodeGeneratorRequest must be a string (LEN), not a varint.
        let mut request = Vec::new();
        request.extend(encode_varint_field(1, 42));
        let err = decode_request(&request).unwrap_err();
        assert!(
            err.to_string()
                .contains("unexpected field 1 while decoding CodeGeneratorRequest")
        );
    }

    #[test]
    fn bad_field_options_packed_gets_field_options_context() {
        // FieldOptions.packed = 2 must be a bool (varint), not LEN.
        let field_options = encode_string_field(2, "nope");
        let mut field = Vec::new();
        field.extend(encode_string_field(1, "ids"));
        field.extend(encode_message_field(8, &field_options));
        let mut message = Vec::new();
        message.extend(encode_string_field(1, "M"));
        message.extend(encode_message_field(2, &field));
        let mut file = Vec::new();
        file.extend(encode_string_field(1, "t.proto"));
        file.extend(encode_message_field(4, &message));
        let mut request = Vec::new();
        request.extend(encode_string_field(1, "t.proto"));
        request.extend(encode_message_field(15, &file));

        let err = decode_request(&request).unwrap_err();
        assert!(
            err.to_string()
                .contains("unexpected field 2 while decoding FieldOptions")
        );
    }

    #[test]
    fn bad_feature_set_field_gets_feature_set_context() {
        // FeatureSet.field_presence = 1 must be a varint, not LEN.
        let feature_set = encode_string_field(1, "nope");
        let file_options = encode_message_field(50, &feature_set);
        let mut file = Vec::new();
        file.extend(encode_string_field(1, "ed.proto"));
        file.extend(encode_message_field(8, &file_options));
        let mut request = Vec::new();
        request.extend(encode_string_field(1, "ed.proto"));
        request.extend(encode_message_field(15, &file));

        let err = decode_request(&request).unwrap_err();
        assert!(
            err.to_string()
                .contains("unexpected field 1 while decoding FeatureSet")
        );
    }
}
