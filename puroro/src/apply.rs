pub trait FieldVisitor<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn visit(&mut self, field: &FieldType, field_number: usize) -> Result<ReturnType, ErrorType>;
}

pub trait FieldVisitorMut<LabelTag, WireAndValueTag, FieldType, ReturnType, ErrorType> {
    fn visit_mut(
        &mut self,
        field: &mut FieldType,
        field_number: usize,
    ) -> Result<ReturnType, ErrorType>;
}
