//! Code emission from codegen IR.
//!
//! Current scope is a **fake** generator: only a single field-less root message
//! is supported. That is enough to exercise the request → IR → Rust → compile
//! → runtime pipeline before real catalog emission exists.

use crate::error::{Error, Result};
use crate::ir::{CodegenRequest, MessageDesc, ProtoFile};
use crate::plugin_io::{CodeGeneratorResponse, ResponseFile};

mod empty_message;

/// Generate plugin response files from a decoded request.
pub fn emit(request: &CodegenRequest) -> Result<CodeGeneratorResponse> {
    let mut files = Vec::new();

    for target in &request.file_to_generate {
        let proto = request
            .proto_files
            .iter()
            .find(|f| f.name == *target)
            .ok_or_else(|| {
                Error::Codegen(format!(
                    "file_to_generate `{target}` was not present in proto_file"
                ))
            })?;
        files.push(emit_proto_file(proto)?);
    }

    Ok(CodeGeneratorResponse::from_files(files))
}

fn emit_proto_file(proto: &ProtoFile) -> Result<ResponseFile> {
    validate_fake_proto(proto)?;
    let message = &proto.messages[0];
    let content = empty_message::render(proto, message);

    Ok(ResponseFile {
        name: proto_path_to_rust_path(&proto.name),
        content,
    })
}

/// Fake generator limits: one root message, no fields / nested types / enums.
fn validate_fake_proto(proto: &ProtoFile) -> Result<()> {
    if !proto.enums.is_empty() {
        return Err(Error::Codegen(
            "fake generator does not support file-level enums yet".into(),
        ));
    }
    if proto.messages.len() != 1 {
        return Err(Error::Codegen(format!(
            "fake generator requires exactly one root message, found {}",
            proto.messages.len()
        )));
    }
    validate_empty_message(&proto.messages[0])
}

fn validate_empty_message(message: &MessageDesc) -> Result<()> {
    if !is_simple_ident(&message.name) {
        return Err(Error::Codegen(format!(
            "message name `{}` is not a simple Rust identifier",
            message.name
        )));
    }
    if !message.fields.is_empty()
        || !message.nested_messages.is_empty()
        || !message.nested_enums.is_empty()
        || !message.oneofs.is_empty()
    {
        return Err(Error::Codegen(format!(
            "fake generator only supports field-less message `{}`",
            message.name
        )));
    }
    Ok(())
}

fn is_simple_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

/// Map `foo/bar/baz.proto` → `foo/bar/baz.rs`.
fn proto_path_to_rust_path(proto_name: &str) -> String {
    if let Some(stem) = proto_name.strip_suffix(".proto") {
        format!("{stem}.rs")
    } else {
        format!("{proto_name}.rs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{FieldDesc, FieldLabel, FieldType, MessageDesc, ProtoFile};

    fn empty_request(message_name: &str) -> CodegenRequest {
        CodegenRequest {
            file_to_generate: vec!["empty.proto".into()],
            parameter: None,
            proto_files: vec![ProtoFile {
                name: "empty.proto".into(),
                package: String::new(),
                dependency: vec![],
                messages: vec![MessageDesc {
                    name: message_name.into(),
                    fields: vec![],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                }],
                enums: vec![],
            }],
        }
    }

    #[test]
    fn emit_empty_message_mentions_type_name() {
        let response = emit(&empty_request("Empty")).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "empty.rs");
        assert!(response.files[0].content.contains("struct Empty"));
        assert!(
            response.files[0]
                .content
                .contains("@generated from empty.proto")
        );
    }

    #[test]
    fn reject_message_with_fields() {
        let mut request = empty_request("Task");
        request.proto_files[0].messages[0].fields.push(FieldDesc {
            name: "title".into(),
            number: 1,
            label: FieldLabel::Optional,
            type_: FieldType::String,
            type_name: None,
            oneof_index: None,
            proto3_optional: false,
        });
        let err = emit(&request).unwrap_err();
        assert!(err.to_string().contains("field-less"));
    }

    #[test]
    fn reject_multiple_root_messages() {
        let mut request = empty_request("Empty");
        request.proto_files[0].messages.push(MessageDesc {
            name: "Other".into(),
            fields: vec![],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
        });
        let err = emit(&request).unwrap_err();
        assert!(err.to_string().contains("exactly one root message"));
    }
}
