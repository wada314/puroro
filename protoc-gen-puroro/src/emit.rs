//! Code emission from a resolved schema.
//!
//! Current scope: file-level and nested enums/messages with singular / repeated
//! scalar / string / bytes / bool / enum / message fields, real oneofs, and
//! maps with legal keys and Copy scalar / bool values. All `file_to_generate`
//! entries share one [`ModuleForest`] so cross-file type refs use a single
//! `self::_root`.

use crate::descriptor::{CodegenRequest, ProtoFqn};
use crate::error::{Error, Result};
use crate::field_kind::plan_fields;
use crate::module_tree::layout::{ModuleLayout, render};
use crate::module_tree::{ModuleForest, ModuleNode, ModuleOrigin, type_name_to_module_ident};
use crate::plugin_io::CodeGeneratorResponse;
use crate::resolved::{Arena, Enum, File, Message, resolve};
use ::proc_macro2::Ident;
use ::quote::quote;
use ::syn::{Attribute, Item};

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
        forest.install_file(prepare_file(file)?);
    }

    let mut files = render(
        &forest,
        &ModuleLayout::SingleFile {
            path: "lib.rs".into(),
        },
    )?;
    let file = files
        .pop()
        .ok_or_else(|| Error::internal("layout produced no files"))?;
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

/// One message, prepared to install into a parent module.
///
/// The struct (and its impls) go on the parent. A snake_case companion holds
/// `FIELD_*` / `BIT_*`, nested types, and oneofs. Nested messages install into
/// that companion the same way. Empty companions are omitted.
struct PreparedMessage {
    /// Items appended to the parent — e.g. `[pub struct Foo, impl Foo { … }]`.
    type_items: Vec<Item>,
    /// Companion module ident — e.g. `foo` for `message Foo`. Unused when omitted.
    module_name: Ident,
    /// Protobuf FQN of this message — e.g. `.example.Foo`. Used at install to
    /// mark the companion with [`ModuleOrigin::Message`].
    proto_fqn: ProtoFqn,
    /// Items in the companion — e.g. `[pub const FIELD_TITLE, …]`.
    /// Empty together with [`Self::nested_enums`] and [`Self::nested`] means no
    /// companion.
    companion_items: Vec<Item>,
    /// Nested enums installed into this companion — e.g. `[PreparedEnum` for `Kind]`.
    nested_enums: Vec<PreparedEnum>,
    /// Nested messages installed into this companion — e.g. `[PreparedMessage` for `Bar]`.
    nested: Vec<PreparedMessage>,
}

/// One enum, prepared to install into a parent module.
struct PreparedEnum {
    /// Items appended to the parent — e.g. `[pub struct Status, impl Status { … }]`.
    items: Vec<Item>,
}

/// One `.proto` file, prepared to install into a package module.
struct PreparedFile {
    /// Protobuf package — e.g. `example.v1`. Empty string is the forest root.
    package: String,
    /// File-level enums — e.g. `[PreparedEnum` for `Status]`.
    enums: Vec<PreparedEnum>,
    /// File-level messages — e.g. `[PreparedMessage` for `Foo]`.
    messages: Vec<PreparedMessage>,
}

fn prepare_message(message: &Message<'_>) -> Result<PreparedMessage> {
    if !ident::is_simple_ident(message.name()) {
        return Err(Error::Codegen(format!(
            "cannot use message name `{}` as a Rust identifier",
            message.name()
        )));
    }
    let module_name = type_name_to_module_ident(message.name());
    let type_name = ident::escape_ident(message.name());
    let field_plan = plan_fields(message)?;
    let rendered = message::render_items(&field_plan, &type_name, message.name(), &module_name)?;

    let nested_enums = message
        .nested_enums()
        .map(prepare_enum)
        .collect::<Result<Vec<_>>>()?;
    let nested = message
        .nested_messages()
        .filter(|m| !m.is_map_entry())
        .map(prepare_message)
        .collect::<Result<Vec<_>>>()?;

    Ok(PreparedMessage {
        type_items: rendered.type_items,
        module_name,
        proto_fqn: message.fqn().clone(),
        companion_items: rendered.companion_items,
        nested_enums,
        nested,
    })
}

fn prepare_enum(enumeration: &Enum<'_>) -> Result<PreparedEnum> {
    Ok(PreparedEnum {
        items: enumeration::render_enum(enumeration)?,
    })
}

