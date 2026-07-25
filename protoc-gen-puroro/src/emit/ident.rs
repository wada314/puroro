//! Rust identifier helpers for generated code.

use ::proc_macro2::{Ident, Span};

/// Protobuf name → Rust `Ident`, using `r#…` for keywords (`type`, `self`, …).
pub fn rust_ident(name: &str) -> Ident {
    if is_rust_keyword(name) {
        Ident::new_raw(name, Span::call_site())
    } else {
        Ident::new(name, Span::call_site())
    }
}

pub fn is_simple_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

/// `email_address` / `EmailAddress` → `EmailAddress`.
pub fn to_pascal_case(name: &str) -> String {
    let mut out = String::new();
    let mut capitalize = true;
    for c in name.chars() {
        if c == '_' {
            capitalize = true;
            continue;
        }
        if capitalize {
            for upper in c.to_uppercase() {
                out.push(upper);
            }
            capitalize = false;
        } else {
            out.push(c);
        }
    }
    out
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
        assert_eq!(rust_ident("type").to_string(), "r#type");
        assert_eq!(rust_ident("match").to_string(), "r#match");
        // Proto field names that are not Rust keywords stay plain.
        assert_eq!(rust_ident("reserved").to_string(), "reserved");
        assert_eq!(rust_ident("repeated").to_string(), "repeated");
        assert_eq!(rust_ident("street").to_string(), "street");
    }

    #[test]
    fn pascal_case_from_snake_and_camel() {
        assert_eq!(to_pascal_case("email_address"), "EmailAddress");
        assert_eq!(to_pascal_case("urgent"), "Urgent");
        assert_eq!(to_pascal_case("EmailAddress"), "EmailAddress");
    }
}
