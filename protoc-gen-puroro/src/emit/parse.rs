//! Parse generated `quote!` output into typed `syn` syntax nodes.

use crate::error::{Error, Result};
use ::proc_macro2::TokenStream;
use ::syn::parse::{ParseStream, Parser};
use ::syn::{File, Item};

pub(super) fn parse_file(tokens: TokenStream) -> Result<File> {
    ::syn::parse2(tokens)
        .map_err(|e| Error::internal(format!("generated tokens are not a valid Rust file: {e}")))
}

pub(super) fn parse_items(tokens: TokenStream) -> Result<Vec<Item>> {
    (|input: ParseStream| {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(items)
    })
    .parse2(tokens)
    .map_err(|e| Error::internal(format!("generated tokens are not valid items: {e}")))
}