fn prepare_file(file: &File<'_>) -> Result<PreparedFile> {
    Ok(PreparedFile {
        package: file.package().to_owned(),
        enums: file.enums().map(prepare_enum).collect::<Result<Vec<_>>>()?,
        messages: file
            .messages()
            .map(prepare_message)
            .collect::<Result<Vec<_>>>()?,
    })
}

impl ModuleForest {
    fn install_file(&mut self, prepared: PreparedFile) {
        let package = self.ensure_package(&prepared.package);
        for e in prepared.enums {
            package.install_enum(e);
        }
        for message in prepared.messages {
            package.install_message(message);
        }
    }
}

impl ModuleNode {
    fn install_message(&mut self, prepared: PreparedMessage) {
        self.append_items(prepared.type_items);
        if prepared.companion_items.is_empty()
            && prepared.nested_enums.is_empty()
            && prepared.nested.is_empty()
        {
            return;
        }
        let companion = self.get_or_insert_child(prepared.module_name);
        companion.add_origin(ModuleOrigin::Message {
            proto_fqn: prepared.proto_fqn,
        });
        companion.append_items(prepared.companion_items);
        for nested_enum in prepared.nested_enums {
            companion.install_enum(nested_enum);
        }
        for nested in prepared.nested {
            companion.install_message(nested);
        }
    }

    fn install_enum(&mut self, prepared: PreparedEnum) {
        self.append_items(prepared.items);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::features::{EnumType, Utf8Validation};
    use crate::descriptor::test_helpers as desc;
    use crate::descriptor::{
        BytesLayout, CodegenRequest, Edition, EnumDesc, FeatureSet, FieldDesc, FieldLabel,
        FieldType, MessageDesc, ProtoFile, ProtoFqn, StringLayout, Syntax,
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
        desc::request(vec![ProtoFile {
            messages: vec![desc::message(message_name)],
            ..desc::proto_file("empty.proto", package)
        }])
    }

    #[test]
    fn emit_empty_message_mentions_type_name() {
        let response = emit(&empty_request("Empty")).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "lib.rs");
        assert!(response.files[0].content.contains("struct Empty"));
        assert!(response.files[0].content.contains("@generated"));
        assert!(response.files[0].content.contains("- `empty.proto`"));
    }

