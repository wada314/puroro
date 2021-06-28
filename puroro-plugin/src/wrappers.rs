#![allow(unused)]

use crate::protos::enums::google::protobuf::field_descriptor_proto::Type as FieldTypeProto;
use crate::protos::simple::google::protobuf;
use crate::utils;
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::once_cell::unsync::{Lazy, OnceCell};
use ::std::borrow::Borrow;
use ::std::iter;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};
use protobuf::compiler::CodeGeneratorRequest;
use protobuf::{DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Context {
    input_files: Vec<Rc<InputFile>>,
    lazy_fqtn_to_type_map: OnceCell<HashMap<String, MessageOrEnum>>,
}

#[derive(Debug)]
pub struct InputFile {
    context: Weak<Context>,
    package: Rc<Vec<String>>,
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
}

#[derive(Debug)]
pub struct Message {
    input_file: Weak<InputFile>,
    rust_ident: String,
    package: Rc<Vec<String>>,
    outer_messages: Rc<Vec<String>>,
    lazy_fields: OnceCell<Vec<Field>>,
    nested_messages: Vec<Rc<Message>>,
    nested_enums: Vec<Rc<Enum>>,
}

#[derive(Debug)]
pub struct Enum {
    input_file: Weak<InputFile>,
    rust_ident: String,
    package: Rc<Vec<String>>,
    outer_messages: Rc<Vec<String>>,
}

#[derive(Debug)]
pub struct Field {
    rust_ident: String,
    proto_type: FieldType,
}

#[derive(Debug)]
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
    Enum(Weak<Enum>),
    Message(Weak<Message>),
}

#[derive(Debug)]
pub enum MessageOrEnum {
    Message(Rc<Message>),
    Enum(Rc<Enum>),
}

impl Context {
    pub fn try_from_proto(proto: CodeGeneratorRequest) -> Result<Rc<Context>> {
        // lazy fields initialization order is important.
        //  1. Context::lazy_input_files,
        //  2. Context::lazy_fqtn_to_type_map,
        //  3. Message::lazy_fields
        // because the latters refer the formers.
        let context = Rc::new_cyclic(|weak_context| Context {
            input_files: proto
                .proto_file
                .into_iter()
                .map(|file| InputFile::try_from_proto(weak_context.clone(), file))
                .collect::<Result<Vec<_>>>()
                .expect("I need try_new_cyclic..."),
            lazy_fqtn_to_type_map: OnceCell::new(),
        });
        context
            .lazy_fqtn_to_type_map
            .set(context.generate_fqtn_to_type_map()?);
        Ok(context)
    }

    pub fn input_files(&self) -> &[Rc<InputFile>] {
        &self.input_files
    }

    pub fn get_type_from_fqtn(&self, fqtn: &str) -> Option<&MessageOrEnum> {
        let fqtn = fqtn.trim_start_matches('.');
        self.lazy_fqtn_to_type_map
            .get()
            .and_then(|map| map.get(fqtn))
    }

    fn generate_fqtn_to_type_map(&self) -> Result<HashMap<String, MessageOrEnum>> {
        let mut map = HashMap::new();
        self.visit_message_or_enums(|more| {
            map.insert(more.fully_qualified_type_name(), more);
            Ok(())
        })?;
        Ok(map)
    }

    fn visit_message_or_enums<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(MessageOrEnum) -> Result<()>,
    {
        let mut visit_queue = VecDeque::new();
        // init queue
        for file in self.input_files() {
            visit_queue.extend(
                file.messages()
                    .iter()
                    .map(|m| MessageOrEnum::Message(Clone::clone(m))),
            );
            visit_queue.extend(
                file.enums()
                    .iter()
                    .map(|e| MessageOrEnum::Enum(Clone::clone(e))),
            );
        }
        while let Some(item) = visit_queue.pop_front() {
            // extend queue for the item's children
            if let MessageOrEnum::Message(submsg) = &item {
                visit_queue.extend(
                    submsg
                        .nested_messages()
                        .iter()
                        .map(|m| MessageOrEnum::Message(Clone::clone(m))),
                );
                visit_queue.extend(
                    submsg
                        .nested_enums()
                        .iter()
                        .map(|e| MessageOrEnum::Enum(Clone::clone(e))),
                );
            }
            // visit
            (f)(item)?;
        }

        Ok(())
    }
}

