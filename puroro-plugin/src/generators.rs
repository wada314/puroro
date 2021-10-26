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

use crate::utils::{get_keyword_safe_ident, to_camel_case, upgrade};
use crate::wrappers::{self, FieldType};
use crate::{ErrorKind, Result};
use ::askama::Template;
use ::itertools::Itertools;

#[derive(Template)]
#[template(path = "output_file.rs.txt")]
pub struct OutputFile {
    pub package: String,
    pub subpackages: Vec<String>,
    pub input_file: Option<MessagesAndEnums>,
}

impl OutputFile {
    pub fn new(package: &str) -> Self {
        Self {
            package: package.to_string(),
            subpackages: Vec::new(),
            input_file: None,
        }
    }
}

#[derive(Template)]
#[template(path = "messages_and_enums.rs.txt")]
pub struct MessagesAndEnums {
    messages: Vec<Message>,
    enums: Vec<Enum>,
}

impl MessagesAndEnums {
    pub fn try_new(f: &wrappers::InputFile) -> Result<Self> {
        let messages = f
            .messages()
            .into_iter()
            .map(|m| Message::try_new(m))
            .try_collect()?;
        let enums = f
            .enums()
            .into_iter()
            .map(|e| Enum::try_new(e))
            .try_collect()?;
        Ok(Self { messages, enums })
    }
}

struct Message {
    ident: String,
    proto_name: String,
    trait_ident: String,
    trait_delegate_macro_ident: String,
    trait_generic_ser_function_ident: String,
    submodule_ident: String,
    nested: MessagesAndEnums,
    fields: Vec<Field>,
    fields_len: usize,
    oneofs: Vec<Oneof>,
    simple_ident: String,
    single_field_ident: String,
    builder_ident: String,
}

impl Message {
    fn try_new(m: &wrappers::Message) -> Result<Self> {
        let fields: Vec<Field> = m
            .fields()
            .into_iter()
            .map(|f| Field::try_new(f))
            .try_collect()?;
        let fields_len = fields.len();
        let oneofs = m
            .oneofs()
            .into_iter()
            .filter(|o| matches!(o.is_synthetic(), Ok(false)))
            .map(|o| Oneof::try_new(o))
            .try_collect()?;
        let nested_messages = m
            .nested_messages()
            .into_iter()
            .map(|m| Message::try_new(m))
            .try_collect()?;
        let nested_enums = m
            .nested_enums()
            .into_iter()
            .map(|e| Enum::try_new(e))
            .try_collect()?;
        Ok(Self {
            ident: m.rust_ident().to_string(),
            proto_name: m.proto_name().to_string(),
            trait_ident: m.rust_trait_ident().to_string(),
            trait_delegate_macro_ident: format!("{}_delegate", m.rust_nested_module_ident()),
            trait_generic_ser_function_ident: format!("{}_ser", m.rust_nested_module_ident()),
            submodule_ident: m.rust_nested_module_ident().to_string(),
            nested: MessagesAndEnums {
                messages: nested_messages,
                enums: nested_enums,
            },
            fields,
            fields_len,
            oneofs,
            simple_ident: m.rust_impl_ident(""),
            single_field_ident: m.rust_impl_ident("SingleField"),
            builder_ident: m.rust_impl_ident("Builder"),
        })
    }
}

#[derive(Template)]
#[template(path = "enum.rs.txt")]
struct Enum {
    ident: String,
    values: Vec<EnumValue>,
    first_value_ident: String,
    is_proto3: bool,
}

impl Enum {
    fn try_new(e: &wrappers::Enum) -> Result<Self> {
        let values = e
            .values()
            .into_iter()
            .map(|v| -> Result<_> { Ok(EnumValue::try_new(v)?) })
            .try_collect()?;
        let first_value_ident = e
            .values()
            .first()
            .ok_or(ErrorKind::EmptyEnum {
                name: e.proto_name().to_string(),
            })?
            .rust_ident()
            .to_string();
        Ok(Self {
            ident: e.rust_ident().to_string(),
            values,
            first_value_ident,
            is_proto3: matches!(e.syntax()?, wrappers::ProtoSyntax::Proto3),
        })
    }
}

struct EnumValue {
    ident: String,
    number: i32,
}

impl EnumValue {
    fn try_new(v: &wrappers::EnumValue) -> Result<Self> {
        Ok(Self {
            ident: v.rust_ident().to_string(),
            number: v.number(),
        })
    }
}

struct Field {
    ident: String,
    ident_unesc: String,
    proto_name: String,
    number: i32,
    oneof_index: i32,
    is_message: bool,
    is_string: bool,
    is_bytes: bool,
    is_length_delimited: bool,
    is_explicit_oneof_field: bool,
    is_repeated: bool,
    is_unlabeled: bool,
    has_default_value: bool,
    default_value: String,
    trait_scalar_getter_type: String,
    trait_maybe_field_message_trait_path: Option<String>,
    trait_label_and_type_tags: String,
    oneof_enum_value_ident: String,
    simple_field_type: String,
    simple_scalar_field_type: String,
    simple_maybe_field_message_path: Option<String>,
    simple_maybe_borrowed_field_type: Option<String>,
    simple_label_and_type_tags: String,
    single_field_type: String,
    single_numerical_rust_type: String,
    single_field_label_and_type_tags: String,
}

