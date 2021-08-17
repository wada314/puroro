use crate::wrappers;
use crate::{ErrorKind, Result};
use ::askama::Template;
use ::std::rc::Rc;

#[derive(Template)]
#[template(path = "output_file.rs.txt")]
pub struct OutputFile {
    pub package: String,
    pub subpackages: Vec<String>,
    pub input_file: Option<Rc<InputFile>>,
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
pub struct InputFile {
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
    package: String,
}

impl InputFile {
    pub fn try_new(f: &wrappers::InputFile) -> Result<Self> {
        Ok(Self {
            messages: f
                .messages()
                .into_iter()
                .map(|m| Ok(Rc::new(Message::try_new(m)?)))
                .collect::<Result<Vec<_>>>()?,
            enums: f
                .enums()
                .into_iter()
                .map(|e| Ok(Rc::new(Enum::try_new(e)?)))
                .collect::<Result<Vec<_>>>()?,
            package: f.package().join("."),
        })
    }
}

struct Message {
    nested_messages: Vec<Rc<Message>>,
    nested_enums: Vec<Rc<Enum>>,
    fields: Vec<Rc<Field>>,
    oneofs: Vec<Rc<Oneof>>,
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
        Ok(Self {
            nested_messages: m
                .nested_messages()
                .into_iter()
                .map(|m| -> Result<_> { Ok(Rc::new(Message::try_new(m)?)) })
                .collect::<Result<Vec<_>>>()?,
            nested_enums: m
                .nested_enums()
                .into_iter()
                .map(|e| -> Result<_> { Ok(Rc::new(Enum::try_new(e)?)) })
                .collect::<Result<Vec<_>>>()?,
            fields,
            oneofs,
        })
    }
}

#[derive(Template)]
#[template(path = "enum.rs.txt")]
struct Enum {
    ident: String,
    absolute_path: String,
    values: Vec<Rc<EnumValue>>,
    first_value: Rc<EnumValue>,
    is_proto3: bool,
}

impl Enum {
    fn try_new(e: &wrappers::Enum) -> Result<Self> {
        let values = e
            .values()
            .into_iter()
            .map(|v| -> Result<_> { Ok(Rc::new(EnumValue::try_new(v)?)) })
            .collect::<Result<Vec<_>>>()?;
        let first_value = values
            .first()
            .ok_or(ErrorKind::EmptyEnum {
                name: e.proto_name().to_string(),
            })?
            .clone();
        Ok(Self {
            ident: e.rust_ident().to_string(),
            absolute_path: e.rust_absolute_path(),
            values,
            first_value,
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
    trait_has_scalar_getter: bool,
    trait_has_optional_getter: bool,
    trait_has_repeated_getter: bool,
}

impl Field {
    fn try_new(f: &wrappers::Field) -> Result<Self> {
        Ok(Field {
            ident: f.rust_ident().to_string(),
            oneof_ident: f.rust_oneof_ident().to_string(),
            number: f.number(),
            is_message: f.field_type()?.is_message(),
            trait_has_scalar_getter: f.has_scalar_getter(),
            trait_has_optional_getter: f.has_scalar_optional_getter(),
            trait_has_repeated_getter: f.has_repeated_getter(),
        })
    }
}

struct Oneof {
    enum_ident: String,
    getter_ident: String,
    fields: Vec<Rc<Field>>,
}

impl Oneof {
    fn try_new(o: &wrappers::Oneof, fields: &[Rc<Field>]) -> Result<Self> {
        Ok(Oneof {
            enum_ident: o.rust_enum_ident().to_string(),
            getter_ident: o.rust_getter_ident().to_string(),
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
