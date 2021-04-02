use crate::Result;
use ::puroro::tags;
use ::puroro_serializer::deserializer::stream::{
    DelayedLengthDelimitedDeserializer, Field, LengthDelimitedDeserializer,
};
use puroro::tags::FieldTypeTag;

trait FromField {
    type Output;
    fn from_field(field: Field<DelayedLengthDelimitedDeserializer>) -> Result<Self::Output>;
}

impl FromField for tags::Int32 {
    type Output = <Self as FieldTypeTag>::SingularRustType;

    fn from_field(field: Field<DelayedLengthDelimitedDeserializer>) -> Result<Self::Output> {
        todo!()
    }
}
