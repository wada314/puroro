//! Build script: discover `fixtures/<case>/`, run the (fake) code generator for
//! each case's `.proto`, and wire that case's `test.rs` into the lib's unit tests.
//!
//! Layout per case:
//! ```text
//! fixtures/<case>/
//!   *.proto     # exactly one input schema
//!   test.rs     # behavioural tests for the generated module `crate::<case>`
//! ```

use ::protoc_gen_puroro::descriptor::{
    CodegenMeta, CodegenRequest, Edition, FeatureSet, MessageDesc, ProtoFile, Syntax,
};
use ::protoc_gen_puroro::emit::emit;
use ::std::env;
use ::std::fs;
use ::std::io::Write;
use ::std::path::{Path, PathBuf};
use ::std::str;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let fixtures_src = manifest_dir.join("fixtures");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let generated_dir = out_dir.join("generated");
    fs::create_dir_all(&generated_dir).expect("create generated dir");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", fixtures_src.display());

    let mut cases = list_fixture_cases(&fixtures_src);
    cases.sort_by(|a, b| a.module.cmp(&b.module));

    let mut generated = fs::File::create(out_dir.join("generated.rs")).expect("generated.rs");
    let mut case_tests = fs::File::create(out_dir.join("case_tests.rs")).expect("case_tests.rs");

    for case in &cases {
        println!("cargo:rerun-if-changed={}", case.dir.display());
        println!("cargo:rerun-if-changed={}", case.proto_path.display());
        println!("cargo:rerun-if-changed={}", case.test_path.display());

        let request = request_from_proto_file(&case.proto_path);
        let response = emit(&request)
            .unwrap_or_else(|e| panic!("codegen failed for fixture `{}`: {e}", case.dir.display()));
        assert_eq!(
            response.files.len(),
            1,
            "fixture `{}` must emit exactly one file",
            case.dir.display()
        );

        let rs_path = generated_dir.join(format!("{}.rs", case.module));
        fs::write(&rs_path, &response.files[0].content).expect("write generated rs");

        let gen_path = rs_path.to_str().expect("OUT_DIR path must be UTF-8");
        writeln!(
            generated,
            "#[path = r\"{gen_path}\"]\npub mod {};",
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

struct FixtureCase {
    /// Directory name → Rust module name (`crate::<module>`).
    module: String,
    dir: PathBuf,
    proto_path: PathBuf,
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

        let proto_path = find_single_proto(&dir);
        let test_path = dir.join("test.rs");
        if !test_path.is_file() {
            panic!("fixture `{}` is missing required `test.rs`", dir.display());
        }

        cases.push(FixtureCase {
            module,
            dir,
            proto_path,
            test_path,
        });
    }
    cases
}

fn find_single_proto(case_dir: &Path) -> PathBuf {
    let mut protos = Vec::new();
    for entry in fs::read_dir(case_dir).expect("read fixture case dir") {
        let entry = entry.expect("read fixture case entry");
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "proto") && path.is_file() {
            protos.push(path);
        }
    }
    match protos.len() {
        1 => protos.pop().unwrap(),
        0 => panic!(
            "fixture `{}` must contain exactly one `.proto` file",
            case_dir.display()
        ),
        n => panic!(
            "fixture `{}` must contain exactly one `.proto` file, found {n}",
            case_dir.display()
        ),
    }
}

fn request_from_proto_file(proto_path: &Path) -> CodegenRequest {
    let contents = fs::read_to_string(proto_path)
        .unwrap_or_else(|e| panic!("failed to read `{}`: {e}", proto_path.display()));
    let file_name = proto_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| panic!("non-UTF8 file name: {}", proto_path.display()))
        .to_owned();

    let package = parse_package(&contents).unwrap_or_default();
    let syntax = parse_language_mode(&contents).unwrap_or(Syntax::Proto2);
    let message_names = parse_empty_message_names(&contents);
    if message_names.is_empty() {
        panic!(
            "fixture `{}`: expected at least one `message Name {{ }}`",
            proto_path.display()
        );
    }

    let messages = message_names
        .into_iter()
        .map(|name| MessageDesc {
            name,
            fields: vec![],
            nested_messages: vec![],
            nested_enums: vec![],
            oneofs: vec![],
        })
        .collect();

    CodegenRequest {
        meta: CodegenMeta {
            file_to_generate: vec![file_name.clone()],
            parameter: None,
        },
        proto_files: vec![ProtoFile {
            name: file_name,
            package,
            syntax,
            features: FeatureSet::default(),
            dependency: vec![],
            messages,
            enums: vec![],
        }],
    }
}

