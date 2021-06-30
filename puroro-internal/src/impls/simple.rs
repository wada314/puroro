use ::puroro::{tags, FieldTypeGen};

struct SimpleImpl;

impl<T: tags::FieldLabelAndTypeTag> FieldTypeGen<T> for SimpleImpl {
    type Type = i32;
}