impl Field {
    fn try_new(f: &wrappers::Field) -> Result<Self> {
        let trait_maybe_field_message_trait_path =
            if let wrappers::FieldType::Message(m) = f.field_type()? {
                Some(upgrade(&m)?.rust_trait_path())
            } else {
                None
            };
        let simple_maybe_field_message_path =
            if let wrappers::FieldType::Message(m) = f.field_type()? {
                Some(upgrade(&m)?.rust_impl_path("Simple"))
            } else {
                None
            };

        Ok(Field {
            ident: f.rust_ident().to_string(),
            ident_unesc: f.rust_ident_unesc().to_string(),
            proto_name: f.proto_name().to_string(),
            number: f.number(),
            oneof_index: f.oneof_index().unwrap_or(-1),
            is_message: matches!(f.field_type()?, wrappers::FieldType::Message(_)),
            is_string: matches!(f.field_type()?, wrappers::FieldType::String),
            is_bytes: matches!(f.field_type()?, wrappers::FieldType::Bytes),
            is_length_delimited: matches!(
                f.field_type()?,
                wrappers::FieldType::Bytes
                    | wrappers::FieldType::String
                    | wrappers::FieldType::Message(_)
            ),
            is_explicit_oneof_field: f.oneof_index().is_some() && !f.is_optional3(),
            is_repeated: matches!(f.field_label()?, wrappers::FieldLabel::Repeated),
            is_unlabeled: matches!(f.field_label()?, wrappers::FieldLabel::Unlabeled),
            has_default_value: f.default_value().is_some(),
            default_value: f
                .default_value()
                .map(|v| -> Result<_> { Ok(Self::convert_default_value(v, f.field_type()?)?) })
                .transpose()?
                .unwrap_or(Default::default()),
            trait_scalar_getter_type: f.trait_scalar_getter_type()?,
            trait_maybe_field_message_trait_path,
            trait_label_and_type_tags: f.rust_label_and_type_tags(|_| {
                Ok(format!("Self::Field{}MessageType", f.number()))
            })?,
            oneof_enum_value_ident: f.rust_oneof_ident().to_string(),
            simple_field_type: f.simple_field_type()?,
            simple_scalar_field_type: f.simple_scalar_field_type()?,
            simple_maybe_field_message_path,
            simple_maybe_borrowed_field_type: f
                .maybe_trait_scalar_getter_type_borrowed("Simple")?,
            simple_label_and_type_tags: f.rust_label_and_type_tags(|msg| {
                Ok(
                    if matches!(f.field_label()?, wrappers::FieldLabel::Repeated) {
                        msg.rust_impl_path("Simple")
                    } else {
                        format!("::std::boxed::Box<{}>", msg.rust_impl_path("Simple"))
                    },
                )
            })?,
            single_field_type: f.single_field_type()?,
            single_numerical_rust_type: f.single_numerical_rust_type().unwrap_or("".to_string()),
            single_field_label_and_type_tags: f
                .rust_label_and_type_tags(|_| Ok("ScalarType".to_string()))?,
        })
    }

