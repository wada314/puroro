use ::std::env;
use ::std::path::PathBuf;
use ::std::process::Command;

fn main() {
    //println!("cargo:warning=OUT_DIR = {}", env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=../puroro/**/*");
    println!("cargo:rerun-if-changed=../puroro-internal/**/*");
    println!("cargo:rerun-if-changed=../puroro-plugin/**/*");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("cargo")
        .args(&["build", "-v", "-p", "puroro-plugin", "--target-dir"])
        .arg(&out_dir)
        .status()
        .unwrap();

    let plugin_exe_path = [&out_dir, "debug", "puroro-plugin.exe"]
        .iter()
        .collect::<PathBuf>();
    let output_rust_path = [&out_dir].iter().collect::<PathBuf>();

    let protoc_exe = env::var("PURORO_PROTOC_PATH").unwrap_or("protoc".to_string());
    let status = Command::new(&protoc_exe)
        .arg("../protobuf/src/google/protobuf/compiler/plugin.proto")
        .arg(format!(
            "--plugin=protoc-gen-rust={}",
            plugin_exe_path.to_str().unwrap()
        ))
        .arg(format!("--rust_out={}", output_rust_path.to_str().unwrap()))
        .arg(format!("--proto_path={}", "../protobuf/src"))
        .status()
        .unwrap();

    println!("cargo:warning=WARNING = {:?}", status);
    println!("cargo:warning=WARNING = {:?}", plugin_exe_path.to_str());
    println!("cargo:warning=WARNING = {:?}", output_rust_path.to_str());
}
