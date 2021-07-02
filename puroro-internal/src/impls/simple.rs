mod type_gen;

use ::puroro::de::from_iter::ImplIsDeserializableFromIter;
use ::puroro::{tags, FieldTypeGen, Result, StructInternalTypeGen};

pub struct SimpleImpl;
impl tags::ImplTypeTag for SimpleImpl {}

trait DeserFromIter<LabelAndType>: FieldTypeGen<LabelAndType> {
    fn deser<I>(
        field: &mut <Self as puroro::FieldTypeGen<LabelAndType>>::Type,
        input: puroro::de::FieldData<I>,
    ) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>;
}

impl ImplIsDeserializableFromIter for SimpleImpl {
    fn deser_field<LabelAndType, I>(
        field: &mut <Self as puroro::FieldTypeGen<LabelAndType>>::Type,
        input: puroro::de::FieldData<I>,
    ) -> puroro::Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        Self: puroro::FieldTypeGen<LabelAndType>,
    {
        <Self as DeserFromIter<LabelAndType>>::deser(field, input)
    }
}

// Struct's internal type generator

impl StructInternalTypeGen for SimpleImpl {
    // TODO
    type Type = ();
}
