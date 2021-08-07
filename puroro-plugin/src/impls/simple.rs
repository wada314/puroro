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
impl Deref for Field {
    type Target = wrappers::Field;
    fn deref(&self) -> &Self::Target {
        &self.field
    }
}
impl Field {
    pub fn rust_field_type(&self, impl_tag: &'static str) -> Result<String> {
        let scalar_type = match self.field.field_type()? {
            FieldType::Group => Err(ErrorKind::GroupNotSupported)?,
            FieldType::String => "::std::string::String".to_string(),
            FieldType::Bytes => "::std::vec::Vec<u8>".to_string(),
            FieldType::Enum(e) => {
                let bare_enum = upgrade(&e)?.rust_absolute_path();
                match self.field.message()?.input_file()?.syntax() {
                    wrappers::ProtoSyntax::Proto2 => bare_enum,
                    wrappers::ProtoSyntax::Proto3 => {
                        format!("::std::result::Result<{}, i32>", bare_enum)
                    }
                }
            }
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
        };
        Ok(match self.field.field_label()? {
            FieldLabel::Required | FieldLabel::Optional => {
                format!("::std::option::Option<{}>", scalar_type)
            }
            FieldLabel::Unlabeled => scalar_type,
            FieldLabel::Repeated => format!("::std::vec::Vec<{}>", scalar_type),
        })
    }
}
