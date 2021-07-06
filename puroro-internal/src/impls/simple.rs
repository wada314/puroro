mod deser_iter;
mod field_wrapper;
mod type_gen;

use crate::StructInternalTypeGen;
use ::puroro::tags;
pub use field_wrapper::{LabelWrappedLDType, LabelWrappedType};

pub struct SimpleImpl;
impl tags::ImplTypeTag for SimpleImpl {}

// Struct's internal type generator

impl StructInternalTypeGen for SimpleImpl {
    // TODO
    type Type = ();
}
