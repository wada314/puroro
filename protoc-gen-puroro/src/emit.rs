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
    // Plugin plumbing, codegen errors, and tokens that must not appear.
    // Behaviour of generated types: `puroro-codegen-tests`. Field planning:
    // `field_kind::plan`.
    use super::*;
    use crate::descriptor::features::Utf8Validation;
    use crate::descriptor::test_helpers as desc;
    use crate::descriptor::{
        BytesLayout, CodegenRequest, Edition, FeatureSet, FieldDesc, FieldLabel, FieldType,
        MessageDesc, ProtoFile, ProtoFqn, StringLayout, Syntax,
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

    fn emit_lib(request: &CodegenRequest) -> String {
        let response = emit(request).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "lib.rs");
        response.files[0].content.clone()
    }

    #[test]
    fn plugin_response_is_single_lib_rs_with_generated_header() {
        let content = emit_lib(&empty_request("Empty"));
        assert!(content.contains("@generated"));
        assert!(content.contains("- `empty.proto`"));
    }

    #[test]
    fn empty_companion_module_is_omitted() {
        let content = emit_lib(&empty_request_with_package("Empty", "example.v1"));
        assert!(!content.contains("pub mod empty"));
        assert!(!content.contains("pub use empty::Empty"));
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
        let _ = emit_lib(&request);
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
    fn heap_string_layout_does_not_emit_sso() {
        let mut request = empty_request("HeapString");
        request.proto_files[0].messages[0].fields = vec![FieldDesc {
            proto3_optional: true,
            string_layout: Some(StringLayout::Heap),
            ..desc::field("body", 1, FieldType::String)
        }];
        let content = emit_lib(&request);
        assert!(
            !content.contains("InlineOrHeap"),
            "string_layout=HEAP must not emit SSO layout: {content}"
        );
        assert!(
            !content.contains("BIT_BODY_SSO"),
            "string_layout=HEAP must not allocate an SSO bit: {content}"
        );
    }

    #[test]
    fn heap_bytes_layout_does_not_emit_sso() {
        let mut request = empty_request("HeapBytes");
        request.proto_files[0].messages[0].fields = vec![FieldDesc {
            proto3_optional: true,
            bytes_layout: Some(BytesLayout::Heap),
            ..desc::field("body", 1, FieldType::Bytes)
        }];
        let content = emit_lib(&request);
        assert!(
            !content.contains("InlineOrHeap"),
            "bytes_layout=HEAP must not emit SSO layout: {content}"
        );
        assert!(
            !content.contains("BIT_BODY_SSO"),
            "bytes_layout=HEAP must not allocate an SSO bit: {content}"
        );
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
        let content = emit_lib(&request);
        assert!(content.contains("ProtoString"));
        assert!(!content.contains("ProtoBytes"));
    }

    #[test]
    fn map_does_not_emit_entry_struct_or_retired_apis() {
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
        let content = emit_lib(&request);
        assert!(!content.contains("struct AttributesEntry"));
        assert!(!content.contains("RepeatedField"));
        assert!(!content.contains("MapStrInsert"));
        assert!(!content.contains("MapEntryInsert"));
        assert!(!content.contains("MapEntryMut"));
    }

    #[test]
    fn type_zero_default_does_not_emit_custom_marker() {
        let mut request = empty_request("Holder");
        request.proto_files[0].syntax = Syntax::Proto2;
        request.proto_files[0].messages[0].fields = vec![FieldDesc {
            default_value: Some("0".into()),
            ..desc::field("zero_int", 1, FieldType::Int32)
        }];
        let content = emit_lib(&request);
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
    }
}
