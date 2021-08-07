#![allow(unused)]

use crate::protos::google::protobuf;
use crate::protos::google::protobuf::field_descriptor_proto::{
    Label as FieldLabelProto, Type as FieldTypeProto,
};
use crate::utils::*;
use crate::{ErrorKind, Result};
use ::askama::Template;
use ::itertools::Itertools;
use ::once_cell::unsync::{Lazy, OnceCell};
use ::std::borrow::Borrow;
use ::std::iter;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};
use protobuf::compiler::CodeGeneratorRequest;
use protobuf::{DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

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
    syntax: ProtoSyntax,
}

#[derive(Debug)]
pub struct Message {
    input_file: Weak<InputFile>,
    rust_ident: String,
    rust_nested_module_ident: String,
    proto_name: String,
    package: Rc<Vec<String>>,
    outer_messages: Rc<Vec<String>>,
    fields: Vec<Rc<Field>>,
    nested_messages: Vec<Rc<Message>>,
    nested_enums: Vec<Rc<Enum>>,
}

#[derive(Debug)]
pub struct Enum {
    input_file: Weak<InputFile>,
    rust_ident: String,
    proto_name: String,
    package: Rc<Vec<String>>,
    outer_messages: Rc<Vec<String>>,
    values: Vec<EnumValue>,
}

#[derive(Debug)]
pub struct Field {
    message: Weak<Message>,
    rust_ident: String,
    lazy_proto_type: OnceCell<FieldType>,
    proto_name: String,
    proto_type_name: String,
    proto_type_enum: FieldTypeProto,
    proto_label: FieldLabelProto,
    proto_is_optional3: bool,
    lazy_label: OnceCell<FieldLabel>,
    number: i32,
}

#[derive(Debug)]
pub struct EnumValue {
    rust_ident: String,
    number: i32,
}

#[derive(Debug, Clone)]
pub enum ProtoSyntax {
    Proto2,
    Proto3,
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
    Enum(Weak<Enum>),
    Message(Weak<Message>),
}
// Do not cache this type! This type contains a strong `Rc` ponter
// to other `Message` type, causes cycle-reference if it is not handled properly.
#[derive(Debug, Clone)]
pub enum MaybeEnumOrMessage {
    Enum(Rc<Enum>),
    Message(Rc<Message>),
    Others,
}

#[derive(Debug, Clone)]
pub enum FieldLabel {
    Required,
    Optional,
    Unlabeled,
    Repeated,
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
        let proto_syntax = proto.syntax;
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
            syntax: match proto_syntax.as_deref() {
                Some("proto2") | None => ProtoSyntax::Proto2,
                Some("proto3") => ProtoSyntax::Proto3,
                Some(syntax) => Err(ErrorKind::UnknownProtoSyntax {
                    name: syntax.to_owned(),
                })
                .expect("I need try_new_cyclic..."),
            },
        });
        Ok(file)
    }

    pub fn context(&self) -> Result<Rc<Context>> {
        Ok(upgrade(&self.context)?)
    }

    pub fn package(&self) -> &[String] {
        &self.package
    }
    pub fn messages(&self) -> &[Rc<Message>] {
        &self.messages
    }
    pub fn enums(&self) -> &[Rc<Enum>] {
        &self.enums
    }
    pub fn syntax(&self) -> ProtoSyntax {
        self.syntax.clone()
    }
    pub fn rust_path_to_root_module(&self) -> String {
        iter::repeat("super").take(self.package().len()).join("::")
    }
}

