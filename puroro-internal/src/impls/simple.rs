mod type_gen;

use ::puroro::{tags, Result, StructInternalTypeGen};

pub struct SimpleImpl;
impl tags::ImplTypeTag for SimpleImpl {}

// Struct's internal type generator

impl StructInternalTypeGen for SimpleImpl {
    // TODO
    type Type = ();
}
