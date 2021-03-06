// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused)]

use crate::utils::*;
use crate::{ErrorKind, Result};
use ::askama::Template;
use ::itertools::Itertools;
use ::once_cell::unsync::{Lazy, OnceCell};
use ::puroro_protobuf_compiled::google::protobuf;
use ::std::borrow::Borrow;
use ::std::borrow::Cow;
use ::std::collections::{HashMap, VecDeque};
use ::std::convert::TryInto;
use ::std::hash::Hash;
use ::std::iter;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};
use protobuf::compiler::CodeGeneratorRequest;
use protobuf::field_descriptor_proto::{Label as FieldLabelProto, Type as FieldTypeProto};
use protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    OneofDescriptorProto,
};
use puroro::tags::LengthDelimited;

#[derive(Debug)]
pub struct Context {
    input_files: Vec<Rc<InputFile>>,
    lazy_fqtn_to_type_map: OnceCell<HashMap<String, MessageOrEnum>>,
}

impl Context {
    pub fn try_from_proto(proto: CodeGeneratorRequest) -> Result<Rc<Context>> {
        let context = Rc::new_cyclic(|weak_context| Context {
            input_files: proto
                .proto_file()
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

#[derive(Debug)]
pub struct InputFile {
    context: Weak<Context>,
    package: Rc<Vec<String>>,
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
    syntax: ProtoSyntax,
}

impl InputFile {
    pub fn try_from_proto(context: Weak<Context>, proto: &FileDescriptorProto) -> Result<Rc<Self>> {
        let package = Rc::new(
            proto
                .package()
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
        let proto_messages = proto.message_type();
        let proto_enums = proto.enum_type();
        let proto_syntax = proto.syntax();
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
            syntax: match proto_syntax {
                "proto2" | "" => ProtoSyntax::Proto2,
                "proto3" => ProtoSyntax::Proto3,
                syntax => Err(ErrorKind::UnknownProtoSyntax {
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
    oneofs: Vec<Rc<Oneof>>,
}

impl Message {
    pub fn try_from_proto(
        input_file: Weak<InputFile>,
        proto: &DescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Rc<Self>> {
        let proto_name = proto.name().to_string();
        let new_outer_messages = Rc::new({
            let mut v = outer_messages.deref().clone();
            v.push(proto_name.clone());
            v
        });
        let proto_field = proto.field();
        let proto_nested_type = proto.nested_type();
        let proto_enum_type = proto.enum_type();
        let proto_oneofs = proto.oneof_decl();
        let message = Rc::new_cyclic(|message| Self {
            input_file: Clone::clone(&input_file),
            rust_ident: get_keyword_safe_ident(&to_camel_case(&proto_name)).to_string(),
            rust_nested_module_ident: get_keyword_safe_ident(&to_lower_snake_case(&proto_name))
                .to_string(),
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
            oneofs: proto_oneofs
                .into_iter()
                .enumerate()
                .map(|(index, proto)| -> Result<_> {
                    let index_i32 = index.try_into().map_err(|_| ErrorKind::TooLargeLength)?;
                    Oneof::try_from_proto(Clone::clone(message), proto, index_i32)
                })
                .collect::<Result<Vec<_>>>()
                .expect("I need try_new_cyclic..."),
        });
        Ok(message)
    }

    pub fn rust_path(&self) -> String {
        format!(
            "{path}::{ident}",
            path = self.rust_module_path(),
            ident = self.rust_ident
        )
    }
    pub fn rust_trait_path(&self) -> String {
        format!(
            "{path}::_puroro_traits::{ident}Trait",
            path = self.rust_module_path(),
            ident = self.rust_ident
        )
    }
    pub fn rust_module_path(&self) -> String {
        make_module_path(
            self.package.iter().map(|s| s.borrow()),
            self.outer_messages.iter().map(|s| s.borrow()),
        )
    }
    pub fn rust_impl_path(&self, impl_name: &str, gp: &[&str]) -> String {
        // "Simple" impls are separeted out to a special namespace.
        let module = if impl_name == "Simple" {
            "_puroro_simple_impl"
        } else {
            "_puroro_impls"
        };
        let maybe_gp = if gp.is_empty() {
            "".to_string()
        } else {
            format!("<{}>", gp.join(", "))
        };
        format!(
            "{path}::{module}::{ident}{maybe_gp}",
            path = self.rust_module_path(),
            module = module,
            ident = self.rust_impl_ident(impl_name),
            maybe_gp = maybe_gp,
        )
    }

    pub fn input_file(&self) -> Result<Rc<InputFile>> {
        Ok(upgrade(&self.input_file)?)
    }
    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn rust_trait_ident(&self) -> String {
        format!("{}Trait", &self.rust_ident)
    }
    pub fn rust_impl_ident(&self, impl_name: &str) -> String {
        // Simple impl uses raw name without suffix.
        if impl_name == "Simple" {
            self.rust_ident.clone()
        } else {
            format!("{}{}", &self.rust_ident, impl_name)
        }
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
    pub fn oneofs(&self) -> &[Rc<Oneof>] {
        &self.oneofs
    }

    pub fn has_nested_items(&self) -> bool {
        self.nested_messages().len() + self.nested_enums().len() + self.oneofs().len() > 0
    }
}
impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.package.hash(state);
        self.outer_messages.hash(state);
        self.proto_name.hash(state);
    }
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

impl Enum {
    pub fn try_from_proto(
        input_file: Weak<InputFile>,
        proto: &EnumDescriptorProto,
        package: Rc<Vec<String>>,
        outer_messages: Rc<Vec<String>>,
    ) -> Result<Rc<Self>> {
        let proto_name = proto.name().to_string();
        let proto_value = proto.value();
        Ok(Rc::new(Self {
            input_file: input_file,
            rust_ident: get_keyword_safe_ident(&to_camel_case(&proto_name)).to_string(),
            proto_name,
            package: package,
            outer_messages: outer_messages,
            values: proto_value
                .into_iter()
                .map(|v| EnumValue {
                    rust_ident: get_keyword_safe_ident(&to_camel_case(&v.name())).to_string(),
                    number: v.number(),
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
    pub fn syntax(&self) -> Result<ProtoSyntax> {
        Ok(upgrade(&self.input_file)?.syntax())
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

    pub fn rust_path(&self) -> String {
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

#[derive(Debug)]
pub struct Field {
    message: Weak<Message>,
    lazy_oneof: OnceCell<Option<Rc<Oneof>>>,
    ident_lower_snake: String,
    ident_lower_snake_unesc: String,
    ident_camel: String,
    ident_camel_unesc: String,
    lazy_proto_type: OnceCell<FieldType>,
    proto_name: String,
    proto_type_name: String,
    proto_type_enum: FieldTypeProto,
    proto_label: FieldLabelProto,
    proto_is_optional3: bool,
    proto_default_value: Option<String>,
    proto_options_packed: Option<bool>,
    lazy_label: OnceCell<FieldLabel>,
    lazy_allow_variant_packing: OnceCell<bool>,
    number: i32,
    proto_oneof_index: Option<i32>,
}

impl Field {
    pub fn try_from_proto(message: Weak<Message>, proto: &FieldDescriptorProto) -> Result<Self> {
        let proto_name = proto.name().to_string();
        let proto_type_name = proto.type_name().to_string();
        let proto_type_enum = proto.r#type();
        let proto_label = proto.label();
        let proto_is_optional3 = proto.proto3_optional();
        let proto_number = proto.number();
        let proto_oneof_index = proto.oneof_index_opt();
        Ok(Self {
            message: Clone::clone(&message),
            lazy_oneof: OnceCell::new(),
            ident_lower_snake: get_keyword_safe_ident(&to_lower_snake_case(&proto_name))
                .to_string(),
            ident_lower_snake_unesc: to_lower_snake_case(&proto_name).to_string(),
            ident_camel: get_keyword_safe_ident(&to_camel_case(&proto_name)).to_string(),
            ident_camel_unesc: to_camel_case(&proto_name).to_string(),
            proto_name,
            proto_type_name,
            proto_type_enum,
            lazy_proto_type: OnceCell::new(),
            proto_label,
            proto_is_optional3,
            proto_default_value: proto.default_value_opt().map(|s| s.to_string()),
            proto_options_packed: proto.options_opt().and_then(|opt| opt.packed_opt()),
            lazy_label: OnceCell::new(),
            lazy_allow_variant_packing: OnceCell::new(),
            number: proto_number,
            proto_oneof_index,
        })
    }

    pub fn proto_name(&self) -> &str {
        &self.proto_name
    }
    pub fn ident_lower_snake(&self) -> &str {
        &self.ident_lower_snake
    }
    pub fn lower_snake_ident_unesc(&self) -> &str {
        &self.ident_lower_snake_unesc
    }
    pub fn ident_camel(&self) -> &str {
        &self.ident_camel
    }
    pub fn ident_camel_unesc(&self) -> &str {
        &self.ident_camel_unesc
    }
    pub fn message(&self) -> Result<Rc<Message>> {
        Ok(upgrade(&self.message)?)
    }
    pub fn oneof(&self) -> Result<Option<Rc<Oneof>>> {
        Ok(if let Some(oneof_index) = self.oneof_index() {
            self.message()?.oneofs().get(oneof_index as usize).cloned()
        } else {
            None
        })
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
                } else if self.is_non_synthetic_oneof_item()? {
                    FieldLabel::OneofField
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
    pub fn is_repeated(&self) -> Result<bool> {
        Ok(matches!(self.field_label()?, FieldLabel::Repeated))
    }
    pub fn is_message(&self) -> Result<bool> {
        Ok(matches!(self.field_type()?, FieldType::Message(_)))
    }
    pub fn is_length_delimited(&self) -> Result<bool> {
        Ok(matches!(
            self.field_type()?,
            FieldType::Message(_) | FieldType::String | FieldType::Bytes
        ))
    }
    pub fn is_optional3(&self) -> bool {
        self.proto_is_optional3
    }
    pub fn default_value(&self) -> Option<&str> {
        self.proto_default_value.as_ref().map(|v| v.as_str())
    }
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn oneof_index(&self) -> Option<i32> {
        self.proto_oneof_index.clone()
    }
    pub fn allow_variant_packing(&self) -> Result<bool> {
        self.lazy_allow_variant_packing
            .get_or_try_init(|| {
                let syntax = self.message()?.input_file()?.syntax();
                // In proto2, packed is true only if it's explicitly set to true.
                // In proto3, packed is false only if it's explicitly set to false.
                Ok(match syntax {
                    ProtoSyntax::Proto2 => self.proto_options_packed.unwrap_or(false),
                    ProtoSyntax::Proto3 => self.proto_options_packed.unwrap_or(true),
                })
            })
            .map(|b| *b)
    }

    pub fn rust_type_tag<F: Fn(&Message) -> Result<String>>(
        &self,
        gen_msg_path: F,
    ) -> Result<String> {
        Ok(format!(
            "::puroro::tags::{type_tag}",
            type_tag = self.field_type()?.tag_ident_and_gp(gen_msg_path)?,
        ))
    }
    pub fn rust_label_and_type_tags<F: Fn(&Message) -> Result<String>>(
        &self,
        gen_msg_path: F,
    ) -> Result<String> {
        Ok(format!(
            "::puroro::tags::{label_tag}, {type_tag}",
            label_tag = self.field_label()?.tag_ident(),
            type_tag = self.rust_type_tag(gen_msg_path)?,
        ))
    }

    pub fn rust_one_line_comment(&self) -> Result<String> {
        Ok(format!(
            "{label_then_space}{field_type} {name} = {number};",
            label_then_space = match self.field_label()? {
                FieldLabel::Unlabeled | FieldLabel::OneofField => "",
                FieldLabel::Required => "required ",
                FieldLabel::Optional => "optional ",
                FieldLabel::Repeated => "repeated ",
            },
            field_type = self.field_type()?.proto_name()?,
            name = &self.proto_name,
            number = self.number(),
        ))
    }

    pub fn trait_scalar_getter_type(&self) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => "&'this str".into(),
            LengthDelimited(Bytes) => "&'this [u8]".into(),
            LengthDelimited(Message(_)) => format!(
                "Self::{ident_camel}MessageType<'this>",
                ident_camel = self.ident_camel_unesc
            )
            .into(),
            Trivial(field_type) => field_type.rust_type_name()?,
        })
    }
    pub fn trait_oneof_field_type(&self, lt: &str, trait_impl: &str) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => format!("&{lt} str", lt = lt).into(),
            LengthDelimited(Bytes) => format!("&{lt} [u8]", lt = lt).into(),
            LengthDelimited(Message(_)) => format!(
                "<{trait_impl} as {trait_path}>::{ident_camel}MessageType<{lt}>",
                lt = lt,
                trait_impl = trait_impl,
                trait_path = self.message()?.rust_trait_path(),
                ident_camel = self.ident_camel_unesc,
            )
            .into(),
            Trivial(field_type) => field_type.rust_type_name()?,
        })
    }

    pub fn maybe_trait_scalar_getter_type_borrowed(
        &self,
        impl_name: &str,
        gp: &[&str],
    ) -> Result<Option<String>> {
        Ok(match self.field_type() {
            Ok(FieldType::String) => Some("str".to_string()),
            Ok(FieldType::Bytes) => Some("[u8]".to_string()),
            Ok(FieldType::Message(m)) => Some(upgrade(&m)?.rust_impl_path(impl_name, gp)),
            _ => None,
        })
    }

    pub fn is_non_synthetic_oneof_item(&self) -> Result<bool> {
        let message = self.message()?;
        let maybe_oneof = self
            .oneof_index()
            .and_then(|index| message.oneofs().get(index as usize));
        return maybe_oneof.map_or(Ok(false), |oneof| oneof.is_synthetic().map(|b| !b));
    }
    pub fn has_scalar_getter(&self) -> Result<bool> {
        Ok(!self.is_non_synthetic_oneof_item()?
            && matches!(self.field_label(), Ok(FieldLabel::Unlabeled))
            && !matches!(self.field_type(), Ok(FieldType::Message(_))))
    }
    pub fn has_scalar_optional_getter(&self) -> Result<bool> {
        Ok(self.is_non_synthetic_oneof_item()?
            || match self.field_label() {
                Ok(FieldLabel::Optional | FieldLabel::Required) => true,
                Ok(FieldLabel::Unlabeled) => matches!(self.field_type(), Ok(FieldType::Message(_))),
                _ => false,
            })
    }
    pub fn has_repeated_getter(&self) -> Result<bool> {
        Ok(matches!(self.field_label(), Ok(FieldLabel::Repeated)))
    }

    pub fn bumpalo_oneof_field_type(&self) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        let bare_type = match self.field_type()?.categories()? {
            LengthDelimited(String) => "::puroro::internal::NoAllocBumpString".into(),
            LengthDelimited(Bytes) => "::puroro::internal::NoAllocBumpVec<u8>".into(),
            LengthDelimited(Message(m)) => {
                let bumpalo_message_type = upgrade(&m)?.rust_impl_path("Bumpalo", &["'bump"]);
                format!(
                    "::puroro::internal::NoAllocBumpBox<{message_type}>",
                    message_type = bumpalo_message_type,
                )
                .into()
            }
            Trivial(field_type) => field_type.rust_type_name()?,
        };
        Ok(format!("::puroro::internal::Bare<{}>", bare_type).into())
    }

    pub fn simple_field_type(&self) -> Result<Cow<'static, str>> {
        let scalar_type = self.simple_scalar_field_type()?;
        Ok(match self.field_label()? {
            FieldLabel::Repeated => format!("::std::vec::Vec<{}>", scalar_type).into(),
            FieldLabel::OneofField => format!("::puroro::internal::Bare<{}>", scalar_type).into(),
            _ => {
                if matches!(self.field_type(), Ok(FieldType::Message(_))) {
                    format!("::std::option::Option<{}>", scalar_type).into()
                } else {
                    format!("::puroro::internal::Bare<{}>", scalar_type).into()
                }
            }
        })
    }

    pub fn simple_scalar_field_type(&self) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => "::std::string::String".into(),
            LengthDelimited(Bytes) => "::std::vec::Vec<u8>".into(),
            LengthDelimited(Message(m)) => {
                let bare_msg = upgrade(&m)?.rust_impl_path("Simple", &[]);
                if matches!(self.field_label(), Ok(FieldLabel::Repeated)) {
                    bare_msg.into()
                } else {
                    format!("::std::boxed::Box<{}>", bare_msg).into()
                }
            }
            Trivial(field_type) => field_type.rust_type_name()?,
        })
    }

    pub fn simple_getter_scalar_type(&self, lt: &str) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => format!("&{} str", lt).into(),
            LengthDelimited(Bytes) => format!("&{} [u8]", lt).into(),
            LengthDelimited(Message(m)) => {
                let msg_type = upgrade(&m)?.rust_impl_path("Simple", &[]);
                format!("&{lt} {msg}", lt = lt, msg = msg_type).into()
            }
            Trivial(field_type) => field_type.rust_type_name()?,
        })
    }

    pub fn simple_getter_repeated_type(&self, lt: &str) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => {
                format!(
                    "&{lt}[impl ::std::ops::Deref<Target=str> + ::std::fmt::Debug]",
                    lt = lt
                )
            }
            LengthDelimited(Bytes) => {
                format!(
                    "&{lt}[impl ::std::ops::Deref<Target=[u8]> + ::std::fmt::Debug]",
                    lt = lt
                )
            }
            LengthDelimited(Message(m)) => {
                let msg_type = upgrade(&m)?.rust_impl_path("Simple", &[]);
                format!("&{lt}[{msg}]", lt = lt, msg = msg_type)
            }
            Trivial(field_type) => {
                format!("&{lt}[{ty}]", lt = lt, ty = field_type.rust_type_name()?)
            }
        }
        .into())
    }

    pub fn simple_getter_mut_type(&self, lt: &str) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok({
            let scalar_type = match self.field_type()?.categories()? {
                LengthDelimited(String) => "::std::string::String".into(),
                LengthDelimited(Bytes) => "::std::vec::Vec<u8>".into(),
                LengthDelimited(Message(m)) => upgrade(&m)?.rust_impl_path("Simple", &[]).into(),
                Trivial(field_type) => field_type.rust_type_name()?,
            };
            if self.is_repeated()? {
                format!("&{lt} mut ::std::vec::Vec<{ty}>", lt = lt, ty = scalar_type)
            } else {
                format!("&{lt} mut {ty}", lt = lt, ty = scalar_type)
            }
            .into()
        })
    }

    pub fn bumpalo_field_type(&self) -> Result<Cow<'static, str>> {
        let scalar_type = self.bumpalo_scalar_field_type()?;
        if self.is_repeated()? {
            Ok(format!("::puroro::internal::NoAllocBumpVec<{}>", scalar_type).into())
        } else if self.is_message()? {
            Ok(format!(
                "::std::option::Option<::puroro::internal::NoAllocBumpBox<{}>>",
                scalar_type
            )
            .into())
        } else {
            Ok(format!("::puroro::internal::Bare<{}>", scalar_type).into())
        }
    }

    pub fn bumpalo_scalar_field_type(&self) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => "::puroro::internal::NoAllocBumpString".into(),
            LengthDelimited(Bytes) => "::puroro::internal::NoAllocBumpVec<u8>".into(),
            LengthDelimited(Message(m)) => {
                upgrade(&m)?.rust_impl_path("Bumpalo", &["'bump"]).into()
            }
            Trivial(field_type) => field_type.rust_type_name()?,
        })
    }

    pub fn bumpalo_getter_scalar_type(&self, lt: &str) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => format!("&{} str", lt).into(),
            LengthDelimited(Bytes) => format!("&{} [u8]", lt).into(),
            LengthDelimited(Message(m)) => {
                let msg_type = upgrade(&m)?.rust_impl_path("Bumpalo", &[lt]);
                format!("&{lt} {msg}", lt = lt, msg = msg_type).into()
            }
            Trivial(field_type) => field_type.rust_type_name()?,
        })
    }

    pub fn bumpalo_getter_repeated_type(&self, lt: &str) -> Result<Cow<'static, str>> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => {
                format!("&{lt}[impl ::std::ops::Deref<Target=str>]", lt = lt)
            }
            LengthDelimited(Bytes) => {
                format!("&{lt}[impl ::std::ops::Deref<Target=[u8]>]", lt = lt)
            }
            LengthDelimited(Message(m)) => {
                let msg_type = upgrade(&m)?.rust_impl_path("Bumpalo", &[lt]);
                format!("&{lt}[{msg}]", lt = lt, msg = msg_type)
            }
            Trivial(field_type) => {
                format!("&{lt}[{ty}]", lt = lt, ty = field_type.rust_type_name()?)
            }
        }
        .into())
    }

    pub fn bumpalo_getter_mut_type(&self, bump_lt: &str, this_lt: &str) -> Result<String> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            // ??? Maybe need to double check the lt params
            LengthDelimited(String) => format!(
                "impl {this_lt} + ::std::ops::DerefMut<Target=::puroro::bumpalo::collections::String<{bump_lt}>>",
                bump_lt = bump_lt,
                this_lt = this_lt,
            ),
            LengthDelimited(Bytes) => format!(
                "impl {this_lt} + ::std::ops::DerefMut<Target=::puroro::bumpalo::collections::Vec<{bump_lt}, u8>>",
                bump_lt = bump_lt,
                this_lt = this_lt,
            ),
            LengthDelimited(Message(m)) => {
                let msg_type = upgrade(&m)?.rust_impl_path("Bumpalo", &[bump_lt]);
                format!("&{lt} mut {msg}", lt = this_lt, msg = msg_type)
            }
            Trivial(field_type) => format!(
                "&{lt} mut {ty}",
                lt = this_lt,
                ty = field_type.rust_type_name()?
            ),
        })
    }

    pub fn bumpalo_getter_repeated_mut_type(&self, bump_lt: &str, this_lt: &str) -> Result<String> {
        use FieldTypeCategories::*;
        use LdFieldType::*;
        Ok(match self.field_type()?.categories()? {
            LengthDelimited(String) => format!(
                "::puroro::internal::AddBumpVecView<{bump_lt}, {this_lt}, ::puroro::internal::NoAllocBumpString>",
                bump_lt = bump_lt,
                this_lt = this_lt
            ),
            LengthDelimited(Bytes) => format!(
                "::puroro::internal::AddBumpVecView<{bump_lt}, {this_lt}, ::puroro::internal::NoAllocBumpVec<u8>>",
                bump_lt = bump_lt,
                this_lt = this_lt
            ),
            LengthDelimited(Message(m)) => format!(
                "::puroro::internal::RefMutBumpVec<{bump_lt}, {this_lt}, {ty}>",
                bump_lt = bump_lt,
                this_lt = this_lt,
                ty = upgrade(&m)?.rust_impl_path("Bumpalo", &[bump_lt])
            ),
            Trivial(field_type) => {
                format!(
                    "::puroro::internal::RefMutBumpVec<{bump_lt}, {this_lt}, {ty}>",
                    bump_lt = bump_lt,
                    this_lt = this_lt,
                    ty = field_type.rust_type_name()?
                )
            }
        })
    }

    pub fn single_field_type(&self) -> Result<String> {
        Ok(if self.is_repeated()? {
            "RepeatedType"
        } else {
            "ScalarType"
        }
        .to_string())
    }

    pub fn single_numerical_rust_type(&self) -> Result<Cow<'static, str>> {
        Ok(match self.field_type()?.categories()? {
            FieldTypeCategories::Trivial(field_type) => field_type.rust_type_name()?,
            _ => "".into(),
        })
    }
}

