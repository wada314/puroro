//! Generate `google.protobuf` Rust sources with `protoc-gen-puroro`.
//!
//! Feature flags select which `.proto` files are passed to `protoc`. The
//! plugin writes a single `lib.rs` forest into `OUT_DIR`; [`src/lib.rs`](src/lib.rs)
//! `include!`s it at the crate root so generated `#![…]` attrs apply.

use ::std::env;
use ::std::fs;
use ::std::path::{Path, PathBuf};
use ::std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let proto_root = manifest_dir.join("proto");
    let stamp = out_dir.join("generated_lib.rs");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", proto_root.display());
    println!("cargo:rerun-if-env-changed=PROTOC");
    println!("cargo:rerun-if-env-changed=PROTOC_GEN_PURORO");

    let mut protos: Vec<&str> = Vec::new();
    if cfg!(feature = "descriptors") {
        protos.push("google/protobuf/descriptor.proto");
    }
    if cfg!(feature = "wkt") {
        protos.extend([
            "google/protobuf/any.proto",
            "google/protobuf/duration.proto",
            "google/protobuf/empty.proto",
            "google/protobuf/field_mask.proto",
            "google/protobuf/struct.proto",
            "google/protobuf/timestamp.proto",
            "google/protobuf/wrappers.proto",
        ]);
    }

    if protos.is_empty() {
        fs::write(
            &stamp,
            "// No `descriptors` / `wkt` features enabled; empty generated body.\n",
        )
        .expect("write empty generated stub");
        return;
    }

    for rel in &protos {
        println!("cargo:rerun-if-changed={}", proto_root.join(rel).display());
    }

    let gen_dir = out_dir.join("protoc_out");
    if gen_dir.exists() {
        fs::remove_dir_all(&gen_dir).expect("clean previous generated dir");
    }
    fs::create_dir_all(&gen_dir).expect("create generated dir");

    let protoc = resolve_protoc();
    let plugin = resolve_plugin_bin();
    run_protoc(&protoc, &plugin, &proto_root, &gen_dir, &protos);

    let lib_rs = gen_dir.join("lib.rs");
    if !lib_rs.is_file() {
        panic!(
            "protoc-gen-puroro did not write lib.rs under {}",
            gen_dir.display()
        );
    }
    let generated = fs::read_to_string(&lib_rs).expect("read generated lib.rs");
    // Drop the plugin's crate-level doc / `#![…]` so this file can be
    // `include!`d after the hand-written crate docs and allows in `src/lib.rs`.
    let body = strip_crate_header(&generated);
    fs::write(&stamp, body).expect("write generated body into OUT_DIR");
}

/// Remove a leading `/*! … */` and crate-level `#![…]` lines from plugin output.
fn strip_crate_header(src: &str) -> String {
    let mut rest = src;
    let trimmed = rest.trim_start();
    if let Some(after) = trimmed.strip_prefix("/*!") {
        if let Some(end) = after.find("*/") {
            rest = after[end + 2..].trim_start();
        }
    }
    let mut out = String::new();
    for line in rest.lines() {
        let t = line.trim_start();
        if t.starts_with("#![") {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn resolve_protoc() -> PathBuf {
    env::var_os("PROTOC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("protoc"))
}

fn resolve_plugin_bin() -> PathBuf {
    if let Some(p) = env::var_os("PROTOC_GEN_PURORO") {
        let path = PathBuf::from(p);
        if !path.is_file() {
            panic!(
                "PROTOC_GEN_PURORO=`{}` does not point to an existing file",
                path.display()
            );
        }
        return path;
    }
    let path = PathBuf::from(
        env::var_os("CARGO_BIN_FILE_PROTOC_GEN_PURORO").unwrap_or_else(|| {
            panic!(
                "CARGO_BIN_FILE_PROTOC_GEN_PURORO unset; need nightly Cargo `-Z bindeps` \
                 (see `.cargo/config.toml`) or set PROTOC_GEN_PURORO"
            )
        }),
    );
    if !path.is_file() {
        panic!("artifact dependency binary missing: {}", path.display());
    }
    path
}

fn run_protoc(protoc: &Path, plugin: &Path, proto_root: &Path, out_dir: &Path, protos: &[&str]) {
    let mut cmd = Command::new(protoc);
    cmd.arg(format!("--plugin=protoc-gen-puroro={}", plugin.display()))
        .arg(format!("--puroro_out={}", out_dir.display()))
        .arg("--puroro_opt=crate_name=puroro-types")
        .arg(format!("-I{}", proto_root.display()));
    for rel in protos {
        cmd.arg(rel);
    }
    let output = cmd
        .output()
        .unwrap_or_else(|e| panic!("failed to spawn `{}`: {e}", protoc.display()));
    if !output.status.success() {
        panic!(
            "protoc failed (status {}):\ncommand: {cmd:?}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}
