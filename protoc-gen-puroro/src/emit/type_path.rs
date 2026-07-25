//! Map protobuf FQNs to generated Rust paths via `self::_root::…`.

use crate::descriptor::ProtoFqn;
use crate::error::{Error, Result};
use crate::module_tree::type_name_to_module_ident;
use crate::resolved::Message;
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;

/// `.example.v1.Status` → `self::_root::example::v1::Status`.
///
/// File-level enums sit directly under the package module (no snake_case
/// submodule). Nested types are out of scope for this helper until nested
/// enum emission lands.
pub fn fqn_to_root_path(fqn: &ProtoFqn) -> Result<TokenStream> {
    let (package_segments, type_name) = split_fqn(fqn)?;
    let type_ident = Ident::new(type_name, Span::call_site());
    let mut path = quote! { self::_root };
    for seg in package_segments {
        let ident = Ident::new(seg, Span::call_site());
        path = quote! { #path::#ident };
    }
    Ok(quote! { #path::#type_ident })
}

/// File-level message `.example.Address` → `self::_root::example::address::Address`.
///
/// Nested message declarations are rejected (no enclosing-type modules yet).
pub fn fqn_to_message_root_path(message: &Message<'_>) -> Result<TokenStream> {
    if message.parent().is_some() {
        return Err(Error::Codegen(format!(
            "nested message `{}` is not supported in type paths yet",
            message.name()
        )));
    }
    let (package_segments, type_name) = split_fqn(message.fqn())?;
    if type_name != message.name() {
        return Err(Error::Codegen(format!(
            "internal error: message name `{}` does not match FQN `{}`",
            message.name(),
            message.fqn()
        )));
    }
    let type_ident = Ident::new(type_name, Span::call_site());
    let mod_ident = type_name_to_module_ident(type_name);
    let mut path = quote! { self::_root };
    for seg in package_segments {
        let ident = Ident::new(seg, Span::call_site());
        path = quote! { #path::#ident };
    }
    Ok(quote! { #path::#mod_ident::#type_ident })
}

fn split_fqn(fqn: &ProtoFqn) -> Result<(Vec<&str>, &str)> {
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
    Ok((segments, type_name))
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
    use crate::descriptor::{FeatureSet, MessageDesc, ProtoFile, Syntax};
    use crate::resolved::{Arena, resolve};

    #[test]
    fn maps_packaged_and_top_level_enums() {
        let packaged = fqn_to_root_path(&ProtoFqn::parse(".demo.Status")).unwrap();
        assert_eq!(packaged.to_string(), "self :: _root :: demo :: Status");
        let top = fqn_to_root_path(&ProtoFqn::parse(".Status")).unwrap();
        assert_eq!(top.to_string(), "self :: _root :: Status");
    }

    #[test]
    fn maps_file_level_message_through_snake_module() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "t.proto".into(),
            package: "demo".into(),
            syntax: Syntax::Proto3,
            features: FeatureSet::default(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "Address".into(),
                fields: vec![],
                nested_messages: vec![],
                nested_enums: vec![],
                oneofs: vec![],
            }],
            enums: vec![],
        }];
        let set = resolve(&arena, &files).unwrap();
        let msg = set.lookup(".demo.Address").unwrap().as_message().unwrap();
        let path = fqn_to_message_root_path(msg).unwrap();
        assert_eq!(
            path.to_string(),
            "self :: _root :: demo :: address :: Address"
        );
    }
}
