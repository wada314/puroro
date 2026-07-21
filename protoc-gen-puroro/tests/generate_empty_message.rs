//! Compile and run the fake empty-message generator output.
//!
//! Strategy: emit Rust → write a temporary crate → `cargo test` that crate.
//! Assertions live in the generated harness (runtime behaviour), not in a
//! string diff of the emitter output.

use ::protoc_gen_puroro::emit::emit;
use ::protoc_gen_puroro::ir::{CodegenRequest, MessageDesc, ProtoFile};
use ::std::fs;
use ::std::path::{Path, PathBuf};
use ::std::process::Command;
use ::tempfile::TempDir;

#[test]
fn generated_empty_message_compiles_and_behaves() {
    let workspace_root = workspace_root();
    let response = emit(&empty_request()).expect("emit empty message");
    assert_eq!(response.files.len(), 1);

    let tmp = TempDir::new().expect("tempdir");
    let harness_root = tmp.path();
    write_harness(
        harness_root,
        &workspace_root,
        &response.files[0].content,
        BEHAVIOR_TESTS,
    );

    let status = Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(harness_root.join("Cargo.toml"))
        .arg("--quiet")
        // Keep artefacts out of the main workspace target/.
        .env("CARGO_TARGET_DIR", harness_root.join("target"))
        .status()
        .expect("spawn cargo test");

    assert!(
        status.success(),
        "cargo test failed for generated empty-message harness (status {status})"
    );
}

fn empty_request() -> CodegenRequest {
    CodegenRequest {
        file_to_generate: vec!["empty.proto".into()],
        parameter: None,
        proto_files: vec![ProtoFile {
            name: "empty.proto".into(),
            package: String::new(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "Empty".into(),
                fields: vec![],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            enums: vec![],
        }],
    }
}

fn workspace_root() -> PathBuf {
    // tests/ → crate root → workspace root
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .to_path_buf()
}

fn write_harness(root: &Path, workspace_root: &Path, generated_rs: &str, behavior_rs: &str) {
    let src = root.join("src");
    let tests = root.join("tests");
    fs::create_dir_all(&src).unwrap();
    fs::create_dir_all(&tests).unwrap();

    let puroro = workspace_root;
    let puroro_rt = workspace_root.join("puroro-rt");

    let cargo_toml = format!(
        r#"[package]
name = "puroro-gen-empty-harness"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
allocator-api2 = "0.2"
bitvec = "1"
bytes = "1"
puroro = {{ path = "{puroro}" }}
puroro-rt = {{ path = "{puroro_rt}" }}
"#,
        puroro = puroro.display(),
        puroro_rt = puroro_rt.display(),
    );

    fs::write(root.join("Cargo.toml"), cargo_toml).unwrap();
    fs::write(
        src.join("lib.rs"),
        "//! Temporary crate that hosts generator output for runtime tests.\n\n\
         #[allow(dead_code)]\n\
         mod generated;\n\
         pub use generated::*;\n",
    )
    .unwrap();
    fs::write(src.join("generated.rs"), generated_rs).unwrap();
    fs::write(tests.join("behavior.rs"), behavior_rs).unwrap();
}

/// Runtime checks compiled against the generated `Empty` type.
const BEHAVIOR_TESTS: &str = r#"
use ::puroro::Message;
use ::puroro_gen_empty_harness::Empty;

#[test]
fn default_encodes_to_empty() {
    let msg: Empty = Empty::new();
    assert_eq!(Message::encoded_len(&msg), 0);
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn decode_empty_bytes() {
    let msg: Empty = Empty::decode(&[][..]).expect("decode empty");
    assert_eq!(Message::encoded_len(&msg), 0);
    assert!(msg.encode_to_vec().is_empty());
}

#[test]
fn round_trip_unknown_field() {
    // field 1, varint 150
    let wire = [0x08_u8, 0x96, 0x01];
    let msg: Empty = Empty::decode(&wire[..]).expect("decode unknown");
    assert_eq!(msg.encode_to_vec(), wire);
    assert_eq!(msg.unknown_fields().count(), 1);
}

#[test]
fn clone_and_eq() {
    let a: Empty = Empty::new();
    let b = a.clone();
    assert_eq!(a, b);

    let wire = [0x08_u8, 0x01];
    let with_unknown: Empty = Empty::decode(&wire[..]).unwrap();
    assert_ne!(a, with_unknown);
    assert_eq!(with_unknown.clone(), with_unknown);
}
"#;
