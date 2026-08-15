//! Minimal wire codec for `plugin.proto` / `descriptor.proto` subsets.
//!
//! Decodes only the fields needed to build [`crate::descriptor::CodegenRequest`],
//! and encodes a minimal [`CodeGeneratorResponse`].

use crate::descriptor::features::FeatureSet;
use crate::descriptor::{
    BYTES_LAYOUT_OPTION_NUMBER, BytesLayout, CodegenMeta, CodegenRequest, Edition, EnumDesc,
    EnumValueDesc, FieldDesc, FieldLabel, FieldType, MessageDesc, OneofDesc, ProtoFile, ProtoFqn,
    STRING_LAYOUT_OPTION_NUMBER, StringLayout, Syntax,
};
use crate::error::{Error, Result};
use ::protobuf_core::{
    AsRefExtProtobuf, Field, FieldNumber, FieldValue, ProtobufError, WriteExtProtobuf,
};
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
            // optional string error = 1;
            write_string_field(&mut out, 1, error)?;
        }
        if self.supported_features != 0 {
            // optional uint64 supported_features = 2;
            write_uint64_field(&mut out, 2, self.supported_features)?;
        }
        if let Some(edition) = self.minimum_edition {
            // optional int32 minimum_edition = 3;
            write_int32_field(&mut out, 3, edition)?;
        }
        if let Some(edition) = self.maximum_edition {
            // optional int32 maximum_edition = 4;
            write_int32_field(&mut out, 4, edition)?;
        }
        for file in &self.files {
            let mut nested = Vec::new();
            // CodeGeneratorResponse.File.name = 1;
            write_string_field(&mut nested, 1, &file.name)?;
            // CodeGeneratorResponse.File.content = 15;
            write_string_field(&mut nested, 15, &file.content)?;
            // repeated File file = 15;
            write_len_field(&mut out, 15, &nested)?;
        }

        Ok(out)
    }
}

/// Decode `CodeGeneratorRequest` bytes into descriptor + metadata.
pub fn decode_request(bytes: &[u8]) -> Result<CodegenRequest> {
    with_decoding_context("CodeGeneratorRequest", || {
        let mut file_to_generate = Vec::new();
        let mut parameter = None;
        let mut proto_files = Vec::new();

        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // repeated string file_to_generate = 1;
                1 => file_to_generate.push(expect_string(&field)?),
                // optional string parameter = 2;
                2 => parameter = Some(expect_string(&field)?),
                // optional Version compiler_version = 3; (ignored)
                // repeated FileDescriptorProto proto_file = 15;
                15 => {
                    let nested = expect_len(&field)?;
                    proto_files.push(decode_file_descriptor(nested)?);
                }
                // repeated FileDescriptorProto source_file_descriptors = 17; (ignored)
                _ => {}
            }
        }

        Ok(CodegenRequest {
            meta: CodegenMeta {
                file_to_generate,
                parameter,
            },
            proto_files,
        })
    })
}

