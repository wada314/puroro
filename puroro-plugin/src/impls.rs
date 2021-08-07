use crate::wrappers;
use ::askama::Template;
use ::std::rc::Rc;

#[derive(Template)]
#[template(path = "impl_simple.rs.txt")]
pub struct SimpleImpl {
    pub message: Rc<wrappers::Message>,
}