#[derive(Debug)]
pub struct Oneof {
    message: Weak<Message>,
    index: i32,
    rust_enum_ident: String,
    rust_getter_ident: String,
    lazy_fields: OnceCell<Vec<Rc<Field>>>,
    lazy_is_synthetic: OnceCell<bool>,
}

impl Oneof {
    pub fn try_from_proto(
        message: Weak<Message>,
        proto: &OneofDescriptorProto,
        index: i32,
    ) -> Result<Rc<Self>> {
        Ok(Rc::new(Self {
            message,
            index: index,
            rust_enum_ident: get_keyword_safe_ident(&to_camel_case(proto.name())).to_string(),
            rust_getter_ident: get_keyword_safe_ident(&to_lower_snake_case(proto.name()))
                .to_string(),
            lazy_fields: OnceCell::new(),
            lazy_is_synthetic: OnceCell::new(),
        }))
    }
    pub fn rust_enum_ident(&self) -> &str {
        &self.rust_enum_ident
    }
    pub fn rust_getter_ident(&self) -> &str {
        &self.rust_getter_ident
    }
    pub fn message(&self) -> Result<Rc<Message>> {
        upgrade(&self.message)
    }
    pub fn index(&self) -> i32 {
        self.index
    }
    pub fn is_synthetic(&self) -> Result<bool> {
        self.lazy_is_synthetic
            .get_or_try_init(|| {
                let fields = self.fields()?;
                Ok(if let Some(first) = fields.first() {
                    fields.len() == 1 && first.is_optional3()
                } else {
                    false
                })
            })
            .map(|b| *b)
    }
    pub fn fields(&self) -> Result<&[Rc<Field>]> {
        self.lazy_fields
            .get_or_try_init(|| {
                let message = upgrade(&self.message)?;
                Ok(message
                    .fields()
                    .into_iter()
                    .cloned()
                    .filter(|field| {
                        if let Some(oneof_index) = field.oneof_index() {
                            oneof_index == self.index
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>())
            })
            .map(|v| v.as_slice())
    }
    pub fn field_indices(&self) -> Result<Vec<usize>> {
        Ok(upgrade(&self.message)?
            .fields()
            .into_iter()
            .cloned()
            .enumerate()
            .filter_map(|(field_index, field)| {
                if let Some(oneof_index) = field.oneof_index() {
                    if oneof_index == self.index {
                        return Some(field_index);
                    }
                }
                None
            })
            .collect::<Vec<_>>())
    }
    pub fn trait_maybe_generic_params(&self, lt: &str, trait_impl: &str) -> Result<String> {
        let need_lt = self.fields()?.iter().any(|field| {
            matches!(
                field.field_type(),
                Ok(FieldType::Bytes | FieldType::String | FieldType::Message(_))
            )
        });
        let need_trait_impl = self
            .fields()?
            .iter()
            .any(|field| matches!(field.field_type(), Ok(FieldType::Message(_))));
        Ok(match (need_lt, need_trait_impl) {
            (true, true) => {
                let trait_path = self.message()?.rust_trait_path();
                format!("<{}, {}: {}>", lt, trait_impl, trait_path)
            }
            (true, false) => format!("<{}>", lt),
            _ => "".to_string(),
        })
    }
}

#[derive(Debug)]
pub struct EnumValue {
    rust_ident: String,
    number: i32,
}

impl EnumValue {
    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn number(&self) -> i32 {
        self.number
    }
}

#[derive(Debug, Clone)]
pub enum ProtoSyntax {
    Proto2,
    Proto3,
}

impl ProtoSyntax {
    pub fn tag_ident(&self) -> &str {
        match *self {
            ProtoSyntax::Proto2 => "Proto2",
            ProtoSyntax::Proto3 => "Proto3",
        }
    }
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
    Enum2(Weak<Enum>),
    Enum3(Weak<Enum>),
    Message(Weak<Message>),
}

impl FieldType {
    pub fn try_from_type_proto(
        message: Weak<Message>,
        proto_type_enum: &FieldTypeProto,
        proto_type_name: &str,
    ) -> Result<Self> {
        let input_file = upgrade(&message)?.input_file()?;
        let context = input_file.context()?;
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
                Some(MessageOrEnum::Enum(e)) => match input_file.syntax() {
                    ProtoSyntax::Proto2 => FieldType::Enum2(Rc::downgrade(e)),
                    ProtoSyntax::Proto3 => FieldType::Enum3(Rc::downgrade(e)),
                },
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

    pub fn tag_ident_and_gp<F: Fn(&Message) -> Result<String>>(
        &self,
        gen_msg_path: F,
    ) -> Result<String> {
        Ok(match self {
            FieldType::Double => "Double".to_string(),
            FieldType::Float => "Float".to_string(),
            FieldType::Int32 => "Int32".to_string(),
            FieldType::Int64 => "Int64".to_string(),
            FieldType::UInt32 => "UInt32".to_string(),
            FieldType::UInt64 => "UInt64".to_string(),
            FieldType::SInt32 => "SInt32".to_string(),
            FieldType::SInt64 => "SInt64".to_string(),
            FieldType::Fixed32 => "Fixed32".to_string(),
            FieldType::Fixed64 => "Fixed64".to_string(),
            FieldType::SFixed32 => "SFixed32".to_string(),
            FieldType::SFixed64 => "SFixed64".to_string(),
            FieldType::Bool => "Bool".to_string(),
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "String".to_string(),
            FieldType::Bytes => "Bytes".to_string(),
            FieldType::Enum2(e) => format!("Enum2<{}>", upgrade(e)?.rust_path()),
            FieldType::Enum3(e) => format!("Enum3<{}>", upgrade(e)?.rust_path()),
            FieldType::Message(m) => {
                format!(
                    "Message<{path}>",
                    path = (gen_msg_path)(upgrade(m)?.deref())?,
                )
            }
        })
    }

    pub fn categories(self) -> Result<FieldTypeCategories> {
        use FieldTypeCategories::*;
        Ok(match self {
            FieldType::Double => Trivial(TrivialFieldType::Double),
            FieldType::Float => Trivial(TrivialFieldType::Float),
            FieldType::Int32 => Trivial(TrivialFieldType::Int32),
            FieldType::Int64 => Trivial(TrivialFieldType::Int64),
            FieldType::UInt32 => Trivial(TrivialFieldType::UInt32),
            FieldType::UInt64 => Trivial(TrivialFieldType::UInt64),
            FieldType::SInt32 => Trivial(TrivialFieldType::SInt32),
            FieldType::SInt64 => Trivial(TrivialFieldType::SInt64),
            FieldType::Fixed32 => Trivial(TrivialFieldType::Fixed32),
            FieldType::Fixed64 => Trivial(TrivialFieldType::Fixed64),
            FieldType::SFixed32 => Trivial(TrivialFieldType::SFixed32),
            FieldType::SFixed64 => Trivial(TrivialFieldType::SFixed64),
            FieldType::Bool => Trivial(TrivialFieldType::Bool),
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => LengthDelimited(LdFieldType::String),
            FieldType::Bytes => LengthDelimited(LdFieldType::Bytes),
            FieldType::Enum2(e) => Trivial(TrivialFieldType::Enum2(e)),
            FieldType::Enum3(e) => Trivial(TrivialFieldType::Enum3(e)),
            FieldType::Message(m) => LengthDelimited(LdFieldType::Message(m)),
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
            FieldType::Enum2(e) => upgrade(e)?.proto_name().to_string(),
            FieldType::Enum3(e) => upgrade(e)?.proto_name().to_string(),
            FieldType::Message(m) => upgrade(m)?.proto_name().to_string(),
        })
    }
}

pub enum LdFieldType {
    String,
    Bytes,
    Message(Weak<Message>),
}

pub enum TrivialFieldType {
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
    Enum2(Weak<Enum>),
    Enum3(Weak<Enum>),
}

impl TrivialFieldType {
    pub fn rust_type_name(&self) -> Result<Cow<'static, str>> {
        Ok(match self {
            TrivialFieldType::Double => "f64".into(),
            TrivialFieldType::Float => "f32".into(),
            TrivialFieldType::Int32 => "i32".into(),
            TrivialFieldType::Int64 => "i64".into(),
            TrivialFieldType::UInt32 => "u32".into(),
            TrivialFieldType::UInt64 => "u64".into(),
            TrivialFieldType::SInt32 => "i32".into(),
            TrivialFieldType::SInt64 => "i64".into(),
            TrivialFieldType::Fixed32 => "u32".into(),
            TrivialFieldType::Fixed64 => "u64".into(),
            TrivialFieldType::SFixed32 => "i32".into(),
            TrivialFieldType::SFixed64 => "i64".into(),
            TrivialFieldType::Bool => "bool".into(),
            TrivialFieldType::Enum2(e) | TrivialFieldType::Enum3(e) => {
                upgrade(e)?.rust_path().into()
            }
        })
    }
}

pub enum FieldTypeCategories {
    LengthDelimited(LdFieldType),
    Trivial(TrivialFieldType),
}

#[derive(Debug, Clone)]
pub enum FieldLabel {
    Required,
    Optional,
    Unlabeled,
    Repeated,
    OneofField,
}

impl FieldLabel {
    pub fn tag_ident(&self) -> &str {
        match *self {
            FieldLabel::Required => "Required",
            FieldLabel::Optional => "Optional",
            FieldLabel::Unlabeled => "Unlabeled",
            FieldLabel::Repeated => "Repeated",
            FieldLabel::OneofField => "OneofField",
        }
    }
}

#[derive(Debug)]
pub enum MessageOrEnum {
    Message(Rc<Message>),
    Enum(Rc<Enum>),
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
        "self::_puroro_root::google::protobuf::compiler",
        make_module_path(package.clone(), empty.clone())
    );
    assert_eq!(
        "self::_puroro_root::_puroro_nested::code_generator_response",
        make_module_path(empty.clone(), outer_messages.clone())
    );
    assert_eq!(
        "self::_puroro_root::google::protobuf::compiler::_puroro_nested::code_generator_response",
        make_module_path(package.clone(), outer_messages.clone())
    );
}
