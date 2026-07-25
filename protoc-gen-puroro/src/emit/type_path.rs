//! Map protobuf FQNs to generated Rust paths via `self::_root::…`.

use crate::descriptor::ProtoFqn;
use crate::error::{Error, Result};
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;

/// `.example.v1.Status` → `self::_root::example::v1::Status`.
///
/// File-level enums sit directly under the package module (no snake_case
/// submodule). Nested types are out of scope for this helper until nested
/// enum emission lands.
pub fn fqn_to_root_path(fqn: &ProtoFqn) -> Result<TokenStream> {
    let raw = fqn.as_str().strip_prefix('.').unwrap_or(fqn.as_str());
    if raw.is_empty() {
        return Err(Error::Codegen(format!(
            "cannot map package-root FQN `{fqn}` to a Rust type path"
        )));
    }
    let mut segments: Vec<&str> = raw.split('.').collect();
    let type_name = segments.pop().expect("non-empty raw has a last segment");
    if !is_simple_ident(type_name) {
        return Err(Error::Codegen(format!(
            "type name `{type_name}` in `{fqn}` is not a simple Rust identifier"
        )));
    }
    for seg in &segments {
        if !is_simple_ident(seg) {
            return Err(Error::Codegen(format!(
                "package segment `{seg}` in `{fqn}` is not a simple Rust identifier"
            )));
        }
    }

    let type_ident = Ident::new(type_name, Span::call_site());
    let mut path = quote! { self::_root };
    for seg in segments {
        let ident = Ident::new(seg, Span::call_site());
        path = quote! { #path::#ident };
    }
    Ok(quote! { #path::#type_ident })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_packaged_and_top_level() {
        let packaged = fqn_to_root_path(&ProtoFqn::parse(".demo.Status")).unwrap();
        assert_eq!(packaged.to_string(), "self :: _root :: demo :: Status");
        let top = fqn_to_root_path(&ProtoFqn::parse(".Status")).unwrap();
        assert_eq!(top.to_string(), "self :: _root :: Status");
    }
}
