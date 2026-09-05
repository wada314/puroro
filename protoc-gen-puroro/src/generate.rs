//! Plugin orchestration: resolve → prepare → forest → layout.
//!
//! Quote and `Prepared*` types live in [`crate::emit`]. This module selects
//! `file_to_generate`, installs into one [`crate::module_tree::ModuleForest`],
//! and renders plugin files.

use crate::descriptor::CodegenRequest;
use crate::descriptor::features::Utf8Validation;
use crate::emit::{generated_file_attrs, prepare_file};
use crate::error::{Error, Result};
use crate::field_kind::plan_message_storage;
use crate::module_tree::ModuleForest;
use crate::module_tree::layout::{ModuleLayout, render};
use crate::plugin_io::CodeGeneratorResponse;
use crate::resolved::{Arena, resolve_with};

/// Generate-time knobs parsed from [`CodegenRequest`] `parameter`.
struct GenerateOptions {
    /// proto2 syntax default for `utf8_validation` (spec default is `NONE`).
    proto2_utf8: Utf8Validation,
}

fn parse_generate_options(parameter: Option<&str>) -> Result<GenerateOptions> {
    let mut options = GenerateOptions {
        proto2_utf8: Utf8Validation::None,
    };
    let Some(raw) = parameter else {
        return Ok(options);
    };
    for part in raw.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let Some((key, value)) = part.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key == "proto2_utf8" {
            options.proto2_utf8 = parse_proto2_utf8(value)?;
        }
    }
    Ok(options)
}

fn parse_proto2_utf8(value: &str) -> Result<Utf8Validation> {
    match value {
        "none" | "NONE" => Ok(Utf8Validation::None),
        "verify" | "VERIFY" | "str" => Ok(Utf8Validation::Verify),
        other => Err(Error::Codegen(format!(
            "unknown `proto2_utf8` value `{other}` (expected `none` or `verify`)"
        ))),
    }
}