fn decode_file_descriptor(bytes: &[u8]) -> Result<ProtoFile> {
    with_decoding_context("FileDescriptorProto", || {
        let mut name = String::new();
        let mut package = String::new();
        let mut syntax_raw: Option<String> = None;
        let mut edition_raw: Option<i32> = None;
        let mut features = FeatureSet::default();
        let mut dependency = Vec::new();
        let mut messages = Vec::new();
        let mut enums = Vec::new();

        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional string name = 1; // file name, relative to root of source tree
                1 => name = expect_string(&field)?,
                // optional string package = 2; // e.g. "foo", "foo.bar", etc.
                2 => package = expect_string(&field)?,
                // repeated string dependency = 3;
                3 => dependency.push(expect_string(&field)?),
                // repeated DescriptorProto message_type = 4;
                4 => {
                    let nested = expect_len(&field)?;
                    messages.push(decode_descriptor(nested)?);
                }
                // repeated EnumDescriptorProto enum_type = 5;
                5 => {
                    let nested = expect_len(&field)?;
                    enums.push(decode_enum(nested)?);
                }
                // optional FileOptions options = 8;
                8 => {
                    let nested = expect_len(&field)?;
                    features = decode_options_features("FileOptions", 50, nested)?;
                }
                // optional string syntax = 12;
                12 => syntax_raw = Some(expect_string(&field)?),
                // optional Edition edition = 14;
                14 => edition_raw = Some(expect_int32(&field)?),
                _ => {}
            }
        }

        let syntax = resolve_syntax(syntax_raw.as_deref(), edition_raw)?;

        Ok(ProtoFile {
            name,
            package,
            syntax,
            features,
            dependency,
            messages,
            enums,
        })
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

fn decode_descriptor(bytes: &[u8]) -> Result<MessageDesc> {
    with_decoding_context("DescriptorProto", || {
        let mut name = String::new();
        let mut fields = Vec::new();
        let mut nested_messages = Vec::new();
        let mut nested_enums = Vec::new();
        let mut oneofs = Vec::new();
        let mut map_entry = false;

        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional string name = 1;
                1 => name = expect_string(&field)?,
                // repeated FieldDescriptorProto field = 2;
                2 => {
                    let nested = expect_len(&field)?;
                    fields.push(decode_field(nested)?);
                }
                // repeated DescriptorProto nested_type = 3;
                3 => {
                    let nested = expect_len(&field)?;
                    nested_messages.push(decode_descriptor(nested)?);
                }
                // repeated EnumDescriptorProto enum_type = 4;
                4 => {
                    let nested = expect_len(&field)?;
                    nested_enums.push(decode_enum(nested)?);
                }
                // optional MessageOptions options = 7;
                7 => {
                    let nested = expect_len(&field)?;
                    map_entry = decode_message_options_map_entry(nested)?;
                }
                // repeated OneofDescriptorProto oneof_decl = 8;
                8 => {
                    let nested = expect_len(&field)?;
                    oneofs.push(decode_oneof(nested)?);
                }
                _ => {}
            }
        }

        Ok(MessageDesc {
            name,
            fields,
            nested_messages,
            nested_enums,
            oneofs,
            map_entry,
        })
    })
}

/// `MessageOptions`: only `map_entry` (7) is consumed today.
fn decode_message_options_map_entry(bytes: &[u8]) -> Result<bool> {
    with_decoding_context("MessageOptions", || {
        let mut map_entry = false;
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            // optional bool map_entry = 7;
            if field.field_number.as_u32() == 7 {
                map_entry = expect_bool(&field)?;
            }
        }
        Ok(map_entry)
    })
}

fn decode_field(bytes: &[u8]) -> Result<FieldDesc> {
    with_decoding_context("FieldDescriptorProto", || {
        let mut name = String::new();
        let mut number = 0_i32;
        let mut label = FieldLabel::Optional;
        let mut type_ = FieldType::Int32;
        let mut type_name = None;
        let mut oneof_index = None;
        let mut proto3_optional = false;
        let mut default_value = None;
        let mut packed = None;
        let mut string_layout = None;
        let mut bytes_layout = None;
        let mut features = FeatureSet::default();

        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional string name = 1;
                1 => name = expect_string(&field)?,
                // optional int32 number = 3;
                3 => number = expect_int32(&field)?,
                // optional Label label = 4;
                4 => label = expect_enum(&field)?,
                // optional Type type = 5;
                5 => type_ = expect_enum(&field)?,
                // optional string type_name = 6;
                6 => type_name = Some(ProtoFqn::parse(expect_string(&field)?)),
                // optional string default_value = 7;
                7 => default_value = Some(expect_string(&field)?),
                // optional FieldOptions options = 8;
                8 => {
                    let nested = expect_len(&field)?;
                    let decoded = decode_field_options(nested)?;
                    features = decoded.features;
                    packed = decoded.packed;
                    string_layout = decoded.string_layout;
                    bytes_layout = decoded.bytes_layout;
                }
                // optional int32 oneof_index = 9;
                9 => oneof_index = Some(expect_int32(&field)?),
                // optional bool proto3_optional = 17;
                17 => proto3_optional = expect_bool(&field)?,
                _ => {}
            }
        }

        Ok(FieldDesc {
            name,
            number,
            label,
            type_,
            type_name,
            oneof_index,
            proto3_optional,
            default_value,
            packed,
            string_layout,
            bytes_layout,
            features,
        })
    })
}