impl InputFile {
    pub fn try_from_proto(context: Weak<Context>, proto: FileDescriptorProto) -> Result<Rc<Self>> {
        let package = Rc::new(
            proto
                .package
                .unwrap_or_default()
                .split('.')
                .filter_map(|p| {
                    if p.is_empty() {
                        None
                    } else {
                        Some(p.to_string())
                    }
                })
                .collect_vec(),
        );
        let proto_messages = proto.message_type;
        let proto_enums = proto.enum_type;
        let mut file = Rc::new_cyclic(|file| Self {
            context: context,
            package: Clone::clone(&package),
            messages: proto_messages
                .into_iter()
                .map(|m| {
                    Message::try_from_proto(
                        file.clone(),
                        m,
                        Clone::clone(&package),
                        Rc::new(Vec::new()),
                    )
                })
                .collect::<Result<Vec<_>>>()
                .expect("I need try_new_cyclic..."),
            enums: proto_enums
                .into_iter()
                .map(|e| {
                    Enum::try_from_proto(
                        file.clone(),
                        e,
                        Clone::clone(&package),
                        Rc::new(Vec::new()),
                    )
                })
                .collect::<Result<Vec<_>>>()
                .expect("I need try_new_cyclic..."),
        });
        Ok(file)
    }

    fn messages(&self) -> &[Rc<Message>] {
        &self.messages
    }

    fn enums(&self) -> &[Rc<Enum>] {
        &self.enums
    }
}

impl Message {
    pub fn try_from_proto(
        input_file: Weak<InputFile>,
        proto: DescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Rc<Self>> {
        let proto_name = proto.name.ok_or(ErrorKind::EmptyInputField {
            name: "DescriptorProto.name".to_string(),
        })?;
        let new_outer_messages = Rc::new({
            let mut v = outer_messages.deref().clone();
            v.push(proto_name.clone());
            v
        });
        let message = Rc::new(Self {
            input_file: Clone::clone(&input_file),
            rust_ident: utils::get_keyword_safe_ident(&utils::to_camel_case(&proto_name)),
            package: package.clone(),
            outer_messages: outer_messages.clone(),
            // delay initialize. do it after Context::lazy_fqtn_to_type_map is initialized!
            lazy_fields: OnceCell::new(),
            nested_messages: proto
                .nested_type
                .into_iter()
                .map(|proto| -> Result<_> {
                    Ok(Message::try_from_proto(
                        Clone::clone(&input_file),
                        proto,
                        package.clone(),
                        new_outer_messages.clone(),
                    )?)
                })
                .collect::<Result<Vec<_>>>()?,
            nested_enums: proto
                .enum_type
                .into_iter()
                .map(|proto| -> Result<_> {
                    Ok(Enum::try_from_proto(
                        Clone::clone(&input_file),
                        proto,
                        package.clone(),
                        new_outer_messages.clone(),
                    )?)
                })
                .collect::<Result<Vec<_>>>()?,
        });
        Ok(message)
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
        input_file: Weak<InputFile>,
        proto: EnumDescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Rc<Self>> {
        let proto_name = proto.name.ok_or(ErrorKind::EmptyInputField {
            name: "EnumDescriptorProto.name".to_string(),
        })?;
        Ok(Rc::new(Self {
            input_file: input_file,
            rust_ident: utils::get_keyword_safe_ident(&utils::to_camel_case(&proto_name)),
            package: package,
            outer_messages: outer_messages,
        }))
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
    pub fn try_from_proto(message: Weak<Message>, proto: FieldDescriptorProto) -> Result<Self> {
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

impl MessageOrEnum {
    fn fully_qualified_type_name(&self) -> String {
        match self {
            MessageOrEnum::Message(m) => m
                .package()
                .iter()
                .chain(m.outer_messages().iter())
                .join("."),
            MessageOrEnum::Enum(e) => e
                .package()
                .iter()
                .chain(e.outer_messages().iter())
                .join("."),
        }
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