/// Minimal `syntax = "...";` / `edition = "...";` extractor.
fn parse_language_mode(src: &str) -> Option<Syntax> {
    for raw_line in src.lines() {
        let line = strip_line_comment(raw_line).trim();
        if let Some(rest) = line.strip_prefix("edition") {
            let rest = rest.trim_start().strip_prefix('=')?.trim_start();
            let rest = rest.strip_suffix(';')?.trim();
            let quoted = rest.strip_prefix('"')?.strip_suffix('"')?;
            return Some(match quoted {
                "2023" => Syntax::Editions(Edition::Edition2023),
                "2024" => Syntax::Editions(Edition::Edition2024),
                _ => return None,
            });
        }
        if let Some(rest) = line.strip_prefix("syntax") {
            let rest = rest.trim_start().strip_prefix('=')?.trim_start();
            let rest = rest.strip_suffix(';')?.trim();
            let quoted = rest.strip_prefix('"')?.strip_suffix('"')?;
            return Some(match quoted {
                "proto3" => Syntax::Proto3,
                "proto2" => Syntax::Proto2,
                _ => return None,
            });
        }
    }
    None
}

/// Minimal `package foo.bar;` extractor (line-oriented; ignores `//` comments).
fn parse_package(src: &str) -> Option<String> {
    for raw_line in src.lines() {
        let line = strip_line_comment(raw_line).trim();
        let Some(rest) = line.strip_prefix("package") else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(pkg) = rest.strip_suffix(';') else {
            continue;
        };
        let pkg = pkg.trim();
        if !pkg.is_empty() {
            return Some(pkg.to_owned());
        }
    }
    None
}

/// Finds top-level `message Ident { }` with an empty body.
///
/// This is intentionally tiny — enough for the fake empty-message generator.
/// Non-empty message bodies are skipped here so `emit` can still reject richer
/// schemas once the IR carries fields.
fn parse_empty_message_names(src: &str) -> Vec<String> {
    let mut names = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if starts_with_word(bytes, i, b"message") {
            i += "message".len();
            i = skip_ws_and_comments(bytes, i);
            let Some((name, next)) = take_ident(bytes, i) else {
                break;
            };
            i = skip_ws_and_comments(bytes, next);
            if i >= bytes.len() || bytes[i] != b'{' {
                continue;
            }
            i += 1;
            let body_start = i;
            let Some(body_end) = find_matching_brace(bytes, body_start) else {
                break;
            };
            let body = &src[body_start..body_end];
            if body_is_empty(body) {
                names.push(name);
            }
            i = body_end + 1;
            continue;
        }
        i += 1;
    }
    names
}

fn body_is_empty(body: &str) -> bool {
    for raw_line in body.lines() {
        if !strip_line_comment(raw_line).trim().is_empty() {
            return false;
        }
    }
    true
}

fn strip_line_comment(line: &str) -> &str {
    match line.find("//") {
        Some(idx) => &line[..idx],
        None => line,
    }
}

fn starts_with_word(bytes: &[u8], i: usize, word: &[u8]) -> bool {
    if i + word.len() > bytes.len() {
        return false;
    }
    if &bytes[i..i + word.len()] != word {
        return false;
    }
    let before_ok = i == 0 || !is_ident_byte(bytes[i - 1]);
    let after_i = i + word.len();
    let after_ok = after_i >= bytes.len() || !is_ident_byte(bytes[after_i]);
    before_ok && after_ok
}

fn skip_ws_and_comments(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'\t' | b'\n' | b'\r' => i += 1,
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'/' => {
                i += 2;
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            _ => break,
        }
    }
    i
}

fn take_ident(bytes: &[u8], i: usize) -> Option<(String, usize)> {
    if i >= bytes.len() || !(bytes[i].is_ascii_alphabetic() || bytes[i] == b'_') {
        return None;
    }
    let mut j = i + 1;
    while j < bytes.len() && is_ident_byte(bytes[j]) {
        j += 1;
    }
    let name = str::from_utf8(&bytes[i..j]).ok()?.to_owned();
    Some((name, j))
}

fn find_matching_brace(bytes: &[u8], body_start: usize) -> Option<usize> {
    let mut depth = 1_usize;
    let mut i = body_start;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
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