/// `FileOptions` / `EnumOptions`: read `features` at `features_field`.
///
/// Field numbers differ by options message:
/// - `FileOptions.features` = 50
/// - `EnumOptions.features` = 7
fn decode_options_features(
    message: &'static str,
    features_field: u32,
    bytes: &[u8],
) -> Result<FeatureSet> {
    with_decoding_context(message, || {
        let mut features = FeatureSet::default();
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            if field.field_number.as_u32() == features_field {
                let nested = expect_len(&field)?;
                features = decode_feature_set(nested)?;
            }
        }
        Ok(features)
    })
}

struct DecodedFieldOptions {
    features: FeatureSet,
    packed: Option<bool>,
    string_layout: Option<StringLayout>,
    bytes_layout: Option<BytesLayout>,
}

/// `FieldOptions`: `packed` (2), `features` (21), and puroro layout extensions.
fn decode_field_options(bytes: &[u8]) -> Result<DecodedFieldOptions> {
    with_decoding_context("FieldOptions", || {
        let mut features = FeatureSet::default();
        let mut packed = None;
        let mut string_layout = None;
        let mut bytes_layout = None;
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional bool packed = 2;
                2 => packed = Some(expect_bool(&field)?),
                // optional FeatureSet features = 21;
                21 => {
                    let nested = expect_len(&field)?;
                    features = decode_feature_set(nested)?;
                }
                // extend FieldOptions { optional StringLayout string_layout = 51400; }
                STRING_LAYOUT_OPTION_NUMBER => {
                    string_layout = Some(expect_enum(&field)?);
                }
                // extend FieldOptions { optional BytesLayout bytes_layout = 51401; }
                BYTES_LAYOUT_OPTION_NUMBER => {
                    bytes_layout = Some(expect_enum(&field)?);
                }
                _ => {}
            }
        }
        Ok(DecodedFieldOptions {
            features,
            packed,
            string_layout,
            bytes_layout,
        })
    })
}

fn decode_feature_set(bytes: &[u8]) -> Result<FeatureSet> {
    with_decoding_context("FeatureSet", || {
        let mut features = FeatureSet::default();
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional FieldPresence field_presence = 1;
                1 => features.field_presence = Some(expect_enum(&field)?),
                // optional EnumType enum_type = 2;
                2 => features.enum_type = Some(expect_enum(&field)?),
                // optional RepeatedFieldEncoding repeated_field_encoding = 3;
                3 => features.repeated_field_encoding = Some(expect_enum(&field)?),
                // optional Utf8Validation utf8_validation = 4;
                4 => features.utf8_validation = Some(expect_enum(&field)?),
                // optional MessageEncoding message_encoding = 5;
                5 => features.message_encoding = Some(expect_enum(&field)?),
                // optional JsonFormat json_format = 6;
                6 => features.json_format = Some(expect_enum(&field)?),
                // optional EnforceNamingStyle enforce_naming_style = 7;
                7 => features.enforce_naming_style = Some(expect_enum(&field)?),
                // optional DefaultSymbolVisibility default_symbol_visibility = 8;
                8 => features.default_symbol_visibility = Some(expect_enum(&field)?),
                // Language-specific FeatureSet extensions (e.g. 1000+) are LEN.
                _ => {}
            }
        }
        Ok(features)
    })
}

fn decode_oneof(bytes: &[u8]) -> Result<OneofDesc> {
    with_decoding_context("OneofDescriptorProto", || {
        let mut name = String::new();
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            // optional string name = 1;
            if field.field_number.as_u32() == 1 {
                name = expect_string(&field)?;
            }
        }
        Ok(OneofDesc { name })
    })
}

