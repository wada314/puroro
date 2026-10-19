//! Shared ASCII case-conversion helpers for protobuf → Rust naming.
//!
//! Inputs are **not** assumed to already be in a particular style. Each public
//! converter first splits the string into words (underscores and camel / acronym
//! boundaries), then joins those words in the target style.
//!
//! These helpers are ASCII-oriented (protobuf identifiers), not a full Unicode
//! inflection library.

use ::itertools::Itertools as _;
use ::std::iter;

/// `email_address` / `EmailAddress` / `EMAIL_ADDRESS` → `EmailAddress`.
pub fn to_pascal_case(name: &str) -> String {
    words(name)
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first
                    .to_uppercase()
                    .chain(chars.flat_map(|c| c.to_lowercase()))
                    .collect(),
                None => String::new(),
            }
        })
        .collect()
}

/// `Task` / `FooBar` / `FOO_BAR` → `task` / `foo_bar`.
pub fn to_snake_case(name: &str) -> String {
    words(name).map(str::to_ascii_lowercase).join("_")
}

/// `Status` / `FooBar` / `max_retries` / `XMLParser` → `STATUS` / `FOO_BAR` /
/// `MAX_RETRIES` / `XML_PARSER` (UPPER_SNAKE).
pub fn to_upper_snake(name: &str) -> String {
    words(name).map(str::to_ascii_uppercase).join("_")
}

/// Iterate word slices of `name` without assuming snake vs camel vs UPPER_SNAKE.
///
/// Yields borrowed `&str` segments of `name` (no per-word allocation).
pub fn words(name: &str) -> impl Iterator<Item = &str> + '_ {
    let bytes = name.as_bytes();
    let mut i = 0usize;
    iter::from_fn(move || {
        while i < bytes.len() && bytes[i] == b'_' {
            i += 1;
        }
        if i >= bytes.len() {
            return None;
        }

        let start = i;
        i += 1;
        while i < bytes.len() {
            let c = bytes[i];
            if c == b'_' {
                break;
            }
            if c.is_ascii_uppercase() {
                let prev_lower = bytes[i - 1].is_ascii_lowercase();
                let next_lower = bytes.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
                // `fooBar` or `XMLParser` (split before `P`)
                if prev_lower || next_lower {
                    break;
                }
            }
            i += 1;
        }
        Some(&name[start..i])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_mixed_styles() {
        assert_eq!(
            words("email_address").collect::<Vec<_>>(),
            ["email", "address"]
        );
        assert_eq!(
            words("EmailAddress").collect::<Vec<_>>(),
            ["Email", "Address"]
        );
        assert_eq!(
            words("EMAIL_ADDRESS").collect::<Vec<_>>(),
            ["EMAIL", "ADDRESS"]
        );
        assert_eq!(words("XMLParser").collect::<Vec<_>>(), ["XML", "Parser"]);
        assert_eq!(words("maxRetries").collect::<Vec<_>>(), ["max", "Retries"]);
    }

    #[test]
    fn pascal_case_from_any_style() {
        assert_eq!(to_pascal_case("email_address"), "EmailAddress");
        assert_eq!(to_pascal_case("urgent"), "Urgent");
        assert_eq!(to_pascal_case("EmailAddress"), "EmailAddress");
        assert_eq!(to_pascal_case("EMAIL_ADDRESS"), "EmailAddress");
        assert_eq!(to_pascal_case("maxRetries"), "MaxRetries");
    }

    #[test]
    fn snake_case_from_any_style() {
        assert_eq!(to_snake_case("Task"), "task");
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
        assert_eq!(to_snake_case("FOO_BAR"), "foo_bar");
        assert_eq!(to_snake_case("XMLParser"), "xml_parser");
        assert_eq!(to_snake_case("email_address"), "email_address");
    }

    #[test]
    fn upper_snake_from_any_style() {
        assert_eq!(to_upper_snake("Status"), "STATUS");
        assert_eq!(to_upper_snake("FooBar"), "FOO_BAR");
        assert_eq!(to_upper_snake("XMLParser"), "XML_PARSER");
        assert_eq!(to_upper_snake("max_retries"), "MAX_RETRIES");
        assert_eq!(to_upper_snake("title"), "TITLE");
        assert_eq!(to_upper_snake("MAX_RETRIES"), "MAX_RETRIES");
    }
}
