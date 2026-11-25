//! Proto comments → rustdoc attributes.

use crate::descriptor::Comments;
use ::syn::{Attribute, parse_quote};

/// `#[doc = "…"]` from proto comments, or `None` when absent.
pub(super) fn doc_attr(comments: &Comments) -> Option<Attribute> {
    comments.doc_text().map(|text| {
        let cleaned = format_proto_doc(text);
        parse_quote! { #[doc = #cleaned] }
    })
}

/// Normalize SourceCodeInfo comment text for a single `#[doc = …]` string.
///
/// Protoc already strips `//` / `/* */` markers. Trailing newlines are common;
/// keep interior blank lines so paragraph breaks survive in rustdoc. Keep a
/// leading space on the first line when present (`// text` → `" text"`).
///
/// Proto comments are not rewritten for doctest safety: generated crates set
/// `[lib] doctest = false` so rustdoc examples in comments are never compiled.
fn format_proto_doc(text: &str) -> String {
    text.trim_end_matches(['\n', '\r'])
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_comments_emit_nothing() {
        assert!(doc_attr(&Comments::default()).is_none());
    }

    #[test]
    fn leading_becomes_doc_attr() {
        let comments = Comments {
            leading: Some("Hello task.\n".into()),
            trailing: None,
        };
        let attr = doc_attr(&comments).expect("doc attr");
        let s = ::quote::quote! { #attr }.to_string();
        assert!(s.contains("doc"), "{s}");
        assert!(s.contains("Hello task."), "{s}");
    }

    #[test]
    fn trailing_used_when_no_leading() {
        let comments = Comments {
            leading: None,
            trailing: Some(" inline note ".into()),
        };
        assert_eq!(comments.doc_text(), Some(" inline note "));
    }

    #[test]
    fn format_keeps_blank_line_paragraphs() {
        let text = format_proto_doc(" First.\n\nSecond.\n");
        assert_eq!(text, " First.\n\nSecond.");
    }

    #[test]
    fn format_does_not_retag_fences() {
        let text = format_proto_doc(" Example:\n```\nFoo foo = 1;\n```\n");
        assert!(
            text.contains("```\nFoo foo = 1;\n```"),
            "proto fences stay as written: {text}"
        );
        assert!(!text.contains("```text"), "{text}");
    }
}
