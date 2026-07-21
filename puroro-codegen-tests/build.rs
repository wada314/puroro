//! Build script: run the (fake) code generator once per fixture into `OUT_DIR`.
//!
//! Generated modules are linked from `src/lib.rs` via `#[path]`, so hundreds of
//! cases still share a single `cargo test` compilation — no subprocess cargo
//! per case.

use ::protoc_gen_puroro::emit::emit;
use ::protoc_gen_puroro::ir::{CodegenRequest, MessageDesc, ProtoFile};
use ::std::env;
use ::std::fs;
use ::std::io::Write;
use ::std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let fixtures_dir = out_dir.join("fixtures");
    fs::create_dir_all(&fixtures_dir).expect("create fixtures dir");

    let mut generated = fs::File::create(out_dir.join("generated.rs")).expect("generated.rs");

    for fixture in fixtures() {
        let response = emit(&fixture.request)
            .unwrap_or_else(|e| panic!("codegen failed for fixture `{}`: {e}", fixture.module));
        assert_eq!(
            response.files.len(),
            1,
            "fixture `{}` must emit exactly one file",
            fixture.module
        );

        let rs_path = fixtures_dir.join(format!("{}.rs", fixture.module));
        fs::write(&rs_path, &response.files[0].content).expect("write fixture rs");

        // Use `#[path]` so the generated file is a real module root (`//!` docs ok).
        let path = rs_path.to_str().expect("OUT_DIR path must be UTF-8");
        writeln!(
            generated,
            "#[path = r\"{path}\"]\npub mod {};",
            fixture.module,
        )
        .expect("write module wrapper");
    }

    println!("cargo:rerun-if-changed=build.rs");
}

struct Fixture {
    /// Rust module name under `generated` (snake_case).
    module: &'static str,
    request: CodegenRequest,
}

fn fixtures() -> Vec<Fixture> {
    vec![Fixture {
        module: "empty",
        request: CodegenRequest {
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
        },
    }]
}
