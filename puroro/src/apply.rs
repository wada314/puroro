pub trait ApplyToField<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply(&mut self, field: &FieldType) -> Result<ReturnType, ErrorType>;
}

pub trait ApplyToFieldMut<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply_mut(&mut self, field: &mut FieldType) -> Result<ReturnType, ErrorType>;
}