    fn convert_default_value(input: &str, field_type: FieldType) -> Result<String> {
        Ok(match field_type {
            FieldType::Float => match input {
                "inf" => "f32::INFINITY".to_string(),
                "-inf" => "f32::NEG_INFINITY".to_string(),
                "nan" | "-nan" => "f32::NAN".to_string(),
                digits => {
                    let append_zero = if digits.ends_with('.') { "0" } else { "" };
                    format!("{}{}f32", digits, append_zero)
                }
            },
            FieldType::Double => match input {
                "inf" => "f64::INFINITY".to_string(),
                "-inf" => "f64::NEG_INFINITY".to_string(),
                "nan" | "-nan" => "f64::NAN".to_string(),
                digits => {
                    let append_zero = if digits.ends_with('.') { "0" } else { "" };
                    format!("{}{}f64", digits, append_zero)
                }
            },
            FieldType::Int32
            | FieldType::Int64
            | FieldType::UInt32
            | FieldType::UInt64
            | FieldType::SInt32
            | FieldType::SInt64
            | FieldType::Fixed32
            | FieldType::Fixed64
            | FieldType::SFixed32
            | FieldType::SFixed64 => {
                // As-is is okay. Even if the input is octal of hexadecimal format,
                // the protoc command automatically converts it to decimal format
                // so we only need to treat a decimal format.
                input.to_string()
            }
            FieldType::Bool => {
                // the possible input is "true" or "false", so as-is is okay.
                input.to_string()
            }
            FieldType::String => {
                format!(r###""{}""###, input.escape_default().collect::<String>())
            }
            FieldType::Bytes => {
                // protoc escapes 0x7F~0xFF character as octal escape "\234".
                // Rust does not support that style so we need to re-encode it.
                let mut decoded = Vec::new();
                let mut bytes = input.bytes();
                loop {
                    if let Some(c) = bytes.next() {
                        if c == b'\\' {
                            if let Some(d) = bytes.next() {
                                match d {
                                    b'\\' | b'\"' | b'\'' => {
                                        decoded.push(d);
                                    }
                                    b'r' => decoded.push(b'\r'),
                                    b'n' => decoded.push(b'\n'),
                                    b't' => decoded.push(b'\t'),
                                    _ => {
                                        let e_opt = bytes.next();
                                        let f_opt = bytes.next();
                                        match (d, e_opt, f_opt) {
                                            (
                                                (b'0'..=b'9'),
                                                Some(e @ (b'0'..=b'9')),
                                                Some(f @ (b'0'..=b'9')),
                                            ) => {
                                                let u8_value = u8::from_str_radix(
                                                    &format!(
                                                        "{}{}{}",
                                                        d - b'0',
                                                        e - b'0',
                                                        f - b'0'
                                                    ),
                                                    8,
                                                )
                                                .map_err(|e| ErrorKind::ParseIntError {
                                                    source: e,
                                                })?;
                                                decoded.push(u8_value);
                                            }
                                            _ => Err(ErrorKind::InvalidString {
                                                string: input.to_string(),
                                            })?,
                                        }
                                    }
                                }
                            } else {
                                Err(ErrorKind::InvalidString {
                                    string: input.to_string(),
                                })?
                            }
                        } else {
                            decoded.push(c);
                        }
                    } else {
                        break;
                    }
                }
                let reencoded = decoded
                    .into_iter()
                    .map(|b| format!(r"\x{:02x}", b))
                    .collect::<String>();
                format!(r#"b"{}""#, reencoded)
            }
            FieldType::Enum2(e) | FieldType::Enum3(e) => {
                format!(
                    "{}::{}",
                    upgrade(&e)?.rust_path(),
                    get_keyword_safe_ident(&to_camel_case(input))
                )
            }
            FieldType::Message(_) | FieldType::Group => {
                unreachable!("Message and Group should not have default values")
            }
        })
    }
}

#[derive(Template)]
#[template(path = "oneof.rs.txt")]
struct Oneof {
    index: i32,
    enum_ident: String,
    field_ident: String,
    fields: Vec<OneofField>,
    has_ld_field: bool,
    has_message_field: bool,
    owner_message_trait_path: String,
}

impl Oneof {
    fn try_new(o: &wrappers::Oneof) -> Result<Self> {
        Ok(Oneof {
            index: o.index(),
            enum_ident: o.rust_enum_ident().to_string(),
            field_ident: o.rust_getter_ident().to_string(),
            fields: o
                .fields()?
                .into_iter()
                .map(|f| OneofField::try_new(f))
                .try_collect()?,
            has_ld_field: o.fields()?.into_iter().any(|f| {
                matches!(
                    f.field_type(),
                    Ok(wrappers::FieldType::Bytes
                        | wrappers::FieldType::String
                        | wrappers::FieldType::Message(_))
                )
            }),
            has_message_field: o
                .fields()?
                .into_iter()
                .any(|f| matches!(f.field_type(), Ok(wrappers::FieldType::Message(_)))),
            owner_message_trait_path: o.message()?.rust_trait_path(),
        })
    }
}

struct OneofField {
    ident: String,
    getter_ident: String,
    getter_ident_unesc: String,
    number: i32,
    is_length_delimited: bool,
    is_message: bool,
    field_type: String,
    trait_getter_type: String,
    simple_field_type_tag: String,
}

impl OneofField {
    fn try_new(f: &wrappers::Field) -> Result<Self> {
        Ok(Self {
            ident: f.rust_oneof_ident().to_string(),
            getter_ident: f.rust_ident().to_string(),
            getter_ident_unesc: f.rust_ident_unesc().to_string(),
            number: f.number(),
            is_length_delimited: matches!(
                f.field_type()?,
                wrappers::FieldType::Bytes
                    | wrappers::FieldType::String
                    | wrappers::FieldType::Message(_)
            ),
            is_message: matches!(f.field_type()?, wrappers::FieldType::Message(_)),
            field_type: f.oneof_field_type()?,
            trait_getter_type: f.trait_oneof_field_type("'this", "Self")?,
            simple_field_type_tag: f.rust_type_tag(|msg| {
                Ok(
                    if matches!(f.field_label()?, wrappers::FieldLabel::Repeated) {
                        msg.rust_impl_path("Simple")
                    } else {
                        format!("::std::boxed::Box<{}>", msg.rust_impl_path("Simple"))
                    },
                )
            })?,
        })
    }
}

#[derive(Template)]
#[template(path = "structs.rs.txt")]
struct Structs<'a> {
    messages: &'a [Message],
}

#[derive(Template)]
#[template(path = "traits.rs.txt")]
struct Traits<'a> {
    messages: &'a [Message],
}

mod filters {
    use super::*;
    pub(super) fn print_structs(messages: &[Message]) -> ::askama::Result<Structs> {
        Ok(Structs { messages })
    }
    pub(super) fn print_traits(messages: &[Message]) -> ::askama::Result<Traits> {
        Ok(Traits { messages })
    }
}