    #[test]
    fn emit_empty_message_nests_package_modules() {
        let response = emit(&empty_request_with_package("Empty", "example.v1")).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("pub mod example"));
        assert!(content.contains("pub mod v1"));
        assert!(!content.contains("pub mod empty"));
        assert!(!content.contains("pub use empty::Empty"));
        assert!(content.contains("struct Empty"));
        let example_idx = content.find("pub mod example").expect("example mod");
        let struct_idx = content.find("struct Empty").expect("struct Empty");
        assert!(
            struct_idx > example_idx,
            "struct Empty should appear inside the package module tree"
        );
    }

    #[test]
    fn emit_singular_scalar_fields() {
        let mut request = empty_request("Scalars");
        request.meta.file_to_generate = vec!["scalars.proto".into()];
        request.proto_files[0].name = "scalars.proto".into();
        request.proto_files[0].messages[0].name = "Scalars".into();
        request.proto_files[0].messages[0].fields = vec![
            desc::field("score", 1, FieldType::Int32),
            FieldDesc {
                proto3_optional: true,
                ..desc::field("title", 2, FieldType::String)
            },
            desc::field("done", 3, FieldType::Bool),
            FieldDesc {
                proto3_optional: true,
                ..desc::field("payload", 4, FieldType::Bytes)
            },
            desc::field("zigzag", 5, FieldType::SInt32),
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
            proto3_optional: true,
            string_layout: Some(StringLayout::Heap),
            ..desc::field("body", 1, FieldType::String)
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
            proto3_optional: true,
            bytes_layout: Some(BytesLayout::Heap),
            ..desc::field("body", 1, FieldType::Bytes)
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
        let request = desc::request(vec![ProtoFile {
            messages: vec![MessageDesc {
                fields: vec![FieldDesc {
                    type_name: Some(ProtoFqn::parse(".demo.Status")),
                    ..desc::field("status", 1, FieldType::Enum)
                }],
                ..desc::message("Holder")
            }],
            enums: vec![desc::enumeration(
                "Status",
                vec![
                    desc::enum_value("STATUS_UNSPECIFIED", 0),
                    desc::enum_value("STATUS_PENDING", 1),
                ],
            )],
            ..desc::proto_file("t.proto", "demo")
        }]);
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
        let request = desc::request(vec![
            ProtoFile {
                syntax: Syntax::Editions(Edition::Edition2023),
                enums: vec![desc::enumeration(
                    "Status",
                    vec![
                        desc::enum_value("STATUS_UNSPECIFIED", 0),
                        desc::enum_value("STATUS_PENDING", 1),
                    ],
                )],
                ..desc::proto_file("status.proto", "demo")
            },
            ProtoFile {
                syntax: Syntax::Editions(Edition::Edition2024),
                enums: vec![EnumDesc {
                    features: closed,
                    ..desc::enumeration(
                        "Priority",
                        vec![
                            desc::enum_value("PRIORITY_UNSPECIFIED", 0),
                            desc::enum_value("PRIORITY_HIGH", 1),
                        ],
                    )
                }],
                ..desc::proto_file("priority.proto", "demo")
            },
            ProtoFile {
                syntax: Syntax::Editions(Edition::Edition2023),
                dependency: vec!["status.proto".into(), "priority.proto".into()],
                messages: vec![MessageDesc {
                    fields: vec![
                        FieldDesc {
                            type_name: Some(ProtoFqn::parse(".demo.Status")),
                            ..desc::field("status", 1, FieldType::Enum)
                        },
                        FieldDesc {
                            type_name: Some(ProtoFqn::parse(".demo.Priority")),
                            ..desc::field("priority", 2, FieldType::Enum)
                        },
                    ],
                    ..desc::message("Holder")
                }],
                ..desc::proto_file("holder.proto", "demo")
            },
        ]);
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
        let request = desc::request(vec![ProtoFile {
            messages: vec![MessageDesc {
                fields: vec![
                    FieldDesc {
                        label: FieldLabel::Repeated,
                        ..desc::field("tag_ids", 1, FieldType::Int32)
                    },
                    FieldDesc {
                        label: FieldLabel::Repeated,
                        type_name: Some(ProtoFqn::parse(".demo.Outer.Inner")),
                        ..desc::field("inners", 2, FieldType::Message)
                    },
                    FieldDesc {
                        type_name: Some(ProtoFqn::parse(".demo.Outer.Kind")),
                        ..desc::field("kind", 3, FieldType::Enum)
                    },
                ],
                nested_messages: vec![desc::message("Inner")],
                nested_enums: vec![desc::enumeration(
                    "Kind",
                    vec![
                        desc::enum_value("KIND_UNSPECIFIED", 0),
                        desc::enum_value("KIND_A", 1),
                    ],
                )],
                ..desc::message("Outer")
            }],
            ..desc::proto_file("t.proto", "demo")
        }]);
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("RepeatedField"));
        assert!(content.contains("::puroro_rt::Packed"));
        assert!(content.contains("pub struct Inner"));
        assert!(content.contains("pub struct Kind"));
        assert!(
            content.contains("self :: _root :: demo :: outer :: Inner")
                || content.contains("self::_root::demo::outer::Inner")
        );
        assert!(
            content.contains("self :: _root :: demo :: outer :: Kind")
                || content.contains("self::_root::demo::outer::Kind")
        );
    }

    #[test]
    fn reject_open_enum_without_leading_zero() {
        let mut request = empty_request("Holder");
        request.proto_files[0].enums.push(desc::enumeration(
            "Kind",
            vec![desc::enum_value("KIND_A", 1)],
        ));
        request.proto_files[0].messages[0].fields.push(FieldDesc {
            type_name: Some(ProtoFqn::parse(".Kind")),
            ..desc::field("kind", 1, FieldType::Enum)
        });
        let err = emit(&request).unwrap_err();
        assert!(err.to_string().contains("must define 0 as its first value"));
    }

    #[test]
    fn emit_zero_less_nested_enum() {
        let request = desc::request(vec![ProtoFile {
            syntax: Syntax::Proto2,
            messages: vec![MessageDesc {
                nested_enums: vec![desc::enumeration(
                    "Type",
                    vec![
                        desc::enum_value("TYPE_DOUBLE", 1),
                        desc::enum_value("TYPE_FLOAT", 2),
                    ],
                )],
                ..desc::message("Field")
            }],
            ..desc::proto_file("t.proto", "demo")
        }]);
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
        let request = desc::request(vec![ProtoFile {
            messages: vec![
                desc::message("Address"),
                MessageDesc {
                    fields: vec![FieldDesc {
                        type_name: Some(ProtoFqn::parse(".demo.Address")),
                        ..desc::field("assignee", 1, FieldType::Message)
                    }],
                    ..desc::message("Task")
                },
            ],
            ..desc::proto_file("t.proto", "demo")
        }]);
        let response = emit(&request).unwrap();
        let content = &response.files[0].content;
        assert!(content.contains("pub struct Address"));
        assert!(content.contains("pub struct Task"));
        assert!(content.contains("ProtoMessage"));
        assert!(content.contains("::puroro_rt::Message"));
        assert!(
            content.contains("self :: _root :: demo :: Address")
                || content.contains("self::_root::demo::Address")
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
        let request = desc::request(vec![ProtoFile {
            messages: vec![
                desc::message("Peer"),
                MessageDesc {
                    fields: vec![
                        FieldDesc {
                            oneof_index: Some(0),
                            ..desc::field("email", 1, FieldType::String)
                        },
                        FieldDesc {
                            oneof_index: Some(0),
                            ..desc::field("code", 2, FieldType::Int32)
                        },
                        FieldDesc {
                            oneof_index: Some(0),
                            ..desc::field("urgent", 3, FieldType::Bool)
                        },
                        FieldDesc {
                            type_name: Some(ProtoFqn::parse(".Peer")),
                            oneof_index: Some(0),
                            ..desc::field("peer", 4, FieldType::Message)
                        },
                    ],
                    oneofs: vec![desc::oneof("choice")],
                    ..desc::message("Holder")
                },
            ],
            ..desc::proto_file("t.proto", "")
        }]);
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
        let request = desc::request(vec![ProtoFile {
            messages: vec![MessageDesc {
                fields: vec![FieldDesc {
                    label: FieldLabel::Repeated,
                    type_name: Some(ProtoFqn::parse(".Holder.AttributesEntry")),
                    ..desc::field("attributes", 1, FieldType::Message)
                }],
                nested_messages: vec![desc::map_entry(
                    "AttributesEntry",
                    desc::field("key", 1, FieldType::String),
                    desc::field("value", 2, FieldType::Int32),
                )],
                ..desc::message("Holder")
            }],
            ..desc::proto_file("t.proto", "")
        }]);
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
        let request = desc::request(vec![ProtoFile {
            messages: vec![MessageDesc {
                fields: vec![FieldDesc {
                    label: FieldLabel::Repeated,
                    type_name: Some(ProtoFqn::parse(".Holder.FlagsEntry")),
                    ..desc::field("flags", 1, FieldType::Message)
                }],
                nested_messages: vec![desc::map_entry(
                    "FlagsEntry",
                    desc::field("key", 1, FieldType::Int32),
                    desc::field("value", 2, FieldType::Bool),
                )],
                ..desc::message("Holder")
            }],
            ..desc::proto_file("t.proto", "")
        }]);
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
        let request = desc::request(vec![ProtoFile {
            syntax: Syntax::Editions(Edition::Edition2023),
            messages: vec![MessageDesc {
                fields: vec![FieldDesc {
                    features: FeatureSet {
                        utf8_validation: Some(Utf8Validation::None),
                        ..FeatureSet::default()
                    },
                    ..desc::field("title", 1, FieldType::String)
                }],
                ..desc::message("M")
            }],
            ..desc::proto_file("t.proto", "")
        }]);
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
        request.proto_files[0].messages[0].oneofs = vec![desc::oneof("choice")];
        request.proto_files[0].messages[0].fields = vec![
            FieldDesc {
                default_value: Some("3".into()),
                ..desc::field("max_retries", 1, FieldType::Int32)
            },
            FieldDesc {
                default_value: Some("0".into()),
                ..desc::field("zero_int", 2, FieldType::Int32)
            },
            FieldDesc {
                oneof_index: Some(0),
                default_value: Some("-1".into()),
                ..desc::field("webhook_id", 3, FieldType::Int32)
            },
            FieldDesc {
                oneof_index: Some(0),
                ..desc::field("note", 4, FieldType::String)
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
