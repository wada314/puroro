//! Rust identifier helpers for generated code.

use ::proc_macro2::{Ident, Span};

pub use crate::case::to_pascal_case;

/// Escape `name` as a Rust identifier, using a raw ident (`r#…`) for keywords.
pub fn escape_ident(name: &str) -> Ident {
    if is_rust_keyword(name) {
        Ident::new_raw(name, Span::call_site())
    } else {
        Ident::new(name, Span::call_site())
    }
}

/// True if `name` is a simple ASCII identifier: starts with `A–Z` / `a–z` / `_`,
/// and the rest is only those plus `0–9`.
///
/// This is a character-class check only. Keywords such as `type` still pass;
/// [`escape_ident`] turns those into raw idents. Empty strings, leading digits,
/// hyphens, dots, and non-ASCII are rejected (`""`, `"2foo"`, `"foo-bar"`,
/// `"pkg.name"`, `"café"`).
pub fn is_simple_ident(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(c) = chars.next() else {
        return false;
    };
    (c.is_ascii_alphabetic() || c == '_') && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
            | "gen"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_use_raw_idents() {
        assert_eq!(escape_ident("type").to_string(), "r#type");
        assert_eq!(escape_ident("match").to_string(), "r#match");
        // Proto field names that are not Rust keywords stay plain.
        assert_eq!(escape_ident("reserved").to_string(), "reserved");
        assert_eq!(escape_ident("repeated").to_string(), "repeated");
        assert_eq!(escape_ident("street").to_string(), "street");
    }

    #[test]
    fn pascal_case_reexport() {
        // Smoke check that emit code can keep importing `to_pascal_case` here.
        assert_eq!(to_pascal_case("email_address"), "EmailAddress");
    }
}
