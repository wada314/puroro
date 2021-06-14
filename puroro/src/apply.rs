use crate::tags;

pub trait ApplyToFieldMut<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply(&mut self, field: &mut FieldType) -> Result<ReturnType, ErrorType>;
}

pub trait ApplyToFieldMutForSimpleMessage<ReturnType, ErrorType>:
    ApplyToFieldMut<tags::Required, tags::Int32, i32, ReturnType, ErrorType>
{
}

macro_rules! simple_message_triples {
    () => {
        (
            (tags::Required, tags::Int32, i32),
            (tags::Required, tags::String, String),
        )
    };
}