/// Generate plugin response files from a decoded request.
pub fn generate(request: &CodegenRequest) -> Result<CodeGeneratorResponse> {
    let options = parse_generate_options(request.meta.parameter.as_deref())?;
    let arena = Arena::new();
    let file_set = resolve_with(&arena, &request.proto_files, options.proto2_utf8)?;

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

    let storage = plan_message_storage(&file_set);
    for file in &targets {
        forest.install_file(prepare_file(file, &storage)?);
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
        MessageDesc, MessageLayout, ProtoFile, ProtoFqn, StringLayout, Syntax, UnknownFieldsPolicy,
    };
    use crate::plugin_io::decode_request;
    use ::protobuf_core::{AsRefExtProtobuf, Field, FieldNumber, FieldValue, WriteExtProtobuf};
    use ::std::env;
    use ::std::fs;
    use ::std::path::PathBuf;
    use ::std::process::{self, Command};
    use ::std::thread;

    /// Successful `generate` results are also written under `generated-preview/`
    /// (gitignored) so the pretty-printed Rust can be inspected. Overwritten
    /// whenever that test runs.
    fn generate(request: &CodegenRequest) -> Result<CodeGeneratorResponse> {
        let result = super::generate(request);
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
            .strip_prefix("generate::tests::")
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

    fn generate_lib(request: &CodegenRequest) -> String {
        let response = generate(request).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "lib.rs");
        response.files[0].content.clone()
    }

    #[test]
    fn plugin_response_is_single_lib_rs_with_generated_header() {
        let content = generate_lib(&empty_request("Empty"));
        assert!(content.contains("@generated"));
        assert!(content.contains("- `empty.proto`"));
    }

    #[test]
    fn empty_companion_module_is_omitted() {
        let content = generate_lib(&empty_request_with_package("Empty", "example.v1"));
        assert!(!content.contains("pub mod empty"));
        assert!(!content.contains("pub use empty::Empty"));
    }

    #[test]
    fn empty_file_to_generate_yields_no_files() {
        let mut request = empty_request("Empty");
        request.meta.file_to_generate.clear();
        let response = generate(&request).unwrap();
        assert!(response.files.is_empty());
    }

    #[test]
    fn unknown_parameter_is_ignored() {
        let mut request = empty_request("Empty");
        request.meta.parameter = Some("not_a_real_option=1".into());
        let _ = generate_lib(&request);
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
        let err = generate(&request).unwrap_err();
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
        let content = generate_lib(&request);
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
    fn discard_unknowns_bakes_store_and_generic_skip() {
        let mut request = empty_request("Marker");
        request.proto_files[0].messages[0].unknown_fields = UnknownFieldsPolicy::Discard;
        request.proto_files[0].messages[0].fields = vec![desc::field("n", 1, FieldType::Int32)];
        let content = generate_lib(&request);
        assert!(
            content.contains("DiscardUnknowns"),
            "discard must bake DiscardUnknowns: {content}"
        );
        assert!(
            content.contains("skip_field_and_save_in"),
            "discard must use the generic skip: {content}"
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
        let content = generate_lib(&request);
        assert!(
            !content.contains("InlineOrHeap"),
            "bytes_layout=HEAP must not emit SSO layout: {content}"
        );
        assert!(
            !content.contains("BIT_BODY_SSO"),
            "bytes_layout=HEAP must not allocate an SSO bit: {content}"
        );
    }

    fn msg_field(name: &str, number: i32, type_name: &str) -> FieldDesc {
        FieldDesc {
            type_name: Some(ProtoFqn::parse(type_name)),
            ..desc::field(name, number, FieldType::Message)
        }
    }

    #[test]
    fn nested_message_auto_inline_emits_explicit_bit() {
        let request = desc::request(vec![ProtoFile {
            messages: vec![
                MessageDesc {
                    fields: vec![
                        desc::field("x", 1, FieldType::Int32),
                        desc::field("y", 2, FieldType::Int32),
                    ],
                    ..desc::message("Point")
                },
                MessageDesc {
                    fields: vec![msg_field("origin", 1, ".Point")],
                    ..desc::message("Holder")
                },
            ],
            ..desc::proto_file("t.proto", "")
        }]);
        let content = generate_lib(&request);
        assert!(
            content.contains("Explicit<{ holder::BIT_ORIGIN }>"),
            "auto-inlined origin must use a presence bit: {content}"
        );
        assert!(
            !content.contains("Boxed"),
            "auto-inlined origin must not emit Boxed: {content}"
        );
    }

    #[test]
    fn nested_message_default_boxed_emits_boxed_layout() {
        let request = desc::request(vec![ProtoFile {
            messages: vec![
                MessageDesc {
                    fields: vec![FieldDesc {
                        proto3_optional: true,
                        ..desc::field("street", 1, FieldType::String)
                    }],
                    ..desc::message("Address")
                },
                MessageDesc {
                    fields: vec![msg_field("assignee", 1, ".Address")],
                    ..desc::message("Holder")
                },
            ],
            ..desc::proto_file("t.proto", "")
        }]);
        let content = generate_lib(&request);
        assert!(
            content.contains("::puroro_rt::Boxed"),
            "Address-like child must emit Boxed: {content}"
        );
        assert!(
            content.contains("::puroro_rt::Message"),
            "boxed singular must use Message presence: {content}"
        );
        assert!(
            !content.contains("BIT_ASSIGNEE"),
            "boxed assignee must not allocate a presence bit: {content}"
        );
    }

    #[test]
    fn recursive_inline_hint_still_emits_boxed() {
        let request = desc::request(vec![ProtoFile {
            messages: vec![MessageDesc {
                fields: vec![FieldDesc {
                    message_layout: Some(MessageLayout::Inline),
                    ..msg_field("child", 1, ".Nest")
                }],
                ..desc::message("Nest")
            }],
            ..desc::proto_file("t.proto", "")
        }]);
        let content = generate_lib(&request);
        assert!(
            content.contains("::puroro_rt::Boxed"),
            "recursive Nest.child must stay Boxed: {content}"
        );
        assert!(
            content.contains("::puroro_rt::Message"),
            "recursive child must use Message presence: {content}"
        );
    }

    #[test]
    fn oneof_tiny_message_variant_emits_inline() {
        let request = desc::request(vec![ProtoFile {
            messages: vec![
                MessageDesc {
                    fields: vec![desc::field("x", 1, FieldType::Int32)],
                    ..desc::message("Point")
                },
                MessageDesc {
                    fields: vec![FieldDesc {
                        oneof_index: Some(0),
                        ..msg_field("postal", 1, ".Point")
                    }],
                    oneofs: vec![desc::oneof("note")],
                    ..desc::message("Holder")
                },
            ],
            ..desc::proto_file("t.proto", "")
        }]);
        let content = generate_lib(&request);
        assert!(
            !content.contains("Boxed"),
            "auto-inlined oneof Point must not emit Boxed: {content}"
        );
        assert!(
            !content.contains("BIT_POSTAL"),
            "oneof message variant must not allocate a presence bit: {content}"
        );
        assert!(
            content.contains("::puroro_rt::Oneof"),
            "oneof message variant must keep Oneof presence: {content}"
        );
    }

    #[test]
    fn oneof_address_variant_emits_boxed() {
        let request = desc::request(vec![ProtoFile {
            messages: vec![
                MessageDesc {
                    fields: vec![FieldDesc {
                        proto3_optional: true,
                        ..desc::field("street", 1, FieldType::String)
                    }],
                    ..desc::message("Address")
                },
                MessageDesc {
                    fields: vec![FieldDesc {
                        oneof_index: Some(0),
                        ..msg_field("postal", 1, ".Address")
                    }],
                    oneofs: vec![desc::oneof("note")],
                    ..desc::message("Holder")
                },
            ],
            ..desc::proto_file("t.proto", "")
        }]);
        let content = generate_lib(&request);
        assert!(
            content.contains("::puroro_rt::Boxed"),
            "Address-like oneof variant must emit Boxed: {content}"
        );
        assert!(
            !content.contains("BIT_POSTAL"),
            "oneof message variant must not allocate a presence bit: {content}"
        );
    }

    #[test]
    fn oneof_recursive_inline_hint_still_emits_boxed() {
        let request = desc::request(vec![ProtoFile {
            messages: vec![MessageDesc {
                fields: vec![FieldDesc {
                    oneof_index: Some(0),
                    message_layout: Some(MessageLayout::Inline),
                    ..msg_field("child", 1, ".Nest")
                }],
                oneofs: vec![desc::oneof("nest")],
                ..desc::message("Nest")
            }],
            ..desc::proto_file("t.proto", "")
        }]);
        let content = generate_lib(&request);
        assert!(
            content.contains("::puroro_rt::Boxed"),
            "recursive oneof Nest.child must stay Boxed: {content}"
        );
        assert!(
            !content.contains("BIT_CHILD"),
            "oneof message variant must not allocate a presence bit: {content}"
        );
    }

    #[test]
    fn utf8_validation_none_emits_unchecked_marker() {
        // Contract: IR records utf8_validation=NONE; plan selects
        // `ProtoStringUnchecked` (VERIFY keeps `ProtoString`).
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
        let content = generate_lib(&request);
        assert!(content.contains("ProtoStringUnchecked"));
        assert!(!content.contains("ProtoBytes"));
        assert!(content.contains("BytesMut"));
    }

    #[test]
    fn proto2_string_emits_unchecked_bytes_views() {
        let mut request = empty_request("M");
        request.proto_files[0].syntax = Syntax::Proto2;
        request.proto_files[0].messages[0].fields =
            vec![desc::field("title", 1, FieldType::String)];
        let content = generate_lib(&request);
        assert!(content.contains("ProtoStringUnchecked"));
        assert!(content.contains("BytesMut"));
        assert!(!content.contains("StringMut"));
    }

    #[test]
    fn proto2_required_boxed_message_emits_validate() {
        let request = desc::request(vec![ProtoFile {
            syntax: Syntax::Proto2,
            messages: vec![
                MessageDesc {
                    fields: vec![desc::field("street", 1, FieldType::String)],
                    ..desc::message("Address")
                },
                MessageDesc {
                    fields: vec![FieldDesc {
                        label: FieldLabel::Required,
                        ..msg_field("addr", 1, ".Address")
                    }],
                    ..desc::message("Holder")
                },
            ],
            ..desc::proto_file("t.proto", "")
        }]);
        let content = generate_lib(&request);
        assert!(
            content.contains("::puroro_rt::Boxed"),
            "required Address stays boxed: {content}"
        );
        assert!(
            content.contains("validate_required"),
            "boxed required message must participate in validate(): {content}"
        );
        assert!(
            !content.contains("BIT_ADDR"),
            "boxed required message must not allocate a presence bit: {content}"
        );
    }

    #[test]
    fn proto2_utf8_verify_emits_proto_string() {
        let mut request = empty_request("M");
        request.proto_files[0].syntax = Syntax::Proto2;
        request.proto_files[0].messages[0].fields =
            vec![desc::field("title", 1, FieldType::String)];
        request.meta.parameter = Some("proto2_utf8=verify".into());
        let content = generate_lib(&request);
        assert!(!content.contains("ProtoStringUnchecked"));
        assert!(content.contains("ProtoString"));
        assert!(content.contains("StringMut"));
    }

    #[test]
    fn proto2_utf8_does_not_override_editions_none() {
        let mut request = desc::request(vec![ProtoFile {
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
        request.meta.parameter = Some("proto2_utf8=verify".into());
        let content = generate_lib(&request);
        assert!(content.contains("ProtoStringUnchecked"));
        assert!(content.contains("BytesMut"));
    }

    #[test]
    fn proto2_utf8_unknown_value_is_error() {
        let mut request = empty_request("M");
        request.meta.parameter = Some("proto2_utf8=utf8".into());
        let err = generate(&request).unwrap_err();
        assert!(err.to_string().contains("proto2_utf8"));
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
        let content = generate_lib(&request);
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
        let content = generate_lib(&request);
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
    fn generate_official_descriptor_and_plugin_proto() {
        let fds = official_file_descriptor_set();
        let request = request_from_descriptor_set(&fds, &[DESCRIPTOR_PROTO, PLUGIN_PROTO]);
        assert_eq!(
            request.meta.file_to_generate,
            vec![DESCRIPTOR_PROTO, PLUGIN_PROTO]
        );
        assert_eq!(request.proto_files.len(), 2);

        let response = generate(&request).unwrap();
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].name, "lib.rs");
    }
}
