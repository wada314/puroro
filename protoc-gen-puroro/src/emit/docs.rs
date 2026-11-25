//! Proto comments → rustdoc attributes.

use crate::descriptor::Comments;
use ::proc_macro2::TokenStream;
use ::quote::quote;

/// `#[doc = "…"]` from proto comments, or empty tokens when absent.
pub(super) fn doc_attr(comments: &Comments) -> TokenStream {
    match comments.doc_text() {
        Some(text) => {
            let cleaned = format_proto_doc(text);
            quote! { #[doc = #cleaned] }
        }
        None => quote! {},
    }
}

/// Normalize SourceCodeInfo comment text for a single `#[doc = …]` string.
///
/// Protoc already strips `//` / `/* */` markers. Trailing newlines are common;
/// keep interior blank lines so paragraph breaks survive in rustdoc. Keep a
/// leading space on the first line when present (`// text` → `" text"`).
fn format_proto_doc(text: &str) -> String {
    let trimmed = text.trim_end_matches(['\n', '\r']);
    trimmed
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
        let tokens = doc_attr(&Comments::default());
        assert!(tokens.is_empty());
    }

    #[test]
    fn leading_becomes_doc_attr() {
        let comments = Comments {
            leading: Some("Hello task.\n".into()),
            trailing: None,
        };
        let tokens = doc_attr(&comments);
        let s = tokens.to_string();
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
}
