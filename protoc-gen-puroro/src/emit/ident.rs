//! Re-exports for emit helpers (keeps existing `emit::ident` import paths).

pub use crate::case::to_pascal_case;
pub use crate::rust_ident::{escape_ident, is_simple_ident};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_case_reexport() {
        assert_eq!(to_pascal_case("email_address"), "EmailAddress");
    }
}
