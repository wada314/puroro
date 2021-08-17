use crate::wrappers;
use crate::{ErrorKind, Result};
use ::std::rc::Rc;

struct InputFile {
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
}

struct Message {
    nested_messages: Vec<Rc<Message>>,
    nested_enums: Vec<Rc<Enum>>,
    fields: Vec<Rc<Field>>,
    oneofs: Vec<Rc<Oneof>>,
}

struct Enum {
    ident: String,
    absolute_path: String,
    values: Vec<Rc<EnumValue>>,
    first_value: Rc<EnumValue>,
}

impl Enum {
    fn try_new(e: &wrappers::Enum) -> Result<Self> {
        let values = e
            .values()
            .into_iter()
            .map(|v| -> Result<_> { Rc::new(EnumValue::try_new(v)) })
            .collect::<Result<Vec<_>>>()?;
        let first_value = values
            .first()
            .ok_or(ErrorKind::EmptyEnum {
                name: e.proto_name(),
            })?
            .clone();
        Ok(Self {
            ident: e.rust_ident(),
            absolute_path: e.rust_absolute_path(),
            values,
            first_value,
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
            ident: v.rust_ident(),
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
