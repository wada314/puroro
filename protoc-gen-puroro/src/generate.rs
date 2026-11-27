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
use crate::plugin_io::{CodeGeneratorResponse, ResponseFile, encode_file_descriptor_set};
use crate::resolved::{Arena, File, resolve_with};

/// Package name written into the generated `Cargo.toml` when `crate_name` is unset.
const DEFAULT_CRATE_NAME: &str = "generated";

/// One crate listed under `[dependencies]` in the generated `Cargo.toml`.
struct GeneratedDependency {
    name: &'static str,
    version: &'static str,
}

impl GeneratedDependency {
    /// One `[dependencies]` line, including the trailing newline.
    fn toml_line(&self) -> String {
        format!("{} = \"{}\"\n", self.name, self.version)
    }
}

/// Crates that generated `lib.rs` names directly, with the package version from
/// the `Cargo.lock` that compiled this plugin (`build.rs`).
const GENERATED_DEPENDENCIES: &[GeneratedDependency] = &[
    GeneratedDependency {
        name: "allocator-api2",
        version: env!("PURORO_GEN_DEP_ALLOCATOR_API2"),
    },
    GeneratedDependency {
        name: "bitvec",
        version: env!("PURORO_GEN_DEP_BITVEC"),
    },
    GeneratedDependency {
        name: "bytes",
        version: env!("PURORO_GEN_DEP_BYTES"),
    },
    GeneratedDependency {
        name: "puroro",
        version: env!("PURORO_GEN_DEP_PURORO"),
    },
    GeneratedDependency {
        name: "puroro-rt",
        version: env!("PURORO_GEN_DEP_PURORO_RT"),
    },
];

/// Generate-time knobs parsed from [`CodegenRequest`] `parameter`.
struct GenerateOptions {
    /// proto2 syntax default for `utf8_validation` (spec default is `NONE`).
    proto2_utf8: Utf8Validation,
    /// Cargo package name for the generated crate (`[lib] path = "lib.rs"`).
    crate_name: String,
    /// Emit experimental `NameLazy` aliases and enable `puroro-rt/lazy`.
    ///
    /// Off by default. Pass `--puroro_opt=lazy=true` (or bare `lazy`) to opt in.
    lazy: bool,
}

fn parse_generate_options(parameter: Option<&str>) -> Result<GenerateOptions> {
    let mut options = GenerateOptions {
        proto2_utf8: Utf8Validation::None,
        crate_name: DEFAULT_CRATE_NAME.to_owned(),
        lazy: false,
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
            // Bare `lazy` opts in without a value.
            if part == "lazy" {
                options.lazy = true;
            }
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key == "proto2_utf8" {
            options.proto2_utf8 = parse_proto2_utf8(value)?;
        } else if key == "crate_name" {
            options.crate_name = parse_crate_name(value)?;
        } else if key == "lazy" {
            options.lazy = parse_bool_flag(key, value)?;
        }
    }
    Ok(options)
}

/// Parse a `--puroro_opt` boolean value (`true`/`false`, plus common aliases).
fn parse_bool_flag(key: &str, value: &str) -> Result<bool> {
    match value {
        "true" | "TRUE" | "1" | "yes" | "YES" => Ok(true),
        "false" | "FALSE" | "0" | "no" | "NO" => Ok(false),
        other => Err(Error::Codegen(format!(
            "unknown `{key}` value `{other}` (expected `true` or `false`)"
        ))),
    }
}

fn parse_crate_name(value: &str) -> Result<String> {
    let mut chars = value.chars();
    let ok = match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {
            chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        }
        _ => false,
    };
    if !ok {
        return Err(Error::Codegen(format!(
            "invalid `crate_name` `{value}` (expected a lowercase Cargo package name)"
        )));
    }
    Ok(value.to_owned())
}

