//! Code emission from a resolved schema.
//!
//! Current scope: file-level and nested enums/messages with singular / repeated
//! scalar / string / bytes / bool / enum / message fields, real oneofs, and
//! maps with legal keys and Copy scalar / bool values. All `file_to_generate`
//! entries share one [`ModuleForest`] so cross-file type refs use a single
//! `self::_root`.

use crate::descriptor::CodegenRequest;
use crate::error::{Error, Result};
use crate::field_kind::{MessagePlan, plan_message};
use crate::module_tree::layout::{ModuleLayout, render};
use crate::module_tree::{ModuleForest, ModuleNode, ModuleOrigin, type_name_to_module_ident};
use crate::plugin_io::CodeGeneratorResponse;
use crate::resolved::{Arena, File, Message, resolve};
use ::proc_macro2::{Ident, Span};
use ::quote::quote;
use ::syn::{Attribute, Item, parse_quote};

mod defaults;
mod enumeration;
mod ident;
mod message;
mod oneof;
mod parse;
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
        let file = file_set.file(target).ok_or_else(|| {
            Error::Codegen(format!(
                "file_to_generate `{target}` was not present in proto_file"
            ))
        })?;
        targets.push(file);
    }

    let mut forest = ModuleForest::new();
    forest
        .root_mut()
        .append_inner_attrs(generated_file_attrs(&targets)?);

    for file in &targets {
        install_file(&mut forest, file)?;
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

/// Crate-level inner attributes (`//!` / `/*!` header and clippy allows).
///
/// Tooling only needs the `@generated` substring near the top of the file.
/// Source `.proto` paths go in the same doc string so they stay one attribute;
/// prettyplease prints a multiline doc as `/*! … */`.
fn generated_file_attrs(targets: &[&File<'_>]) -> Result<Vec<Attribute>> {
    let files = targets
        .iter()
        .map(|file| format!("- `{}`", file.name()))
        .collect::<Vec<_>>()
        .join("\n");
    let doc = format!("\n@generated — do not edit\n{files}\n");
    let root_tokens = quote! {
        #![doc = #doc]
        #![allow(clippy::absolute_paths)]
        // Empty messages emit a catch-all-only `match` until field arms exist.
        #![allow(clippy::match_single_binding)]
    };
    Ok(parse::parse_file(root_tokens)?.attrs)
}

/// Constructed message module ready to install into a package or enclosing message module.
struct EmittedMessage {
    pub_use: Item,
    module_name: Ident,
    origin: ModuleOrigin,
    items: Vec<Item>,
    nested: Vec<EmittedMessage>,
}

fn emit_message(plan: &MessagePlan<'_>) -> Result<EmittedMessage> {
    let message = plan.message();
    let module_name = type_name_to_module_ident(message.name());
    let type_name = Ident::new(message.name(), Span::call_site());
    let pub_use = parse_quote! {
        pub use #module_name::#type_name;
    };

    let nested_messages: Vec<_> = message
        .nested_messages()
        .filter(|m| !m.is_map_entry())
        .collect();
    for nested in &nested_messages {
        validate_emit_message(nested)?;
    }

    let mut items = Vec::new();
    for e in message.nested_enums() {
        items.extend(enumeration::render_enum(e)?);
    }
    items.extend(message::render_items(plan)?);

    let mut nested = Vec::with_capacity(nested_messages.len());
    for nested_msg in nested_messages {
        nested.push(emit_message(&plan_message(nested_msg)?)?);
    }

    Ok(EmittedMessage {
        pub_use,
        module_name,
        origin: ModuleOrigin::Message {
            proto_fqn: message.fqn().clone(),
        },
        items,
        nested,
    })
}

fn install_file(forest: &mut ModuleForest, file: &File<'_>) -> Result<()> {
    let package = forest.ensure_package(file.package());
    package.append_items(
        file.enums()
            .filter(|e| e.parent().is_none())
            .map(enumeration::render_enum)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten(),
    );
    for message in file.messages() {
        validate_emit_message(message)?;
        install_message(package, emit_message(&plan_message(message)?)?);
    }
    Ok(())
}

fn install_message(module: &mut ModuleNode, emitted: EmittedMessage) {
    module.append_items([emitted.pub_use]);
    let message_mod = module.get_or_insert_child(emitted.module_name);
    message_mod.add_origin(emitted.origin);
    message_mod.append_items(emitted.items);
    for nested in emitted.nested {
        install_message(message_mod, nested);
    }
}

/// Nested type declarations are emitted into this message's module.
/// Proto3 optional synthetic oneofs resolve as Explicit and need no special case.
fn validate_emit_message(message: &Message<'_>) -> Result<()> {
    if !ident::is_simple_ident(message.name()) {
        return Err(Error::Codegen(format!(
            "message name `{}` is not a simple Rust identifier",
            message.name()
        )));
    }
    Ok(())
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
    use crate::descriptor::features::{EnumType, Utf8Validation};
    use crate::descriptor::{
        BytesLayout, CodegenMeta, CodegenRequest, Edition, EnumDesc, EnumValueDesc, FeatureSet,
        FieldDesc, FieldLabel, FieldType, MessageDesc, OneofDesc, ProtoFile, ProtoFqn,
        StringLayout, Syntax,
    };
    use crate::plugin_io::decode_request;
    use ::protobuf_core::{AsRefExtProtobuf, Field, FieldNumber, FieldValue, WriteExtProtobuf};
    use ::std::env;
    use ::std::fs;
    use ::std::path::PathBuf;
    use ::std::process::{self, Command};
    use ::std::thread;

    /// Successful `emit` results are also written under `generated-preview/`
    /// (gitignored) so the pretty-printed Rust can be inspected. Overwritten
    /// whenever that test runs.
    fn emit(request: &CodegenRequest) -> Result<CodeGeneratorResponse> {
        let result = super::emit(request);
        if let Ok(response) = &result {
            write_generated_preview(response);
        }
        result
    }

    fn write_generated_preview(response: &CodeGeneratorResponse) {
        if response.files.is_empty() {
            return;
        }
        let Some(thread_name) = thread::current().name().map(str::to_owned) else {
            return;
        };
        let test = thread_name
            .strip_prefix("emit::tests::")
            .unwrap_or(&thread_name);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("generated-preview")
            .join(test);
        fs::create_dir_all(&dir).unwrap_or_else(|e| {
            panic!(
                "failed to create generated preview dir {}: {e}",
                dir.display()
            )
        });
        for file in &response.files {
            let path = dir.join(&file.name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap_or_else(|e| {
                    panic!(
                        "failed to create generated preview parent {}: {e}",
                        parent.display()
                    )
                });
            }
            fs::write(&path, &file.content).unwrap_or_else(|e| {
                panic!("failed to write generated preview {}: {e}", path.display())
            });
        }
    }

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
                    map_entry: false,
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
        assert!(response.files[0].content.contains("@generated"));
        assert!(response.files[0].content.contains("- `empty.proto`"));
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
                default_value: None,
                packed: None,
                string_layout: None,
                bytes_layout: None,
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
                default_value: None,
                packed: None,
                string_layout: None,
                bytes_layout: None,
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
                default_value: None,
                packed: None,
                string_layout: None,
                bytes_layout: None,
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
                default_value: None,
                packed: None,
                string_layout: None,
                bytes_layout: None,
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
                default_value: None,
                packed: None,
                string_layout: None,
                bytes_layout: None,
                features: FeatureSet::default(),
            },
        ];
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("struct Scalars"));
        assert!(content.contains("pub const FIELD_SCORE"));
        assert!(content.contains("ProtoInt32"));
        assert!(content.contains("ProtoBytes"));
        assert!(content.contains("ProtoSInt32"));
        assert!(content.contains("BitPacked"));
        assert!(content.contains("BIT_PAYLOAD_SSO"));
        assert!(content.contains("impl ::puroro::BytesMut<A>"));
    }

    #[test]
    fn emit_string_layout_heap_uses_inline_not_sso() {
        let mut request = empty_request("HeapString");
        request.meta.file_to_generate = vec!["t.proto".into()];
        request.proto_files[0].name = "t.proto".into();
        request.proto_files[0].messages[0].name = "HeapString".into();
        request.proto_files[0].messages[0].fields = vec![FieldDesc {
            name: "body".into(),
            number: 1,
            label: FieldLabel::Optional,
            type_: FieldType::String,
            type_name: None,
            oneof_index: None,
            proto3_optional: true,
            default_value: None,
            packed: None,
            string_layout: Some(StringLayout::Heap),
            bytes_layout: None,
            features: FeatureSet::default(),
        }];
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("ProtoString"));
        assert!(
            !content.contains("InlineOrHeap"),
            "string_layout=HEAP must not emit SSO layout: {content}"
        );
        assert!(
            !content.contains("BIT_BODY_SSO"),
            "string_layout=HEAP must not allocate an SSO bit: {content}"
        );
        assert!(content.contains("impl ::core::ops::DerefMut<Target = ::puroro::String<A>>"));
    }

    #[test]
    fn emit_bytes_layout_heap_uses_inline_not_sso() {
        let mut request = empty_request("HeapBytes");
        request.meta.file_to_generate = vec!["t.proto".into()];
        request.proto_files[0].name = "t.proto".into();
        request.proto_files[0].messages[0].name = "HeapBytes".into();
        request.proto_files[0].messages[0].fields = vec![FieldDesc {
            name: "body".into(),
            number: 1,
            label: FieldLabel::Optional,
            type_: FieldType::Bytes,
            type_name: None,
            oneof_index: None,
            proto3_optional: true,
            default_value: None,
            packed: None,
            string_layout: None,
            bytes_layout: Some(BytesLayout::Heap),
            features: FeatureSet::default(),
        }];
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("ProtoBytes"));
        assert!(
            !content.contains("InlineOrHeap"),
            "bytes_layout=HEAP must not emit SSO layout: {content}"
        );
        assert!(
            !content.contains("BIT_BODY_SSO"),
            "bytes_layout=HEAP must not allocate an SSO bit: {content}"
        );
        assert!(
            content.contains("DerefMut") && content.contains("::allocator_api2::vec::Vec<u8, A>"),
            "bytes_layout=HEAP must emit DerefMut to Vec: {content}"
        );
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
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
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
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
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
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                        ],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                        map_entry: false,
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
    fn emit_repeated_and_nested_types() {
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
                    name: "Outer".into(),
                    fields: vec![
                        FieldDesc {
                            name: "tag_ids".into(),
                            number: 1,
                            label: FieldLabel::Repeated,
                            type_: FieldType::Int32,
                            type_name: None,
                            oneof_index: None,
                            proto3_optional: false,
                            default_value: None,
                            packed: None,
                            string_layout: None,
                            bytes_layout: None,
                            features: FeatureSet::default(),
                        },
                        FieldDesc {
                            name: "inners".into(),
                            number: 2,
                            label: FieldLabel::Repeated,
                            type_: FieldType::Message,
                            type_name: Some(ProtoFqn::parse(".demo.Outer.Inner")),
                            oneof_index: None,
                            proto3_optional: false,
                            default_value: None,
                            packed: None,
                            string_layout: None,
                            bytes_layout: None,
                            features: FeatureSet::default(),
                        },
                        FieldDesc {
                            name: "kind".into(),
                            number: 3,
                            label: FieldLabel::Optional,
                            type_: FieldType::Enum,
                            type_name: Some(ProtoFqn::parse(".demo.Outer.Kind")),
                            oneof_index: None,
                            proto3_optional: false,
                            default_value: None,
                            packed: None,
                            string_layout: None,
                            bytes_layout: None,
                            features: FeatureSet::default(),
                        },
                    ],
                    nested_messages: vec![MessageDesc {
                        name: "Inner".into(),
                        fields: vec![],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                        map_entry: false,
                    }],
                    nested_enums: vec![EnumDesc {
                        name: "Kind".into(),
                        values: vec![
                            EnumValueDesc {
                                name: "KIND_UNSPECIFIED".into(),
                                number: 0,
                            },
                            EnumValueDesc {
                                name: "KIND_A".into(),
                                number: 1,
                            },
                        ],
                        features: FeatureSet::default(),
                    }],
                    oneofs: vec![],
                    map_entry: false,
                }],
                enums: vec![],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("RepeatedField"));
        assert!(content.contains("::puroro_rt::Packed"));
        assert!(content.contains("pub struct Inner"));
        assert!(content.contains("pub struct Kind"));
        assert!(
            content.contains("self :: _root :: demo :: outer :: inner :: Inner")
                || content.contains("self::_root::demo::outer::inner::Inner")
        );
        assert!(
            content.contains("self :: _root :: demo :: outer :: Kind")
                || content.contains("self::_root::demo::outer::Kind")
        );
    }

    #[test]
    fn reject_open_enum_without_leading_zero() {
        let mut request = empty_request("Holder");
        request.proto_files[0].enums.push(EnumDesc {
            name: "Kind".into(),
            values: vec![EnumValueDesc {
                name: "KIND_A".into(),
                number: 1,
            }],
            features: FeatureSet::default(),
        });
        request.proto_files[0].messages[0].fields.push(FieldDesc {
            name: "kind".into(),
            number: 1,
            label: FieldLabel::Optional,
            type_: FieldType::Enum,
            type_name: Some(ProtoFqn::parse(".Kind")),
            oneof_index: None,
            proto3_optional: false,
            default_value: None,
            packed: None,
            string_layout: None,
            bytes_layout: None,
            features: FeatureSet::default(),
        });
        let err = emit(&request).unwrap_err();
        assert!(err.to_string().contains("must define 0 as its first value"));
    }

    #[test]
    fn emit_zero_less_nested_enum() {
        let request = CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec!["t.proto".into()],
                parameter: None,
            },
            proto_files: vec![ProtoFile {
                name: "t.proto".into(),
                package: "demo".into(),
                syntax: Syntax::Proto2,
                features: FeatureSet::default(),
                dependency: vec![],
                messages: vec![MessageDesc {
                    name: "Field".into(),
                    fields: vec![],
                    nested_messages: vec![],
                    nested_enums: vec![EnumDesc {
                        name: "Type".into(),
                        values: vec![
                            EnumValueDesc {
                                name: "TYPE_DOUBLE".into(),
                                number: 1,
                            },
                            EnumValueDesc {
                                name: "TYPE_FLOAT".into(),
                                number: 2,
                            },
                        ],
                        features: FeatureSet::default(),
                    }],
                    oneofs: vec![],
                    map_entry: false,
                }],
                enums: vec![],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("pub struct Type"));
        assert!(content.contains("pub const DOUBLE: Self = Self(1i32)"));
        // proto2 default = first defined enumerator, not wire 0.
        assert!(content.contains("Self :: DOUBLE") || content.contains("Self::DOUBLE"));
        assert!(content.contains("Type :: DOUBLE") || content.contains("Type::DOUBLE"));
        assert!(!content.contains("Self(0)"));
    }

    #[test]
    fn emit_peer_message_field() {
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
                messages: vec![
                    MessageDesc {
                        name: "Address".into(),
                        fields: vec![],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                        map_entry: false,
                    },
                    MessageDesc {
                        name: "Task".into(),
                        fields: vec![FieldDesc {
                            name: "assignee".into(),
                            number: 1,
                            label: FieldLabel::Optional,
                            type_: FieldType::Message,
                            type_name: Some(ProtoFqn::parse(".demo.Address")),
                            oneof_index: None,
                            proto3_optional: false,
                            default_value: None,
                            packed: None,
                            string_layout: None,
                            bytes_layout: None,
                            features: FeatureSet::default(),
                        }],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                        map_entry: false,
                    },
                ],
                enums: vec![],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("pub struct Address"));
        assert!(content.contains("pub struct Task"));
        assert!(content.contains("ProtoMessage"));
        assert!(content.contains("::puroro_rt::Message"));
        assert!(
            content.contains("self :: _root :: demo :: address :: Address")
                || content.contains("self::_root::demo::address::Address")
        );
        assert!(content.contains("fn assignee("));
        assert!(content.contains(".get()"));
        assert!(content.contains(".get_mut()"));
    }

    #[test]
    fn empty_file_to_generate_yields_no_files() {
        let mut request = empty_request("Empty");
        request.meta.file_to_generate.clear();
        let response = emit(&request).unwrap();
        assert!(response.files.is_empty());
    }

    #[test]
    fn unknown_parameter_is_ignored() {
        let mut request = empty_request("Empty");
        request.meta.parameter = Some("not_a_real_option=1".into());
        let response = emit(&request).unwrap();
        assert_eq!(response.files.len(), 1);
        assert!(response.files[0].content.contains("struct Empty"));
    }

    #[test]
    fn emit_real_oneof_group() {
        let request = CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec!["t.proto".into()],
                parameter: None,
            },
            proto_files: vec![ProtoFile {
                name: "t.proto".into(),
                package: String::new(),
                syntax: Syntax::Proto3,
                features: FeatureSet::default(),
                dependency: vec![],
                messages: vec![
                    MessageDesc {
                        name: "Peer".into(),
                        fields: vec![],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                        map_entry: false,
                    },
                    MessageDesc {
                        name: "Holder".into(),
                        fields: vec![
                            FieldDesc {
                                name: "email".into(),
                                number: 1,
                                label: FieldLabel::Optional,
                                type_: FieldType::String,
                                type_name: None,
                                oneof_index: Some(0),
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "code".into(),
                                number: 2,
                                label: FieldLabel::Optional,
                                type_: FieldType::Int32,
                                type_name: None,
                                oneof_index: Some(0),
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "urgent".into(),
                                number: 3,
                                label: FieldLabel::Optional,
                                type_: FieldType::Bool,
                                type_name: None,
                                oneof_index: Some(0),
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "peer".into(),
                                number: 4,
                                label: FieldLabel::Optional,
                                type_: FieldType::Message,
                                type_name: Some(ProtoFqn::parse(".Peer")),
                                oneof_index: Some(0),
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                        ],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![OneofDesc {
                            name: "choice".into(),
                        }],
                        map_entry: false,
                    },
                ],
                enums: vec![],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("OneofSlot"));
        assert!(content.contains("OneofGroup"));
        assert!(content.contains("ChoiceCase"));
        assert!(content.contains("ChoiceStorage"));
        assert!(content.contains("BitPacked"));
        assert!(content.contains("fn email"));
        assert!(content.contains("fn peer"));
        assert!(content.contains("fn choice"));
    }

    #[test]
    fn emit_map_string_int32() {
        let request = CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec!["t.proto".into()],
                parameter: None,
            },
            proto_files: vec![ProtoFile {
                name: "t.proto".into(),
                package: String::new(),
                syntax: Syntax::Proto3,
                features: FeatureSet::default(),
                dependency: vec![],
                messages: vec![MessageDesc {
                    name: "Holder".into(),
                    fields: vec![FieldDesc {
                        name: "attributes".into(),
                        number: 1,
                        label: FieldLabel::Repeated,
                        type_: FieldType::Message,
                        type_name: Some(ProtoFqn::parse(".Holder.AttributesEntry")),
                        oneof_index: None,
                        proto3_optional: false,
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![MessageDesc {
                        name: "AttributesEntry".into(),
                        fields: vec![
                            FieldDesc {
                                name: "key".into(),
                                number: 1,
                                label: FieldLabel::Optional,
                                type_: FieldType::String,
                                type_name: None,
                                oneof_index: None,
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "value".into(),
                                number: 2,
                                label: FieldLabel::Optional,
                                type_: FieldType::Int32,
                                type_name: None,
                                oneof_index: None,
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                        ],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                        map_entry: true,
                    }],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
                }],
                enums: vec![],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("MapField"));
        assert!(content.contains("ProtoString"));
        assert!(content.contains("ProtoInt32"));
        assert!(content.contains("MapRef"));
        assert!(content.contains("::puroro::MapMut"));
        assert!(content.contains("fn attributes"));
        assert!(content.contains("clear_attributes"));
        // Synthetic map-entry message must not be emitted as a user type.
        assert!(!content.contains("struct AttributesEntry"));
        assert!(!content.contains("RepeatedField"));
        assert!(!content.contains("MapStrInsert"));
        assert!(!content.contains("MapEntryInsert"));
        assert!(!content.contains("MapEntryMut"));
    }

    #[test]
    fn emit_map_int32_bool_uses_entry_mut() {
        let request = CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec!["t.proto".into()],
                parameter: None,
            },
            proto_files: vec![ProtoFile {
                name: "t.proto".into(),
                package: String::new(),
                syntax: Syntax::Proto3,
                features: FeatureSet::default(),
                dependency: vec![],
                messages: vec![MessageDesc {
                    name: "Holder".into(),
                    fields: vec![FieldDesc {
                        name: "flags".into(),
                        number: 1,
                        label: FieldLabel::Repeated,
                        type_: FieldType::Message,
                        type_name: Some(ProtoFqn::parse(".Holder.FlagsEntry")),
                        oneof_index: None,
                        proto3_optional: false,
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet::default(),
                    }],
                    nested_messages: vec![MessageDesc {
                        name: "FlagsEntry".into(),
                        fields: vec![
                            FieldDesc {
                                name: "key".into(),
                                number: 1,
                                label: FieldLabel::Optional,
                                type_: FieldType::Int32,
                                type_name: None,
                                oneof_index: None,
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                            FieldDesc {
                                name: "value".into(),
                                number: 2,
                                label: FieldLabel::Optional,
                                type_: FieldType::Bool,
                                type_name: None,
                                oneof_index: None,
                                proto3_optional: false,
                                default_value: None,
                                packed: None,
                                string_layout: None,
                                bytes_layout: None,
                                features: FeatureSet::default(),
                            },
                        ],
                        nested_messages: vec![],
                        nested_enums: vec![],
                        oneofs: vec![],
                        map_entry: true,
                    }],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
                }],
                enums: vec![],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("MapField"));
        assert!(content.contains("::puroro::MapMut"));
        assert!(content.contains("ProtoInt32"));
        assert!(content.contains("ProtoBool"));
        assert!(!content.contains("MapEntryInsert"));
        assert!(!content.contains("MapStrInsert"));
        assert!(!content.contains("MapEntryMut"));
        assert!(!content.contains("struct FlagsEntry"));
    }

    #[test]
    fn utf8_validation_none_still_emits_proto_string() {
        // Contract: IR records utf8_validation=NONE, but the emitter always uses
        // `ProtoString` (VERIFY semantics). Lock that until NONE is implemented
        // or explicitly rejected.
        let request = CodegenRequest {
            meta: CodegenMeta {
                file_to_generate: vec!["t.proto".into()],
                parameter: None,
            },
            proto_files: vec![ProtoFile {
                name: "t.proto".into(),
                package: String::new(),
                syntax: Syntax::Editions(Edition::Edition2023),
                features: FeatureSet::default(),
                dependency: vec![],
                messages: vec![MessageDesc {
                    name: "M".into(),
                    fields: vec![FieldDesc {
                        name: "title".into(),
                        number: 1,
                        label: FieldLabel::Optional,
                        type_: FieldType::String,
                        type_name: None,
                        oneof_index: None,
                        proto3_optional: false,
                        default_value: None,
                        packed: None,
                        string_layout: None,
                        bytes_layout: None,
                        features: FeatureSet {
                            utf8_validation: Some(Utf8Validation::None),
                            ..FeatureSet::default()
                        },
                    }],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                    map_entry: false,
                }],
                enums: vec![],
            }],
        };
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("ProtoString"));
        assert!(!content.contains("ProtoBytes"));
    }

    #[test]
    fn emit_custom_defaults_on_singular_and_oneof() {
        let mut request = empty_request("Holder");
        request.meta.file_to_generate = vec!["defaults.proto".into()];
        request.proto_files[0].name = "defaults.proto".into();
        request.proto_files[0].syntax = Syntax::Proto2;
        request.proto_files[0].messages[0].name = "Holder".into();
        request.proto_files[0].messages[0].oneofs = vec![OneofDesc {
            name: "choice".into(),
        }];
        request.proto_files[0].messages[0].fields = vec![
            FieldDesc {
                name: "max_retries".into(),
                number: 1,
                label: FieldLabel::Optional,
                type_: FieldType::Int32,
                type_name: None,
                oneof_index: None,
                proto3_optional: false,
                default_value: Some("3".into()),
                packed: None,
                string_layout: None,
                bytes_layout: None,
                features: FeatureSet::default(),
            },
            FieldDesc {
                name: "zero_int".into(),
                number: 2,
                label: FieldLabel::Optional,
                type_: FieldType::Int32,
                type_name: None,
                oneof_index: None,
                proto3_optional: false,
                default_value: Some("0".into()),
                packed: None,
                string_layout: None,
                bytes_layout: None,
                features: FeatureSet::default(),
            },
            FieldDesc {
                name: "webhook_id".into(),
                number: 3,
                label: FieldLabel::Optional,
                type_: FieldType::Int32,
                type_name: None,
                oneof_index: Some(0),
                proto3_optional: false,
                default_value: Some("-1".into()),
                packed: None,
                string_layout: None,
                bytes_layout: None,
                features: FeatureSet::default(),
            },
            FieldDesc {
                name: "note".into(),
                number: 4,
                label: FieldLabel::Optional,
                type_: FieldType::String,
                type_name: None,
                oneof_index: Some(0),
                proto3_optional: false,
                default_value: None,
                packed: None,
                string_layout: None,
                bytes_layout: None,
                features: FeatureSet::default(),
            },
        ];
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("mod defaults"));
        assert!(content.contains("struct MaxRetriesDefault"));
        assert!(content.contains("const DEFAULT: i32 = 3i32"));
        assert!(content.contains("MaxRetriesDefault"));
        assert!(content.contains("struct WebhookIdDefault"));
        assert!(content.contains("WebhookIdDefault"));
        // Type-zero `[default = 0]` must not invent a custom marker.
        assert!(!content.contains("ZeroIntDefault"));
    }

    const DESCRIPTOR_PROTO: &str = "google/protobuf/descriptor.proto";
    const PLUGIN_PROTO: &str = "google/protobuf/compiler/plugin.proto";

    fn official_plugin_fixture_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../puroro-codegen-tests/fixtures/official_plugin")
    }

    fn resolve_protoc() -> PathBuf {
        env::var_os("PROTOC")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("protoc"))
    }

    /// Compile the vendored official schemas to a `FileDescriptorSet`.
    fn official_file_descriptor_set() -> Vec<u8> {
        let fixture = official_plugin_fixture_dir();
        let out = env::temp_dir().join(format!("puroro-official-fds-{}.bin", process::id()));
        let protoc = resolve_protoc();
        let mut cmd = Command::new(&protoc);
        cmd.arg(format!("-I{}", fixture.display()))
            .arg(format!("--descriptor_set_out={}", out.display()))
            .arg(DESCRIPTOR_PROTO)
            .arg(PLUGIN_PROTO);
        let output = cmd
            .output()
            .unwrap_or_else(|e| panic!("failed to spawn `{}`: {e}", protoc.display()));
        if !output.status.success() {
            let _ = fs::remove_file(&out);
            panic!(
                "protoc --descriptor_set_out failed (status {}):\n\
                 command: {cmd:?}\n\
                 stdout:\n{}\n\
                 stderr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }
        let bytes = fs::read(&out)
            .unwrap_or_else(|e| panic!("failed to read descriptor set {}: {e}", out.display()));
        let _ = fs::remove_file(&out);
        bytes
    }

    /// Wrap `FileDescriptorSet.file` blobs as `CodeGeneratorRequest.proto_file`.
    fn request_from_descriptor_set(
        descriptor_set: &[u8],
        file_to_generate: &[&str],
    ) -> CodegenRequest {
        let mut request = Vec::new();
        for name in file_to_generate {
            let field = Field::new(
                FieldNumber::try_new(1).expect("file_to_generate field number"),
                FieldValue::Len(name.as_bytes()),
            );
            request
                .write_protobuf_field(&field)
                .expect("write file_to_generate");
        }
        for field in AsRefExtProtobuf::read_protobuf_fields(&descriptor_set) {
            let field = field.expect("FileDescriptorSet field");
            if field.field_number.as_u32() != 1 {
                continue;
            }
            let FieldValue::Len(bytes) = field.value else {
                panic!("FileDescriptorSet.file must be length-delimited");
            };
            let wrapped = Field::new(
                FieldNumber::try_new(15).expect("proto_file field number"),
                FieldValue::Len(bytes),
            );
            request
                .write_protobuf_field(&wrapped)
                .expect("write proto_file");
        }
        decode_request(&request).expect("decode CodeGeneratorRequest from descriptor set")
    }

    #[test]
    fn emit_official_descriptor_and_plugin_proto() {
        let fds = official_file_descriptor_set();
        let request = request_from_descriptor_set(&fds, &[DESCRIPTOR_PROTO, PLUGIN_PROTO]);
        assert_eq!(
            request.meta.file_to_generate,
            vec![DESCRIPTOR_PROTO, PLUGIN_PROTO]
        );
        assert_eq!(request.proto_files.len(), 2);

        let response = emit(&request).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "lib.rs");
        let content = &response.files[0].content;
        assert!(
            content.contains("struct FileDescriptorProto"),
            "descriptor.proto types missing: {content}"
        );
        assert!(
            content.contains("struct CodeGeneratorRequest"),
            "plugin.proto types missing: {content}"
        );
        assert!(
            content.contains("struct CodeGeneratorResponse"),
            "plugin.proto types missing: {content}"
        );
    }
}
