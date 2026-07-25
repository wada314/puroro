//! Map protobuf FQNs to generated Rust paths via `self::_root::…`.

use super::ident::{is_simple_ident, rust_ident};
use crate::descriptor::ProtoFqn;
use crate::error::{Error, Result};
use crate::module_tree::type_name_to_module_ident;
use crate::resolved::{Enum, Message};
use ::proc_macro2::{Ident, Span, TokenStream};
use ::quote::quote;

/// File-level enum `.example.v1.Status` → `self::_root::example::v1::Status`.
///
/// Prefer [`fqn_to_enum_root_path`] when an [`Enum`] handle is available (nested
/// enums need the parent message module chain).
pub fn fqn_to_root_path(fqn: &ProtoFqn) -> Result<TokenStream> {
    let (package_segments, type_name) = split_fqn_file_level(fqn)?;
    let type_ident = rust_ident(type_name);
    let mut path = quote! { self::_root };
    for seg in package_segments {
        let ident = Ident::new(seg, Span::call_site());
        path = quote! { #path::#ident };
    }
    Ok(quote! { #path::#type_ident })
}

/// Enum path: file-level or nested under parent message modules.
///
/// - `.demo.Status` → `self::_root::demo::Status`
/// - `.demo.Task.Kind` → `self::_root::demo::task::Kind`
pub fn fqn_to_enum_root_path<'a>(enumeration: &'a Enum<'a>) -> Result<TokenStream> {
    let type_ident = rust_ident(enumeration.name());
    match enumeration.parent() {
        None => fqn_to_root_path(enumeration.fqn()),
        Some(parent) => {
            let parent_mod = message_module_path(parent)?;
            Ok(quote! { #parent_mod::#type_ident })
        }
    }
}

/// Message path including snake_case modules for every enclosing message.
///
/// - `.demo.Address` → `self::_root::demo::address::Address`
/// - `.demo.Outer.Inner` → `self::_root::demo::outer::inner::Inner`
pub fn fqn_to_message_root_path<'a>(message: &'a Message<'a>) -> Result<TokenStream> {
    let type_ident = rust_ident(message.name());
    let module_path = message_module_path(message)?;
    Ok(quote! { #module_path::#type_ident })
}

/// `self::_root::pkg…::outer::inner` (message modules only; no trailing type).
fn message_module_path<'a>(message: &'a Message<'a>) -> Result<TokenStream> {
    let (package, type_names) = package_and_message_chain(message)?;
    let mut path = quote! { self::_root };
    for seg in package {
        if !is_simple_ident(seg) {
            return Err(Error::Codegen(format!(
                "package segment `{seg}` in `{}` is not a simple Rust identifier",
                message.fqn()
            )));
        }
        let ident = Ident::new(seg, Span::call_site());
        path = quote! { #path::#ident };
    }
    for name in type_names {
        if !is_simple_ident(name) {
            return Err(Error::Codegen(format!(
                "message name `{name}` in `{}` is not a simple Rust identifier",
                message.fqn()
            )));
        }
        let mod_ident = type_name_to_module_ident(name);
        path = quote! { #path::#mod_ident };
    }
    Ok(path)
}

/// Split FQN into package segments + outer-to-inner message simple names.
fn package_and_message_chain<'a>(message: &'a Message<'a>) -> Result<(Vec<&'a str>, Vec<&'a str>)> {
    let mut type_names = Vec::new();
    let mut current = Some(message);
    while let Some(m) = current {
        type_names.push(m.name());
        current = m.parent();
    }
    type_names.reverse();

    let raw = message
        .fqn()
        .as_str()
        .strip_prefix('.')
        .unwrap_or_else(|| message.fqn().as_str());
    if raw.is_empty() {
        return Err(Error::Codegen(format!(
            "cannot map package-root FQN `{}` to a Rust type path",
            message.fqn()
        )));
    }
    let segments: Vec<&str> = raw.split('.').collect();
    if segments.len() < type_names.len() {
        return Err(Error::Codegen(format!(
            "FQN `{}` is shorter than message nesting chain",
            message.fqn()
        )));
    }
    let type_start = segments.len() - type_names.len();
    for (i, expected) in type_names.iter().enumerate() {
        if segments[type_start + i] != *expected {
            return Err(Error::Codegen(format!(
                "FQN `{}` does not end with message chain {:?}",
                message.fqn(),
                type_names
            )));
        }
    }
    Ok((segments[..type_start].to_vec(), type_names))
}

fn split_fqn_file_level(fqn: &ProtoFqn) -> Result<(Vec<&str>, &str)> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::{EnumDesc, EnumValueDesc, FeatureSet, MessageDesc, ProtoFile, Syntax};
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

    #[test]
    fn maps_nested_message_and_nested_enum() {
        let arena = Arena::new();
        let files = [ProtoFile {
            name: "t.proto".into(),
            package: "demo".into(),
            syntax: Syntax::Proto3,
            features: FeatureSet::default(),
            dependency: vec![],
            messages: vec![MessageDesc {
                name: "Outer".into(),
                fields: vec![],
                nested_messages: vec![MessageDesc {
                    name: "Inner".into(),
                    fields: vec![],
                    nested_messages: vec![],
                    nested_enums: vec![],
                    oneofs: vec![],
                }],
                nested_enums: vec![EnumDesc {
                    name: "Kind".into(),
                    values: vec![EnumValueDesc {
                        name: "KIND_UNSPECIFIED".into(),
                        number: 0,
                    }],
                    features: FeatureSet::default(),
                }],
                oneofs: vec![],
            }],
            enums: vec![],
        }];
        let set = resolve(&arena, &files).unwrap();
        let inner = set
            .lookup(".demo.Outer.Inner")
            .unwrap()
            .as_message()
            .unwrap();
        assert_eq!(
            fqn_to_message_root_path(inner).unwrap().to_string(),
            "self :: _root :: demo :: outer :: inner :: Inner"
        );
        let kind = set.lookup(".demo.Outer.Kind").unwrap().as_enum().unwrap();
        assert_eq!(
            fqn_to_enum_root_path(kind).unwrap().to_string(),
            "self :: _root :: demo :: outer :: Kind"
        );
    }
}
