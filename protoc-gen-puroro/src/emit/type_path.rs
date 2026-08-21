//! Map protobuf FQNs to generated Rust paths via `self::_root::…`.

use super::ident::{escape_ident, is_simple_ident};
use crate::descriptor::ProtoFqn;
use crate::error::{Error, Result};
use crate::module_tree::type_name_to_module_ident;
use crate::resolved::{Enum, Message};
use ::proc_macro2::{Ident, Span};
use ::syn::parse_quote;
use ::syn::{Path, PathSegment};

/// File-level enum `.example.v1.Status` → `self::_root::example::v1::Status`.
///
/// Prefer [`fqn_to_enum_root_path`] when an [`Enum`] handle is available (nested
/// enums need the parent message module chain).
pub fn fqn_to_root_path(fqn: &ProtoFqn) -> Result<Path> {
    let (package_segments, type_name) = split_fqn_file_level(fqn)?;
    let mut path = root_path();
    for seg in package_segments {
        path.segments
            .push(PathSegment::from(Ident::new(seg, Span::call_site())));
    }
    path.segments
        .push(PathSegment::from(escape_ident(type_name)));
    Ok(path)
}

/// Enum path: file-level or nested under parent message modules.
///
/// - `.demo.Status` → `self::_root::demo::Status`
/// - `.demo.Task.Kind` → `self::_root::demo::task::Kind`
pub fn fqn_to_enum_root_path<'a>(enumeration: &'a Enum<'a>) -> Result<Path> {
    match enumeration.parent() {
        None => fqn_to_root_path(enumeration.fqn()),
        Some(parent) => {
            let mut path = message_module_path(parent)?;
            path.segments
                .push(PathSegment::from(escape_ident(enumeration.name())));
            Ok(path)
        }
    }
}

/// Message type path: the struct lives in the parent module.
///
/// - `.demo.Address` → `self::_root::demo::Address`
/// - `.demo.Outer.Inner` → `self::_root::demo::outer::Inner`
pub fn fqn_to_message_root_path<'a>(message: &'a Message<'a>) -> Result<Path> {
    let mut path = parent_module_path(message)?;
    path.segments
        .push(PathSegment::from(escape_ident(message.name())));
    Ok(path)
}

fn root_path() -> Path {
    parse_quote!(self::_root)
}

/// Module that contains this message's struct (`package` or parent companion).
fn parent_module_path<'a>(message: &'a Message<'a>) -> Result<Path> {
    match message.parent() {
        Some(parent) => message_module_path(parent),
        None => {
            let (package, _) = package_and_message_chain(message)?;
            module_path_from_package(package, message.fqn().as_str())
        }
    }
}

/// Companion module: `self::_root::pkg…::outer::inner` (includes this message).
fn message_module_path<'a>(message: &'a Message<'a>) -> Result<Path> {
    let mut path = parent_module_path(message)?;
    let name = message.name();
    if !is_simple_ident(name) {
        return Err(Error::Codegen(format!(
            "message name `{name}` in `{}` is not a simple Rust identifier",
            message.fqn()
        )));
    }
    path.segments
        .push(PathSegment::from(type_name_to_module_ident(name)));
    Ok(path)
}

fn module_path_from_package(package: Vec<&str>, fqn: &str) -> Result<Path> {
    let mut path = root_path();
    for seg in package {
        if !is_simple_ident(seg) {
            return Err(Error::Codegen(format!(
                "package segment `{seg}` in `{fqn}` is not a simple Rust identifier"
            )));
        }
        path.segments
            .push(PathSegment::from(Ident::new(seg, Span::call_site())));
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
    use crate::resolved::{Arena, FileSet, resolve};
    use ::quote::ToTokens;

    fn message<'a>(set: &FileSet<'a>, name: &str) -> &'a Message<'a> {
        fn walk<'a>(m: &'a Message<'a>, name: &str) -> Option<&'a Message<'a>> {
            if m.name() == name {
                return Some(m);
            }
            m.nested_messages().find_map(|n| walk(n, name))
        }
        set.files()
            .flat_map(|f| f.messages())
            .find_map(|m| walk(m, name))
            .unwrap_or_else(|| panic!("missing message {name}"))
    }

    fn enumeration<'a>(set: &FileSet<'a>, name: &str) -> &'a Enum<'a> {
        fn walk_msg<'a>(m: &'a Message<'a>, name: &str) -> Option<&'a Enum<'a>> {
            m.nested_enums()
                .find(|e| e.name() == name)
                .or_else(|| m.nested_messages().find_map(|n| walk_msg(n, name)))
        }
        set.files()
            .flat_map(|f| f.enums())
            .find(|e| e.name() == name)
            .or_else(|| {
                set.files()
                    .flat_map(|f| f.messages())
                    .find_map(|m| walk_msg(m, name))
            })
            .unwrap_or_else(|| panic!("missing enum {name}"))
    }

    #[test]
    fn maps_packaged_and_top_level_enums() {
        let packaged = fqn_to_root_path(&ProtoFqn::parse(".demo.Status")).unwrap();
        assert_eq!(
            packaged.to_token_stream().to_string(),
            "self :: _root :: demo :: Status"
        );
        let top = fqn_to_root_path(&ProtoFqn::parse(".Status")).unwrap();
        assert_eq!(top.to_token_stream().to_string(), "self :: _root :: Status");
    }

    #[test]
    fn maps_file_level_message_in_package_module() {
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
                map_entry: false,
            }],
            enums: vec![],
        }];
        let set = resolve(&arena, &files).unwrap();
        let msg = message(&set, "Address");
        let path = fqn_to_message_root_path(msg).unwrap();
        assert_eq!(
            path.to_token_stream().to_string(),
            "self :: _root :: demo :: Address"
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
                    map_entry: false,
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
                map_entry: false,
            }],
            enums: vec![],
        }];
        let set = resolve(&arena, &files).unwrap();
        let inner = message(&set, "Inner");
        assert_eq!(
            fqn_to_message_root_path(inner)
                .unwrap()
                .to_token_stream()
                .to_string(),
            "self :: _root :: demo :: outer :: Inner"
        );
        let kind = enumeration(&set, "Kind");
        assert_eq!(
            fqn_to_enum_root_path(kind)
                .unwrap()
                .to_token_stream()
                .to_string(),
            "self :: _root :: demo :: outer :: Kind"
        );
    }
}
