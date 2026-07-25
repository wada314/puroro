//! Code emission from a resolved schema.
//!
//! Current scope: one root message per file, singular scalar / string / bytes /
//! bool fields. Nested types, enums, repeated, oneof, and message fields are
//! still rejected. Emission goes through [`resolve`] + [`plan_message`].

use crate::descriptor::CodegenRequest;
use crate::error::{Error, Result};
use crate::field_kind::{MessagePlan, plan_message};
use crate::module_tree::layout::{ModuleLayout, render};
use crate::module_tree::{ModuleForest, ModuleOrigin, type_name_to_module_ident};
use crate::plugin_io::{CodeGeneratorResponse, ResponseFile};
use crate::resolved::{Arena, FieldOccurrence, File, FileSet, Message, SingularPresence, resolve};
use ::proc_macro2::{Ident, Span};
use ::quote::quote;

mod message;

/// Generate plugin response files from a decoded request.
pub fn emit(request: &CodegenRequest) -> Result<CodeGeneratorResponse> {
    let arena = Arena::new();
    let file_set = resolve(&arena, &request.proto_files)?;

    let mut files = Vec::new();
    for target in &request.meta.file_to_generate {
        let file = find_file(&file_set, target)?;
        files.push(emit_resolved_file(file)?);
    }

    Ok(CodeGeneratorResponse::from_files(files))
}

fn find_file<'a>(file_set: &FileSet<'a>, target: &str) -> Result<&'a File<'a>> {
    file_set
        .files()
        .find(|f| f.name() == target)
        .ok_or_else(|| {
            Error::Codegen(format!(
                "file_to_generate `{target}` was not present in proto_file"
            ))
        })
}

fn emit_resolved_file<'a>(file: &'a File<'a>) -> Result<ResponseFile> {
    let message = validate_emit_file(file)?;
    let plan = plan_message(message)?;
    let forest = build_forest(file, &plan)?;

    let mut files = render(
        &forest,
        &ModuleLayout::SingleFile {
            path: proto_path_to_rust_path(file.name()),
        },
    )?;
    files
        .pop()
        .ok_or_else(|| Error::Codegen("layout produced no files".into()))
}

fn build_forest(file: &File<'_>, plan: &MessagePlan<'_>) -> Result<ModuleForest> {
    let message = plan.message();
    let mut forest = ModuleForest::new();

    // Root carries file-level docs / inner attributes.
    let doc = format!("@generated from {} — do not edit", file.name());
    let mut root_items = quote! {
        #![doc = #doc]
        #![allow(clippy::absolute_paths)]
        // Empty messages emit a catch-all-only `match` until field arms exist.
        #![allow(clippy::match_single_binding)]
    };
    if !file.package().is_empty() {
        let package_doc = format!("Package `{}`", file.package());
        root_items.extend(quote! {
            #![doc = #package_doc]
        });
    }
    forest.root_mut().append_items(root_items);

    let parent = forest.ensure_package(file.package());
    let mod_name = type_name_to_module_ident(message.name());
    let type_name = Ident::new(message.name(), Span::call_site());

    // Main message type is visible from the parent namespace.
    parent.append_items(quote! {
        pub use #mod_name::#type_name;
    });

    let child = parent.get_or_insert_child(mod_name);
    child.add_origin(ModuleOrigin::Message {
        proto_fqn: message.fqn().clone(),
    });
    child.append_items(message::render_items(plan)?);

    Ok(forest)
}

