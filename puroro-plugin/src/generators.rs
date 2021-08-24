use crate::utils::upgrade;
use crate::wrappers;
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
    trait_ident: String,
    submodule_ident: String,
    nested: MessagesAndEnums,
    fields: Vec<Field>,
    oneofs: Vec<Oneof>,
    simple_ident: String,
    empty_ident: String,
    merged_ident: String,
    simple_single_field_ident: String,
}

impl Message {
    fn try_new(m: &wrappers::Message) -> Result<Self> {
        let fields = m
            .fields()
            .into_iter()
            .map(|f| Field::try_new(f))
            .try_collect()?;
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
            trait_ident: m.rust_trait_ident().to_string(),
            submodule_ident: m.rust_nested_module_ident().to_string(),
            nested: MessagesAndEnums {
                messages: nested_messages,
                enums: nested_enums,
            },
            fields,
            oneofs,
            simple_ident: m.rust_impl_ident("Simple"),
            empty_ident: m.rust_impl_ident("Empty"),
            merged_ident: m.rust_impl_ident("Merged"),
            simple_single_field_ident: m.rust_impl_ident("SimpleField"),
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
    number: i32,
    is_message: bool,
    is_length_delimited: bool,
    is_explicit_oneof_field: bool,
    trait_has_scalar_getter: bool,
    trait_has_optional_getter: bool,
    trait_has_repeated_getter: bool,
    trait_scalar_getter_type: String,
    trait_maybe_field_message_trait_path: Option<String>,
    simple_field_type: String,
    simple_maybe_field_message_path: Option<String>,
    simple_maybe_borrowed_field_type: Option<String>,
    simple_label_and_type_tags: String,
    empty_maybe_field_message_path: Option<String>,
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
        let empty_maybe_field_message_path =
            if let wrappers::FieldType::Message(m) = f.field_type()? {
                Some(upgrade(&m)?.rust_impl_path("Empty"))
            } else {
                None
            };
        Ok(Field {
            ident: f.rust_ident().to_string(),
            number: f.number(),
            is_message: matches!(f.field_type()?, wrappers::FieldType::Message(_)),
            is_length_delimited: matches!(
                f.field_type()?,
                wrappers::FieldType::Bytes
                    | wrappers::FieldType::String
                    | wrappers::FieldType::Message(_)
            ),
            is_explicit_oneof_field: f.oneof_index().is_some() && !f.is_optional3(),
            trait_has_scalar_getter: f.has_scalar_getter()?,
            trait_has_optional_getter: f.has_scalar_optional_getter()?,
            trait_has_repeated_getter: f.has_repeated_getter()?,
            trait_scalar_getter_type: f.trait_scalar_getter_type()?,
            trait_maybe_field_message_trait_path,
            simple_field_type: f.simple_field_type()?,
            simple_maybe_field_message_path,
            simple_maybe_borrowed_field_type: f
                .maybe_trait_scalar_getter_type_borrowed("Simple")?,
            simple_label_and_type_tags: f.rust_label_and_type_tags("Simple")?,
            empty_maybe_field_message_path,
        })
    }
}

#[derive(Template)]
#[template(path = "oneof.rs.txt")]
struct Oneof {
    enum_ident: String,
    field_ident: String,
    fields: Vec<OneofField>,
    is_synthetic: bool,
    has_message_field: bool,
    has_reference_field: bool,
    owner_message_trait_path: String,
    simple_owner_message_path: String,
}

impl Oneof {
    fn try_new(o: &wrappers::Oneof) -> Result<Self> {
        Ok(Oneof {
            enum_ident: o.rust_enum_ident().to_string(),
            field_ident: o.rust_getter_ident().to_string(),
            fields: o
                .fields()?
                .into_iter()
                .map(|f| OneofField::try_new(f))
                .try_collect()?,
            is_synthetic: o.is_synthetic()?,
            has_message_field: o
                .fields()?
                .into_iter()
                .any(|f| matches!(f.field_type(), Ok(wrappers::FieldType::Message(_)))),
            has_reference_field: o.fields()?.into_iter().any(|f| {
                matches!(
                    f.field_type(),
                    Ok(wrappers::FieldType::Bytes
                        | wrappers::FieldType::String
                        | wrappers::FieldType::Message(_))
                )
            }),
            owner_message_trait_path: o.message()?.rust_trait_path(),
            simple_owner_message_path: o.message()?.rust_impl_path("Simple"),
        })
    }
}

struct OneofField {
    ident: String,
    number: i32,
    is_length_delimited: bool,
    trait_field_type: String,
    simple_field_type: String,
    simple_field_type_tag: String,
}

impl OneofField {
    fn try_new(f: &wrappers::Field) -> Result<Self> {
        Ok(Self {
            ident: f.rust_oneof_ident().to_string(),
            number: f.number(),
            is_length_delimited: matches!(
                f.field_type()?,
                wrappers::FieldType::Bytes
                    | wrappers::FieldType::String
                    | wrappers::FieldType::Message(_)
            ),
            trait_field_type: f.trait_oneof_field_type("'msg", "T")?,
            simple_field_type: f.simple_oneof_field_type()?,
            simple_field_type_tag: f.rust_type_tag("Simple")?,
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
