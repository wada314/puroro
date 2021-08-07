use crate::wrappers;
use ::askama::Template;
use ::std::rc::Rc;

#[derive(Template)]
#[template(path = "impl_simple.rs.txt")]
pub struct SimpleImpl {
    message: Rc<wrappers::Message>,
    rust_ident: String,
}

impl SimpleImpl {
    pub fn new(message: &Rc<wrappers::Message>) -> Self {
        Self {
            message: message.clone(),
            rust_ident: format!("{}_SimpleImpl", message.rust_ident()),
        }
    }
    pub fn rust_ident(&self) -> &str {
        &self.rust_ident
    }
}