/// One root message; no nested types / file-level enums / real oneofs.
///
/// Proto3 `optional` synthetic oneofs are allowed — they appear in
/// `message.oneofs()` but fields keep [`SingularPresence::Explicit`].
/// Field support is enforced by [`message::render_items`].
fn validate_emit_file<'a>(file: &'a File<'a>) -> Result<&'a Message<'a>> {
    if file.enums().next().is_some() {
        return Err(Error::Codegen(
            "generator does not support file-level enums yet".into(),
        ));
    }

    let mut messages = file.messages();
    let Some(message) = messages.next() else {
        return Err(Error::Codegen(
            "generator requires exactly one root message, found 0".into(),
        ));
    };
    let extra = messages.count();
    if extra > 0 {
        return Err(Error::Codegen(format!(
            "generator requires exactly one root message, found {}",
            extra + 1
        )));
    }

    if !is_simple_ident(message.name()) {
        return Err(Error::Codegen(format!(
            "message name `{}` is not a simple Rust identifier",
            message.name()
        )));
    }
    if message.nested_messages().next().is_some() || message.nested_enums().next().is_some() {
        return Err(Error::Codegen(format!(
            "generator does not support nested types on message `{}` yet",
            message.name()
        )));
    }
    if message.fields().any(|f| {
        matches!(
            f.occurrence(),
            FieldOccurrence::Singular(SingularPresence::Oneof)
        )
    }) {
        return Err(Error::Codegen(format!(
            "generator does not support oneofs on message `{}` yet",
            message.name()
        )));
    }
    Ok(message)
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
    use crate::descriptor::{
        CodegenMeta, CodegenRequest, FeatureSet, FieldDesc, FieldLabel, FieldType, MessageDesc,
        ProtoFile, Syntax,
    };

    fn empty_request(message_name: &str) -> CodegenRequest {
        empty_request_with_package(message_name, "")
    }

    fn empty_request_with_package(message_name: &str, package: &str) -> CodegenRequest {
        CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec!["empty.proto".into()],
                parameter: None,
            },
            proto_files: vec![ProtoFile {
                name: "empty.proto".into(),
                package: package.into(),
                syntax: Syntax::Proto3,
                features: FeatureSet::default(),
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
        assert!(response.files[0].content.contains("pub use empty"));
        assert!(
            response.files[0]
                .content
                .contains("@generated from empty.proto")
        );
    }

    #[test]
    fn emit_empty_message_nests_package_modules() {
        let response = emit(&empty_request_with_package("Empty", "example.v1")).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("pub mod example"));
        assert!(content.contains("pub mod v1"));
        assert!(content.contains("pub mod empty"));
        assert!(content.contains("pub use empty::Empty"));
        assert!(content.contains("struct Empty"));
        // Re-export should sit under the package leaf, not the forest root.
        let example_idx = content.find("pub mod example").expect("example mod");
        let use_idx = content.find("pub use empty::Empty").expect("pub use");
        assert!(
            use_idx > example_idx,
            "pub use should appear inside the package module tree"
        );
    }

    #[test]
    fn emit_singular_scalar_fields() {
        let mut request = empty_request("Scalars");
        request.meta.file_to_generate = vec!["scalars.proto".into()];
        request.proto_files[0].name = "scalars.proto".into();
        request.proto_files[0].messages[0].name = "Scalars".into();
        request.proto_files[0].messages[0].fields = vec![
            FieldDesc {
                name: "score".into(),
                number: 1,
                label: FieldLabel::Optional,
                type_: FieldType::Int32,
                type_name: None,
                oneof_index: None,
                proto3_optional: false,
                packed: None,
                features: FeatureSet::default(),
            },
            FieldDesc {
                name: "title".into(),
                number: 2,
                label: FieldLabel::Optional,
                type_: FieldType::String,
                type_name: None,
                oneof_index: None,
                proto3_optional: true,
                packed: None,
                features: FeatureSet::default(),
            },
            FieldDesc {
                name: "done".into(),
                number: 3,
                label: FieldLabel::Optional,
                type_: FieldType::Bool,
                type_name: None,
                oneof_index: None,
                proto3_optional: false,
                packed: None,
                features: FeatureSet::default(),
            },
        ];
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("struct Scalars"));
        assert!(content.contains("pub const FIELD_SCORE"));
        assert!(content.contains("pub const FIELD_TITLE"));
        assert!(content.contains("pub const BIT_TITLE"));
        assert!(content.contains("pub const BIT_DONE_VALUE"));
        assert!(content.contains("ProtoInt32"));
        assert!(content.contains("ProtoString"));
        assert!(content.contains("ProtoBool"));
        assert!(content.contains("BitPacked"));
        assert!(content.contains("fn score("));
        assert!(content.contains(".optional()"));
        assert!(content.contains("BIT_DONE_VALUE"));
        assert!(content.contains("FIELD_SCORE =>"));
    }

    #[test]
    fn reject_repeated_field() {
        let mut request = empty_request("Task");
        request.proto_files[0].messages[0].fields.push(FieldDesc {
            name: "tag_ids".into(),
            number: 1,
            label: FieldLabel::Repeated,
            type_: FieldType::Int32,
            type_name: None,
            oneof_index: None,
            proto3_optional: false,
            packed: None,
            features: FeatureSet::default(),
        });
        let err = emit(&request).unwrap_err();
        assert!(err.to_string().contains("repeated"));
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
