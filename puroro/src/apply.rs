pub trait ApplyToField<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply(&self, field: &mut FieldType) -> Result<ReturnType, ErrorType>;
}

pub trait ApplyToFieldMut<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply_mut(&mut self, field: &mut FieldType) -> Result<ReturnType, ErrorType>;
}