fn decode_enum(bytes: &[u8]) -> Result<EnumDesc> {
    with_decoding_context("EnumDescriptorProto", || {
        let mut name = String::new();
        let mut values = Vec::new();
        let mut features = FeatureSet::default();
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional string name = 1;
                1 => name = expect_string(&field)?,
                // repeated EnumValueDescriptorProto value = 2;
                2 => {
                    let nested = expect_len(&field)?;
                    values.push(decode_enum_value(nested)?);
                }
                // optional EnumOptions options = 3;
                3 => {
                    let nested = expect_len(&field)?;
                    features = decode_options_features("EnumOptions", 7, nested)?;
                }
                _ => {}
            }
        }
        Ok(EnumDesc {
            name,
            values,
            features,
        })
    })
}

fn decode_enum_value(bytes: &[u8]) -> Result<EnumValueDesc> {
    with_decoding_context("EnumValueDescriptorProto", || {
        let mut name = String::new();
        let mut number = 0_i32;
        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional string name = 1;
                1 => name = expect_string(&field)?,
                // optional int32 number = 2;
                2 => number = expect_int32(&field)?,
                _ => {}
            }
        }
        Ok(EnumValueDesc { name, number })
    })
}

fn with_decoding_context<T>(message: &'static str, f: impl FnOnce() -> Result<T>) -> Result<T> {
    f().map_err(|e| e.while_decoding(message))
}

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

fn expect_int32(field: &Field<&[u8]>) -> Result<i32> {
    match &field.value {
        FieldValue::Varint(v) => Ok(v.try_to_int32()?),
        _ => Err(Error::unexpected_field(field.field_number.as_u32())),
    }
}

fn expect_enum<T: TryFrom<i32>>(field: &Field<&[u8]>) -> Result<T> {
    let raw = expect_int32(field)?;
    T::try_from(raw).map_err(|_| Error::unexpected_field(field.field_number.as_u32()))
}

fn expect_bool(field: &Field<&[u8]>) -> Result<bool> {
    match &field.value {
        FieldValue::Varint(v) => Ok(v.to_bool()),
        _ => Err(Error::unexpected_field(field.field_number.as_u32())),
    }
}

fn to_field_number(value: u32) -> Result<FieldNumber> {
    FieldNumber::try_new(value).map_err(|value| {
        ProtobufError::FieldNumberOutOfRange {
            value: i64::from(value),
        }
        .into()
    })
}

fn write_len_field(out: &mut Vec<u8>, number: u32, value: &[u8]) -> Result<()> {
    let field = Field::new(to_field_number(number)?, FieldValue::Len(value));
    out.write_protobuf_field(&field)?;
    Ok(())
}

fn write_string_field(out: &mut Vec<u8>, number: u32, value: &str) -> Result<()> {
    write_len_field(out, number, value.as_bytes())
}

fn write_int32_field(out: &mut Vec<u8>, number: u32, value: i32) -> Result<()> {
    let field: Field<&[u8]> = Field::new(to_field_number(number)?, FieldValue::from_int32(value));
    out.write_protobuf_field(&field)?;
    Ok(())
}

fn write_uint64_field(out: &mut Vec<u8>, number: u32, value: u64) -> Result<()> {
    let field: Field<&[u8]> = Field::new(to_field_number(number)?, FieldValue::from_uint64(value));
    out.write_protobuf_field(&field)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::features::{EnumType, FieldPresence};
    use crate::descriptor::{FieldLabel, FieldType};

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
        assert_eq!(decoded.proto_files[0].name, "example.proto");
        assert_eq!(decoded.proto_files[0].package, "example");
        // syntax field omitted → proto2 default
        assert_eq!(decoded.proto_files[0].syntax, Syntax::Proto2);
        assert!(decoded.proto_files[0].features.is_empty());
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
            // repeated File file = 15;
            if field.field_number.as_u32() == 15 {
                let nested = expect_len(&field).unwrap();
                for inner in AsRefExtProtobuf::read_protobuf_fields(&nested) {
                    let inner = inner.unwrap();
                    match inner.field_number.as_u32() {
                        // optional string name = 1;
                        1 => names.push(expect_string(&inner).unwrap()),
                        // optional string content = 15;
                        15 => contents.push(expect_string(&inner).unwrap()),
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
