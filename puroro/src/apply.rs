pub trait ApplyToField<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply(&mut self, field: &FieldType, field_number: usize) -> Result<ReturnType, ErrorType>;
}

pub trait ApplyToFieldMut<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn apply_mut(
        &mut self,
        field: &mut FieldType,
        field_number: usize,
    ) -> Result<ReturnType, ErrorType>;
}
