#![cfg(any())]

use crate::utils::*;
use crate::wrappers::{FieldLabel, FieldType};
use crate::{wrappers, ErrorKind, Result};
use askama::Template;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Template)]
#[template(path = "impl_simple.rs.txt")]
pub struct SimpleImpl {
    message: Rc<wrappers::Message>,
    rust_ident: String,
    fields: Vec<Field>,
    oneofs: Vec<Oneof>,
}

#[derive(Template)]
#[template(path = "oneof_simple.rs.txt")]
pub struct SimpleOneof {
    oneof: Oneof,
}

impl SimpleImpl {
    pub fn try_new(message: &Rc<wrappers::Message>) -> Result<Self> {
        Ok(Self {
            message: message.clone(),
            rust_ident: format!("{}_SimpleImpl", message.rust_ident()),
            fields: message
                .fields()
                .into_iter()
                .map(|x| Field { field: x.clone() })
                .collect(),
            oneofs: message
                .oneofs()
                .into_iter()
                .map(|o| Oneof::try_new(o.clone()))
                .collect::<Result<Vec<_>>>()?,
        })
    }
    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
    pub fn oneofs(&self) -> &[Oneof] {
        &self.oneofs
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
    pub fn rust_enum_ident(&self) -> String {
        format!("{}_SimpleImpl", self.oneof.rust_enum_ident())
    }
    pub fn rust_field_ident(&self) -> &str {
        self.oneof.rust_getter_ident()
    }
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
    pub fn rust_path(&self) -> Result<String> {
        let message = self.oneof.message()?;
        Ok(format!(
            "{path}::puroro_nested::{submodule}::{ident}",
            path = message.rust_module_path(),
            submodule = message.rust_nested_module_ident(),
            ident = self.rust_enum_ident(),
        ))
    }
}
