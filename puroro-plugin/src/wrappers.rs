#![allow(unused)]

use crate::protos::enums::google::protobuf::field_descriptor_proto::Type as FieldTypeProto;
use crate::protos::simple::google::protobuf;
use crate::utils;
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::once_cell::unsync::OnceCell;
use ::std::borrow::Borrow;
use ::std::iter;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};
use protobuf::{DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    lazy_input_files: OnceCell<Vec<Rc<InputFile>>>,
    lazy_fqtn_to_type_map: OnceCell<HashMap<String, MessageOrEnum>>,
}

#[derive(Debug, Clone)]
pub struct InputFile {
    context: Weak<Context>,
    package: Rc<Vec<String>>,
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
}

#[derive(Debug, Clone)]
pub struct Message {
    input_file: Weak<InputFile>,
    rust_ident: String,
    package: Rc<Vec<String>>,
    outer_messages: Rc<Vec<String>>,
    lazy_fields: OnceCell<Vec<Field>>,
    nested_messages: Vec<Rc<Message>>,
    nested_enums: Vec<Rc<Enum>>,
}

#[derive(Debug, Clone)]
pub struct Enum {
    input_file: Weak<InputFile>,
    rust_ident: String,
    package: Rc<Vec<String>>,
    outer_messages: Rc<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Field {
    rust_ident: String,
    proto_type: FieldType,
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
    Enum(Rc<Enum>),
    Message(Weak<Message>),
}

#[derive(Debug, Clone)]
pub enum MessageOrEnum {
    Message(Rc<Message>),
    Enum(Rc<Enum>),
}

impl InputFile {
    pub fn try_from_proto(context: Rc<Context>, proto: FileDescriptorProto) -> Result<Self> {
        todo!()
    }
}

impl Message {
    pub fn try_from_proto(
        input_file: Rc<InputFile>,
        proto: DescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Self> {
        let proto_name = proto.name.ok_or(ErrorKind::EmptyInputField {
            name: "DescriptorProto.name".to_string(),
        })?;
        let new_outer_messages = Rc::new({
            let mut v = outer_messages.deref().clone();
            v.push(proto_name.clone());
            v
        });
        Ok(Self {
            input_file: Rc::downgrade(&input_file),
            rust_ident: utils::get_keyword_safe_ident(&utils::to_camel_case(&proto_name)),
            package: package.clone(),
            outer_messages: outer_messages.clone(),
            lazy_fields: OnceCell::new(), // delayed initialize
            nested_messages: proto
                .nested_type
                .into_iter()
                .map(|proto| -> Result<_> {
                    Ok(Rc::new(Message::try_from_proto(
                        input_file.clone(),
                        proto,
                        package.clone(),
                        new_outer_messages.clone(),
                    )?))
                })
                .collect::<Result<Vec<_>>>()?,
            nested_enums: proto
                .enum_type
                .into_iter()
                .map(|proto| -> Result<_> {
                    Ok(Rc::new(Enum::try_from_proto(
                        input_file.clone(),
                        proto,
                        package.clone(),
                        new_outer_messages.clone(),
                    )?))
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn rust_absolute_path(&self) -> String {
        format!(
            "{path}::{ident}",
            path = make_module_path(
                self.package.iter().map(|s| s.borrow()),
                self.outer_messages.iter().map(|s| s.borrow())
            ),
            ident = self.rust_ident
        )
    }

    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn package(&self) -> &[String] {
        &self.package
    }
    pub fn outer_messages(&self) -> &[String] {
        &self.outer_messages
    }
    pub fn fields(&self) -> &[Field] {
        todo!()
    }
    pub fn nested_messages(&self) -> &[Rc<Message>] {
        &self.nested_messages
    }
    pub fn nested_enums(&self) -> &[Rc<Enum>] {
        &self.nested_enums
    }
}

impl Enum {
    pub fn try_from_proto(
        input_file: Rc<InputFile>,
        proto: EnumDescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Self> {
        let proto_name = proto.name.ok_or(ErrorKind::EmptyInputField {
            name: "EnumDescriptorProto.name".to_string(),
        })?;
        Ok(Self {
            input_file: Rc::downgrade(&input_file),
            rust_ident: utils::get_keyword_safe_ident(&utils::to_camel_case(&proto_name)),
            package: package,
            outer_messages: outer_messages,
        })
    }

    pub fn rust_absolute_path(&self) -> String {
        format!(
            "{path}::{ident}",
            path = make_module_path(
                self.package.iter().map(|s| s.borrow()),
                self.outer_messages.iter().map(|s| s.borrow())
            ),
            ident = self.rust_ident
        )
    }
}

impl Field {
    pub fn try_from_proto(proto: FieldDescriptorProto) -> Result<Self> {
        let proto_name = proto.name.ok_or(ErrorKind::EmptyInputField {
            name: "FieldDescriptorProto.name".to_string(),
        })?;
        Ok(Self {
            rust_ident: utils::get_keyword_safe_ident(&utils::to_lower_snake_case(&proto_name)),
            proto_type: todo!(),
        })
    }
}

impl FieldType {
    pub fn try_from_type_proto(proto_type: FieldTypeProto) -> Result<Self> {
        Ok(match proto_type {
            FieldTypeProto::TypeDouble => FieldType::Double,
            FieldTypeProto::TypeFloat => FieldType::Float,
            FieldTypeProto::TypeInt64 => FieldType::Int64,
            FieldTypeProto::TypeUint64 => FieldType::UInt64,
            FieldTypeProto::TypeInt32 => FieldType::Int32,
            FieldTypeProto::TypeFixed64 => FieldType::Fixed64,
            FieldTypeProto::TypeFixed32 => FieldType::Fixed32,
            FieldTypeProto::TypeBool => FieldType::Bool,
            FieldTypeProto::TypeString => FieldType::String,
            FieldTypeProto::TypeGroup => FieldType::Group,
            FieldTypeProto::TypeMessage => FieldType::Message(todo!()),
            FieldTypeProto::TypeBytes => FieldType::Bytes,
            FieldTypeProto::TypeUint32 => FieldType::UInt32,
            FieldTypeProto::TypeEnum => FieldType::Enum(todo!()),
            FieldTypeProto::TypeSfixed32 => FieldType::SFixed32,
            FieldTypeProto::TypeSfixed64 => FieldType::SFixed64,
            FieldTypeProto::TypeSint32 => FieldType::SInt32,
            FieldTypeProto::TypeSint64 => FieldType::SInt64,
        })
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
