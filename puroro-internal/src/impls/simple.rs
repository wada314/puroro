use ::puroro::{tags, FieldTypeGen, StructInternalTypeGen};

pub struct SimpleImpl;

impl<T: tags::FieldLabelAndTypeTag> FieldTypeGen<T> for SimpleImpl {
    type Type = i32;
}

impl StructInternalTypeGen for SimpleImpl {
    type Type = ();
}