impl Message {
    pub fn try_from_proto(
        input_file: Weak<InputFile>,
        proto: DescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Rc<Self>> {
        let proto_name = proto
            .name
            .ok_or(ErrorKind::EmptyInputField {
                name: "DescriptorProto.name".into(),
            })?
            .to_string();
        let new_outer_messages = Rc::new({
            let mut v = outer_messages.deref().clone();
            v.push(proto_name.clone());
            v
        });
        let proto_field = proto.field;
        let proto_nested_type = proto.nested_type;
        let proto_enum_type = proto.enum_type;
        let message = Rc::new_cyclic(|message| Self {
            input_file: Clone::clone(&input_file),
            rust_ident: get_keyword_safe_ident(&to_camel_case(&proto_name)),
            rust_nested_module_ident: get_keyword_safe_ident(&to_lower_snake_case(&proto_name)),
            proto_name,
            package: package.clone(),
            outer_messages: outer_messages.clone(),
            fields: proto_field
                .into_iter()
                .map(|f| Field::try_from_proto(Clone::clone(message), f).map(|x| Rc::new(x)))
                .collect::<Result<Vec<_>>>()
                .expect("I need try_new_cyclic..."),
            nested_messages: proto_nested_type
                .into_iter()
                .map(|proto| -> Result<_> {
                    Ok(Message::try_from_proto(
                        Clone::clone(&input_file),
                        proto,
                        package.clone(),
                        new_outer_messages.clone(),
                    )?)
                })
                .collect::<Result<Vec<_>>>()
                .expect("I need try_new_cyclic..."),
            nested_enums: proto_enum_type
                .into_iter()
                .map(|proto| -> Result<_> {
                    Ok(Enum::try_from_proto(
                        Clone::clone(&input_file),
                        proto,
                        package.clone(),
                        new_outer_messages.clone(),
                    )?)
                })
                .collect::<Result<Vec<_>>>()
                .expect("I need try_new_cyclic..."),
        });
        Ok(message)
    }

    pub fn rust_absolute_path(&self) -> String {
        format!(
            "{path}::{ident}",
            path = self.rust_absolute_module_path(),
            ident = self.rust_ident
        )
    }
    pub fn rust_absolute_trait_path(&self) -> String {
        format!(
            "{path}::puroro_traits::{ident}Trait",
            path = self.rust_absolute_module_path(),
            ident = self.rust_ident
        )
    }
    pub fn rust_absolute_module_path(&self) -> String {
        make_module_path(
            self.package.iter().map(|s| s.borrow()),
            self.outer_messages.iter().map(|s| s.borrow()),
        )
    }

    pub fn input_file(&self) -> Result<Rc<InputFile>> {
        Ok(upgrade(&self.input_file)?)
    }
    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn rust_nested_module_ident(&self) -> &str {
        &self.rust_nested_module_ident
    }
    pub fn proto_name(&self) -> &str {
        &self.proto_name
    }
    pub fn package(&self) -> &[String] {
        &self.package
    }
    pub fn outer_messages(&self) -> &[String] {
        &self.outer_messages
    }
    pub fn fields(&self) -> &[Rc<Field>] {
        &self.fields
    }
    pub fn nested_messages(&self) -> &[Rc<Message>] {
        &self.nested_messages
    }
    pub fn nested_enums(&self) -> &[Rc<Enum>] {
        &self.nested_enums
    }

    pub fn has_nested_items(&self) -> bool {
        self.nested_messages().len() + self.nested_enums().len() > 0
    }
}

impl Enum {
    pub fn try_from_proto(
        input_file: Weak<InputFile>,
        proto: EnumDescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Rc<Self>> {
        let proto_name = proto
            .name
            .ok_or(ErrorKind::EmptyInputField {
                name: "EnumDescriptorProto.name".into(),
            })?
            .to_string();
        let proto_value = proto.value;
        Ok(Rc::new(Self {
            input_file: input_file,
            rust_ident: get_keyword_safe_ident(&to_camel_case(&proto_name)),
            proto_name,
            package: package,
            outer_messages: outer_messages,
            values: proto_value
                .into_iter()
                .map(|v| EnumValue {
                    rust_ident: get_keyword_safe_ident(&to_camel_case(&v.name.unwrap_or_default())),
                    number: v.number.unwrap_or_default(),
                })
                .collect_vec(),
        }))
    }

    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn proto_name(&self) -> &str {
        &self.proto_name
    }
    pub fn package(&self) -> &[String] {
        &self.package
    }
    pub fn outer_messages(&self) -> &[String] {
        &self.outer_messages
    }
    pub fn values(&self) -> &[EnumValue] {
        &self.values
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

    pub fn first_value(&self) -> Result<&EnumValue> {
        Ok(self.values.first().ok_or(ErrorKind::EmptyEnum {
            name: self.proto_name.clone(),
        })?)
    }
}

impl Field {
    pub fn try_from_proto(message: Weak<Message>, proto: FieldDescriptorProto) -> Result<Self> {
        let proto_name = proto
            .name
            .ok_or(ErrorKind::EmptyInputField {
                name: "FieldDescriptorProto.name".into(),
            })?
            .to_string();
        let proto_type_name = proto.type_name.unwrap_or_default().to_string();
        let proto_type_enum = proto.type_.ok_or(ErrorKind::InternalError {
            detail: "currently we are assuming the field type enum is always set.".to_string(),
        })?;
        let proto_label = proto.label.ok_or(ErrorKind::InternalError {
            detail: "currently we are assuming the field label enum is always set.".to_string(),
        })?;
        let proto_is_optional3 = proto.proto3_optional.unwrap_or_default();
        let proto_number = proto.number.unwrap_or_default();
        Ok(Self {
            message: Clone::clone(&message),
            rust_ident: get_keyword_safe_ident(&to_lower_snake_case(&proto_name)),
            proto_name,
            proto_type_name,
            proto_type_enum,
            lazy_proto_type: OnceCell::new(),
            proto_label,
            proto_is_optional3,
            lazy_label: OnceCell::new(),
            number: proto_number,
        })
    }

    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn message(&self) -> Result<Rc<Message>> {
        Ok(upgrade(&self.message)?)
    }
    pub fn field_type(&self) -> Result<FieldType> {
        self.lazy_proto_type
            .get_or_try_init(|| {
                FieldType::try_from_type_proto(
                    Clone::clone(&self.message),
                    &self.proto_type_enum,
                    &self.proto_type_name,
                )
            })
            .map(|x| x.clone())
    }
    pub fn field_label(&self) -> Result<FieldLabel> {
        self.lazy_label
            .get_or_try_init(|| {
                Ok(if self.proto_is_optional3 {
                    FieldLabel::Optional
                } else {
                    match self.proto_label {
                        FieldLabelProto::LabelOptional => {
                            match self.message()?.input_file()?.syntax() {
                                ProtoSyntax::Proto2 => FieldLabel::Optional,
                                ProtoSyntax::Proto3 => FieldLabel::Unlabeled,
                            }
                        }
                        FieldLabelProto::LabelRequired => FieldLabel::Required,
                        FieldLabelProto::LabelRepeated => FieldLabel::Repeated,
                    }
                })
            })
            .map(|l| l.clone())
    }
    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn syntax_and_label_and_type_tags(&self) -> Result<String> {
        Ok(format!(
            "::puroro::tags::{syntax}, ::puroro::tags::{label}, \
                ::puroro::tags::{vtype}",
            syntax = self.message()?.input_file()?.syntax().tag_ident(),
            label = self.field_label()?.tag_ident(),
            vtype = self.field_type()?.tag_ident()?
        ))
    }
    pub fn syntax_and_label_tags(&self) -> Result<String> {
        Ok(format!(
            "::puroro::tags::{syntax}, ::puroro::tags::{label}",
            syntax = self.message()?.input_file()?.syntax().tag_ident(),
            label = self.field_label()?.tag_ident(),
        ))
    }

    pub fn rust_one_line_comment(&self) -> Result<String> {
        Ok(format!(
            "{label_then_space}{field_type} {name} = {number};",
            label_then_space = match self.field_label()? {
                FieldLabel::Unlabeled => "",
                FieldLabel::Required => "required ",
                FieldLabel::Optional => "optional ",
                FieldLabel::Repeated => "repeated ",
            },
            field_type = self.field_type()?.proto_name()?,
            name = &self.proto_name,
            number = self.number(),
        ))
    }

    pub fn trait_scalar_getter_type(&self) -> Result<String> {
        Ok(match self.field_type()? {
            FieldType::Double => "f64".to_string(),
            FieldType::Float => "f32".to_string(),
            FieldType::Int32 => "i32".to_string(),
            FieldType::Int64 => "i64".to_string(),
            FieldType::UInt32 => "u32".to_string(),
            FieldType::UInt64 => "u64".to_string(),
            FieldType::SInt32 => "i32".to_string(),
            FieldType::SInt64 => "i64".to_string(),
            FieldType::Fixed32 => "u32".to_string(),
            FieldType::Fixed64 => "u64".to_string(),
            FieldType::SFixed32 => "i32".to_string(),
            FieldType::SFixed64 => "i64".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "::std::borrow::Cow<'this, str>".to_string(),
            FieldType::Bytes => "::std::borrow::Cow<'this, [u8]>".to_string(),
            FieldType::Enum(e) => match self.message()?.input_file()?.syntax() {
                ProtoSyntax::Proto2 => upgrade(&e)?.rust_absolute_path(),
                ProtoSyntax::Proto3 => format!(
                    "::std::result::Result<{}, i32>",
                    upgrade(&e)?.rust_absolute_path()
                ),
            },
            FieldType::Message(m) => format!(
                "::std::borrow::Cow<\
                    'this, Self::Field{number}MessageType<'this>\
                >",
                number = self.number(),
            ),
        })
    }
    pub fn maybe_trait_scalar_getter_type_borrowed(
        &self,
        impl_tag: &str,
    ) -> Result<Option<String>> {
        Ok(match self.field_type() {
            Ok(FieldType::String) => Some("str".to_string()),
            Ok(FieldType::Bytes) => Some("[u8]".to_string()),
            Ok(FieldType::Message(m)) => Some(format!(
                "{path}<{tag}>",
                path = upgrade(&m)?.rust_absolute_path(),
                tag = impl_tag,
            )),
            _ => None,
        })
    }

    pub fn has_scalar_getter(&self) -> bool {
        matches!(self.field_label(), Ok(FieldLabel::Unlabeled))
            && !matches!(self.field_type(), Ok(FieldType::Message(_)))
    }
    pub fn has_scalar_optional_getter(&self) -> bool {
        match self.field_label() {
            Ok(FieldLabel::Optional | FieldLabel::Required) => true,
            Ok(FieldLabel::Unlabeled) => matches!(self.field_type(), Ok(FieldType::Message(_))),
            _ => false,
        }
    }
    pub fn has_repeated_getter(&self) -> bool {
        matches!(self.field_label(), Ok(FieldLabel::Repeated))
    }
}

impl EnumValue {
    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn number(&self) -> i32 {
        self.number
    }
}

impl ProtoSyntax {
    pub fn tag_ident(&self) -> &str {
        match *self {
            ProtoSyntax::Proto2 => "Proto2",
            ProtoSyntax::Proto3 => "Proto3",
        }
    }
}

impl FieldType {
    pub fn try_from_type_proto(
        message: Weak<Message>,
        proto_type_enum: &FieldTypeProto,
        proto_type_name: &str,
    ) -> Result<Self> {
        let context = upgrade(&message)?.input_file()?.context()?;
        Ok(match proto_type_enum {
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
            FieldTypeProto::TypeMessage => match context.get_type_from_fqtn(&proto_type_name) {
                Some(MessageOrEnum::Message(m)) => FieldType::Message(Rc::downgrade(m)),
                _ => Err(ErrorKind::UnknownTypeName {
                    name: proto_type_name.to_string(),
                })?,
            },
            FieldTypeProto::TypeBytes => FieldType::Bytes,
            FieldTypeProto::TypeUint32 => FieldType::UInt32,
            FieldTypeProto::TypeEnum => match context.get_type_from_fqtn(&proto_type_name) {
                Some(MessageOrEnum::Enum(e)) => FieldType::Enum(Rc::downgrade(e)),
                _ => Err(ErrorKind::UnknownTypeName {
                    name: proto_type_name.to_string(),
                })?,
            },
            FieldTypeProto::TypeSfixed32 => FieldType::SFixed32,
            FieldTypeProto::TypeSfixed64 => FieldType::SFixed64,
            FieldTypeProto::TypeSint32 => FieldType::SInt32,
            FieldTypeProto::TypeSint64 => FieldType::SInt64,
        })
    }

    pub fn maybe_enum_or_message(&self) -> Result<MaybeEnumOrMessage> {
        Ok(match self {
            FieldType::Enum(e) => MaybeEnumOrMessage::Enum(upgrade(e)?),
            FieldType::Message(m) => MaybeEnumOrMessage::Message(upgrade(m)?),
            _ => MaybeEnumOrMessage::Others,
        })
    }
    pub fn maybe_message(&self) -> Result<Option<Rc<Message>>> {
        Ok(if let FieldType::Message(m) = self {
            Some(upgrade(m)?)
        } else {
            None
        })
    }
    pub fn is_message(&self) -> bool {
        matches!(self, FieldType::Message(_))
    }

    pub fn tag_ident(&self) -> Result<&'static str> {
        Ok(match *self {
            FieldType::Double => "Double",
            FieldType::Float => "Float",
            FieldType::Int32 => "Int32",
            FieldType::Int64 => "Int64",
            FieldType::UInt32 => "UInt32",
            FieldType::UInt64 => "UInt64",
            FieldType::SInt32 => "SInt32",
            FieldType::SInt64 => "SInt64",
            FieldType::Fixed32 => "Fixed32",
            FieldType::Fixed64 => "Fixed64",
            FieldType::SFixed32 => "SFixed32",
            FieldType::SFixed64 => "SFixed64",
            FieldType::Bool => "Bool",
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "String",
            FieldType::Bytes => "Bytes",
            FieldType::Enum(_) => "Enum",
            FieldType::Message(_) => "Message",
        })
    }

    pub fn numerical_rust_type(&self) -> Result<&'static str> {
        Ok(match *self {
            FieldType::Double => "f64",
            FieldType::Float => "f32",
            FieldType::Int32 => "i32",
            FieldType::Int64 => "i64",
            FieldType::UInt32 => "u32",
            FieldType::UInt64 => "u64",
            FieldType::SInt32 => "i32",
            FieldType::SInt64 => "i64",
            FieldType::Fixed32 => "u32",
            FieldType::Fixed64 => "u64",
            FieldType::SFixed32 => "i32",
            FieldType::SFixed64 => "i64",
            FieldType::Bool => "bool",
            _ => Err(ErrorKind::InternalError {
                detail: "numerical_rust_type() is invoked for non-numerical type".to_string(),
            })?,
        })
    }

