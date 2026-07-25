//! Build script: discover `fixtures/<case>/`, run real `protoc` with the
//! `protoc-gen-puroro` plugin, and wire that case's `test.rs` into the lib's
//! unit tests.
//!
//! Layout per case:
//! ```text
//! fixtures/<case>/
//!   *.proto     # one or more schemas (all passed to protoc)
//!   test.rs     # behavioural tests for the generated module `crate::<case>`
//! ```
//!
//! The plugin binary is built into a nested `CARGO_TARGET_DIR` under `OUT_DIR`
//! so this script never re-enters the parent cargo lock.

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let fixtures_src = manifest_dir.join("fixtures");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let generated_dir = out_dir.join("generated");
    fs::create_dir_all(&generated_dir).expect("create generated dir");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", fixtures_src.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("../protoc-gen-puroro/src").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir
            .join("../protoc-gen-puroro/Cargo.toml")
            .display()
    );
    println!("cargo:rerun-if-env-changed=PROTOC");
    println!("cargo:rerun-if-env-changed=PROTOC_GEN_PURORO");

    let protoc = resolve_protoc();
    let plugin = resolve_plugin_bin(&manifest_dir, &out_dir);

    let mut cases = list_fixture_cases(&fixtures_src);
    cases.sort_by(|a, b| a.module.cmp(&b.module));

    let mut generated = fs::File::create(out_dir.join("generated.rs")).expect("generated.rs");
    let mut case_tests = fs::File::create(out_dir.join("case_tests.rs")).expect("case_tests.rs");

    for case in &cases {
        println!("cargo:rerun-if-changed={}", case.dir.display());
        for proto in &case.proto_paths {
            println!("cargo:rerun-if-changed={}", proto.display());
        }
        println!("cargo:rerun-if-changed={}", case.test_path.display());

        let case_out = generated_dir.join(&case.module);
        if case_out.exists() {
            fs::remove_dir_all(&case_out).expect("clean previous generated case dir");
        }
        fs::create_dir_all(&case_out).expect("create case out dir");

        run_protoc(&protoc, &plugin, case, &case_out);

        let rs_path = find_single_generated_rs(&case_out, &case.module);
        let gen_path = rs_path.to_str().expect("OUT_DIR path must be UTF-8");
        // Embed under a private module so the public `crate::<case>` name does not
        // collide with the message submodule inside the generated forest
        // (`clippy::module_inception`).
        writeln!(
            generated,
            "#[path = r\"{gen_path}\"]\nmod {0}_generated;\npub mod {0} {{\n    pub use super::{0}_generated::*;\n}}",
            case.module
        )
        .expect("write generated module wrapper");

        let test_path = case.test_path.to_str().expect("fixture path must be UTF-8");
        writeln!(
            case_tests,
            "#[path = r\"{test_path}\"]\nmod {};",
            case.module
        )
        .expect("write case test module wrapper");
    }
}

fn resolve_protoc() -> PathBuf {
    if let Some(p) = env::var_os("PROTOC") {
        return PathBuf::from(p);
    }
    PathBuf::from("protoc")
}

fn resolve_plugin_bin(manifest_dir: &Path, out_dir: &Path) -> PathBuf {
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

    // Nested target dir avoids re-entering the parent cargo lock. Incremental
    // builds keep this cheap when the plugin is already up to date.
    build_plugin_in_nested_target(manifest_dir, out_dir)
}

fn build_plugin_in_nested_target(manifest_dir: &Path, out_dir: &Path) -> PathBuf {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());
    let nested_target = out_dir.join("plugin-target");
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let workspace_root = manifest_dir
        .parent()
        .unwrap_or_else(|| panic!("expected workspace member under {}", manifest_dir.display()));

    let mut cmd = Command::new(&cargo);
    cmd.arg("build")
        .args(["-p", "protoc-gen-puroro", "--bin", "protoc-gen-puroro"])
        .env("CARGO_TARGET_DIR", &nested_target)
        .current_dir(workspace_root);
    if profile == "release" {
        cmd.arg("--release");
    }

    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("failed to spawn cargo to build protoc-gen-puroro: {e}"));
    if !status.success() {
        panic!("failed to build protoc-gen-puroro plugin (status {status})");
    }

    let bin = nested_target.join(&profile).join("protoc-gen-puroro");
    if !bin.is_file() {
        panic!(
            "protoc-gen-puroro binary missing after nested cargo build: {}",
            bin.display()
        );
    }
    bin
}

