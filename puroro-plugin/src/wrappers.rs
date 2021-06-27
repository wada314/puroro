use crate::utils;
use ::itertools::Itertools;
use ::std::iter;

#[derive(Debug, Clone)]
pub struct Message {
    pub rust_ident: String,
    pub proto_package: Vec<String>,
    pub proto_outer_messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub rust_ident: String,
    pub proto_package: Vec<String>,
    pub proto_outer_messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub rust_ident: String,
    pub proto_type: FieldType,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Double,
    Float,
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Bool,
    Group,
    String,
    Bytes,
    Enum(Enum),
    Message(Message),
}

impl Enum {
    pub fn rust_absolute_path(&self) -> String {
        format!(
            "{path}::{ident}",
            path = make_module_path(&self.proto_package, &self.proto_outer_messages),
            ident = self.rust_ident
        )
    }
}

#[test]
fn test_make_module_path() {
    let package = "google.protobuf.compiler"
        .split('.')
        .map(|p| p.to_string())
        .collect::<Vec<_>>();
    let outer_messages = vec!["CodeGeneratorResponse".to_string()];
    let empty: Vec<String> = Vec::new();
    assert_eq!(
        "self::puroro_root::google::protobuf::compiler",
        make_module_path(&package, &empty)
    );
    assert_eq!(
        "self::puroro_root::puroro_nested::code_generator_response",
        make_module_path(&empty, &outer_messages)
    );
    assert_eq!(
        "self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response",
        make_module_path(&package, &outer_messages)
    );
}

fn make_module_path(package: &Vec<String>, outer_messages: &Vec<String>) -> String {
    let package_iter = package
        .into_iter()
        .map(|s| utils::get_keyword_safe_ident(s));
    let outer_messages_iter = outer_messages
        .into_iter()
        .map(|s| utils::get_keyword_safe_ident(&utils::to_lower_snake_case(s)));
    let maybe_puroro_nested = if outer_messages.is_empty() {
        None
    } else {
        Some("puroro_nested".to_string())
    }
    .into_iter();
    let mut modules_iter = iter::once("self".to_string())
        .chain(iter::once("puroro_root".to_string()))
        .chain(package_iter)
        .chain(maybe_puroro_nested)
        .chain(outer_messages_iter);
    modules_iter.join("::")
}
