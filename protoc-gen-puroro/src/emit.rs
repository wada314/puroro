//! Code emission from codegen IR.
//!
//! This is intentionally a **skeleton**: it produces placeholder Rust modules
//! that prove the request → IR → files pipeline works. Real catalog emission
//! comes later.

use crate::error::{Error, Result};
use crate::ir::{CodegenRequest, MessageDesc, ProtoFile};
use crate::plugin_io::{CodeGeneratorResponse, ResponseFile};

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
    let rust_path = proto_path_to_rust_path(&proto.name);
    let mut body = String::new();
    body.push_str(&format!(
        "//! @generated from {} — do not edit\n",
        proto.name
    ));
    if !proto.package.is_empty() {
        body.push_str(&format!("//! Package `{}`\n", proto.package));
    }
    body.push('\n');
    body.push_str("// Skeleton output from protoc-gen-puroro.\n");
    body.push_str("// Real message / enum emission is not implemented yet.\n\n");

    for message in &proto.messages {
        append_message_outline(&mut body, message, 0);
    }
    for enum_desc in &proto.enums {
        body.push_str(&format!("// enum {}\n", enum_desc.name));
    }

    Ok(ResponseFile {
        name: rust_path,
        content: body,
    })
}

fn append_message_outline(out: &mut String, message: &MessageDesc, depth: usize) {
    let indent = "  ".repeat(depth);
    out.push_str(&format!("{indent}// message {}\n", message.name));
    for field in &message.fields {
        out.push_str(&format!(
            "{indent}//   field {} = {} ({:?} / {:?})\n",
            field.name, field.number, field.label, field.type_
        ));
    }
    for nested in &message.nested_messages {
        append_message_outline(out, nested, depth + 1);
    }
    for nested_enum in &message.nested_enums {
        out.push_str(&format!("{indent}//   enum {}\n", nested_enum.name));
    }
    for oneof in &message.oneofs {
        out.push_str(&format!("{indent}//   oneof {}\n", oneof.name));
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
    use crate::ir::{
        EnumDesc, EnumValueDesc, FieldDesc, FieldLabel, FieldType, MessageDesc, ProtoFile,
    };

    #[test]
    fn emit_lists_messages() {
        let request = CodegenRequest {
            file_to_generate: vec!["example.proto".into()],
            parameter: None,
            proto_files: vec![ProtoFile {
                name: "example.proto".into(),
                package: "example".into(),
                dependency: vec![],
                messages: vec![MessageDesc {
                    name: "Task".into(),
                    fields: vec![FieldDesc {
                        name: "title".into(),
                        number: 1,
                        label: FieldLabel::Optional,
                        type_: FieldType::String,
                        type_name: None,
                        oneof_index: None,
                        proto3_optional: false,
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                }],
                enums: vec![EnumDesc {
                    name: "Priority".into(),
                    values: vec![EnumValueDesc {
                        name: "PRIORITY_UNSPECIFIED".into(),
                        number: 0,
                    }],
                }],
            }],
        };

        let response = emit(&request).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "example.rs");
        let content = &response.files[0].content;
        assert!(content.contains("@generated from example.proto"));
        assert!(content.contains("Package `example`"));
        assert!(content.contains("message Task"));
        assert!(content.contains("field title = 1"));
        assert!(content.contains("enum Priority"));
    }
}
