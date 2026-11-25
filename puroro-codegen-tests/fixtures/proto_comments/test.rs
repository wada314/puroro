//! Proto comments appear as rustdoc on the generated module.

use std::fs;
use std::path::PathBuf;

#[test]
fn generated_source_includes_proto_comments() {
    let dir = PathBuf::from(env!("OUT_DIR")).join("generated/proto_comments");
    let entries = fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display()));
    let mut rs_files = Vec::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            rs_files.push(path);
        }
    }
    assert_eq!(rs_files.len(), 1, "expected one generated .rs under {dir:?}");
    let content = fs::read_to_string(&rs_files[0]).expect("read generated rs");
    assert!(
        content.contains("A documented work item."),
        "message comment missing: {content}"
    );
    assert!(
        content.contains("Human-readable title."),
        "field comment missing: {content}"
    );
    assert!(
        content.contains("Lifecycle of a tracked item."),
        "enum comment missing: {content}"
    );
    assert!(
        content.contains("Not started yet."),
        "enum value comment missing: {content}"
    );
}
