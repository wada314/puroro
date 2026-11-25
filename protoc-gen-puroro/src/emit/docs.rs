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
///
/// Proto comments often embed examples as Markdown indented code blocks or
/// bare fences. rustdoc treats both as Rust doctests, so this rewrite:
///
/// 1. Wraps indented (≥4 spaces / tab) runs in ` ```text ` fences.
/// 2. Retags bare / `rust` / `rs` fence openers as `text` (closers stay bare).
fn format_proto_doc(text: &str) -> String {
    let trimmed = text.trim_end_matches(['\n', '\r']);
    let lines: Vec<String> = trimmed
        .lines()
        .map(|line| line.trim_end().to_owned())
        .collect();
    let lines = wrap_indented_code_blocks(lines);
    neutralize_doctest_fences(lines).join("\n")
}

/// Wrap Markdown indented code runs in ` ```text ` so rustdoc will not execute them.
fn wrap_indented_code_blocks(lines: Vec<String>) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len());
    let mut i = 0;
    let mut in_fence = false;
    while i < lines.len() {
        let line = &lines[i];
        if split_fence_line(line).is_some() {
            in_fence = !in_fence;
            out.push(line.clone());
            i += 1;
            continue;
        }
        if !in_fence && is_indented_code_line(line) {
            let start = i;
            i += 1;
            while i < lines.len()
                && split_fence_line(&lines[i]).is_none()
                && (is_indented_code_line(&lines[i]) || lines[i].is_empty())
            {
                i += 1;
            }
            let mut end = i;
            while end > start && lines[end - 1].is_empty() {
                end -= 1;
            }
            out.push("```text".into());
            for block_line in &lines[start..end] {
                out.push(strip_markdown_indent(block_line));
            }
            out.push("```".into());
            while end < i {
                out.push(String::new());
                end += 1;
            }
            continue;
        }
        out.push(line.clone());
        i += 1;
    }
    out
}

fn is_indented_code_line(line: &str) -> bool {
    line.starts_with("    ") || line.starts_with('\t')
}

fn strip_markdown_indent(line: &str) -> String {
    if let Some(rest) = line.strip_prefix('\t') {
        return rest.to_owned();
    }
    if let Some(rest) = line.strip_prefix("    ") {
        return rest.to_owned();
    }
    line.to_owned()
}

/// Retag fence openers that rustdoc would run as Rust doctests.
fn neutralize_doctest_fences(lines: Vec<String>) -> Vec<String> {
    let mut in_fence = false;
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
        match split_fence_line(&line) {
            Some((indent, info)) if !in_fence => {
                in_fence = true;
                let info = info.trim();
                if info.is_empty()
                    || info.eq_ignore_ascii_case("rust")
                    || info.eq_ignore_ascii_case("rs")
                {
                    out.push(format!("{indent}```text"));
                } else {
                    out.push(line);
                }
            }
            Some(_) => {
                // Closing fence — keep the original opener/closer spelling.
                in_fence = false;
                out.push(line);
            }
            None => out.push(line),
        }
    }
    out
}

/// If `line` is a markdown fence opener/closer, return `(indent, info_string)`.
///
/// Supports `` ``` `` and `~~~` with optional leading whitespace.
fn split_fence_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim_start();
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];
    let fence_char = trimmed.chars().next()?;
    if fence_char != '`' && fence_char != '~' {
        return None;
    }
    let fence_len = trimmed.chars().take_while(|c| *c == fence_char).count();
    if fence_len < 3 {
        return None;
    }
    // Bytes equal chars here (` and ~ are ASCII).
    Some((indent, trimmed[fence_len..].trim_start()))
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

    #[test]
    fn bare_fence_becomes_text_language() {
        let text = format_proto_doc(" Example:\n```\nFoo foo = 1;\n```\n");
        assert!(
            text.contains("```text\nFoo foo = 1;\n```"),
            "bare fence must not stay a rust doctest: {text}"
        );
    }

    #[test]
    fn rust_fence_becomes_text_language() {
        let text = format_proto_doc("```rust\nlet x = 1;\n```");
        assert!(text.starts_with("```text\n"), "{text}");
        assert!(text.ends_with("\n```"), "{text}");
    }

    #[test]
    fn proto_fence_is_left_alone() {
        let text = format_proto_doc("```proto\nmessage M {}\n```");
        assert!(text.starts_with("```proto\n"), "{text}");
    }

    #[test]
    fn indented_code_becomes_text_fence() {
        let text =
            format_proto_doc(" Example:\n\n    Foo foo = ...;\n    Any any;\n\n More prose.\n");
        assert!(
            text.contains("```text\nFoo foo = ...;\nAny any;\n```"),
            "indented Markdown code must be fenced as text: {text}"
        );
        assert!(text.contains(" More prose."), "{text}");
    }
}
