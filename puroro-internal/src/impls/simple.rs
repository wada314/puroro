mod type_gen;

use ::puroro::de::from_iter::ImplIsDeserializableFromIter;
use ::puroro::{tags, StructInternalTypeGen};

pub struct SimpleImpl;
impl tags::ImplTypeTag for SimpleImpl {}

impl ImplIsDeserializableFromIter for SimpleImpl {
    fn deser_field<LabelAndType, I>(
        field: &mut <Self as puroro::FieldTypeGen<LabelAndType>>::Type,
        input: puroro::de::FieldData<I>,
    ) -> puroro::Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        Self: puroro::FieldTypeGen<LabelAndType>,
    {
        todo!()
    }
}

// Struct's internal type generator

impl StructInternalTypeGen for SimpleImpl {
    // TODO
    type Type = ();
}