fn run_protoc(protoc: &Path, plugin: &Path, case: &FixtureCase, case_out: &Path) {
    let mut cmd = Command::new(protoc);
    cmd.arg(format!("--plugin=protoc-gen-puroro={}", plugin.display()))
        .arg(format!("--puroro_out={}", case_out.display()))
        .arg(format!("-I{}", case.dir.display()));
    for proto in &case.proto_paths {
        let rel = proto.strip_prefix(&case.dir).unwrap_or(proto);
        cmd.arg(rel);
    }

    let output = cmd
        .output()
        .unwrap_or_else(|e| panic!("failed to spawn `{}`: {e}", protoc.display()));
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "protoc failed for fixture `{}` (status {}):\n\
             command: {cmd:?}\n\
             stdout:\n{stdout}\n\
             stderr:\n{stderr}",
            case.dir.display(),
            output.status,
        );
    }
}

fn find_single_generated_rs(case_out: &Path, module: &str) -> PathBuf {
    let mut files = Vec::new();
    collect_rs_files(case_out, &mut files);
    match files.len() {
        1 => files.pop().unwrap(),
        0 => panic!(
            "protoc/plugin produced no `.rs` file for fixture `{module}` under {}",
            case_out.display()
        ),
        n => panic!(
            "protoc/plugin produced {n} `.rs` files for fixture `{module}` under {}; expected 1 \
             (multi-file schemas should emit a single shared forest file)",
            case_out.display()
        ),
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read generated dir `{}`: {e}", dir.display()))
    {
        let entry = entry.expect("read generated dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

struct FixtureCase {
    /// Directory name → Rust module name (`crate::<module>`).
    module: String,
    dir: PathBuf,
    /// All `.proto` files in the fixture, sorted by file name.
    proto_paths: Vec<PathBuf>,
    test_path: PathBuf,
}

fn list_fixture_cases(fixtures_src: &Path) -> Vec<FixtureCase> {
    let entries = fs::read_dir(fixtures_src).unwrap_or_else(|e| {
        panic!(
            "failed to read fixtures dir `{}`: {e}",
            fixtures_src.display()
        )
    });

    let mut cases = Vec::new();
    for entry in entries {
        let entry = entry.expect("read fixtures dir entry");
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }

        let module = dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_else(|| panic!("non-UTF8 fixture dir: {}", dir.display()))
            .to_owned();
        if !is_simple_ident(&module) {
            panic!(
                "fixture directory name `{module}` must be a simple Rust module identifier \
                 (got {})",
                dir.display()
            );
        }

        let proto_paths = list_protos(&dir);
        let test_path = dir.join("test.rs");
        if !test_path.is_file() {
            panic!("fixture `{}` is missing required `test.rs`", dir.display());
        }

        cases.push(FixtureCase {
            module,
            dir,
            proto_paths,
            test_path,
        });
    }
    cases
}

fn list_protos(case_dir: &Path) -> Vec<PathBuf> {
    let mut protos = Vec::new();
    collect_protos(case_dir, &mut protos);
    if protos.is_empty() {
        panic!(
            "fixture `{}` must contain at least one `.proto` file",
            case_dir.display()
        );
    }
    protos.sort();
    protos
}

fn collect_protos(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read fixture case dir") {
        let entry = entry.expect("read fixture case entry");
        let path = entry.path();
        if path.is_dir() {
            collect_protos(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "proto") && path.is_file() {
            out.push(path);
        }
    }
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
