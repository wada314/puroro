//! Minimal wire codec for `plugin.proto` / `descriptor.proto` subsets.
//!
//! Decodes only the fields needed to build [`crate::descriptor::CodegenRequest`],
//! and encodes a minimal [`CodeGeneratorResponse`].

use crate::descriptor::{
    CodegenMeta, CodegenRequest, EnumDesc, EnumValueDesc, FieldDesc, FieldLabel, FieldType,
    MessageDesc, OneofDesc, ProtoFile,
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
    let mut file_to_generate = Vec::new();
    let mut parameter = None;
    let mut proto_files = Vec::new();

    for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
        let field = field?;
        match field.field_number.as_u32() {
            // file_to_generate = 1
            1 => file_to_generate.push(expect_string("CodeGeneratorRequest", &field)?),
            // parameter = 2
            2 => parameter = Some(expect_string("CodeGeneratorRequest", &field)?),
            // proto_file = 15
            15 => {
                let nested = expect_len("CodeGeneratorRequest", &field)?;
                proto_files.push(decode_file_descriptor(nested)?);
            }
            // compiler_version = 3 — ignored for now
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
}

/// Encode a [`CodeGeneratorResponse`] to wire bytes.
pub fn encode_response(response: &CodeGeneratorResponse) -> Result<Vec<u8>> {
    let mut out = Vec::new();

    if let Some(error) = &response.error {
        write_string_field(&mut out, 1, error)?;
    }
    if response.supported_features != 0 {
        let field: Field<&[u8]> = Field::new(
            FieldNumber::try_from(2)?,
            FieldValue::from_uint64(response.supported_features),
        );
        out.write_protobuf_field(&field)?;
    }
    for file in &response.files {
        let mut nested = Vec::new();
        write_string_field(&mut nested, 1, &file.name)?;
        write_string_field(&mut nested, 15, &file.content)?;
        let field = Field::new(FieldNumber::try_from(15)?, FieldValue::Len(nested));
        out.write_protobuf_field(&field)?;
    }

    Ok(out)
}

fn decode_file_descriptor(bytes: &[u8]) -> Result<ProtoFile> {
    let mut name = String::new();
    let mut package = String::new();
    let mut dependency = Vec::new();
    let mut messages = Vec::new();
    let mut enums = Vec::new();

    for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
        let field = field?;
        match field.field_number.as_u32() {
            1 => name = expect_string("FileDescriptorProto", &field)?,
            2 => package = expect_string("FileDescriptorProto", &field)?,
            3 => dependency.push(expect_string("FileDescriptorProto", &field)?),
            4 => {
                let nested = expect_len("FileDescriptorProto", &field)?;
                messages.push(decode_descriptor(nested)?);
            }
            5 => {
                let nested = expect_len("FileDescriptorProto", &field)?;
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
}

fn decode_descriptor(bytes: &[u8]) -> Result<MessageDesc> {
    let mut name = String::new();
    let mut fields = Vec::new();
    let mut nested_messages = Vec::new();
    let mut nested_enums = Vec::new();
    let mut oneofs = Vec::new();

    for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
        let field = field?;
        match field.field_number.as_u32() {
            1 => name = expect_string("DescriptorProto", &field)?,
            2 => {
                let nested = expect_len("DescriptorProto", &field)?;
                fields.push(decode_field(nested)?);
            }
            3 => {
                let nested = expect_len("DescriptorProto", &field)?;
                nested_messages.push(decode_descriptor(nested)?);
            }
            4 => {
                let nested = expect_len("DescriptorProto", &field)?;
                nested_enums.push(decode_enum(nested)?);
            }
            8 => {
                let nested = expect_len("DescriptorProto", &field)?;
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
}

fn decode_field(bytes: &[u8]) -> Result<FieldDesc> {
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
            1 => name = expect_string("FieldDescriptorProto", &field)?,
            3 => number = expect_int32("FieldDescriptorProto", &field)?,
            4 => {
                let raw = expect_int32("FieldDescriptorProto", &field)?;
                label = FieldLabel::from_i32(raw).ok_or(Error::UnexpectedField {
                    message: "FieldDescriptorProto.label",
                    field_number: 4,
                })?;
            }
            5 => {
                let raw = expect_int32("FieldDescriptorProto", &field)?;
                type_ = FieldType::from_i32(raw).ok_or(Error::UnexpectedField {
                    message: "FieldDescriptorProto.type",
                    field_number: 5,
                })?;
            }
            6 => type_name = Some(expect_string("FieldDescriptorProto", &field)?),
            9 => oneof_index = Some(expect_int32("FieldDescriptorProto", &field)?),
            17 => proto3_optional = expect_bool("FieldDescriptorProto", &field)?,
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
}

fn decode_oneof(bytes: &[u8]) -> Result<OneofDesc> {
    let mut name = String::new();
    for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
        let field = field?;
        if field.field_number.as_u32() == 1 {
            name = expect_string("OneofDescriptorProto", &field)?;
        }
    }
    Ok(OneofDesc { name })
}

fn decode_enum(bytes: &[u8]) -> Result<EnumDesc> {
    let mut name = String::new();
    let mut values = Vec::new();
    for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
        let field = field?;
        match field.field_number.as_u32() {
            1 => name = expect_string("EnumDescriptorProto", &field)?,
            2 => {
                let nested = expect_len("EnumDescriptorProto", &field)?;
                values.push(decode_enum_value(nested)?);
            }
            _ => {}
        }
    }
    Ok(EnumDesc { name, values })
}

fn decode_enum_value(bytes: &[u8]) -> Result<EnumValueDesc> {
    let mut name = String::new();
    let mut number = 0_i32;
    for field in AsRefExtProtobuf::read_protobuf_fields(&bytes) {
        let field = field?;
        match field.field_number.as_u32() {
            1 => name = expect_string("EnumValueDescriptorProto", &field)?,
            2 => number = expect_int32("EnumValueDescriptorProto", &field)?,
            _ => {}
        }
    }
    Ok(EnumValueDesc { name, number })
}

fn expect_len<'a>(message: &'static str, field: &Field<&'a [u8]>) -> Result<&'a [u8]> {
    match &field.value {
        FieldValue::Len(bytes) => Ok(*bytes),
        _ => Err(Error::UnexpectedField {
            message,
            field_number: field.field_number.as_u32(),
        }),
    }
}

fn expect_string(message: &'static str, field: &Field<&[u8]>) -> Result<String> {
    let bytes = expect_len(message, field)?;
    Ok(String::from_utf8(bytes.to_vec())?)
}

fn expect_int32(message: &'static str, field: &Field<&[u8]>) -> Result<i32> {
    match &field.value {
        FieldValue::Varint(v) => Ok(v.try_to_int32()?),
        _ => Err(Error::UnexpectedField {
            message,
            field_number: field.field_number.as_u32(),
        }),
    }
}

fn expect_bool(message: &'static str, field: &Field<&[u8]>) -> Result<bool> {
    match &field.value {
        FieldValue::Varint(v) => Ok(v.to_bool()),
        _ => Err(Error::UnexpectedField {
            message,
            field_number: field.field_number.as_u32(),
        }),
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
            if field.field_number.as_u32() == 15 {
                let nested = expect_len("CodeGeneratorResponse.File", &field).unwrap();
                for inner in AsRefExtProtobuf::read_protobuf_fields(&nested) {
                    let inner = inner.unwrap();
                    match inner.field_number.as_u32() {
                        1 => names.push(expect_string("File", &inner).unwrap()),
                        15 => contents.push(expect_string("File", &inner).unwrap()),
                        _ => {}
                    }
                }
            }
        }
        assert_eq!(names, vec!["example.rs"]);
        assert_eq!(contents, vec!["// hello\n"]);
    }
}
