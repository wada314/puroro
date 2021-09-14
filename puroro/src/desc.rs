use ::once_cell::sync::Lazy;

pub struct MessageDescriptor {
    pub(crate) name: &'static str,
    pub(crate) lazy_fields: Lazy<&'static [FieldDescriptor]>,
}
impl MessageDescriptor {
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn fields(&self) -> &'static [FieldDescriptor] {
        *Lazy::force(&self.lazy_fields)
    }
}

pub struct FieldDescriptor {
    pub(crate) name: &'static str,
    pub(crate) number: i32,
    pub(crate) lazy_containing_type: Lazy<&'static MessageDescriptor>,
}
impl FieldDescriptor {
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn containing_type(&self) -> &MessageDescriptor {
        *Lazy::force(&self.lazy_containing_type)
    }
}
