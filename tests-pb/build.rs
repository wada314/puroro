use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../puroro");
    println!("cargo:rerun-if-changed=../puroro-internal");
    println!("cargo:rerun-if-changed=../puroro-plugin");
    println!("cargo:rerun-if-changed=../purotobuf");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=protos/*.proto");

    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_status = Command::new("cargo")
        .args(&[
            "build",
            "-v",
            "--bin",
            "puroro-plugin",
            "-p",
            "puroro-plugin",
            "--target-dir",
        ])
        .arg(&out_dir)
        .status()
        .unwrap();
    if !cargo_status.success() {
        println!("cargo:warning=Failed to run `cargo build -p puroro-plugin`.");
        panic!("Failed to run `cargo build -p puroro-plugin`.")
    }

    let plugin_exe_path = [&out_dir, "debug", "puroro-plugin.exe"]
        .iter()
        .collect::<PathBuf>();
    let output_rust_path = [&out_dir].iter().collect::<PathBuf>();

    let protoc_exe = env::var("PURORO_PROTOC_PATH").unwrap_or("protoc".to_string());
    let protoc_status = Command::new(&protoc_exe)
        .arg("protos/*.proto")
        .arg(format!(
            "--plugin=protoc-gen-rust={}",
            plugin_exe_path.to_str().unwrap()
        ))
        .arg(format!("--rust_out={}", output_rust_path.to_str().unwrap()))
        .arg(format!("--proto_path={}", "./protos/"))
        .status()
        .unwrap();
    if !protoc_status.success() {
        println!("cargo:warning=Failed to run `protoc` command.");
        panic!("Failed to run `protoc` command.")
    }
}
