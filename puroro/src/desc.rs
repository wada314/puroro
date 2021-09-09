use ::once_cell::sync::Lazy;

pub struct MessageDescriptor {
    lazy_fields: Lazy<&'static [FieldDescriptor]>,
}
impl MessageDescriptor {
    pub fn fields(&self) -> &'static [FieldDescriptor] {
        *Lazy::force(&self.lazy_fields)
    }
}

pub struct FieldDescriptor {
    lazy_containing_type: Lazy<&'static MessageDescriptor>,
}
impl FieldDescriptor {
    pub fn containing_type(&self) -> &MessageDescriptor {
        *Lazy::force(&self.lazy_containing_type)
    }
}
