use crate::utils::upgrade;
use crate::wrappers;
use crate::{ErrorKind, Result};
use ::askama::Template;
use ::std::collections::HashMap;
use ::std::rc::Rc;

#[derive(Default)]
pub struct GlobalContext {
    wrappers_message_to_generators_message: HashMap<Rc<wrappers::Message>, Rc<Message>>,
}
impl GlobalContext {
    fn get_message(&mut self, wrappers_message: Rc<wrappers::Message>) -> Rc<Message> {
        todo!()
    }
}

#[derive(Template)]
#[template(path = "output_file.rs.txt")]
pub struct OutputFile {
    pub package: String,
    pub subpackages: Vec<String>,
    pub input_file: Option<Rc<MessagesAndEnums>>,
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
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
}

impl MessagesAndEnums {
    pub fn try_new(f: &wrappers::InputFile) -> Result<Self> {
        let messages = f
            .messages()
            .into_iter()
            .map(|m| Ok(Rc::new(Message::try_new(m)?)))
            .collect::<Result<Vec<_>>>()?;
        let enums = f
            .enums()
            .into_iter()
            .map(|e| Ok(Rc::new(Enum::try_new(e)?)))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { messages, enums })
    }
}

struct Message {
    ident: String,
    submodule_ident: String,
    nested: Rc<MessagesAndEnums>,
    fields: Vec<Rc<Field>>,
    oneofs: Vec<Rc<Oneof>>,

    trait_absolute_path: String,
    simple_ident: String,
}

impl Message {
    fn try_new(m: &wrappers::Message) -> Result<Self> {
        let fields = m
            .fields()
            .into_iter()
            .map(|f| Ok(Rc::new(Field::try_new(f)?)))
            .collect::<Result<Vec<_>>>()?;
        let oneofs = m
            .oneofs()
            .into_iter()
            .map(|o| Ok(Rc::new(Oneof::try_new(o, &fields)?)))
            .collect::<Result<Vec<_>>>()?;
        let nested_messages = m
            .nested_messages()
            .into_iter()
            .map(|m| -> Result<_> { Ok(Rc::new(Message::try_new(m)?)) })
            .collect::<Result<Vec<_>>>()?;
        let nested_enums = m
            .nested_enums()
            .into_iter()
            .map(|e| -> Result<_> { Ok(Rc::new(Enum::try_new(e)?)) })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            ident: m.rust_ident().to_string(),
            submodule_ident: m.rust_nested_module_ident().to_string(),
            nested: Rc::new(MessagesAndEnums {
                messages: nested_messages,
                enums: nested_enums,
            }),
            fields,
            oneofs,
            trait_absolute_path: m.rust_absolute_trait_path(),
            simple_ident: format!("{}_Simple", m.rust_ident()),
        })
    }
}

#[derive(Template)]
#[template(path = "enum.rs.txt")]
struct Enum {
    ident: String,
    absolute_path: String,
    values: Vec<Rc<EnumValue>>,
    first_value_ident: String,
    is_proto3: bool,
}

impl Enum {
    fn try_new(e: &wrappers::Enum) -> Result<Self> {
        let values = e
            .values()
            .into_iter()
            .map(|v| -> Result<_> { Ok(Rc::new(EnumValue::try_new(v)?)) })
            .collect::<Result<Vec<_>>>()?;
        let first_value_ident = values
            .first()
            .ok_or(ErrorKind::EmptyEnum {
                name: e.proto_name().to_string(),
            })?
            .ident
            .clone();
        Ok(Self {
            ident: e.rust_ident().to_string(),
            absolute_path: e.rust_absolute_path(),
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
    oneof_ident: String,
    number: i32,
    is_message: bool,
    is_length_delimited: bool,
    trait_has_scalar_getter: bool,
    trait_has_optional_getter: bool,
    trait_has_repeated_getter: bool,
    trait_scalar_getter_type: String,
    trait_maybe_field_message_absolute_path: Option<String>,
    trait_maybe_field_message_trait_absolute_path: Option<String>,
    simple_field_type: String,
    simple_maybe_borrowed_field_type: Option<String>,
    simple_label_and_type_tags: String,
}

impl Field {
    fn try_new(f: &wrappers::Field) -> Result<Self> {
        let trait_maybe_field_message_absolute_path =
            if let wrappers::FieldType::Message(m) = f.field_type()? {
                Some(upgrade(&m)?.rust_absolute_path())
            } else {
                None
            };
        let trait_maybe_field_message_trait_absolute_path =
            if let wrappers::FieldType::Message(m) = f.field_type()? {
                Some(upgrade(&m)?.rust_absolute_trait_path())
            } else {
                None
            };
        Ok(Field {
            ident: f.rust_ident().to_string(),
            oneof_ident: f.rust_oneof_ident().to_string(),
            number: f.number(),
            is_message: matches!(f.field_type()?, wrappers::FieldType::Message(_)),
            is_length_delimited: matches!(
                f.field_type()?,
                wrappers::FieldType::Bytes
                    | wrappers::FieldType::String
                    | wrappers::FieldType::Message(_)
            ),
            trait_has_scalar_getter: f.has_scalar_getter(),
            trait_has_optional_getter: f.has_scalar_optional_getter(),
            trait_has_repeated_getter: f.has_repeated_getter(),
            trait_scalar_getter_type: f.trait_scalar_getter_type()?,
            trait_maybe_field_message_absolute_path,
            trait_maybe_field_message_trait_absolute_path,
            simple_field_type: f.simple_field_type()?,
            simple_maybe_borrowed_field_type: f
                .maybe_trait_scalar_getter_type_borrowed("::puroro::tags::SimpleImpl")?,
            simple_label_and_type_tags: f
                .rust_label_and_type_tags("::puroro::tags::", "SimpleImpl")?,
        })
    }
}

#[derive(Template)]
#[template(path = "oneof.rs.txt")]
struct Oneof {
    enum_ident: String,
    field_ident: String,
    fields: Vec<Rc<Field>>,
}

impl Oneof {
    fn try_new(o: &wrappers::Oneof, fields: &[Rc<Field>]) -> Result<Self> {
        Ok(Oneof {
            enum_ident: o.rust_enum_ident().to_string(),
            field_ident: o.rust_getter_ident().to_string(),
            fields: o
                .field_indices()?
                .into_iter()
                .map(|index| -> Result<_> {
                    Ok(fields
                        .get(index)
                        .ok_or(ErrorKind::InternalError {
                            detail: "index out of bounds".to_string(),
                        })?
                        .clone())
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[derive(Template)]
#[template(path = "structs.rs.txt")]
struct Structs {
    messages: Vec<Rc<Message>>,
}

#[derive(Template)]
#[template(path = "traits.rs.txt")]
struct Traits {
    messages: Vec<Rc<Message>>,
}

mod filters {
    use super::*;
    pub(super) fn print_structs(messages: &[Rc<Message>]) -> ::askama::Result<Structs> {
        Ok(Structs {
            messages: messages.to_vec(),
        })
    }
    pub(super) fn print_traits(messages: &[Rc<Message>]) -> ::askama::Result<Traits> {
        Ok(Traits {
            messages: messages.to_vec(),
        })
    }
}
