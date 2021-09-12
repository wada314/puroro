pub mod de;
pub mod impls;
pub mod se;

use crate::desc::{FieldDescriptor, MessageDescriptor};
use crate::once_cell::sync::Lazy;

pub struct MessageDescriptorInitializer {
    pub name: &'static str,
    pub lazy_fields: Lazy<&'static [FieldDescriptor]>,
}

pub fn init_message_descriptor(init: MessageDescriptorInitializer) -> MessageDescriptor {
    MessageDescriptor {
        name: init.name,
        lazy_fields: init.lazy_fields,
    }
}

pub struct FieldDescriptorInitializer {
    pub name: &'static str,
    pub number: i32,
    pub lazy_containing_type: Lazy<&'static MessageDescriptor>,
}

pub fn init_field_descriptor(init: FieldDescriptorInitializer) -> FieldDescriptor {
    FieldDescriptor {
        name: init.name,
        number: init.number,
        lazy_containing_type: init.lazy_containing_type,
    }
}
