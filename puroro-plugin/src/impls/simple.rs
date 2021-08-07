use crate::{wrappers, ErrorKind, Result};
use ::askama::Template;
use ::std::rc::Rc;

#[derive(Template)]
#[template(path = "impl_simple.rs.txt")]
pub struct SimpleImpl {
    message: Rc<wrappers::Message>,
    rust_ident: String,
    fields: Vec<Field>,
}

impl SimpleImpl {
    pub fn new(message: &Rc<wrappers::Message>) -> Self {
        Self {
            message: message.clone(),
            rust_ident: format!("{}_SimpleImpl", message.rust_ident()),
            fields: message
                .fields()
                .into_iter()
                .map(|x| Field { field: x.clone() })
                .collect(),
        }
    }
    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

pub struct Field {
    field: Rc<wrappers::Field>,
}
impl Field {
    pub fn rust_ident(&self) -> &str {
        self.field.rust_ident()
    }
    pub fn rust_field_type(&self) -> Result<String> {
        let scalar_type = match self.field.field_type()? {
            wrappers::FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            wrappers::FieldType::String => todo!(),
            wrappers::FieldType::Bytes => todo!(),
            wrappers::FieldType::Enum(_) => todo!(),
            wrappers::FieldType::Message(_) => todo!(),
            t => t.numerical_rust_type()?,
        };
    }
}
