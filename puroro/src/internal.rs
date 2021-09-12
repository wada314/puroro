pub mod de;
pub mod impls;
pub mod se;

use crate::desc::{FieldDescriptor, MessageDescriptor};
use crate::once_cell::sync::Lazy;

pub struct MessageDescriptorInitializer {
    pub lazy_fields: Lazy<&'static [FieldDescriptor]>,
}

pub fn init_message_descriptor(init: MessageDescriptorInitializer) -> MessageDescriptor {
    MessageDescriptor {
        lazy_fields: init.lazy_fields,
    }
}

pub struct FieldDescriptorInitializer {
    pub lazy_containing_type: Lazy<&'static MessageDescriptor>,
}

pub fn init_field_descriptor(init: FieldDescriptorInitializer) -> FieldDescriptor {
    FieldDescriptor {
        lazy_containing_type: init.lazy_containing_type,
    }
}
