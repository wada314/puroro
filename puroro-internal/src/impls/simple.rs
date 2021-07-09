mod deser_iter;
mod field_wrapper;
mod ser_io_write;
mod type_gen;

use crate::{ChooseStructVisibility, StructInternalTypeGen};
use ::puroro::tags;
pub use field_wrapper::{LabelWrappedLdType, LabelWrappedType};

pub struct SimpleImpl;
impl tags::ImplTypeTag for SimpleImpl {}

impl<Public, Private> ChooseStructVisibility<Public, Private> for SimpleImpl {
    type Type = Public;
}

// Struct's internal type generator
impl StructInternalTypeGen for SimpleImpl {
    // TODO
    type Type = ();
}
