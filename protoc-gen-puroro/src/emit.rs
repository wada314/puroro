//! Code emission from a resolved schema.
//!
//! Current scope: file-level enums and one root message per file with singular
//! scalar / string / bytes / bool / enum fields. Nested types, repeated, oneof,
//! and message fields are still rejected. All `file_to_generate` entries share
//! one [`ModuleForest`] so cross-file type refs use a single `self::_root`.

use crate::descriptor::CodegenRequest;
use crate::error::{Error, Result};
use crate::field_kind::{MessagePlan, plan_message};
use crate::module_tree::layout::{ModuleLayout, render};
use crate::module_tree::{ModuleForest, ModuleOrigin, type_name_to_module_ident};
use crate::plugin_io::CodeGeneratorResponse;
use crate::resolved::{Arena, FieldOccurrence, File, FileSet, Message, SingularPresence, resolve};
use ::proc_macro2::{Ident, Span};
use ::quote::quote;

mod enumeration;
mod message;
mod type_path;

/// Generate plugin response files from a decoded request.
pub fn emit(request: &CodegenRequest) -> Result<CodeGeneratorResponse> {
    let arena = Arena::new();
    let file_set = resolve(&arena, &request.proto_files)?;

    if request.meta.file_to_generate.is_empty() {
        return Ok(CodeGeneratorResponse::from_files(vec![]));
    }

    let mut targets = Vec::with_capacity(request.meta.file_to_generate.len());
    for target in &request.meta.file_to_generate {
        targets.push(find_file(&file_set, target)?);
    }

    let mut forest = ModuleForest::new();
    write_root_attrs(&mut forest, &targets);

    for file in &targets {
        append_file_to_forest(&mut forest, file)?;
    }

    let path = if targets.len() == 1 {
        proto_path_to_rust_path(targets[0].name())
    } else {
        // One shared forest needs one physical file so `self::_root` stays coherent.
        "lib.rs".into()
    };

    let mut files = render(&forest, &ModuleLayout::SingleFile { path })?;
    let file = files
        .pop()
        .ok_or_else(|| Error::Codegen("layout produced no files".into()))?;
    Ok(CodeGeneratorResponse::from_files(vec![file]))
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

fn write_root_attrs(forest: &mut ModuleForest, targets: &[&File<'_>]) {
    let doc = if targets.len() == 1 {
        format!("@generated from {} — do not edit", targets[0].name())
    } else {
        let names: Vec<&str> = targets.iter().map(|f| f.name()).collect();
        format!("@generated from {} — do not edit", names.join(", "))
    };
    let mut root_items = quote! {
        #![doc = #doc]
        #![allow(clippy::absolute_paths)]
        // Empty messages emit a catch-all-only `match` until field arms exist.
        #![allow(clippy::match_single_binding)]
    };
    let mut packages: Vec<&str> = targets
        .iter()
        .map(|f| f.package())
        .filter(|p| !p.is_empty())
        .collect();
    packages.sort_unstable();
    packages.dedup();
    for package in packages {
        let package_doc = format!("Package `{package}`");
        root_items.extend(quote! {
            #![doc = #package_doc]
        });
    }
    forest.root_mut().append_items(root_items);
}

fn append_file_to_forest(forest: &mut ModuleForest, file: &File<'_>) -> Result<()> {
    let parent = forest.ensure_package(file.package());
    for e in file.enums() {
        if e.parent().is_some() {
            continue;
        }
        parent.append_items(enumeration::render_enum(e)?);
    }

    let mut messages = file.messages();
    let Some(message) = messages.next() else {
        return Ok(());
    };
    let extra = messages.count();
    if extra > 0 {
        return Err(Error::Codegen(format!(
            "generator requires at most one root message per file, found {} in `{}`",
            extra + 1,
            file.name()
        )));
    }

    let message = validate_emit_message(message)?;
    let plan = plan_message(message)?;
    append_message(forest, file, &plan)
}

fn append_message(
    forest: &mut ModuleForest,
    file: &File<'_>,
    plan: &MessagePlan<'_>,
) -> Result<()> {
    let message = plan.message();
    let parent = forest.ensure_package(file.package());
    let mod_name = type_name_to_module_ident(message.name());
    let type_name = Ident::new(message.name(), Span::call_site());

    parent.append_items(quote! {
        pub use #mod_name::#type_name;
    });

    let child = parent.get_or_insert_child(mod_name);
    child.add_origin(ModuleOrigin::Message {
        proto_fqn: message.fqn().clone(),
    });
    child.append_items(message::render_items(plan)?);
    Ok(())
}

/// Nested types / real oneofs rejected; proto3 optional synthetic oneofs OK.
fn validate_emit_message<'a>(message: &'a Message<'a>) -> Result<&'a Message<'a>> {
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
    use crate::descriptor::features::EnumType;
    use crate::descriptor::{
        CodegenMeta, CodegenRequest, Edition, EnumDesc, EnumValueDesc, FeatureSet, FieldDesc,
        FieldLabel, FieldType, MessageDesc, ProtoFile, ProtoFqn, Syntax,
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
            FieldDesc {
                name: "payload".into(),
                number: 4,
                label: FieldLabel::Optional,
                type_: FieldType::Bytes,
                type_name: None,
                oneof_index: None,
                proto3_optional: true,
                packed: None,
                features: FeatureSet::default(),
            },
            FieldDesc {
                name: "zigzag".into(),
                number: 5,
                label: FieldLabel::Optional,
                type_: FieldType::SInt32,
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
        assert!(content.contains("ProtoInt32"));
        assert!(content.contains("ProtoBytes"));
        assert!(content.contains("ProtoSint32"));
        assert!(content.contains("BitPacked"));
    }

    #[test]
    fn emit_open_enum_field() {
        let request = CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec!["t.proto".into()],
                parameter: None,
            },
            proto_files: vec![ProtoFile {
                name: "t.proto".into(),
                package: "demo".into(),
                syntax: Syntax::Proto3,
                features: FeatureSet::default(),
                dependency: vec![],
                messages: vec![MessageDesc {
                    name: "Holder".into(),
                    fields: vec![FieldDesc {
                        name: "status".into(),
                        number: 1,
                        label: FieldLabel::Optional,
                        type_: FieldType::Enum,
                        type_name: Some(ProtoFqn::parse(".demo.Status")),
                        oneof_index: None,
                        proto3_optional: false,
                        packed: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                }],
                enums: vec![EnumDesc {
                    name: "Status".into(),
                    values: vec![
                        EnumValueDesc {
                            name: "STATUS_UNSPECIFIED".into(),
                            number: 0,
                        },
                        EnumValueDesc {
                            name: "STATUS_PENDING".into(),
                            number: 1,
                        },
                    ],
                    features: FeatureSet::default(),
                }],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("pub struct Status"));
        assert!(content.contains("pub const UNSPECIFIED"));
        assert!(content.contains("pub const PENDING"));
        assert!(content.contains("OpenEnum"));
        assert!(content.contains("ProtoEnum"));
        assert!(
            content.contains("self :: _root :: demo :: Status")
                || content.contains("self::_root::demo::Status")
        );
        assert!(content.contains("::puroro_rt::Open"));
        assert!(content.contains("::puroro_rt::Implicit"));
        assert!(content.contains(".optional()"));
    }

    #[test]
    fn emit_cross_file_open_and_closed_enums() {
        let closed = FeatureSet {
            enum_type: Some(EnumType::Closed),
            ..FeatureSet::default()
        };
        let request = CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec![
                    "status.proto".into(),
                    "priority.proto".into(),
                    "holder.proto".into(),
                ],
                parameter: None,
            },
            proto_files: vec![
                ProtoFile {
                    name: "status.proto".into(),
                    package: "demo".into(),
                    syntax: Syntax::Editions(Edition::Edition2023),
                    features: FeatureSet::default(),
                    dependency: vec![],
                    messages: vec![],
                    enums: vec![EnumDesc {
                        name: "Status".into(),
                        values: vec![
                            EnumValueDesc {
                                name: "STATUS_UNSPECIFIED".into(),
                                number: 0,
                            },
                            EnumValueDesc {
                                name: "STATUS_PENDING".into(),
                                number: 1,
                            },
                        ],
                        features: FeatureSet::default(),
                    }],
                },
                ProtoFile {
                    name: "priority.proto".into(),
                    package: "demo".into(),
                    syntax: Syntax::Editions(Edition::Edition2024),
                    features: FeatureSet::default(),
                    dependency: vec![],
                    messages: vec![],
                    enums: vec![EnumDesc {
                        name: "Priority".into(),
                        values: vec![
                            EnumValueDesc {
                                name: "PRIORITY_UNSPECIFIED".into(),
                                number: 0,
                            },
                            EnumValueDesc {
                                name: "PRIORITY_HIGH".into(),
                                number: 1,
                            },
                        ],
                        features: closed,
                    }],
                },
                ProtoFile {
                    name: "holder.proto".into(),
                    package: "demo".into(),
                    syntax: Syntax::Editions(Edition::Edition2023),
                    features: FeatureSet::default(),
                    dependency: vec!["status.proto".into(), "priority.proto".into()],
                    messages: vec![MessageDesc {
                        name: "Holder".into(),
                        fields: vec![
                            FieldDesc {
                                name: "status".into(),
                                number: 1,
                                label: FieldLabel::Optional,
                                type_: FieldType::Enum,
                                type_name: Some(ProtoFqn::parse(".demo.Status")),
                                oneof_index: None,
                                proto3_optional: false,
                                packed: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "priority".into(),
                                number: 2,
                                label: FieldLabel::Optional,
                                type_: FieldType::Enum,
                                type_name: Some(ProtoFqn::parse(".demo.Priority")),
                                oneof_index: None,
                                proto3_optional: false,
                                packed: None,
                                features: FeatureSet::default(),
                            },
                        ],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                    }],
                    enums: vec![],
                },
            ],
        };
        let response = emit(&request).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "lib.rs");
        let content = &response.files[0].content;
        assert!(content.contains("pub struct Status"));
        assert!(content.contains("pub struct Priority"));
        assert!(content.contains("OpenEnum"));
        assert!(content.contains("ClosedEnum"));
        assert!(content.contains("struct Holder"));
        assert!(content.contains("::puroro_rt::Open"));
        assert!(content.contains("::puroro_rt::Closed"));
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
        assert!(err.to_string().contains("at most one root message"));
    }
}