/// Manifest for the output directory. `lib.rs` is the crate root (`[lib] path`).
///
/// `protoc` overwrites this file on every run. Dependency requirements come
/// from [`GENERATED_DEPENDENCIES`]. When `lazy` is on, `puroro-rt` enables its
/// experimental `lazy` feature.
fn cargo_manifest(crate_name: &str, lazy: bool) -> ResponseFile {
    let mut dependencies = String::new();
    for dep in GENERATED_DEPENDENCIES {
        if dep.name == "puroro-rt" && lazy {
            dependencies.push_str(&format!(
                "{} = {{ version = \"{}\", features = [\"lazy\"] }}\n",
                dep.name, dep.version
            ));
        } else {
            dependencies.push_str(&dep.toml_line());
        }
    }
    let lazy_comment = if lazy {
        "# Generated with `--puroro_opt=lazy=true` (experimental NameLazy aliases).\n"
    } else {
        ""
    };
    ResponseFile {
        name: "Cargo.toml".to_owned(),
        content: format!(
            "\
# @generated by protoc-gen-puroro. Regenerating overwrites this file.
# `name` under [package] is this crate's name.
# This crate re-exports the runtime as `puroro`. Application crates depend on this crate. They do not import `puroro-rt`.
{lazy_comment}
[package]
name = \"{crate_name}\"
version = \"0.0.0\"
edition = \"2024\"
publish = false

[lib]
path = \"lib.rs\"
# Proto comments often contain non-Rust examples; do not compile them as doctests.
doctest = false

[features]
default = []
# Embed encoded `FileDescriptorSet` (raw CodeGeneratorRequest.proto_file bodies)
# as `descriptors::FILE_DESCRIPTOR_SET`. `Message::FULL_NAME` needs no feature.
descriptors = []

[dependencies]
{dependencies}"
        ),
    }
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

    if !options.lazy {
        for file in &targets {
            reject_lazy_child_parse(file)?;
        }
    }

    let mut forest = ModuleForest::new();
    forest
        .root_mut()
        .append_inner_attrs(generated_file_attrs(&targets)?);

    let storage = plan_message_storage(&file_set);
    for file in &targets {
        forest.install_file(prepare_file(file, &storage, options.lazy)?);
    }

    let file_descriptor_set = encode_file_descriptor_set(&request.proto_file_raw)?;
    let mut files = render(
        &forest,
        &ModuleLayout::SingleFile {
            path: "lib.rs".into(),
        },
        &file_descriptor_set,
    )?;
    let file = files
        .pop()
        .ok_or_else(|| Error::internal("layout produced no files"))?;
    let manifest = cargo_manifest(&options.crate_name, options.lazy);
    Ok(CodeGeneratorResponse::from_files(vec![file, manifest]))
}

