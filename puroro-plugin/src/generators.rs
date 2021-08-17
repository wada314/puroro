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
}

struct Enum {
    values: Vec<Rc<EnumValue>>,
}

struct EnumValue {
    ident: String,
    number: i32,
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

struct Oneof {
    enum_ident: String,
    getter_ident: String,
    fields: Vec<Rc<Field>>,
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
