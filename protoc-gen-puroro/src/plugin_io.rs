//! Minimal wire codec for `plugin.proto` / `descriptor.proto` subsets.
//!
//! Decodes only the fields needed to build [`crate::descriptor::CodegenRequest`],
//! and encodes a minimal [`CodeGeneratorResponse`].

use crate::descriptor::{
    CodegenMeta, CodegenRequest, EnumDesc, EnumValueDesc, FieldDesc, FieldLabel, FieldType,
    MessageDesc, OneofDesc, ProtoFile, ProtoFqn,
};
use crate::error::{Error, Result};
use ::protobuf_core::{AsRefExtProtobuf, Field, FieldNumber, FieldValue, WriteExtProtobuf};

/// `CodeGeneratorResponse.FEATURE_PROTO3_OPTIONAL`
pub const FEATURE_PROTO3_OPTIONAL: u64 = 1;

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
    pub files: Vec<ResponseFile>,
}

impl CodeGeneratorResponse {
    pub fn from_files(files: Vec<ResponseFile>) -> Self {
        Self {
            error: None,
            supported_features: FEATURE_PROTO3_OPTIONAL,
            files,
        }
    }

    pub fn from_error(message: impl Into<String>) -> Self {
        Self {
            error: Some(message.into()),
            supported_features: FEATURE_PROTO3_OPTIONAL,
            files: Vec::new(),
        }
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

/// Encode a [`CodeGeneratorResponse`] to wire bytes.
pub fn encode_response(response: &CodeGeneratorResponse) -> Result<Vec<u8>> {
    let mut out = Vec::new();

    if let Some(error) = &response.error {
        // optional string error = 1;
        write_string_field(&mut out, 1, error)?;
    }
    if response.supported_features != 0 {
        // optional uint64 supported_features = 2;
        let field: Field<&[u8]> = Field::new(
            FieldNumber::try_from(2)?,
            FieldValue::from_uint64(response.supported_features),
        );
        out.write_protobuf_field(&field)?;
    }
    for file in &response.files {
        let mut nested = Vec::new();
        // CodeGeneratorResponse.File.name = 1;
        write_string_field(&mut nested, 1, &file.name)?;
        // CodeGeneratorResponse.File.content = 15;
        write_string_field(&mut nested, 15, &file.content)?;
        // repeated File file = 15;
        let field = Field::new(FieldNumber::try_from(15)?, FieldValue::Len(nested));
        out.write_protobuf_field(&field)?;
    }

    Ok(out)
}

fn decode_file_descriptor(bytes: &[u8]) -> Result<ProtoFile> {
    with_decoding_context("FileDescriptorProto", || {
        let mut name = String::new();
        let mut package = String::new();
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
                _ => {}
            }
        }

        Ok(ProtoFile {
            name,
            package,
            dependency,
            messages,
            enums,
        })
    })
}

fn decode_descriptor(bytes: &[u8]) -> Result<MessageDesc> {
    with_decoding_context("DescriptorProto", || {
        let mut name = String::new();
        let mut fields = Vec::new();
        let mut nested_messages = Vec::new();
        let mut nested_enums = Vec::new();
        let mut oneofs = Vec::new();

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
        })
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

        for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
            let field = field?;
            match field.field_number.as_u32() {
                // optional string name = 1;
                1 => name = expect_string(&field)?,
                // optional int32 number = 3;
                3 => number = expect_int32(&field)?,
                // optional Label label = 4;
                4 => {
                    let raw = expect_int32(&field)?;
                    label = FieldLabel::from_i32(raw).ok_or(Error::unexpected_field(4))?;
                }
                // optional Type type = 5;
                5 => {
                    let raw = expect_int32(&field)?;
                    type_ = FieldType::from_i32(raw).ok_or(Error::unexpected_field(5))?;
                }
                // optional string type_name = 6;
                6 => type_name = Some(ProtoFqn::parse(expect_string(&field)?)),
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
        })
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
                _ => {}
            }
        }
        Ok(EnumDesc { name, values })
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

fn expect_bool(field: &Field<&[u8]>) -> Result<bool> {
    match &field.value {
        FieldValue::Varint(v) => Ok(v.to_bool()),
        _ => Err(Error::unexpected_field(field.field_number.as_u32())),
    }
}

fn write_string_field(out: &mut Vec<u8>, field_number: u32, value: &str) -> Result<()> {
    let field = Field::new(
        FieldNumber::try_from(field_number)?,
        FieldValue::Len(value.as_bytes()),
    );
    out.write_protobuf_field(&field)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::{FieldLabel, FieldType};

    fn encode_string_field(field_number: u32, value: &str) -> Vec<u8> {
        let mut out = Vec::new();
        write_string_field(&mut out, field_number, value).unwrap();
        out
    }

    fn encode_message_field(field_number: u32, nested: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let field = Field::new(
            FieldNumber::try_from(field_number).unwrap(),
            FieldValue::Len(nested),
        );
        out.write_protobuf_field(&field).unwrap();
        out
    }

    fn encode_varint_field(field_number: u32, value: i32) -> Vec<u8> {
        let mut out = Vec::new();
        let field: Field<&[u8]> = Field::new(
            FieldNumber::try_from(field_number).unwrap(),
            FieldValue::from_int32(value),
        );
        out.write_protobuf_field(&field).unwrap();
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
    fn encode_response_round_trips_files() {
        let response = CodeGeneratorResponse::from_files(vec![ResponseFile {
            name: "example.rs".into(),
            content: "// hello\n".into(),
        }]);
        let bytes = encode_response(&response).unwrap();

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
}