/// `(puroro.child_parse) = LAZY` needs `--puroro_opt=lazy=true`.
fn reject_lazy_child_parse(file: &File<'_>) -> Result<()> {
    use crate::descriptor::ChildParse;
    use crate::resolved::Message;

    fn walk(message: &Message<'_>) -> Result<()> {
        if message.child_parse() == ChildParse::Lazy {
            return Err(Error::Codegen(format!(
                "message `{}` sets `(puroro.message_child_parse) = LAZY`, which requires \
                 `--puroro_opt=lazy=true` (experimental lazy aliases are off by default)",
                message.fqn()
            )));
        }
        for field in message.fields() {
            if field.child_parse() == Some(ChildParse::Lazy) {
                return Err(Error::Codegen(format!(
                    "field `{}.{}` sets `(puroro.child_parse) = LAZY`, which requires \
                     `--puroro_opt=lazy=true` (experimental lazy aliases are off by default)",
                    message.fqn(),
                    field.name()
                )));
            }
        }
        for nested in message.nested_messages() {
            if nested.is_map_entry() {
                continue;
            }
            walk(nested)?;
        }
        Ok(())
    }

    for message in file.messages() {
        walk(message)?;
    }
    Ok(())
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
        BytesLayout, CodegenRequest, Comments, Edition, EnumDesc, EnumRepr, EnumValueDesc,
        FeatureSet, FieldDesc, FieldLabel, FieldType, MessageDesc, MessageLayout, ProtoFile,
        ProtoFqn, StringLayout, Syntax, UnknownFieldsPolicy,
    };
    use crate::plugin_io::views::{CodeGeneratorRequest, FileDescriptorSet};
    use crate::plugin_io::{decode_request, encode_file_descriptor_set};
    use ::protobuf_core::{AsRefExtProtobuf, Field, FieldNumber, FieldValue, WriteExtProtobuf};
    use ::std::env;
    use ::std::fs;
    use ::std::path::{Path, PathBuf};
    use ::std::process::{self, Command};
    use ::std::thread;
    use ::syn::{Expr, Item, Lit};

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

    fn file_level_enum_request(enum_name: &str, package: &str) -> CodegenRequest {
        desc::request(vec![ProtoFile {
            enums: vec![desc::enumeration(
                enum_name,
                vec![desc::enum_value("UNSPECIFIED", 0)],
            )],
            ..desc::proto_file("empty.proto", package)
        }])
    }

    fn generate_lib(request: &CodegenRequest) -> String {
        let response = generate(request).unwrap();
        assert_eq!(response.files.len(), 2);
        let manifest = response
            .files
            .iter()
            .find(|file| file.name == "Cargo.toml")
            .expect("Cargo.toml");
        assert!(manifest.content.contains("name = \"generated\""));
        assert!(manifest.content.contains("path = \"lib.rs\""));
        assert!(
            manifest.content.contains("doctest = false"),
            "generated crates must disable doctests: {}",
            manifest.content
        );
        response
            .files
            .iter()
            .find(|file| file.name == "lib.rs")
            .expect("lib.rs")
            .content
            .clone()
    }

    #[test]
    fn cargo_manifest_uses_crate_name() {
        let mut request = empty_request_with_package("Book", "library");
        request.meta.parameter = Some("crate_name=library-proto,proto2_utf8=none".into());
        let response = generate(&request).unwrap();
        let manifest = response
            .files
            .iter()
            .find(|file| file.name == "Cargo.toml")
            .expect("Cargo.toml");
        assert!(manifest.content.contains("name = \"library-proto\""));
        assert!(
            manifest.content.contains("doctest = false"),
            "missing doctest = false: {}",
            manifest.content
        );
        assert!(
            manifest.content.contains("descriptors = []"),
            "missing descriptors feature: {}",
            manifest.content
        );
        for dep in GENERATED_DEPENDENCIES {
            let line = dep.toml_line();
            let line = line.trim_end();
            assert!(manifest.content.contains(line), "missing {line}");
        }
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        assert_eq!(
            env!("PURORO_GEN_DEP_PURORO"),
            package_version(&root.join("Cargo.toml"))
        );
        assert_eq!(
            env!("PURORO_GEN_DEP_PURORO_RT"),
            package_version(&root.join("puroro-rt/Cargo.toml"))
        );
        let lib = response
            .files
            .iter()
            .find(|file| file.name == "lib.rs")
            .expect("lib.rs");
        assert!(lib.content.contains("pub mod library"));
    }

    fn package_version(path: &Path) -> String {
        let text =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let package = text
            .split("[package]")
            .nth(1)
            .unwrap_or_else(|| panic!("no [package] in {}", path.display()));
        let body = package.split('[').next().expect("package body");
        for line in body.lines() {
            if let Some(rest) = line.trim().strip_prefix("version = \"") {
                let end = rest.find('"').expect("version quote");
                return rest[..end].to_owned();
            }
        }
        panic!("no package version in {}", path.display());
    }

    #[test]
    fn crate_name_rejects_uppercase() {
        let mut request = empty_request("Empty");
        request.meta.parameter = Some("crate_name=Library".into());
        let err = generate(&request).unwrap_err();
        assert!(err.to_string().contains("crate_name"));
    }

    #[test]
    fn plugin_response_is_single_lib_rs_with_generated_header() {
        let content = generate_lib(&empty_request("Empty"));
        assert!(content.contains("@generated"));
        assert!(content.contains("- `empty.proto`"));
        assert!(content.contains("pub use ::puroro"));
    }

    #[test]
    fn proto_comments_become_rustdoc_on_types_and_getters() {
        let mut request = empty_request("Task");
        request.proto_files[0].messages[0].comments = Comments {
            leading: Some("A tracked work item.\n".into()),
            trailing: None,
        };
        request.proto_files[0].messages[0].fields = vec![FieldDesc {
            proto3_optional: true,
            comments: Comments {
                leading: Some("Human-readable title.\n".into()),
                trailing: None,
            },
            ..desc::field("title", 1, FieldType::String)
        }];
        request.proto_files[0].enums.push(EnumDesc {
            comments: Comments {
                leading: Some("Lifecycle state.\n".into()),
                trailing: None,
            },
            ..desc::enumeration(
                "Status",
                vec![
                    EnumValueDesc {
                        comments: Comments {
                            leading: Some("Not started.\n".into()),
                            trailing: None,
                        },
                        ..desc::enum_value("STATUS_UNSPECIFIED", 0)
                    },
                    desc::enum_value("STATUS_DONE", 1),
                ],
            )
        });
        let content = generate_lib(&request);
        assert!(
            content.contains("A tracked work item."),
            "message docs missing: {content}"
        );
        assert!(
            content.contains("Human-readable title."),
            "field docs missing: {content}"
        );
        assert!(
            content.contains("Lifecycle state."),
            "enum docs missing: {content}"
        );
        assert!(
            content.contains("Not started."),
            "enum value docs missing: {content}"
        );
    }

    #[test]
    fn reserved_crate_root_names_are_rejected() {
        let cases: [(&str, CodegenRequest, Option<&str>); 5] = [
            (
                "package puroro",
                empty_request_with_package("Empty", "puroro"),
                None,
            ),
            (
                "package descriptors",
                empty_request_with_package("Empty", "descriptors"),
                Some("descriptors"),
            ),
            ("message Puroro", empty_request("Puroro"), None),
            (
                "message Descriptors",
                empty_request("Descriptors"),
                Some("descriptors"),
            ),
            ("enum puroro", file_level_enum_request("puroro", ""), None),
        ];
        for (label, req, also_contains) in cases {
            let err = generate(&req).unwrap_err().to_string();
            assert!(
                err.contains("reserved"),
                "{label}: expected reserved error, got {err}"
            );
            if let Some(extra) = also_contains {
                assert!(
                    err.contains(extra),
                    "{label}: expected error to mention {extra:?}, got {err}"
                );
            }
        }
    }

    #[test]
    fn file_level_enum_puroro_pascal_is_not_the_root_module() {
        let content = generate_lib(&file_level_enum_request("Puroro", ""));
        assert!(content.contains("pub use ::puroro"));
        assert!(content.contains("struct Puroro"));
        assert!(!content.contains("mod puroro"));
    }

    #[test]
    fn nested_package_puroro_is_not_the_crate_root() {
        let content = generate_lib(&empty_request_with_package("Empty", "shop.puroro"));
        assert!(content.contains("pub mod shop"));
        assert!(content.contains("pub mod puroro"));
        assert!(content.contains("pub use ::puroro"));
    }

    #[test]
    fn empty_message_companion_holds_layout() {
        let content = generate_lib(&empty_request_with_package("Empty", "example.v1"));
        assert!(content.contains("pub mod empty"));
        assert!(content.contains("pub trait Layout"));
        assert!(content.contains("pub struct Empty<"));
        // Lazy aliases are opt-in (`--puroro_opt=lazy=true`).
        assert!(!content.contains("pub struct EmptyLazy<"));
        assert!(!content.contains("pub use empty::Empty"));
    }

    #[test]
    fn lazy_opt_in_emits_lazy_alias_and_rt_feature() {
        let mut request = empty_request_with_package("Empty", "example.v1");
        request.meta.parameter = Some("lazy=true".into());
        let response = generate(&request).unwrap();
        let lib = response
            .files
            .iter()
            .find(|file| file.name == "lib.rs")
            .expect("lib.rs");
        let manifest = response
            .files
            .iter()
            .find(|file| file.name == "Cargo.toml")
            .expect("Cargo.toml");
        assert!(lib.content.contains("pub struct EmptyLazy<"));
        assert!(lib.content.contains("Experimental lazy alias"));
        assert!(manifest.content.contains("features = [\"lazy\"]"));
        assert!(manifest.content.contains("puroro_opt=lazy=true"));
    }

    #[test]
    fn bare_lazy_parameter_opts_in() {
        let mut request = empty_request("Empty");
        request.meta.parameter = Some("lazy".into());
        let content = generate_lib(&request);
        assert!(content.contains("pub struct EmptyLazy<"));
    }

    #[test]
    fn lazy_child_parse_requires_lazy_opt_in() {
        use crate::descriptor::ChildParse;
        let mut request = empty_request("Parent");
        request.proto_files[0].messages.push(desc::message("Child"));
        request.proto_files[0].messages[0].fields.push(FieldDesc {
            type_name: Some(ProtoFqn::parse(".Child")),
            child_parse: Some(ChildParse::Lazy),
            ..desc::field("child", 1, FieldType::Message)
        });
        let err = generate(&request).unwrap_err().to_string();
        assert!(
            err.contains("lazy=true"),
            "expected lazy opt-in error, got {err}"
        );
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
    fn rust_enum_repr_emits_native_enum() {
        let mut request = empty_request("Holder");
        request.proto_files[0].enums.push(EnumDesc {
            enum_repr: EnumRepr::RustEnum,
            ..desc::enumeration(
                "Status",
                vec![
                    desc::enum_value("STATUS_UNSPECIFIED", 0),
                    desc::enum_value("STATUS_PENDING", 1),
                ],
            )
        });
        request.proto_files[0].messages[0].fields.push(FieldDesc {
            type_name: Some(ProtoFqn::parse(".Status")),
            ..desc::field("status", 1, FieldType::Enum)
        });
        let content = generate_lib(&request);
        assert!(
            content.contains("pub enum Status"),
            "rust_enum must emit a native enum: {content}"
        );
        assert!(
            content.contains("_Unknown"),
            "open rust_enum must keep unknown values: {content}"
        );
        assert!(
            !content.contains("repr(transparent)"),
            "rust_enum must not emit the newtype: {content}"
        );
    }

    #[test]
    fn rust_enum_repr_rejects_aliased_numbers() {
        let mut request = empty_request("Holder");
        request.proto_files[0].enums.push(EnumDesc {
            enum_repr: EnumRepr::RustEnum,
            ..desc::enumeration(
                "Code",
                vec![
                    desc::enum_value("CODE_UNSPECIFIED", 0),
                    desc::enum_value("CODE_OK", 1),
                    desc::enum_value("CODE_SUCCESS", 1),
                ],
            )
        });
        request.proto_files[0].messages[0].fields.push(FieldDesc {
            type_name: Some(ProtoFqn::parse(".Code")),
            ..desc::field("code", 1, FieldType::Enum)
        });
        let err = generate(&request).unwrap_err();
        let message = err.to_string();
        assert!(
            message.contains("cannot generate a Rust enum for `Code`"),
            "{message}"
        );
        assert!(message.contains("CODE_OK"), "{message}");
        assert!(message.contains("CODE_SUCCESS"), "{message}");
        assert!(message.contains("number 1"), "{message}");
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
            content.contains("::puroro_rt::forward_record!(eager Marker, MarkerFields)"),
            "discard still uses the Record shell, which calls skip_field_and_save_in: {content}"
        );
    }

    #[test]
    fn reject_unknowns_bakes_store() {
        let mut request = empty_request("Strict");
        request.proto_files[0].messages[0].unknown_fields = UnknownFieldsPolicy::Reject;
        request.proto_files[0].messages[0].fields = vec![desc::field("n", 1, FieldType::Int32)];
        let content = generate_lib(&request);
        assert!(
            content.contains("RejectUnknowns"),
            "reject must bake RejectUnknowns: {content}"
        );
        assert!(
            !content.contains("DiscardUnknowns"),
            "reject must not bake DiscardUnknowns: {content}"
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
        assert!(
            content.contains("pub fn set_origin"),
            "singular message fields move-replace via set_*: {content}"
        );
        assert!(
            content.contains("*self.origin_mut() = src"),
            "follow-parent set_origin assigns the eager child: {content}"
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
        assert!(
            content.contains("*self.assignee_mut() = src"),
            "boxed follow-parent set_assignee assigns the eager child: {content}"
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
        assert_eq!(
            content.matches("fn after_deallocate").count(),
            1,
            "eager-only oneof group wipes parent bits once: {content}"
        );

        let mut lazy_request = request;
        lazy_request.meta.parameter = Some("lazy=true".into());
        let lazy_content = generate_lib(&lazy_request);
        assert_eq!(
            lazy_content.matches("fn after_deallocate").count(),
            2,
            "eager and lazy oneof groups both wipe parent bits: {lazy_content}"
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
                FieldNumber::try_new(CodeGeneratorRequest::FILE_TO_GENERATE)
                    .expect("file_to_generate field number"),
                FieldValue::Len(name.as_bytes()),
            );
            request
                .write_protobuf_field(&field)
                .expect("write file_to_generate");
        }
        for field in AsRefExtProtobuf::read_protobuf_fields(&descriptor_set) {
            let field = field.expect("FileDescriptorSet field");
            if field.field_number.as_u32() != FileDescriptorSet::FILE {
                continue;
            }
            let FieldValue::Len(bytes) = field.value else {
                panic!("FileDescriptorSet.file must be length-delimited");
            };
            let wrapped = Field::new(
                FieldNumber::try_new(CodeGeneratorRequest::PROTO_FILE)
                    .expect("proto_file field number"),
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
        assert_eq!(request.proto_file_raw.len(), 2);

        let expected_set = encode_file_descriptor_set(&request.proto_file_raw).unwrap();
        assert_eq!(expected_set, fds);

        let response = generate(&request).unwrap();
        assert_eq!(response.files.len(), 2);
        let lib = response
            .files
            .iter()
            .find(|file| file.name == "lib.rs")
            .expect("lib.rs");
        assert!(response.files.iter().any(|file| file.name == "Cargo.toml"));

        let embedded = extract_file_descriptor_set_literal(&lib.content);
        assert_eq!(embedded, expected_set);
    }

    /// Pull `descriptors::FILE_DESCRIPTOR_SET`'s byte literal out of generated Rust.
    fn extract_file_descriptor_set_literal(lib_rs: &str) -> Vec<u8> {
        let file: syn::File = syn::parse_str(lib_rs).expect("parse generated lib.rs");
        for item in file.items {
            let Item::Mod(module) = item else {
                continue;
            };
            if module.ident != "descriptors" {
                continue;
            }
            let (_, items) = module.content.expect("descriptors mod has a body");
            for item in items {
                let Item::Const(item) = item else {
                    continue;
                };
                if item.ident != "FILE_DESCRIPTOR_SET" {
                    continue;
                }
                let Expr::Lit(expr) = *item.expr else {
                    panic!("FILE_DESCRIPTOR_SET is not a literal");
                };
                let Lit::ByteStr(lit) = expr.lit else {
                    panic!("FILE_DESCRIPTOR_SET is not a byte string");
                };
                return lit.value();
            }
            panic!("FILE_DESCRIPTOR_SET const missing");
        }
        panic!("descriptors module missing");
    }
}