    pub fn proto_name(&self) -> Result<String> {
        Ok(match self {
            FieldType::Double => "double".to_string(),
            FieldType::Float => "float".to_string(),
            FieldType::Int32 => "int32".to_string(),
            FieldType::Int64 => "int64".to_string(),
            FieldType::UInt32 => "uint32".to_string(),
            FieldType::UInt64 => "uint64".to_string(),
            FieldType::SInt32 => "sint32".to_string(),
            FieldType::SInt64 => "sint64".to_string(),
            FieldType::Fixed32 => "fixed32".to_string(),
            FieldType::Fixed64 => "fixed64".to_string(),
            FieldType::SFixed32 => "sfixed32".to_string(),
            FieldType::SFixed64 => "sfixed64".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "string".to_string(),
            FieldType::Bytes => "bytes".to_string(),
            FieldType::Enum(e) => upgrade(e)?.proto_name().to_string(),
            FieldType::Message(m) => upgrade(m)?.proto_name().to_string(),
        })
    }
}

impl FieldLabel {
    pub fn tag_ident(&self) -> &str {
        match *self {
            FieldLabel::Required => "Required",
            FieldLabel::Optional => "Optional",
            FieldLabel::Unlabeled => "Unlabeled",
            FieldLabel::Repeated => "Repeated",
        }
    }
}

impl MessageOrEnum {
    fn fully_qualified_type_name(&self) -> String {
        match self {
            MessageOrEnum::Message(m) => m
                .package()
                .iter()
                .chain(m.outer_messages().iter())
                .chain(iter::once(&m.proto_name().to_string()))
                .join("."),
            MessageOrEnum::Enum(e) => e
                .package()
                .iter()
                .chain(e.outer_messages().iter())
                .chain(iter::once(&e.proto_name().to_string()))
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
