use crate::utils;
use ::itertools::Itertools;
use ::std::borrow::Borrow;
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
            path = make_module_path(
                self.proto_package.iter().map(|s| s.borrow()),
                self.proto_outer_messages.iter().map(|s| s.borrow())
            ),
            ident = self.rust_ident
        )
    }
}

#[test]
fn test_make_module_path() {
    let package = "google.protobuf.compiler".split('.');
    let outer_messages = iter::once("CodeGeneratorResponse");
    let empty = iter::empty();
    assert_eq!(
        "self::puroro_root::google::protobuf::compiler",
        make_module_path(package.clone(), empty.clone())
    );
    assert_eq!(
        "self::puroro_root::puroro_nested::code_generator_response",
        make_module_path(empty.clone(), outer_messages.clone())
    );
    assert_eq!(
        "self::puroro_root::google::protobuf::compiler::puroro_nested::code_generator_response",
        make_module_path(package.clone(), outer_messages.clone())
    );
}

fn make_module_path<'a, I, J>(package: I, outer_messages: J) -> String
where
    I: Iterator<Item = &'a str>,
    J: Iterator<Item = &'a str> + Clone,
{
    let package = package.map(|s| utils::get_keyword_safe_ident(s));
    let maybe_puroro_nested = if outer_messages.clone().count() == 0 {
        None
    } else {
        Some("puroro_nested".to_string())
    }
    .into_iter();
    let outer_messages =
        outer_messages.map(|s| utils::get_keyword_safe_ident(&utils::to_lower_snake_case(s)));
    let mut modules_iter = iter::once("self".to_string())
        .chain(iter::once("puroro_root".to_string()))
        .chain(package)
        .chain(maybe_puroro_nested)
        .chain(outer_messages);
    modules_iter.join("::")
}
