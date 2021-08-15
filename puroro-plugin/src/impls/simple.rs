use crate::utils::*;
use crate::wrappers::{FieldLabel, FieldType};
use crate::{wrappers, ErrorKind, Result};
use ::askama::Template;
use ::std::ops::Deref;
use ::std::rc::Rc;

#[derive(Template)]
#[template(path = "impl_simple.rs.txt")]
pub struct SimpleImpl {
    message: Rc<wrappers::Message>,
    rust_ident: String,
    fields: Vec<Field>,
}

#[derive(Template)]
#[template(path = "oneof_simple.rs.txt")]
pub struct SimpleOneof {
    oneof: Oneof,
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

impl SimpleOneof {
    pub fn try_new(oneof: &Rc<wrappers::Oneof>) -> Result<Self> {
        Ok(Self {
            oneof: Oneof::try_new(oneof.clone())?,
        })
    }
}

pub struct Field {
    field: Rc<wrappers::Field>,
}
impl Deref for Field {
    type Target = wrappers::Field;
    fn deref(&self) -> &Self::Target {
        &self.field
    }
}
impl Field {
    pub fn new(field: Rc<wrappers::Field>) -> Self {
        Self { field }
    }
    pub fn rust_field_type(&self, impl_tag: &'static str) -> Result<String> {
        let scalar_type = self.rust_scalar_field_type(impl_tag)?;
        Ok(match self.field.field_label()? {
            FieldLabel::Required | FieldLabel::Optional => {
                format!("::std::option::Option<{}>", scalar_type)
            }
            FieldLabel::Unlabeled => {
                if matches!(self.field.field_type(), Ok(FieldType::Message(_))) {
                    format!("::std::option::Option<{}>", scalar_type)
                } else {
                    scalar_type
                }
            }
            FieldLabel::Repeated => format!("::std::vec::Vec<{}>", scalar_type),
        })
    }

    pub fn rust_oneof_field_type(&self, impl_tag: &'static str) -> Result<String> {
        let scalar_type = self.rust_scalar_field_type(impl_tag)?;
        Ok(scalar_type)
    }

    fn rust_scalar_field_type(&self, impl_tag: &'static str) -> Result<String> {
        Ok(match self.field.field_type()? {
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "::std::string::String".to_string(),
            FieldType::Bytes => "::std::vec::Vec<u8>".to_string(),
            FieldType::Enum2(e) => upgrade(&e)?.rust_absolute_path(),
            FieldType::Enum3(e) => upgrade(&e)?.rust_absolute_path(),
            FieldType::Message(m) => {
                let bare_msg = format!(
                    "{path}<{tag}>",
                    path = upgrade(&m)?.rust_absolute_path(),
                    tag = impl_tag
                );
                if matches!(self.field.field_label(), Ok(FieldLabel::Repeated)) {
                    bare_msg
                } else {
                    format!("::std::boxed::Box<{}>", bare_msg)
                }
            }
            t => t.numerical_rust_type()?.to_string(),
        })
    }
}
pub struct Oneof {
    oneof: Rc<wrappers::Oneof>,
    fields: Vec<Field>,
}
impl Deref for Oneof {
    type Target = wrappers::Oneof;
    fn deref(&self) -> &Self::Target {
        &self.oneof
    }
}
impl Oneof {
    pub fn try_new(oneof: Rc<wrappers::Oneof>) -> Result<Self> {
        Ok(Self {
            oneof: oneof.clone(),
            fields: oneof
                .fields()?
                .iter()
                .map(|f| Field::new(f.clone()))
                .collect(),
        })
    }
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}
