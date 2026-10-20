//! Resolve the crates generated `lib.rs` imports, and pass those package
//! versions into the plugin with `cargo:rustc-env`.
//!
//! The plugin writes them into `Cargo.toml` when it runs. A installed binary
//! does not resolve the registry again: the versions are the ones in the
//! `Cargo.lock` Cargo used to compile this binary.
//!
//! In this workspace that lock is `../Cargo.lock`. `cargo install` writes a
//! lock next to this package; the build-dependencies below put the same crates
//! in that lock.

use ::std::collections::HashMap;
use ::std::env;
use ::std::fs;
use ::std::path::{Path, PathBuf};

// Referenced so Cargo treats these build-dependencies as used. Their package
// versions are read from Cargo.lock; this build script does not call them.
extern crate allocator_api2 as _;
extern crate bitvec as _;
extern crate bytes as _;
extern crate puroro as _;
extern crate puroro_rt as _;

const WANTED: &[&str] = &["allocator-api2", "bitvec", "bytes", "puroro", "puroro-rt"];

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let lock_path = lock_path(&manifest_dir);
    println!("cargo:rerun-if-changed={}", lock_path.display());

    let text = fs::read_to_string(&lock_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", lock_path.display()));
    let versions = package_versions(&text);
    for name in WANTED {
        let version = versions.get(*name).unwrap_or_else(|| {
            panic!(
                "{name} is not in {} (it must be a dependency of protoc-gen-puroro)",
                lock_path.display()
            )
        });
        let env_key = format!(
            "PURORO_GEN_DEP_{}",
            name.to_ascii_uppercase().replace('-', "_")
        );
        println!("cargo:rustc-env={env_key}={version}");
    }
}

fn lock_path(manifest_dir: &Path) -> PathBuf {
    let local = manifest_dir.join("Cargo.lock");
    if local.is_file() {
        return local;
    }
    let parent_lock = manifest_dir.join("../Cargo.lock");
    let parent_manifest = manifest_dir.join("../Cargo.toml");
    if parent_lock.is_file() && is_puroro_workspace(&parent_manifest) {
        return parent_lock;
    }
    panic!(
        "Cargo.lock not found next to {} or in the puroro workspace",
        manifest_dir.display()
    );
}

fn is_puroro_workspace(manifest: &Path) -> bool {
    let Ok(text) = fs::read_to_string(manifest) else {
        return false;
    };
    text.contains("name = \"puroro\"") && text.contains("protoc-gen-puroro")
}

/// `name -> version` from Cargo.lock `[[package]]` entries.
///
/// Two entries with the same name and different versions are an error: the
/// generated manifest can name each crate only once.
fn package_versions(lock: &str) -> HashMap<String, String> {
    let mut versions = HashMap::<String, String>::new();
    let mut name: Option<String> = None;
    let mut version: Option<String> = None;
    for line in lock.lines() {
        if line == "[[package]]" {
            record(&mut versions, &mut name, &mut version);
            continue;
        }
        if let Some(value) = quoted_field(line, "name") {
            name = Some(value);
        } else if let Some(value) = quoted_field(line, "version") {
            version = Some(value);
        }
    }
    record(&mut versions, &mut name, &mut version);
    versions
}

fn record(
    versions: &mut HashMap<String, String>,
    name: &mut Option<String>,
    version: &mut Option<String>,
) {
    let (Some(name), Some(version)) = (name.take(), version.take()) else {
        return;
    };
    if !WANTED.contains(&name.as_str()) {
        return;
    }
    if let Some(prev) = versions.insert(name.clone(), version.clone()) {
        if prev != version {
            panic!("{name} appears as both {prev} and {version} in Cargo.lock");
        }
    }
}

fn quoted_field(line: &str, field: &str) -> Option<String> {
    let prefix = format!("{field} = \"");
    let rest = line.trim().strip_prefix(&prefix)?;
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}
