use crate::tags;

pub trait ApplyToFieldMut<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply(&mut self, field: &mut FieldType) -> Result<ReturnType, ErrorType>;
}

pub trait ApplyToFieldMutForSimpleMessage<ReturnType, ErrorType>:
    ApplyToFieldMut<tags::Required, tags::Int32, i32, ReturnType, ErrorType>
{
}
